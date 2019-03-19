use crate::{ChipFeatures, DecodedInst, Error};
use xed_sys2::xed_interface::*;

use std::mem;

/// This is the main interface to the decoder.
///
/// # Parameters
/// - `itext`: The slice of instruction text bytes. 1 to 15 bytes,
///            anything more is ignored.
///
/// # Returns
/// A result indicating success or failure. Note failure can be
/// due to not enough bytes in the input array.
///
/// The maximum instruction is 15B and XED will tell you how long
/// the instruction actually is if you call `DecodedInst::get_length()`.
/// However, it is not always safe or advisable for XED to read 15
/// bytes is the decode location is at the boundary of some protection
/// limit. For example, if one is decoding near the end of a page and
/// the XED user does not want to cause extra page faults, one might
/// send in the number of bytes that would stop at the page boundary.
/// In this case, XED might not be able to decode the instruction and
/// would return an error. The XED user would then have to decide if it
/// was safe to touch the next page and try again to decode with more
/// bytes. Also sometimes the user process does not have read access to
/// the next page and this allows the user to prevent XED from causing
/// process termination by limiting the memory rnage that XED will access.
pub fn decode(itext: &[u8]) -> Result<DecodedInst, Error> {
    unsafe {
        let mut inst: xed_decoded_inst_t = mem::zeroed();

        let res = xed_decode(&mut inst as *mut _, itext.as_ptr(), itext.len() as u32);

        if res != XED_ERROR_NONE {
            return Err(res.into());
        }

        Ok(inst.into())
    }
}

/// Similar to `xed_decode()`.
///
/// This version of the decode API adds a CPUID feature vector to
/// support restricting decode based on both a specified chip via
/// `DecodedInst::set_input_chip()` and a modify-able cpuid vector
/// obtained via `ChipFeatures::new()`.
pub fn decode_with_features(
    itext: &[u8],
    features: &mut ChipFeatures,
) -> Result<DecodedInst, Error> {
    unsafe {
        let mut inst: xed_decoded_inst_t = mem::zeroed();

        let res = xed_decode_with_features(
            &mut inst as *mut _,
            itext.as_ptr(),
            itext.len() as u32,
            features.inner_mut() as *mut _,
        );

        if res != XED_ERROR_NONE {
            return Err(res.into());
        }

        Ok(inst.into())
    }
}
