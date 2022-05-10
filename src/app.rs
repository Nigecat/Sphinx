use crate::{Page, Repainter, Runtime, View};
use eframe::egui::{CentralPanel, TopBottomPanel, Ui};

/// The options to create the application window with.
pub struct WindowOptions {
    /// The window title
    pub title: String,

    /// Disable the top bar ([`Page::top`]), useful if you are not displaying anything there.
    ///
    /// This can be changed at runtime with [`View::enable_top`] / [`View::disable_top`].
    pub disable_top: bool,
    /// Disable the bottom bar ([`Page::bottom`]), useful if you are not displaying anything there.
    ///
    /// This can be changed at runtime with [`View::enable_bottom`] / [`View::disable_bottom`].
    pub disable_bottom: bool,
}

impl WindowOptions {
    /// Create a window with a single section ([`Page::render`]), this will disable both the top bar ([`Page::top`]) and bottom bar ([`Page::bottom`]).
    ///
    /// These can be enabled at runtime with [`View::enable_top`] / [`View::disable_top`] or [`View::enable_bottom`] / [`View::disable_bottom`] respectively.
    pub fn single() -> Self {
        WindowOptions {
            disable_top: true,
            disable_bottom: true,
            ..Self::default()
        }
    }

    /// Convert these options into the native options along with the title and view information
    fn collapse(self) -> (String, eframe::NativeOptions, View) {
        let view = View {
            top_enabled: !self.disable_top,
            bottom_enabled: !self.disable_bottom,
        };
        (self.title, Default::default(), view)
    }
}

impl Default for WindowOptions {
    fn default() -> Self {
        WindowOptions {
            title: env!("CARGO_PKG_NAME").to_string(),
            disable_top: false,
            disable_bottom: false,
        }
    }
}

/// The data given when the renderer must provide an update.
pub struct UpdateContext<'u> {
    pub runtime: &'u Runtime,
    pub repainter: &'u Repainter,
    pub ctx: &'u eframe::egui::Context,
    pub frame: &'u mut eframe::Frame,
    pub ui: &'u mut Ui,
    pub view: &'u mut View,
}

pub trait App {
    fn initial_page(&mut self) -> Box<dyn Page>;
}

pub(crate) struct Application {
    app: Box<dyn App>,
    page: Box<dyn Page>,
    repainter: Repainter,
    runtime: Runtime,
    view: View,
    error: Option<Box<dyn ::std::error::Error>>,
}

impl Application {
    pub fn run<A: App + 'static>(app: A, options: WindowOptions) -> ! {
        let mut app: Box<dyn App> = Box::new(app);
        let (app_name, native_options, view) = options.collapse();

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
                    view,
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
        let name = self.page.name();
        let _span = span!(tracing::Level::DEBUG, "{}", name).entered();

        macro_rules! bind {
            ($ui: ident, $method: ident) => {{
                let ctx = UpdateContext {
                    ctx,
                    frame,
                    repainter: &self.repainter,
                    runtime: &self.runtime,
                    ui: $ui,
                    view: &mut self.view,
                };

                let res = self.page.$method(ctx);
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
            }};
        }

        if let Some(ref err) = self.error {
            let err = err.to_string();
            eframe::egui::Window::new("Error").show(ctx, |ui| {
                ui.label(&format!("{}", err));
                if ui.button("Ok").clicked() {
                    self.error = None;
                }
            });

            // Create blank pannels to ensure ui does not shift (this additionally ensures background colour is correct)
            if self.view.top_enabled {
                TopBottomPanel::top("top").show(ctx, |_| {});
            }

            if self.view.bottom_enabled {
                TopBottomPanel::bottom("bottom").show(ctx, |_| {});
            }

            CentralPanel::default().show(ctx, |_| {});

            return;
        }

        if self.view.top_enabled {
            TopBottomPanel::top("top").show(ctx, |ui| bind!(ui, top));
        }

        if self.view.bottom_enabled {
            TopBottomPanel::bottom("bottom").show(ctx, |ui| bind!(ui, bottom));
        }

        CentralPanel::default().show(ctx, |ui| bind!(ui, render));
    }
}
