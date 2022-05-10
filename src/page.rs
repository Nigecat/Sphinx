use crate::UpdateContext;

#[macro_export]
macro_rules! ok {
    () => {
        return Ok(None);
    };
}

#[macro_export]
macro_rules! switch_to_page {
    ($page: ident) => {
        return ::std::result::Result::Ok(::std::option::Option::Some(::std::boxed::Box::new(
            $page,
        )));
    };
}

pub type Switch = Result<Option<Box<dyn Page>>, Box<dyn ::std::error::Error>>;

pub trait Page {
    /// The name of the page, this is used for error reporting and logging
    fn name(&self) -> &str;

    /// What to do if a render method on this page returns an error.
    /// This will be called **after** the user has clicked `Ok` on the error prompt.
    /// Note that returning an error from this function could lead to an infinite loop and should be dealt with accordingly.
    fn on_error(&mut self, _err: Box<dyn std::error::Error>) -> Switch {
        Ok(None)
    }

    fn top(&mut self, _ctx: UpdateContext) -> Switch {
        Ok(None)
    }

    fn render(&mut self, _ctx: UpdateContext) -> Switch {
        Ok(None)
    }

    fn bottom(&mut self, _ctx: UpdateContext) -> Switch {
        Ok(None)
    }
}
