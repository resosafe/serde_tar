#[macro_use]
extern crate serde;

pub mod error;
pub mod ser;


pub use error::Error;
pub use ser::{to_writer, Serializer};