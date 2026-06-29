#![no_std]

pub mod date_at_boot;
pub mod efi_system_table;
pub mod executable_address;
pub mod executable_file;
pub mod file;
pub mod firmware_type;
pub mod framebuffer;
pub mod hhdm;
pub mod memory_map;
pub mod mp;
mod request;
pub mod rsdp;
pub mod smbios;
pub mod stack_size;

pub use request::RequestHeader;
pub use request::RequestsEndMarker;
pub use request::RequestsStartMarker;

pub const COMMON_MAGIC: [u64; 2] = [0xc7b1dd30df4c8b88, 0x0a82e883a194f07b];
pub const CURRENT_BASE_REVISION: u64 = 6;

#[repr(C)]
pub struct BaseRevision {
    magic: [u64; 2],
    revision: u64,
}

unsafe impl Send for BaseRevision {}
unsafe impl Sync for BaseRevision {}

impl BaseRevision {
    pub const fn new() -> Self {
        Self {
            magic: [0xf9562b2d5c95a6c8, 0x6a7b384944536bdc],
            revision: CURRENT_BASE_REVISION,
        }
    }

    pub const fn is_supported(&self) -> bool {
        self.revision == 0
    }
}

#[cfg(not(target_arch = "x86_64"))]
compile_error!("Only `x86_64` is supported");
