use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::os::raw::*;
use std::ptr::{self, NonNull};
use std::slice;

use libwebp_sys as sys;

use boxed::{WebpBox, WebpYuvBox};
use error::WebpUnknownError;
use ffi_utils::check_int;

pub use libwebp_sys::{VP8StatusCode, WEBP_CSP_MODE};

macro_rules! check_int {
    ($e:expr) => {
        check_int($e, stringify!($e))
    };
}

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
            check_int!(output_stride),
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
            check_int!(output_stride),
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
            check_int!(output_stride),
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
            check_int!(output_stride),
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
            check_int!(output_stride),
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
            check_int!(luma_stride),
            u.as_mut_ptr(),
            u.len(),
            check_int!(u_stride),
            v.as_mut_ptr(),
            v.len(),
            check_int!(v_stride),
        )
    };
    if !res.is_null() {
        Ok(())
    } else {
        Err(WebpUnknownError)
    }
}

#[allow(non_snake_case)]
pub fn WebPIsPremultipliedMode(mode: WEBP_CSP_MODE) -> bool {
    use self::WEBP_CSP_MODE::*;
    mode == MODE_rgbA || mode == MODE_bgrA || mode == MODE_Argb || mode == MODE_rgbA_4444
}

#[allow(non_snake_case)]
pub fn WebPIsAlphaMode(mode: WEBP_CSP_MODE) -> bool {
    use self::WEBP_CSP_MODE::*;
    mode == MODE_RGBA
        || mode == MODE_BGRA
        || mode == MODE_ARGB
        || mode == MODE_RGBA_4444
        || mode == MODE_YUVA
        || WebPIsPremultipliedMode(mode)
}

#[allow(non_snake_case)]
pub fn WebPIsRGBMode(mode: WEBP_CSP_MODE) -> bool {
    use self::WEBP_CSP_MODE::*;
    mode < MODE_YUV
}

// #[repr(transparent)] // TODO: MSRV >= 1.28.0
pub struct WebPDecBuffer<'a>(sys::WebPDecBuffer, PhantomData<&'a mut ()>);

impl<'a> Drop for WebPDecBuffer<'a> {
    fn drop(&mut self) {
        unsafe {
            sys::WebPFreeDecBuffer(&mut self.0);
        }
    }
}

impl<'a> WebPDecBuffer<'a> {
    pub fn colorspace(&self) -> WEBP_CSP_MODE {
        self.0.colorspace
    }

    pub fn set_colorspace(&mut self, colorspace: WEBP_CSP_MODE) {
        assert!(
            self.0.is_external_memory <= 0
                || WebPIsRGBMode(self.0.colorspace) == WebPIsRGBMode(colorspace),
            "unsafe colorspace change",
        );
        self.0.colorspace = colorspace;
    }

    pub fn width(&self) -> u32 {
        self.0.width as u32
    }

    pub fn height(&self) -> u32 {
        self.0.height as u32
    }

    pub fn is_external_memory(&self) -> bool {
        self.0.is_external_memory > 0
    }

    pub fn is_slow_memory(&self) -> bool {
        self.0.is_external_memory > 1
    }

    pub fn set_slow_memory(&mut self, is_slow_memory: bool) {
        if self.0.is_external_memory > 0 {
            self.0.is_external_memory = is_slow_memory as c_int + 1;
        }
    }

    pub fn set_rgba_buffer(&mut self, rgba: &'a mut [u8], stride: u32) {
        assert!(
            self.0.private_memory.is_null(),
            "Internal buffer cannot be turned into external"
        );
        let stride = check_int!(stride);
        if !WebPIsRGBMode(self.0.colorspace) {
            self.0.colorspace = WEBP_CSP_MODE::MODE_RGB;
        }
        if self.0.is_external_memory <= 0 {
            self.0.is_external_memory = 1;
        }
        {
            let rgbabuf = unsafe { &mut self.0.u.RGBA };
            rgbabuf.rgba = rgba.as_mut_ptr();
            rgbabuf.size = rgba.len();
            rgbabuf.stride = stride;
        }
    }

    pub fn with_rgba_buffer<'b>(self, rgba: &'b mut [u8], stride: u32) -> WebPDecBuffer<'b> {
        let mut this = unsafe { mem::transmute::<WebPDecBuffer<'a>, WebPDecBuffer<'b>>(self) };
        this.set_rgba_buffer(rgba, stride);
        this
    }

    pub fn set_yuva_buffer(
        &mut self,
        y: &'a mut [u8],
        u: &'a mut [u8],
        v: &'a mut [u8],
        a: &'a mut [u8],
        y_stride: u32,
        u_stride: u32,
        v_stride: u32,
        a_stride: u32,
    ) {
        assert!(
            self.0.private_memory.is_null(),
            "Internal buffer cannot be turned into external"
        );
        let y_stride = check_int!(y_stride);
        let u_stride = check_int!(u_stride);
        let v_stride = check_int!(v_stride);
        let a_stride = check_int!(a_stride);
        if WebPIsRGBMode(self.0.colorspace) {
            self.0.colorspace = WEBP_CSP_MODE::MODE_YUV;
        }
        if self.0.is_external_memory <= 0 {
            self.0.is_external_memory = 1;
        }
        {
            let yuvabuf = unsafe { &mut self.0.u.YUVA };
            yuvabuf.y = y.as_mut_ptr();
            yuvabuf.u = u.as_mut_ptr();
            yuvabuf.v = v.as_mut_ptr();
            yuvabuf.a = a.as_mut_ptr();
            yuvabuf.y_size = y.len();
            yuvabuf.u_size = u.len();
            yuvabuf.v_size = v.len();
            yuvabuf.a_size = a.len();
            yuvabuf.y_stride = y_stride;
            yuvabuf.u_stride = u_stride;
            yuvabuf.v_stride = v_stride;
            yuvabuf.a_stride = a_stride;
        }
    }

    pub fn with_yuva_buffer<'b>(
        self,
        y: &'b mut [u8],
        u: &'b mut [u8],
        v: &'b mut [u8],
        a: &'b mut [u8],
        y_stride: u32,
        u_stride: u32,
        v_stride: u32,
        a_stride: u32,
    ) -> WebPDecBuffer<'b> {
        let mut this = unsafe { mem::transmute::<WebPDecBuffer<'a>, WebPDecBuffer<'b>>(self) };
        this.set_yuva_buffer(y, u, v, a, y_stride, u_stride, v_stride, a_stride);
        this
    }

    pub fn set_to_internal(&mut self) {
        self.0.is_external_memory = 0;
    }

    pub fn into_internal(self) -> WebPDecBuffer<'static> {
        let mut this = unsafe { mem::transmute::<WebPDecBuffer<'a>, WebPDecBuffer<'static>>(self) };
        this.set_to_internal();
        this
    }
}

impl<'a> fmt::Debug for WebPDecBuffer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: debug-output u
        f.debug_struct("WebPDecBuffer")
            .field("colorspace", &self.0.colorspace)
            .field("width", &self.0.width)
            .field("height", &self.0.height)
            .field("is_external_memory", &self.0.is_external_memory)
            .finish()
    }
}

#[allow(non_snake_case)]
pub fn WebPInitDecBuffer() -> Result<WebPDecBuffer<'static>, WebpUnknownError> {
    // TODO: use MaybeUninit (MSRV >= nightly)
    let mut buffer = unsafe { mem::uninitialized() };
    let res = unsafe { sys::WebPInitDecBuffer(&mut buffer) };
    if res != 0 {
        Ok(WebPDecBuffer(buffer, PhantomData))
    } else {
        mem::forget(buffer);
        Err(WebpUnknownError)
    }
}

pub struct WebPIDecoder<'a>(NonNull<sys::WebPIDecoder>, PhantomData<&'a mut ()>);

impl<'a> Drop for WebPIDecoder<'a> {
    fn drop(&mut self) {
        unsafe {
            sys::WebPIDelete(self.0.as_ptr());
        }
    }
}

impl<'a> WebPIDecoder<'a> {
    pub fn as_ptr(&self) -> *const sys::WebPIDecoder {
        self.0.as_ptr() as *const sys::WebPIDecoder
    }
    pub fn as_mut_ptr(&mut self) -> *mut sys::WebPIDecoder {
        self.0.as_ptr()
    }
}

impl<'a> fmt::Debug for WebPIDecoder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WebPIDecoder { ... }")
    }
}

#[allow(non_snake_case)]
pub fn WebPINewDecoder<'a, 'b>(
    output_buffer: Option<&'b mut WebPDecBuffer<'a>>,
) -> Result<WebPIDecoder<'b>, WebpUnknownError> {
    let output_buffer = match output_buffer {
        None => ptr::null_mut(),
        Some(p) => (&mut p.0) as *mut sys::WebPDecBuffer,
    };
    let res = unsafe { sys::WebPINewDecoder(output_buffer) };
    if let Some(ptr) = NonNull::new(res) {
        Ok(WebPIDecoder(ptr, PhantomData))
    } else {
        Err(WebpUnknownError)
    }
}

fn opt_slice_mut<T>(s: Option<&mut [T]>) -> (*mut T, usize) {
    if let Some(s) = s {
        (s.as_mut_ptr(), s.len())
    } else {
        (ptr::null_mut(), 0)
    }
}

#[allow(non_snake_case)]
pub fn WebPINewRGB<'a>(
    csp: WEBP_CSP_MODE,
    output_buffer: Option<&'a mut [u8]>,
    output_stride: u32,
) -> Result<WebPIDecoder<'a>, WebpUnknownError> {
    let (output_buffer, output_buffer_size) = opt_slice_mut(output_buffer);
    let res = unsafe {
        sys::WebPINewRGB(
            csp,
            output_buffer,
            output_buffer_size,
            check_int!(output_stride),
        )
    };
    if let Some(ptr) = NonNull::new(res) {
        Ok(WebPIDecoder(ptr, PhantomData))
    } else {
        Err(WebpUnknownError)
    }
}

#[allow(non_snake_case)]
pub fn WebPINewYUVA<'a>(
    luma: Option<&'a mut [u8]>,
    luma_stride: u32,
    u: Option<&'a mut [u8]>,
    u_stride: u32,
    v: Option<&'a mut [u8]>,
    v_stride: u32,
    a: Option<&'a mut [u8]>,
    a_stride: u32,
) -> Result<WebPIDecoder<'a>, WebpUnknownError> {
    let (luma, luma_size) = opt_slice_mut(luma);
    let (u, u_size) = opt_slice_mut(u);
    let (v, v_size) = opt_slice_mut(v);
    let (a, a_size) = opt_slice_mut(a);
    let res = unsafe {
        sys::WebPINewYUVA(
            luma,
            luma_size,
            check_int!(luma_stride),
            u,
            u_size,
            check_int!(u_stride),
            v,
            v_size,
            check_int!(v_stride),
            a,
            a_size,
            check_int!(a_stride),
        )
    };
    if let Some(ptr) = NonNull::new(res) {
        Ok(WebPIDecoder(ptr, PhantomData))
    } else {
        Err(WebpUnknownError)
    }
}

#[deprecated(note = "Use WebPINewYUVA.")]
#[allow(non_snake_case)]
pub fn WebPINewYUV<'a>(
    luma: Option<&'a mut [u8]>,
    luma_stride: u32,
    u: Option<&'a mut [u8]>,
    u_stride: u32,
    v: Option<&'a mut [u8]>,
    v_stride: u32,
) -> Result<WebPIDecoder<'a>, WebpUnknownError> {
    let (luma, luma_size) = opt_slice_mut(luma);
    let (u, u_size) = opt_slice_mut(u);
    let (v, v_size) = opt_slice_mut(v);
    let res = unsafe {
        sys::WebPINewYUV(
            luma,
            luma_size,
            check_int!(luma_stride),
            u,
            u_size,
            check_int!(u_stride),
            v,
            v_size,
            check_int!(v_stride),
        )
    };
    if let Some(ptr) = NonNull::new(res) {
        Ok(WebPIDecoder(ptr, PhantomData))
    } else {
        Err(WebpUnknownError)
    }
}

#[allow(non_snake_case)]
pub fn WebPIAppend<'a>(idec: &mut WebPIDecoder<'a>, data: &[u8]) -> VP8StatusCode {
    unsafe { sys::WebPIAppend(idec.as_mut_ptr(), data.as_ptr(), data.len()) }
}

// NOTE: it's OK that `data` is an ephemeral reference,
// because only `WebPIAppend` and `WebPIUpdate` will touch the buffer,
// but `WebPIAppend` after `WebPIUpdate` is forbidden
// and `WebPIUpdate` after `WebPIUpdate` will override the old buffer
// with the newly supplied one.
#[allow(non_snake_case)]
pub fn WebPIUpdate<'a>(idec: &mut WebPIDecoder<'a>, data: &[u8]) -> VP8StatusCode {
    unsafe { sys::WebPIUpdate(idec.as_mut_ptr(), data.as_ptr(), data.len()) }
}

#[allow(non_snake_case)]
pub fn WebPIDecGetRGB<'a, 'b>(
    idec: &'b WebPIDecoder<'a>,
) -> Result<(&'b [u8], u32, u32, u32, u32), WebpUnknownError> {
    let mut last_y: c_int = 0;
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let mut stride: c_int = 0;
    let res = unsafe {
        sys::WebPIDecGetRGB(
            idec.as_ptr(),
            &mut last_y,
            &mut width,
            &mut height,
            &mut stride,
        )
    };
    if !res.is_null() {
        let last_y = last_y as u32;
        let width = width as u32;
        let height = height as u32;
        let stride = stride as u32;
        let data = unsafe { slice::from_raw_parts(res, stride as usize * last_y as usize) };
        Ok((data, last_y, width, height, stride))
    } else {
        Err(WebpUnknownError)
    }
}

#[allow(non_snake_case)]
pub fn WebPIDecGetYUVA<'a, 'b>(
    idec: &'b WebPIDecoder<'a>,
) -> Result<
    (
        &'b [u8],
        u32,
        &'b [u8],
        &'b [u8],
        Option<&'b [u8]>,
        u32,
        u32,
        u32,
        u32,
        u32,
    ),
    WebpUnknownError,
> {
    let mut last_y: c_int = 0;
    let mut u: *mut u8 = ptr::null_mut();
    let mut v: *mut u8 = ptr::null_mut();
    let mut a: *mut u8 = ptr::null_mut();
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let mut stride: c_int = 0;
    let mut uv_stride: c_int = 0;
    let mut a_stride: c_int = 0;
    let res = unsafe {
        sys::WebPIDecGetYUVA(
            idec.as_ptr(),
            &mut last_y,
            &mut u,
            &mut v,
            &mut a,
            &mut width,
            &mut height,
            &mut stride,
            &mut uv_stride,
            &mut a_stride,
        )
    };
    if !res.is_null() {
        let last_y = last_y as u32;
        let width = width as u32;
        let height = height as u32;
        let stride = stride as u32;
        let uv_stride = uv_stride as u32;
        let a_stride = if !a.is_null() { a_stride as u32 } else { 0 };
        let y = unsafe { slice::from_raw_parts(res, last_y as usize * stride as usize) };
        let u = unsafe { slice::from_raw_parts(u, (last_y as usize + 1) / 2 * uv_stride as usize) };
        let v = unsafe { slice::from_raw_parts(v, (last_y as usize + 1) / 2 * uv_stride as usize) };
        let a = if !a.is_null() {
            Some(unsafe { slice::from_raw_parts(a, last_y as usize * a_stride as usize) })
        } else {
            None
        };
        Ok((
            y, last_y, u, v, a, width, height, stride, uv_stride, a_stride,
        ))
    } else {
        Err(WebpUnknownError)
    }
}

#[deprecated(note = "Use WebPIDecGetYUVA.")]
#[allow(non_snake_case)]
pub fn WebPIDecGetYUV<'a, 'b>(
    idec: &'b WebPIDecoder<'a>,
) -> Result<(&'b [u8], u32, &'b [u8], &'b [u8], u32, u32, u32, u32), WebpUnknownError> {
    WebPIDecGetYUVA(idec).map(|result| {
        let (y, last_y, u, v, _, width, height, stride, uv_stride, _) = result;
        (y, last_y, u, v, width, height, stride, uv_stride)
    })
}

#[allow(non_snake_case)]
pub fn WebPIDecodedArea<'a, 'b>(
    idec: &'b WebPIDecoder<'a>,
) -> Result<(&'b WebPDecBuffer<'b>, u32, u32, u32, u32), WebpUnknownError> {
    let mut left: c_int = 0;
    let mut top: c_int = 0;
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let res = unsafe {
        sys::WebPIDecodedArea(idec.as_ptr(), &mut left, &mut top, &mut width, &mut height)
    };
    if !res.is_null() {
        let left = left as u32;
        let top = top as u32;
        let width = width as u32;
        let height = height as u32;
        let buf: *const sys::WebPDecBuffer = res;
        let buf = unsafe { &*(buf as *const WebPDecBuffer) };
        Ok((buf, left, top, width, height))
    } else {
        Err(WebpUnknownError)
    }
}

// #[repr(transparent)] // TODO: MSRV >= 1.28.0
pub struct WebPBitstreamFeatures(sys::WebPBitstreamFeatures);

impl WebPBitstreamFeatures {
    pub fn width(&self) -> u32 {
        self.0.width as u32
    }

    pub fn height(&self) -> u32 {
        self.0.height as u32
    }

    pub fn has_alpha(&self) -> bool {
        self.0.has_alpha != 0
    }

    pub fn has_animation(&self) -> bool {
        self.0.has_animation != 0
    }

    pub fn format(&self) -> WebPBitstreamFormat {
        match self.0.format {
            0 => WebPBitstreamFormat::UNDEFINED,
            1 => WebPBitstreamFormat::LOSSY,
            2 => WebPBitstreamFormat::LOSSLESS,
            _ => unreachable!(),
        }
    }
}

impl<'a> fmt::Debug for WebPBitstreamFeatures {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("WebPBitstreamFeatures")
            .field("width", &self.width())
            .field("height", &self.height())
            .field("has_alpha", &self.has_alpha())
            .field("has_animation", &self.has_animation())
            .field("format", &self.format())
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WebPBitstreamFormat {
    UNDEFINED = 0,
    LOSSY = 1,
    LOSSLESS = 2,
}

#[allow(non_snake_case)]
pub fn WebPGetFeatures(data: &[u8]) -> Result<WebPBitstreamFeatures, VP8StatusCode> {
    let mut features: WebPBitstreamFeatures = unsafe { mem::uninitialized() };
    let res = unsafe { sys::WebPGetFeatures(data.as_ptr(), data.len(), &mut features.0) };
    if res == VP8StatusCode::VP8_STATUS_OK {
        Ok(features)
    } else {
        Err(res)
    }
}

// TODO: wrappers for advanced decoding functions

#[cfg(test)]
mod tests {
    use super::*;

    use test_utils::*;

    #[test]
    fn test_get_info() {
        for test_case in test_cases() {
            let dim = WebPGetInfo(&test_case.webp_data).unwrap();
            assert_eq!(dim, (test_case.image.width(), test_case.image.height()));
        }
    }

    #[test]
    fn test_decode_rgba() {
        for test_case in test_cases() {
            let (width, height, data) = WebPDecodeRGBA(&test_case.webp_data).unwrap();
            let image = Image::new(ColorType::RGBA, width, height, width * 4, data.to_vec());
            assert_abs_diff_eq!(image, test_case.image, epsilon = 1);
        }
    }

    #[test]
    fn test_decode_argb() {
        for test_case in test_cases() {
            let (width, height, data) = WebPDecodeARGB(&test_case.webp_data).unwrap();
            let image = Image::new(ColorType::ARGB, width, height, width * 4, data.to_vec());
            assert_abs_diff_eq!(image, test_case.image, epsilon = 1);
        }
    }

    #[test]
    fn test_decode_bgra() {
        for test_case in test_cases() {
            let (width, height, data) = WebPDecodeBGRA(&test_case.webp_data).unwrap();
            let image = Image::new(ColorType::BGRA, width, height, width * 4, data.to_vec());
            assert_abs_diff_eq!(image, test_case.image, epsilon = 1);
        }
    }

    #[test]
    fn test_decode_rgb() {
        for test_case in test_cases() {
            let (width, height, data) = WebPDecodeRGB(&test_case.webp_data).unwrap();
            let image = Image::new(ColorType::RGB, width, height, width * 3, data.to_vec());
            assert_abs_diff_eq!(image, test_case.image_opaque, epsilon = 1);
        }
    }

    #[test]
    fn test_decode_bgr() {
        for test_case in test_cases() {
            let (width, height, data) = WebPDecodeBGR(&test_case.webp_data).unwrap();
            let image = Image::new(ColorType::BGR, width, height, width * 3, data.to_vec());
            assert_abs_diff_eq!(image, test_case.image_opaque, epsilon = 1);
        }
    }

    #[test]
    fn test_decode_yuv() {
        for test_case in test_cases() {
            let (width, height, y_stride, uv_stride, yuv) =
                WebPDecodeYUV(&test_case.webp_data).unwrap();
            let uv_width = (width + 1) / 2;
            let uv_height = (height + 1) / 2;
            let y = yuv.y().to_vec();
            let u = yuv.u().to_vec();
            let v = yuv.v().to_vec();
            let y = Image::new(ColorType::Grayscale, width, height, y_stride, y);
            let u = Image::new(ColorType::Grayscale, uv_width, uv_height, uv_stride, u);
            let v = Image::new(ColorType::Grayscale, uv_width, uv_height, uv_stride, v);
            assert_abs_diff_eq!(y, test_case.y_image, epsilon = 1);
            assert_abs_diff_eq!(u, test_case.u_image, epsilon = 1);
            assert_abs_diff_eq!(v, test_case.v_image, epsilon = 1);
        }
    }

    #[test]
    fn test_decode_rgba_into() {
        for test_case in test_cases() {
            let (width, height) = WebPGetInfo(&test_case.webp_data).unwrap();
            let mut data = vec![0; width as usize * height as usize * 4];
            WebPDecodeRGBAInto(&test_case.webp_data, &mut data, width * 4).unwrap();
            let image = Image::new(ColorType::RGBA, width, height, width * 4, data);
            assert_abs_diff_eq!(image, test_case.image, epsilon = 1);
        }
    }

    #[test]
    fn test_decode_argb_into() {
        for test_case in test_cases() {
            let (width, height) = WebPGetInfo(&test_case.webp_data).unwrap();
            let mut data = vec![0; width as usize * height as usize * 4];
            WebPDecodeARGBInto(&test_case.webp_data, &mut data, width * 4).unwrap();
            let image = Image::new(ColorType::ARGB, width, height, width * 4, data);
            assert_abs_diff_eq!(image, test_case.image, epsilon = 1);
        }
    }

    #[test]
    fn test_decode_bgra_into() {
        for test_case in test_cases() {
            let (width, height) = WebPGetInfo(&test_case.webp_data).unwrap();
            let mut data = vec![0; width as usize * height as usize * 4];
            WebPDecodeBGRAInto(&test_case.webp_data, &mut data, width * 4).unwrap();
            let image = Image::new(ColorType::BGRA, width, height, width * 4, data);
            assert_abs_diff_eq!(image, test_case.image, epsilon = 1);
        }
    }

    #[test]
    fn test_decode_rgb_into() {
        for test_case in test_cases() {
            let (width, height) = WebPGetInfo(&test_case.webp_data).unwrap();
            let mut data = vec![0; width as usize * height as usize * 3];
            WebPDecodeRGBInto(&test_case.webp_data, &mut data, width * 3).unwrap();
            let image = Image::new(ColorType::RGB, width, height, width * 3, data);
            assert_abs_diff_eq!(image, test_case.image, epsilon = 1);
        }
    }

    #[test]
    fn test_decode_bgr_into() {
        for test_case in test_cases() {
            let (width, height) = WebPGetInfo(&test_case.webp_data).unwrap();
            let mut data = vec![0; width as usize * height as usize * 3];
            WebPDecodeBGRInto(&test_case.webp_data, &mut data, width * 3).unwrap();
            let image = Image::new(ColorType::BGR, width, height, width * 3, data);
            assert_abs_diff_eq!(image, test_case.image, epsilon = 1);
        }
    }

    #[test]
    fn test_decode_yuv_into() {
        for test_case in test_cases() {
            let (width, height) = WebPGetInfo(&test_case.webp_data).unwrap();
            let uv_width = (width + 1) / 2;
            let uv_height = (height + 1) / 2;
            let y_stride = width;
            let u_stride = uv_width;
            let v_stride = uv_width;
            let mut y = vec![0; y_stride as usize * height as usize];
            let mut u = vec![0; u_stride as usize * uv_height as usize];
            let mut v = vec![0; v_stride as usize * uv_height as usize];
            WebPDecodeYUVInto(
                &test_case.webp_data,
                &mut y,
                y_stride,
                &mut u,
                u_stride,
                &mut v,
                v_stride,
            )
            .unwrap();
            let y = Image::new(ColorType::Grayscale, width, height, y_stride, y);
            let u = Image::new(ColorType::Grayscale, uv_width, uv_height, u_stride, u);
            let v = Image::new(ColorType::Grayscale, uv_width, uv_height, v_stride, v);
            assert_abs_diff_eq!(y, test_case.y_image, epsilon = 1);
            assert_abs_diff_eq!(u, test_case.u_image, epsilon = 1);
            assert_abs_diff_eq!(v, test_case.v_image, epsilon = 1);
        }
    }

    #[test]
    fn test_incremental_decode_rgb() {
        let mut buf = WebPInitDecBuffer().unwrap();
        for test_case in test_cases() {
            let mut idec = WebPINewDecoder(Some(&mut buf)).unwrap();
            let mut last_status = VP8StatusCode::VP8_STATUS_SUSPENDED;
            for chunk in test_case.webp_data.chunks(1024) {
                assert_eq!(last_status, VP8StatusCode::VP8_STATUS_SUSPENDED);
                last_status = WebPIAppend(&mut idec, chunk);
            }
            assert_eq!(last_status, VP8StatusCode::VP8_STATUS_OK);
            let (data, last_y, width, height, stride) = WebPIDecGetRGB(&idec).unwrap();
            assert_eq!(last_y, height);
            let image = Image::new(ColorType::RGB, width, height, stride, data.to_vec());
            assert_abs_diff_eq!(image, test_case.image_opaque, epsilon = 1);
        }
    }

    #[test]
    fn test_incremental_decode_rgba() {
        let mut buf = WebPInitDecBuffer().unwrap();
        buf.set_colorspace(WEBP_CSP_MODE::MODE_RGBA);
        for test_case in test_cases() {
            let mut idec = WebPINewDecoder(Some(&mut buf)).unwrap();
            let mut last_status = VP8StatusCode::VP8_STATUS_SUSPENDED;
            for chunk in test_case.webp_data.chunks(1024) {
                assert_eq!(last_status, VP8StatusCode::VP8_STATUS_SUSPENDED);
                last_status = WebPIAppend(&mut idec, chunk);
            }
            assert_eq!(last_status, VP8StatusCode::VP8_STATUS_OK);
            let (data, last_y, width, height, stride) = WebPIDecGetRGB(&idec).unwrap();
            assert_eq!(last_y, height);
            let image = Image::new(ColorType::RGBA, width, height, stride, data.to_vec());
            assert_abs_diff_eq!(image, test_case.image_opaque, epsilon = 1);
        }
    }

    #[test]
    fn test_get_features() {
        let data = b"\
            RIFFV\x00\x00\x00WEBPVP8\x20\
            J\x00\x00\x00\xD0\x01\x00\x9D\x01*\x03\x00\x02\x00\x02\x00\
            4%\xA8\x02t\x01\x0E\xFE\x03\x8E\x00\x00\xFE\xAD\xFF\xF1\
            \x5C\xB4\xF8\xED\xFF\xF0\xC0\xBA\xBF\x93\x05\xEA\x0C\x9F\x93?\
            \xE8\xC0\xBF?\xFF\xA9\xBF\xFF${\xCB\xFFF\x05\xF9\xFF\
            \xFDM\xFE0\xE5\x86\xAA\x071#o\x00\x00\x00";
        let feature = WebPGetFeatures(&data[..32]).unwrap();
        assert_eq!(feature.width(), 3);
        assert_eq!(feature.height(), 2);
        assert_eq!(feature.has_alpha(), false);
        assert_eq!(feature.has_animation(), false);
        assert_eq!(feature.format(), WebPBitstreamFormat::LOSSY);

        assert_eq!(
            WebPGetFeatures(&data[..16]).err(),
            Some(VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA)
        );

        let data = b"\
            RIFFV\x00\x00\x00WEBPVP8\x20\
            K\x00\x00\x00\xD0\x01\x00\x9D\x01*\x03\x00\x02\x00\x02\x00";
        assert_eq!(
            WebPGetFeatures(data).err(),
            Some(VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR)
        );
    }
}
