use num_traits::FromPrimitive;
use xed_sys2::xed_interface::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Primitive)]
pub enum Flag {
    Invalid = XED_FLAG_INVALID as isize,
    Ac = XED_FLAG_ac as isize,
    Af = XED_FLAG_af as isize,
    Cf = XED_FLAG_cf as isize,
    Df = XED_FLAG_df as isize,
    Fc0 = XED_FLAG_fc0 as isize,
    Fc1 = XED_FLAG_fc1 as isize,
    Fc2 = XED_FLAG_fc2 as isize,
    Fc3 = XED_FLAG_fc3 as isize,
    Id = XED_FLAG_id as isize,
    If = XED_FLAG_if as isize,
    Iopl = XED_FLAG_iopl as isize,
    Nt = XED_FLAG_nt as isize,
    Of = XED_FLAG_of as isize,
    Pf = XED_FLAG_pf as isize,
    Rf = XED_FLAG_rf as isize,
    Sf = XED_FLAG_sf as isize,
    Tf = XED_FLAG_tf as isize,
    Vif = XED_FLAG_vif as isize,
    Vip = XED_FLAG_vip as isize,
    Vm = XED_FLAG_vm as isize,
    Zf = XED_FLAG_zf as isize,
}

impl From<xed_flag_enum_t> for Flag {
    fn from(x: xed_flag_enum_t) -> Flag {
        Self::from_u32(x).unwrap_or(Flag::Invalid)
    }
}

impl From<Flag> for xed_flag_enum_t {
    fn from(x: Flag) -> Self {
        x as Self
    }
}
