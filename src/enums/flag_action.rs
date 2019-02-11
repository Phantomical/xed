use num_traits::*;
use xed_sys2::xed_interface::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Primitive)]
pub enum FlagActionEnum {
    Action0 = XED_FLAG_ACTION_0 as isize,
    Action1 = XED_FLAG_ACTION_1 as isize,
    Invalid = XED_FLAG_ACTION_INVALID as isize,
    Ah = XED_FLAG_ACTION_ah as isize,
    Mod = XED_FLAG_ACTION_mod as isize,
    Pop = XED_FLAG_ACTION_pop as isize,
    Tst = XED_FLAG_ACTION_tst as isize,
    U = XED_FLAG_ACTION_u as isize,
}

impl From<xed_flag_action_enum_t> for FlagActionEnum {
    fn from(x: xed_flag_action_enum_t) -> Self {
        Self::from_u32(x).unwrap_or(FlagActionEnum::Invalid)
    }
}

impl From<FlagActionEnum> for xed_flag_action_enum_t {
    fn from(x: FlagActionEnum) -> Self {
        x as Self
    }
}
