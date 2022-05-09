use crate::{Repainter, Runtime};

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
}

pub trait App {
    fn update(&mut self, ctx: UpdateContext);
}

pub(crate) struct Application {
    app: Box<dyn App>,
    repainter: Repainter,
    runtime: Runtime,
}

impl Application {
    pub fn run<A: App + 'static>(app: A, options: WindowOptions) -> ! {
        let app: Box<dyn App> = Box::new(app);
        let (app_name, native_options) = options.collapse();

        eframe::run_native(
            &app_name,
            native_options,
            Box::new(|ctx| {
                let repainter = Repainter::new(ctx.egui_ctx.clone());
                let runtime =
                    Runtime::new(repainter.clone()).expect("unable to start async runtime");

                let application = Application {
                    app,
                    repainter,
                    runtime,
                };

                Box::new(application)
            }),
        )
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        let ctx = UpdateContext {
            ctx,
            frame,
            repainter: &self.repainter,
            runtime: &self.runtime,
        };
        self.app.update(ctx);
    }
}
