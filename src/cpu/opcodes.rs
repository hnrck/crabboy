use std::collections::HashMap;
use crate::mmu::MMU;
use crate::cpu::registers::Registers;

pub type ExecuteFn = fn(&mut Registers, &mut MMU) -> bool;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Instruction {
    pub(crate) mnemonic: &'static str,
    pub(crate) execute: ExecuteFn,
    pub(crate) cycles_taken: u8,
    pub(crate) cycles_not_taken: Option<u8>,
    pub(crate) bytes: u8,
}

impl Instruction {
    pub(crate) fn new(mnemonic: &'static str, execute: ExecuteFn, cycles_taken: u8) -> Self {
        Instruction {
            mnemonic,
            execute,
            cycles_taken,
            cycles_not_taken: None,
            bytes: 1,
        }
    }

    fn with_bytes(self, bytes: u8) -> Self {
        Instruction {
            bytes,
            ..self
        }
    }

    fn with_cycles_not_taken(self, cycles_not_taken: u8) -> Self {
        Instruction {
            cycles_not_taken: Some(cycles_not_taken),
            ..self
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Opcode {
    // 0x0_
    Nop,
}

impl Opcode {
    pub(crate) fn from_byte(byte: u8) -> Opcode {
        match byte {
            0x00 => Opcode::Nop,
            _ => panic!("Unknown opcode: {:X}", byte),
        }
    }
}


pub(crate) fn initialize_opcodes_instructions_map() -> HashMap<Opcode, Instruction> {
    let mut opcode_instructions = HashMap::new();

    opcode_instructions.insert(
        Opcode::Nop, Instruction::new(
            "NOP", |_registers, _memory| { false }, 1,
        ),
    );

    opcode_instructions
}

