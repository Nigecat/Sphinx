use eframe::egui::Context;

/// A threadsafe object capable of requesting a repaint to the main ui thread
#[derive(Clone)]
pub struct Repainter {
    ctx: Context,
}

impl Repainter {
    pub(crate) fn new(ctx: Context) -> Self {
        Repainter { ctx }
    }

    /// Request a repaint to the renderer.
    pub fn request_repaint(&self) {
        self.ctx.request_repaint();
    }
}

impl From<Context> for Repainter {
    fn from(ctx: Context) -> Self {
        Repainter::new(ctx)
    }
}
