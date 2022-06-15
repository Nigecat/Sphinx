use crate::{Response, TextBuffer, Ui, Vec2, Widget};

pub trait Sealed {}

impl Sealed for Ui {}

/// Extends a [`crate::Ui`].
pub trait UiExt: Sealed {
    /// Add an element taking up [`Ui::available_height`] and `width`.
    fn add_with_width(&mut self, width: f32, widget: impl Widget) -> Response;

    /// Add a square element, with side length equal to [`Ui::available_height`].
    fn add_square(&mut self, widget: impl Widget) -> Response;

    /// Add an element taking up [`Ui::available_size`].
    fn add_max(&mut self, widget: impl Widget) -> Response;

    /// Add a [`crate::widgets::TextEdit`] taking up [`Ui::available_width`].
    fn text_edit_single_line_width<S>(&mut self, width: f32, text: &mut S) -> Response
    where
        S: TextBuffer;

    /// Add an element using both [`UiExt::add_with_width`] and [`Ui::add_enabled`].
    fn add_enabled_with_width(
        &mut self,
        enabled: bool,
        width: f32,
        widget: impl Widget,
    ) -> Response;

    /// Add an element using both [`Ui::add_sized`] and [`Ui::add_enabled`].
    fn add_enabled_with_size(
        &mut self,
        enabled: bool,
        size: impl Into<Vec2>,
        widget: impl Widget,
    ) -> Response;
}

impl UiExt for Ui {
    fn add_with_width(&mut self, width: f32, widget: impl Widget) -> Response {
        self.add_sized(Vec2::new(width, self.available_height()), widget)
    }

    fn add_square(&mut self, widget: impl Widget) -> Response {
        let height = self.available_height();
        self.add_sized(Vec2::new(height, height), widget)
    }

    fn add_max(&mut self, widget: impl Widget) -> Response {
        self.add_sized(self.available_size(), widget)
    }

    fn text_edit_single_line_width<S>(&mut self, width: f32, text: &mut S) -> Response
    where
        S: TextBuffer,
    {
        self.add_sized(
            Vec2::new(width, self.spacing().interact_size.y),
            crate::widgets::TextEdit::singleline(text),
        )
    }

    fn add_enabled_with_width(
        &mut self,
        enabled: bool,
        width: f32,
        widget: impl Widget,
    ) -> Response {
        self.add_enabled_with_size(enabled, Vec2::new(width, self.available_height()), widget)
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
