// #[macro_use]
extern crate cfg_if;

extern crate libc;

extern crate libwebp_sys;

#[cfg(test)]
#[macro_use]
extern crate approx;

#[cfg(test)]
extern crate png;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;

pub mod boxed;
pub mod decode;
pub mod encode;
mod error;
mod ffi_utils;
#[cfg(test)]
mod test_utils;

pub use error::WebpUnknownError;
