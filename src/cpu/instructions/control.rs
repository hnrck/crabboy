use crate::cpu::instructions::{Cycles, ExecutionResult, Instruction, InstructionsMap};
use crate::cpu::registers::CpuState;

pub(super) fn instructions_map_control_commands(instructions_map: &mut InstructionsMap) -> () {
    instructions_map.insert(
        0x00, Instruction::new(
            "NOP", |_registers, _memory| { ExecutionResult::default() }, Cycles::new(1), 1,
        ),
    );

    instructions_map.insert(
        0x10, Instruction::new(
            "STOP", |registers, _memory| {
                registers.cpu_state = CpuState::Stopped;
                ExecutionResult::default()
            }, Cycles::new(2), 1,
        ),
    );

    instructions_map.insert(
        0x76, Instruction::new(
            "HALT", |registers, _memory| {
                registers.cpu_state = CpuState::Halted;
                ExecutionResult::default()
            }, Cycles::new(1), 1,
        ),
    );

    instructions_map.insert(
        0xCB, Instruction::new(
            "PREFIX", |_registers, _memory| { ExecutionResult::default() }, Cycles::new(1), 1,
        ),
    );

    instructions_map.insert(
        0xF3, Instruction::new(
            "DI", |registers, _memory| {
                registers.interrupts_enabled = false;
                ExecutionResult::default()
            }, Cycles::new(1), 1,
        ),
    );

    instructions_map.insert(
        0xFB, Instruction::new(
            "EI", |registers, _memory| {
                registers.interrupts_enabled = true;
                ExecutionResult::default()
            }, Cycles::new(1), 1,
        ),
    );
}