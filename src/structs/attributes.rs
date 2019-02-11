use xed_sys2::xed_interface::*;

#[derive(Copy, Clone, Debug)]
pub struct Attributes {
    inner: xed_attributes_t,
}

impl From<xed_attributes_t> for Attributes {
    fn from(inner: xed_attributes_t) -> Self {
        Self { inner }
    }
}
