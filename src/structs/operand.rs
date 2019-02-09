
use crate::*;
use xed_sys2::xed_interface::*;

use std::fmt;
use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub struct OperandValues<'a> {
    values: *const xed_operand_values_t,
    marker: PhantomData<&'a ()>
}

impl<'a> OperandValues<'a> {
    pub(crate) fn new(values: *const xed_operand_values_t) -> Self {
        Self {
            values,
            marker: PhantomData
        }
    }
}

/// Functions
impl<'a> OperandValues<'a> {
    pub fn branch_not_taken_hint(&self) -> bool {
        unsafe {
            xed_operand_values_branch_not_taken_hint(self.values) != 0
        }
    }

    pub fn branch_taken_hint(&self) -> bool {
        unsafe {
            xed_operand_values_branch_taken_hint(self.values) != 0
        }
    }

    pub fn effective_address_width(&self) -> u32 {
        unsafe {
            xed_operand_values_get_effective_address_width(self.values)
        }
    }

    pub fn effective_operand_width(&self) -> u32 {
        unsafe {
            xed_operand_values_get_effective_operand_width(self.values)
        }
    }

    pub fn iclass(&self) -> IClass {
        unsafe {
            xed_operand_values_get_iclass(self.values).into()
        }
    }

    pub fn long_mode(&self) -> bool {
        unsafe {
            xed_operand_values_get_long_mode(self.values) != 0
        }
    }

    pub fn real_mode(&self) -> bool {
        unsafe {
            xed_operand_values_get_real_mode(self.values) != 0
        }
    }

    pub fn stack_address_width(&self) -> u32 {
        unsafe {
            xed_operand_values_get_stack_address_width(self.values)
        }
    }

    pub fn has_66_prefix(&self) -> bool {
        unsafe {
            xed_operand_values_has_66_prefix(self.values) != 0
        }
    }

    pub fn has_address_size_prefix(&self) -> bool {
        unsafe {
            xed_operand_values_has_address_size_prefix(self.values) != 0
        }
    }

    pub fn has_branch_displacement(&self) -> bool {
        unsafe {
            xed_operand_values_has_branch_displacement(self.values) != 0
        }
    }

    pub fn has_displacement(&self) -> bool {
        unsafe {
            xed_operand_values_has_displacement(self.values) != 0
        }
    }

    pub fn has_immediate(&self) -> bool {
        unsafe {
            xed_operand_values_has_immediate(self.values) != 0
        }
    }

    pub fn has_memory_displacement(&self) -> bool {
        unsafe {
            xed_operand_values_has_memory_displacement(self.values) != 0
        }
    }

    pub fn has_operand_size_prefix(&self) -> bool {
        unsafe {
            xed_operand_values_has_operand_size_prefix(self.values) != 0
        }
    }

    pub fn has_rexw_prefix(&self) -> bool {
        unsafe {
            xed_operand_values_has_rexw_prefix(self.values) != 0
        }
    }

    pub fn has_segment_prefix(&self) -> bool {
        unsafe {
            xed_operand_values_has_segment_prefix(self.values) != 0
        }
    }

    pub fn is_nop(&self) -> bool {
        unsafe {
            xed_operand_values_is_nop(self.values) != 0
        }
    }

    pub fn is_prefetch(&self) -> bool {
        unsafe {
            xed_operand_values_is_prefetch(self.values) != 0
        }
    }

    pub fn segment_prefix(&self) -> Reg {
        unsafe {
            xed_operand_values_segment_prefix(self.values).into()
        }
    }

    pub fn using_default_segment(&self, i: u32) -> bool {
        unsafe {
            xed_operand_values_using_default_segment(self.values, i) != 0
        }
    }
}

/// REP/REPNE Prefixes
impl<'a> OperandValues<'a> {
    pub fn has_real_rep(&self) -> bool {
        unsafe {
            xed_operand_values_has_real_rep(self.values) != 0
        }
    }

    pub fn has_rep_prefix(&self) -> bool {
        unsafe {
            xed_operand_values_has_rep_prefix(self.values) != 0
        }
    }

    pub fn has_repne_prefix(&self) -> bool {
        unsafe {
            xed_operand_values_has_repne_prefix(self.values) != 0
        }
    }
}

/// Atomic/Locked operations
impl<'a> OperandValues<'a> {
    pub fn get_atomic(&self) -> bool {
        unsafe {
            xed_operand_values_get_atomic(self.values) != 0
        }
    }

    pub fn has_lock_prefix(&self) -> bool {
        unsafe {
            xed_operand_values_has_lock_prefix(self.values) != 0
        }
    }

    pub fn lockable(&self) -> bool {
        unsafe {
            xed_operand_values_lockable(self.values) != 0
        }
    }
}

/// Memory Addressing
impl<'a> OperandValues<'a> {
    pub fn accesses_memory(&self) -> bool {
        unsafe {
            xed_operand_values_accesses_memory(self.values) != 0
        }
    }

    pub fn no_memory_operands(&self) -> u32 {
        unsafe {
            xed_operand_values_number_of_memory_operands(self.values)
        }
    }

    pub fn memory_operand_length(&self, memop_idx: u32) -> u32 {
        unsafe {
            xed_operand_values_get_memory_operand_length(self.values, memop_idx)
        }
    }

    pub fn base_reg(&self, memop_idx: u32) -> Reg {
        unsafe {
            xed_operand_values_get_base_reg(self.values, memop_idx).into()
        }
    }

    pub fn index_reg(&self, memop_idx: u32) -> Reg {
        unsafe {
            xed_operand_values_get_index_reg(self.values, memop_idx).into()
        }
    }

    pub fn seg_reg(&self, memop_idx: u32) -> Reg {
        unsafe {
            xed_operand_values_get_seg_reg(self.values, memop_idx).into()
        }
    }

    pub fn scale(&self) -> u32 {
        unsafe {
            xed_operand_values_get_scale(self.values)
        }
    }

    pub fn memop_without_modrm(&self) -> bool {
        unsafe {
            xed_operand_values_memop_without_modrm(self.values) != 0
        }
    }

    pub fn has_modrm_byte(&self) -> bool {
        unsafe {
            xed_operand_values_has_modrm_byte(self.values) != 0
        }
    }

    pub fn has_sib_byte(&self) -> bool {
        unsafe {
            xed_operand_values_has_sib_byte(self.values) != 0
        }
    }
}

