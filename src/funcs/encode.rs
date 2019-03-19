use crate::*;
use xed_sys2::xed_interface::*;

/// This is the main interface to the encoder.
///
/// The slice should be at most 15 bytes long. If the array is
/// too short, the encoder may fail to encode the request.
///
/// # Parameters
/// - `r`: Encoder request description, includes mode info.
/// - `buf`: The encoded instruction bytes are stored here.
///
/// # Returns
/// The number of bytes that were written to buf, or an error
/// if encoding failed.
pub fn encode(r: &EncoderRequest, buf: &mut [u8]) -> Result<u32> {
    unsafe {
        let mut olen = 0;

        let res = xed_encode(
            // The XED declaration has incorrect mutability
            r.inner_ptr() as *mut _,
            buf.as_mut_ptr(),
            buf.len() as u32,
            &mut olen as *mut _,
        );

        if res != XED_ERROR_NONE {
            return Err(res.into());
        }

        Ok(olen)
    }
}

/// Attempts to encode a NOP of exactly `ilen` bytes.
///
/// If such a NOP is not encodable, then an error will be
/// returned. If `buf` is too small to store the resulting
/// NOP then an error will be returned.
///
/// # Parameters
/// - `buf`: The encoded instruction bytes are stored here.
/// - `ilen`: The desired NOP length. Must be less than `buf.len()`.
pub fn encode_nop(buf: &mut [u8], ilen: u32) -> Result<()> {
    unsafe {
        if ilen as usize >= buf.len() {
            return Err(Error::BufferTooShort);
        }

        let res = xed_encode_nop(buf.as_mut_ptr(), ilen);

        if res != XED_ERROR_NONE {
            return Err(res.into());
        }

        Ok(())
    }
}
