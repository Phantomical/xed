use crate::*;
use xed_sys2::xed_interface::*;

use std::fmt;
use std::mem;

/// Encapsulates machine mode for decode/encoder requests.
///
/// It specifies the machine operating mode as a
/// [`MachineMode`](crate::MachineMode) for decoding and
/// encoding. The machine mode corresponds to the default
/// data operand width for that mode. For all modes other
/// than 64b long mode
/// [`MachineMode::Long64`](crate::MachineMode::Long64),
/// a default addressing width, and a stack addressing
/// width must be supplied of type
/// [`AddressWidth`](crate::AddressWidth).
#[derive(Copy, Clone, Debug)]
pub struct State {
    inner: xed_state_t,
}

// Constructors
impl State {
    pub fn new(mmode: MachineMode, stack_addr_width: AddressWidth) -> Self {
        init_tables();

        unsafe {
            let mut inner = mem::zeroed();

            xed_state_init2(&mut inner as *mut _, mmode.into(), stack_addr_width.into());

            Self { inner }
        }
    }
    pub fn zeroed() -> Self {
        unsafe {
            Self {
                inner: mem::zeroed(),
            }
        }
    }

    pub fn machine_mode(&self) -> MachineMode {
        unsafe { xed_state_get_machine_mode(&self.inner as *const _).into() }
    }

    pub fn address_width(&self) -> AddressWidth {
        unsafe { xed_state_get_address_width(&self.inner as *const _).into() }
    }

    pub fn stack_address_width(&self) -> AddressWidth {
        unsafe { xed_state_get_stack_address_width(&self.inner as *const _).into() }
    }

    pub fn long64_mode(&self) -> bool {
        unsafe { xed_state_long64_mode(&self.inner as *const _) != 0 }
    }

    pub fn mode_width_16(&self) -> bool {
        unsafe { xed_state_mode_width_16(&self.inner as *const _) != 0 }
    }

    pub fn mode_width_32(&self) -> bool {
        unsafe { xed_state_mode_width_32(&self.inner as *const _) != 0 }
    }

    pub fn real_mode(&self) -> bool {
        unsafe { xed_state_real_mode(&self.inner as *const _) != 0 }
    }

    pub fn set_machine_mode(&mut self, mmode: MachineMode) {
        unsafe {
            xed_state_set_machine_mode(&mut self.inner as *mut _, mmode.into());
        }
    }

    pub fn set_stack_address_width(&mut self, stack_addr_width: AddressWidth) {
        unsafe {
            xed_state_set_stack_address_width(&mut self.inner as *mut _, stack_addr_width.into());
        }
    }

    pub fn into_inner(self) -> xed_state_t {
        self.inner
    }
}

impl Default for State {
    fn default() -> Self {
        Self::zeroed()
    }
}

impl fmt::Display for State {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use std::ffi::CStr;
        use std::os::raw::c_int;

        let mut buf = [0; 2049];

        unsafe {
            xed_state_print(
                &self.inner as *const _,
                (&mut buf).as_mut_ptr() as *mut _,
                // ensure that a nul byte is always placed
                // at the end of the buffer
                (buf.len() - 1) as c_int,
            );

            let s = CStr::from_ptr((&mut buf).as_mut_ptr())
                .to_str()
                .expect("xed_state_print printed an invalid character");

            fmt.write_str(s)
        }
    }
}

impl From<xed_state_t> for State {
    fn from(inner: xed_state_t) -> Self {
        Self { inner }
    }
}
