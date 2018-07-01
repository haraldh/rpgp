#[macro_use]
extern crate nom;
// extern crate openssl;
extern crate base64;
extern crate byteorder;
extern crate crc24;
#[macro_use]
extern crate enum_primitive;
extern crate chrono;
#[macro_use]
extern crate failure;

#[cfg(test)]
#[macro_use]
extern crate hex_literal;

pub mod email;
pub mod key;

// public so it can be used in doc test
pub mod util;

mod armor;
mod errors;
mod header;
mod packet;
mod types;
