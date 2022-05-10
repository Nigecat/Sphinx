#[allow(unused_imports)] // used for doc comments
use crate::Page;

/// A visual theme.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    /// The light theme.
    Light,
    /// The dark theme.
    Dark,
}

impl Theme {
    pub(crate) fn visuals(&self) -> eframe::egui::Visuals {
        match self {
            Theme::Dark => eframe::egui::Visuals::dark(),
            Theme::Light => eframe::egui::Visuals::light(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Dark
    }
}

/// An interface to update the view of the app state at runtime.
pub struct View {
    /// Whether the top bar is enabled
    pub(crate) top_enabled: bool,
    /// Whether the bottom bar is enabled
    pub(crate) bottom_enabled: bool,
    /// The current theme
    pub(crate) theme: Theme,
}

impl View {
    /// Enable the top bar ([`Page::top`]).
    pub fn enable_top(&mut self) {
        self.top_enabled = true;
    }

    /// Disable the top bar ([`Page::top`]).
    pub fn disable_top(&mut self) {
        self.top_enabled = false;
    }

    /// Enable the top bar ([`Page::bottom`]).
    pub fn enable_bottom(&mut self) {
        self.bottom_enabled = true;
    }

    /// Disable the top bar ([`Page::bottom`]).
    pub fn disable_bottom(&mut self) {
        self.bottom_enabled = false;
    }

    /// Get the current theme.
    pub fn current_theme(&self) -> Theme {
        self.theme
    }

    /// Update the current theme.
    pub fn set_theme(&mut self, theme: Theme, ctx: &eframe::egui::Context) {
        self.theme = theme;
        ctx.set_visuals(theme.visuals());
    }
}

impl Default for View {
    fn default() -> Self {
        View {
            top_enabled: true,
            bottom_enabled: true,
            theme: Theme::default(),
        }
    }
}
