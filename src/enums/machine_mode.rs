use num_traits::FromPrimitive;
use xed_sys2::xed_interface::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Primitive)]
pub enum MachineMode {
    Long64 = XED_MACHINE_MODE_LONG_64 as isize,
    LongCompat32 = XED_MACHINE_MODE_LONG_COMPAT_32 as isize,
    LongCompat16 = XED_MACHINE_MODE_LONG_COMPAT_16 as isize,
    Legacy32 = XED_MACHINE_MODE_LEGACY_32 as isize,
    Legacy16 = XED_MACHINE_MODE_LEGACY_16 as isize,
    Real16 = XED_MACHINE_MODE_REAL_16 as isize,
    Invalid = XED_MACHINE_MODE_INVALID as isize,
}

impl From<xed_machine_mode_enum_t> for MachineMode {
    fn from(x: xed_machine_mode_enum_t) -> MachineMode {
        use self::MachineMode::*;

        Self::from_u32(x).unwrap_or(Invalid)
    }
}

impl From<MachineMode> for xed_machine_mode_enum_t {
    fn from(x: MachineMode) -> xed_machine_mode_enum_t {
        x as xed_machine_mode_enum_t
    }
}
