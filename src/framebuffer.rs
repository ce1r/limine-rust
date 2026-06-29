use crate::RequestHeader;
use core::ffi::c_void;

#[repr(C, align(8))]
pub struct FramebufferRequest {
    header: RequestHeader<FramebufferResponse>,
}

unsafe impl Send for FramebufferRequest {}
unsafe impl Sync for FramebufferRequest {}

impl FramebufferRequest {
    pub const fn new() -> Self {
        Self {
            header: RequestHeader::new([0x9d5827dcd881dd75, 0xa3148604f6fab11b]),
        }
    }

    pub fn response(&self) -> Option<&'static FramebufferResponse> {
        self.header.response()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct FramebufferResponse {
    revision: u64,
    framebuffer_count: u64,
    framebuffers: *const *const Framebuffer,
}

unsafe impl Send for FramebufferResponse {}
unsafe impl Sync for FramebufferResponse {}

impl FramebufferResponse {
    pub fn framebuffers(&self) -> &[&Framebuffer] {
        unsafe {
            core::slice::from_raw_parts(
                self.framebuffers.cast::<&Framebuffer>(),
                self.framebuffer_count as usize,
            )
        }
    }
}

#[derive(Debug)]
pub enum MemoryModel {
    RGB,
    Unknown,
}

#[repr(C)]
#[derive(Debug)]
pub struct Framebuffer {
    address: *mut c_void,
    pub width: u64,
    pub height: u64,
    pub pitch: u64,
    pub bpp: u16,
    pub memory_model: u8,
    pub red_mask_size: u8,
    pub red_mask_shift: u8,
    pub green_mask_size: u8,
    pub green_mask_shift: u8,
    pub blue_mask_size: u8,
    pub blue_mask_shift: u8,
    _reserved: [u8; 7],
    edid_size: u64,
    edid: *mut c_void,
    mode_count: u64,
    modes: *const *const VideoMode,
}

impl Framebuffer {
    pub fn modes(&self) -> &[&VideoMode] {
        unsafe {
            core::slice::from_raw_parts(self.modes.cast::<&VideoMode>(), self.mode_count as usize)
        }
    }

    pub fn size(&self) -> usize {
        (self.height * self.pitch) as usize
    }

    pub fn memory_model(&self) -> MemoryModel {
        match self.memory_model {
            1 => MemoryModel::RGB,
            _ => MemoryModel::Unknown,
        }
    }

    pub fn edid(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.edid.cast(), self.edid_size as usize) }
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.address.cast(), self.size()) }
    }

    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.address.cast(), self.size()) }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct VideoMode {
    pub pitch: u64,
    pub width: u64,
    pub height: u64,
    pub bpp: u16,
    pub memory_model: u8,
    pub red_mask_size: u8,
    pub red_mask_shift: u8,
    pub green_mask_size: u8,
    pub green_mask_shift: u8,
    pub blue_mask_size: u8,
    pub blue_mask_shift: u8,
}
