use num_traits::FromPrimitive;
use xed_sys2::xed_interface::*;

/// TODO: Document, impement Error trait
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Primitive)]
pub enum Error {
    BadEvexLL = XED_ERROR_BAD_EVEX_LL as isize,
    BadEvexUbit = XED_ERROR_BAD_EVEX_UBIT as isize,
    BadEvexVPrime = XED_ERROR_BAD_EVEX_V_PRIME as isize,
    BadEvexZNoMasking = XED_ERROR_BAD_EVEX_Z_NO_MASKING as isize,
    BadLegacyPrefix = XED_ERROR_BAD_LEGACY_PREFIX as isize,
    BadLockPrefix = XED_ERROR_BAD_LOCK_PREFIX as isize,
    BadMap = XED_ERROR_BAD_MAP as isize,
    BadMemopIndex = XED_ERROR_BAD_MEMOP_INDEX as isize,
    BadRegister = XED_ERROR_BAD_REGISTER as isize,
    BadRepPrefix = XED_ERROR_BAD_REP_PREFIX as isize,
    BadRexPrefix = XED_ERROR_BAD_REX_PREFIX as isize,
    BufferTooShort = XED_ERROR_BUFFER_TOO_SHORT as isize,
    CallbackProblem = XED_ERROR_CALLBACK_PROBLEM as isize,
    GatherRegs = XED_ERROR_GATHER_REGS as isize,
    GeneralError = XED_ERROR_GENERAL_ERROR as isize,
    InstrTooLong = XED_ERROR_INSTR_TOO_LONG as isize,
    InvalidForChip = XED_ERROR_INVALID_FOR_CHIP as isize,
    InvalidMode = XED_ERROR_INVALID_MODE as isize,
    //NoAgenCallbackRegistered = XED_ERROR_NO_AGEN_CALLBACK_REGISTERED as isize,
    NoOutputPointer = XED_ERROR_NO_OUTPUT_POINTER as isize,

    None = XED_ERROR_NONE as isize,
}

impl From<xed_error_enum_t> for Error {
    fn from(x: xed_error_enum_t) -> Self {
        use self::Error::*;

        Self::from_u32(x).unwrap_or(GeneralError)
    }
}

impl From<Error> for xed_error_enum_t {
    fn from(x: Error) -> xed_error_enum_t {
        x as xed_error_enum_t
    }
}
