use crate::{Page, Repainter, Runtime, Theme, View};
use eframe::egui::{CentralPanel, TopBottomPanel, Ui};

/// The options to create the application window with.
pub struct WindowOptions {
    /// The window title
    pub title: String,

    /// The initial theme to use, defaults to [`Theme::Dark`].
    pub theme: Theme,

    /// Disable the top bar ([`Page::top`]), useful if you are not displaying anything there.
    ///
    /// This can be changed at runtime with [`View::enable_top`] / [`View::disable_top`].
    pub disable_top: bool,

    /// Disable the bottom bar ([`Page::bottom`]), useful if you are not displaying anything there.
    ///
    /// This can be changed at runtime with [`View::enable_bottom`] / [`View::disable_bottom`].
    pub disable_bottom: bool,

    /// The native integration options.
    pub native: crate::NativeOptions,
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
            ..View::default()
        };
        (self.title, self.native, view)
    }
}

impl Default for WindowOptions {
    fn default() -> Self {
        WindowOptions {
            title: env!("CARGO_PKG_NAME").to_string(),
            disable_top: false,
            disable_bottom: false,
            theme: Theme::default(),
            native: eframe::NativeOptions::default(),
        }
    }
}

/// The data given when the renderer must provide an update.
pub struct UpdateContext<'u> {
    /// The async runtime.
    pub runtime: &'u Runtime,
    /// An object capable of requesting a repaint.
    pub repainter: &'u Repainter,
    /// The internal render context.
    pub ctx: &'u eframe::egui::Context,
    /// The render frame.
    pub frame: &'u mut eframe::Frame,
    /// The ui to render into.
    pub ui: &'u mut Ui,
    /// The application view.
    pub view: &'u mut View,
}

/// An application.
pub trait App {
    /// The inital page the application should open.
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
    fn process(&mut self, switch: crate::Switch) {
        match switch {
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
    }

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

                info!("Using {:?} theme", view.theme);
                ctx.egui_ctx.set_visuals(view.theme.visuals());

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
                self.process(res);
            }};
        }

        if let Some(ref err) = self.error {
            let err = err.to_string();
            eframe::egui::Window::new("Error").show(ctx, |ui| {
                ui.label(err.to_string());
                if ui.button("Ok").clicked() {
                    if let Some(err) = self.error.take() {
                        let res = self.page.on_error(err);
                        self.process(res);
                    }
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
