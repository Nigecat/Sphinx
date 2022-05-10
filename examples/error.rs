struct ErrorExample;

impl sphinx::App for ErrorExample {
    fn initial_page(&mut self) -> Box<dyn sphinx::Page> {
        Box::new(ErrorExamplePage)
    }
}

struct ErrorExamplePage;

impl sphinx::Page for ErrorExamplePage {
    fn name(&self) -> &str {
        "example-error"
    }

    fn render(&mut self, _ctx: sphinx::UpdateContext) -> sphinx::Switch {
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "demonstration error",
        ))?;
        sphinx::ok!();
    }
}

fn main() {
    tracing_subscriber::fmt().init();
    sphinx::run(ErrorExample, sphinx::WindowOptions::default());
}
