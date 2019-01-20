use std::os::raw::*;
use std::ptr;

use libwebp_sys as sys;

use boxed::WebpBox;
use error::WebpUnknownError;
use ffi_utils::{check_int, check_stride};

macro_rules! check_int {
    ($e:expr) => {
        check_int($e, stringify!($e))
    };
}

#[allow(non_snake_case)]
pub fn WebPEncodeRGB(
    rgb: &[u8],
    width: u32,
    height: u32,
    stride: u32,
    quality_factor: f32,
) -> Result<WebpBox<[u8]>, WebpUnknownError> {
    check_stride(rgb.len(), width, height, stride, 3);
    let mut output: *mut u8 = ptr::null_mut();
    let res = unsafe {
        sys::WebPEncodeRGB(
            rgb.as_ptr(),
            check_int!(width),
            check_int!(height),
            check_int!(stride),
            quality_factor as c_float,
            &mut output,
        )
    };
    if res != 0 {
        Ok(unsafe { WebpBox::from_raw_parts(output, res) })
    } else {
        Err(WebpUnknownError)
    }
}

#[allow(non_snake_case)]
pub fn WebPEncodeBGR(
    bgr: &[u8],
    width: u32,
    height: u32,
    stride: u32,
    quality_factor: f32,
) -> Result<WebpBox<[u8]>, WebpUnknownError> {
    check_stride(bgr.len(), width, height, stride, 3);
    let mut output: *mut u8 = ptr::null_mut();
    let res = unsafe {
        sys::WebPEncodeBGR(
            bgr.as_ptr(),
            check_int!(width),
            check_int!(height),
            check_int!(stride),
            quality_factor as c_float,
            &mut output,
        )
    };
    if res != 0 {
        Ok(unsafe { WebpBox::from_raw_parts(output, res) })
    } else {
        Err(WebpUnknownError)
    }
}

#[allow(non_snake_case)]
pub fn WebPEncodeRGBA(
    rgba: &[u8],
    width: u32,
    height: u32,
    stride: u32,
    quality_factor: f32,
) -> Result<WebpBox<[u8]>, WebpUnknownError> {
    check_stride(rgba.len(), width, height, stride, 4);
    let mut output: *mut u8 = ptr::null_mut();
    let res = unsafe {
        sys::WebPEncodeRGBA(
            rgba.as_ptr(),
            check_int!(width),
            check_int!(height),
            check_int!(stride),
            quality_factor as c_float,
            &mut output,
        )
    };
    if res != 0 {
        Ok(unsafe { WebpBox::from_raw_parts(output, res) })
    } else {
        Err(WebpUnknownError)
    }
}

#[allow(non_snake_case)]
pub fn WebPEncodeBGRA(
    bgra: &[u8],
    width: u32,
    height: u32,
    stride: u32,
    quality_factor: f32,
) -> Result<WebpBox<[u8]>, WebpUnknownError> {
    check_stride(bgra.len(), width, height, stride, 4);
    let mut output: *mut u8 = ptr::null_mut();
    let res = unsafe {
        sys::WebPEncodeBGRA(
            bgra.as_ptr(),
            check_int!(width),
            check_int!(height),
            check_int!(stride),
            quality_factor as c_float,
            &mut output,
        )
    };
    if res != 0 {
        Ok(unsafe { WebpBox::from_raw_parts(output, res) })
    } else {
        Err(WebpUnknownError)
    }
}

#[allow(non_snake_case)]
pub fn WebPEncodeLosslessRGB(
    rgb: &[u8],
    width: u32,
    height: u32,
    stride: u32,
) -> Result<WebpBox<[u8]>, WebpUnknownError> {
    check_stride(rgb.len(), width, height, stride, 3);
    let mut output: *mut u8 = ptr::null_mut();
    let res = unsafe {
        sys::WebPEncodeLosslessRGB(
            rgb.as_ptr(),
            check_int!(width),
            check_int!(height),
            check_int!(stride),
            &mut output,
        )
    };
    if res != 0 {
        Ok(unsafe { WebpBox::from_raw_parts(output, res) })
    } else {
        Err(WebpUnknownError)
    }
}

#[allow(non_snake_case)]
pub fn WebPEncodeLosslessBGR(
    bgr: &[u8],
    width: u32,
    height: u32,
    stride: u32,
) -> Result<WebpBox<[u8]>, WebpUnknownError> {
    check_stride(bgr.len(), width, height, stride, 3);
    let mut output: *mut u8 = ptr::null_mut();
    let res = unsafe {
        sys::WebPEncodeLosslessBGR(
            bgr.as_ptr(),
            check_int!(width),
            check_int!(height),
            check_int!(stride),
            &mut output,
        )
    };
    if res != 0 {
        Ok(unsafe { WebpBox::from_raw_parts(output, res) })
    } else {
        Err(WebpUnknownError)
    }
}

#[allow(non_snake_case)]
pub fn WebPEncodeLosslessRGBA(
    rgba: &[u8],
    width: u32,
    height: u32,
    stride: u32,
) -> Result<WebpBox<[u8]>, WebpUnknownError> {
    check_stride(rgba.len(), width, height, stride, 4);
    let mut output: *mut u8 = ptr::null_mut();
    let res = unsafe {
        sys::WebPEncodeLosslessRGBA(
            rgba.as_ptr(),
            check_int!(width),
            check_int!(height),
            check_int!(stride),
            &mut output,
        )
    };
    if res != 0 {
        Ok(unsafe { WebpBox::from_raw_parts(output, res) })
    } else {
        Err(WebpUnknownError)
    }
}

#[allow(non_snake_case)]
pub fn WebPEncodeLosslessBGRA(
    bgra: &[u8],
    width: u32,
    height: u32,
    stride: u32,
) -> Result<WebpBox<[u8]>, WebpUnknownError> {
    check_stride(bgra.len(), width, height, stride, 4);
    let mut output: *mut u8 = ptr::null_mut();
    let res = unsafe {
        sys::WebPEncodeLosslessBGRA(
            bgra.as_ptr(),
            check_int!(width),
            check_int!(height),
            check_int!(stride),
            &mut output,
        )
    };
    if res != 0 {
        Ok(unsafe { WebpBox::from_raw_parts(output, res) })
    } else {
        Err(WebpUnknownError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use decode::*;
    use test_utils::*;

    fn data4_input(color_order: ColorOrder) -> Vec<u8> {
        decode_png(include_bytes!("../data/4.png"), color_order)
    }

    #[test]
    fn test_encode_rgb() {
        let orig_data = data4_input(ColorOrder::RGB);
        let webp = WebPEncodeRGB(&orig_data, 1024, 772, 1024 * 3, 75.0).unwrap();
        let (width, height, data) = WebPDecodeRGB(&webp).unwrap();
        assert_eq!((width, height), (1024, 772));
        assert_abs_diff_eq!(&data as &[u8], &orig_data as &[u8], epsilon = 128);
    }

    #[test]
    fn test_encode_bgr() {
        let orig_data = data4_input(ColorOrder::BGR);
        let webp = WebPEncodeBGR(&orig_data, 1024, 772, 1024 * 3, 75.0).unwrap();
        let (width, height, data) = WebPDecodeBGR(&webp).unwrap();
        assert_eq!((width, height), (1024, 772));
        assert_abs_diff_eq!(&data as &[u8], &orig_data as &[u8], epsilon = 128);
    }

    #[test]
    fn test_encode_rgba() {
        let orig_data = data4_input(ColorOrder::RGBA);
        let webp = WebPEncodeRGBA(&orig_data, 1024, 772, 1024 * 4, 75.0).unwrap();
        let (width, height, data) = WebPDecodeRGBA(&webp).unwrap();
        assert_eq!((width, height), (1024, 772));
        assert_abs_diff_eq!(&data as &[u8], &orig_data as &[u8], epsilon = 128);
    }

    #[test]
    fn test_encode_bgra() {
        let orig_data = data4_input(ColorOrder::BGRA);
        let webp = WebPEncodeBGRA(&orig_data, 1024, 772, 1024 * 4, 75.0).unwrap();
        let (width, height, data) = WebPDecodeBGRA(&webp).unwrap();
        assert_eq!((width, height), (1024, 772));
        assert_abs_diff_eq!(&data as &[u8], &orig_data as &[u8], epsilon = 128);
    }

    #[test]
    fn test_encode_lossless_rgb() {
        let orig_data = data4_input(ColorOrder::RGB);
        let webp = WebPEncodeLosslessRGB(&orig_data, 1024, 772, 1024 * 3).unwrap();
        let (width, height, data) = WebPDecodeRGB(&webp).unwrap();
        assert_eq!((width, height), (1024, 772));
        assert_eq!(&data as &[u8], &orig_data as &[u8]);
    }

    #[test]
    fn test_encode_lossless_bgr() {
        let orig_data = data4_input(ColorOrder::BGR);
        let webp = WebPEncodeLosslessBGR(&orig_data, 1024, 772, 1024 * 3).unwrap();
        let (width, height, data) = WebPDecodeBGR(&webp).unwrap();
        assert_eq!((width, height), (1024, 772));
        assert_eq!(&data as &[u8], &orig_data as &[u8]);
    }

    #[test]
    fn test_encode_lossless_rgba() {
        let orig_data = data4_input(ColorOrder::RGBA);
        let webp = WebPEncodeLosslessRGBA(&orig_data, 1024, 772, 1024 * 4).unwrap();
        let (width, height, data) = WebPDecodeRGBA(&webp).unwrap();
        assert_eq!((width, height), (1024, 772));
        assert_eq!(&data as &[u8], &orig_data as &[u8]);
    }

    #[test]
    fn test_encode_lossless_bgra() {
        let orig_data = data4_input(ColorOrder::BGRA);
        let webp = WebPEncodeLosslessBGRA(&orig_data, 1024, 772, 1024 * 4).unwrap();
        let (width, height, data) = WebPDecodeBGRA(&webp).unwrap();
        assert_eq!((width, height), (1024, 772));
        assert_eq!(&data as &[u8], &orig_data as &[u8]);
    }
}
