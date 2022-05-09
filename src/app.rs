use crate::{Page, Repainter, Runtime};
use eframe::egui::CentralPanel;

pub struct WindowOptions {
    /// The window title
    pub title: String,
}

impl WindowOptions {
    /// Convert these options into the native options along with the title
    fn collapse(self) -> (String, eframe::NativeOptions) {
        (self.title, Default::default())
    }
}

impl Default for WindowOptions {
    fn default() -> Self {
        WindowOptions {
            title: env!("CARGO_PKG_NAME").to_string(),
        }
    }
}

pub struct UpdateContext<'u> {
    pub runtime: &'u Runtime,
    pub repainter: &'u Repainter,
    pub ctx: &'u eframe::egui::Context,
    pub frame: &'u mut eframe::Frame,
    pub ui: &'u mut eframe::egui::Ui,
}

pub trait App {
    fn initial_page(&mut self) -> Box<dyn Page>;
}

pub(crate) struct Application {
    app: Box<dyn App>,
    page: Box<dyn Page>,
    repainter: Repainter,
    runtime: Runtime,
    error: Option<Box<dyn ::std::error::Error>>,
}

impl Application {
    pub fn run<A: App + 'static>(app: A, options: WindowOptions) -> ! {
        let mut app: Box<dyn App> = Box::new(app);
        let (app_name, native_options) = options.collapse();

        eframe::run_native(
            &app_name,
            native_options,
            Box::new(|ctx| {
                let repainter = Repainter::new(ctx.egui_ctx.clone());
                let runtime =
                    Runtime::new(repainter.clone()).expect("unable to start async runtime");

                let application = Application {
                    page: app.initial_page(),
                    app,
                    repainter,
                    runtime,
                    error: None,
                };

                let name = application.page.name();
                info!("Starting with page: {:?}", name);

                Box::new(application)
            }),
        )
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let name = self.page.name();
            let _span = span!(tracing::Level::DEBUG, "{}", name).entered();

            let ctx = UpdateContext {
                ctx,
                frame,
                repainter: &self.repainter,
                runtime: &self.runtime,
                ui,
            };

            let res = self.page.render(ctx);
            match res {
                Ok(page) => {
                    if let Some(page) = page {
                        info!("Switched to page: {:?}", page.name());
                        self.page = page;
                    }
                }
                Err(err) => {
                    error!("{:?}", err);
                    self.error = Some(err)
                }
            };
        });
    }
}
