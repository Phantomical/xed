use xed_sys2::xed_interface::*;

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct EncoderOperand {
    inner: xed_encoder_operand_t,
}

impl EncoderOperand {
    pub fn into_inner(self) -> xed_encoder_operand_t {
        self.inner
    }

    pub fn inner(&self) -> &xed_encoder_operand_t {
        &self.inner
    }
    pub fn inner_mut(&mut self) -> &mut xed_encoder_operand_t {
        &mut self.inner
    }
}

impl From<xed_encoder_operand_t> for EncoderOperand {
    fn from(inner: xed_encoder_operand_t) -> Self {
        Self { inner }
    }
}

impl From<EncoderOperand> for xed_encoder_operand_t {
    fn from(op: EncoderOperand) -> Self {
        op.into_inner()
    }
}
