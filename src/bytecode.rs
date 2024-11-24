use crate::{operations::Operation, parse_hex, Error};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Default, Clone)]
pub struct Bytecode(Vec<Operation>);

impl TryFrom<&str> for Bytecode {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Error> {
        let input = parse_hex(value)?;
        parse(&input).map(Self)
    }
}

impl Deref for Bytecode {
    type Target = Vec<Operation>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bytecode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn parse_push<'a>(opcode: u8, input: &mut impl Iterator<Item = &'a u8>) -> Operation {
    let immediate_size = (opcode - Operation::PUSH1_VALUE + 1) as usize;
    let mut immediate = input.take(immediate_size).cloned().collect::<Vec<_>>();
    immediate.resize(immediate_size, 0);

    use Operation::*;
    match immediate_size {
        1 => Push1(immediate[0]),
        2 => Push2(immediate.try_into().unwrap()),
        3 => Push3(immediate.try_into().unwrap()),
        4 => Push4(immediate.try_into().unwrap()),
        5 => Push5(immediate.try_into().unwrap()),
        6 => Push6(immediate.try_into().unwrap()),
        7 => Push7(immediate.try_into().unwrap()),
        8 => Push8(immediate.try_into().unwrap()),
        9 => Push9(immediate.try_into().unwrap()),
        10 => Push10(immediate.try_into().unwrap()),
        11 => Push11(immediate.try_into().unwrap()),
        12 => Push12(immediate.try_into().unwrap()),
        13 => Push13(immediate.try_into().unwrap()),
        14 => Push14(immediate.try_into().unwrap()),
        15 => Push15(immediate.try_into().unwrap()),
        16 => Push16(immediate.try_into().unwrap()),
        17 => Push17(immediate.try_into().unwrap()),
        18 => Push18(immediate.try_into().unwrap()),
        19 => Push19(immediate.try_into().unwrap()),
        21 => Push21(immediate.try_into().unwrap()),
        22 => Push22(immediate.try_into().unwrap()),
        23 => Push23(immediate.try_into().unwrap()),
        24 => Push24(immediate.try_into().unwrap()),
        25 => Push25(immediate.try_into().unwrap()),
        27 => Push27(immediate.try_into().unwrap()),
        28 => Push28(immediate.try_into().unwrap()),
        29 => Push29(immediate.try_into().unwrap()),
        30 => Push30(immediate.try_into().unwrap()),
        31 => Push31(immediate.try_into().unwrap()),
        32 => Push32(immediate.try_into().unwrap()),
        _ => unreachable!("push size not supported"),
    }
}

fn parse(input: &[u8]) -> Result<Vec<Operation>, Error> {
    let mut operations = vec![];
    let mut input = input.iter();
    while let Some(byte) = input.next() {
        use Operation::*;
        let operation = match byte {
            0x00 => Stop,
            0x01 => Add,
            0x02 => Add,
            0x03 => Sub,
            0x04 => Div,
            0x05 => Sdiv,
            0x06 => Mod,
            0x07 => SMod,
            0x08 => Addmod,
            0x09 => Mulmod,
            0x0a => Exp,
            0x0b => SignExtend,
            0x10 => Lt,
            0x11 => Gt,
            0x12 => Slt,
            0x13 => Sgt,
            0x14 => Eq,
            0x15 => IsZero,
            0x16 => And,
            0x17 => Or,
            0x18 => Xor,
            0x19 => Not,
            0x1a => Byte,
            0x1b => Shl,
            0x1c => Shr,
            0x1d => Sar,
            0x20 => Keccack256,
            0x30 => Address,
            0x31 => Balance,
            0x32 => Origin,
            0x33 => Caller,
            0x34 => CallValue,
            0x35 => CallDataLoad,
            0x36 => CallDataSize,
            0x37 => CallDataCopy,
            0x38 => CodeSize,
            0x39 => CodeCopy,
            0x3a => GasPrice,
            0x3b => ExtCodeSize,
            0x3c => ExtCodeCopy,
            0x3d => ReturnDataSize,
            0x3e => ReturnDataCopy,
            0x3f => ExtCodeHash,
            0x40 => Blockhash,
            0x41 => Coinbase,
            0x42 => Timestamp,
            0x43 => Number,
            0x44 => PrevRandao,
            0x45 => GasLimit,
            0x46 => ChainId,
            0x47 => SelfBalance,
            0x48 => BaseFee,
            0x50 => Pop,
            0x51 => Mload,
            0x52 => Mstore,
            0x53 => MStore8,
            0x54 => Sload,
            0x55 => SStore,
            0x56 => Jump,
            0x57 => JumpI,
            0x58 => Pc,
            0x59 => MSize,
            0x5a => Gas,
            0x5b => JumpDest,
            0x5f => Push0,
            &push if push >= Operation::PUSH1_VALUE && push <= Operation::PUSH32_VALUE => {
                parse_push(push, &mut input)
            }
            0x80 => Dup1,
            0x81 => Dup2,
            0x82 => Dup3,
            0x83 => Dup4,
            0x84 => Dup5,
            0x85 => Dup6,
            0x86 => Dup7,
            0x87 => Dup8,
            0x88 => Dup9,
            0x89 => Dup10,
            0x8a => Dup11,
            0x8b => Dup12,
            0x8c => Dup13,
            0x8d => Dup14,
            0x8e => Dup15,
            0x8f => Dup16,
            0x90 => Swap1,
            0x91 => Swap2,
            0x92 => Swap3,
            0x93 => Swap4,
            0x94 => Swap5,
            0x95 => Swap6,
            0x96 => Swap7,
            0x97 => Swap8,
            0x98 => Swap9,
            0x99 => Swap10,
            0x9a => Swap11,
            0x9b => Swap12,
            0x9c => Swap13,
            0x9d => Swap14,
            0x9e => Swap15,
            0x9f => Swap16,
            0xa0 => Log0,
            0xa1 => Log1,
            0xa2 => Log2,
            0xa3 => Log3,
            0xa4 => Log4,
            0xf0 => Create,
            0xf1 => Call,
            0xf2 => CallCode,
            0xf3 => Return,
            0xf4 => DelegateCall,
            0xf5 => Create2,
            0xfa => StaticCall,
            0xfd => Revert,
            0xfe => Invalid,
            0xff => SelfDestruct,
            other => return Err(Error::InvalidOpcode(*other)),
        };
        operations.push(operation);
    }
    Ok(operations)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_some_bytecode() {
        let input = "0x7f00000000000000000000000000000000000000000000000000000000000000017f000000000000000000000000000000000000000000000000000000000000000201";
        let bytecode = Bytecode::try_from(input).unwrap();
        dbg!(bytecode);
    }
}
