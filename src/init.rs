use parking_lot::{Once, ONCE_INIT};
use xed_sys2::xed_interface::*;

static TABLES_INIT: Once = ONCE_INIT;

/// Initialize XED encode and decode tables.
///
/// This must be called before using XED.
/// If called multiple times the tables will
/// only be initialized once.
pub fn init_tables() {
    TABLES_INIT.call_once(|| unsafe {
        xed_tables_init();
    });
}
