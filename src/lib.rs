//! Wrapper type for by-address hashing and comparison.  
//! This crate is an extension of [`by_address`] crate with the short name of core struct ByAddress
//! and the implementation of some From/Into-like traits. This crate DOES depend on libstd,
//! unlike original [`by_address`] crate, so it can not be used in [`no_std`] projects.
//! The description below was taken mainly from the original crate.
//!
//! [ByAddr] can be used to wrap any pointer type (i.e. any type that implements the Deref
//! trait).  This includes references, raw pointers, smart pointers like `Rc<T>` and `Box<T>`, and
//! specialized pointer-like types such as `Vec<T>` and `String`.
//!
//! Comparison, ordering, and hashing of the wrapped pointer will be based on the address of its
//! contents, rather than their value.
//!
//! ```
//! use by_addr::ByAddr;
//! use std::rc::Rc;
//!
//! let rc = Rc::new(5);
//! let x = ByAddr(rc.clone());
//! let y = ByAddr(rc.clone());
//!
//! // x and y are two pointers to the same address:
//! assert_eq!(x, y);
//!
//! // Same as let z = 5.into_byaddr(); 
//! //      or ByAddr::from_target(5) (only for ByAddr<T> where T: Sized)
//! let z = ByAddr(Rc::new(5));
//!
//! // *x and *z have the same value, but not the same address:
//! assert_ne!(x, z);
//! ```
//!
//! You can use wrapped pointers as keys in hashed or ordered collections, like BTreeMap/BTreeSet
//! or HashMap/HashSet, even if the target of the pointer doesn't implement hashing or ordering.
//! This even includes pointers to trait objects, which usually don't implement the Eq trait
//! because it is not object-safe.
//!
//! ```
//! # use by_addr::ByAddr;
//! # use std::collections::HashSet;
//! #
//! /// Call each item in `callbacks`, skipping any duplicate references.
//! fn call_each_once(callbacks: &[&Fn()]) {
//!     let mut seen: HashSet<ByAddr<&Fn()>> = HashSet::new();
//!     for &f in callbacks {
//!         if seen.insert(ByAddr(f)) {
//!             f();
//!         }
//!     }
//! }
//! ```
//!
//! If `T` is a pointer to an unsized type, then comparison and ordering of `ByAddr<T>` compare
//! the entire fat pointer, not just the "thin" data address.  This means that two slice pointers
//! are consider equal only if they have the same starting address *and* length.
//!
//! ```
//! # use by_addr::ByAddr;
//! #
//! let v = [1, 2, 3, 4];
//!
//! assert_eq!(ByAddr(&v[0..4]), ByAddr(&v[0..4])); // Same address and length.
//! assert_ne!(ByAddr(&v[0..4]), ByAddr(&v[0..2])); // Same address, different length.
//! ```
//!
//! [`no_std`]: https://doc.rust-lang.org/book/first-edition/using-rust-without-the-standard-library.html
//! [`by_address`]: https://docs.rs/by_address/1.0.4/by_address/
//! [ByAddr]: struct.ByAddr.html

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub use by_address::ByAddress as ByAddr;
use std::ops::Deref;

pub trait FromTarget<T>: Deref {
    fn from_target(t: T) -> Self;
}

impl<T, Y> FromTarget<Y> for ByAddr<T> where T: From<Y> + Deref {
    fn from_target(t: Y) -> ByAddr<T> { ByAddr(t.into()) }
}

pub trait IntoByAddr<T>: Into<T> where T: Deref {
    fn into_byaddr(self) -> ByAddr<T>;
}

impl<T, Y> IntoByAddr<T> for Y where Y: Into<T>, T: Deref + From<Y> {
    fn into_byaddr(self) -> ByAddr<T> { ByAddr::from_target(self) }
}
