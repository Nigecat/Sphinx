use sphinx::progress::ShowProgress as _;
use sphinx::{UpdateContext, Widget};

struct App;

impl sphinx::App for App {
    fn initial_page(&mut self) -> Box<dyn sphinx::Page> {
        Box::new(Page {
            run: false,
            progress: None,
        })
    }
}

struct Page {
    run: bool,
    progress: Option<sphinx::progress::ProgressUi>,
}

impl sphinx::Page for Page {
    fn name(&self) -> &str {
        "example-progress"
    }

    fn render(&mut self, ctx: UpdateContext) -> sphinx::Switch {
        let UpdateContext {
            runtime,
            ui,
            repainter,
            ..
        } = ctx;

        if let Some(ref mut progress) = self.progress {
            if progress.complete() && ui.button("Reset").clicked() {
                self.run = false;
            }

            if let Some(bar) = progress.bar() {
                bar.ui(ui);
            }

            ui.separator();

            if let Some(spinner) = progress.spinner() {
                spinner.ui(ui);
            }
        }

        if !self.run {
            self.run = true;

            let (progress, adapter) = sphinx::progress::create(repainter);
            self.progress = Some(progress);
            runtime.execute(
                async move {
                    for i in (0..100).into_iter().show_progress(adapter) {
                        ::std::thread::sleep(::std::time::Duration::from_millis(25));
                        println!("{}", i);
                    }
                },
                |_| println!("done!"),
            );
        }

        sphinx::ok!();
    }
}

fn main() {
    tracing_subscriber::fmt().init();
    sphinx::run(App, sphinx::WindowOptions::single());
}
