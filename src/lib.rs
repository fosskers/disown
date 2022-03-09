//! Drop ownership from "method position".
//!
//! # Motivation
//!
//! Normally, unowned data is automatically dropped at the end of its residing
//! block. We can also ignore unuseful return values with `;`, which is
//! essentially a `T -> ()` transformation. However, there are cases where we
//! wish to drop ownership and return cleanly with a `()`, but don't want to
//! involve `;` (such as in closures or simple `match` arms). We could use
//! [`std::mem::drop`] for this, but `drop` is a function, not a method, and
//! would visually mar a nice chain of method calls.
//!
//! Hence the [`Disown`] trait and its method `disown`. It is `drop`, but in
//! "method position".
//!
//! ```
//! use disown::Disown;
//! use std::collections::HashSet;
//!
//! enum Person {
//!     Bob,
//!     Sam,
//! }
//!
//! let mut set = HashSet::new();
//! let person = Person::Bob;
//!
//! match person {
//!   Person::Bob => set.insert(0).disown(),
//!   Person::Sam => set.insert(1).disown(),
//! }
//! ```
//!
//! `HashSet::insert` returns a `bool`, not `()`, and the above code would not
//! compile without opening a pair of `{}` and using a `;`, which doesn't look
//! as nice.

/// Consume ownership in style.
///
/// Unlike [`std::ops::Drop`], this is implemented for all `T`.
pub trait Disown {
    fn disown(self);
}

impl<T> Disown for T {
    fn disown(self) {}
}
