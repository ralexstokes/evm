use crate::gas_schedule;
use crate::interpreter::Word;
use crate::interpreter::{Memory, Stack};
use crate::primitives::U256;
use crate::{Context, Error};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    // Stop and arithmetic
    Stop,
    Add,
    Mul,
    Sub,
    Div,
    Sdiv,
    Mod,
    SMod,
    Addmod,
    Mulmod,
    Exp,
    SignExtend,
    // Comparison and bitwise operators
    Lt,
    Gt,
    Slt,
    Sgt,
    Eq,
    IsZero,
    And,
    Or,
    Xor,
    Not,
    Byte,
    Shl,
    Shr,
    Sar,
    // Hashing
    Keccack256,
    // Environment
    Address,
    Balance,
    Origin,
    Caller,
    CallValue,
    CallDataLoad,
    CallDataSize,
    CallDataCopy,
    CodeSize,
    CodeCopy,
    GasPrice,
    ExtCodeSize,
    ExtCodeCopy,
    ReturnDataSize,
    ReturnDataCopy,
    ExtCodeHash,
    // Block info
    Blockhash,
    Coinbase,
    Timestamp,
    Number,
    PrevRandao,
    GasLimit,
    ChainId,
    SelfBalance,
    BaseFee,
    // Stack, memory, storage, flow operations
    Pop,
    Mload,
    Mstore,
    MStore8,
    Sload,
    SStore,
    Jump,
    JumpI,
    Pc,
    MSize,
    Gas,
    JumpDest,
    // Push operations
    Push0,
    Push1(u8),
    Push2([u8; 2]),
    Push3([u8; 3]),
    Push4([u8; 4]),
    Push5([u8; 5]),
    Push6([u8; 6]),
    Push7([u8; 7]),
    Push8([u8; 8]),
    Push9([u8; 9]),
    Push10([u8; 10]),
    Push11([u8; 11]),
    Push12([u8; 12]),
    Push13([u8; 13]),
    Push14([u8; 14]),
    Push15([u8; 15]),
    Push16([u8; 16]),
    Push17([u8; 17]),
    Push18([u8; 18]),
    Push19([u8; 19]),
    Push20([u8; 20]),
    Push21([u8; 21]),
    Push22([u8; 22]),
    Push23([u8; 23]),
    Push24([u8; 24]),
    Push25([u8; 25]),
    Push26([u8; 26]),
    Push27([u8; 27]),
    Push28([u8; 28]),
    Push29([u8; 29]),
    Push30([u8; 30]),
    Push31([u8; 31]),
    Push32([u8; 32]),
    // Duplication operations
    Dup1,
    Dup2,
    Dup3,
    Dup4,
    Dup5,
    Dup6,
    Dup7,
    Dup8,
    Dup9,
    Dup10,
    Dup11,
    Dup12,
    Dup13,
    Dup14,
    Dup15,
    Dup16,
    // Exchange operations
    Swap1,
    Swap2,
    Swap3,
    Swap4,
    Swap5,
    Swap6,
    Swap7,
    Swap8,
    Swap9,
    Swap10,
    Swap11,
    Swap12,
    Swap13,
    Swap14,
    Swap15,
    Swap16,
    // Logging
    Log0,
    Log1,
    Log2,
    Log3,
    Log4,
    // System operations
    Create,
    Call,
    CallCode,
    Return,
    DelegateCall,
    Create2,
    StaticCall,
    Revert,
    Invalid,
    SelfDestruct,
}

#[derive(Debug, Default)]
pub struct OperationResult {
    pub gas_used: U256,
    pub program_counter_increment: usize,
}

impl From<&Operation> for OperationResult {
    fn from(value: &Operation) -> Self {
        OperationResult {
            gas_used: value.gas_used(),
            program_counter_increment: value.program_counter_increment(),
        }
    }
}

fn program_counter_increment_for_push(n: usize) -> usize {
    n + 1
}

impl Operation {
    pub(crate) const PUSH1_VALUE: u8 = 0x60;
    pub(crate) const PUSH32_VALUE: u8 = 0x7f;

    pub fn get_push_size(&self) -> Option<usize> {
        use Operation::*;

        match self {
            Push0 => Some(0),
            Push1(_) => Some(1),
            Push2(_) => Some(2),
            Push3(_) => Some(3),
            Push4(_) => Some(4),
            Push5(_) => Some(5),
            Push6(_) => Some(6),
            Push7(_) => Some(7),
            Push8(_) => Some(8),
            Push9(_) => Some(9),
            Push10(_) => Some(10),
            Push11(_) => Some(11),
            Push12(_) => Some(12),
            Push13(_) => Some(13),
            Push14(_) => Some(14),
            Push15(_) => Some(15),
            Push16(_) => Some(16),
            Push17(_) => Some(17),
            Push18(_) => Some(18),
            Push19(_) => Some(19),
            Push20(_) => Some(20),
            Push21(_) => Some(21),
            Push22(_) => Some(22),
            Push23(_) => Some(23),
            Push24(_) => Some(24),
            Push25(_) => Some(25),
            Push26(_) => Some(26),
            Push27(_) => Some(27),
            Push28(_) => Some(28),
            Push29(_) => Some(29),
            Push30(_) => Some(30),
            Push31(_) => Some(31),
            Push32(_) => Some(32),
            _ => None,
        }
    }

    pub fn is_push(&self) -> bool {
        self.get_push_size().is_some()
    }

    pub fn is_dup(&self) -> bool {
        use Operation::*;

        match self {
            Dup1 | Dup2 | Dup3 | Dup4 | Dup5 | Dup6 | Dup7 | Dup8 | Dup9 | Dup10 | Dup11
            | Dup12 | Dup13 | Dup14 | Dup15 | Dup16 => true,
            _ => false,
        }
    }

    pub fn is_swap(&self) -> bool {
        use Operation::*;

        match self {
            Swap1 | Swap2 | Swap3 | Swap4 | Swap5 | Swap6 | Swap7 | Swap8 | Swap9 | Swap10
            | Swap11 | Swap12 | Swap13 | Swap14 | Swap15 | Swap16 => true,
            _ => false,
        }
    }

    pub fn gas_schedule_cost(&self) -> usize {
        use Operation::*;

        match self {
            Stop | Return | Revert => gas_schedule::G_ZERO,
            Address | Origin | Caller | CallValue | CallDataSize | CodeSize | GasPrice
            | Coinbase | Timestamp | Number | PrevRandao | GasLimit | ChainId | ReturnDataSize
            | Pop | Pc | MSize | Gas | BaseFee | Push0 => gas_schedule::G_BASE,
            Add | Sub | Not | Lt | Gt | Slt | Sgt | Eq | IsZero | And | Or | Xor | Byte | Shl
            | Shr | Sar | CallDataLoad | Mload | Mstore | MStore8 => gas_schedule::G_VERY_LOW,
            operation
                // NOTE: Push0 should already match, but add conditional here for extra safety
                if (operation.is_push() | operation.is_dup() | operation.is_swap())
                    && operation != &Push0 =>
            {
                gas_schedule::G_VERY_LOW
            }
            Mul | Div | Sdiv | Mod | SMod | SignExtend | SelfBalance => gas_schedule::G_LOW,
            Addmod | Mulmod | Jump => gas_schedule::G_MID,
            JumpI => gas_schedule::G_HIGH,
            _ => todo!(),
        }
    }

    pub fn gas_used(&self) -> U256 {
        U256::from(self.gas_schedule_cost())
    }

    pub fn program_counter_increment(&self) -> usize {
        if let Some(push_size) = self.get_push_size() {
            program_counter_increment_for_push(push_size)
        } else {
            1
        }
    }

    pub fn apply(
        &self,
        stack: &mut Stack,
        memory: &mut Memory,
        context: &Context,
    ) -> Result<OperationResult, Error> {
        use Operation::*;
        match self {
            Add => self.do_add(stack),
            Push1(immediate) => self.do_push(&[*immediate], stack),
            Push32(immediate) => self.do_push(immediate, stack),
            _ => todo!(),
        }
    }

    pub fn do_add(&self, stack: &mut Stack) -> Result<OperationResult, Error> {
        let a = stack.pop()?;
        let b = stack.pop()?;
        stack.push(a + b)?;
        Ok(self.into())
    }

    pub fn do_push<const N: usize>(
        &self,
        immediate: &[u8; N],
        stack: &mut Stack,
    ) -> Result<OperationResult, Error> {
        stack.push(Word::from_be_slice(immediate))?;
        Ok(self.into())
    }
}
