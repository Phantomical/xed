use num_traits::FromPrimitive;
use xed_sys2::xed_interface::*;

use crate::Inst;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Primitive)]
pub enum Exception {
    Avx512E1 = XED_EXCEPTION_AVX512_E1 as isize,
    Avx512E10 = XED_EXCEPTION_AVX512_E10 as isize,
    Avx512E10Nf = XED_EXCEPTION_AVX512_E10NF as isize,
    Avx512E11 = XED_EXCEPTION_AVX512_E11 as isize,
    Avx512E11Nf = XED_EXCEPTION_AVX512_E11NF as isize,
    Avx512E12 = XED_EXCEPTION_AVX512_E12 as isize,
    Avx512E12Np = XED_EXCEPTION_AVX512_E12NP as isize,
    Avx512E1Nf = XED_EXCEPTION_AVX512_E1NF as isize,
    Avx512E2 = XED_EXCEPTION_AVX512_E2 as isize,
    Avx512E3 = XED_EXCEPTION_AVX512_E3 as isize,
    Avx512E3Nf = XED_EXCEPTION_AVX512_E3NF as isize,
    Avx512E4 = XED_EXCEPTION_AVX512_E4 as isize,
    Avx512E4Nf = XED_EXCEPTION_AVX512_E4NF as isize,
    Avx512E5 = XED_EXCEPTION_AVX512_E5 as isize,
    Avx512E5Nf = XED_EXCEPTION_AVX512_E5NF as isize,
    Avx512E6 = XED_EXCEPTION_AVX512_E6 as isize,
    Avx512E6Nf = XED_EXCEPTION_AVX512_E6NF as isize,
    Avx512E7Nm = XED_EXCEPTION_AVX512_E7NM as isize,
    Avx512E7Nm128 = XED_EXCEPTION_AVX512_E7NM128 as isize,
    Avx512E9Nf = XED_EXCEPTION_AVX512_E9NF as isize,
    Avx512K20 = XED_EXCEPTION_AVX512_K20 as isize,
    Avx512K21 = XED_EXCEPTION_AVX512_K21 as isize,
    AvxType1 = XED_EXCEPTION_AVX_TYPE_1 as isize,
    AvxType11 = XED_EXCEPTION_AVX_TYPE_11 as isize,
    AvxType12 = XED_EXCEPTION_AVX_TYPE_12 as isize,
    AvxType2 = XED_EXCEPTION_AVX_TYPE_2 as isize,
    AvxType2D = XED_EXCEPTION_AVX_TYPE_2D as isize,
    AvxType3 = XED_EXCEPTION_AVX_TYPE_3 as isize,
    AvxType4 = XED_EXCEPTION_AVX_TYPE_4 as isize,
    AvxType4M = XED_EXCEPTION_AVX_TYPE_4M as isize,
    AvxType5 = XED_EXCEPTION_AVX_TYPE_5 as isize,
    AvxType5L = XED_EXCEPTION_AVX_TYPE_5L as isize,
    AvxType6 = XED_EXCEPTION_AVX_TYPE_6 as isize,
    AvxType7 = XED_EXCEPTION_AVX_TYPE_7 as isize,
    AvxType8 = XED_EXCEPTION_AVX_TYPE_8 as isize,
    Invalid = XED_EXCEPTION_INVALID as isize,
    MmxFp = XED_EXCEPTION_MMX_FP as isize,
    MmxFp16Align = XED_EXCEPTION_MMX_FP_16ALIGN as isize,
    MmxMem = XED_EXCEPTION_MMX_MEM as isize,
    MmxNoFP = XED_EXCEPTION_MMX_NOFP as isize,
    MmxNoFP2 = XED_EXCEPTION_MMX_NOFP2 as isize,
    MmxNoMem = XED_EXCEPTION_MMX_NOMEM as isize,
    SseType1 = XED_EXCEPTION_SSE_TYPE_1 as isize,
    SseType2 = XED_EXCEPTION_SSE_TYPE_2 as isize,
    SseType2D = XED_EXCEPTION_SSE_TYPE_2D as isize,
    SseType3 = XED_EXCEPTION_SSE_TYPE_3 as isize,
    SseType4 = XED_EXCEPTION_SSE_TYPE_4 as isize,
    SseType4M = XED_EXCEPTION_SSE_TYPE_4M as isize,
    SseType5 = XED_EXCEPTION_SSE_TYPE_5 as isize,
    SseType7 = XED_EXCEPTION_SSE_TYPE_7 as isize,
}

impl From<&'_ Inst> for Exception {
    fn from(x: &Inst) -> Self {
        unsafe { xed_inst_exception(x.inner_ptr()).into() }
    }
}

impl From<xed_exception_enum_t> for Exception {
    fn from(x: xed_exception_enum_t) -> Self {
        Self::from_u32(x).unwrap_or(Exception::Invalid)
    }
}

impl From<Exception> for xed_exception_enum_t {
    fn from(x: Exception) -> Self {
        x as Self
    }
}
