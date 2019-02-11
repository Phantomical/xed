use num_traits::FromPrimitive;
use xed_sys2::xed_interface::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Primitive)]
pub enum OperandElementType {
    Invalid = XED_OPERAND_ELEMENT_TYPE_INVALID as isize,
    UInt = XED_OPERAND_ELEMENT_TYPE_UINT as isize,
    Int = XED_OPERAND_ELEMENT_TYPE_INT as isize,
    Single = XED_OPERAND_ELEMENT_TYPE_SINGLE as isize,
    Double = XED_OPERAND_ELEMENT_TYPE_DOUBLE as isize,
    LongDouble = XED_OPERAND_ELEMENT_TYPE_LONGDOUBLE as isize,
    LongBCD = XED_OPERAND_ELEMENT_TYPE_LONGBCD as isize,
    Struct = XED_OPERAND_ELEMENT_TYPE_STRUCT as isize,
    Variable = XED_OPERAND_ELEMENT_TYPE_VARIABLE as isize,
    Float16 = XED_OPERAND_ELEMENT_TYPE_FLOAT16 as isize,
}

impl From<xed_operand_element_type_enum_t> for OperandElementType {
    fn from(x: xed_operand_element_type_enum_t) -> Self {
        Self::from_u32(x).unwrap_or(OperandElementType::Invalid)
    }
}

impl From<OperandElementType> for xed_operand_element_type_enum_t {
    fn from(x: OperandElementType) -> Self {
        x as Self
    }
}
