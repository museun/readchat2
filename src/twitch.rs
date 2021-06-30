use std::{
    io::{Read, Write},
    net::TcpStream,
    ops::Deref,
    time::{Duration, Instant, SystemTime},
};

use twitchchat::{
    commands::*,
    messages::Commands::{self, *},
    UserConfig, {Decoder, Encoder, FromIrcMessage as _},
};

use crate::{app::App, entry::Entry};

enum Activity {
    Pong,
    Ping(String),
    Message,
}

#[derive(Debug)]
enum Update {
    Raw(String),
    Append(Entry),
    Connecting,
    Connected,
    Ping,
    Pong,
    Joining(String),
    Joined(String),
}

fn read_loop<I, R>(
    stream: I,
    channel: &str,
    updates: flume::Sender<Update>,
    activity: flume::Sender<Activity>,
) -> anyhow::Result<()>
where
    I: Deref<Target = R> + Clone + Send + 'static,
    for<'i> &'i R: Read + Write + Send + Sync,
{
    let decoder = Decoder::new(&*stream);
    let mut encoder = Encoder::new(&*stream);

    let mut our_name = String::new();

    for message in decoder
        .into_iter()
        .flatten()
        .map(Commands::from_irc)
        .flatten()
    {
        updates.send(Update::Raw(message.raw().to_string()))?;

        match message {
            Ready(msg) => {
                updates.send(Update::Connected)?;
                our_name = msg.username().to_string();

                updates.send(Update::Joining(channel.to_string()))?;
                encoder.encode(join(channel))?;

                activity.send(Activity::Message)?;
            }

            Join(msg) if our_name == msg.name() => {
                updates.send(Update::Joined(channel.to_string()))?;
                activity.send(Activity::Message)?;
            }

            Privmsg(msg) => {
                updates.send(Update::Append(msg.into()))?;
                activity.send(Activity::Message)?;
            }

            Ping(msg) => {
                updates.send(Update::Ping)?;
                activity.send(Activity::Ping(msg.token().to_string()))?;
            }
            Pong(_) => {
                updates.send(Update::Pong)?;
                activity.send(Activity::Pong)?;
            }

            ClearChat(_) => {}
            ClearMsg(_) => {}

            HostTarget(_) => {}
            Reconnect(_) => {}

            _ => {}
        }
    }

    Ok(())
}

fn connect_inner() -> anyhow::Result<TcpStream> {
    let config = UserConfig::builder()
        .anonymous()
        .enable_all_capabilities()
        .build()?;

    let stream = TcpStream::connect(twitchchat::TWITCH_IRC_ADDRESS)?;
    Encoder::new(&stream)
        .encode(register(&config))
        .map(|_| stream)
        .map_err(Into::into)
}

fn inner_loop(
    mut encoder: twitchchat::Encoder<&TcpStream>,
    updates_rx: flume::Receiver<Update>,
    activity_rx: flume::Receiver<Activity>,
    sink: cursive::CbSink,
) -> anyhow::Result<()> {
    const WINDOW: Duration = Duration::from_secs(15);
    const TIMEOUT: Duration = Duration::from_secs(30);

    enum Step {
        Continue,
        Exit,
    }

    let ts = SystemTime::UNIX_EPOCH
        .elapsed()
        .map(|dur| dur.as_millis().to_string())
        .unwrap();
    encoder.encode(ping(&ts)).unwrap();

    let mut last = Instant::now();

    loop {
        std::thread::yield_now();

        match flume::Selector::new()
            .recv(&activity_rx, |activity| match activity {
                Ok(Activity::Ping(tok)) => {
                    let _ = encoder.encode(pong(&tok));
                    last = Instant::now();
                    Step::Continue
                }
                Ok(..) => {
                    last = Instant::now();
                    Step::Continue
                }
                Err(..) => Step::Exit,
            })
            .recv(&updates_rx, |update| match update {
                Ok(update) => {
                    let cb: Box<dyn FnOnce(&mut cursive::Cursive) + Send> = match update {
                        Update::Raw(raw) => Box::new(|c| App::append_raw(c, raw)),
                        Update::Append(entry) => Box::new(|c| App::append_entry(c, entry)),
                        Update::Connecting => Box::new(App::on_connecting),
                        Update::Connected => Box::new(App::on_connected),
                        Update::Ping => Box::new(App::on_ping),
                        Update::Pong => Box::new(App::on_pong),
                        Update::Joining(channel) => Box::new(|c| App::on_joining(c, channel)),
                        Update::Joined(channel) => Box::new(|c| App::on_joined(c, channel)),
                    };
                    if sink.send(cb).is_ok() {
                        Step::Continue
                    } else {
                        Step::Exit
                    }
                }
                Err(_) => Step::Exit,
            })
            .wait_timeout(TIMEOUT)
        {
            Ok(step) => match step {
                Step::Continue => continue,
                Step::Exit => break,
            },
            Err(_) => {
                match last.elapsed() {
                    dur if dur >= WINDOW => {
                        let ts = SystemTime::UNIX_EPOCH
                            .elapsed()
                            .map(|dur| dur.as_millis().to_string())?;
                        encoder.encode(ping(&ts))?;
                        // TODO don't do this
                        std::thread::sleep(std::time::Duration::from_millis(150));
                        // give the network some time to do something
                    }
                    dur if dur >= TIMEOUT => {
                        encoder.encode(raw("QUIT :leaving"))?;
                        anyhow::bail!("we've timed out");
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

pub fn connect(channel: &str) -> anyhow::Result<impl FnOnce(cursive::CbSink)> {
    let (updates_tx, updates_rx) = flume::unbounded();
    updates_tx.send(Update::Connecting)?;

    let stream = connect_inner().map(std::sync::Arc::new)?;
    let channel = channel.to_string();

    let cb = move |sink: cursive::CbSink| {
        let (activity_tx, activity_rx) = flume::unbounded();

        let read_handle = std::thread::spawn({
            let channel = channel.to_string();
            let stream = stream.clone();
            move || read_loop(stream, &channel, updates_tx, activity_tx)
        });

        std::thread::spawn::<_, anyhow::Result<()>>({
            let stream = stream.clone();
            move || {
                let stream = stream;
                inner_loop(Encoder::new(&*stream), updates_rx, activity_rx, sink)?;
                let mut encoder = Encoder::new(&*stream);
                let _ = encoder.encode(raw("QUIT :leaving"));
                let _ = read_handle.join();
                Ok(())
            }
        });
    };

    Ok(cb)
}
