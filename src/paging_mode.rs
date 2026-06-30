use crate::request::RequestHeader;

#[repr(u64)]
#[derive(Debug, Clone, Copy)]
pub enum PagingMode {
    X86_64_4LVL = 0,
    X86_64_5LVL = 1,
}

impl PagingMode {
    pub const DEFAULT: PagingMode = PagingMode::X86_64_4LVL;
    pub const MIN: PagingMode = PagingMode::X86_64_4LVL;
    pub const MAX: PagingMode = PagingMode::X86_64_5LVL;
}

#[repr(C, align(8))]
pub struct PagingModeRequest {
    header: RequestHeader<PagingModeResponse>,
    mode: u64,
    max_mode: u64,
    min_mode: u64,
}

unsafe impl Send for PagingModeRequest {}
unsafe impl Sync for PagingModeRequest {}

impl PagingModeRequest {
    pub const fn new(mode: PagingMode, min_mode: PagingMode, max_mode: PagingMode) -> Self {
        let mode = mode as u64;
        let min_mode = min_mode as u64;
        let max_mode = max_mode as u64;

        assert!(min_mode <= mode && mode <= max_mode);

        Self {
            header: RequestHeader::new([0x95c1a0edab0944cb, 0xa4e5cb3842f7488a]),
            mode,
            max_mode,
            min_mode,
        }
    }

    pub const fn new_exact(mode: PagingMode) -> Self {
        let mode = mode as u64;

        Self {
            header: RequestHeader::new([0x95c1a0edab0944cb, 0xa4e5cb3842f7488a]),
            mode,
            max_mode: mode,
            min_mode: mode,
        }
    }

    pub const fn new_default() -> Self {
        let mode = PagingMode::DEFAULT as u64;

        Self {
            header: RequestHeader::new([0x95c1a0edab0944cb, 0xa4e5cb3842f7488a]),
            mode,
            max_mode: mode,
            min_mode: mode,
        }
    }

    pub fn response(&self) -> Option<&'static PagingModeResponse> {
        self.header.response()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct PagingModeResponse {
    revision: u64,
    mode: u64,
}

unsafe impl Send for PagingModeResponse {}
unsafe impl Sync for PagingModeResponse {}

impl PagingModeResponse {
    pub fn mode(&self) -> Option<PagingMode> {
        match self.mode {
            0 => Some(PagingMode::X86_64_4LVL),
            1 => Some(PagingMode::X86_64_5LVL),
            _ => None,
        }
    }
}
