use eframe::egui::Widget;
use sphinx::progress::ShowProgress as _;
use sphinx::UpdateContext;

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
            progress.bar().ui(ui);
        }

        if !self.run {
            self.run = true;
            let repainter = repainter.clone();

            let (progress, adapter) = sphinx::progress::create();
            self.progress = Some(progress);
            runtime.execute(
                async move {
                    for _ in (0..10).into_iter().show_progress(adapter) {
                        ::std::thread::sleep(::std::time::Duration::from_secs(1));
                        println!("tick");
                        repainter.request_repaint();
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