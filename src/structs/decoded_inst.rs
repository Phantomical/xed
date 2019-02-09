use xed_sys2::xed_interface::*;

use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DecodedInst {
    pub(crate) inner: xed_decoded_inst_t,
}

impl DecodedInst {
    pub fn conditionally_writes_registers(&self) -> bool {
        unsafe { xed_decoded_inst_conditionally_writes_registers(&self.inner as *const _) != 0 }
    }

    /// The modrm byte of the instruction
    pub fn modrm(&self) -> u8 {
        unsafe { xed_decoded_inst_get_modrm(&self.inner as *const _) }
    }

    /// Indicates whether the instruction has the MPX prefix
    pub fn has_mpx_prefix(&self) -> bool {
        unsafe { xed_decoded_inst_has_mpx_prefix(&self.inner as *const _) != 0 }
    }

    /// Indicates whether the instruction is xacquire
    pub fn is_xaquire(&self) -> bool {
        unsafe { xed_decoded_inst_is_xacquire(&self.inner as *const _) != 0 }
    }

    /// Indicates whether the instruction is xrelease
    pub fn is_xrelease(&self) -> bool {
        unsafe { xed_decoded_inst_is_xrelease(&self.inner as *const _) != 0 }
    }

    /// Indicates whether the instruction uses destination-masking
    pub fn is_masked_vector_operation(&mut self) -> bool {
        unsafe { xed_decoded_inst_masked_vector_operation(&mut self.inner as *mut _) != 0 }
    }

    /// Returns 128, 256, or 512 for operations in the VEX,
    /// EVEX (or XOP) encoding space and returns 0 for
    /// (most) nonvector operations.
    pub fn vector_length_bits(&self) -> u32 {
        unsafe { xed_decoded_inst_vector_length_bits(&self.inner as *const _) }
    }

    /// Indicates whether the instruction is valid.
    pub fn is_valid(&self) -> bool {
        unsafe { xed_decoded_inst_valid(&self.inner as *const _) != 0 }
    }

    /// Return the [`Inst`](crate::Inst) structure for this instruction.
    pub fn inst<'a>(&'a self) -> &'a Inst {
        unsafe {
            (xed_decoded_inst_inst(&self.inner as *const _) as *const Inst)
                .as_ref()
                .expect("Inst returned null pointer")
        }
    }

    /// The [`Category`](crate::Category) of this instruction.
    pub fn category(&self) -> Category {
        unsafe { xed_decoded_inst_get_category(&self.inner as *const _).into() }
    }

    /// The instruction [`Extension`](crate::Extension) for
    /// this instruction.
    pub fn extension(&self) -> Extension {
        unsafe { xed_decoded_inst_get_extension(&self.inner as *const _).into() }
    }

    /// The instruction isa set
    pub fn isa_set(&self) -> IsaSet {
        unsafe { xed_decoded_inst_get_isa_set(&self.inner as *const _).into() }
    }

    /// Instruction class
    pub fn iclass(&self) -> IClass {
        unsafe { xed_decoded_inst_get_iclass(&self.inner as *const _).into() }
    }

    pub fn iform(&self) -> IForm {
        unsafe { 
            xed_decoded_inst_get_iform_enum(
                &self.inner as *const _
            ).into() 
        }
    }

    /// Indicate whether the attribute is defined for this instruction.
    pub fn get_attribute(&self, attr: Attribute) -> bool {
        unsafe { xed_decoded_inst_get_attribute(&self.inner as *const _, attr.into()) != 0 }
    }

    /// Get the attribute bitvector
    pub fn get_attributes(&self) -> xed_attributes_t {
        unsafe { xed_decoded_inst_get_attributes(&self.inner as *const _) }
    }

    pub fn operands<'a>(&'a self) -> OperandValues<'a> {
        unsafe {
            OperandValues::new(
                xed_decoded_inst_operands_const(&self.inner as *const _)
            )
        }
    }

    pub fn operand_length_bits(&self, index: u32) -> u32 {
        unsafe {
            assert!(index < xed_decoded_inst_noperands(&self.inner as *const _));

            xed_decoded_inst_operand_length_bits(&self.inner as *const _, index)
        }
    }

    pub fn operand_elements(&self, index: u32) -> u32 {
        unsafe {
            assert!(index < xed_decoded_inst_noperands(&self.inner as *const _));

            xed_decoded_inst_operand_elements(&self.inner as *const _, index)
        }
    }

    pub fn operand_element_size_bits(&self, index: u32) -> u32 {
        unsafe {
            assert!(index < xed_decoded_inst_noperands(&self.inner as *const _));

            xed_decoded_inst_operand_element_size_bits(&self.inner as *const _, index)
        }
    }

    pub fn operand_element_type(&self, index: u32) -> ElementType {
        unsafe {
            assert!(index < xed_decoded_inst_noperands(&self.inner as *const _));

            xed_decoded_inst_operand_element_type(&self.inner as *const _, index).into()
        }
    }

    pub fn operand_action(&self, index: u32) -> OperandAction {
        unsafe {
            assert!(index < xed_decoded_inst_noperands(&self.inner as *const _));

            xed_decoded_inst_operand_action(&self.inner as *const _, index).into()
        }
    }

    pub fn inner(&self) -> &xed_decoded_inst_t {
        &self.inner
    }
}

