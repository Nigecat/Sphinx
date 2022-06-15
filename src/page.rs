#[allow(unused_imports)] // used for doc comments
use crate::WindowOptions;
use crate::{UpdateContext, View};

/// Call this to indicate a page render executed successfully.
#[macro_export]
macro_rules! ok {
    () => {
        return Ok(None);
    };
}

/// Given an expression which results in a [`crate::Switch`], if the final value is anything other than `Ok(None)`, return it.
#[macro_export]
macro_rules! resolve {
    ($( $ex: expr ),* $(,)?) => {
        $(
            let __resolve_expr_result: ::sphinx::Switch = $ex;
            match __resolve_expr_result {
                Ok(None) => (),
                switch => return switch,
            }
        )*
    };
}

/// Switch to the given page, the given object must implement the [`Page`] trait.
#[macro_export]
macro_rules! switch_to_page {
    ($page: expr) => {
        let __switch_to_page_instance: ::std::boxed::Box<dyn ::sphinx::Page> =
            ::std::boxed::Box::new($page);
        return ::std::result::Result::Ok(::std::option::Option::Some(__switch_to_page_instance));
    };
}

/// The result of a page render.
///
/// Call [`switch_to_page`] to switch to a different page,
/// otherwise, [`ok`] should be called (note that any error can be returned from this to be displayed by the renderer).
pub type Switch = Result<Option<Box<dyn Page>>, Box<dyn ::std::error::Error>>;

/// A page capable of being rendered.
pub trait Page {
    /// The name of the page, this is used for error reporting and logging
    fn name(&self) -> &str;

    /// Called when the page is switched to, use this to update the view if necessary.
    fn enter(&mut self, _view: &mut View) {}

    /// Called when the page is switched off of, use this to undo [`Page::enter`] if necessary.
    fn exit(&mut self, _view: &mut View) {}

    /// What to do if a render method on this page returns an error.
    /// This will be called **after** the user has clicked `Ok` on the error prompt.
    /// Note that returning an error from this function could lead to an infinite loop and should be dealt with accordingly.
    fn on_error(&mut self, _err: Box<dyn std::error::Error>) -> Switch {
        Ok(None)
    }

    /// Run any local initialization with the given context.
    ///
    /// This is called after [`Page::enter`] and should not render any elements to the ui.
    fn init(&mut self, _ctx: UpdateContext) {}

    /// The top bar.
    ///
    /// This can be disabled at runtime with [`View::disable_top`] or at startup with [`WindowOptions::disable_top`].
    fn top(&mut self, _ctx: UpdateContext) -> Switch {
        Ok(None)
    }

    /// The main content section.
    fn render(&mut self, _ctx: UpdateContext) -> Switch {
        Ok(None)
    }

    /// The bottom bar.
    ///
    /// This can be disabled at runtime with [`View::disable_bottom`] or at startup with [`WindowOptions::disable_bottom`].
    fn bottom(&mut self, _ctx: UpdateContext) -> Switch {
        Ok(None)
    }
}
