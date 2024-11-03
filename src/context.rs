use crate::storage::Storage;
use crate::Address;
use crate::U256;

#[derive(Debug, Default)]
pub struct Block {}

#[derive(Debug, Default)]
pub struct Transaction {
    pub gas_limit: U256,
    pub gas_price: U256,
    pub input: Vec<u8>,
    pub sender: Address,
}

#[derive(Debug, Default)]
pub struct Precompiles {}

#[derive(Debug, Default)]
pub struct Context {
    pub block: Block,
    pub transaction: Transaction,
    pub precompiles: Precompiles,
    pub state: Storage,
}
