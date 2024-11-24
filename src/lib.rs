pub mod bytecode;
pub mod context;
mod error;
pub mod gas_schedule;
pub mod interpreter;
pub mod operations;
pub mod primitives;

pub use context::Context;
pub use error::Error;
pub use interpreter::Interpreter;

pub fn parse_hex(input: &str) -> Result<Vec<u8>, Error> {
    let input = input.trim().strip_prefix("0x").unwrap_or(input);
    hex::decode(input).map_err(|err| err.into())
}
