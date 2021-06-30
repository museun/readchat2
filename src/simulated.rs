use std::{
    collections::HashSet,
    io::{BufRead as _, BufReader, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::Arc,
};

type Color = cursive::theme::Color;

const IPSUM: [&str; 89] = include!("../etc/ipsum.inc");
const ANIMALS: [&str; 25] = include!("../etc/animals.inc");
const ADJECTIVES: [&str; 23] = include!("../etc/adjectives.inc");

pub fn simulated_twitch_chat() -> anyhow::Result<(String, Arc<TcpStream>)> {
    const SIMULATED_CHANNEL: &str = "#testing";

    let addr = twitch_chat_experience()?;
    let stream = TcpStream::connect(addr).map(Arc::new)?;
    Ok((SIMULATED_CHANNEL.to_string(), stream))
}

struct Chatter {
    name: Arc<str>,
    display_color: String,
}

impl Chatter {
    fn new() -> Self {
        let mut name = format!(
            "{}{}",
            ADJECTIVES.choose().unwrap(),
            ANIMALS.choose().unwrap()
        );

        name.extend(
            std::iter::repeat_with(|| fastrand::u8(b'0'..=b'9'))
                .take(fastrand::usize(0..3))
                .map(|c| c as char),
        );

        let color = crate::colors::DEFAULT_COLORS.choose().copied().unwrap();
        let display_color = match color {
            Color::Rgb(r, g, b) => {
                format!("#{r:02X}{g:02X}{b:02X}", r = r, g = g, b = b)
            }
            _ => unreachable!(),
        };

        Self {
            name: name.into(),
            display_color,
        }
    }

    fn speak(&self) -> String {
        const MIN: usize = 5;
        const MAX: usize = 150;

        let mut len = fastrand::usize(MIN..MAX);
        let mut data = String::with_capacity(MAX);
        let mut iter = IPSUM.iter().cycle();
        while len > 0 {
            let ipsum = iter.next().unwrap();
            if fastrand::bool() {
                continue;
            }

            data.push_str(ipsum);
            data.push(' ');
            len = len.saturating_sub(ipsum.len() - 1);
        }
        data
    }
}

fn garbage_out(io: &mut dyn Write, chatters: &[Chatter]) -> anyhow::Result<()> {
    const MIN: u64 = 250;
    const MAX: u64 = 1500;

    while let Some(chatter) = chatters.choose() {
        write!(
            io,
            "@color={color} :{name}!{name}@{name} PRIVMSG #testing :{msg}\r\n",
            color = chatter.display_color,
            name = chatter.name,
            msg = chatter.speak()
        )?;
        std::thread::sleep(std::time::Duration::from_millis(fastrand::u64(MIN..MAX)))
    }

    Ok(())
}

fn twitch_chat_experience() -> anyhow::Result<SocketAddr> {
    let cap = fastrand::usize(5..15);
    let mut chatters = Vec::with_capacity(cap);
    let mut seen = HashSet::new();
    for chatter in std::iter::repeat_with(Chatter::new) {
        if seen.insert(Arc::clone(&chatter.name)) {
            chatters.push(chatter)
        }
        if chatters.len() == cap {
            break;
        }
    }

    let listener = TcpListener::bind("localhost:0")?;
    let addr = listener.local_addr()?;
    let _ = std::thread::spawn(move || feed_chat(listener, chatters));
    Ok(addr)
}

fn feed_chat(listener: TcpListener, chatters: Vec<Chatter>) {
    for mut socket in listener.incoming().flatten() {
        if wait_for_join(&socket).is_err() {
            continue;
        }

        if garbage_out(&mut socket, &chatters).is_err() {
            continue;
        }
    }
}

fn wait_for_join(mut io: &TcpStream) -> anyhow::Result<()> {
    const JOIN_MESSAGE: &str = include!("../etc/join.inc");
    const READY: [&str; 5] = include!("../etc/ready.inc");

    READY
        .iter()
        .map(|s| s.as_bytes())
        .map(|line| io.write_all(line).map_err(Into::into))
        .collect::<anyhow::Result<()>>()?;

    for line in BufReader::new(io).lines().flatten() {
        if line == "JOIN #testing" {
            io.write_all(JOIN_MESSAGE.as_bytes())?;
            break;
        }
    }

    Ok(())
}

trait RandExt {
    type Output: ?Sized;
    fn choose(&self) -> Option<&Self::Output>;
}

impl<T> RandExt for [T] {
    type Output = T;
    fn choose(&self) -> Option<&Self::Output> {
        self.get(fastrand::usize(0..self.len()))
    }
}
