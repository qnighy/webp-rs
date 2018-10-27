// #[macro_use]
extern crate cfg_if;

extern crate libc;

extern crate libwebp_sys;

#[cfg(test)]
#[macro_use]
extern crate approx;

#[cfg(test)]
extern crate png;

pub mod boxed;
pub mod decode;
pub mod encode;
#[cfg(test)]
mod test_utils;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct WebpUnknownError;
