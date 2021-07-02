use std::{borrow::Cow, collections::BTreeMap};

use anyhow::Context;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct KeyBinds {
    #[serde(flatten)]
    pub map: BTreeMap<Action, Input>,
}

impl Default for KeyBinds {
    fn default() -> Self {
        Self {
            map: <_>::into_iter([
                (Action::FocusStatusView, Input::char('0')),
                (Action::FocusMessagesView, Input::char('1')),
                (Action::FocusLinksView, Input::char('2')),
                (Action::FocusHighlightsView, Input::char('3')),
                (Action::Quit, Input::char('q')),
                (Action::ToggleTimestamp, Input::char('t')),
                (Action::ToggleBadges, Input::char('b')),
            ])
            .collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    FocusStatusView,
    FocusMessagesView,
    FocusLinksView,
    FocusHighlightsView,

    Quit,
    ToggleTimestamp,
    ToggleBadges,
}

#[derive(Copy, Clone, Debug)]
pub struct Input {
    pub key: Key,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <_>::into_iter([
            (self.ctrl, 'c'), //
            (self.alt, 'a'),
            (self.shift, 's'),
        ])
        .filter_map(|(k, v)| k.then(|| [v, '-']))
        .flatten()
        .try_for_each(|ch| write!(f, "{}", ch))?;

        match self.key {
            Key::Char(ch) => write!(f, "{}", ch),
            Key::F(n) => {
                let head = std::iter::once('f');
                let tail = split_digits(n).map(|c| (c + b'0') as char);
                head.chain(tail).try_for_each(|ch| write!(f, "{}", ch))
            }
        }
    }
}

impl From<Input> for cursive::event::Event {
    fn from(input: Input) -> Self {
        if let Some(fkey) = input.key.maybe_function_key() {
            return match (input.ctrl, input.alt, input.shift) {
                (true, true, true) => unreachable!("this is an invalid combination"), // TODO: error
                (true, true, false) => Self::CtrlAlt(fkey),
                (true, false, true) => Self::CtrlShift(fkey),
                (true, false, false) => Self::Ctrl(fkey),
                (false, true, true) => Self::AltShift(fkey),
                (false, true, false) => Self::Alt(fkey),
                (false, false, true) => Self::Shift(fkey),
                (false, false, false) => Self::Key(fkey),
            };
        };

        if let Key::Char(ch) = input.key {
            return match (input.ctrl, input.alt) {
                (true, true) => unreachable!("this is an invalid combination"),
                (true, false) => Self::CtrlChar(ch),
                (false, true) => Self::AltChar(ch),
                (false, false) => Self::Char(ch),
            };
        }

        unreachable!("a key should have been processed")
    }
}

impl Input {
    const fn char(ch: char) -> Self {
        Self {
            key: Key::Char(ch),
            ctrl: false,
            alt: false,
            shift: false,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Key {
    Char(char),
    F(u8),
}

impl Key {
    fn maybe_function_key(&self) -> Option<cursive::event::Key> {
        use cursive::event::Key as K;
        Some(match self {
            Self::Char(_) => return None,
            Self::F(0) => K::F0,
            Self::F(1) => K::F1,
            Self::F(2) => K::F2,
            Self::F(3) => K::F3,
            Self::F(4) => K::F4,
            Self::F(5) => K::F5,
            Self::F(6) => K::F6,
            Self::F(7) => K::F7,
            Self::F(8) => K::F8,
            Self::F(9) => K::F9,
            Self::F(10) => K::F10,
            Self::F(11) => K::F11,
            Self::F(12) => K::F12,
            _ => unreachable!(),
        })
    }
}

impl serde::Serialize for Input {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}

impl<'de> serde::Deserialize<'de> for Input {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <Cow<'_, str>>::deserialize(deserializer)?
            .parse()
            .map_err(|err| <D::Error as serde::de::Error>::custom(err))
    }
}

impl std::str::FromStr for Input {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> anyhow::Result<Self> {
        let input = input.trim();
        let (mut ctrl, mut alt, mut shift) = <_>::default();
        let mut out = None;

        for ch in input.split_terminator('-') {
            anyhow::ensure!(out.is_none(), "character was already found");
            match ch.as_bytes() {
                [b'c'] => ctrl = true,
                [b'a'] => alt = true,
                [b's'] => shift = true,

                [b'f', n @ ..] => match std::str::from_utf8(n)?.parse()? {
                    n @ 0..=12 => out = Some(Key::F(n)),
                    _ => anyhow::bail!("only f0-f12 is supported"),
                },
                [ch] if ch.is_ascii_alphanumeric() || ch.is_ascii_punctuation() => {
                    if ch.is_ascii_uppercase() {
                        shift = true;
                    }
                    out = Some(Key::Char((*ch as char).to_ascii_lowercase()))
                }

                ch => anyhow::bail!("unsupported input: {}", std::str::from_utf8(ch)?),
            };
        }

        if out.is_none() {
            match (ctrl, alt, shift) {
                (true, _, _) => out = Some(Key::Char('c')),
                (_, true, _) => out = Some(Key::Char('a')),
                (_, _, true) => out = Some(Key::Char('s')),
                _ => {}
            }
        }

        let mut key = out.with_context(|| "character sequence must be present")?;
        if matches!(key, Key::Char(_)) && (ctrl && alt) {
            anyhow::bail!("ctrl cannot be used with alt for character keys")
        }

        if matches!(key, Key::F(..)) && (ctrl && alt && shift) {
            anyhow::bail!("ctrl cannot be used with alt and shift for function keys")
        }

        if shift {
            if let Key::Char(ch) = &mut key {
                *ch = ch.to_ascii_uppercase()
            }
        }

        Ok(Self {
            key,
            ctrl,
            alt,
            shift,
        })
    }
}

fn split_digits(mut n: u8) -> impl Iterator<Item = u8> {
    let y = 0_u8.count_zeros() - n.leading_zeros();
    let x = ((y + 1) * 1233) >> 12;
    let mut mag = 10_u8.pow(x);

    if n < mag {
        mag /= 10;
    }

    std::iter::from_fn(move || {
        (mag > 0).then(|| {
            let d = n / mag;
            n %= mag;
            mag /= 10;
            d
        })
    })
}
