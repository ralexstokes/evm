use crate::bytecode::Bytecode;

#[derive(Debug, Default)]
pub struct Block {}

#[derive(Debug, Default)]
pub struct Transaction {}

#[derive(Debug, Default)]
pub struct Context {
    pub block: Block,
    pub transaction: Transaction,
    pub input: Bytecode,
}
