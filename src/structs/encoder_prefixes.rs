
use xed_sys2::xed_interface::*;

pub struct EncoderPrefixes {
    inner: xed_encoder_prefixes_t
}

impl EncoderPrefixes {
    pub fn into_inner(self) -> xed_encoder_prefixes_t {
        self.inner
    }

    pub fn inner(&self) -> &xed_encoder_prefixes_t {
        &self.inner
    }
    pub fn inner_mut(&mut self) -> &mut xed_encoder_prefixes_t {
        &mut self.inner
    }

    pub fn br_hint_not_taken(&self) -> bool {
        unsafe {
            self.inner.s.br_hint_not_taken() != 0
        }
    }
    pub fn br_hint_taken(&self) -> bool {
        unsafe {
            self.inner.s.br_hint_taken() != 0
        }
    }
    pub fn rep(&self) -> bool {
        unsafe {
            self.inner.s.rep() != 0
        }
    }
    pub fn repne(&self) -> bool {
        unsafe {
            self.inner.s.repne() != 0
        }
    }
    pub fn bits(&self) -> u32 {
        unsafe { self.inner.i }
    }

    pub fn set_br_hint_not_taken(&mut self, v: bool) {
        unsafe {
            self.inner.s.set_br_hint_not_taken(if v { 1 } else { 0 })
        }
    }
    pub fn set_br_hint_taken(&mut self, v: bool) {
        unsafe {
            self.inner.s.set_br_hint_taken(if v { 1 } else { 0 });
        }
    }
    pub fn set_rep(&mut self, v: bool) {
        unsafe {
            self.inner.s.set_rep(if v { 1 } else { 0 });
        }
    }
    pub fn set_repne(&mut self, v: bool) {
        unsafe {
            self.inner.s.set_repne(if v { 1 } else { 0 });
        }
    }
}

impl From<xed_encoder_prefixes_t> for EncoderPrefixes {
    fn from(inner: xed_encoder_prefixes_t) -> Self {
        Self { inner }
    }
}

impl From<EncoderPrefixes> for xed_encoder_prefixes_t {
    fn from(x: EncoderPrefixes) -> Self {
        x.into_inner()
    }
}
