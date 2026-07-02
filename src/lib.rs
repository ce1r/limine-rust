#![no_std]

pub mod request;
pub mod response;

mod bootloader_info;
mod bootloader_performance;
mod date_at_boot;
mod efi_memory_map;
mod efi_system_table;
mod entry_point;
mod executable_address;
mod executable_cmdline;
mod executable_file;
mod file;
mod firmware_type;
mod flanterm;
mod framebuffer;
mod hhdm;
mod keep_iommu;
mod memory_map;
mod mp;
mod paging_mode;
mod rsdp;
mod smbios;
mod stack_size;
mod tsc_frequency;

pub use file::File;
pub use file::MediaType;
pub use file::Uuid;
pub use firmware_type::FirmwareType;
pub use flanterm::ParamEntry;
pub use flanterm::Rotation;
pub use framebuffer::Framebuffer;
pub use framebuffer::VideoMode;
pub use memory_map::MemoryRegion;
pub use memory_map::MemoryType;
pub use mp::Cpu;
pub use paging_mode::PagingMode;

/// The first 2 magic numbers common to every request.
pub const COMMON_MAGIC: [u64; 2] = [0xc7b1dd30df4c8b88, 0x0a82e883a194f07b];

/// The currently supported base revision (6).
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

#[cfg(test)]
pub mod bindings {
    #![allow(
        non_upper_case_globals,
        non_camel_case_types,
        non_snake_case,
        dead_code
    )]
    include!(concat!(env!("OUT_DIR"), "/limine.rs"));
}

#[doc(hidden)]
#[macro_export]
macro_rules! field_size {
    ($struct:ty, $field:ident) => {{
        fn get_field_size<F: FnOnce(T) -> U, T, U>(_: F) -> usize {
            core::mem::size_of::<U>()
        }

        get_field_size(|s: $struct| s.$field)
    }};
}

#[cfg(not(target_arch = "x86_64"))]
compile_error!("Only `x86_64` is supported");
