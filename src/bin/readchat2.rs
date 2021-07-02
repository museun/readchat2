use std::sync::{Arc, RwLock};

use readchat2::*;

pub struct Args {
    channel: Option<String>,
    simulated: bool,
    transcribe: bool,
}

impl Args {
    const HEADER: &'static str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

    const SHORT_HELP: &'static str = r#"
USAGE:
    readchat2 [flags] <channel>

FLAGS:
    -h, --help                  show the help messages
    -v, --version               show the current version
    --transcribe                logs all messages to disk
    --simulated                 shows a simulated chat
    --print-default-config      print the default toml configuration
    --print-config-path         print the default configuration path
    "#;

    pub fn parse() -> anyhow::Result<Self> {
        let mut args = pico_args::Arguments::from_env();
        if args.contains(["-h", "--help"]) {
            println!("{}\n\n{}", Self::HEADER, Self::SHORT_HELP);
            std::process::exit(0);
        }

        if args.contains(["-v", "--version"]) {
            println!("{}", Self::HEADER);
            std::process::exit(0);
        }

        if args.contains("--print-default-config") {
            println!("{}", Config::default_config().trim());
            std::process::exit(0);
        }

        if args.contains("--print-config-path") {
            println!("{}", Config::config_path()?.to_string_lossy());
            std::process::exit(0);
        }
        let simulated = args.contains("--simulated");
        let transcribe = args.contains("--transcribe");
        let channel = args.finish().pop().map(|s| s.to_string_lossy().to_string());
        Ok(Self {
            channel,
            simulated,
            transcribe,
        })
    }
}

fn new_cursive() -> cursive::CursiveRunnable {
    let mut cursive = cursive::default();
    cursive.set_theme(colors::sensible_theme());
    cursive.add_fullscreen_layer(build_ui());
    cursive
        .focus_name(ui::MessagesView::name())
        .expect("MessageView should be in the tree");
    cursive
}

fn main() -> anyhow::Result<()> {
    let Args {
        channel,
        simulated,
        transcribe,
    } = Args::parse()?;

    panic_logger::setup();

    let config = match std::fs::read(Config::config_path()?) {
        Ok(data) => Config::from_yaml(data)?,
        Err(err) if matches!(err.kind(), std::io::ErrorKind::NotFound) => {
            let config_dir = Config::config_dir()?;
            let opt_out = config_dir.join(".no_config_wanted");
            if !opt_out.exists() {
                eprintln!(
                    "no configuration file was found at: {}",
                    Config::config_path()?.to_string_lossy()
                );
                eprintln!("if you want to configure the colors / default appearance then:");
                eprintln!(" 1. mkdir -p $(readchat2 --print-config-path)");
                eprintln!(
                    " 2. readchat2 --print-default-config > $(readchat2 --print-config-path)"
                );
                eprintln!(" 3. $EDITOR $(readchat2 --print-config-path)");

                eprintln!();
                eprintln!("this message will only show once.");
                eprintln!("if you don't care about having a configuration file");
                eprintln!("then re-run the program and it'll start normally");

                std::fs::create_dir_all(&config_dir)?;
                std::fs::write(&opt_out, "you're ignoring the configuration option. remove this file to get the warning again")?;
                std::process::exit(1);
            }

            Config::default()
        }
        Err(err) => return Err(err.into()),
    };

    type Logger = Box<dyn std::io::Write + Send + Sync + 'static>;

    let logger: Logger = if channel.as_ref().filter(|_| transcribe).is_some() {
        let name = Config::data_dir()?
            .join(format!(
                "{}-{}",
                channel.as_deref().expect("channel must exist"),
                std::time::SystemTime::now().elapsed()?.as_secs()
            ))
            .with_extension(".log");
        let file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(name)?;
        Box::new(file) as _
    } else {
        Box::new(std::io::sink()) as _
    };

    let chat_mode = if simulated {
        ChatMode::Simulated
    } else {
        let channel = match (
            channel.filter(|s| !s.is_empty()),
            config.channel.clone().filter(|s| !s.is_empty()),
        ) {
            (Some(left), ..) => left,
            (.., Some(right)) => right,
            _ => {
                eprintln!("please provide a channel: readchat2 <channel>");
                eprintln!("alternatively add it to the configuration file");
                std::process::exit(1);
            }
        };
        assert!(!channel.is_empty(), "channel shouldn't be empty");

        ChatMode::Real(channel)
    };

    let config = Arc::new(RwLock::new(config));

    readchat2::CONFIG
        .set(Arc::clone(&config))
        .expect("single initialization of the global configuration");

    let mut cursive = new_cursive();

    for (action, binding) in &config.read().unwrap().keybinds.map {
        let func = match action {
            Action::FocusStatusView => App::focus_status_view,
            Action::FocusMessagesView => App::focus_messages_view,
            Action::FocusLinksView => App::focus_links_view,
            Action::FocusHighlightsView => App::focus_highlights_view,
            Action::Quit => App::quit,
            Action::ToggleTimestamp => App::toggle_timestamp,
            Action::ToggleBadges => App::toggle_badges,
        };
        cursive.set_global_callback(*binding, func);
    }

    App::focus_status_view(&mut cursive);

    let sink = cursive.cb_sink().clone();
    chat_mode.connect(logger)?(sink);
    cursive.run();
    Ok(())
}
