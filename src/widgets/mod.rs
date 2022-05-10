//! An assorted collection of useful widgets.

#[cfg(feature = "animation")]
mod animation;
#[cfg(feature = "animation")]
pub use animation::AnimationPlayer;

pub use eframe::egui::*;
