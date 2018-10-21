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

#[allow(non_snake_case)]
pub fn WebPDecodeARGB(data: &[u8]) -> Option<(u32, u32, WebpBox<[u8]>)> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let res = unsafe { sys::WebPDecodeARGB(data.as_ptr(), data.len(), &mut width, &mut height) };
    if !res.is_null() {
        let b = unsafe { WebpBox::from_raw_parts(res, width as usize * height as usize * 4) };
        Some((width as u32, height as u32, b))
    } else {
        None
    }
}

#[allow(non_snake_case)]
pub fn WebPDecodeBGRA(data: &[u8]) -> Option<(u32, u32, WebpBox<[u8]>)> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let res = unsafe { sys::WebPDecodeBGRA(data.as_ptr(), data.len(), &mut width, &mut height) };
    if !res.is_null() {
        let b = unsafe { WebpBox::from_raw_parts(res, width as usize * height as usize * 4) };
        Some((width as u32, height as u32, b))
    } else {
        None
    }
}

#[allow(non_snake_case)]
pub fn WebPDecodeRGB(data: &[u8]) -> Option<(u32, u32, WebpBox<[u8]>)> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let res = unsafe { sys::WebPDecodeRGB(data.as_ptr(), data.len(), &mut width, &mut height) };
    if !res.is_null() {
        let b = unsafe { WebpBox::from_raw_parts(res, width as usize * height as usize * 3) };
        Some((width as u32, height as u32, b))
    } else {
        None
    }
}

#[allow(non_snake_case)]
pub fn WebPDecodeBGR(data: &[u8]) -> Option<(u32, u32, WebpBox<[u8]>)> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let res = unsafe { sys::WebPDecodeBGR(data.as_ptr(), data.len(), &mut width, &mut height) };
    if !res.is_null() {
        let b = unsafe { WebpBox::from_raw_parts(res, width as usize * height as usize * 3) };
        Some((width as u32, height as u32, b))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum ColorOrder {
        RGBA,
        ARGB,
        BGRA,
        RGB,
        BGR,
    }
    fn decode_png(data: &[u8], color_order: ColorOrder) -> Vec<u8> {
        use png::{ColorType, Decoder};
        use std::io::Cursor;
        use std::iter;
        let (info, mut r) = Decoder::new(Cursor::new(data)).read_info().unwrap();
        let mut buf = vec![0; info.buffer_size()];
        r.next_frame(&mut buf).unwrap();
        buf.chunks(info.color_type.samples())
            .flat_map(|chunk| {
                let (r, g, b, a) = match info.color_type {
                    ColorType::Grayscale => (chunk[0], chunk[0], chunk[0], 255),
                    ColorType::RGB => (chunk[0], chunk[1], chunk[2], 255),
                    ColorType::Indexed => unimplemented!(),
                    ColorType::GrayscaleAlpha => (chunk[0], chunk[0], chunk[0], chunk[1]),
                    ColorType::RGBA => (chunk[0], chunk[1], chunk[2], chunk[3]),
                };
                match color_order {
                    ColorOrder::RGBA => iter::once(r)
                        .chain(iter::once(g))
                        .chain(iter::once(b))
                        .chain(Some(a)),
                    ColorOrder::ARGB => iter::once(a)
                        .chain(iter::once(r))
                        .chain(iter::once(g))
                        .chain(Some(b)),
                    ColorOrder::BGRA => iter::once(b)
                        .chain(iter::once(g))
                        .chain(iter::once(r))
                        .chain(Some(a)),
                    ColorOrder::RGB => iter::once(r)
                        .chain(iter::once(g))
                        .chain(iter::once(b))
                        .chain(None),
                    ColorOrder::BGR => iter::once(b)
                        .chain(iter::once(g))
                        .chain(iter::once(r))
                        .chain(None),
                }
            }).collect()
    }

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
        assert_eq!(WebPGetInfo(data4_webp()), Some((1024, 772)));
        assert_eq!(WebPGetInfo(data5_webp()), Some((1024, 752)));
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
}
