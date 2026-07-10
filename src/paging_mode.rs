use crate::RequestHeader;

/// Returns a [`PagingModeResponse`].
#[repr(C, align(8))]
#[cfg_attr(test, limine_test::test_layout(limine_paging_mode_request))]
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

/// Returned by [`PagingModeRequest`].
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, limine_test::test_layout(limine_paging_mode_response))]
pub struct PagingModeResponse {
    revision: u64,
    mode: u64,
}

unsafe impl Send for PagingModeResponse {}
unsafe impl Sync for PagingModeResponse {}

impl PagingModeResponse {
    pub const fn mode(&self) -> Option<PagingMode> {
        PagingMode::from_raw(self.mode)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PagingMode {
    #[cfg(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "loongarch64",
    ))]
    FourLevel,

    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    FiveLevel,

    #[cfg(target_arch = "riscv64")]
    Sv39,

    #[cfg(target_arch = "riscv64")]
    Sv48,

    #[cfg(target_arch = "riscv64")]
    Sv57,
}

impl PagingMode {
    #[cfg(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "loongarch64"
    ))]
    pub const DEFAULT: Self = Self::FourLevel;

    #[cfg(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "loongarch64"
    ))]
    pub const MIN: Self = Self::FourLevel;

    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    pub const MAX: Self = Self::FiveLevel;

    #[cfg(target_arch = "loongarch64")]
    pub const MAX: Self = Self::FourLevel;

    #[cfg(target_arch = "riscv64")]
    pub const DEFAULT: Self = Self::Sv48;

    #[cfg(target_arch = "riscv64")]
    pub const MIN: Self = Self::Sv39;

    #[cfg(target_arch = "riscv64")]
    pub const MAX: Self = Self::Sv57;

    pub const fn from_raw(value: u64) -> Option<Self> {
        #[cfg(any(
            target_arch = "x86_64",
            target_arch = "aarch64",
            target_arch = "loongarch64"
        ))]
        {
            match value {
                0 => Some(Self::FourLevel),
                1 => Some(Self::FiveLevel),
                _ => None,
            }
        }

        #[cfg(target_arch = "riscv64")]
        {
            match value {
                0 => Some(Self::Sv39),
                1 => Some(Self::Sv48),
                2 => Some(Self::Sv57),
                _ => None,
            }
        }
    }
}
