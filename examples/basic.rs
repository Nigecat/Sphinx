use sphinx::UpdateContext;

struct App;

impl sphinx::App for App {
    fn initial_page(&mut self) -> Box<dyn sphinx::Page> {
        Box::new(PageA::default())
    }
}

struct PageA;

impl sphinx::Page for PageA {
    fn name(&self) -> &str {
        "demo-page-a"
    }

    fn render(&mut self, ctx: UpdateContext) -> sphinx::Switch {
        let UpdateContext { ui, .. } = ctx;
        ui.label("Page A");

        if ui.button("Switch").clicked() {
            sphinx::switch_to_page!(PageB);
        }

        Ok(None)
    }
}

impl Default for PageA {
    fn default() -> Self {
        PageA
    }
}

struct PageB;

impl sphinx::Page for PageB {
    fn name(&self) -> &str {
        "demo-page-b"
    }

    fn render(&mut self, ctx: UpdateContext) -> sphinx::Switch {
        let UpdateContext { ui, .. } = ctx;
        ui.label("Page B");

        if ui.button("Switch").clicked() {
            sphinx::switch_to_page!(PageA);
        }

        Ok(None)
    }
}

fn main() {
    tracing_subscriber::fmt().init();
    sphinx::run(App, sphinx::WindowOptions::single());
}
