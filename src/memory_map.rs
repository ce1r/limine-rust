use crate::RequestHeader;

#[derive(Debug)]
pub enum EntryType {
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

#[repr(C, align(8))]
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

#[repr(C)]
#[derive(Debug)]
pub struct MemoryMapResponse {
    revision: u64,
    entry_count: u64,
    entries: *const *const Entry,
}

unsafe impl Send for MemoryMapResponse {}
unsafe impl Sync for MemoryMapResponse {}

impl MemoryMapResponse {
    pub fn entries(&self) -> &[&Entry] {
        unsafe {
            core::slice::from_raw_parts(self.entries.cast::<&Entry>(), self.entry_count as usize)
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Entry {
    pub base: u64,
    pub length: u64,
    pub entry_type: u64,
}

impl Entry {
    pub fn entry_type(&self) -> EntryType {
        match self.entry_type {
            0 => EntryType::Usable,
            1 => EntryType::Reserved,
            2 => EntryType::AcpiReclaimable,
            3 => EntryType::AcpiNVS,
            4 => EntryType::BadMemory,
            5 => EntryType::BootloaderReclaimable,
            6 => EntryType::ExecutableAndModules,
            7 => EntryType::Framebuffer,
            8 => EntryType::ReservedMapped,
            _ => EntryType::Unknown,
        }
    }
}
