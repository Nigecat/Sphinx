//! An iterator adapter to display a progress bar.

use eframe::egui::{ProgressBar, Spinner};
use std::sync::mpsc;

/// The user interface of the progress widget.
/// This can either be rendered as a [`ProgressBar`] or [`Spinner`].
pub struct ProgressUi {
    repainter: crate::Repainter,
    progress: f32,
    update: mpsc::Receiver<f32>,
}

impl ProgressUi {
    /// Check the update channel for any updates and update the internal progress counter.
    fn update_progress(&mut self) {
        // We must repaint every frame so the mpsc channel event actually fires
        self.repainter.request_repaint();

        if let Ok(progress) = self.update.try_recv() {
            self.progress = progress;
        }
    }

    /// Get the progress indicator as a progress bar, this will sit on 100% once complete.
    pub fn cbar(&mut self) -> ProgressBar {
        // No point updating after iterator is complete
        if !self.complete() {
            self.update_progress();
        }
        ProgressBar::new(self.progress)
    }

    /// Get the progress indicator as a progress bar, this will return `None` once the iterator is complete.
    pub fn bar(&mut self) -> Option<ProgressBar> {
        self.update_progress();
        match self.complete() {
            true => None,
            false => Some(ProgressBar::new(self.progress)),
        }
    }

    /// Get the progress indicator as a spinner, this will return `None` once the iterator is complete.
    pub fn spinner(&mut self) -> Option<Spinner> {
        self.update_progress();
        match self.complete() {
            true => None,
            false => Some(Spinner::new()),
        }
    }

    /// Check if the progress bar is at 100%
    pub fn complete(&mut self) -> bool {
        (self.progress - 1.0).abs() < f32::EPSILON
    }
}

/// The iterator adapter for a [`ProgressUi`].
/// Give this to [`ShowProgress::show_progress`] to be able to render the progress state.
pub struct ProgressAdapter(mpsc::Sender<f32>);

/// An iterator which sends its position to a [`ProgressUi`].
pub struct ProgressView<I: Iterator> {
    length: usize,
    inner: I,
    update: mpsc::Sender<f32>,
}

/// Create a matching [`ProgressUi`] and [`ProgressAdapter`].
#[must_use]
pub fn create(repainter: &crate::Repainter) -> (ProgressUi, ProgressAdapter) {
    let (sender, receiver) = mpsc::channel::<f32>();
    (
        ProgressUi {
            repainter: repainter.clone(),
            progress: 0.0,
            update: receiver,
        },
        ProgressAdapter(sender),
    )
}

/// Show a progress bar for the given iterator.
pub trait ShowProgress<I: Iterator> {
    /// Bind the given adapter to this progress view.
    fn show_progress(self, adapter: ProgressAdapter) -> ProgressView<I>;
}

impl<I: Iterator> Iterator for ProgressView<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let (current, _) = self.inner.size_hint();

        if self.length > 0 {
            let position = (self.length - current) as f32 / self.length as f32;
            if let Err(err) = self.update.send(position) {
                error!("Unable to send update to progress bar: {:?}", err);
            }
        }

        self.inner.next()
    }
}

impl<I: Iterator> ShowProgress<I> for I {
    fn show_progress(self, adapter: ProgressAdapter) -> ProgressView<I> {
        if self.size_hint().1.is_none() {
            warn!("Unable to detect upper bound for progress iterator, progress bar will not be work (spinner will work)");
        }

        ProgressView {
            length: self.size_hint().1.unwrap_or(0),
            inner: self,
            update: adapter.0,
        }
    }
}
