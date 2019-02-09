use xed_sys2::xed_interface::*;

#[repr(C)]
#[derive(Debug)]
pub struct Inst {
    pub(crate) inner: xed_inst_t,
}
