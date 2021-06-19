use cursive::{theme::Style, utils::span::SpannedString};

pub trait SpannedAppender {
    fn append<T: Into<String>, S: Into<Style>>(self, text: T, style: S) -> Self;
    fn reversed<T: Into<String>>(self, text: T) -> Self
    where
        Self: Sized,
    {
        self.append(text, cursive::theme::Effect::Reverse)
    }
    fn append_plain<T: Into<String>>(self, text: T) -> Self;
}

impl SpannedAppender for SpannedString<Style> {
    fn append<T: Into<String>, S: Into<Style>>(mut self, text: T, style: S) -> Self {
        let this = &mut self;
        this.append_styled(text, style);
        self
    }

    fn append_plain<T: Into<String>>(mut self, text: T) -> Self {
        let this = &mut self;
        this.append_plain(text);
        self
    }
}
