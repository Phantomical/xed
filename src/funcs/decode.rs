use std::os::raw::c_uint;
use xed_sys2::xed_interface::*;

use crate::{ChipFeatures, DecodedInst, Result};
use std::mem;

pub fn decode<'a>(bytes: &'a [u8]) -> Result<DecodedInst> {
    unsafe {
        let mut inst: DecodedInst = mem::zeroed();

        let err = xed_decode(
            &mut inst.inner as *mut _,
            bytes.as_ptr(),
            bytes.len() as c_uint,
        );

        if err != XED_ERROR_NONE {
            return Err(err.into());
        }

        Ok(inst)
    }
}

pub fn decode_with_features<'a>(
    bytes: &'a [u8],
    features: &mut ChipFeatures,
) -> Result<DecodedInst> {
    unsafe {
        let mut inst: DecodedInst = mem::zeroed();

        let err = xed_decode_with_features(
            &mut inst.inner as *mut _,
            bytes.as_ptr(),
            bytes.len() as c_uint,
            &mut features.inner as *mut _,
        );

        if err != XED_ERROR_NONE {
            return Err(err.into());
        }

        Ok(inst)
    }
}
