use sphinx::UpdateContext;

#[derive(Default)]
struct State {
    counter: usize,
}

struct App;

impl sphinx::App for App {
    fn initial_page(&mut self) -> Box<dyn sphinx::Page> {
        Box::new(PageA)
    }
}

struct PageA;

impl sphinx::Page for PageA {
    fn name(&self) -> &str {
        "example-shared-state-pagea"
    }

    #[sphinx::use_state]
    fn render(&mut self, ctx: UpdateContext, state: State) -> sphinx::Switch {
        let UpdateContext { ui, .. } = ctx;

        ui.label("----- Page A -----");
        ui.label(format!("Counter: {}", state.counter));

        if ui.button("boop").clicked() {
            state.counter += 1;
            sphinx::switch_to_page!(PageB);
        }

        sphinx::ok!();
    }
}

struct PageB;

impl sphinx::Page for PageB {
    fn name(&self) -> &str {
        "example-shared-state-pageb"
    }

    #[sphinx::use_state]
    fn render(&mut self, ctx: UpdateContext, state: State) -> sphinx::Switch {
        let UpdateContext { ui, .. } = ctx;

        ui.label("----- Page B -----");
        ui.label(format!("Counter: {}", state.counter));

        if ui.button("boop").clicked() {
            state.counter += 1;
            sphinx::switch_to_page!(PageA);
        }

        sphinx::ok!();
    }
}

fn main() {
    sphinx::run_with_state(App, sphinx::WindowOptions::single(), State::default());
}
