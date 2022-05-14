use crate::{Response, Ui, Vec2, Widget};

pub trait Sealed {}

impl Sealed for Ui {}

/// Extends a [`crate::Ui`].
pub trait UiExt: Sealed {
    /// Add an element taking up [`Ui::available_size`].
    fn add_max(&mut self, widget: impl Widget) -> Response;

    /// Add an element using both [`Ui::add_sized`] and [`Ui::add_enabled`].
    fn add_enabled_with_size(
        &mut self,
        enabled: bool,
        size: impl Into<Vec2>,
        widget: impl Widget,
    ) -> Response;
}

impl UiExt for Ui {
    fn add_max(&mut self, widget: impl Widget) -> Response {
        self.add_sized(self.available_size(), widget)
    }

    fn add_enabled_with_size(
        &mut self,
        enabled: bool,
        size: impl Into<Vec2>,
        widget: impl Widget,
    ) -> Response {
        if self.is_enabled() && !enabled {
            self.set_enabled(false);
            let response = self.add_sized(size, widget);
            self.set_enabled(true);
            response
        } else {
            self.add_sized(size, widget)
        }
    }
}
