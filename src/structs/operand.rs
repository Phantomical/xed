use static_assertions::assert_eq_size;
use xed_sys2::xed_interface::*;

use crate::*;

use std::fmt;

// Note: need repr(C) here since there are instances
//       of *const xed_operand_t being cast to instances
//       of *const Operand elsewhere within the program.
//       repr(C) should hopefully be enough to ensure
//       that Operand is layout-compatible with
//       xed_operand_t
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Operand {
    inner: xed_operand_t,
}

assert_eq_size!(size_is_consistent; Operand, xed_operand_t);

impl Operand {
    fn inner_ptr(&self) -> *const xed_operand_t {
        &self.inner as *const _
    }

    pub fn into_inner(self) -> xed_operand_t {
        self.inner
    }

    pub fn rw(&self) -> OperandAction {
        unsafe { xed_operand_rw(self.inner_ptr()).into() }
    }

    pub fn imm(&self) -> u32 {
        unsafe { xed_operand_imm(self.inner_ptr()) }
    }

    pub fn reg(&self) -> Reg {
        unsafe { xed_operand_reg(self.inner_ptr()).into() }
    }

    pub fn name(&self) -> OperandEnum {
        unsafe { xed_operand_name(self.inner_ptr()).into() }
    }

    /// Whether the operand is read, including conditional reads
    pub fn read(&self) -> bool {
        unsafe { xed_operand_read(self.inner_ptr()) != 0 }
    }

    pub fn ty(&self) -> OperandType {
        unsafe { xed_operand_type(self.inner_ptr()).into() }
    }

    pub fn width(&self) -> OperandWidth {
        unsafe { xed_operand_width(self.inner_ptr()).into() }
    }

    pub fn xtype(&self) -> OperandElementXType {
        unsafe { xed_operand_xtype(self.inner_ptr()).into() }
    }

    /// Whether the operand is written, including conditional writes.
    pub fn written(&self) -> bool {
        unsafe { xed_operand_written(self.inner_ptr()) != 0 }
    }

    /// Whether the operand is read-only, including conditional reads.
    pub fn read_only(&self) -> bool {
        unsafe { xed_operand_read_only(self.inner_ptr()) != 0 }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        util::fmt_helper(self.inner_ptr(), xed_operand_print, fmt)
    }
}
