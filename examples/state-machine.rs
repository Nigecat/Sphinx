use sphinx::progress::ShowProgress as _;
use sphinx::{UpdateContext, Widget};

struct App;

impl sphinx::App for App {
    fn initial_page(&mut self) -> Box<dyn sphinx::Page> {
        Box::new(Page::default())
    }
}

enum Page {
    WaitingForInput,
    Calculating((sphinx::progress::ProgressUi, sphinx::oneshot::Receiver<u64>)),
    Complete(u64),
}

impl Default for Page {
    fn default() -> Self {
        Page::WaitingForInput
    }
}

impl sphinx::Page for Page {
    fn name(&self) -> &str {
        "example-state-machine"
    }

    fn render(&mut self, ctx: UpdateContext) -> sphinx::Switch {
        let UpdateContext {
            ui,
            runtime,
            repainter,
            ..
        } = ctx;

        match self {
            Page::WaitingForInput => {
                if ui.button("Click Me!").clicked() {
                    let (progress, adapter) = sphinx::progress::create(repainter);

                    let proc = runtime.execute_oneshot(async {
                        // A very slow method of totalling the first 1000 integers

                        let mut sum: u64 = 0;

                        for i in (0..1000).into_iter().show_progress(adapter) {
                            sum += i;
                            std::thread::sleep(std::time::Duration::from_millis(10));
                        }

                        sum
                    });

                    *self = Page::Calculating((progress, proc));
                }
            }
            Page::Calculating((view, rec)) => {
                if let Some(bar) = view.bar() {
                    bar.ui(ui);
                }

                if let Ok(Some(sum)) = rec.try_recv() {
                    *self = Page::Complete(sum);
                }
            }
            Page::Complete(sum) => {
                ui.label(format!("Sum (0..1000): {sum}"));
            }
        };

        sphinx::ok!();
    }
}

fn main() {
    sphinx::run(App, sphinx::WindowOptions::single());
}
