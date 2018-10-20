use std::os::raw::*;

use mux_types::*;

cfg_if! {
    if #[cfg(feature = "0.6.0")] {
        pub const WEBP_MUX_ABI_VERSION: c_int = 0x0108;
    } else if #[cfg(feature = "0.6.0-rc2")] {
        pub const WEBP_MUX_ABI_VERSION: c_int = 0x0107;
    } else if #[cfg(feature = "0.5.0")] {
        pub const WEBP_MUX_ABI_VERSION: c_int = 0x0106;
    } else {
        pub const WEBP_MUX_ABI_VERSION: c_int = 0x0101;
    }
}
// extern {
//     type WebPMux;
// }
#[repr(C)]
pub struct WebPMux(c_void);

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum WebPMuxError {
    WEBP_MUX_OK = 1,
    WEBP_MUX_NOT_FOUND = 0,
    WEBP_MUX_INVALID_ARGUMENT = -1,
    WEBP_MUX_BAD_DATA = -2,
    WEBP_MUX_MEMORY_ERROR = -3,
    WEBP_MUX_NOT_ENOUGH_DATA = -4,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum WebPChunkId {
    WEBP_CHUNK_VP8X,
    WEBP_CHUNK_ICCP,
    WEBP_CHUNK_ANIM,
    WEBP_CHUNK_ANMF,
    WEBP_CHUNK_FRGM,
    WEBP_CHUNK_ALPHA,
    WEBP_CHUNK_IMAGE,
    WEBP_CHUNK_EXIF,
    WEBP_CHUNK_XMP,
    WEBP_CHUNK_UNKNOWN,
    WEBP_CHUNK_NIL,
}

#[repr(C)]
pub struct WebPMuxFrameInfo {
    pub bitstream: WebPData,
    pub x_offset: c_int,
    pub y_offset: c_int,
    pub duration: c_int,
    pub id: WebPChunkId,
    pub dispose_method: WebPMuxAnimDispose,
    pub blend_method: WebPMuxAnimBlend,
    pub pad: [u32; 1],
}

#[repr(C)]
pub struct WebPMuxAnimParams {
    pub bgcolor: u32,
    pub loop_count: c_int,
}

#[link(name = "webp")]
extern "C" {
    pub fn WebPGetMuxVersion() -> c_int;
    pub fn WebPNewInternal(_: c_int) -> *mut WebPMux;
    pub fn WebPMuxDelete(mux: *mut WebPMux);
    pub fn WebPMuxCreateInternal(_: *const WebPData, _: c_int, _: c_int) -> *mut WebPMux;
    pub fn WebPMuxSetChunk(
        mux: *mut WebPMux,
        fourcc: *const c_char,
        chunk_data: *const WebPData,
        copy_data: c_int,
    ) -> WebPMuxError;
    pub fn WebPMuxGetChunk(
        mux: *const WebPMux,
        fourcc: *const c_char,
        chunk_data: *mut WebPData,
    ) -> WebPMuxError;
    pub fn WebPMuxDeleteChunk(mux: *mut WebPMux, fourcc: *const c_char) -> WebPMuxError;
    pub fn WebPMuxSetImage(
        mux: *mut WebPMux,
        bitstream: *const WebPData,
        copy_data: c_int,
    ) -> WebPMuxError;
    pub fn WebPMuxPushFrame(
        mux: *mut WebPMux,
        frame: *const WebPMuxFrameInfo,
        copy_data: c_int,
    ) -> WebPMuxError;
    pub fn WebPMuxGetFrame(
        mux: *const WebPMux,
        nth: u32,
        frame: *mut WebPMuxFrameInfo,
    ) -> WebPMuxError;
    pub fn WebPMuxDeleteFrame(mux: *mut WebPMux, nth: u32) -> WebPMuxError;
    pub fn WebPMuxSetAnimationParams(
        mux: *mut WebPMux,
        params: *const WebPMuxAnimParams,
    ) -> WebPMuxError;
    pub fn WebPMuxGetAnimationParams(
        mux: *const WebPMux,
        params: *mut WebPMuxAnimParams,
    ) -> WebPMuxError;
    // #if WEBP_MUX_ABI_VERSION > 0x0101
    // pub fn WebPMuxSetCanvasSize(mux: *mut WebPMux, width: c_int, height: c_int) -> WebPMuxError;
    // #endif
    pub fn WebPMuxGetCanvasSize(
        mux: *const WebPMux,
        width: *mut c_int,
        height: *mut c_int,
    ) -> WebPMuxError;
    pub fn WebPMuxGetFeatures(mux: *const WebPMux, flags: *mut u32) -> WebPMuxError;
    pub fn WebPMuxNumChunks(
        mux: *const WebPMux,
        id: WebPChunkId,
        num_elements: *mut c_int,
    ) -> WebPMuxError;
    pub fn WebPMuxAssemble(mux: *mut WebPMux, assembled_data: *mut WebPData) -> WebPMuxError;
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn WebPMuxNew() -> *mut WebPMux {
    WebPNewInternal(WEBP_MUX_ABI_VERSION)
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn WebPMuxCreate(
    bitstream: *const WebPData,
    copy_data: c_int,
) -> *mut WebPMux {
    WebPMuxCreateInternal(bitstream, copy_data, WEBP_MUX_ABI_VERSION)
}
