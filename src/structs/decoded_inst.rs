use static_assertions::assert_eq_size;
use xed_sys2::xed_interface::*;

use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DecodedInst {
    inner: xed_decoded_inst_t,
}

assert_eq_size!(decoded_inst; DecodedInst, xed_decoded_inst_t);

impl DecodedInst {
    fn inner_ptr(&self) -> *const xed_decoded_inst_t {
        &self.inner as *const _
    }

    fn inner_ptr_mut(&mut self) -> *mut xed_decoded_inst_t {
        &mut self.inner as *mut _
    }
}

// xed_decoded_inst_*
impl DecodedInst {
    pub fn conditionally_writes_registers(&self) -> bool {
        unsafe { xed_decoded_inst_conditionally_writes_registers(self.inner_ptr()) != 0 }
    }

    /// Indicates whether the attribute is defined
    /// for this instruction.
    pub fn attribute(&self, attr: Attribute) -> bool {
        unsafe { xed_decoded_inst_get_attribute(self.inner_ptr(), attr.into()) != 0 }
    }

    /// Returns the attribute bitvector
    pub fn attributes(&self) -> Attributes {
        unsafe { xed_decoded_inst_get_attributes(self.inner_ptr()).into() }
    }

    pub fn base_reg(&self, mem_idx: u32) -> Reg {
        unsafe { xed_decoded_inst_get_base_reg(self.inner_ptr(), mem_idx).into() }
    }

    pub fn branch_displacement(&self) -> i32 {
        unsafe { xed_decoded_inst_get_branch_displacement(self.inner_ptr()) }
    }

    /// Result in bytes
    pub fn branch_displacement_width(&self) -> u32 {
        unsafe { xed_decoded_inst_get_branch_displacement_width(self.inner_ptr()) }
    }

    /// Result in bits
    pub fn branch_displacement_width_bits(&self) -> u32 {
        unsafe { xed_decoded_inst_get_branch_displacement_width_bits(self.inner_ptr()) }
    }

    pub fn byte(&self, byte_index: u32) -> u8 {
        unsafe { xed_decoded_inst_get_byte(self.inner_ptr(), byte_index) }
    }

    pub fn category(&self) -> Category {
        unsafe { xed_decoded_inst_get_category(self.inner_ptr()).into() }
    }

    pub fn extension(&self) -> Extension {
        unsafe { xed_decoded_inst_get_extension(self.inner_ptr()).into() }
    }

    pub fn iclass(&self) -> IClass {
        unsafe { xed_decoded_inst_get_iclass(self.inner_ptr()).into() }
    }

    pub fn iform(&self) -> IForm {
        unsafe { xed_decoded_inst_get_iform_enum(self.inner_ptr()).into() }
    }

    /// Returns true if the first immediate (IMM0) is signed
    pub fn immediate_is_signed(&self) -> bool {
        unsafe { xed_decoded_inst_get_immediate_is_signed(self.inner_ptr()) != 0 }
    }

    /// Return the immediate width in bytes
    pub fn immediate_width(&self) -> u32 {
        unsafe { xed_decoded_inst_get_immediate_width(self.inner_ptr()) }
    }

    /// Return the immediate width in bits
    pub fn immediate_width_bits(&self) -> u32 {
        unsafe { xed_decoded_inst_get_immediate_width_bits(self.inner_ptr()) }
    }

    pub fn index_reg(&self, mem_idx: u32) -> Reg {
        unsafe { xed_decoded_inst_get_index_reg(self.inner_ptr(), mem_idx).into() }
    }

    pub fn input_chip(&self) -> Chip {
        unsafe { xed_decoded_inst_get_input_chip(self.inner_ptr()).into() }
    }

    pub fn isa_set(&self) -> IsaSet {
        unsafe { xed_decoded_inst_get_isa_set(self.inner_ptr()).into() }
    }

    pub fn length(&self) -> u32 {
        unsafe { xed_decoded_inst_get_length(self.inner_ptr()) }
    }

    pub fn machine_mode_bits(&self) -> u32 {
        unsafe { xed_decoded_inst_get_machine_mode_bits(self.inner_ptr()) }
    }

    /// Returns the addressing width in bits (16, 32, 64)
    /// for MEM1 (if `memop_idx == 0`) or MEM1 (if
    /// `memop_idx == 1`).
    pub fn memop_address_width(&self, memop_idx: u32) -> u32 {
        unsafe { xed_decoded_inst_get_memop_address_width(self.inner_ptr(), memop_idx) }
    }

    pub fn memory_displacement(&self, mem_idx: u32) -> i64 {
        unsafe { xed_decoded_inst_get_memory_displacement(self.inner_ptr(), mem_idx) }
    }

    pub fn memory_displacement_width(&self, mem_idx: u32) -> u32 {
        unsafe { xed_decoded_inst_get_memory_displacement_width(self.inner_ptr(), mem_idx) }
    }

    pub fn memory_displacement_width_bits(&self, mem_idx: u32) -> u32 {
        unsafe { xed_decoded_inst_get_memory_displacement_width_bits(self.inner_ptr(), mem_idx) }
    }

    pub fn memory_operand_length(&self, memop_idx: u32) -> u32 {
        unsafe { xed_decoded_inst_get_memory_operand_length(self.inner_ptr(), memop_idx) }
    }

    pub fn modrm(&self) -> u8 {
        unsafe { xed_decoded_inst_get_modrm(self.inner_ptr()) }
    }

    /// Returns the number of legacy prefixes
    pub fn nprefixes(&self) -> u32 {
        unsafe { xed_decoded_inst_get_nprefixes(self.inner_ptr()) }
    }

    /// Returns the operand width in bits: 8/16/32/64.
    ///
    /// This is different than [`effective_operand_width()`]
    /// which only returns 16/32/64. This factors in the
    /// BYTEOP attribute when computing its return value. This
    /// function provides iformation that is only useful for
    /// (scalable) GPR-operations. Individual operands have more
    /// specific information available from
    /// [`element_size_bits()`].
    pub fn operand_width(&self) -> u32 {
        unsafe { xed_decoded_inst_get_operand_width(self.inner_ptr()) }
    }

    /// Return the specified register operand. The specifier
    /// is of type [`OperandEnum`](crate::OperandEnum).
    pub fn reg(&self, reg_operand: OperandEnum) -> Reg {
        unsafe { xed_decoded_inst_get_reg(self.inner_ptr(), reg_operand.into()).into() }
    }

    /// See the commend on [`uses_rflags()`]. This can return
    /// 0 if the flags are really not used by this instruction.
    pub fn rflags_info(&self) -> SimpleFlag {
        unsafe { (*xed_decoded_inst_get_rflags_info(self.inner_ptr())).into() }
    }

    pub fn scale(&self, mem_idx: u32) -> u32 {
        unsafe { xed_decoded_inst_get_scale(self.inner_ptr(), mem_idx) }
    }

    pub fn second_immediate(&self) -> u8 {
        unsafe { xed_decoded_inst_get_second_immediate(self.inner_ptr()) }
    }

    pub fn seg_reg(&self, mem_idx: u32) -> Reg {
        unsafe { xed_decoded_inst_get_seg_reg(self.inner_ptr(), mem_idx).into() }
    }

    pub fn signed_immediate(&self) -> i32 {
        unsafe { xed_decoded_inst_get_signed_immediate(self.inner_ptr()) }
    }

    pub fn stack_address_mode_bits(&self) -> u32 {
        unsafe { xed_decoded_inst_get_stack_address_mode_bits(self.inner_ptr()) }
    }

    pub fn unsigned_immediate(&self) -> u64 {
        unsafe { xed_decoded_inst_get_unsigned_immediate(self.inner_ptr()) }
    }

    pub fn has_mpx_prefix(&self) -> bool {
        unsafe { xed_decoded_inst_has_mpx_prefix(self.inner_ptr()) != 0 }
    }

    pub fn inst(&self) -> &Inst {
        unsafe {
            (xed_decoded_inst_inst(self.inner_ptr()) as *const Inst)
                .as_ref()
                .expect("xed_decoded_inst_inst returned null pointer")
        }
    }

    /// Returns true for broadcast instructions or AVX512 load-op
    /// instructions using the broadcast feature and false otherwise.
    ///
    /// Logical OR of [`is_broadcast_instruction()`] and
    /// [`uses_embedded_broadcast()`].
    pub fn is_broadcast(&self) -> bool {
        unsafe { xed_decoded_inst_is_broadcast(self.inner_ptr()) != 0 }
    }

    /// Indicate whether the instruction is a broadcast instruction.
    /// (Not including ACX512 load-op instaructions).
    ///
    /// Just a category check.
    pub fn is_broadcast_instruction(&self) -> bool {
        unsafe { xed_decoded_inst_is_broadcast_instruction(self.inner_ptr()) != 0 }
    }

    /// Indicate whether the instruction is a prefetch.
    pub fn is_prefetch(&self) -> bool {
        unsafe { xed_decoded_inst_is_prefetch(self.inner_ptr()) != 0 }
    }

    /// Indicate whether the instruction is xacquire.
    pub fn is_xacquire(&self) -> bool {
        unsafe { xed_decoded_inst_is_xacquire(self.inner_ptr()) != 0 }
    }

    /// Indicate whether the instruction is xrelease.
    pub fn is_xrelease(&self) -> bool {
        unsafe { xed_decoded_inst_is_xrelease(self.inner_ptr()) != 0 }
    }

    /// Returns true iff the instruction uses destination-masking.
    ///
    /// This is 0 for blend operations that use their mask field
    /// as a control.
    pub fn masked_vector_operation(&mut self) -> bool {
        unsafe { xed_decoded_inst_masked_vector_operation(&mut self.inner as *mut _) != 0 }
    }

    /// Indicates whether the instruction uses write-masking.
    pub fn masking(&self) -> bool {
        unsafe { xed_decoded_inst_masking(self.inner_ptr()) != 0 }
    }

    pub fn mem_read(&self, mem_idx: u32) -> bool {
        unsafe { xed_decoded_inst_mem_read(self.inner_ptr(), mem_idx) != 0 }
    }

    pub fn mem_written(&self, mem_idx: u32) -> bool {
        unsafe { xed_decoded_inst_mem_written(self.inner_ptr(), mem_idx) != 0 }
    }

    pub fn mem_written_only(&self, mem_idx: u32) -> bool {
        unsafe { xed_decoded_inst_mem_written_only(self.inner_ptr(), mem_idx) != 0 }
    }

    /// Indicates whether the instruction uses write-masking with
    /// merging.
    pub fn merging(&self) -> bool {
        unsafe { xed_decoded_inst_merging(self.inner_ptr()) != 0 }
    }

    pub fn noperands(&self) -> u32 {
        unsafe { xed_decoded_inst_noperands(self.inner_ptr()) }
    }

    pub fn number_of_memory_operands(&self) -> u32 {
        unsafe { xed_decoded_inst_number_of_memory_operands(self.inner_ptr()) }
    }

    /// Interpret the operand in light of AVX512 masking and
    /// zeroing/merging.
    ///
    /// If masking and merging are used together, the dest
    /// operand may also be read. If masking and merging
    /// the elements of the dest operand register may be
    /// conditionally written (so that input values live on
    /// in the output register).
    pub fn operand_action(&self, operand_index: u32) -> OperandAction {
        unsafe { xed_decoded_inst_operand_action(self.inner_ptr(), operand_index).into() }
    }

    /// The size of an element in bits (for SSE and AVX operands)
    pub fn operand_element_size_bits(&self, operand_index: u32) -> u32 {
        unsafe { xed_decoded_inst_operand_element_size_bits(self.inner_ptr(), operand_index) }
    }

    /// The type of an element of type
    /// [`OperandElementType`](crate::OperandElementType)
    /// (for SSE and AVX operands)
    pub fn operand_element_type(&self, operand_index: u32) -> OperandElementType {
        unsafe { xed_decoded_inst_operand_element_type(self.inner_ptr(), operand_index).into() }
    }

    pub fn operands(&self) -> &[DecodedInst] {
        use std::slice;

        unsafe {
            let nops = self.noperands() as usize;
            let ptr = xed_decoded_inst_operands_const(self.inner_ptr()) as *const DecodedInst;

            slice::from_raw_parts(ptr, nops)
        }
    }

    pub fn operands_mut(&mut self) -> &[DecodedInst] {
        use std::slice;

        unsafe {
            let nops = self.noperands() as usize;
            let ptr = xed_decoded_inst_operands(&mut self.inner as *mut _) as *mut DecodedInst;

            slice::from_raw_parts(ptr, nops)
        }
    }

    /// Return true for AVX512 load-op instructions using
    /// the broadcast feature, false otherwise.
    pub fn uses_embedded_broadcast(&self) -> bool {
        unsafe { xed_decoded_inst_uses_embedded_broadcast(self.inner_ptr()) != 0 }
    }

    /// Indicates whether the flags are read or written.
    ///
    /// This will return false if the flags are really not
    /// used by this instruction. For some shifts/rotates,
    /// XED puts a flags operand in the operand array before
    /// it knows if the flags are used because of
    /// mode-dependant masking effects on the immediate.
    pub fn uses_rflags(&self) -> bool {
        unsafe { xed_decoded_inst_uses_rflags(self.inner_ptr()) != 0 }
    }

    pub fn valid(&self) -> bool {
        unsafe { xed_decoded_inst_valid(self.inner_ptr()) != 0 }
    }

    /// Indicate whether this decoded instruction is valid
    /// for the specified [chip](crate::Chip).
    pub fn valid_for_chip(&self, chip: Chip) -> bool {
        unsafe { xed_decoded_inst_valid_for_chip(self.inner_ptr(), chip.into()) != 0 }
    }

    /// Returns 128, 256 or 512 for operations in the VEC, EVEX
    /// (or XOP) encoding space and returns 0 for (most)
    /// nonvector operations.
    ///
    /// This is usually the content of the VEC.L or EVEX.LL
    /// field, reinterpreted. Some GPR instructions (like
    /// the BMI1/BMI2) are encoded in the VEX space and
    /// return non-zero values from this API.
    pub fn vector_length_bits(&self) -> u32 {
        unsafe { xed_decoded_inst_vector_length_bits(self.inner_ptr()) }
    }
}

// xed_decoded_inst_set_*
impl DecodedInst {
    /// Set the branch displacement using a byte length
    pub fn set_branch_displacement(&mut self, disp: i32, length_bytes: u32) {
        unsafe {
            xed_decoded_inst_set_branch_displacement(self.inner_ptr_mut(), disp, length_bytes)
        }
    }

    /// Set the branch displacement using a bits length
    pub fn set_branch_displacement_bits(&mut self, disp: i32, length_bits: u32) {
        unsafe {
            xed_decoded_inst_set_branch_displacement_bits(self.inner_ptr_mut(), disp, length_bits)
        }
    }

    /// Set the signed immediate using a byte length
    pub fn set_immediate_signed(&mut self, x: i32, length_bytes: u32) {
        unsafe { xed_decoded_inst_set_immediate_signed(self.inner_ptr_mut(), x, length_bytes) }
    }

    /// Set the signed immediate using a bits length
    pub fn set_immediate_signed_bits(&mut self, x: i32, length_bits: u32) {
        unsafe { xed_decoded_inst_set_immediate_signed_bits(self.inner_ptr_mut(), x, length_bits) }
    }

    /// Set the unsigned immediate using a byte length
    pub fn set_immediate_unsigned(&mut self, x: u64, length_bytes: u32) {
        unsafe { xed_decoded_inst_set_immediate_unsigned(self.inner_ptr_mut(), x, length_bytes) }
    }

    /// Set the unsigned immediate using a bits length
    pub fn set_immediate_unsigned_bits(&mut self, x: u64, length_bits: u32) {
        unsafe {
            xed_decoded_inst_set_immediate_unsigned_bits(self.inner_ptr_mut(), x, length_bits)
        }
    }

    pub fn set_input_chip(&mut self, chip: Chip) {
        unsafe { xed_decoded_inst_set_input_chip(self.inner_ptr_mut(), chip.into()) }
    }

    /// Set the memory displacement using a byte length
    pub fn set_memory_displacement(&mut self, disp: i64, length_bytes: u32) {
        unsafe {
            xed_decoded_inst_set_memory_displacement(self.inner_ptr_mut(), disp, length_bytes)
        }
    }

    /// Set the memory displacement using a bits length
    pub fn set_memory_displacement_bits(&mut self, disp: i64, length_bits: u32) {
        unsafe {
            xed_decoded_inst_set_memory_displacement_bits(self.inner_ptr_mut(), disp, length_bits)
        }
    }

    pub fn set_mode(&mut self, mmode: MachineMode, stack_addr_width: AddressWidth) {
        unsafe {
            xed_decoded_inst_set_mode(self.inner_ptr_mut(), mmode.into(), stack_addr_width.into())
        }
    }

    pub fn set_scale(&mut self, scale: u32) {
        unsafe { xed_decoded_inst_set_scale(self.inner_ptr_mut(), scale) }
    }

    pub fn set_user_data(&mut self, new_value: u64) {
        unsafe { xed_decoded_inst_set_user_data(self.inner_ptr_mut(), new_value) }
    }
}

// xed_operand_values_*

impl From<xed_decoded_inst_t> for DecodedInst {
    fn from(inner: xed_decoded_inst_t) -> Self {
        Self { inner }
    }
}
