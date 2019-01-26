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

    fn pngs() -> &'static [Image] {
        lazy_static! {
            static ref PNGS: Vec<Image> = {
                vec![
                    decode_png(include_bytes!("../data/4.png")),
                    decode_png(include_bytes!("../data/5.png")),
                ]
            };
        }
        &PNGS
    }
    fn from_webp(webp: &[u8]) -> Image {
        let (width, height, data) = WebPDecodeRGBA(webp).unwrap();
        Image::new(ColorType::RGBA, width, height, width * 4, data.to_vec())
    }

    #[test]
    fn test_encode_rgb() {
        for image in pngs() {
            let image = image.convert_auto_stride(ColorType::RGB);
            let webp = WebPEncodeRGB(
                &image.data(),
                image.width(),
                image.height(),
                image.stride(),
                75.0,
            )
            .unwrap();
            assert_abs_diff_eq!(from_webp(&webp), image, epsilon = 128);
        }
    }

    #[test]
    fn test_encode_bgr() {
        for image in pngs() {
            let image = image.convert_auto_stride(ColorType::BGR);
            let webp = WebPEncodeBGR(
                &image.data(),
                image.width(),
                image.height(),
                image.stride(),
                75.0,
            )
            .unwrap();
            assert_abs_diff_eq!(from_webp(&webp), image, epsilon = 128);
        }
    }

    #[test]
    fn test_encode_rgba() {
        for image in pngs() {
            let image = image.convert_auto_stride(ColorType::RGBA);
            let webp = WebPEncodeRGBA(
                &image.data(),
                image.width(),
                image.height(),
                image.stride(),
                75.0,
            )
            .unwrap();
            assert_abs_diff_eq!(from_webp(&webp), image, epsilon = 128);
        }
    }

    #[test]
    fn test_encode_bgra() {
        for image in pngs() {
            let image = image.convert_auto_stride(ColorType::BGRA);
            let webp = WebPEncodeBGRA(
                &image.data(),
                image.width(),
                image.height(),
                image.stride(),
                75.0,
            )
            .unwrap();
            assert_abs_diff_eq!(from_webp(&webp), image, epsilon = 128);
        }
    }

    #[test]
    fn test_encode_lossless_rgb() {
        for image in pngs() {
            let image = image.convert_auto_stride(ColorType::RGB);
            let webp =
                WebPEncodeLosslessRGB(&image.data(), image.width(), image.height(), image.stride())
                    .unwrap();
            assert_eq!(from_webp(&webp), image);
        }
    }

    #[test]
    fn test_encode_lossless_bgr() {
        for image in pngs() {
            let image = image.convert_auto_stride(ColorType::BGR);
            let webp =
                WebPEncodeLosslessBGR(&image.data(), image.width(), image.height(), image.stride())
                    .unwrap();
            assert_eq!(from_webp(&webp), image);
        }
    }

    #[test]
    fn test_encode_lossless_rgba() {
        for image in pngs() {
            let image = image.convert_auto_stride(ColorType::RGBA);
            let webp = WebPEncodeLosslessRGBA(
                &image.data(),
                image.width(),
                image.height(),
                image.stride(),
            )
            .unwrap();
            assert_eq!(from_webp(&webp), image);
        }
    }

    #[test]
    fn test_encode_lossless_bgra() {
        for image in pngs() {
            let image = image.convert_auto_stride(ColorType::BGRA);
            let webp = WebPEncodeLosslessBGRA(
                &image.data(),
                image.width(),
                image.height(),
                image.stride(),
            )
            .unwrap();
            assert_eq!(from_webp(&webp), image);
        }
    }
}
