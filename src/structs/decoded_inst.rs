use xed_sys2::xed_interface::*;

pub struct DecodedInst {
    pub(crate) inner: xed_decoded_inst_t,
}

impl DecodedInst {
    pub fn conditionally_writes_registers(&self) -> bool {
        unsafe { xed_decoded_inst_conditionally_writes_registers(&self.inner as *const _) != 0 }
    }

    //pub fn get_attribute(&self, )
}
