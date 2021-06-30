use cursive::theme::Effect;

macro_rules! make_effects {
    ($($id:ident)*) => {
        #[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd)]
        pub struct Effects {
            $(
                pub $id: bool,
            )*
        }

        impl Effects {
            pub const fn empty() -> Self {
                Self {
                    $($id: false,)*
                }
            }

            $(
                pub const fn $id() -> Self {
                    let mut e = Self::empty();
                    e.$id = true;
                    e
                }
            )*

            pub fn into_iter(self) -> impl Iterator<Item = (&'static str, bool)> + DoubleEndedIterator + ExactSizeIterator {
                <_>::into_iter([$(stringify!($id),)*]).zip([$(self.$id,)*])
            }
        }
    };
}

make_effects! {
    simple
    reverse
    bold
    italic
    strikethrough
    underline
    blink
}

impl std::ops::BitOr<Effect> for Effects {
    type Output = Self;

    fn bitor(mut self, rhs: Effect) -> Self::Output {
        use cursive::theme::Effect::*;

        *match rhs {
            Simple => &mut self.simple,
            Reverse => &mut self.reverse,
            Bold => &mut self.bold,
            Italic => &mut self.italic,
            Strikethrough => &mut self.strikethrough,
            Underline => &mut self.underline,
            Blink => &mut self.blink,
        } |= true;

        self
    }
}

const SEPERATOR: char = '|';

impl<'de> serde::Deserialize<'de> for Effects {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct V;
        impl<'de> serde::de::Visitor<'de> for V {
            type Value = Effects;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.split(SEPERATOR)
                    .map(str::trim)
                    .try_fold(Effects::default(), |mut eff, key| {
                        *match key {
                            "simple" => &mut eff.simple,
                            "reverse" => &mut eff.reverse,
                            "bold" => &mut eff.bold,
                            "italic" => &mut eff.italic,
                            "strikethrough" => &mut eff.strikethrough,
                            "underline" => &mut eff.underline,
                            "blink" => &mut eff.blink,
                            e => return Err(E::custom(&format!("unknown effect: {}", e))),
                        } = true;
                        Ok(eff)
                    })
            }
        }

        deserializer.deserialize_str(V)
    }
}

impl serde::Serialize for Effects {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s =
            self.into_iter()
                .filter_map(|(k, v)| v.then(|| k))
                .fold(String::new(), |mut a, c| {
                    if !a.is_empty() {
                        a.push(' ');
                        a.push(SEPERATOR);
                        a.push(' ');
                    }
                    a.push_str(c.as_ref());
                    a
                });

        if s.is_empty() {
            serializer.serialize_none()
        } else {
            serializer.serialize_some(&s)
        }
    }
}

impl From<Effects> for cursive::theme::Style {
    fn from(effects: Effects) -> Self {
        Self {
            effects: effects
                .into_iter()
                .zip([
                    Effect::Simple,
                    Effect::Reverse,
                    Effect::Bold,
                    Effect::Italic,
                    Effect::Strikethrough,
                    Effect::Underline,
                    Effect::Blink,
                ])
                .fold(Default::default(), |eff, ((_, v), e)| {
                    v.then(|| eff | e).unwrap_or_else(|| eff & e)
                }),
            ..Default::default()
        }
    }
}
