use std::os::raw::*;
use std::ptr;

use mux_types::*;

cfg_if! {
    if #[cfg(feature = "0.5.0")] {
        pub const WEBP_DEMUX_ABI_VERSION: c_int = 0x0107;
    } else {
        pub const WEBP_DEMUX_ABI_VERSION: c_int = 0x0101;
    }
}
// extern {
//     type WebPDemuxer;
// }
#[repr(C)]
pub struct WebPDemuxer(c_void);

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum WebPDemuxState {
    WEBP_DEMUX_PARSE_ERROR = -1,
    WEBP_DEMUX_PARSING_HEADER = 0,
    WEBP_DEMUX_PARSED_HEADER = 1,
    WEBP_DEMUX_DONE = 2,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum WebPFormatFeature {
    WEBP_FF_FORMAT_FLAGS = 0,
    WEBP_FF_CANVAS_WIDTH = 1,
    WEBP_FF_CANVAS_HEIGHT = 2,
    WEBP_FF_LOOP_COUNT = 3,
    WEBP_FF_BACKGROUND_COLOR = 4,
    WEBP_FF_FRAME_COUNT = 5,
}

#[repr(C)]
pub struct WebPIterator {
    pub frame_num: c_int,
    pub num_frames: c_int,
    pub fragment_num: c_int,
    pub num_fragments: c_int,
    pub x_offset: c_int,
    pub y_offset: c_int,
    pub width: c_int,
    pub height: c_int,
    pub duration: c_int,
    pub dispose_method: WebPMuxAnimDispose,
    pub complete: c_int,
    pub fragment: WebPData,
    pub has_alpha: c_int,
    pub blend_method: WebPMuxAnimBlend,
    pub pad: [u32; 2],
    private_: *mut c_void,
}

#[repr(C)]
pub struct WebPChunkIterator {
    pub chunk_num: c_int,
    pub num_chunks: c_int,
    pub chunk: WebPData,
    pub pad: [u32; 6],
    private_: *mut c_void,
}

#[link(name = "webp")]
extern "C" {
    pub fn WebPGetDemuxVersion() -> c_int;
    fn WebPDemuxInternal(
        _: *const WebPData,
        _: c_int,
        _: *mut WebPDemuxState,
        _: c_int,
    ) -> *mut WebPDemuxer;
    pub fn WebPDemuxDelete(dmux: *mut WebPDemuxer);
    pub fn WebPDemuxGetI(dmux: *const WebPDemuxer, feature: WebPFormatFeature) -> u32;
    pub fn WebPDemuxGetFrame(
        dmux: *const WebPDemuxer,
        frame_number: c_int,
        iter: *mut WebPIterator,
    ) -> c_int;
    pub fn WebPDemuxNextFrame(iter: *mut WebPIterator) -> c_int;
    pub fn WebPDemuxPrevFrame(iter: *mut WebPIterator) -> c_int;
    pub fn WebPDemuxSelectFragment(iter: *mut WebPIterator, fragment_num: c_int) -> c_int;
    pub fn WebPDemuxReleaseIterator(iter: *mut WebPIterator);
    pub fn WebPDemuxGetChunk(
        dmux: *const WebPDemuxer,
        fourcc: *const c_char,
        chunk_number: c_int,
        iter: *mut WebPChunkIterator,
    ) -> c_int;
    pub fn WebPDemuxNextChunk(iter: *mut WebPChunkIterator) -> c_int;
    pub fn WebPDemuxPrevChunk(iter: *mut WebPChunkIterator) -> c_int;
    pub fn WebPDemuxReleaseChunkIterator(iter: *mut WebPChunkIterator);
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn WebPDemux(data: *const WebPData) -> *mut WebPDemuxer {
    WebPDemuxInternal(data, 0, ptr::null_mut(), WEBP_DEMUX_ABI_VERSION)
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn WebPDemuxPartial(
    data: *const WebPData,
    state: *mut WebPDemuxState,
) -> *mut WebPDemuxer {
    WebPDemuxInternal(data, 1, state, WEBP_DEMUX_ABI_VERSION)
}
