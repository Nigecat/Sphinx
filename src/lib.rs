//! A thin wrapper around egui to provide basic page switching and other utilities along with an async runtime.

#![forbid(unsafe_code)]

#[macro_use]
extern crate tracing;

mod app;
mod page;
pub mod progress;
mod repaint;
mod runtime;
mod view;

pub use app::{App, UpdateContext, WindowOptions};
pub use page::{Page, Switch};
pub use repaint::Repainter;
pub use runtime::Runtime;
pub use view::{Theme, View};

pub use eframe::egui as raw;
pub use eframe::egui::emath as math;
pub use eframe::egui::epaint as paint;
pub use eframe::egui::{Context, Layout, Pos2, Rect, Vec2};
pub use eframe::{glow, Frame, IconData, NativeOptions};

pub use thiserror::Error;

pub fn run<A: App + 'static>(app: A, options: WindowOptions) -> ! {
    app::Application::run(app, options)
}
