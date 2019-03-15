
use xed_sys2::xed_interface::*;
use crate::*;

pub trait OperandValues : InnerPtr<xed_operand_values_t> {
    fn accesses_memory(&self) -> bool {
        unsafe {
            xed_operand_values_accesses_memory(self.inner_ptr()) != 0
        }
    }

    fn branch_not_taken_hint(&self) -> bool {
        unsafe {
            xed_operand_values_branch_not_taken_hint(
                self.inner_ptr()
            ) != 0
        }
    }

    fn branch_taken_hint(&self) -> bool {
        unsafe {
            xed_operand_values_branch_taken_hint(self.inner_ptr()) != 0
        }
    }

    /// Indicates whether the memory opetions has atomic
    /// read-modify-write semantics. An XCHG accessing memory
    /// is atomic with or without a LOCK prefix.
    fn atomic(&self) -> bool {
        unsafe {
            xed_operand_values_get_atomic(self.inner_ptr()) != 0
        }
    }

    fn base_reg(&self, memop_idx: u32) -> Reg {
        unsafe {
            xed_operand_values_get_base_reg(
                self.inner_ptr(),
                memop_idx
            ).into()
        }
    }

    fn branch_displacement_byte(&self, i: u32) -> u8 {
        unsafe {
            xed_operand_values_get_branch_displacement_byte(
                self.inner_ptr(),
                i
            )
        }
    }

    fn branch_displacement_int32(&self) -> i32 {
        unsafe {
            xed_operand_values_get_branch_displacement_int32(
                self.inner_ptr(),
            )
        }
    }

    /// The branch displacement width in bytes
    fn branch_displacement_length(&self) -> u32 {
        unsafe {
            xed_operand_values_get_branch_displacement_length(
                self.inner_ptr()
            )
        }
    }

    /// The branch displacement width in bits
    fn branch_displacement_length_bits(&self) -> u32 {
        unsafe {
            xed_operand_values_get_branch_displacement_length_bits(
                self.inner_ptr()
            )
        }
    }

    /// The effective address width in bits: 16/32/64
    fn effective_address_width(&self) -> u32 {
        unsafe {
            xed_operand_values_get_effective_address_width(
                self.inner_ptr()
            )
        }
    }

    /// The effective operand width in bits: 16/32/64
    ///
    /// Note this is not the same as the width of the operand
    /// which can vary! For 8 bit operations, the effective 
    /// operand width is the maching mode's default width.
    /// If you want to identify byte operations uses the
    /// higher level function [`operand_width()`].
    fn effective_operand_width(&self) -> u32 {
        unsafe {
            xed_operand_values_get_effective_operand_width(
                self.inner_ptr()
            )
        }
    }

    fn iclass(&self) -> IClass {
        unsafe {
            xed_operand_values_get_iclass(self.inner_ptr()).into()
        }
    }

    /// The i'th byte of the immediate.
    fn immediate_byte(&self, i: u32) -> u8 {
        unsafe {
            xed_operand_values_get_immediate_byte(
                self.inner_ptr(),
                i
            )
        }
    }

    fn immediate_int64(&self) -> i64 {
        unsafe {
            xed_operand_values_get_immediate_int64(self.inner_ptr())
        }
    }

    /// Indicate whether the first immediate (IMM0) is signed.
    fn immediate_is_signed(&self) -> bool {
        unsafe {
            xed_operand_values_get_immediate_is_signed(self.inner_ptr()) != 0
        }
    }

    fn immediate_uint64(&self) -> u64 {
        unsafe {
            xed_operand_values_get_immediate_uint64(
                self.inner_ptr()
            )
        }
    }

    fn index_reg(&self, memop_idx: u32) -> Reg {
        unsafe {
            xed_operand_values_get_index_reg(
                self.inner_ptr(),
                memop_idx
            ).into()
        }
    }

    fn long_mode(&self) -> bool {
        unsafe {
            xed_operand_values_get_long_mode(self.inner_ptr()) != 0
        }
    }

    fn memory_displacement_byte(&self, i: u32) -> u8 {
        unsafe {
            xed_operand_values_get_memory_displacement_byte(
                self.inner_ptr(),
                i
            )
        }
    }

    /// The potentially scaled value of the memory displacement.
    /// 
    /// Certain AVX512 memory displacements are scaled before 
    /// they are used.
    fn memory_displacement_int64(&self) -> i64 {
        unsafe {
            xed_operand_values_get_memory_displacement_int64(
                self.inner_ptr()
            )
        }
    }

    /// The unscaled (raw) memory displacement.
    /// 
    /// Certain AVX512 memory displacements are scaled before
    /// they are used.
    fn memory_displacement_int64_raw(&self) -> i64 {
        unsafe {
            xed_operand_values_get_memory_displacement_int64_raw(
                self.inner_ptr()
            )
        }
    }

    /// The memory displacement width in bytes.
    fn memory_displacement_length(&self) -> u32 {
        unsafe {
            xed_operand_values_get_memory_displacement_length(
                self.inner_ptr()
            )
        }
    }

    /// The memory displacement width in bits.
    fn memory_displacement_length_bits(&self) -> u32 {
        unsafe {
            xed_operand_values_get_memory_displacement_length_bits(
                self.inner_ptr()
            )
        }
    }

    /// The memory displacement width in bits.
    fn memory_displacement_length_bits_raw(&self) -> u32 {
        unsafe {
            xed_operand_values_get_memory_displacement_length_bits_raw(
                self.inner_ptr()
            )
        }
    }

    fn memory_operand_length(&self, memop_idx: u32) -> u32 {
        unsafe {
            xed_operand_values_get_memory_operand_length(
                self.inner_ptr(),
                memop_idx
            )
        }
    }

    fn real_mode(&self) -> bool {
        unsafe {
            xed_operand_values_get_real_mode(
                self.inner_ptr(),
            ) != 0
        }
    }

    fn scale(&self) -> u32 {
        unsafe {
            xed_operand_values_get_scale(self.inner_ptr())
        }
    }

    fn second_immediate(&self) -> u8 {
        unsafe {
            xed_operand_values_get_second_immediate(
                self.inner_ptr()
            )
        }
    }

    fn seg_reg(&self, memop_idx: u32) -> Reg {
        unsafe {
            xed_operand_values_get_seg_reg(
                self.inner_ptr(),
                memop_idx
            ).into()
        }
    }

    /// The stack address width in bits: 16/32/64
    fn stack_address_width(&self) -> u32 {
        unsafe {
            xed_operand_values_get_stack_address_width(
                self.inner_ptr()
            )
        }
    }

    /// This includes any 66 prefix that shows up even
    /// if it is ignored.
    fn has_66_prefix(&self) -> bool {
        unsafe {
            xed_operand_values_has_66_prefix(self.inner_ptr()) != 0
        }
    }

    /// This indicates the presence of a 67 prefix.
    fn has_address_size_prefix(&self) -> bool {
        unsafe {
            xed_operand_values_has_address_size_prefix(
                self.inner_ptr()
            ) != 0
        }
    }

    /// Indicates whether there is a branch displacement.
    fn has_branch_displacement(&self) -> bool {
        unsafe {
            xed_operand_values_has_branch_displacement(
                self.inner_ptr()
            ) != 0
        }
    }

    /// Indicates whether there is a memory or branch displacement.
    fn has_displacement(&self) -> bool {
        unsafe {
            xed_operand_values_has_displacement(
                self.inner_ptr()
            ) != 0
        }
    }

    /// Indicate whether there is an immediate operand.
    fn has_immediate(&self) -> bool {
        unsafe {
            xed_operand_values_has_immediate(
                self.inner_ptr()
            ) != 0
        }
    }

    /// Indicate whether the memory operation has a valid
    /// lock prefix.
    fn has_lock_prefix(&self) -> bool {
        unsafe {
            xed_operand_values_has_lock_prefix(
                self.inner_ptr()
            ) != 0
        }
    }

    /// Indicate whether there is a memory displacement
    fn has_memory_displacement(&self) -> bool {
        unsafe {
            xed_operand_values_has_memory_displacement(
                self.inner_ptr()
            ) != 0
        }
    }

    /// Indicate whether the instruction has a modrm byte.
    fn has_modrm_byte(&self) -> bool {
        unsafe {
            xed_operand_values_has_modrm_byte(
                self.inner_ptr()
            ) != 0
        }
    }

    /// This does not include the cases when the 66 prefix
    /// is used as an opcode-refining prefix for multibyte
    /// opcodes.
    fn has_operand_size_prefix(&self) -> bool {
        unsafe {
            xed_operand_values_has_operand_size_prefix(
                self.inner_ptr()
            ) != 0
        }
    }

    /// True if the instruction has a real rep prefix. This
    /// returns false if there is no F2/F3 prefix or the
    /// F2/F3 prefix is used to refine the opcode as in some
    /// SSE operations. 
    fn has_real_rep(&self) -> bool {
        unsafe {
            xed_operand_values_has_real_rep(
                self.inner_ptr()
            ) != 0
        }
    }

    /// Indicates whether the instruction has a F3 rep prefix
    /// 
    /// (used for opcode refinint, for rep for string operations,
    /// or ignored). 
    fn has_rep_prefix(&self) -> bool {
        unsafe {
            xed_operand_values_has_rep_prefix(
                self.inner_ptr()
            ) != 0
        }
    }

    /// Indicates whether the instruction has a F2 rep prefix
    /// 
    /// (used for opcode refining, for rep for string operations,
    /// or ignored).
    fn has_repne_prefix(&self) -> bool {
        unsafe {
            xed_operand_values_has_repne_prefix(
                self.inner_ptr()
            ) != 0
        }
    }

    /// Indicate whether this instruction has a rex prefix with
    /// the W bit set. 
    fn has_rexw_prefix(&self) -> bool {
        unsafe {
            xed_operand_values_has_rexw_prefix(
                self.inner_ptr()
            ) != 0
        }
    }

    fn has_segment_prefix(&self) -> bool {
        unsafe {
            xed_operand_values_has_segment_prefix(
                self.inner_ptr()
            ) != 0
        }
    }

    /// Indicate whether the instruction has a SIB byte
    fn has_sib_byte(&self) -> bool {
        unsafe {
            xed_operand_values_has_sib_byte(
                self.inner_ptr()
            ) != 0
        }
    }

    fn is_nop(&self) -> bool {
        unsafe {
            xed_operand_values_is_nop(self.inner_ptr()) != 0
        }
    }

    fn is_prefetch(&self) -> bool {
        unsafe {
            xed_operand_values_is_prefetch(self.inner_ptr()) != 0
        }
    }

    /// Indicate whether the instruction could be 
    /// re-encoded to have a lock prefix but does not
    /// have one currently.
    fn lockable(&self) -> bool {
        unsafe {
            xed_operand_values_lockable(self.inner_ptr()) != 0
        }
    }

    /// Indicates whether this instruction accesses memory
    /// but without a using a modrm byte limiting its
    /// addressing modes.
    fn memop_withou_modrm(&self) -> bool {
        unsafe {
            xed_operand_values_memop_without_modrm(self.inner_ptr()) != 0
        }
    }

    fn number_of_memory_operands(&self) -> u32 {
        unsafe {
            xed_operand_values_number_of_memory_operands(
                self.inner_ptr()
            )
        }
    }

    fn segment_prefix(&self) -> Reg {
        unsafe {
            xed_operand_values_segment_prefix(
                self.inner_ptr()
            ).into()
        }
    }

    fn set_base_reg(&mut self, memop_idx: u32, new_base: Reg) {
        unsafe {
            xed_operand_values_set_base_reg(
                self.inner_ptr_mut(),
                memop_idx,
                new_base.into()
            );
        }
    }

    /// Set the branch displacement using a bytes length
    fn set_branch_displacement(&mut self, x: i32, len: u32) {
        unsafe {
            xed_operand_values_set_branch_displacement(
                self.inner_ptr_mut(),
                x,
                len
            );
        }
    }

    /// Set the branch displacement using a bits length
    fn set_branch_displacement_bits(&mut self, x: i32, len_bits: u32) {
        unsafe {
            xed_operand_values_set_branch_displacement_bits(
                self.inner_ptr_mut(),
                x,
                len_bits
            )
        }
    }

    /// Width is bits 16, 32, 64
    fn set_effective_address_width(&mut self, width: u32) {
        unsafe {
            xed_operand_values_set_effective_address_width(
                self.inner_ptr_mut(),
                width
            );
        }
    }

    /// Width is bits 8, 16, 32, 64
    fn set_effective_operand_width(&mut self, width: u32) {
        unsafe {
            xed_operand_values_set_effective_operand_width(
                self.inner_ptr_mut(),
                width
            )
        }
    }

    fn set_iclass(&mut self, iclass: IClass) {
        unsafe {
            xed_operand_values_set_iclass(
                self.inner_ptr_mut(),
                iclass.into()
            )
        }
    }

    /// Set the signed immediate using a bytes length
    fn set_immediate_signed(&mut self, x: i32, bytes: u32) {
        unsafe {
            xed_operand_values_set_immediate_signed(
                self.inner_ptr_mut(),
                x, bytes
            )
        }
    }

    /// Set the signed immediate using a bits length
    fn set_immediate_signed_bits(&mut self, x: i32, bits: u32) {
        unsafe {
            xed_operand_values_set_immediate_signed_bits(
                self.inner_ptr_mut(),
                x,
                bits
            )
        }
    }

    /// Set the unsigned immediate using a bytes length
    fn set_immediate_unsigned(&mut self, x: u64, bytes: u32) {
        unsafe {
            xed_operand_values_set_immediate_unsigned(
                self.inner_ptr_mut(),
                x, bytes
            )
        }
    }

    /// Set the unisigned immediate using a bits length
    fn set_immediate_unsigned_bits(&mut self, x: u64, bits: u32) {
        unsafe {
            xed_operand_values_set_immediate_unsigned_bits(
                self.inner_ptr_mut(),
                x, bits
            )
        }
    }

    fn set_index_reg(&mut self, memop_idx: u32, new_index: Reg) {
        unsafe {
            xed_operand_values_set_index_reg(
                self.inner_ptr_mut(),
                memop_idx,
                new_index.into()
            )
        }
    }

    fn set_lock(&mut self) {
        unsafe {
            xed_operand_values_set_lock(
                self.inner_ptr_mut()
            )
        }
    }

    /// Set the memory displacement using a bytes length
    fn set_memory_displacement(&mut self, x: i64, len: u32) {
        unsafe {
            xed_operand_values_set_memory_displacement(
                self.inner_ptr_mut(),
                x,
                len
            )
        }
    }

    /// Set the memory displacement using a bits length
    fn set_memory_displacement_bits(&mut self, x: i64, len: u32) {
        unsafe {
            xed_operand_values_set_memory_displacement_bits(
                self.inner_ptr_mut(),
                x,
                len
            )
        }
    }

    /// Takes bytes, not bits, as an argument
    fn set_memory_operand_length(&mut self, memop_length: u32) {
        unsafe {
            xed_operand_values_set_memory_operand_length(
                self.inner_ptr_mut(),
                memop_length
            )
        }
    }

    /// Set the mode value
    fn set_mode(&mut self, dstate: &State) {
        unsafe {
            xed_operand_values_set_mode(
                self.inner_ptr_mut(),
                dstate.inner_ptr()
            )
        }
    }

    /// Set the operand storage field entry named "operand_name"
    /// to the register value specified by "reg_name".
    fn set_operand_reg(&mut self, operand_name: OperandEnum, reg_name: Reg) {
        unsafe {
            xed_operand_values_set_operand_reg(
                self.inner_ptr_mut(),
                operand_name.into(),
                reg_name.into()
            )
        }
    }

    /// Indicate that we have a relative branch
    fn set_relbr(&mut self) {
        unsafe {
            xed_operand_values_set_relbr(
                self.inner_ptr_mut()
            )
        }
    }

    fn set_scale(&mut self, memop_idx: u32, new_scale: u32) {
        unsafe {
            xed_operand_values_set_scale(
                self.inner_ptr_mut(),
                memop_idx,
                new_scale
            )
        }
    }

    fn set_seg_reg(&mut self, memop_idx: u32, new_seg: Reg) {
        unsafe {
            xed_operand_values_set_seg_reg(
                self.inner_ptr_mut(),
                memop_idx,
                new_seg.into()
            )
        }
    }

    /// Indicates whether the default segment is being used.
    /// 
    /// `i` is 0 or 1, indicating which memory operation to
    /// query. 
    /// 
    /// Returns true if the memory operation is using the
    /// default segment for the associated addressing mode
    /// base register.
    fn using_default_segment(&self, i: u32) -> bool {
        unsafe {
            xed_operand_values_using_default_segment(
                self.inner_ptr(),
                i
            ) != 0
        }
    }    
}
