//! ## Example
//!
//! ```
//! # #[macro_use] extern crate approx;
//! extern crate libwebp;
//! use libwebp::*;
//!
//! # fn main() {
//!     let data: &[u8];
//!     # data = b"\
//!     # RIFFV\x00\x00\x00WEBPVP8\x20\
//!     # J\x00\x00\x00\xD0\x01\x00\x9D\x01*\x03\x00\x02\x00\x02\x00\
//!     # 4%\xA8\x02t\x01\x0E\xFE\x03\x8E\x00\x00\xFE\xAD\xFF\xF1\
//!     # \x5C\xB4\xF8\xED\xFF\xF0\xC0\xBA\xBF\x93\x05\xEA\x0C\x9F\x93?\
//!     # \xE8\xC0\xBF?\xFF\xA9\xBF\xFF${\xCB\xFFF\x05\xF9\xFF\
//!     # \xFDM\xFE0\xE5\x86\xAA\x071#o\x00\x00\x00";
//!     # assert_eq!(data.len(), 94);
//!     let (width, height, data) = WebPDecodeRGBA(data).unwrap();
//!     assert_eq!((width, height), (3, 2));
//!     assert_abs_diff_eq!(data[0], 86, epsilon = 1);
//! # }
//! ```

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
