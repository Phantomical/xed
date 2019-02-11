use num_traits::FromPrimitive;
use xed_sys2::xed_interface::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Primitive)]
pub enum OperandElementXType {
    Invalid = XED_OPERAND_XTYPE_INVALID as isize,
    B80 = XED_OPERAND_XTYPE_B80 as isize,
    F16 = XED_OPERAND_XTYPE_F16 as isize,
    F32 = XED_OPERAND_XTYPE_F32 as isize,
    F64 = XED_OPERAND_XTYPE_F64 as isize,
    F80 = XED_OPERAND_XTYPE_F80 as isize,
    I1 = XED_OPERAND_XTYPE_I1 as isize,
    I16 = XED_OPERAND_XTYPE_I16 as isize,
    I32 = XED_OPERAND_XTYPE_I32 as isize,
    I64 = XED_OPERAND_XTYPE_I64 as isize,
    I8 = XED_OPERAND_XTYPE_I8 as isize,
    Int = XED_OPERAND_XTYPE_INT as isize,
    Struct = XED_OPERAND_XTYPE_STRUCT as isize,
    U128 = XED_OPERAND_XTYPE_U128 as isize,
    U16 = XED_OPERAND_XTYPE_U16 as isize,
    U256 = XED_OPERAND_XTYPE_U256 as isize,
    U32 = XED_OPERAND_XTYPE_U32 as isize,
    U64 = XED_OPERAND_XTYPE_U64 as isize,
    U8 = XED_OPERAND_XTYPE_U8 as isize,
    UInt = XED_OPERAND_XTYPE_UINT as isize,
    Var = XED_OPERAND_XTYPE_VAR as isize,
}

impl From<xed_operand_element_xtype_enum_t> for OperandElementXType {
    fn from(x: xed_operand_element_xtype_enum_t) -> Self {
        Self::from_u32(x).unwrap_or(OperandElementXType::Invalid)
    }
}

impl From<OperandElementXType> for xed_operand_element_xtype_enum_t {
    fn from(x: OperandElementXType) -> Self {
        x as Self
    }
}
