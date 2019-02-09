use num_traits::FromPrimitive;
use xed_sys2::xed_interface::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Primitive)]
pub enum Extension {
    Invalid = XED_EXTENSION_INVALID as isize,
    _3DNow = XED_EXTENSION_3DNOW as isize,
    AdoxAdcx = XED_EXTENSION_ADOX_ADCX as isize,
    Aes = XED_EXTENSION_AES as isize,
    Avx = XED_EXTENSION_AVX as isize,
    Avx2 = XED_EXTENSION_AVX2 as isize,
    Avx2Gather = XED_EXTENSION_AVX2GATHER as isize,
    Avx512Evex = XED_EXTENSION_AVX512EVEX as isize,
    AvxAes = XED_EXTENSION_AVXAES as isize,
    Base = XED_EXTENSION_BASE as isize,
    Bmi1 = XED_EXTENSION_BMI1 as isize,
    Bmi2 = XED_EXTENSION_BMI2 as isize,
    Cet = XED_EXTENSION_CET as isize,
    ClDemote = XED_EXTENSION_CLDEMOTE as isize,
    ClFlushOpt = XED_EXTENSION_CLFLUSHOPT as isize,
    ClFsh = XED_EXTENSION_CLFSH as isize,
    ClWb = XED_EXTENSION_CLWB as isize,
    ClZero = XED_EXTENSION_CLZERO as isize,
    F16C = XED_EXTENSION_F16C as isize,
    Fma = XED_EXTENSION_FMA as isize,
    Fma4 = XED_EXTENSION_FMA4 as isize,
    Gfni = XED_EXTENSION_GFNI as isize,
    InvPcid = XED_EXTENSION_INVPCID as isize,
    LongMode = XED_EXTENSION_LONGMODE as isize,
    Lzcnt = XED_EXTENSION_LZCNT as isize,
    Mmx = XED_EXTENSION_MMX as isize,
    Monitor = XED_EXTENSION_MONITOR as isize,
    MonitorX = XED_EXTENSION_MONITORX as isize,
    Mpx = XED_EXTENSION_MPX as isize,
    Pause = XED_EXTENSION_PAUSE as isize,
    PclMulQdq = XED_EXTENSION_PCLMULQDQ as isize,
    PConfig = XED_EXTENSION_PCONFIG as isize,
    Pku = XED_EXTENSION_PKU as isize,
    PrefetchWT1 = XED_EXTENSION_PREFETCHWT1 as isize,
    Pt = XED_EXTENSION_PT as isize,
    RdPid = XED_EXTENSION_RDPID as isize,
    RdRand = XED_EXTENSION_RDRAND as isize,
    RdSeed = XED_EXTENSION_RDSEED as isize,
    RdTscp = XED_EXTENSION_RDTSCP as isize,
    RdWrFsGs = XED_EXTENSION_RDWRFSGS as isize,
    Rtm = XED_EXTENSION_RTM as isize,
    Sgx = XED_EXTENSION_SGX as isize,
    SgxEnclv = XED_EXTENSION_SGX_ENCLV as isize,
    Sha = XED_EXTENSION_SHA as isize,
    SMap = XED_EXTENSION_SMAP as isize,
    Smx = XED_EXTENSION_SMX as isize,
    Sse = XED_EXTENSION_SSE as isize,
    Sse2 = XED_EXTENSION_SSE2 as isize,
    Sse3 = XED_EXTENSION_SSE3 as isize,
    Sse4 = XED_EXTENSION_SSE4 as isize,
    Svm = XED_EXTENSION_SVM as isize,
    Tbm = XED_EXTENSION_TBM as isize,
    Vaes = XED_EXTENSION_VAES as isize,
    VmFunc = XED_EXTENSION_VMFUNC as isize,
    VPclMulQdq = XED_EXTENSION_VPCLMULQDQ as isize,
    Vtx = XED_EXTENSION_VTX as isize,
    WaitPkg = XED_EXTENSION_WAITPKG as isize,
    WbNoInvD = XED_EXTENSION_WBNOINVD as isize,
    X87 = XED_EXTENSION_X87 as isize,
    Xop = XED_EXTENSION_XOP as isize,
    XSave = XED_EXTENSION_XSAVE as isize,
    XSaveC = XED_EXTENSION_XSAVEC as isize,
    XSaveOpt = XED_EXTENSION_XSAVEOPT as isize,
    XSaves = XED_EXTENSION_XSAVES as isize,
}

impl From<xed_extension_enum_t> for Extension {
    fn from(x: xed_extension_enum_t) -> Self {
        Self::from_u32(x).unwrap_or(Extension::Invalid)
    }
}

impl From<Extension> for xed_extension_enum_t {
    fn from(x: Extension) -> Self {
        x as Self
    }
}
