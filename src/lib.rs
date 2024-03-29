//!

#[macro_use]
extern crate enum_primitive_derive;

mod builders;
mod enums;
mod funcs;
mod init;
mod structs;
mod traits;
mod util;

pub use self::builders::*;
pub use self::enums::*;
pub use self::funcs::*;
pub use self::structs::*;
pub use self::traits::*;

pub use self::init::init_tables;

pub type Result<T> = std::result::Result<T, Error>;
