use std::{
    collections::HashSet,
    io::{BufReader, Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

use twitchchat::messages::Privmsg;

type Color = cursive::theme::Color;

const IPSUM: [&str; 150] = include!("../etc/ipsum.inc");
const ADJECTIVES: [&str; 140] = include!("../etc/adjectives.inc");
const ANIMALS: [&str; 291] = include!("../etc/animals.inc");

const READY: [&str; 4] = include!("../etc/ready.inc");

pub fn simulated_twitch_chat() -> impl Iterator<Item = Privmsg<'static>> {
    let addr = make_interesting_chat().unwrap();
    let stream = TcpStream::connect(addr).unwrap();

    twitchchat::Encoder::new(&stream)
        .encode(twitchchat::commands::join("#testing"))
        .unwrap();
    twitchchat::Decoder::new(stream)
        .into_iter()
        .flatten()
        .flat_map(twitchchat::FromIrcMessage::from_irc)
}

struct Chatter {
    name: String,
    color: Color,
}

impl Chatter {
    fn new() -> Self {
        let mut name = format!(
            "{}{}",
            ADJECTIVES.choose().unwrap(),
            ANIMALS.choose().unwrap()
        );
        name.extend(
            std::iter::repeat_with(|| fastrand::u8(b'0'..b'9'))
                .take(fastrand::usize(0..5))
                .map(|c| c as char),
        );

        let color = *crate::colors::DEFAULT_COLORS.choose().unwrap();
        Self { name, color }
    }

    fn display_color(&self) -> String {
        match self.color {
            Color::Rgb(r, g, b) => format!("#{r:02X}{g:02X}{b:02X}", r = r, g = g, b = b,),
            _ => unreachable!(),
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
            len = len.saturating_sub(ipsum.len() + 1);
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
            color = chatter.display_color(),
            name = chatter.name,
            msg = chatter.speak(),
        )?;

        std::thread::sleep(std::time::Duration::from_millis(fastrand::u64(MIN..MAX)))
    }

    Ok(())
}

fn make_interesting_chat() -> anyhow::Result<SocketAddr> {
    let cap = fastrand::usize(5..15);
    let mut chatters = Vec::with_capacity(cap);
    let mut seen = HashSet::new();
    for chatter in std::iter::repeat_with(Chatter::new) {
        if seen.insert(chatter.name.clone()) {
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

fn wait_for_join(mut io: &TcpStream) -> anyhow::Result<()> {
    const JOIN: &str = ":justinfan1234!justinfan1234@justinfan1234.tmi.twitch.tv JOIN #testing\r\n";

    for line in &READY {
        io.write_all(line.as_bytes())?;
    }

    use std::io::BufRead as _;
    for line in BufReader::new(io).lines().flatten() {
        if line == "JOIN #testing" {
            io.write_all(JOIN.as_bytes())?;
            break;
        }
    }

    Ok(())
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

pub trait RandExt {
    type Output: ?Sized;
    #[track_caller]
    fn choose(&self) -> Option<&Self::Output>;
}

impl<T> RandExt for [T] {
    type Output = T;
    #[track_caller]
    fn choose(&self) -> Option<&Self::Output> {
        self.get(fastrand::usize(0..self.len()))
    }
}
