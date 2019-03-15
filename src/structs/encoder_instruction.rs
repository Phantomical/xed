use xed_sys2::xed_interface::*;
use crate::*;

pub struct EncoderInstruction {
    inner: xed_encoder_instruction_t,
}

impl EncoderInstruction {
    pub fn into_inner(self) -> xed_encoder_instruction_t {
        self.inner
    }

    pub fn inner(&self) -> &xed_encoder_instruction_t {
        &self.inner
    }
    pub fn inner_mut(&mut self) -> &mut xed_encoder_instruction_t {
        &mut self.inner
    }

    pub fn mode(&self) -> State {
        self.inner.mode.into()
    }
    pub fn iclass(&self) -> IClass {
        self.inner.iclass.into()
    }
    pub fn effective_operand_width(&self) -> u32 {
        self.inner.effective_operand_width
    }
    pub fn effective_address_width(&self) -> u32 {
        self.inner.effective_address_width
    }
    #[deprecated(note="TODO: Wrap xed_encoder_prefixes_t")]
    pub fn prefixes(&self) -> xed_encoder_prefixes_t {
        self.inner.prefixes
    }

    pub fn operands(&self) -> &[EncoderOperand] {
        unsafe {
            let ptr = (&self.inner.operands[..]).as_ptr();

            std::slice::from_raw_parts(
                ptr as *const EncoderOperand,
                self.inner.noperands as usize
            )
        }
    }
    pub fn operands_mut(&mut self) -> &mut [EncoderOperand] {
        unsafe {
            let ptr = (&mut self.inner.operands[..]).as_mut_ptr();

            std::slice::from_raw_parts_mut(
                ptr as *mut EncoderOperand,
                self.inner.noperands as usize
            )
        }
    }

    pub fn set_iclass(&mut self, iclass: IClass) {
        self.inner.iclass = iclass.into();
    }
    pub fn set_effective_operand_width(&mut self, effective_operand_width: u32) {
        self.inner.effective_operand_width = effective_operand_width;
    }
    pub fn set_effective_address_width(&mut self, effective_address_width: u32) {
        self.inner.effective_address_width = effective_address_width;
    }
    #[deprecated(note="TODO: Wrap xed_encoder_prefixes_t")]
    pub fn set_prefixes(&mut self, prefixes: xed_encoder_prefixes_t) {
        self.inner.prefixes = prefixes;
    }

    /// Specify an effective address size different than the default.
    /// 
    /// For things with base or index registers, XED picks it up
    /// from the registers. But for things that have implicit
    /// memops, or no base or index register, we must allow the 
    /// user to set the address width directly.
    /// 
    /// # Parameters
    /// - `width_bits`: The intended effective address size in bits.
    ///   Values: 16, 32, or 64.
    pub fn set_addr(&mut self, width_bits: u32) {
        unsafe {
            xed_addr(
                &mut self.inner as *mut _,
                width_bits
            );
        }
    }

    /// Add a REP (0xF3) prefix.
    pub fn set_rep(&mut self) {
        unsafe {
            xed_rep(self.inner_mut() as *mut _)
        }
    }

    /// Add a REPNE (0xF2) prefix.
    pub fn set_repne(&mut self) {
        unsafe {
            xed_repne(self.inner_mut() as &mut _)
        }
    }
}

impl From<xed_encoder_instruction_t> for EncoderInstruction {
    fn from(inner: xed_encoder_instruction_t) -> Self {
        Self { inner }
    }
}

impl From<EncoderInstruction> for EncoderRequest {
    fn from(mut inst: EncoderInstruction) -> Self {
        unsafe {
            let mut req: xed_encoder_request_t = std::mem::uninitialized();

            let res = xed_convert_to_encoder_request(
                &mut req as *mut _,
                inst.inner_mut() as *mut _,
            );

            assert!(res != 0);

            req.into()
        }
    }
}

