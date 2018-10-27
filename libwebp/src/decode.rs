use std::os::raw::*;
use std::ptr;

use libwebp_sys as sys;

use boxed::{WebpBox, WebpYuvBox};
use WebpUnknownError;

#[allow(non_snake_case)]
pub fn WebPGetDecoderVersion() -> u32 {
    unsafe { sys::WebPGetDecoderVersion() as u32 }
}

#[allow(non_snake_case)]
pub fn WebPGetInfo(data: &[u8]) -> Result<(u32, u32), WebpUnknownError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let res = unsafe { sys::WebPGetInfo(data.as_ptr(), data.len(), &mut width, &mut height) };
    if res != 0 {
        Ok((width as u32, height as u32))
    } else {
        Err(WebpUnknownError)
    }
}

#[allow(non_snake_case)]
pub fn WebPDecodeRGBA(data: &[u8]) -> Result<(u32, u32, WebpBox<[u8]>), WebpUnknownError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let res = unsafe { sys::WebPDecodeRGBA(data.as_ptr(), data.len(), &mut width, &mut height) };
    if !res.is_null() {
        let b = unsafe { WebpBox::from_raw_parts(res, width as usize * height as usize * 4) };
        Ok((width as u32, height as u32, b))
    } else {
        Err(WebpUnknownError)
    }
}

#[allow(non_snake_case)]
pub fn WebPDecodeARGB(data: &[u8]) -> Result<(u32, u32, WebpBox<[u8]>), WebpUnknownError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let res = unsafe { sys::WebPDecodeARGB(data.as_ptr(), data.len(), &mut width, &mut height) };
    if !res.is_null() {
        let b = unsafe { WebpBox::from_raw_parts(res, width as usize * height as usize * 4) };
        Ok((width as u32, height as u32, b))
    } else {
        Err(WebpUnknownError)
    }
}

#[allow(non_snake_case)]
pub fn WebPDecodeBGRA(data: &[u8]) -> Result<(u32, u32, WebpBox<[u8]>), WebpUnknownError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let res = unsafe { sys::WebPDecodeBGRA(data.as_ptr(), data.len(), &mut width, &mut height) };
    if !res.is_null() {
        let b = unsafe { WebpBox::from_raw_parts(res, width as usize * height as usize * 4) };
        Ok((width as u32, height as u32, b))
    } else {
        Err(WebpUnknownError)
    }
}

#[allow(non_snake_case)]
pub fn WebPDecodeRGB(data: &[u8]) -> Result<(u32, u32, WebpBox<[u8]>), WebpUnknownError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let res = unsafe { sys::WebPDecodeRGB(data.as_ptr(), data.len(), &mut width, &mut height) };
    if !res.is_null() {
        let b = unsafe { WebpBox::from_raw_parts(res, width as usize * height as usize * 3) };
        Ok((width as u32, height as u32, b))
    } else {
        Err(WebpUnknownError)
    }
}

#[allow(non_snake_case)]
pub fn WebPDecodeBGR(data: &[u8]) -> Result<(u32, u32, WebpBox<[u8]>), WebpUnknownError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let res = unsafe { sys::WebPDecodeBGR(data.as_ptr(), data.len(), &mut width, &mut height) };
    if !res.is_null() {
        let b = unsafe { WebpBox::from_raw_parts(res, width as usize * height as usize * 3) };
        Ok((width as u32, height as u32, b))
    } else {
        Err(WebpUnknownError)
    }
}

#[allow(non_snake_case)]
pub fn WebPDecodeYUV(data: &[u8]) -> Result<(u32, u32, u32, u32, WebpYuvBox), WebpUnknownError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let mut u: *mut u8 = ptr::null_mut();
    let mut v: *mut u8 = ptr::null_mut();
    let mut stride: c_int = 0;
    let mut uv_stride: c_int = 0;
    let res = unsafe {
        sys::WebPDecodeYUV(
            data.as_ptr(),
            data.len(),
            &mut width,
            &mut height,
            &mut u,
            &mut v,
            &mut stride,
            &mut uv_stride,
        )
    };
    if !res.is_null() {
        let uv_height = (height + 1) / 2;
        let y_size = stride as usize * height as usize;
        let uv_size = uv_stride as usize * uv_height as usize;
        let yuv = unsafe { WebpYuvBox::from_raw_parts(res, y_size, u, uv_size, v, uv_size) };
        Ok((
            width as u32,
            height as u32,
            stride as u32,
            uv_stride as u32,
            yuv,
        ))
    } else {
        Err(WebpUnknownError)
    }
}

#[allow(non_snake_case)]
pub fn WebPDecodeRGBAInto(
    data: &[u8],
    output_buffer: &mut [u8],
    output_stride: u32,
) -> Result<(), WebpUnknownError> {
    let res = unsafe {
        sys::WebPDecodeRGBAInto(
            data.as_ptr(),
            data.len(),
            output_buffer.as_mut_ptr(),
            output_buffer.len(),
            output_stride as c_int,
        )
    };
    if !res.is_null() {
        Ok(())
    } else {
        Err(WebpUnknownError)
    }
}

#[allow(non_snake_case)]
pub fn WebPDecodeARGBInto(
    data: &[u8],
    output_buffer: &mut [u8],
    output_stride: u32,
) -> Result<(), WebpUnknownError> {
    let res = unsafe {
        sys::WebPDecodeARGBInto(
            data.as_ptr(),
            data.len(),
            output_buffer.as_mut_ptr(),
            output_buffer.len(),
            output_stride as c_int,
        )
    };
    if !res.is_null() {
        Ok(())
    } else {
        Err(WebpUnknownError)
    }
}

#[allow(non_snake_case)]
pub fn WebPDecodeBGRAInto(
    data: &[u8],
    output_buffer: &mut [u8],
    output_stride: u32,
) -> Result<(), WebpUnknownError> {
    let res = unsafe {
        sys::WebPDecodeBGRAInto(
            data.as_ptr(),
            data.len(),
            output_buffer.as_mut_ptr(),
            output_buffer.len(),
            output_stride as c_int,
        )
    };
    if !res.is_null() {
        Ok(())
    } else {
        Err(WebpUnknownError)
    }
}

#[allow(non_snake_case)]
pub fn WebPDecodeRGBInto(
    data: &[u8],
    output_buffer: &mut [u8],
    output_stride: u32,
) -> Result<(), WebpUnknownError> {
    let res = unsafe {
        sys::WebPDecodeRGBInto(
            data.as_ptr(),
            data.len(),
            output_buffer.as_mut_ptr(),
            output_buffer.len(),
            output_stride as c_int,
        )
    };
    if !res.is_null() {
        Ok(())
    } else {
        Err(WebpUnknownError)
    }
}

#[allow(non_snake_case)]
pub fn WebPDecodeBGRInto(
    data: &[u8],
    output_buffer: &mut [u8],
    output_stride: u32,
) -> Result<(), WebpUnknownError> {
    let res = unsafe {
        sys::WebPDecodeBGRInto(
            data.as_ptr(),
            data.len(),
            output_buffer.as_mut_ptr(),
            output_buffer.len(),
            output_stride as c_int,
        )
    };
    if !res.is_null() {
        Ok(())
    } else {
        Err(WebpUnknownError)
    }
}

#[allow(non_snake_case)]
pub fn WebPDecodeYUVInto(
    data: &[u8],
    luma: &mut [u8],
    luma_stride: u32,
    u: &mut [u8],
    u_stride: u32,
    v: &mut [u8],
    v_stride: u32,
) -> Result<(), WebpUnknownError> {
    let res = unsafe {
        sys::WebPDecodeYUVInto(
            data.as_ptr(),
            data.len(),
            luma.as_mut_ptr(),
            luma.len(),
            luma_stride as c_int,
            u.as_mut_ptr(),
            u.len(),
            u_stride as c_int,
            v.as_mut_ptr(),
            v.len(),
            v_stride as c_int,
        )
    };
    if !res.is_null() {
        Ok(())
    } else {
        Err(WebpUnknownError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_utils::*;

    fn data4_webp() -> &'static [u8] {
        include_bytes!("../data/4.webp")
    }

    fn data4_expect(color_order: ColorOrder) -> Vec<u8> {
        decode_png(include_bytes!("../data/4.webp.png"), color_order)
    }

    fn data5_webp() -> &'static [u8] {
        include_bytes!("../data/5.webp")
    }

    fn data5_expect(color_order: ColorOrder) -> Vec<u8> {
        decode_png(include_bytes!("../data/5.webp.png"), color_order)
    }

    #[test]
    fn test_get_info() {
        assert_eq!(WebPGetInfo(data4_webp()), Ok((1024, 772)));
        assert_eq!(WebPGetInfo(data5_webp()), Ok((1024, 752)));
    }

    #[test]
    fn test_decode_rgba() {
        let (width, height, data) = WebPDecodeRGBA(data4_webp()).unwrap();
        assert_eq!((width, height), (1024, 772));
        assert_eq!(&data as &[u8], &data4_expect(ColorOrder::RGBA) as &[u8]);

        let (width, height, data) = WebPDecodeRGBA(data5_webp()).unwrap();
        assert_eq!((width, height), (1024, 752));
        assert_eq!(&data as &[u8], &data5_expect(ColorOrder::RGBA) as &[u8]);
    }

    #[test]
    fn test_decode_argb() {
        let (width, height, data) = WebPDecodeARGB(data4_webp()).unwrap();
        assert_eq!((width, height), (1024, 772));
        assert_eq!(&data as &[u8], &data4_expect(ColorOrder::ARGB) as &[u8]);

        let (width, height, data) = WebPDecodeARGB(data5_webp()).unwrap();
        assert_eq!((width, height), (1024, 752));
        assert_eq!(&data as &[u8], &data5_expect(ColorOrder::ARGB) as &[u8]);
    }

    #[test]
    fn test_decode_bgra() {
        let (width, height, data) = WebPDecodeBGRA(data4_webp()).unwrap();
        assert_eq!((width, height), (1024, 772));
        assert_eq!(&data as &[u8], &data4_expect(ColorOrder::BGRA) as &[u8]);

        let (width, height, data) = WebPDecodeBGRA(data5_webp()).unwrap();
        assert_eq!((width, height), (1024, 752));
        assert_eq!(&data as &[u8], &data5_expect(ColorOrder::BGRA) as &[u8]);
    }

    #[test]
    fn test_decode_rgb() {
        let (width, height, data) = WebPDecodeRGB(data4_webp()).unwrap();
        assert_eq!((width, height), (1024, 772));
        assert_eq!(&data as &[u8], &data4_expect(ColorOrder::RGB) as &[u8]);

        let (width, height, data) = WebPDecodeRGB(data5_webp()).unwrap();
        assert_eq!((width, height), (1024, 752));
        assert_eq!(&data as &[u8], &data5_expect(ColorOrder::RGB) as &[u8]);
    }

    #[test]
    fn test_decode_bgr() {
        let (width, height, data) = WebPDecodeBGR(data4_webp()).unwrap();
        assert_eq!((width, height), (1024, 772));
        assert_eq!(&data as &[u8], &data4_expect(ColorOrder::BGR) as &[u8]);

        let (width, height, data) = WebPDecodeBGR(data5_webp()).unwrap();
        assert_eq!((width, height), (1024, 752));
        assert_eq!(&data as &[u8], &data5_expect(ColorOrder::BGR) as &[u8]);
    }

    #[test]
    fn test_decode_yuv() {
        fn stride_conv(v: &[u8], stride: usize, width: usize) -> Vec<u8> {
            v.chunks(stride)
                .flat_map(|chunk| &chunk[..width])
                .cloned()
                .collect()
        }
        let (width, height, y_stride, uv_stride, yuv) = WebPDecodeYUV(data4_webp()).unwrap();
        assert_eq!((width, height), (1024, 772));
        let y = stride_conv(yuv.y(), y_stride as usize, width as usize);
        let u = stride_conv(yuv.u(), uv_stride as usize, (width as usize + 1) / 2);
        let v = stride_conv(yuv.v(), uv_stride as usize, (width as usize + 1) / 2);
        assert_eq!(y[..], include_bytes!("../data/4.y.dat")[..]);
        assert_eq!(u[..], include_bytes!("../data/4.u.dat")[..]);
        assert_eq!(v[..], include_bytes!("../data/4.v.dat")[..]);

        let (width, height, y_stride, uv_stride, yuv) = WebPDecodeYUV(data5_webp()).unwrap();
        assert_eq!((width, height), (1024, 752));
        let y = stride_conv(yuv.y(), y_stride as usize, width as usize);
        let u = stride_conv(yuv.u(), uv_stride as usize, (width as usize + 1) / 2);
        let v = stride_conv(yuv.v(), uv_stride as usize, (width as usize + 1) / 2);
        assert_eq!(y[..], include_bytes!("../data/5.y.dat")[..]);
        assert_eq!(u[..], include_bytes!("../data/5.u.dat")[..]);
        assert_eq!(v[..], include_bytes!("../data/5.v.dat")[..]);
    }

    #[test]
    fn test_decode_rgba_into() {
        let (width, height) = WebPGetInfo(data4_webp()).unwrap();
        let mut data = vec![0; width as usize * height as usize * 4];
        WebPDecodeRGBAInto(data4_webp(), &mut data, width * 4).unwrap();
        assert_eq!((width, height), (1024, 772));
        assert_eq!(&data as &[u8], &data4_expect(ColorOrder::RGBA) as &[u8]);

        let (width, height) = WebPGetInfo(data5_webp()).unwrap();
        let mut data = vec![0; width as usize * height as usize * 4];
        WebPDecodeRGBAInto(data5_webp(), &mut data, width * 4).unwrap();
        assert_eq!((width, height), (1024, 752));
        assert_eq!(&data as &[u8], &data5_expect(ColorOrder::RGBA) as &[u8]);
    }

    #[test]
    fn test_decode_argb_into() {
        let (width, height) = WebPGetInfo(data4_webp()).unwrap();
        let mut data = vec![0; width as usize * height as usize * 4];
        WebPDecodeARGBInto(data4_webp(), &mut data, width * 4).unwrap();
        assert_eq!((width, height), (1024, 772));
        assert_eq!(&data as &[u8], &data4_expect(ColorOrder::ARGB) as &[u8]);

        let (width, height) = WebPGetInfo(data5_webp()).unwrap();
        let mut data = vec![0; width as usize * height as usize * 4];
        WebPDecodeARGBInto(data5_webp(), &mut data, width * 4).unwrap();
        assert_eq!((width, height), (1024, 752));
        assert_eq!(&data as &[u8], &data5_expect(ColorOrder::ARGB) as &[u8]);
    }

    #[test]
    fn test_decode_bgra_into() {
        let (width, height) = WebPGetInfo(data4_webp()).unwrap();
        let mut data = vec![0; width as usize * height as usize * 4];
        WebPDecodeBGRAInto(data4_webp(), &mut data, width * 4).unwrap();
        assert_eq!((width, height), (1024, 772));
        assert_eq!(&data as &[u8], &data4_expect(ColorOrder::BGRA) as &[u8]);

        let (width, height) = WebPGetInfo(data5_webp()).unwrap();
        let mut data = vec![0; width as usize * height as usize * 4];
        WebPDecodeBGRAInto(data5_webp(), &mut data, width * 4).unwrap();
        assert_eq!((width, height), (1024, 752));
        assert_eq!(&data as &[u8], &data5_expect(ColorOrder::BGRA) as &[u8]);
    }

    #[test]
    fn test_decode_rgb_into() {
        let (width, height) = WebPGetInfo(data4_webp()).unwrap();
        let mut data = vec![0; width as usize * height as usize * 3];
        WebPDecodeRGBInto(data4_webp(), &mut data, width * 3).unwrap();
        assert_eq!((width, height), (1024, 772));
        assert_eq!(&data as &[u8], &data4_expect(ColorOrder::RGB) as &[u8]);

        let (width, height) = WebPGetInfo(data5_webp()).unwrap();
        let mut data = vec![0; width as usize * height as usize * 3];
        WebPDecodeRGBInto(data5_webp(), &mut data, width * 3).unwrap();
        assert_eq!((width, height), (1024, 752));
        assert_eq!(&data as &[u8], &data5_expect(ColorOrder::RGB) as &[u8]);
    }

    #[test]
    fn test_decode_bgr_into() {
        let (width, height) = WebPGetInfo(data4_webp()).unwrap();
        let mut data = vec![0; width as usize * height as usize * 3];
        WebPDecodeBGRInto(data4_webp(), &mut data, width * 3).unwrap();
        assert_eq!((width, height), (1024, 772));
        assert_eq!(&data as &[u8], &data4_expect(ColorOrder::BGR) as &[u8]);

        let (width, height) = WebPGetInfo(data5_webp()).unwrap();
        let mut data = vec![0; width as usize * height as usize * 3];
        WebPDecodeBGRInto(data5_webp(), &mut data, width * 3).unwrap();
        assert_eq!((width, height), (1024, 752));
        assert_eq!(&data as &[u8], &data5_expect(ColorOrder::BGR) as &[u8]);
    }

    #[test]
    fn test_decode_yuv_into() {
        let (width, height) = WebPGetInfo(data4_webp()).unwrap();
        let uv_width = (width + 1) / 2;
        let uv_height = (height + 1) / 2;
        let mut y = vec![0; width as usize * height as usize];
        let mut u = vec![0; uv_width as usize * uv_height as usize];
        let mut v = vec![0; uv_width as usize * uv_height as usize];
        WebPDecodeYUVInto(
            data4_webp(),
            &mut y,
            width,
            &mut u,
            uv_width,
            &mut v,
            uv_width,
        ).unwrap();
        assert_eq!(y[..], include_bytes!("../data/4.y.dat")[..]);
        assert_eq!(u[..], include_bytes!("../data/4.u.dat")[..]);
        assert_eq!(v[..], include_bytes!("../data/4.v.dat")[..]);

        let (width, height) = WebPGetInfo(data5_webp()).unwrap();
        let uv_width = (width + 1) / 2;
        let uv_height = (height + 1) / 2;
        let mut y = vec![0; width as usize * height as usize];
        let mut u = vec![0; uv_width as usize * uv_height as usize];
        let mut v = vec![0; uv_width as usize * uv_height as usize];
        WebPDecodeYUVInto(
            data5_webp(),
            &mut y,
            width,
            &mut u,
            uv_width,
            &mut v,
            uv_width,
        ).unwrap();
        assert_eq!(y[..], include_bytes!("../data/5.y.dat")[..]);
        assert_eq!(u[..], include_bytes!("../data/5.u.dat")[..]);
        assert_eq!(v[..], include_bytes!("../data/5.v.dat")[..]);
    }
}
