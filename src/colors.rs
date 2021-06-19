use cursive::theme::{BorderStyle, Color, Palette, PaletteColor, Theme};
use std::sync::atomic::{AtomicUsize, Ordering};

#[allow(unused_macros)]
macro_rules! const_array {
    (@count) => { 0 };
    (@count $odd:tt $($a:tt $b:tt)*) => { const_array!(@count $($a)*) << 1 | 1 };
    (@count $($a:tt $even:tt)*) => { const_array!(@count $($a)*) << 1 };

    ($vis:vis $ident:ident ; $ty:ty [ $($expr:expr),* $(,)?]) => {
        $vis const $ident: [$ty; const_array!(@count $($expr)*)] = [$($expr),*];
    };
}

const_array! {
    pub DEFAULT_COLORS ; Color [
        Color::Rgb(0, 0, 255),     // `Blue`       : `#0000FF`
        Color::Rgb(138, 43, 226),  // `BlueViolet` : `#8A2BE2`
        Color::Rgb(95, 158, 160),  // `CadetBlue`  : `#5F9EA0`
        Color::Rgb(210, 105, 30),  // `Chocolate`  : `#D2691E`
        Color::Rgb(255, 127, 80),  // `Coral`      : `#FF7F50`
        Color::Rgb(30, 144, 255),  // `DodgerBlue` : `#1E90FF`
        Color::Rgb(178, 34, 34),   // `Firebrick`  : `#B22222`
        Color::Rgb(218, 165, 32),  // `GoldenRod`  : `#DAA520`
        Color::Rgb(0, 128, 0),     // `Green`      : `#008000`
        Color::Rgb(255, 105, 180), // `HotPink`    : `#FF69B4`
        Color::Rgb(255, 69, 0),    // `OrangeRed`  : `#FF4500`
        Color::Rgb(255, 0, 0),     // `Red`        : `#FF0000`
        Color::Rgb(46, 139, 87),   // `SeaGreen`   : `#2E8B57`
        Color::Rgb(0, 255, 127),   // `SpringGreen`: `#00FF7F`
        Color::Rgb(173, 255, 47),  // `YellowGreen`: `#ADFF2F`
    ]
}

pub fn sensible_theme() -> Theme {
    Theme {
        shadow: false,
        borders: BorderStyle::None,
        palette: default_palette(),
    }
}

pub fn choose_color() -> Color {
    thread_local! { static COUNTER: AtomicUsize = AtomicUsize::new(0); }
    let n = std::thread::LocalKey::with(&COUNTER, move |c| c.fetch_add(1, Ordering::SeqCst));
    let max = DEFAULT_COLORS.len();
    DEFAULT_COLORS[(n + max - 1) % max]
}

pub fn default_palette() -> Palette {
    const_array! {
        pub PALETTE_COLORS ; PaletteColor [
            PaletteColor::Background,
            PaletteColor::Shadow,
            PaletteColor::View,
            PaletteColor::Primary,
            PaletteColor::Secondary,
            PaletteColor::Tertiary,
            PaletteColor::TitlePrimary,
            PaletteColor::TitleSecondary,
            PaletteColor::Highlight,
            PaletteColor::HighlightInactive,
            PaletteColor::HighlightText,
        ]
    }

    <_>::into_iter(PALETTE_COLORS)
        .zip(std::iter::repeat(Color::TerminalDefault))
        .fold(Palette::default(), |mut p, (k, v)| {
            p[k] = v;
            p
        })
}
