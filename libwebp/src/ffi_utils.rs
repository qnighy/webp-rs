use std::error::Error;
use std::fmt;
use std::os::raw::c_int;

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
pub(crate) fn check_stride(len: usize, width: u32, height: u32, stride: u32, bpp: u32) {
    assert_eq!(
        Some(len),
        (stride as usize).checked_mul(height as usize),
        "buffer length {} should be equal to stride {} * height {}",
        len,
        stride,
        height,
    );
    assert!(
        stride / bpp >= width,
        "stride {} should be greater than or equal to width {} * {}",
        stride,
        width,
        bpp,
    );
}

pub(crate) fn check_int(value: u32, name: &str) -> c_int {
    if let Ok(value) = PolyfillTryInto::try_into(value) {
        value
    } else {
        panic!("{} {} out of range for c_int", name, value);
    }
}

pub(crate) trait PolyfillTryFrom<T>: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

pub(crate) trait PolyfillTryInto<T>: Sized {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Void {}

impl<T, U> PolyfillTryInto<U> for T
where
    U: PolyfillTryFrom<T>,
{
    type Error = <U as PolyfillTryFrom<T>>::Error;
    fn try_into(self) -> Result<U, Self::Error> {
        <U as PolyfillTryFrom<T>>::try_from(self)
    }
}

impl PolyfillTryFrom<i64> for u32 {
    type Error = TryFromIntError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value >= 0 && value < 0x1_0000_0000 {
            Ok(value as u32)
        } else {
            Err(TryFromIntError(()))
        }
    }
}

impl PolyfillTryFrom<i32> for u32 {
    type Error = TryFromIntError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(value as u32)
        } else {
            Err(TryFromIntError(()))
        }
    }
}

impl PolyfillTryFrom<i16> for u32 {
    type Error = TryFromIntError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(value as u32)
        } else {
            Err(TryFromIntError(()))
        }
    }
}

impl PolyfillTryFrom<u32> for i64 {
    type Error = Void;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(i64::from(value))
    }
}

impl PolyfillTryFrom<u32> for i32 {
    type Error = TryFromIntError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value < 0x8000_0000 {
            Ok(value as i32)
        } else {
            Err(TryFromIntError(()))
        }
    }
}

impl PolyfillTryFrom<u32> for i16 {
    type Error = TryFromIntError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value < 0x8000 {
            Ok(value as i16)
        } else {
            Err(TryFromIntError(()))
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TryFromIntError(());

impl fmt::Display for TryFromIntError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(fmt)
    }
}

impl Error for TryFromIntError {
    fn description(&self) -> &str {
        "out of range integral type conversion attempted"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_stride() {
        check_stride(1056, 15, 22, 48, 3)
    }

    #[test]
    #[should_panic(expected = "buffer length")]
    fn test_check_stride_less_length() {
        check_stride(1055, 15, 22, 48, 3)
    }

    #[test]
    #[should_panic(expected = "buffer length")]
    fn test_check_stride_greater_length() {
        check_stride(1057, 15, 22, 48, 3)
    }

    #[test]
    #[should_panic(expected = "buffer length")]
    fn test_check_stride_overflow() {
        check_stride(1, 2000000, 641, 6700417, 3)
    }

    #[test]
    #[should_panic(expected = "stride")]
    fn test_check_stride_greater_stride() {
        check_stride(1056, 15, 22, 48, 4)
    }

    #[test]
    fn test_check_int() {
        assert_eq!(check_int(42, "foo"), 42);
        assert_eq!(check_int(0, "foo"), 0);
    }

    #[test]
    #[should_panic(expected = "foo")]
    fn test_check_int_overflow() {
        check_int(0xFFFF_FFFF, "foo");
    }
}
