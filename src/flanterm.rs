use crate::request::RequestHeader;
use core::ffi::c_void;

#[repr(C, align(8))]
pub struct FlantermParamsRequest {
    header: RequestHeader<FlantermParamsResponse>,
}

unsafe impl Send for FlantermParamsRequest {}
unsafe impl Sync for FlantermParamsRequest {}

impl FlantermParamsRequest {
    pub const fn new() -> Self {
        Self {
            header: RequestHeader::new([0x3259399fe7c5f126, 0xe01c1c8c5db9d1a9]),
        }
    }

    pub fn response(&self) -> Option<&'static FlantermParamsResponse> {
        self.header.response()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct FlantermParamsResponse {
    revision: u64,
    entry_count: u64,
    entries: *const *const ParamEntry,
}

unsafe impl Send for FlantermParamsResponse {}
unsafe impl Sync for FlantermParamsResponse {}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ParamEntry {
    pub canvas: *const u32,
    pub canvas_size: u64,
    pub ansi_colours: [u32; 8],
    pub ansi_bright_colours: [u32; 8],
    pub default_bg: u32,
    pub default_fg: u32,
    pub default_bg_bright: u32,
    pub default_fg_bright: u32,
    pub font: *const c_void,
    pub font_width: u64,
    pub font_height: u64,
    pub font_spacing: u64,
    pub font_scale_x: u64,
    pub font_scale_y: u64,
    pub margin: u64,
    pub rotation: u64,
}

#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rotation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
    Unknown,
}

impl ParamEntry {
    pub fn rotation(&self) -> Rotation {
        match self.rotation {
            0 => Rotation::Deg0,
            1 => Rotation::Deg90,
            2 => Rotation::Deg180,
            3 => Rotation::Deg270,
            _ => Rotation::Unknown,
        }
    }
}
