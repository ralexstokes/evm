use crate::context::Context;
use crate::storage::Storage;
use crate::{Address, Error, U256};

const MAXIMUM_STACK_SIZE: usize = 1024;

type Word = U256;
type Code = Vec<u8>;

#[derive(Debug, Default)]
pub struct Account {
    balance: U256,
    nonce: U256,
    storage: Storage,
    code: Code,
}

#[derive(Debug, Default)]
pub struct Frame {
    caller: Address,
    value: U256,
    account: Account,

    program_counter: U256,
    gas_used: U256,
    output: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct Memory {
    data: Vec<u8>,
    active_word_count: usize,
}

#[derive(Debug, Default)]
pub struct Interpreter {
    frames: Vec<Frame>,
    stack: Vec<Word>,
    memory: Memory,
    can_modify_state: bool,
}

#[derive(Debug, Default)]
pub struct Execution {
    context: Context,
}

impl Interpreter {
    pub fn execute(self, context: Context) -> Result<Execution, Error> {
        Ok(Execution { context })
    }
}
