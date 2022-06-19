//! A thin wrapper around egui to provide basic page switching and other utilities along with an async runtime.
//!
//! # Usage
//! Create a struct which implements [`App`] then call [`run`].

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::float_cmp)]
#![warn(clippy::trivially_copy_pass_by_ref)]

#[macro_use]
extern crate tracing;

mod app;
pub mod ext;
mod page;
pub mod progress;
mod repaint;
#[cfg(feature = "runtime")]
mod runtime;
pub mod utils;
mod view;
pub mod widgets;

pub use app::{App, UpdateContext, WindowOptions};
pub use page::{Page, Switch};
pub use repaint::Repainter;
#[cfg(feature = "runtime")]
pub use runtime::{oneshot, Runtime};
pub use view::{Theme, View};

#[doc(hidden)]
pub use eframe::egui as raw;
pub use eframe::egui::emath as math;
pub use eframe::egui::epaint as paint;
pub use eframe::egui::menu;
pub use eframe::egui::{Area, CentralPanel, SidePanel, TopBottomPanel, Window};
pub use eframe::egui::{Color32, ComboBox, TextBuffer, Ui, Widget};
pub use eframe::egui::{Context, Event, Key, Layout, Response, Sense};
pub use eframe::egui::{Pos2, Rect, Vec2};
pub use eframe::{glow, Frame, IconData, NativeOptions};

pub use sphinx_use_state::use_state;
pub use thiserror::Error;

/// Start a window with the given application and options.
pub fn run<A: App + 'static>(app: A, options: WindowOptions) -> ! {
    app::Application::run(app, options, ())
}

/// Start a window with the given application and options, an additional state can be passed which will be exposed as [`UpdateContext::state`].
pub fn run_with_state<A, S>(app: A, options: WindowOptions, state: S) -> !
where
    A: App + 'static,
    S: std::any::Any,
{
    app::Application::run(app, options, state)
}
