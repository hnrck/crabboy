use std::collections::HashMap;
use crate::mmu::MMU;
use crate::cpu::registers::Registers;

pub type ExecuteFn = fn(&mut Registers, &mut MMU) -> bool;

#[derive(Debug, PartialEq, Eq, Clone)]
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

fn instructions_map_control_commands(instructions_map: &mut HashMap<u8, Instruction>) -> () {
    instructions_map.insert(
        0x00, Instruction::new(
            "NOP", |_registers, _memory| { true }, 1,
        ),
    );
}

fn instructions_map_jump_commands(instructions_map: &mut HashMap<u8, Instruction>) -> () {
    fn jp(registers: &mut Registers, address: u16) { registers.pc = address }

    instructions_map.insert(
        0xc3, Instruction::new(
            "JP A16", |registers, memory| {
                jp(registers, memory.read_word(registers.pc + 1));
                true
            }, 16,
        ).with_bytes(0),
    );
}

pub(crate) fn initialize_instructions_map() -> HashMap<u8, Instruction> {
    let mut instructions_map = HashMap::new();

    instructions_map_control_commands(&mut instructions_map);
    instructions_map_jump_commands(&mut instructions_map);

    instructions_map
}

