//! A thin wrapper around egui to provide basic page switching and other utilities along with an async runtime.

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::float_cmp)]
#![warn(clippy::trivially_copy_pass_by_ref)]
#![warn(clippy::integer_division, clippy::integer_arithmatic)]

#[macro_use]
extern crate tracing;

mod app;
mod page;
pub mod progress;
mod repaint;
mod runtime;
mod view;
pub mod widgets;

pub use app::{App, UpdateContext, WindowOptions};
pub use page::{Page, Switch};
pub use repaint::Repainter;
pub use runtime::Runtime;
pub use view::{Theme, View};

#[doc(hidden)]
pub use eframe::egui as raw;
pub use eframe::egui::emath as math;
pub use eframe::egui::epaint as paint;
pub use eframe::egui::{Color32, Context, Event, Key, Layout, Pos2, Rect, Sense, Ui, Vec2, Widget};
pub use eframe::{glow, Frame, IconData, NativeOptions};

pub use thiserror::Error;

/// Start a window with the given application and options.
pub fn run<A: App + 'static>(app: A, options: WindowOptions) -> ! {
    app::Application::run(app, options)
}
