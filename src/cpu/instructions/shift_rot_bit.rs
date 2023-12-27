use std::collections::HashMap;

use crate::cpu::instructions::Instruction;

pub(super) fn instructions_map_shift_rot_bit_instructions(instructions_map: &mut HashMap<u8, Instruction>) -> () {
    instructions_map_8_bit_shift_rot_bit_instructions(instructions_map)
}

fn instructions_map_8_bit_shift_rot_bit_instructions(_instructions_map: &mut HashMap<u8, Instruction>) -> () {
    // TODO(henrick) Implementation
}