#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ColorOrder {
    RGBA,
    ARGB,
    BGRA,
    RGB,
    BGR,
}

pub fn decode_png(data: &[u8], color_order: ColorOrder) -> Vec<u8> {
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
