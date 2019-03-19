use crate::*;
use xed_sys2::xed_interface::*;

#[derive(Copy, Clone, Debug)]
pub struct MemOp {
    inner: xed_memop_t,
}

impl MemOp {
    pub fn inner(&self) -> &xed_memop_t {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut xed_memop_t {
        &mut self.inner
    }

    pub fn into_inner(self) -> xed_memop_t {
        self.inner
    }
}

impl MemOp {
    pub fn base(&self) -> Reg {
        self.inner.base.into()
    }

    pub fn disp(&self) -> EncDisplacement {
        self.inner.disp
    }

    pub fn index(&self) -> Reg {
        self.inner.index.into()
    }

    pub fn scale(&self) -> u32 {
        self.inner.scale
    }

    pub fn seg(&self) -> Reg {
        self.inner.seg.into()
    }
}

impl From<xed_memop_t> for MemOp {
    fn from(x: xed_memop_t) -> Self {
        Self { inner: x }
    }
}
