use num_traits::FromPrimitive;
use xed_sys2::xed_interface::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Primitive)]
pub enum Chip {
    Invalid = XED_CHIP_INVALID as isize,
    I86 = XED_CHIP_I86 as isize,
    I86FP = XED_CHIP_I86FP as isize,
    I286Real = XED_CHIP_I286REAL as isize,
    I286 = XED_CHIP_I286 as isize,
    I2186FP = XED_CHIP_I2186FP as isize,
    I386Real = XED_CHIP_I386REAL as isize,
    I386 = XED_CHIP_I386 as isize,
    I486Real = XED_CHIP_I486REAL as isize,
    I486 = XED_CHIP_I486 as isize,
    PentiumReal = XED_CHIP_PENTIUMREAL as isize,
    Pentium = XED_CHIP_PENTIUM as isize,
    Quark = XED_CHIP_QUARK as isize,
    PentiumMmxReal = XED_CHIP_PENTIUMMMXREAL as isize,
    PentiumMmx = XED_CHIP_PENTIUMMMX as isize,
    AllReal = XED_CHIP_ALLREAL as isize,
    PentiumPro = XED_CHIP_PENTIUMPRO as isize,
    Pentium2 = XED_CHIP_PENTIUM2 as isize,
    Pentium3 = XED_CHIP_PENTIUM3 as isize,
    Pentium4 = XED_CHIP_PENTIUM4 as isize,
    P4Prescott = XED_CHIP_P4PRESCOTT as isize,
    P4PrescottNoLahf = XED_CHIP_P4PRESCOTT_NOLAHF as isize,
    P4PrescotVtx = XED_CHIP_P4PRESCOTT_VTX as isize,
    Core2 = XED_CHIP_CORE2 as isize,
    Penryn = XED_CHIP_PENRYN as isize,
    PenrynE = XED_CHIP_PENRYN_E as isize,
    Nehalem = XED_CHIP_NEHALEM as isize,
    Westmere = XED_CHIP_WESTMERE as isize,
    Bonnell = XED_CHIP_BONNELL as isize,
    Saltwell = XED_CHIP_SALTWELL as isize,
    Silvermont = XED_CHIP_SILVERMONT as isize,
    AMD = XED_CHIP_AMD as isize,
    Goldmont = XED_CHIP_GOLDMONT as isize,
    Tremont = XED_CHIP_TREMONT as isize,
    SandyBridge = XED_CHIP_SANDYBRIDGE as isize,
    IvyBridge = XED_CHIP_IVYBRIDGE as isize,
    Skylake = XED_CHIP_SKYLAKE as isize,
    SkylakeServer = XED_CHIP_SKYLAKE_SERVER as isize,
    KNL = XED_CHIP_KNL as isize,
    Icelake = XED_CHIP_ICELAKE as isize,
    IcelakeServer = XED_CHIP_ICELAKE_SERVER as isize,
    Future = XED_CHIP_FUTURE as isize,
    All = XED_CHIP_ALL as isize,
}

impl From<xed_chip_enum_t> for Chip {
    fn from(x: xed_chip_enum_t) -> Self {
        use self::Chip::*;

        Self::from_u32(x).unwrap_or(Invalid)
    }
}

impl From<Chip> for xed_chip_enum_t {
    fn from(x: Chip) -> Self {
        x as xed_chip_enum_t
    }
}
