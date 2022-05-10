#[allow(unused_imports)] // used for doc comments
use crate::Page;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
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

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
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
