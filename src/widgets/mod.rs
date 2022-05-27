//! An assorted collection of useful widgets.

#[cfg(feature = "animation")]
mod animation;
#[cfg(feature = "animation")]
pub use animation::AnimationPlayer;

#[cfg(feature = "datepicker")]
mod datepicker;
#[cfg(feature = "datepicker")]
pub use datepicker::DatePicker;

pub use eframe::egui::*;
