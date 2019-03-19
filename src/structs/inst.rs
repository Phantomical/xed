use crate::*;
use xed_sys2::xed_interface::*;

#[derive(Copy, Clone, Debug)]
pub struct Inst {
    inner: xed_inst_t,
}

impl Inst {
    pub fn inner_ptr(&self) -> *const xed_inst_t {
        self.inner() as *const _
    }

    pub fn inner(&self) -> &xed_inst_t {
        &self.inner
    }
    pub fn inner_mut(&mut self) -> &mut xed_inst_t {
        &mut self.inner
    }

    pub fn into_inner(self) -> xed_inst_t {
        self.inner
    }
}

impl Inst {
    pub fn iclass(&self) -> IClass {
        unsafe { xed_inst_iclass(self.inner_ptr()).into() }
    }

    pub fn isa_set(&self) -> IsaSet {
        unsafe { xed_inst_isa_set(self.inner_ptr()).into() }
    }

    pub fn noperands(&self) -> u32 {
        unsafe { xed_inst_noperands(self.inner_ptr()) }
    }

    pub fn operand(&self, i: u32) -> Option<&Operand> {
        if i >= self.noperands() {
            return None;
        }

        unsafe { (xed_inst_operand(self.inner_ptr(), i) as *const Operand).as_ref() }
    }

    pub fn category(&self) -> Category {
        unsafe { xed_inst_category(self.inner_ptr()).into() }
    }

    pub fn exception(&self) -> Exception {
        unsafe { xed_inst_exception(self.inner_ptr()).into() }
    }

    pub fn extension(&self) -> Extension {
        unsafe { xed_inst_extension(self.inner_ptr()).into() }
    }

    pub fn iform(&self) -> IForm {
        unsafe { xed_inst_iform_enum(self.inner_ptr()).into() }
    }

    /// Scan for the attribute attr and indicate whether it
    /// is bound.
    pub fn attribute(&self, attr: Attribute) -> bool {
        unsafe { xed_inst_get_attribute(self.inner_ptr(), attr.into()) != 0 }
    }

    /// The attributes bit vector
    pub fn attributes(&self) -> Attributes {
        unsafe { xed_inst_get_attributes(self.inner_ptr()).into() }
    }

    pub fn flag_info_index(&self) -> u32 {
        unsafe { xed_inst_flag_info_index(self.inner_ptr()) }
    }
}
