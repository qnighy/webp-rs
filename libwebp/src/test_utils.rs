use approx::AbsDiffEq;
use png;

pub(crate) fn test_cases() -> &'static [TestCase] {
    lazy_static! {
        static ref TEST_CASES: Vec<TestCase> = {
            let data = [
                (
                    include_bytes!("../data/4.webp.png") as &[_],
                    include_bytes!("../data/4.webp") as &[_],
                    include_bytes!("../data/4.y.png") as &[_],
                    include_bytes!("../data/4.u.png") as &[_],
                    include_bytes!("../data/4.v.png") as &[_],
                ),
                (
                    include_bytes!("../data/5.webp.png"),
                    include_bytes!("../data/5.webp"),
                    include_bytes!("../data/5.y.png"),
                    include_bytes!("../data/5.u.png"),
                    include_bytes!("../data/5.v.png"),
                ),
            ];
            data.iter()
                .map(|&(png, webp, y_png, u_png, v_png)| {
                    let image = decode_png(png);
                    let image_opaque = image.to_opaque();
                    let webp_data = webp.to_vec();
                    let y_image = decode_png(y_png);
                    let u_image = decode_png(u_png);
                    let v_image = decode_png(v_png);
                    TestCase {
                        image,
                        image_opaque,
                        webp_data,
                        y_image,
                        u_image,
                        v_image,
                    }
                })
                .collect()
        };
    }
    &TEST_CASES
}

#[derive(Debug, Clone)]
pub(crate) struct TestCase {
    pub(crate) image: Image,
    pub(crate) image_opaque: Image,
    pub(crate) webp_data: Vec<u8>,
    pub(crate) y_image: Image,
    pub(crate) u_image: Image,
    pub(crate) v_image: Image,
}

#[derive(Debug, Clone)]
pub(crate) struct Image {
    color_type: ColorType,
    width: u32,
    height: u32,
    stride: u32,
    data: Vec<u8>,
}

impl Image {
    pub(crate) fn new(
        color_type: ColorType,
        width: u32,
        height: u32,
        stride: u32,
        data: Vec<u8>,
    ) -> Self {
        assert!(stride as usize / color_type.byte_len() >= width as usize);
        assert_eq!(
            (stride as usize).checked_mul(height as usize).unwrap(),
            data.len()
        );
        Self {
            color_type,
            width,
            height,
            stride,
            data,
        }
    }

    pub(crate) fn width(&self) -> u32 {
        self.width
    }

    pub(crate) fn height(&self) -> u32 {
        self.height
    }

    pub(crate) fn stride(&self) -> u32 {
        self.stride
    }

    pub(crate) fn data(&self) -> &[u8] {
        &self.data
    }

    pub(crate) fn to_opaque(&self) -> Image {
        let mut this = self.clone();
        this.make_opaque();
        this
    }

    fn make_opaque(&mut self) {
        use self::ColorType::*;

        let pixel_len = self.color_type.byte_len();
        for y in 0..self.height as usize {
            let line = &mut self.data[y * self.stride as usize..(y + 1) * self.stride as usize];
            for x in 0..self.width as usize {
                let pixel = &mut line[x * pixel_len..(x + 1) * pixel_len];
                match self.color_type {
                    RGBA => pixel[3] = 255,
                    BGRA => pixel[3] = 255,
                    ARGB => pixel[0] = 255,
                    RGB | BGR | Grayscale | GrayscaleAlpha => {}
                }
            }
        }
    }

    pub(crate) fn convert_auto_stride(&self, new_color_type: ColorType) -> Image {
        self.convert(
            new_color_type,
            new_color_type.byte_len() as u32 * self.width,
        )
    }
    pub(crate) fn convert(&self, new_color_type: ColorType, new_stride: u32) -> Image {
        assert!(new_stride as usize / new_color_type.byte_len() >= self.width as usize);
        let mut data = vec![0_u8; new_stride as usize * self.height as usize];

        let pixel_len = self.color_type.byte_len();
        let new_pixel_len = new_color_type.byte_len();
        for y in 0..self.height as usize {
            let line = &self.data[y * self.stride as usize..(y + 1) * self.stride as usize];
            let new_line = &mut data[y * new_stride as usize..(y + 1) * new_stride as usize];
            for x in 0..self.width as usize {
                let pixel = &line[x * pixel_len..(x + 1) * pixel_len];
                let pixel = self.color_type.convert_rgba(pixel);
                let new_pixel = &mut new_line[x * new_pixel_len..(x + 1) * new_pixel_len];
                new_color_type.convert_from_rgba(new_pixel, pixel);
            }
        }
        Image::new(new_color_type, self.width, self.height, new_stride, data)
    }

    fn eq_general<F: FnMut([u8; 4], [u8; 4]) -> bool>(&self, other: &Image, mut f: F) -> bool {
        if self.width != other.width {
            return false;
        }
        if self.height != other.height {
            return false;
        }
        let self_pixel_len = self.color_type.byte_len();
        let other_pixel_len = other.color_type.byte_len();
        for y in 0..self.height as usize {
            let self_line = &self.data[y * self.stride as usize..(y + 1) * self.stride as usize];
            let other_line =
                &other.data[y * other.stride as usize..(y + 1) * other.stride as usize];
            for x in 0..self.width as usize {
                let self_pixel = &self_line[x * self_pixel_len..(x + 1) * self_pixel_len];
                let other_pixel = &other_line[x * other_pixel_len..(x + 1) * other_pixel_len];
                let self_pixel = self.color_type.convert_rgba(self_pixel);
                let other_pixel = other.color_type.convert_rgba(other_pixel);
                if !f(self_pixel, other_pixel) {
                    return false;
                }
            }
        }
        true
    }
}

impl PartialEq for Image {
    fn eq(&self, other: &Image) -> bool {
        self.eq_general(other, |this, other| this == other)
    }
}

impl Eq for Image {}

impl AbsDiffEq for Image {
    type Epsilon = u8;

    fn default_epsilon() -> Self::Epsilon {
        u8::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.eq_general(other, move |this, other| this.abs_diff_eq(&other, epsilon))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum ColorType {
    RGBA,
    ARGB,
    BGRA,
    RGB,
    BGR,
    Grayscale,
    GrayscaleAlpha,
}

impl ColorType {
    pub(crate) fn byte_len(&self) -> usize {
        use self::ColorType::*;

        match *self {
            RGBA | ARGB | BGRA => 4,
            RGB | BGR => 3,
            GrayscaleAlpha => 2,
            Grayscale => 1,
        }
    }

    pub(crate) fn convert_rgba(&self, data: &[u8]) -> [u8; 4] {
        use self::ColorType::*;
        match *self {
            RGBA => [data[0], data[1], data[2], data[3]],
            ARGB => [data[1], data[2], data[3], data[0]],
            BGRA => [data[2], data[1], data[0], data[3]],
            RGB => [data[0], data[1], data[2], 255],
            BGR => [data[2], data[1], data[0], 255],
            GrayscaleAlpha => [data[0], data[0], data[0], data[1]],
            Grayscale => [data[0], data[0], data[0], 255],
        }
    }

    pub(crate) fn convert_from_rgba(&self, data: &mut [u8], rgba: [u8; 4]) {
        use self::ColorType::*;
        match *self {
            RGBA => {
                data[0] = rgba[0];
                data[1] = rgba[1];
                data[2] = rgba[2];
                data[3] = rgba[3];
            }
            BGRA => {
                data[0] = rgba[2];
                data[1] = rgba[1];
                data[2] = rgba[0];
                data[3] = rgba[3];
            }
            ARGB => {
                data[0] = rgba[3];
                data[1] = rgba[0];
                data[2] = rgba[1];
                data[3] = rgba[2];
            }
            RGB => {
                data[0] = rgba[0];
                data[1] = rgba[1];
                data[2] = rgba[2];
            }
            BGR => {
                data[0] = rgba[2];
                data[1] = rgba[1];
                data[2] = rgba[0];
            }
            GrayscaleAlpha => {
                let luma = ((rgba[0] as i32 + rgba[1] as i32 + rgba[2] as i32) / 3) as u8;
                data[0] = luma;
                data[1] = rgba[3];
            }
            Grayscale => {
                let luma = ((rgba[0] as i32 + rgba[1] as i32 + rgba[2] as i32) / 3) as u8;
                data[0] = luma;
            }
        }
    }
}

pub(crate) fn decode_png(data: &[u8]) -> Image {
    use png::{self, Decoder};
    use std::io::Cursor;
    let (info, mut r) = Decoder::new(Cursor::new(data)).read_info().unwrap();
    let mut data = vec![0; info.buffer_size()];
    r.next_frame(&mut data).unwrap();
    let color_type = match info.color_type {
        png::ColorType::Grayscale => ColorType::Grayscale,
        png::ColorType::GrayscaleAlpha => ColorType::GrayscaleAlpha,
        png::ColorType::Indexed => unimplemented!(),
        png::ColorType::RGB => ColorType::RGB,
        png::ColorType::RGBA => ColorType::RGBA,
    };
    Image::new(
        color_type,
        info.width,
        info.height,
        info.line_size as u32,
        data,
    )
}

#[allow(unused)]
pub(crate) fn save_png(image: &Image, color_type: png::ColorType, path: &str) {
    use png::HasParameters;
    use std::fs::File;
    use std::io::{self, BufWriter, Write};

    let conv_color_type = match color_type {
        png::ColorType::RGBA => ColorType::RGBA,
        png::ColorType::RGB => ColorType::RGB,
        png::ColorType::GrayscaleAlpha => ColorType::GrayscaleAlpha,
        png::ColorType::Grayscale => ColorType::Grayscale,
        png::ColorType::Indexed => unimplemented!(),
    };
    let image = image.convert_auto_stride(conv_color_type);

    let f = File::create(path).unwrap();
    let mut f = BufWriter::new(f);

    {
        let mut enc = png::Encoder::new(&mut f, image.width, image.height);
        enc.set(color_type).set(png::BitDepth::Eight);
        let mut enc = enc.write_header().unwrap();
        enc.write_image_data(&image.data);
    }

    f.flush().unwrap();
}
