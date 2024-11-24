use crate::{
    interpreter::Account,
    primitives::{Address, U256},
};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    rc::Rc,
};

type StateBacking = HashMap<Address, Rc<Account>>;

#[derive(Debug, Default)]
pub struct State(StateBacking);

impl Deref for State {
    type Target = StateBacking;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for State {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for State
where
    T: Iterator<Item = (Address, Account)>,
{
    fn from(values: T) -> Self {
        Self(Iterator::collect(
            values.map(|(address, account)| (address, Rc::new(account))),
        ))
    }
}

#[derive(Debug, Default)]
pub struct Block {}

#[derive(Debug, Default)]
pub struct Transaction {
    pub gas_limit: U256,
    pub gas_price: U256,
    pub sender: Address,
    pub recipient: Address,
    pub value: U256,
    pub input: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct Precompiles {}

#[derive(Debug, Default)]
pub struct Context {
    pub block: Block,
    pub transaction: Transaction,
    pub precompiles: Precompiles,
    pub state: State,
}
