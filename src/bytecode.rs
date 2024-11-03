use crate::Error;

fn parse_hex(input: &str) -> Result<Vec<u8>, Error> {
    let input = input.trim().strip_prefix("0x").unwrap_or(input);
    hex::decode(input).map_err(|err| err.into())
}

#[derive(Debug, Default)]
pub struct Bytecode(pub Vec<u8>);

impl TryFrom<&str> for Bytecode {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Error> {
        let input = parse_hex(value)?;
        Ok(Self(input))
    }
}
