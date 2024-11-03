use evm::{Context, Interpreter};
use std::error::Error;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut handle = io::stdin();
    handle.read_to_string(&mut input)?;
    let input = input.as_str().try_into()?;

    let context = Context {
        block: Default::default(),
        transaction: Default::default(),
        input,
    };
    let execution = Interpreter::default().execute(context)?;
    dbg!(execution);
    Ok(())
}
