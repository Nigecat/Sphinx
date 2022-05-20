//! General utility methods and traits.

/// Generate a plural based on the number of items.
///
/// # Examples
///
/// ```rust
/// # fn main() {
/// use sphinx::utils::pluralize::Pluralize;
///
/// let obj = "book";
/// assert_eq!(obj.pluralize("books", 0), "books");
/// assert_eq!(obj.pluralize("books", 1), "book");
/// assert_eq!(obj.pluralize("books", 2), "books");
/// # }
/// ```
pub mod pluralize {
    /// See module level docs.
    pub trait Pluralize<T> {
        /// See module level docs.
        fn pluralize(self, plural: T, items: usize) -> T;
    }

    impl<T> Pluralize<T> for T {
        fn pluralize(self, plural: T, items: usize) -> T {
            match items {
                1 => self,
                _ => plural,
            }
        }
    }
}
