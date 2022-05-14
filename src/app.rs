#[cfg(feature = "runtime")]
use crate::Runtime;
use crate::{Page, Repainter, Theme, View};
use eframe::egui::{CentralPanel, TopBottomPanel, Ui};
use std::any::Any;

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
            theme: self.theme,
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
///
/// This should be destructured to get the specific data you need for a render, ensure to use the `..` syntax as this object will likely expand in future releases.  
/// For example:
/// ```rust
/// use sphinx::UpdateContext;
///
/// /* in sphinx::Page implementation*/
/// fn render(&mut self, ctx: UpdateContext) -> sphinx::Switch {
///     let UpdateContext { ui, .. } = ctx;
///     ui.label("Hello, World!");
///     sphinx::ok!();
/// }
/// ```
///
/// Additionally, state can be fetched with the [`crate::use_state`] macro.
/// ```rust
/// use sphinx::{UpdateContext, use_state};
///
/// #[derive(Default)]
/// struct State {
///     counter: usize,
/// }
///
/// // ..
///
/// /* in sphinx::Page implementation */
/// #[use_state]
/// fn render(&mut self, ctx: UpdateContext, state: State) -> sphinx::Switch {
///     assert_eq!(state.counter, 0);
///     sphinx::ok!();
/// }
///
/// // ..
///
/// fn main() {
///     sphinx::run_with_state(/* app */, sphinx::WindowOptions::default(), State::default());
/// }
/// ```
/// Note that the type given to the `state` parameter of the `render` method **must** be the same type as the instance given to the [`crate::run_with_state`] method.
/// Doing otherwise will lead to a runtime panic when the page attempts to render.
pub struct UpdateContext<'u> {
    /// The async runtime.
    #[cfg(feature = "runtime")]
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
    /// The application.
    pub app: &'u mut Box<dyn App>,
    /// The application state.
    /// This should not be used directly. Use [`crate::use_state`] instead.
    ///
    /// This defaults to `()` if [`crate::run`] was used to start the app, otherwise it will be whatever `state` value was passed to [`crate::run_with_state`].
    #[doc(hidden)]
    pub state: &'u mut Box<dyn Any>,
}

/// An application.
pub trait App {
    /// The inital page the application should open.
    fn initial_page(&mut self) -> Box<dyn Page>;

    /// Called to save state, this is called before [`App::on_exit`].
    fn save(&mut self, _state: &Box<dyn Any>) {}

    /// Called on shutdown.
    fn on_exit(&mut self) {}
}

pub(crate) struct Application {
    /// Whether the init method has been run for the current page
    init: bool,
    app: Box<dyn App>,
    page: Box<dyn Page>,
    repainter: Repainter,
    #[cfg(feature = "runtime")]
    runtime: Runtime,
    view: View,
    error: Option<Box<dyn ::std::error::Error>>,
    state: Box<dyn Any>,
}

impl Application {
    fn process(&mut self, switch: crate::Switch) {
        match switch {
            Ok(page) => {
                if let Some(page) = page {
                    self.page.exit(&mut self.view);
                    debug!("Switched to page: {:?}", page.name());
                    self.page = page;
                    self.page.enter(&mut self.view);
                    self.init = false;
                    self.repainter.request_repaint();
                }
            }
            Err(err) => {
                error!("{:?}", err);
                self.error = Some(err)
            }
        };
    }

    pub fn run<A, S>(app: A, options: WindowOptions, state: S) -> !
    where
        A: App + 'static,
        S: Any,
    {
        let mut app: Box<dyn App> = Box::new(app);
        let (app_name, native_options, view) = options.collapse();

        eframe::run_native(
            &app_name,
            native_options,
            Box::new(|ctx| {
                let repainter = Repainter::new(ctx.egui_ctx.clone());
                #[cfg(feature = "runtime")]
                let runtime =
                    Runtime::new(repainter.clone()).expect("unable to start async runtime");

                debug!("Using {:?} theme", view.theme);
                ctx.egui_ctx.set_visuals(view.theme.visuals());

                let mut application = Application {
                    init: false,
                    page: app.initial_page(),
                    app,
                    repainter,
                    #[cfg(feature = "runtime")]
                    runtime,
                    error: None,
                    view,
                    state: Box::new(state),
                };

                application.page.enter(&mut application.view);

                let name = application.page.name();
                debug!("Starting with page: {:?}", name);

                Box::new(application)
            }),
        )
    }
}

impl eframe::App for Application {
    fn on_exit(&mut self, _gl: &eframe::glow::Context) {
        self.app.save(&self.state);
        self.app.on_exit();
    }

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        let name = self.page.name();
        let _span = span!(tracing::Level::DEBUG, "{}", name).entered();

        macro_rules! bind {
            ($ui: ident) => {
                UpdateContext {
                    ctx,
                    frame,
                    repainter: &self.repainter,
                    #[cfg(feature = "runtime")]
                    runtime: &self.runtime,
                    ui: $ui,
                    view: &mut self.view,
                    app: &mut self.app,
                    state: &mut self.state,
                }
            };

            ($ui: ident, $method: ident) => {{
                let ctx = bind!($ui);
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

        if !self.init {
            self.init = true;

            crate::Area::new("init")
                .fixed_pos(crate::Pos2::new(0.0, 0.0))
                .enabled(false)
                .movable(false)
                .interactable(false)
                .order(crate::raw::Order::Background)
                .show(ctx, |ui| {
                    let ctx = bind!(ui);
                    self.page.init(ctx);
                });

            return;
        }

        if self.view.top_enabled {
            TopBottomPanel::top("top").show(ctx, |ui| bind!(ui, top));
        }

        if self.view.bottom_enabled {
            TopBottomPanel::bottom("bottom").show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| bind!(ui, bottom))
            });
        }

        CentralPanel::default().show(ctx, |ui| bind!(ui, render));
    }
}
