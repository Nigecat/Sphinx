use sphinx::{Theme, UpdateContext};

struct App;

impl sphinx::App for App {
    fn initial_page(&mut self) -> Box<dyn sphinx::Page> {
        Box::new(Page)
    }
}

struct Page;

impl sphinx::Page for Page {
    fn name(&self) -> &str {
        "example-theme"
    }

    fn render(&mut self, ctx: UpdateContext) -> sphinx::Switch {
        let UpdateContext { ui, ctx, view, .. } = ctx;

        if ui.button("Switch Theme").clicked() {
            let current = view.current_theme();
            let new = match current {
                Theme::Dark => Theme::Light,
                Theme::Light => Theme::Dark,
                _ => unreachable!(),
            };
            view.set_theme(new, ctx);
        }

        sphinx::ok!();
    }
}

fn main() {
    tracing_subscriber::fmt().init();
    sphinx::run(App, sphinx::WindowOptions::single());
}
