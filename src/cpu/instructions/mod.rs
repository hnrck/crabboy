use std::collections::HashMap;

use crate::cpu::registers::Registers;
use crate::mmu::MMU;

mod control;
mod jump;
mod load;
mod arithmetic_logical;
mod shift_rot_bit;

pub struct ExecutionResult {
    pub update_pc: bool,
    pub action_taken: bool,
}

impl Default for ExecutionResult {
    fn default() -> Self {
        ExecutionResult {
            update_pc: true,
            action_taken: true,
        }
    }
}

impl ExecutionResult {
    fn without_pc_update(self) -> Self {
        ExecutionResult {
            update_pc: false,
            ..self
        }
    }

    fn without_action(self) -> Self {
        ExecutionResult {
            action_taken: false,
            ..self
        }
    }
}

pub type ExecuteFn = fn(&mut Registers, &mut MMU) -> ExecutionResult;

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

pub(super) type InstructionsMap = HashMap<u8, Instruction>;

pub(super) struct InstructionsMapsManager {
    default_map: InstructionsMap,
    prefix_cb_map: InstructionsMap,
    state: InstructionsMapState,
}

#[derive(PartialEq)]
enum InstructionsMapState {
    Default,
    PrefixCB,
}

impl InstructionsMapsManager {
    pub(super) fn new() -> Self {
        let mut default_map = HashMap::new(); // Initialize with your default instructions map
        let mut prefix_cb_map = HashMap::new(); // Initialize with your prefix CB instructions map

        control::instructions_map_control_commands(&mut default_map);
        jump::instructions_map_jump_commands(&mut default_map);
        load::instructions_map_load_instructions(&mut default_map);
        arithmetic_logical::instructions_map_arithmetic_logical_instructions(&mut default_map);
        shift_rot_bit::instructions_map_shift_rot_bit_instructions(&mut default_map, &mut prefix_cb_map);

        InstructionsMapsManager {
            default_map,
            prefix_cb_map,
            state: InstructionsMapState::Default,
        }
    }

    pub(super) fn get_instruction_map(&mut self) -> &mut InstructionsMap {
        match self.state {
            InstructionsMapState::Default => &mut self.default_map,
            InstructionsMapState::PrefixCB => &mut self.prefix_cb_map,
        }
    }

    pub(super) fn is_default_map(&self) -> bool { self.state == InstructionsMapState::Default }

    pub(super) fn set_prefix_cb_state(&mut self) { self.state = InstructionsMapState::PrefixCB }

    pub(super) fn reset_state(&mut self) { self.state = InstructionsMapState::Default }
}