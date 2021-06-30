use cursive::{views::HideableView, Cursive, View};

use crate::{get_config_mut, Config};

pub trait FindView {
    fn seek_and_toggle<T, F>(&mut self, key: &str, extract: F)
    where
        T: View,
        F: Fn(&mut Config) -> &mut bool;
}

impl FindView for Cursive {
    fn seek_and_toggle<T, F>(&mut self, key: &str, extract: F)
    where
        T: View,
        F: Fn(&mut Config) -> &mut bool,
    {
        let show = {
            let config = &mut *get_config_mut();
            let show = extract(config);
            *show = !*show;
            *show
        };
        self.call_on_all_named(key, |view: &mut HideableView<T>| view.set_visible(show))
    }
}
