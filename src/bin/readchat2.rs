#![cfg_attr(debug_assertions, allow(dead_code,))]
use std::sync::Arc;

use readchat2::*;

pub struct Args {
    channel: String,
}

impl Args {
    const HEADER: &'static str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

    const SHORT_HELP: &'static str = r#"
USAGE:
    readchat2 [flags] <channel>

FLAGS:
    -h, --help                  show the help messages
    -v, --version               show the current version
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

        let channel = match args.finish().as_slice() {
            [channel] => channel.to_string_lossy().to_string(),
            [] => {
                eprintln!("please provide a channel: readchat <channel>");
                std::process::exit(1);
            }
            _ => {
                eprintln!("only provide a single channel: readchat <channel>");
                std::process::exit(1);
            }
        };

        Ok(Self { channel })
    }
}

fn new_cursive(config: Config) -> cursive::CursiveRunnable {
    let mut cursive = cursive::default();
    cursive.set_theme(colors::sensible_theme());
    cursive.add_fullscreen_layer(build_ui());
    cursive
        .focus_name(MessagesView::name())
        .expect("MessageView should be in the tree");
    cursive.set_user_data(Arc::new(config));
    cursive
}

fn main() -> anyhow::Result<()> {
    let Args { channel } = Args::parse()?;

    panic_logger::setup();

    let config = match std::fs::read(Config::config_path()?) {
        Ok(data) => Config::from_toml(data)?,
        Err(err) if matches!(err.kind(), std::io::ErrorKind::NotFound) => {
            let config_dir = Config::config_dir()?;
            let opt_out = config_dir.join(".no_config_wanted");
            if !opt_out.exists() {
                eprintln!(
                    "no configuration file was found at: {}",
                    Config::config_path()?.to_string_lossy()
                );
                eprintln!("if you want to configure the colors / default appearance then:");
                eprintln!(" 1. mkdir -p $(readchat --print-config-path)");
                eprintln!(" 2. readchat --print-default-config > $(readchat --print-config-path)");
                eprintln!(" 3. $EDITOR $(readchat --print-config-path)");

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

    let channel = config.channel.clone().unwrap_or_else(|| channel);

    let mut cursive = new_cursive(config);
    cursive.set_global_callback('q', App::quit);

    cursive.set_global_callback('0', App::focus_status_view);
    cursive.set_global_callback('1', App::focus_messages_view);
    cursive.set_global_callback('2', App::focus_links_view);

    cursive.set_global_callback('t', App::toggle_timestamp);
    cursive.set_global_callback('b', App::toggle_badges);

    App::focus_status_view(&mut cursive);

    let sink = cursive.cb_sink().clone();
    twitch::connect(&*channel)?(sink);
    cursive.run();
    Ok(())
}
