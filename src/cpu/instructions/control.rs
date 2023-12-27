use std::collections::HashMap;

use crate::cpu::instructions::{Cycles, Instruction};
use crate::cpu::registers::CpuState;

pub(super) fn instructions_map_control_commands(instructions_map: &mut HashMap<u8, Instruction>) -> () {
    instructions_map.insert(
        0x00, Instruction::new(
            "NOP", |_registers, _memory| { (true, true) }, Cycles::new(1), 1,
        ),
    );

    instructions_map.insert(
        0x10, Instruction::new(
            "STOP", |registers, _memory| {
                registers.cpu_state = CpuState::Stopped;
                (true, true)
            }, Cycles::new(2), 1,
        ),
    );

    instructions_map.insert(
        0x76, Instruction::new(
            "HALT", |registers, _memory| {
                registers.cpu_state = CpuState::Halted;
                (true, true)
            }, Cycles::new(1), 1,
        ),
    );

    // TODO Implementation for handling extended instruction set
    instructions_map.insert(
        0xCB, Instruction::new(
            "PREFIX", |_registers, _memory| { (true, true) }, Cycles::new(1), 1,
        ),
    );

    instructions_map.insert(
        0xF3, Instruction::new(
            "DI", |registers, _memory| {
                registers.interrupts_enabled = false;
                (true, true)
            }, Cycles::new(1), 1,
        ),
    );

    instructions_map.insert(
        0xFB, Instruction::new(
            "EI", |registers, _memory| {
                registers.interrupts_enabled = true;
                (true, true)
            }, Cycles::new(1), 1,
        ),
    );
}