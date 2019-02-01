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

pub type Result<T> = std::result::Result<T, Error>;
