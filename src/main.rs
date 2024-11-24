use evm::{
    context, context::Transaction as TransactionContext, interpreter::Account, parse_hex, Context,
    Interpreter,
};
use std::error::Error;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut handle = io::stdin();
    handle.read_to_string(&mut input)?;
    let input = input.split(",").collect::<Vec<&str>>();
    let bytecode = input[0].try_into()?;
    let input = parse_hex(input[1])?;

    let transaction = TransactionContext {
        gas_limit: Default::default(),
        gas_price: Default::default(),
        sender: Default::default(),
        recipient: Default::default(),
        value: Default::default(),
        input,
    };
    let account = Account {
        code: bytecode,
        ..Default::default()
    };
    let state = context::State::from([(transaction.recipient, account)].into_iter());

    let context = Context {
        block: Default::default(),
        transaction,
        precompiles: Default::default(),
        state,
    };

    let execution = Interpreter::default().execute(context)?;
    dbg!(&execution);
    dbg!(execution.output());
    Ok(())
}
