use std::os::raw::*;
use std::ptr;

use libwebp_sys as sys;

use boxed::WebpBox;
use WebpUnknownError;

#[allow(non_snake_case)]
pub fn WebPEncodeRGB(
    rgb: &[u8],
    width: u32,
    height: u32,
    stride: u32,
    quality_factor: f32,
) -> Result<WebpBox<[u8]>, WebpUnknownError> {
    assert_eq!(rgb.len(), stride as usize * height as usize);
    assert!(stride >= width * 3);
    let mut output: *mut u8 = ptr::null_mut();
    let res = unsafe {
        sys::WebPEncodeRGB(
            rgb.as_ptr(),
            width as c_int,
            height as c_int,
            stride as c_int,
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
    assert_eq!(bgr.len(), stride as usize * height as usize);
    assert!(stride >= width * 3);
    let mut output: *mut u8 = ptr::null_mut();
    let res = unsafe {
        sys::WebPEncodeBGR(
            bgr.as_ptr(),
            width as c_int,
            height as c_int,
            stride as c_int,
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
    assert_eq!(rgba.len(), stride as usize * height as usize);
    assert!(stride >= width * 4);
    let mut output: *mut u8 = ptr::null_mut();
    let res = unsafe {
        sys::WebPEncodeRGBA(
            rgba.as_ptr(),
            width as c_int,
            height as c_int,
            stride as c_int,
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
    assert_eq!(bgra.len(), stride as usize * height as usize);
    assert!(stride >= width * 4);
    let mut output: *mut u8 = ptr::null_mut();
    let res = unsafe {
        sys::WebPEncodeBGRA(
            bgra.as_ptr(),
            width as c_int,
            height as c_int,
            stride as c_int,
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
    assert_eq!(rgb.len(), stride as usize * height as usize);
    assert!(stride >= width * 3);
    let mut output: *mut u8 = ptr::null_mut();
    let res = unsafe {
        sys::WebPEncodeLosslessRGB(
            rgb.as_ptr(),
            width as c_int,
            height as c_int,
            stride as c_int,
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
    assert_eq!(bgr.len(), stride as usize * height as usize);
    assert!(stride >= width * 3);
    let mut output: *mut u8 = ptr::null_mut();
    let res = unsafe {
        sys::WebPEncodeLosslessBGR(
            bgr.as_ptr(),
            width as c_int,
            height as c_int,
            stride as c_int,
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
    assert_eq!(rgba.len(), stride as usize * height as usize);
    assert!(stride >= width * 4);
    let mut output: *mut u8 = ptr::null_mut();
    let res = unsafe {
        sys::WebPEncodeLosslessRGBA(
            rgba.as_ptr(),
            width as c_int,
            height as c_int,
            stride as c_int,
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
    assert_eq!(bgra.len(), stride as usize * height as usize);
    assert!(stride >= width * 4);
    let mut output: *mut u8 = ptr::null_mut();
    let res = unsafe {
        sys::WebPEncodeLosslessBGRA(
            bgra.as_ptr(),
            width as c_int,
            height as c_int,
            stride as c_int,
            &mut output,
        )
    };
    if res != 0 {
        Ok(unsafe { WebpBox::from_raw_parts(output, res) })
    } else {
        Err(WebpUnknownError)
    }
}
