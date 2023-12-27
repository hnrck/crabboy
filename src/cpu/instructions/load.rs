use std::collections::HashMap;

use crate::cpu::instructions::Instruction;
use crate::cpu::registers::Registers;
use crate::mmu::MMU;

pub(super) fn instructions_map_load_instructions(instructions_map: &mut HashMap<u8, Instruction>) -> () {
    instructions_map_8_bit_load_instructions(instructions_map);
    instructions_map_16_bit_load_instructions(instructions_map);
}

fn instructions_map_8_bit_load_instructions(_instructions_map: &mut HashMap<u8, Instruction>) -> () {
    fn ld(from: &u8, to: &mut u8) { *to = *from; }

    // TODO(henrick) Implementation
}

fn instructions_map_16_bit_load_instructions(_instructions_map: &mut HashMap<u8, Instruction>) -> () {
    fn ld(from: &u16, to: &mut u16) { *to = *from; }

    fn pop(registers: &mut Registers, memory: &MMU, to: &mut u16) {
        *to = memory.read_word(registers.sp);
        registers.sp += 2;
    }

    fn push(registers: &mut Registers, memory: &mut MMU, from: &u16) {
        memory.write_word(registers.sp, *from);
        registers.sp += 2;
    }

    // TODO(henrick) Implementation
}