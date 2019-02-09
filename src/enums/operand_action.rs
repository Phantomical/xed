use num_traits::FromPrimitive;
use xed_sys2::xed_interface::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Primitive)]
pub enum OperandAction {
    Invalid = XED_OPERAND_ACTION_INVALID as isize,
    Rw = XED_OPERAND_ACTION_RW as isize,
    R = XED_OPERAND_ACTION_R as isize,
    W = XED_OPERAND_ACTION_W as isize,
    Rcw = XED_OPERAND_ACTION_RCW as isize,
    Cw = XED_OPERAND_ACTION_CW as isize,
    Crw = XED_OPERAND_ACTION_CRW as isize,
    Cr = XED_OPERAND_ACTION_CR as isize,
}

impl From<xed_operand_action_enum_t> for OperandAction {
    fn from(x: xed_operand_action_enum_t) -> Self {
        Self::from_u32(x).unwrap_or(OperandAction::Invalid)
    }
}

impl From<OperandAction> for xed_operand_action_enum_t {
    fn from(x: OperandAction) -> Self {
        x as Self
    }
}
