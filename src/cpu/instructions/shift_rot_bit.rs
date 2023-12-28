use crate::cpu::instructions::{Cycles, Instruction, InstructionsMap};

pub(super) fn instructions_map_shift_rot_bit_instructions(instructions_map: &mut InstructionsMap, prefix_cb: &mut InstructionsMap) -> () {
    instructions_map_8_bit_shift_rot_bit_instructions(instructions_map);
    instructions_map_8_bit_prefix_cb_instructions(prefix_cb)
}

fn instructions_map_8_bit_shift_rot_bit_instructions(instructions_map: &mut InstructionsMap) -> () {
    instructions_map.insert(
        0x07, Instruction::new(
            "RLCA", |registers, _memory| {
                let carry = (registers.a & 0x80) != 0;
                registers.a = (registers.a << 1) | (carry as u8);
                registers.f.z = false;
                registers.f.n = false;
                registers.f.h = false;
                registers.f.c = carry;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x17, Instruction::new(
            "RLA", |registers, _memory| {
                let carry = (registers.a & 0x80) != 0;
                registers.a = (registers.a << 1) | (registers.f.c as u8);
                registers.f.z = false;
                registers.f.n = false;
                registers.f.h = false;
                registers.f.c = carry;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x0F, Instruction::new(
            "RRCA", |registers, _memory| {
                let carry = registers.a & 0x01 != 0;
                registers.a = ((carry as u8) << 7) | (registers.a >> 1);
                registers.f.z = false;
                registers.f.n = false;
                registers.f.h = false;
                registers.f.c = carry;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x1F, Instruction::new(
            "RRA", |registers, _memory| {
                let carry = registers.a & 0x01 != 0;
                registers.a = ((registers.f.c as u8) << 7) | (registers.a >> 1);
                registers.f.z = false;
                registers.f.n = false;
                registers.f.h = false;
                registers.f.c = carry;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );
}

fn instructions_map_8_bit_prefix_cb_instructions(_prefix_cb: &mut InstructionsMap) -> () {
    // TODO(henrick) Impl
}
