use crate::interpreter::StackError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Hex(#[from] hex::FromHexError),
    #[error("expected opcode at position but none was found")]
    InvalidCode,
    #[error("encountered an unknown opcode {0}")]
    InvalidOpcode(u8),
    #[error(transparent)]
    Stack(#[from] StackError),
    #[error("gas used overflowed the maximum amount")]
    GasUsedOverflow,
    #[error("expected frame but there was none")]
    FrameUnderflow,
}
