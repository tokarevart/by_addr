# by_addr

Wrapper type for by-address hashing and comparison.  
This crate is an extension of [`by_address`] crate with the short name of core struct ByAddress
and the implementation of some From/Into-like traits. This crate DOES depend on libstd,
unlike original [`by_address`] crate, so it can not be used in [`no_std`] projects.
The description below was taken mainly from the original crate.

# Overview

`ByAddr` can be used to wrap any pointer type (i.e. any type that implements the Deref
trait).  This includes references, raw pointers, smart pointers like `Rc<T>`
and `Box<T>`, and specialized pointer-like types such as `Vec<T>` and `String`.

The wrapped pointer implements the following traits based on the address of
its contents, rather than their value:

* Hash
* Eq, PartialEq
* Ord, PartialOrd

## License

Licensed under the Apache License, Version 2.0 or the MIT license, at your
option.  See the license files in this directory for details.
