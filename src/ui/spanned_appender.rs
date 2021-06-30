use cursive::{theme::Style, utils::span::SpannedString};

pub trait SpannedAppender {
    fn append<T, S>(self, text: T, style: S) -> Self
    where
        T: Into<String>,
        S: Into<Style>;

    fn reversed<T>(self, text: T) -> Self
    where
        Self: Sized,
        T: Into<String>,
    {
        self.append(text, cursive::theme::Effect::Reverse)
    }

    fn append_plain<T: Into<String>>(self, text: T) -> Self;
}

impl SpannedAppender for SpannedString<Style> {
    fn append<T, S>(mut self, text: T, style: S) -> Self
    where
        T: Into<String>,
        S: Into<Style>,
    {
        let this = &mut self;
        this.append_styled(text, style);
        self
    }

    fn append_plain<T>(mut self, text: T) -> Self
    where
        T: Into<String>,
    {
        let this = &mut self;
        this.append_plain(text);
        self
    }
}
