use cursive::{views::BoxedView, Cursive, View};

pub trait OnView<'c>: Sized + 'c {
    type View: View;

    fn with(cursive: &'c mut Cursive) -> Self;
    fn cursive(&mut self) -> &mut Cursive;
    fn name() -> &'static str;

    #[track_caller]
    fn on<F, R>(&mut self, callback: F) -> R
    where
        F: FnOnce(&mut Self::View) -> R,
    {
        let name = Self::name();
        self.cursive()
            .call_on_name(name, |bview: &mut BoxedView| {
                callback(bview.downcast_mut::<Self::View>().unwrap_or_else(|| {
                    let ty = std::any::type_name::<Self::View>();
                    panic!("the wrong type was supplied for {}: {}", name, ty)
                }))
            })
            .unwrap_or_else(|| panic!("cannot find name: {}", name))
    }
}

// TODO get rid of this macro
#[macro_export]
macro_rules! on_view {
    ($name:ident => $view:ty) => {
        impl<'c> $crate::ui::OnView<'c> for $name<'c> {
            type View = $view;
            fn with(cursive: &'c mut Cursive) -> Self {
                Self(cursive)
            }
            fn cursive(&mut self) -> &mut Cursive {
                &mut self.0
            }
            fn name() -> &'static str {
                static NAME: once_cell::sync::Lazy<String> =
                    once_cell::sync::Lazy::new($crate::ui::next_unique_name);
                &*NAME
            }
        }
    };
}
