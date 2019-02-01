#![allow(non_upper_case_globals)]

use num_traits::FromPrimitive;
use xed_sys2::xed_interface::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Primitive)]
pub enum AddressWidth {
    Width16b = XED_ADDRESS_WIDTH_16b as isize,
    Width32b = XED_ADDRESS_WIDTH_32b as isize,
    Width64b = XED_ADDRESS_WIDTH_64b as isize,
    Invalid = XED_ADDRESS_WIDTH_INVALID as isize,
}

impl From<xed_address_width_enum_t> for AddressWidth {
    fn from(x: xed_address_width_enum_t) -> Self {
        use self::AddressWidth::*;

        Self::from_u32(x).unwrap_or(Invalid)
    }
}

impl From<AddressWidth> for xed_address_width_enum_t {
    fn from(x: AddressWidth) -> Self {
        x as xed_address_width_enum_t
    }
}
