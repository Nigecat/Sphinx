//! A thin wrapper around egui to provide basic page switching and other utilities along with an async runtime.

#[macro_use]
extern crate tracing;

mod app;
mod repaint;
mod runtime;

pub use app::{App, UpdateContext, WindowOptions};
pub use repaint::Repainter;
pub use runtime::Runtime;

pub use eframe::egui::emath as math;
pub use eframe::egui::epaint as paint;
pub use eframe::egui::Context;
pub use eframe::glow;
pub use eframe::Frame;

pub use thiserror::Error;

pub fn run<A: App + 'static>(app: A, options: WindowOptions) -> ! {
    app::Application::run(app, options)
}
