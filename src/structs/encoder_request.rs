use crate::*;
use xed_sys2::xed_interface::*;

use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct EncoderRequest {
    inner: xed_encoder_request_t,
}

impl EncoderRequest {
    pub fn inner(&self) -> &xed_encoder_request_t {
        &self.inner
    }
    pub fn inner_mut(&mut self) -> &mut xed_encoder_request_t {
        &mut self.inner
    }

    pub fn into_inner(self) -> xed_encoder_request_t {
        self.inner
    }

    pub fn zeroed() -> Self {
        Self {
            inner: unsafe { std::mem::zeroed() },
        }
    }

    pub fn with_mode(state: &State) -> Self {
        unsafe {
            let mut inner = std::mem::uninitialized();

            xed_encoder_request_zero_set_mode(&mut inner as *mut _, state.inner_ptr());

            Self { inner }
        }
    }
}

// Functions
impl EncoderRequest {
    pub fn operands(&self) -> &[impl OperandValues] {
        unsafe {
            let nops = xed_decoded_inst_noperands(self.inner_ptr());
            let ops = xed_encoder_request_operands_const(self.inner_ptr()) as *const DecodedInst;

            std::slice::from_raw_parts(ops, nops as usize)
        }
    }

    pub fn operands_mut(&mut self) -> &mut [impl OperandValues] {
        unsafe {
            let nops = xed_decoded_inst_noperands(self.inner_ptr());
            let ops = xed_encoder_request_operands(self.inner_ptr_mut()) as *mut DecodedInst;

            std::slice::from_raw_parts_mut(ops, nops as usize)
        }
    }

    pub fn iclass(&self) -> IClass {
        unsafe { xed_encoder_request_get_iclass(self.inner_ptr()).into() }
    }

    pub fn set_iclass(&mut self, iclass: IClass) {
        unsafe {
            xed_encoder_request_set_iclass(self.inner_ptr_mut(), iclass.into());
        }
    }
}

// Prefixes
impl EncoderRequest {
    /// For REPNE(F2) prefix on string ops.
    pub fn set_repne(&mut self) {
        unsafe {
            xed_encoder_request_set_repne(self.inner_ptr_mut());
        }
    }

    /// For REP(F3) prefix on string ops.
    pub fn set_rep(&mut self) {
        unsafe {
            xed_encoder_request_set_rep(self.inner_ptr_mut());
        }
    }

    /// Clear the REP prefix indicator
    pub fn clear_rep(&mut self) {
        unsafe {
            xed_encoder_request_clear_rep(self.inner_ptr_mut());
        }
    }
}

// Primary Encode Functions
impl EncoderRequest {
    pub fn set_effective_operand_width(&mut self, width_bits: u32) {
        unsafe {
            xed_encoder_request_set_effective_operand_width(self.inner_ptr_mut(), width_bits);
        }
    }

    pub fn set_effective_address_size(&mut self, width_bits: u32) {
        unsafe {
            xed_encoder_request_set_effective_address_size(self.inner_ptr_mut(), width_bits);
        }
    }

    pub fn set_reg(&mut self, operand: OperandEnum, reg: Reg) {
        unsafe {
            xed_encoder_request_set_reg(self.inner_ptr_mut(), operand.into(), reg.into());
        }
    }
}

// Operand Order
impl EncoderRequest {
    /// Sets the n'th operand in operand order.
    pub fn set_operand_order(&mut self, operand_index: u32, name: OperandEnum) {
        unsafe {
            xed_encoder_request_set_operand_order(self.inner_ptr_mut(), operand_index, name.into());
        }
    }

    /// Retrieve the n'th operand in the operand order.
    pub fn operand_order(&self, operand_index: u32) -> OperandEnum {
        unsafe {
            xed_encoder_request_get_operand_order(
                // The C code needs a *mut pointer, but
                self.inner_ptr() as *mut _,
                operand_index,
            )
            .into()
        }
    }

    /// Returns the number of entries in the operand order array.
    pub fn operand_order_entries(&self) -> u32 {
        unsafe {
            xed_encoder_request_operand_order_entries(
                // XED takes a *mut pointer here but it
                // should really be *const
                self.inner_ptr() as *mut _,
            )
        }
    }
}

// Branches and Far Pointers
impl EncoderRequest {
    pub fn set_relbr(&mut self) {
        unsafe { xed_encoder_request_set_relbr(self.inner_ptr_mut()) }
    }

    pub fn set_branch_displacement(&mut self, brdisp: i32, nbytes: u32) {
        unsafe {
            xed_encoder_request_set_branch_displacement(self.inner_ptr_mut(), brdisp, nbytes);
        }
    }

    pub fn set_ptr(&mut self) {
        unsafe { xed_encoder_request_set_ptr(self.inner_ptr_mut()) }
    }
}

// Immediates
impl EncoderRequest {
    /// Set the uimm0 using a byte width.
    pub fn set_uimm0(&mut self, uimm: u64, nbytes: u32) {
        unsafe { xed_encoder_request_set_uimm0(self.inner_ptr_mut(), uimm, nbytes) }
    }

    /// Set the uimm0 using a bit width.
    pub fn set_uimm0_bits(&mut self, uimm: u64, nbits: u32) {
        unsafe {
            xed_encoder_request_set_uimm0_bits(self.inner_ptr_mut(), uimm, nbits);
        }
    }

    pub fn set_uimm1(&mut self, uimm: u8) {
        unsafe { xed_encoder_request_set_uimm1(self.inner_ptr_mut(), uimm) }
    }

    /// Same storage as uimm0
    pub fn set_simm(&mut self, simm: i32, nbytes: u32) {
        unsafe { xed_encoder_request_set_simm(self.inner_ptr_mut(), simm, nbytes) }
    }
}

// Memory
impl EncoderRequest {
    pub fn set_memory_displacement(&mut self, memdisp: i64, nbytes: u32) {
        unsafe {
            xed_encoder_request_set_memory_displacement(self.inner_ptr_mut(), memdisp, nbytes);
        }
    }

    pub fn set_agen(&mut self) {
        unsafe { xed_encoder_request_set_agen(self.inner_ptr_mut()) }
    }

    pub fn set_mem0(&mut self) {
        unsafe { xed_encoder_request_set_mem0(self.inner_ptr_mut()) }
    }

    pub fn set_mem1(&mut self) {
        unsafe { xed_encoder_request_set_mem1(self.inner_ptr_mut()) }
    }

    pub fn set_memory_operand_length(&mut self, nbytes: u32) {
        unsafe { xed_encoder_request_set_memory_operand_length(self.inner_ptr_mut(), nbytes) }
    }

    pub fn set_seg0(&mut self, seg_reg: Reg) {
        unsafe { xed_encoder_request_set_seg0(self.inner_ptr_mut(), seg_reg.into()) }
    }

    pub fn set_seg1(&mut self, seg_reg: Reg) {
        unsafe { xed_encoder_request_set_seg1(self.inner_ptr_mut(), seg_reg.into()) }
    }

    pub fn set_base0(&mut self, base_reg: Reg) {
        unsafe { xed_encoder_request_set_base0(self.inner_ptr_mut(), base_reg.into()) }
    }

    pub fn set_base1(&mut self, base_reg: Reg) {
        unsafe { xed_encoder_request_set_base1(self.inner_ptr_mut(), base_reg.into()) }
    }

    pub fn set_index(&mut self, index_reg: Reg) {
        unsafe { xed_encoder_request_set_index(self.inner_ptr_mut(), index_reg.into()) }
    }

    pub fn set_scale(&mut self, scale: u32) {
        unsafe { xed_encoder_request_set_scale(self.inner_ptr_mut(), scale) }
    }
}

impl fmt::Display for EncoderRequest {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use std::ffi::CStr;
        use std::fmt::Debug;
        use std::os::raw::c_char;

        let mut buffer = [0; 2049];

        unsafe {
            let buffer: &mut [c_char] = &mut buffer;

            xed_encode_request_print(
                self.inner_ptr(),
                buffer.as_mut_ptr(),
                (buffer.len() - 1) as u32,
            );

            let s = CStr::from_ptr(buffer.as_ptr());

            s.fmt(fmt)
        }
    }
}

impl InnerPtr<xed_encoder_request_t> for EncoderRequest {
    fn inner_ptr(&self) -> *const xed_encoder_request_t {
        self.inner() as *const _
    }

    fn inner_ptr_mut(&mut self) -> *mut xed_encoder_request_t {
        self.inner_mut() as *mut _
    }
}

impl From<xed_encoder_request_t> for EncoderRequest {
    fn from(x: xed_encoder_request_t) -> Self {
        Self { inner: x }
    }
}

impl From<DecodedInst> for EncoderRequest {
    fn from(inst: DecodedInst) -> Self {
        let mut inst = inst.into_inner();

        unsafe {
            xed_encoder_request_init_from_decode(&mut inst as *mut _);
        }

        inst.into()
    }
}
