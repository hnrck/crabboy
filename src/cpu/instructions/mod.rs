use std::collections::HashMap;

use crate::cpu::registers::Registers;
use crate::mmu::MMU;

mod control;
mod jump;
mod load;
mod arithmetic_logical;
mod shift_rot_bit;

// return (true, _) if pc have to be updated, i.e. pc += instruction.bytes
// return (false, _) if pc should not be updated, i.e. pc += 0
// return (_, true) if action was taken, i.e. cycles += instruction.cycles.taken
// return (_, false) if action was not taken, i.e. cycles += instruction.cycles.not_taken
pub type ExecuteFn = fn(&mut Registers, &mut MMU) -> (bool, bool);

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Cycles {
    pub(crate) taken: u8,
    pub(crate) not_taken: u8,
}

impl Cycles {
    pub(crate) fn new(taken: u8) -> Self {
        Cycles {
            taken,
            not_taken: taken,
        }
    }

    fn with_not_taken(self, not_taken: u8) -> Self {
        Cycles {
            not_taken,
            ..self
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Instruction {
    pub(crate) mnemonic: &'static str,
    pub(crate) execute: ExecuteFn,
    pub(crate) cycles: Cycles,
    pub(crate) bytes: u8,
}

impl Instruction {
    pub(crate) fn new(mnemonic: &'static str, execute: ExecuteFn, cycles: Cycles, bytes: u8) -> Self {
        Instruction {
            mnemonic,
            execute,
            cycles,
            bytes,
        }
    }
}

pub(crate) fn initialize_instructions_map() -> HashMap<u8, Instruction> {
    let mut instructions_map = HashMap::new();

    control::instructions_map_control_commands(&mut instructions_map);
    jump::instructions_map_jump_commands(&mut instructions_map);
    load::instructions_map_load_instructions(&mut instructions_map);
    arithmetic_logical::instructions_map_arithmetic_logical_instructions(&mut instructions_map);
    shift_rot_bit::instructions_map_shift_rot_bit_instructions(&mut instructions_map);

    instructions_map
}