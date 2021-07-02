use std::io::Write;

use twitchchat::{commands::raw, Encoder};

use crate::twitch::Update;

pub enum ChatMode {
    Real(String),
    Simulated,
}

impl ChatMode {
    pub fn connect(
        self,
        logger: impl Write + Send + Sync + 'static,
    ) -> anyhow::Result<impl FnOnce(cursive::CbSink)> {
        let (updates_tx, updates_rx) = flume::unbounded();
        updates_tx.send(Update::Connecting)?;

        let (channel, stream) = match self {
            Self::Real(channel) => {
                let stream = crate::twitch::connect().map(std::sync::Arc::new)?;
                (channel, stream)
            }
            Self::Simulated => crate::simulated::simulated_twitch_chat()?,
        };

        let cb = move |sink: cursive::CbSink| {
            let (activity_tx, activity_rx) = flume::unbounded();

            let read_handle = std::thread::spawn({
                let channel = channel.to_string();
                let stream = stream.clone();
                move || crate::twitch::read_loop(stream, &channel, updates_tx, activity_tx)
            });

            std::thread::spawn::<_, anyhow::Result<()>>({
                let stream = stream.clone();
                move || {
                    let stream = stream;
                    crate::twitch::inner_loop(
                        Encoder::new(&*stream),
                        updates_rx,
                        activity_rx,
                        sink,
                        logger,
                    )?;
                    let _ = Encoder::new(&*stream).encode(raw("QUIT :leaving"));
                    let _ = read_handle.join();
                    Ok(())
                }
            });
        };

        Ok(cb)
    }
}
