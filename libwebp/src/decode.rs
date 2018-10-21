use std::os::raw::*;

use libwebp_sys as sys;

use boxed::WebpBox;

#[allow(non_snake_case)]
pub fn WebPGetDecoderVersion() -> u32 {
    unsafe { sys::WebPGetDecoderVersion() as u32 }
}

#[allow(non_snake_case)]
pub fn WebPGetInfo(data: &[u8]) -> Option<(u32, u32)> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let res = unsafe { sys::WebPGetInfo(data.as_ptr(), data.len(), &mut width, &mut height) };
    if res != 0 {
        Some((width as u32, height as u32))
    } else {
        None
    }
}

#[allow(non_snake_case)]
pub fn WebPDecodeRGBA(data: &[u8]) -> Option<(u32, u32, WebpBox<[u8]>)> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let res = unsafe { sys::WebPDecodeRGBA(data.as_ptr(), data.len(), &mut width, &mut height) };
    if !res.is_null() {
        let b = unsafe { WebpBox::from_raw_parts(res, width as usize * height as usize * 4) };
        Some((width as u32, height as u32, b))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_rgba() {
        WebPDecodeRGBA(&[]);
    }
}
