use crate::UpdateContext;

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

    fn render(&mut self, ctx: UpdateContext) -> Switch;
}
