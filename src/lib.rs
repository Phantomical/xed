//!

#[macro_use]
extern crate enum_primitive_derive;

mod enums;
mod funcs;
mod init;
mod structs;

pub use self::enums::*;
pub use self::funcs::*;
pub use self::structs::*;

pub use self::init::{init_tables, is_initialized};

pub type Result<T> = std::result::Result<T, Error>;
