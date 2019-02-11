use std::ffi::CStr;
use std::fmt::*;
use std::os::raw::{c_char, c_int};

pub(crate) fn fmt_helper<T, R>(
    p: *const T,
    print: unsafe extern "C" fn(p: *const T, buf: *mut c_char, len: c_int) -> R,
    fmt: &mut Formatter,
) -> Result {
    let mut buf = [0; 2048];

    unsafe {
        print(p, (&mut buf).as_mut_ptr(), buf.len() as c_int);

        let s = CStr::from_ptr((&buf).as_ptr())
            .to_str()
            .expect("Print function wrote out invalid UTF-8");

        fmt.write_str(s)
    }
}
