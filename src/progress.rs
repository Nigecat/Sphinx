//! An iterator adapter to display a progress bar.

use eframe::egui::ProgressBar;
use std::sync::mpsc;

pub struct ProgressUi {
    progress: f32,
    update: mpsc::Receiver<f32>,
}

impl ProgressUi {
    pub fn bar(&mut self) -> ProgressBar {
        if let Ok(progress) = self.update.try_recv() {
            self.progress = progress;
        }

        ProgressBar::new(self.progress)
    }

    /// Check if the progress bar is at 100%
    pub fn complete(&mut self) -> bool {
        self.progress == 1.0
    }
}

pub struct ProgressAdapter(mpsc::Sender<f32>);

pub struct ProgressView<I: Iterator> {
    length: usize,
    inner: I,
    update: mpsc::Sender<f32>,
}

pub fn create() -> (ProgressUi, ProgressAdapter) {
    let (sender, receiver) = mpsc::channel::<f32>();
    (
        ProgressUi {
            progress: 0.0,
            update: receiver,
        },
        ProgressAdapter(sender),
    )
}

pub trait ShowProgress<I: Iterator> {
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
            warn!("Unable to detect upper bound for progress iterator, progress bar will not be visible");
        }

        ProgressView {
            length: self.size_hint().1.unwrap_or(0),
            inner: self,
            update: adapter.0,
        }
    }
}
