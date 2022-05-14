use crate::{Pos2, Response};

pub trait Sealed {}

impl Sealed for Response {}

/// Extends a [`crate::Response`].
pub trait ResponseExt: Sealed {
    /// A mixture of [`Response::clicked`] and [`Response::interact_pointer_pos`].  
    ///
    /// Returns `None` if the element was not clicked.   
    /// Otherwise, returns `Some(pos)` where `pos` is the position the element was clicked.
    fn clicked_with_pos(&self) -> Option<Pos2>;
}

impl ResponseExt for Response {
    fn clicked_with_pos(&self) -> Option<Pos2> {
        if self.clicked() {
            if let Some(pos) = self.interact_pointer_pos() {
                return Some(pos);
            }
        }

        None
    }
}
