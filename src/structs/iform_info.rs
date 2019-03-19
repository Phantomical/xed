use crate::*;
use xed_sys2::xed_interface::*;

/// Information for an `IForm`.
///
/// Intel XED classifies instructions as iclasses (ADD, SUB, MUL,
/// etc.) of type [`IClass`](crate::IClass). TO get more information
/// about instructions and their operands, Intel XED creates iforms
/// of type [`IForm`](crate::IForm). The iforms are supposed to aid
/// in creating dispatch tables for instructions. You can often use
/// a flat array indexed by iform.
///
/// The iforms sometimes d onot uniquely identify instructions. For
/// example, many instructions in the ISA are "scalable" in that
/// their operand width depends on the machine mode and the prefixes.
/// The memory operation of these scalable opcodes is either 16 bits,
/// 32 bits or 64 bits. The same opcode can represent several
/// instructions if you factor in the machine mode and prefixes.
/// Those instructions often map to a single iform and need to be
/// further refined by the
/// [`OperandValues::effective_operand_width`](crate::OperandValues::effective_operand_width)
/// function.
///
/// The names the of the iforms are derived from information about
/// the [`IClass`](crate::IClass) and the names of their explicit
/// operands (the names of nonterminals in the Intel XED internal
/// grammar) and the data types of those operands. Other information
/// is sometimes included to disambiguate similar instructions. For
/// example, there are several opcodes and operands for encoding a
/// 1-byte register-register ADD instruction as well as the 1-byte
/// register-immediate ADD, so to differentiate those, Intel XED
/// includes the opcode bytes as suffixes for the iform name:
///
/// ```
/// ADD_GPR8_GPR8_00
/// ADD_GPR8_GPR8_02
/// ADD_GPR8_IMMb_80r2
/// ADD_GPR8_IMMb_82r0
/// ```
///
/// The naming scheme for iforms can get rather complex and continues
/// to evolve over time as the instruction architecture grows. They
/// mostly use the lower-case letter codes found in the opcode map
/// found in the appendix to the Intel(R) 64 and IA-32 Architecture
/// Software Developers Manual. For example, the scalable instructions
/// mentioned about use the "v" code which the manuals describe as
/// representing 16, 32, or 64b operands depending on the effective
/// operand size. The code "z" implies either 16 or 32b operation.
/// When the effective operand size is 64, the operand is still 32b.
/// Other common suffices one might see are "d" for 32b and "q" for
/// 64b. The codes "ps" and "pd" stand for packed scalar (single
/// precision floating point) and packed double (double precision
/// floating point). The code "dq" is used to describe 128b (16B)
/// quantities typically in memory or an XMM register. Similarly,
/// "qq" describes a 256b (32B) quantity in memory or a YMM register.
/// In many cases the codes were sufficient to describe what was
/// needed; in other cases it was necessary to improvise.
///
/// To get the first iform of a particualy iclass you can use
/// [`IClass::first_iform`](crate::IClass::first_iform) at runtime.
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct IFormInfo {
    inner: xed_iform_info_t,
}

impl IFormInfo {
    pub fn into_inner(self) -> xed_iform_info_t {
        self.inner
    }

    pub fn inner(&self) -> &xed_iform_info_t {
        &self.inner
    }
    pub fn inner_mut(&mut self) -> &mut xed_iform_info_t {
        &mut self.inner
    }

    pub fn category(&self) -> Category {
        self.inner.category().into()
    }
    pub fn extension(&self) -> Extension {
        self.inner.extension().into()
    }
    pub fn iclass(&self) -> IClass {
        self.inner.iclass().into()
    }
    pub fn isa_set(&self) -> IsaSet {
        self.inner.isa_set().into()
    }
    pub fn string_table_idx(&self) -> u32 {
        self.inner.string_table_idx().into()
    }
}

impl From<xed_iform_info_t> for IFormInfo {
    fn from(inner: xed_iform_info_t) -> Self {
        Self { inner }
    }
}
