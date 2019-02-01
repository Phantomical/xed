use xed_sys2::xed_interface::*;

use std::mem;

use crate::enums::*;
use crate::init::init_tables;

#[derive(Copy, Clone, Debug)]
pub struct State {
    inner: xed_state_t,
}

impl State {
    /// Construct a new `State` with a machine mode and
    /// stack address width.
    ///
    /// In `64b` mode ([`MachineMode::Long64b`]) the address
    /// widths are also `64b` ([`AddressWidth::Width64b`]).
    /// In other machine modes, you must specify valid\
    /// addressing widths.
    pub fn new(mmode: MachineMode, stack_addr_width: AddressWidth) -> Self {
        // Tables must be initialized before using XED.
        // Creating a new state is a good place to do that.
        init_tables();

        // xed_state_t doesn't implement Default
        let mut inner: xed_state_t = unsafe { mem::zeroed() };

        unsafe {
            xed_state_init2(&mut inner as *mut _, mmode.into(), stack_addr_width.into());
        }

        Self { inner }
    }
}
