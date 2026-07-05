use crate::request::RequestHeader;
use core::ffi::c_void;

/// Returns a [`FramebufferResponse`].
#[repr(C, align(8))]
#[cfg_attr(test, limine_test::test_layout(limine_framebuffer_request))]
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

/// Returned by [`FramebufferRequest`].
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, limine_test::test_layout(limine_framebuffer_response))]
pub struct FramebufferResponse {
    revision: u64,
    framebuffer_count: u64,
    framebuffers: *const *const Framebuffer,
}

unsafe impl Send for FramebufferResponse {}
unsafe impl Sync for FramebufferResponse {}

impl FramebufferResponse {
    pub const fn framebuffers(&self) -> &[&Framebuffer] {
        unsafe {
            core::slice::from_raw_parts(
                self.framebuffers.cast::<&Framebuffer>(),
                self.framebuffer_count as usize,
            )
        }
    }
}

#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, limine_test::test_layout(limine_framebuffer))]
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
    unused: [u8; 7],
    edid_size: u64,
    edid: *const c_void,
    mode_count: u64,
    modes: *const *const VideoMode,
}

impl Framebuffer {
    pub const fn modes(&self) -> &[&VideoMode] {
        unsafe { core::slice::from_raw_parts(self.modes.cast(), self.mode_count as usize) }
    }

    pub const fn size(&self) -> usize {
        (self.height * self.pitch) as usize
    }

    pub const fn memory_model(&self) -> MemoryModel {
        match self.memory_model {
            1 => MemoryModel::Rgb,
            _ => MemoryModel::Unknown,
        }
    }

    pub const fn edid(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.edid.cast(), self.edid_size as usize) }
    }

    pub const fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.address.cast(), self.size()) }
    }

    pub const fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.address.cast(), self.size()) }
    }
}

#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, limine_test::test_layout(limine_video_mode))]
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

#[derive(Debug)]
pub enum MemoryModel {
    Rgb,
    Unknown,
}
