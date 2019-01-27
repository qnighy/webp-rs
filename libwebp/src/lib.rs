// #[macro_use]
extern crate cfg_if;

extern crate libwebp_sys;

#[cfg(test)]
#[macro_use]
extern crate approx;

#[cfg(test)]
extern crate png;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;

pub use boxed::*;
pub use decode::*;
pub use encode::*;
pub use error::*;

mod boxed;
mod decode;
mod encode;
mod error;
mod ffi_utils;
#[cfg(test)]
mod test_utils;
