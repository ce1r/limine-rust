use crate::RequestHeader;

/// Returns a [`MemoryMapResponse`].
#[repr(C, align(8))]
#[cfg_attr(test, limine_test::test_layout(limine_memmap_request))]
pub struct MemoryMapRequest {
    header: RequestHeader<MemoryMapResponse>,
}

unsafe impl Send for MemoryMapRequest {}
unsafe impl Sync for MemoryMapRequest {}

impl MemoryMapRequest {
    pub const fn new() -> Self {
        Self {
            header: RequestHeader::new([0x67cf3d9d378a806f, 0xe304acdfc50c3c62]),
        }
    }

    pub fn response(&self) -> Option<&'static MemoryMapResponse> {
        self.header.response()
    }
}

/// Returned by [`MemoryMapRequest`].
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, limine_test::test_layout(limine_memmap_response))]
pub struct MemoryMapResponse {
    revision: u64,
    entry_count: u64,
    entries: *const *const MemoryRegion,
}

unsafe impl Send for MemoryMapResponse {}
unsafe impl Sync for MemoryMapResponse {}

impl MemoryMapResponse {
    /// Returns a slice of all the memory regions.
    pub const fn regions(&self) -> &[&MemoryRegion] {
        unsafe { core::slice::from_raw_parts(self.entries.cast(), self.entry_count as usize) }
    }
}

/// A region of physical memory.
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, limine_test::test_layout(limine_memmap_entry))]
pub struct MemoryRegion {
    base: u64,
    length: u64,
    type_: u64,
}

impl MemoryRegion {
    /// Returns the starting physical address of the memory region.
    pub const fn start(&self) -> u64 {
        self.base
    }

    /// Returns the ending physical address (exclusive) of the memory region.
    pub const fn end(&self) -> u64 {
        self.base + self.length
    }

    /// Returns the size of the memory region in bytes.
    pub const fn size(&self) -> u64 {
        self.length
    }

    /// Returns the [`MemoryType`] of the region.
    pub const fn memory_type(&self) -> MemoryType {
        match self.type_ {
            0 => MemoryType::Usable,
            1 => MemoryType::Reserved,
            2 => MemoryType::AcpiReclaimable,
            3 => MemoryType::AcpiNVS,
            4 => MemoryType::BadMemory,
            5 => MemoryType::BootloaderReclaimable,
            6 => MemoryType::ExecutableAndModules,
            7 => MemoryType::Framebuffer,
            8 => MemoryType::ReservedMapped,
            _ => MemoryType::Unknown,
        }
    }

    /// Returns whether the memory region is [`MemoryType::Usable`].
    pub fn is_usable(&self) -> bool {
        self.memory_type() == MemoryType::Usable
    }
}

/// The type of a memory region returned by the bootloader.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MemoryType {
    Usable,
    Reserved,
    AcpiReclaimable,
    AcpiNVS,
    BadMemory,
    BootloaderReclaimable,
    ExecutableAndModules,
    Framebuffer,
    ReservedMapped,
    Unknown,
}
