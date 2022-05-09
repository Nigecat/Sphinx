#[allow(unused_imports)] // used for doc comments
use crate::Page;

/// An interface to update the view of the app state at runtime.
pub struct View {
    /// Whether the top bar is enabled
    pub(crate) top_enabled: bool,
    /// Whether the bottom bar is enabled
    pub(crate) bottom_enabled: bool,
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
}
