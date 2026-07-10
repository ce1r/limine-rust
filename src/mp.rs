use crate::RequestHeader;

/// Returns a [`MpResponse`].
#[repr(C, align(8))]
#[cfg_attr(test, limine_test::test_layout(limine_mp_request))]
pub struct MpRequest {
    header: RequestHeader<MpResponse>,
    flags: u64,
}

unsafe impl Send for MpRequest {}
unsafe impl Sync for MpRequest {}

impl MpRequest {
    pub const fn new(#[cfg(target_arch = "x86_64")] enable_x2apic: bool) -> Self {
        Self {
            header: RequestHeader::new([0x95a67b819a1b857e, 0xa0b61b723b6a73e0]),

            #[cfg(target_arch = "x86_64")]
            flags: enable_x2apic as u64,

            #[cfg(not(target_arch = "x86_64"))]
            flags: 0,
        }
    }

    pub fn response(&self) -> Option<&'static MpResponse> {
        self.header.response()
    }
}

cfg_select! {
    target_arch = "x86_64" => {
        include!("mp/x86_64.rs");
    }

    target_arch = "aarch64" => {
        include!("mp/aarch64.rs");
    }

    target_arch = "riscv64" => {
        include!("mp/riscv64.rs");
    }

    target_arch = "loongarch64" => {
        include!("mp/loongarch64.rs");
    }
}
