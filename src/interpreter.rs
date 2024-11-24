use crate::{
    bytecode::Bytecode,
    context::Context,
    operations::{Operation, OperationResult},
    primitives::{Address, U256},
    Error,
};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    rc::Rc,
};
use thiserror::Error;

const MAXIMUM_STACK_SIZE: usize = 1024;

pub type Word = U256;

#[derive(Debug, Error)]
pub enum StackError {
    #[error("stack underflow")]
    Underflow,
}

#[derive(Debug, Default)]
pub struct Stack(Vec<Word>);

impl Deref for Stack {
    type Target = Vec<Word>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Stack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Stack {
    pub fn push(&mut self, word: Word) -> Result<(), StackError> {
        self.0.push(word);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Word, StackError> {
        self.0.pop().ok_or(StackError::Underflow)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Account {
    pub balance: U256,
    pub nonce: U256,
    pub storage: HashMap<Word, Word>,
    pub code: Bytecode,
}

#[derive(Debug, Default)]
pub struct Frame {
    pub caller: Address,
    pub value: U256,
    pub gas_limit: U256,
    pub account: Rc<Account>,

    pub program_counter: usize,
    pub gas_used: U256,

    pub stack: Stack,
    pub memory: Memory,
    pub output: Vec<u8>,

    pub can_modify_state: bool,

    operation_counter: usize,
}

impl From<&Context> for Frame {
    fn from(context: &Context) -> Self {
        Frame {
            caller: context.transaction.sender,
            value: context.transaction.value,
            gas_limit: context.transaction.gas_limit,
            account: context
                .state
                .get(&context.transaction.recipient)
                .cloned()
                .unwrap_or_default(),
            program_counter: Default::default(),
            gas_used: Default::default(),
            stack: Default::default(),
            memory: Default::default(),
            output: Default::default(),
            can_modify_state: false,
            operation_counter: 0,
        }
    }
}

impl Frame {
    pub fn next_operation(&mut self) -> Option<Operation> {
        self.account.code.get(self.operation_counter).cloned()
    }

    pub fn apply(&mut self, result: OperationResult) -> Result<(), Error> {
        self.gas_used = self
            .gas_used
            .checked_add(result.gas_used)
            .ok_or(Error::GasUsedOverflow)?;
        self.program_counter += result.program_counter_increment;
        self.operation_counter += 1;
        Ok(())
    }

    pub fn run(&mut self, context: &Context) -> Result<(), Error> {
        // TODO: (journal) value transfer
        while let Some(operation) = self.next_operation() {
            let result = self.dispatch_operation(operation, context)?;
            self.apply(result)?;
        }
        Ok(())
    }

    pub fn dispatch_operation(
        &mut self,
        operation: Operation,
        context: &Context,
    ) -> Result<OperationResult, Error> {
        operation.apply(&mut self.stack, &mut self.memory, context)
    }
}

#[derive(Debug, Default)]
pub struct Memory {
    data: Vec<u8>,
    active_word_count: usize,
}

#[derive(Debug, Default)]
pub struct Interpreter {
    frames: Vec<Frame>,
}

#[derive(Debug, Default)]
pub struct Execution {
    pub context: Context,
    pub frame: Frame,
}

impl Execution {
    pub fn output(&self) -> (&Stack, &[u8]) {
        let stack = &self.frame.stack;
        let output = &self.frame.output;
        (stack, output)
    }
}

impl Interpreter {
    pub fn execute(mut self, context: Context) -> Result<Execution, Error> {
        let frame = From::from(&context);
        self.frames.push(frame);
        self.run(&context)?;
        let frame = self.frames.pop().ok_or(Error::FrameUnderflow)?;
        Ok(Execution { context, frame })
    }

    pub fn run(&mut self, context: &Context) -> Result<(), Error> {
        let frame = self.frames.last_mut().ok_or(Error::FrameUnderflow)?;
        frame.run(context)?;
        Ok(())
    }
}
