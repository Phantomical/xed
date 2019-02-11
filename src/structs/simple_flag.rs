use crate::*;
use xed_sys2::xed_interface::*;

/// A collection of [`FlagAction`](crate::FlagAction)'s
/// and unions of read and written flags.
#[derive(Copy, Clone, Debug)]
pub struct SimpleFlag {
    inner: xed_simple_flag_t,
}

impl SimpleFlag {
    pub fn into_inner(self) -> xed_simple_flag_t {
        self.inner
    }
}

impl SimpleFlag {
    pub fn fa_index(&self) -> u16 {
        self.inner.fa_index
    }

    /// Indicates the flags are only conditionally written.
    ///
    /// Usually may-writes of the flags instructions that are
    /// dependent on a REP count.
    pub fn may_write(&self) -> bool {
        self.inner.may_write != 0
    }

    /// The always-written flags.
    pub fn must_write(&self) -> bool {
        self.inner.must_write != 0
    }

    /// Number of flag actions associated with this record
    pub fn nflags(&self) -> u8 {
        self.inner.nflags
    }

    /// Union of read flags
    pub fn read(&self) -> FlagSet {
        self.inner.read.into()
    }
    /// Union of written flags (includes undefined flags)
    pub fn written(&self) -> FlagSet {
        self.inner.written.into()
    }
    /// Union of undefined flags
    pub fn undefined(&self) -> FlagSet {
        self.inner.undefined.into()
    }

    /// The specific flag action. Very detailed low-level information.
    pub fn flag_action(&self, i: u32) -> FlagAction {
        unsafe { (*xed_simple_flag_get_flag_action(&self.inner as *const _, i)).into() }
    }

    /// Test to see if flags are read, scans the flags.
    pub fn reads_flags(&self) -> bool {
        unsafe { xed_simple_flag_reads_flags(&self.inner as *const _) != 0 }
    }

    /// Test to see if flags are written, scans the flags.
    pub fn writes_flags(&self) -> bool {
        unsafe { xed_simple_flag_writes_flags(&self.inner as *const _) != 0 }
    }
}

impl Default for SimpleFlag {
    fn default() -> Self {
        unsafe {
            Self {
                inner: std::mem::zeroed(),
            }
        }
    }
}

impl From<xed_simple_flag_t> for SimpleFlag {
    fn from(inner: xed_simple_flag_t) -> Self {
        Self { inner }
    }
}
