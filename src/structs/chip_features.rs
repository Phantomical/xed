
use xed_sys2::xed_interface::*;

use crate::{Chip, IsaSet};

#[derive(Copy, Clone)]
pub struct ChipFeatures {
    inner: xed_chip_features_t
}

impl ChipFeatures {
    pub fn new(chip: Chip) -> Self {
        unsafe {
            let mut features = Self::zeroed();

            xed_get_chip_features(
                features.inner_mut(),
                chip.into()
            );

            features.into()
        }
    }

    pub fn zeroed() -> Self {
        unsafe {
            Self {
                inner: std::mem::zeroed()
            }
        }
    }

    pub fn modify(&mut self, isa_set: IsaSet, present: bool) {
        unsafe {
            xed_modify_chip_features(
                &mut self.inner as *mut _,
                isa_set.into(),
                if present { 1 } else { 0 }
            );
        }
    }

    pub fn into_inner(self) -> xed_chip_features_t {
        self.inner
    }

    pub fn inner(&self) -> &xed_chip_features_t {
        &self.inner 
    }
    pub fn inner_mut(&mut self) -> &mut xed_chip_features_t {
        &mut self.inner
    }
}

impl From<xed_chip_features_t> for ChipFeatures {
    fn from(x: xed_chip_features_t) -> Self {
        Self {
            inner: x
        }
    }
}

impl Default for ChipFeatures {
    fn default() -> Self {
        Self::zeroed()
    }
}
