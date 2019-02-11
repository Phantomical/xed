use num_traits::FromPrimitive;
use xed_sys2::xed_interface::*;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Primitive)]
pub enum OperandType {
    Invalid = XED_OPERAND_TYPE_INVALID as isize,
    Error = XED_OPERAND_TYPE_ERROR as isize,
    Imm = XED_OPERAND_TYPE_IMM as isize,
    ImmConst = XED_OPERAND_TYPE_IMM_CONST as isize,
    NtLookupFn = XED_OPERAND_TYPE_NT_LOOKUP_FN as isize,
    NtLookupFn4 = XED_OPERAND_TYPE_NT_LOOKUP_FN4 as isize,
    Reg = XED_OPERAND_TYPE_REG as isize,
}

impl From<xed_operand_type_enum_t> for OperandType {
    fn from(x: xed_operand_type_enum_t) -> Self {
        Self::from_u32(x).unwrap_or(OperandType::Invalid)
    }
}

impl From<OperandType> for xed_operand_type_enum_t {
    fn from(x: OperandType) -> Self {
        x as Self
    }
}
