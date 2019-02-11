use xed_sys2::xed_interface::*;

use crate::util::fmt_helper;
use std::fmt;

macro_rules! bit_accessors {
    { $(
        $(#[$attr:meta])*
        $name:ident => $set_name:ident,
     )* } => {
        impl FlagSet {
            $(
                $(#[$attr])*
                pub fn $name(&self) -> bool {
                    unsafe {
                        self.inner.s.$name() != 0
                    }
                }
                $(#[$attr])*
                pub fn $set_name(&mut self, v: bool) {
                    unsafe {
                        self.inner.s.$set_name(if v { 1 } else { 0 })
                    }
                }
            )*
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct FlagSet {
    inner: xed_flag_set_t,
}

impl FlagSet {
    pub fn bits(&self) -> u32 {
        unsafe { self.inner.flat }
    }

    pub fn mask(&self) -> i32 {
        unsafe { xed_flag_set_mask(&self.inner as *const _) }
    }

    pub fn is_subset_of(&self, other: &Self) -> bool {
        unsafe { xed_flag_set_is_subset_of(&self.inner as *const _, &other.inner as *const _) != 0 }
    }

    pub fn r#if(&self) -> bool {
        self._if()
    }
    pub fn _if(&self) -> bool {
        unsafe { self.inner.s._if() != 0 }
    }
    pub fn set_if(&mut self, v: bool) {
        unsafe { self.inner.s.set__if(if v { 1 } else { 0 }) }
    }

    pub fn iopl(&self) -> u8 {
        unsafe { self.inner.s.iopl() as u8 }
    }
    pub fn set_iopl(&mut self, v: u8) {
        unsafe { self.inner.s.set_iopl(v as u32) }
    }

    pub fn into_inner(&self) -> xed_flag_set_t {
        self.inner
    }
}

bit_accessors! {
    ac => set_ac,
    /// Bit 4
    af => set_af,
    /// Bit 0
    cf => set_cf,
    df => set_df,
    /// x87 flag FC0 (not really part of rflags)
    fc0 => set_fc0,
    /// x87 flag FC1 (not really part of rflags)
    fc1 => set_fc1,
    /// x87 flag FC2 (not really part of rflags)
    fc2 => set_fc2,
    /// x87 flag FC3 (not really part of rflgas)
    fc3 => set_fc3,
    /// Bit 21
    id => set_id,
    nt => set_nt,
    of => set_of,
    pf => set_pf,
    /// Bit 16
    rf => set_rf,
    sf => set_sf,
    /// Bit 8
    tf => set_tf,
    vif => set_vif,
    /// Bit 20
    vip => set_vip,
    vm => set_vm,
    zf => set_zf,
}

impl Default for FlagSet {
    fn default() -> Self {
        unsafe {
            Self {
                inner: std::mem::zeroed(),
            }
        }
    }
}

impl From<xed_flag_set_t> for FlagSet {
    fn from(inner: xed_flag_set_t) -> Self {
        Self { inner }
    }
}

impl fmt::Display for FlagSet {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt_helper(&self.inner as *const _, xed_flag_set_print, fmt)
    }
}
