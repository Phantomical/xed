use crate::*;
use xed_sys2::xed_interface::*;

#[derive(Copy, Clone, Debug)]
pub struct FlagAction {
    inner: xed_flag_enum_s,
}

impl FlagAction {
    pub fn flag_action(&self) -> FlagActionEnum {
        self.inner.action.into()
    }

    pub fn flag(&self) -> Flag {
        self.inner.flag.into()
    }

    pub fn into_inner(&self) -> xed_flag_action_t {
        self.inner
    }
}

impl From<xed_flag_action_t> for FlagAction {
    fn from(inner: xed_flag_action_t) -> Self {
        Self { inner }
    }
}
