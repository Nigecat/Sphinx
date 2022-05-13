use crate::Repainter;
pub use futures::channel::oneshot;
use std::future::Future;

/// The async runtime.
pub struct Runtime {
    repainter: Repainter,
    inner: tokio::runtime::Runtime,
}

impl Runtime {
    pub(crate) fn new(repainter: Repainter) -> Result<Self, std::io::Error> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;

        Ok(Runtime {
            repainter,
            inner: runtime,
        })
    }

    /// Execute the given future in the async runtime.
    pub fn execute<F, C>(&self, future: F, on_complete: C)
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
        C: FnOnce(F::Output) + Send + 'static,
    {
        let repainter = self.repainter.clone();
        self.inner.spawn(async move {
            let res = future.await;
            on_complete(res);
            repainter.request_repaint();
        });
    }

    /// Execute the given future in the async runtime, returns a oneshot channel which resolves to the completed future output.
    pub fn execute_oneshot<F>(&self, future: F) -> oneshot::Receiver<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let (sender, receiver) = oneshot::channel::<F::Output>();

        let repainter = self.repainter.clone();
        self.execute(future, move |f| {
            let _ = sender.send(f);
            repainter.request_repaint();
        });

        receiver
    }
}
