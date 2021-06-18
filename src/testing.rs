use crate::{
    colors,
    entry::{Badge, Entry},
    simulated::{self, RandExt},
    App,
};

pub fn simulated() -> impl Fn(cursive::CbSink) {
    |sink| {
        let pick = || {
            [
                Badge::Partner,
                Badge::VIP,
                Badge::Premium,
                Badge::Bits,
                Badge::Turbo,
                Badge::Subscriber,
                Badge::Moderator,
                Badge::Broadcaster,
                Badge::GlobalMod,
                Badge::Staff,
                Badge::Admin,
            ]
            .choose()
            .copied()
        };

        let _handle = std::thread::spawn(move || {
            for msg in simulated::simulated_twitch_chat() {
                let mut entry: Entry = msg.into();

                if fastrand::bool() {
                    entry.badge = pick();
                }

                let send = sink.send(Box::new(move |cursive| App::append_entry(cursive, entry)));
                if send.is_err() {
                    break;
                }
            }
        });
    }
}

pub fn sorta_real() -> impl Fn(cursive::CbSink) {
    let input = [
    ("Apr 19 21 12:23:07","museun","see https://crates.io/crates/regex and https://crates.io/crates/cargo",),
    ("Jun 14 21 12:08:50","mrhalzy","!discord",),
    ("Jun 14 21 12:08:50","kappatan","https://discord.gg/bNcgtCa",),
    ("Jun 14 21 12:29:03","museun","type BoxedFuture<E> = Pin<Box<dyn Future<Output = crate::Result<E>>>>",),
    ("Jun 14 21 12:29:25","museun","or, maybe make E just the output, but I imagine your Result is common",),
    ("Jun 14 21 12:30:09","museun","https://github.com/rust-lang/rust/issues/67644 one day the box::pin etc would be implicit",),
    ("Jun 14 21 12:30:18","museun","https://doc.rust-lang.org/alloc/boxed/struct.Box.html#method.into_future",),
];

    let conv = |s| {
        use chrono::offset::TimeZone as _;
        chrono::Local.datetime_from_str(s, "%b %d %y %X").unwrap()
    };

    move |sink| {
        let mut color_cache = std::collections::HashMap::new();
        for (ts, k, v) in std::array::IntoIter::new(input) {
            let name = k.to_string();
            let data = v.to_string();
            let color = *color_cache.entry(k).or_insert_with(colors::choose_color);

            let entry = Entry {
                name,
                data,
                color,
                ts: conv(ts),
                badge: None,
            };
            if sink
                .send(Box::new(move |cursive| App::append_entry(cursive, entry)))
                .is_err()
            {
                break;
            }
        }
    }
}
