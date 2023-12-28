use crate::cpu::instructions::{Cycles, ExecuteFn, Instruction, InstructionsMap};
use crate::cpu::registers::Flags;

pub(super) fn instructions_map_shift_rot_bit_instructions(instructions_map: &mut InstructionsMap, prefix_cb_map: &mut InstructionsMap) -> () {
    instructions_map_8_bit_shift_rot_bit_instructions(instructions_map);
    instructions_map_8_bitprefix_cb_map_instructions(prefix_cb_map)
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

fn build_instruction(mnemonic: &'static str, execute: ExecuteFn, double_cycles: bool) -> Instruction {
    Instruction::new(mnemonic, execute, Cycles::new(match double_cycles {
        false => 8,
        true => 16
    }), 2)
}


fn instructions_map_8_bitprefix_cb_map_instructions(prefix_cb_map: &mut InstructionsMap) -> () {
    instructions_map_8_bitprefix_cb_map_rlc_instructions(prefix_cb_map);
    instructions_map_8_bitprefix_cb_map_rrc_instructions(prefix_cb_map);
    instructions_map_8_bitprefix_cb_map_rl_instructions(prefix_cb_map);
    instructions_map_8_bitprefix_cb_map_rr_instructions(prefix_cb_map);
    instructions_map_8_bitprefix_cb_map_sla_instructions(prefix_cb_map);
    instructions_map_8_bitprefix_cb_map_sra_instructions(prefix_cb_map);
    instructions_map_8_bitprefix_cb_map_swap_instructions(prefix_cb_map);
    instructions_map_8_bitprefix_cb_map_srl_instructions(prefix_cb_map);
    instructions_map_8_bitprefix_cb_map_bit_instructions(prefix_cb_map);
    instructions_map_8_bitprefix_cb_map_res_instructions(prefix_cb_map);
    instructions_map_8_bitprefix_cb_map_set_instructions(prefix_cb_map)
}

fn instructions_map_8_bitprefix_cb_map_rlc_instructions(prefix_cb_map: &mut InstructionsMap) -> () {
    fn rlc_operator(data: u8, flags: &mut Flags) -> u8 {
        let carry = data >> 7;
        let result = (data << 1) | carry;

        flags.z = result == 0;
        flags.n = false;
        flags.h = false;
        flags.c = carry != 0;

        result
    }

    prefix_cb_map.insert(0x00, build_instruction("RLC B", |registers, _| {
        registers.b = rlc_operator(registers.b, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x01, build_instruction("RLC C", |registers, _| {
        registers.c = rlc_operator(registers.c, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x02, build_instruction("RLC D", |registers, _| {
        registers.d = rlc_operator(registers.d, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x03, build_instruction("RLC E", |registers, _| {
        registers.e = rlc_operator(registers.e, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x04, build_instruction("RLC H", |registers, _| {
        registers.h = rlc_operator(registers.h, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x05, build_instruction("RLC L", |registers, _| {
        registers.l = rlc_operator(registers.l, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x06, build_instruction("RLC (HL)", |registers, memory| {
        memory.write_byte(registers.get_hl(), rlc_operator(memory.read_byte(registers.get_hl()), &mut registers.f));
        (true, true)
    }, true));
    prefix_cb_map.insert(0x07, build_instruction("RLC A", |registers, _| {
        registers.a = rlc_operator(registers.a, &mut registers.f);
        (true, true)
    }, false));
}

fn instructions_map_8_bitprefix_cb_map_rrc_instructions(prefix_cb_map: &mut InstructionsMap) -> () {
    fn rrc_operator(data: u8, flags: &mut Flags) -> u8 {
        let carry = data & 1;
        let result = (data >> 1) | (carry << 7);

        flags.z = result == 0;
        flags.n = false;
        flags.h = false;
        flags.c = carry != 0;

        result
    }

    prefix_cb_map.insert(0x08, build_instruction("RRC B", |registers, _| {
        registers.b = rrc_operator(registers.b, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x09, build_instruction("RRC C", |registers, _| {
        registers.c = rrc_operator(registers.c, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x0A, build_instruction("RRC D", |registers, _| {
        registers.d = rrc_operator(registers.d, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x0B, build_instruction("RRC E", |registers, _| {
        registers.e = rrc_operator(registers.e, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x0C, build_instruction("RRC H", |registers, _| {
        registers.h = rrc_operator(registers.h, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x0D, build_instruction("RRC L", |registers, _| {
        registers.l = rrc_operator(registers.l, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x0E, build_instruction("RRC (HL)", |registers, memory| {
        memory.write_byte(registers.get_hl(), rrc_operator(memory.read_byte(registers.get_hl()), &mut registers.f));
        (true, true)
    }, true));
    prefix_cb_map.insert(0x0F, build_instruction("RRC A", |registers, _| {
        registers.a = rrc_operator(registers.a, &mut registers.f);
        (true, true)
    }, false));
}

fn instructions_map_8_bitprefix_cb_map_rl_instructions(prefix_cb_map: &mut InstructionsMap) -> () {
    fn rl_operator(data: u8, flags: &mut Flags) -> u8 {
        let carry = flags.c as u8;
        let result = (data << 1) | carry;

        flags.z = result == 0;
        flags.n = false;
        flags.h = false;
        flags.c = (data & 0x80) != 0;

        result
    }

    prefix_cb_map.insert(0x10, build_instruction("RL B", |registers, _| {
        registers.b = rl_operator(registers.b, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x11, build_instruction("RL C", |registers, _| {
        registers.c = rl_operator(registers.c, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x12, build_instruction("RL D", |registers, _| {
        registers.d = rl_operator(registers.d, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x13, build_instruction("RL E", |registers, _| {
        registers.e = rl_operator(registers.e, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x14, build_instruction("RL H", |registers, _| {
        registers.h = rl_operator(registers.h, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x15, build_instruction("RL L", |registers, _| {
        registers.l = rl_operator(registers.l, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x16, build_instruction("RL (HL)", |registers, memory| {
        memory.write_byte(registers.get_hl(), rl_operator(memory.read_byte(registers.get_hl()), &mut registers.f));
        (true, true)
    }, true));
    prefix_cb_map.insert(0x17, build_instruction("RL A", |registers, _| {
        registers.a = rl_operator(registers.a, &mut registers.f);
        (true, true)
    }, false));
}

fn instructions_map_8_bitprefix_cb_map_rr_instructions(prefix_cb_map: &mut InstructionsMap) -> () {
    fn rr_operator(data: u8, flags: &mut Flags) -> u8 {
        let carry = flags.c as u8;
        let result = (data >> 1) | (carry << 7);

        flags.z = result == 0;
        flags.n = false;
        flags.h = false;
        flags.c = (data & 1) != 0;

        result
    }

    prefix_cb_map.insert(0x18, build_instruction("RR B", |registers, _| {
        registers.b = rr_operator(registers.b, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x19, build_instruction("RR C", |registers, _| {
        registers.c = rr_operator(registers.c, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x1A, build_instruction("RR D", |registers, _| {
        registers.d = rr_operator(registers.d, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x1B, build_instruction("RR E", |registers, _| {
        registers.e = rr_operator(registers.e, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x1C, build_instruction("RR H", |registers, _| {
        registers.h = rr_operator(registers.h, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x1D, build_instruction("RR L", |registers, _| {
        registers.l = rr_operator(registers.l, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x1E, build_instruction("RR (HL)", |registers, memory| {
        memory.write_byte(registers.get_hl(), rr_operator(memory.read_byte(registers.get_hl()), &mut registers.f));
        (true, true)
    }, true));
    prefix_cb_map.insert(0x1F, build_instruction("RR A", |registers, _| {
        registers.a = rr_operator(registers.a, &mut registers.f);
        (true, true)
    }, false));
}

fn instructions_map_8_bitprefix_cb_map_sla_instructions(prefix_cb_map: &mut InstructionsMap) -> () {
    fn sla_operator(data: u8, flags: &mut Flags) -> u8 {
        let carry = data >> 7;
        let result = data << 1;

        flags.z = result == 0;
        flags.n = false;
        flags.h = false;
        flags.c = carry != 0;

        result
    }

    prefix_cb_map.insert(0x20, build_instruction("SLA B", |registers, _| {
        registers.b = sla_operator(registers.b, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x21, build_instruction("SLA C", |registers, _| {
        registers.c = sla_operator(registers.c, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x22, build_instruction("SLA D", |registers, _| {
        registers.d = sla_operator(registers.d, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x23, build_instruction("SLA E", |registers, _| {
        registers.e = sla_operator(registers.e, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x24, build_instruction("SLA H", |registers, _| {
        registers.h = sla_operator(registers.h, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x25, build_instruction("SLA L", |registers, _| {
        registers.l = sla_operator(registers.l, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x26, build_instruction("SLA (HL)", |registers, memory| {
        memory.write_byte(registers.get_hl(), sla_operator(memory.read_byte(registers.get_hl()), &mut registers.f));
        (true, true)
    }, true));
    prefix_cb_map.insert(0x27, build_instruction("SLA A", |registers, _| {
        registers.a = sla_operator(registers.a, &mut registers.f);
        (true, true)
    }, false));
}

fn instructions_map_8_bitprefix_cb_map_sra_instructions(prefix_cb_map: &mut InstructionsMap) -> () {
    fn sra_operator(data: u8, flags: &mut Flags) -> u8 {
        let carry = data & 1;
        let result = (data >> 1) | (data & 0x80);

        flags.z = result == 0;
        flags.n = false;
        flags.h = false;
        flags.c = carry != 0;

        result
    }

    prefix_cb_map.insert(0x28, build_instruction("SRA B", |registers, _| {
        registers.b = sra_operator(registers.b, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x29, build_instruction("SRA C", |registers, _| {
        registers.c = sra_operator(registers.c, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x2A, build_instruction("SRA D", |registers, _| {
        registers.d = sra_operator(registers.d, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x2B, build_instruction("SRA E", |registers, _| {
        registers.e = sra_operator(registers.e, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x2C, build_instruction("SRA H", |registers, _| {
        registers.h = sra_operator(registers.h, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x2D, build_instruction("SRA L", |registers, _| {
        registers.l = sra_operator(registers.l, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x2E, build_instruction("SRA (HL)", |registers, memory| {
        memory.write_byte(registers.get_hl(), sra_operator(memory.read_byte(registers.get_hl()), &mut registers.f));
        (true, true)
    }, true));
    prefix_cb_map.insert(0x2F, build_instruction("SRA A", |registers, _| {
        registers.a = sra_operator(registers.a, &mut registers.f);
        (true, true)
    }, false));
}

fn instructions_map_8_bitprefix_cb_map_swap_instructions(prefix_cb_map: &mut InstructionsMap) -> () {
    fn swap_operator(data: u8, flags: &mut Flags) -> u8 {
        let result = (data << 4) | (data >> 4);

        flags.z = result == 0;
        flags.n = false;
        flags.h = false;
        flags.c = false;

        result
    }

    prefix_cb_map.insert(0x30, build_instruction("SWAP B", |registers, _| {
        registers.b = swap_operator(registers.b, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x31, build_instruction("SWAP C", |registers, _| {
        registers.c = swap_operator(registers.c, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x32, build_instruction("SWAP D", |registers, _| {
        registers.d = swap_operator(registers.d, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x33, build_instruction("SWAP E", |registers, _| {
        registers.e = swap_operator(registers.e, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x34, build_instruction("SWAP H", |registers, _| {
        registers.h = swap_operator(registers.h, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x35, build_instruction("SWAP L", |registers, _| {
        registers.l = swap_operator(registers.l, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x36, build_instruction("SWAP (HL)", |registers, memory| {
        memory.write_byte(registers.get_hl(), swap_operator(memory.read_byte(registers.get_hl()), &mut registers.f));
        (true, true)
    }, true));
    prefix_cb_map.insert(0x37, build_instruction("SWAP A", |registers, _| {
        registers.a = swap_operator(registers.a, &mut registers.f);
        (true, true)
    }, false));
}

fn instructions_map_8_bitprefix_cb_map_srl_instructions(prefix_cb_map: &mut InstructionsMap) -> () {
    fn srl_operator(data: u8, flags: &mut Flags) -> u8 {
        let carry = data & 1;
        let result = data >> 1;

        flags.z = result == 0;
        flags.n = false;
        flags.h = false;
        flags.c = carry != 0;

        result
    }

    prefix_cb_map.insert(0x38, build_instruction("SRL B", |registers, _| {
        registers.b = srl_operator(registers.b, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x39, build_instruction("SRL C", |registers, _| {
        registers.c = srl_operator(registers.c, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x3A, build_instruction("SRL D", |registers, _| {
        registers.d = srl_operator(registers.d, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x3B, build_instruction("SRL E", |registers, _| {
        registers.e = srl_operator(registers.e, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x3C, build_instruction("SRL H", |registers, _| {
        registers.h = srl_operator(registers.h, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x3D, build_instruction("SRL L", |registers, _| {
        registers.l = srl_operator(registers.l, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x3E, build_instruction("SRL (HL)", |registers, memory| {
        memory.write_byte(registers.get_hl(), srl_operator(memory.read_byte(registers.get_hl()), &mut registers.f));
        (true, true)
    }, true));
    prefix_cb_map.insert(0x3F, build_instruction("SRL A", |registers, _| {
        registers.a = srl_operator(registers.a, &mut registers.f);
        (true, true)
    }, false));
}

fn instructions_map_8_bitprefix_cb_map_bit_instructions(prefix_cb_map: &mut InstructionsMap) -> () {
    fn bit_operator(bit: usize, data: u8, flags: &mut Flags) -> (bool, bool) {
        let mask = 1 << bit;
        flags.z = data & mask == 0;
        flags.n = false;
        flags.h = true;
        (true, true)
    }

    prefix_cb_map.insert(0x40, build_instruction("BIT 0,B", |registers, _| { bit_operator(0, registers.b, &mut registers.f) }, false));
    prefix_cb_map.insert(0x41, build_instruction("BIT 0,C", |registers, _| { bit_operator(0, registers.c, &mut registers.f) }, false));
    prefix_cb_map.insert(0x42, build_instruction("BIT 0,D", |registers, _| { bit_operator(0, registers.d, &mut registers.f) }, false));
    prefix_cb_map.insert(0x43, build_instruction("BIT 0,E", |registers, _| { bit_operator(0, registers.e, &mut registers.f) }, false));
    prefix_cb_map.insert(0x44, build_instruction("BIT 0,H", |registers, _| { bit_operator(0, registers.h, &mut registers.f) }, false));
    prefix_cb_map.insert(0x45, build_instruction("BIT 0,L", |registers, _| { bit_operator(0, registers.l, &mut registers.f) }, false));
    prefix_cb_map.insert(0x46, build_instruction("BIT 0,(HL)", |registers, memory| { bit_operator(0, memory.read_byte(registers.get_hl()), &mut registers.f) }, true));
    prefix_cb_map.insert(0x47, build_instruction("BIT 0,A", |registers, _| { bit_operator(0, registers.a, &mut registers.f) }, false));
    prefix_cb_map.insert(0x48, build_instruction("BIT 1,B", |registers, _| { bit_operator(1, registers.b, &mut registers.f) }, false));
    prefix_cb_map.insert(0x49, build_instruction("BIT 1,C", |registers, _| { bit_operator(1, registers.c, &mut registers.f) }, false));
    prefix_cb_map.insert(0x4A, build_instruction("BIT 1,D", |registers, _| { bit_operator(1, registers.d, &mut registers.f) }, false));
    prefix_cb_map.insert(0x4B, build_instruction("BIT 1,E", |registers, _| { bit_operator(1, registers.e, &mut registers.f) }, false));
    prefix_cb_map.insert(0x4C, build_instruction("BIT 1,H", |registers, _| { bit_operator(1, registers.h, &mut registers.f) }, false));
    prefix_cb_map.insert(0x4D, build_instruction("BIT 1,L", |registers, _| { bit_operator(1, registers.l, &mut registers.f) }, false));
    prefix_cb_map.insert(0x4E, build_instruction("BIT 1,(HL)", |registers, memory| { bit_operator(1, memory.read_byte(registers.get_hl()), &mut registers.f) }, true));
    prefix_cb_map.insert(0x4F, build_instruction("BIT 1,A", |registers, _| { bit_operator(1, registers.a, &mut registers.f) }, false));
    prefix_cb_map.insert(0x50, build_instruction("BIT 2,B", |registers, _| { bit_operator(2, registers.b, &mut registers.f) }, false));
    prefix_cb_map.insert(0x51, build_instruction("BIT 2,C", |registers, _| { bit_operator(2, registers.c, &mut registers.f) }, false));
    prefix_cb_map.insert(0x52, build_instruction("BIT 2,D", |registers, _| { bit_operator(2, registers.d, &mut registers.f) }, false));
    prefix_cb_map.insert(0x53, build_instruction("BIT 2,E", |registers, _| { bit_operator(2, registers.e, &mut registers.f) }, false));
    prefix_cb_map.insert(0x54, build_instruction("BIT 2,H", |registers, _| { bit_operator(2, registers.h, &mut registers.f) }, false));
    prefix_cb_map.insert(0x55, build_instruction("BIT 2,L", |registers, _| { bit_operator(2, registers.l, &mut registers.f) }, false));
    prefix_cb_map.insert(0x56, build_instruction("BIT 2,(HL)", |registers, memory| { bit_operator(2, memory.read_byte(registers.get_hl()), &mut registers.f) }, true));
    prefix_cb_map.insert(0x57, build_instruction("BIT 2,A", |registers, _| { bit_operator(2, registers.a, &mut registers.f) }, false));
    prefix_cb_map.insert(0x58, build_instruction("BIT 3,B", |registers, _| { bit_operator(3, registers.b, &mut registers.f) }, false));
    prefix_cb_map.insert(0x59, build_instruction("BIT 3,C", |registers, _| { bit_operator(3, registers.c, &mut registers.f) }, false));
    prefix_cb_map.insert(0x5A, build_instruction("BIT 3,D", |registers, _| { bit_operator(3, registers.d, &mut registers.f) }, false));
    prefix_cb_map.insert(0x5B, build_instruction("BIT 3,E", |registers, _| { bit_operator(3, registers.e, &mut registers.f) }, false));
    prefix_cb_map.insert(0x5C, build_instruction("BIT 3,H", |registers, _| { bit_operator(3, registers.h, &mut registers.f) }, false));
    prefix_cb_map.insert(0x5D, build_instruction("BIT 3,L", |registers, _| { bit_operator(3, registers.l, &mut registers.f) }, false));
    prefix_cb_map.insert(0x5E, build_instruction("BIT 3,(HL)", |registers, memory| { bit_operator(3, memory.read_byte(registers.get_hl()), &mut registers.f) }, true));
    prefix_cb_map.insert(0x5F, build_instruction("BIT 3,A", |registers, _| { bit_operator(3, registers.a, &mut registers.f) }, false));
    prefix_cb_map.insert(0x60, build_instruction("BIT 4,B", |registers, _| { bit_operator(4, registers.b, &mut registers.f) }, false));
    prefix_cb_map.insert(0x61, build_instruction("BIT 4,C", |registers, _| { bit_operator(4, registers.c, &mut registers.f) }, false));
    prefix_cb_map.insert(0x62, build_instruction("BIT 4,D", |registers, _| { bit_operator(4, registers.d, &mut registers.f) }, false));
    prefix_cb_map.insert(0x63, build_instruction("BIT 4,E", |registers, _| { bit_operator(4, registers.e, &mut registers.f) }, false));
    prefix_cb_map.insert(0x64, build_instruction("BIT 4,H", |registers, _| { bit_operator(4, registers.h, &mut registers.f) }, false));
    prefix_cb_map.insert(0x65, build_instruction("BIT 4,L", |registers, _| { bit_operator(4, registers.l, &mut registers.f) }, false));
    prefix_cb_map.insert(0x66, build_instruction("BIT 4,(HL)", |registers, memory| { bit_operator(4, memory.read_byte(registers.get_hl()), &mut registers.f) }, true));
    prefix_cb_map.insert(0x67, build_instruction("BIT 4,A", |registers, _| { bit_operator(4, registers.a, &mut registers.f) }, false));
    prefix_cb_map.insert(0x68, build_instruction("BIT 5,B", |registers, _| { bit_operator(5, registers.b, &mut registers.f) }, false));
    prefix_cb_map.insert(0x69, build_instruction("BIT 5,C", |registers, _| { bit_operator(5, registers.c, &mut registers.f) }, false));
    prefix_cb_map.insert(0x6A, build_instruction("BIT 5,D", |registers, _| { bit_operator(5, registers.d, &mut registers.f) }, false));
    prefix_cb_map.insert(0x6B, build_instruction("BIT 5,E", |registers, _| { bit_operator(5, registers.e, &mut registers.f) }, false));
    prefix_cb_map.insert(0x6C, build_instruction("BIT 5,H", |registers, _| { bit_operator(5, registers.h, &mut registers.f) }, false));
    prefix_cb_map.insert(0x6D, build_instruction("BIT 5,L", |registers, _| { bit_operator(5, registers.l, &mut registers.f) }, false));
    prefix_cb_map.insert(0x6E, build_instruction("BIT 5,(HL)", |registers, memory| { bit_operator(5, memory.read_byte(registers.get_hl()), &mut registers.f) }, true));
    prefix_cb_map.insert(0x6F, build_instruction("BIT 5,A", |registers, _| { bit_operator(5, registers.a, &mut registers.f) }, false));
    prefix_cb_map.insert(0x70, build_instruction("BIT 6,B", |registers, _| { bit_operator(6, registers.b, &mut registers.f) }, false));
    prefix_cb_map.insert(0x71, build_instruction("BIT 6,C", |registers, _| { bit_operator(6, registers.c, &mut registers.f) }, false));
    prefix_cb_map.insert(0x72, build_instruction("BIT 6,D", |registers, _| { bit_operator(6, registers.d, &mut registers.f) }, false));
    prefix_cb_map.insert(0x73, build_instruction("BIT 6,E", |registers, _| { bit_operator(6, registers.e, &mut registers.f) }, false));
    prefix_cb_map.insert(0x74, build_instruction("BIT 6,H", |registers, _| { bit_operator(6, registers.h, &mut registers.f) }, false));
    prefix_cb_map.insert(0x75, build_instruction("BIT 6,L", |registers, _| { bit_operator(6, registers.l, &mut registers.f) }, false));
    prefix_cb_map.insert(0x76, build_instruction("BIT 6,(HL)", |registers, memory| { bit_operator(6, memory.read_byte(registers.get_hl()), &mut registers.f) }, true));
    prefix_cb_map.insert(0x77, build_instruction("BIT 6,A", |registers, _| { bit_operator(6, registers.a, &mut registers.f) }, false));
    prefix_cb_map.insert(0x78, build_instruction("BIT 7,B", |registers, _| { bit_operator(7, registers.b, &mut registers.f) }, false));
    prefix_cb_map.insert(0x79, build_instruction("BIT 7,C", |registers, _| { bit_operator(7, registers.c, &mut registers.f) }, false));
    prefix_cb_map.insert(0x7A, build_instruction("BIT 7,D", |registers, _| { bit_operator(7, registers.d, &mut registers.f) }, false));
    prefix_cb_map.insert(0x7B, build_instruction("BIT 7,E", |registers, _| { bit_operator(7, registers.e, &mut registers.f) }, false));
    prefix_cb_map.insert(0x7C, build_instruction("BIT 7,H", |registers, _| { bit_operator(7, registers.h, &mut registers.f) }, false));
    prefix_cb_map.insert(0x7D, build_instruction("BIT 7,L", |registers, _| { bit_operator(7, registers.l, &mut registers.f) }, false));
    prefix_cb_map.insert(0x7E, build_instruction("BIT 7,(HL)", |registers, memory| { bit_operator(7, memory.read_byte(registers.get_hl()), &mut registers.f) }, true));
    prefix_cb_map.insert(0x7F, build_instruction("BIT 7,A", |registers, _| { bit_operator(7, registers.a, &mut registers.f) }, false));
}

fn instructions_map_8_bitprefix_cb_map_res_instructions(prefix_cb_map: &mut InstructionsMap) -> () {
    fn res_operator(bit: usize, data: u8, _: &mut Flags) -> u8 { data & !(1 << bit) }

    prefix_cb_map.insert(0x80, build_instruction("RES 0,B", |registers, _| {
        registers.b = res_operator(0, registers.b, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x81, build_instruction("RES 0,C", |registers, _| {
        registers.c = res_operator(0, registers.c, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x82, build_instruction("RES 0,D", |registers, _| {
        registers.d = res_operator(0, registers.d, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x83, build_instruction("RES 0,E", |registers, _| {
        registers.e = res_operator(0, registers.e, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x84, build_instruction("RES 0,H", |registers, _| {
        registers.h = res_operator(0, registers.h, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x85, build_instruction("RES 0,L", |registers, _| {
        registers.l = res_operator(0, registers.l, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x86, build_instruction("RES 0,(HL)", |registers, memory| {
        memory.write_byte(registers.get_hl(), res_operator(0, memory.read_byte(registers.get_hl()), &mut registers.f));
        (true, true)
    }, true));
    prefix_cb_map.insert(0x87, build_instruction("RES 0,A", |registers, _| {
        registers.a = res_operator(0, registers.a, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x88, build_instruction("RES 1,B", |registers, _| {
        registers.b = res_operator(1, registers.b, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x89, build_instruction("RES 1,C", |registers, _| {
        registers.c = res_operator(1, registers.c, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x8A, build_instruction("RES 1,D", |registers, _| {
        registers.d = res_operator(1, registers.d, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x8B, build_instruction("RES 1,E", |registers, _| {
        registers.e = res_operator(1, registers.e, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x8C, build_instruction("RES 1,H", |registers, _| {
        registers.h = res_operator(1, registers.h, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x8D, build_instruction("RES 1,L", |registers, _| {
        registers.l = res_operator(1, registers.l, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x8E, build_instruction("RES 1,(HL)", |registers, memory| {
        memory.write_byte(registers.get_hl(), res_operator(1, memory.read_byte(registers.get_hl()), &mut registers.f));
        (true, true)
    }, true));
    prefix_cb_map.insert(0x8F, build_instruction("RES 1,A", |registers, _| {
        registers.a = res_operator(1, registers.a, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x90, build_instruction("RES 2,B", |registers, _| {
        registers.b = res_operator(2, registers.b, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x91, build_instruction("RES 2,C", |registers, _| {
        registers.c = res_operator(2, registers.c, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x92, build_instruction("RES 2,D", |registers, _| {
        registers.d = res_operator(2, registers.d, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x93, build_instruction("RES 2,E", |registers, _| {
        registers.e = res_operator(2, registers.e, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x94, build_instruction("RES 2,H", |registers, _| {
        registers.h = res_operator(2, registers.h, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x95, build_instruction("RES 2,L", |registers, _| {
        registers.l = res_operator(2, registers.l, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x96, build_instruction("RES 2,(HL)", |registers, memory| {
        memory.write_byte(registers.get_hl(), res_operator(2, memory.read_byte(registers.get_hl()), &mut registers.f));
        (true, true)
    }, true));
    prefix_cb_map.insert(0x97, build_instruction("RES 2,A", |registers, _| {
        registers.a = res_operator(2, registers.a, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x98, build_instruction("RES 3,B", |registers, _| {
        registers.b = res_operator(3, registers.b, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x99, build_instruction("RES 3,C", |registers, _| {
        registers.c = res_operator(3, registers.c, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x9A, build_instruction("RES 3,D", |registers, _| {
        registers.d = res_operator(3, registers.d, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x9B, build_instruction("RES 3,E", |registers, _| {
        registers.e = res_operator(3, registers.e, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x9C, build_instruction("RES 3,H", |registers, _| {
        registers.h = res_operator(3, registers.h, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x9D, build_instruction("RES 3,L", |registers, _| {
        registers.l = res_operator(3, registers.l, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0x9E, build_instruction("RES 3,(HL)", |registers, memory| {
        memory.write_byte(registers.get_hl(), res_operator(3, memory.read_byte(registers.get_hl()), &mut registers.f));
        (true, true)
    }, true));
    prefix_cb_map.insert(0x9F, build_instruction("RES 3,A", |registers, _| {
        registers.a = res_operator(3, registers.a, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xA0, build_instruction("RES 4,B", |registers, _| {
        registers.b = res_operator(4, registers.b, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xA1, build_instruction("RES 4,C", |registers, _| {
        registers.c = res_operator(4, registers.c, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xA2, build_instruction("RES 4,D", |registers, _| {
        registers.d = res_operator(4, registers.d, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xA3, build_instruction("RES 4,E", |registers, _| {
        registers.e = res_operator(4, registers.e, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xA4, build_instruction("RES 4,H", |registers, _| {
        registers.h = res_operator(4, registers.h, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xA5, build_instruction("RES 4,L", |registers, _| {
        registers.l = res_operator(4, registers.l, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xA6, build_instruction("RES 4,(HL)", |registers, memory| {
        memory.write_byte(registers.get_hl(), res_operator(4, memory.read_byte(registers.get_hl()), &mut registers.f));
        (true, true)
    }, true));
    prefix_cb_map.insert(0xA7, build_instruction("RES 4,A", |registers, _| {
        registers.a = res_operator(4, registers.a, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xA8, build_instruction("RES 5,B", |registers, _| {
        registers.b = res_operator(5, registers.b, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xA9, build_instruction("RES 5,C", |registers, _| {
        registers.c = res_operator(5, registers.c, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xAA, build_instruction("RES 5,D", |registers, _| {
        registers.d = res_operator(5, registers.d, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xAB, build_instruction("RES 5,E", |registers, _| {
        registers.e = res_operator(5, registers.e, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xAC, build_instruction("RES 5,H", |registers, _| {
        registers.h = res_operator(5, registers.h, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xAD, build_instruction("RES 5,L", |registers, _| {
        registers.l = res_operator(5, registers.l, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xAE, build_instruction("RES 5,(HL)", |registers, memory| {
        memory.write_byte(registers.get_hl(), res_operator(5, memory.read_byte(registers.get_hl()), &mut registers.f));
        (true, true)
    }, true));
    prefix_cb_map.insert(0xAF, build_instruction("RES 5,A", |registers, _| {
        registers.a = res_operator(5, registers.a, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xB0, build_instruction("RES 6,B", |registers, _| {
        registers.b = res_operator(6, registers.b, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xB1, build_instruction("RES 6,C", |registers, _| {
        registers.c = res_operator(6, registers.c, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xB2, build_instruction("RES 6,D", |registers, _| {
        registers.d = res_operator(6, registers.d, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xB3, build_instruction("RES 6,E", |registers, _| {
        registers.e = res_operator(6, registers.e, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xB4, build_instruction("RES 6,H", |registers, _| {
        registers.h = res_operator(6, registers.h, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xB5, build_instruction("RES 6,L", |registers, _| {
        registers.l = res_operator(6, registers.l, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xB6, build_instruction("RES 6,(HL)", |registers, memory| {
        memory.write_byte(registers.get_hl(), res_operator(6, memory.read_byte(registers.get_hl()), &mut registers.f));
        (true, true)
    }, true));
    prefix_cb_map.insert(0xB7, build_instruction("RES 6,A", |registers, _| {
        registers.a = res_operator(6, registers.a, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xB8, build_instruction("RES 7,B", |registers, _| {
        registers.b = res_operator(7, registers.b, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xB9, build_instruction("RES 7,C", |registers, _| {
        registers.c = res_operator(7, registers.c, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xBA, build_instruction("RES 7,D", |registers, _| {
        registers.d = res_operator(7, registers.d, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xBB, build_instruction("RES 7,E", |registers, _| {
        registers.e = res_operator(7, registers.e, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xBC, build_instruction("RES 7,H", |registers, _| {
        registers.h = res_operator(7, registers.h, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xBD, build_instruction("RES 7,L", |registers, _| {
        registers.l = res_operator(7, registers.l, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xBE, build_instruction("RES 7,(HL)", |registers, memory| {
        memory.write_byte(registers.get_hl(), res_operator(7, memory.read_byte(registers.get_hl()), &mut registers.f));
        (true, true)
    }, true));
    prefix_cb_map.insert(0xBF, build_instruction("RES 7,A", |registers, _| {
        registers.a = res_operator(7, registers.a, &mut registers.f);
        (true, true)
    }, false));
}

fn instructions_map_8_bitprefix_cb_map_set_instructions(prefix_cb_map: &mut InstructionsMap) -> () {
    fn set_operator(bit: usize, data: u8, _: &mut Flags) -> u8 { data | (1 << bit) }

    prefix_cb_map.insert(0xC0, build_instruction("SET 0,B", |registers, _| {
        registers.b = set_operator(0, registers.b, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xC1, build_instruction("SET 0,C", |registers, _| {
        registers.c = set_operator(0, registers.c, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xC2, build_instruction("SET 0,D", |registers, _| {
        registers.d = set_operator(0, registers.d, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xC3, build_instruction("SET 0,E", |registers, _| {
        registers.e = set_operator(0, registers.e, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xC4, build_instruction("SET 0,H", |registers, _| {
        registers.h = set_operator(0, registers.h, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xC5, build_instruction("SET 0,L", |registers, _| {
        registers.l = set_operator(0, registers.l, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xC6, build_instruction("SET 0,(HL)", |registers, memory| {
        memory.write_byte(registers.get_hl(), set_operator(0, memory.read_byte(registers.get_hl()), &mut registers.f));
        (true, true)
    }, true));
    prefix_cb_map.insert(0xC7, build_instruction("SET 0,A", |registers, _| {
        registers.a = set_operator(0, registers.a, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xC8, build_instruction("SET 1,B", |registers, _| {
        registers.b = set_operator(1, registers.b, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xC9, build_instruction("SET 1,C", |registers, _| {
        registers.c = set_operator(1, registers.c, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xCA, build_instruction("SET 1,D", |registers, _| {
        registers.d = set_operator(1, registers.d, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xCB, build_instruction("SET 1,E", |registers, _| {
        registers.e = set_operator(1, registers.e, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xCC, build_instruction("SET 1,H", |registers, _| {
        registers.h = set_operator(1, registers.h, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xCD, build_instruction("SET 1,L", |registers, _| {
        registers.l = set_operator(1, registers.l, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xCE, build_instruction("SET 1,(HL)", |registers, memory| {
        memory.write_byte(registers.get_hl(), set_operator(1, memory.read_byte(registers.get_hl()), &mut registers.f));
        (true, true)
    }, true));
    prefix_cb_map.insert(0xCF, build_instruction("SET 1,A", |registers, _| {
        registers.a = set_operator(1, registers.a, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xD0, build_instruction("SET 2,B", |registers, _| {
        registers.b = set_operator(2, registers.b, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xD1, build_instruction("SET 2,C", |registers, _| {
        registers.c = set_operator(2, registers.c, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xD2, build_instruction("SET 2,D", |registers, _| {
        registers.d = set_operator(2, registers.d, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xD3, build_instruction("SET 2,E", |registers, _| {
        registers.e = set_operator(2, registers.e, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xD4, build_instruction("SET 2,H", |registers, _| {
        registers.h = set_operator(2, registers.h, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xD5, build_instruction("SET 2,L", |registers, _| {
        registers.l = set_operator(2, registers.l, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xD6, build_instruction("SET 2,(HL)", |registers, memory| {
        memory.write_byte(registers.get_hl(), set_operator(2, memory.read_byte(registers.get_hl()), &mut registers.f));
        (true, true)
    }, true));
    prefix_cb_map.insert(0xD7, build_instruction("SET 2,A", |registers, _| {
        registers.a = set_operator(2, registers.a, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xD8, build_instruction("SET 3,B", |registers, _| {
        registers.b = set_operator(3, registers.b, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xD9, build_instruction("SET 3,C", |registers, _| {
        registers.c = set_operator(3, registers.c, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xDA, build_instruction("SET 3,D", |registers, _| {
        registers.d = set_operator(3, registers.d, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xDB, build_instruction("SET 3,E", |registers, _| {
        registers.e = set_operator(3, registers.e, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xDC, build_instruction("SET 3,H", |registers, _| {
        registers.h = set_operator(3, registers.h, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xDD, build_instruction("SET 3,L", |registers, _| {
        registers.l = set_operator(3, registers.l, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xDE, build_instruction("SET 3,(HL)", |registers, memory| {
        memory.write_byte(registers.get_hl(), set_operator(3, memory.read_byte(registers.get_hl()), &mut registers.f));
        (true, true)
    }, true));
    prefix_cb_map.insert(0xDF, build_instruction("SET 3,A", |registers, _| {
        registers.a = set_operator(3, registers.a, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xE0, build_instruction("SET 4,B", |registers, _| {
        registers.b = set_operator(4, registers.b, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xE1, build_instruction("SET 4,C", |registers, _| {
        registers.c = set_operator(4, registers.c, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xE2, build_instruction("SET 4,D", |registers, _| {
        registers.d = set_operator(4, registers.d, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xE3, build_instruction("SET 4,E", |registers, _| {
        registers.e = set_operator(4, registers.e, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xE4, build_instruction("SET 4,H", |registers, _| {
        registers.h = set_operator(4, registers.h, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xE5, build_instruction("SET 4,L", |registers, _| {
        registers.l = set_operator(4, registers.l, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xE6, build_instruction("SET 4,(HL)", |registers, memory| {
        memory.write_byte(registers.get_hl(), set_operator(4, memory.read_byte(registers.get_hl()), &mut registers.f));
        (true, true)
    }, true));
    prefix_cb_map.insert(0xE7, build_instruction("SET 4,A", |registers, _| {
        registers.a = set_operator(4, registers.a, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xE8, build_instruction("SET 5,B", |registers, _| {
        registers.b = set_operator(5, registers.b, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xE9, build_instruction("SET 5,C", |registers, _| {
        registers.c = set_operator(5, registers.c, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xEA, build_instruction("SET 5,D", |registers, _| {
        registers.d = set_operator(5, registers.d, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xEB, build_instruction("SET 5,E", |registers, _| {
        registers.e = set_operator(5, registers.e, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xEC, build_instruction("SET 5,H", |registers, _| {
        registers.h = set_operator(5, registers.h, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xED, build_instruction("SET 5,L", |registers, _| {
        registers.l = set_operator(5, registers.l, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xEE, build_instruction("SET 5,(HL)", |registers, memory| {
        memory.write_byte(registers.get_hl(), set_operator(5, memory.read_byte(registers.get_hl()), &mut registers.f));
        (true, true)
    }, true));
    prefix_cb_map.insert(0xEF, build_instruction("SET 5,A", |registers, _| {
        registers.a = set_operator(5, registers.a, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xF0, build_instruction("SET 6,B", |registers, _| {
        registers.b = set_operator(6, registers.b, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xF1, build_instruction("SET 6,C", |registers, _| {
        registers.c = set_operator(6, registers.c, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xF2, build_instruction("SET 6,D", |registers, _| {
        registers.d = set_operator(6, registers.d, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xF3, build_instruction("SET 6,E", |registers, _| {
        registers.e = set_operator(6, registers.e, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xF4, build_instruction("SET 6,H", |registers, _| {
        registers.h = set_operator(6, registers.h, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xF5, build_instruction("SET 6,L", |registers, _| {
        registers.l = set_operator(6, registers.l, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xF6, build_instruction("SET 6,(HL)", |registers, memory| {
        memory.write_byte(registers.get_hl(), set_operator(6, memory.read_byte(registers.get_hl()), &mut registers.f));
        (true, true)
    }, true));
    prefix_cb_map.insert(0xF7, build_instruction("SET 6,A", |registers, _| {
        registers.a = set_operator(6, registers.a, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xF8, build_instruction("SET 7,B", |registers, _| {
        registers.b = set_operator(7, registers.b, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xF9, build_instruction("SET 7,C", |registers, _| {
        registers.c = set_operator(7, registers.c, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xFA, build_instruction("SET 7,D", |registers, _| {
        registers.d = set_operator(7, registers.d, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xFB, build_instruction("SET 7,E", |registers, _| {
        registers.e = set_operator(7, registers.e, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xFC, build_instruction("SET 7,H", |registers, _| {
        registers.h = set_operator(7, registers.h, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xFD, build_instruction("SET 7,L", |registers, _| {
        registers.l = set_operator(7, registers.l, &mut registers.f);
        (true, true)
    }, false));
    prefix_cb_map.insert(0xFE, build_instruction("SET 7,(HL)", |registers, memory| {
        memory.write_byte(registers.get_hl(), set_operator(7, memory.read_byte(registers.get_hl()), &mut registers.f));
        (true, true)
    }, true));
    prefix_cb_map.insert(0xFF, build_instruction("SET 7,A", |registers, _| {
        registers.a = set_operator(7, registers.a, &mut registers.f);
        (true, true)
    }, false));
}
