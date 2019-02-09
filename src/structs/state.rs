use xed_sys2::xed_interface::*;

use std::mem;

use crate::*;

/// Encapsulates machine modes for decoder/encoder requests.
/// 
/// It specifies the machine operating mode as a 
/// [`MachineMode`](crate::MachineMode) for
/// decoding and encoding. The machine mode corresponds
/// to the default data operand width for that mode. For
/// all modes other than the 64b long mode 
/// ([`MachineMode::Long64`](crate::MachineMode::Long64)),
/// a default addressing width, a stack addressing width
/// must be supplied of type 
/// [`AddressWidth`](crate::AddressWidth).
#[derive(Copy, Clone, Debug)]
pub struct State {
    pub(crate) inner: xed_state_t,
}

impl State {
    /// Construct a new `State` with a machine mode and
    /// stack address width.
    ///
    /// In `64b` mode ([`MachineMode::Long64b`]) the address
    /// widths are also `64b` ([`AddressWidth::Width64b`]).
    /// In other machine modes, you must specify valid
    /// addressing widths.
    pub fn new<W>(mmode: MachineMode, stack_addr_width: W) -> Self 
    where
        W: Into<Option<AddressWidth>>
    {
        // Tables must be initialized before using XED.
        // Creating a new state is a good place to do that.
        init_tables();

        let stack_addr_width = stack_addr_width
            .into()
            .unwrap_or(AddressWidth::Invalid);

        // xed_state_t doesn't implement Default
        let mut inner: xed_state_t = unsafe { mem::zeroed() };

        unsafe {
            xed_state_init2(&mut inner as *mut _, mmode.into(), stack_addr_width.into());
        }

        Self { inner }
    }

    pub fn into_inner(self) -> xed_state_t {
        self.inner
    }
}
