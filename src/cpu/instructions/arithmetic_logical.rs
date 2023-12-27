use std::collections::HashMap;

use crate::cpu::instructions::{Cycles, Instruction};
use crate::cpu::registers::Flags;

pub(super)
fn instructions_map_arithmetic_logical_instructions(instructions_map: &mut HashMap<u8, Instruction>) -> () {
    instructions_map_8_bit_arithmetic_logical_instructions(instructions_map);
    instructions_map_16_bit_arithmetic_logical_instructions(instructions_map);
}

fn instructions_map_8_bit_arithmetic_logical_instructions(instructions_map: &mut HashMap<u8, Instruction>) -> () {
    fn operation(flags: &mut Flags, operator: fn(&mut Flags)) { operator(flags) }
    fn unary_operation(data: u8, flags: &mut Flags, unary_operator: fn(u8, &mut Flags) -> u8) -> u8 { unary_operator(data, flags) }
    fn binary_operation(left: u8, right: u8, flags: &mut Flags, binary_operator: fn(u8, u8, &mut Flags) -> u8) -> u8 { binary_operator(left, right, flags) }

    fn scf_operator(flags: &mut Flags) {
        flags.c = true;
        flags.h = false;
        flags.n = false;
    }

    fn ccf_operator(flags: &mut Flags) {
        flags.c = !flags.c;
        flags.h = false;
        flags.n = false;
    }

    fn cpl_operator(a: u8, flags: &mut Flags) -> u8 {
        let result = !a;
        flags.h = true;
        flags.n = true;
        result
    }

    fn daa_operator(mut a: u8, flags: &mut Flags) -> u8 {
        let mut adjustment = if flags.c { 0x60 } else { 0x00 };
        if flags.h || (!flags.n && (a & 0xF) > 9) {
            adjustment |= 0x06;
        }
        if flags.c || (!flags.n && a > 0x99) {
            adjustment |= 0x60;
            flags.c = true;
        }
        a = if flags.n {
            a.wrapping_sub(adjustment)
        } else {
            a.wrapping_add(adjustment)
        };
        flags.z = a == 0;
        flags.h = false;
        a
    }

    fn inc_operator(data: u8, flags: &mut Flags) -> u8 {
        let result = data.wrapping_add(1);
        flags.z = result == 0;
        flags.n = false;
        flags.h = (data & 0x0F) == 0x0F;
        result
    }

    fn dec_operator(data: u8, flags: &mut Flags) -> u8 {
        let result = data.wrapping_sub(1);
        flags.z = result == 0;
        flags.n = true;
        flags.h = (data & 0x0F) == 0x00;
        result
    }

    fn and_operator(left: u8, right: u8, flags: &mut Flags) -> u8 {
        let result = left & right;
        flags.z = result == 0;
        flags.n = false;
        flags.h = true;
        flags.c = false;
        result
    }

    fn or_operator(left: u8, right: u8, flags: &mut Flags) -> u8 {
        let result = left | right;
        flags.z = result == 0;
        flags.n = false;
        flags.h = false;
        flags.c = false;
        result
    }

    fn xor_operator(left: u8, right: u8, flags: &mut Flags) -> u8 {
        let result = left ^ right;
        flags.z = result == 0;
        flags.n = false;
        flags.h = false;
        flags.c = false;
        result
    }

    fn add_operator(left: u8, right: u8, flags: &mut Flags) -> u8 {
        let result_u16 = left as u16 + right as u16;
        let result = result_u16 as u8;
        flags.z = result == 0;
        flags.n = false;
        flags.h = ((left & 0x0F) + (right & 0x0F)) & 0x10 != 0;
        flags.c = result_u16 > 0xFF;
        result
    }

    fn adc_operator(left: u8, right: u8, flags: &mut Flags) -> u8 {
        let carry = if flags.c { 1 } else { 0 };
        let result_u16 = left as u16 + right as u16 + carry as u16;
        let result = result_u16 as u8;
        flags.z = result == 0;
        flags.n = false;
        flags.h = (((left & 0xF) + (right & 0xF) + carry) & 0x10) != 0;
        flags.c = result_u16 > 0xFF;
        result
    }

    fn sub_operator(left: u8, right: u8, flags: &mut Flags) -> u8 {
        let result_i16 = left as i16 - right as i16;
        let result = result_i16 as u8;
        flags.z = result == 0;
        flags.n = true;
        flags.h = (left & 0x0F) < (right & 0x0F);
        flags.c = result_i16 < 0;
        result
    }

    fn sbc_operator(left: u8, right: u8, flags: &mut Flags) -> u8 {
        let carry = if flags.c { 1 } else { 0 };
        let result_i16 = left as i16 - right as i16 - carry as i16;
        let result = result_i16 as u8;
        flags.z = result == 0;
        flags.n = true;
        flags.h = (left & 0x0F) < (right & 0x0F) + carry;
        flags.c = result_i16 < 0;
        result
    }

    fn cp_operator(left: u8, right: u8, flags: &mut Flags) -> u8 {
        let result = left as i16 - right as i16;
        flags.z = (result as u8) == 0;
        flags.n = true;
        flags.h = (left & 0x0F) < (right & 0x0F);
        flags.c = result < 0;
        left
    }

    instructions_map.insert(
        0x04, Instruction::new(
            "INC B", |registers, _memory| {
                registers.b = unary_operation(registers.b, &mut registers.f, inc_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x14, Instruction::new(
            "INC D", |registers, _memory| {
                registers.d = unary_operation(registers.d, &mut registers.f, inc_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x24, Instruction::new(
            "INC H", |registers, _memory| {
                registers.h = unary_operation(registers.h, &mut registers.f, inc_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x34, Instruction::new(
            "INC (HL)", |registers, memory| {
                memory.write_byte(registers.get_bc(), unary_operation(memory.read_byte(registers.get_bc()), &mut registers.f, inc_operator));
                (true, true)
            }, Cycles::new(12), 1,
        ),
    );

    instructions_map.insert(
        0x0C, Instruction::new(
            "INC C", |registers, _memory| {
                registers.c = unary_operation(registers.c, &mut registers.f, inc_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x1C, Instruction::new(
            "INC E", |registers, _memory| {
                registers.e = unary_operation(registers.e, &mut registers.f, inc_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x2C, Instruction::new(
            "INC L", |registers, _memory| {
                registers.l = unary_operation(registers.l, &mut registers.f, inc_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x3C, Instruction::new(
            "INC A", |registers, _memory| {
                registers.a = unary_operation(registers.a, &mut registers.f, inc_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x05, Instruction::new(
            "DEC B", |registers, _memory| {
                registers.b = unary_operation(registers.b, &mut registers.f, dec_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x15, Instruction::new(
            "DEC D", |registers, _memory| {
                registers.d = unary_operation(registers.d, &mut registers.f, dec_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x25, Instruction::new(
            "DEC L", |registers, _memory| {
                registers.l = unary_operation(registers.l, &mut registers.f, dec_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x35, Instruction::new(
            "DEC (HL)", |registers, memory| {
                memory.write_byte(registers.get_bc(), unary_operation(memory.read_byte(registers.get_bc()), &mut registers.f, inc_operator));
                (true, true)
            }, Cycles::new(12), 1,
        ),
    );

    instructions_map.insert(
        0x0D, Instruction::new(
            "DEC C", |registers, _memory| {
                registers.c = unary_operation(registers.c, &mut registers.f, dec_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x1D, Instruction::new(
            "DEC E", |registers, _memory| {
                registers.e = unary_operation(registers.e, &mut registers.f, dec_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x2D, Instruction::new(
            "DEC L", |registers, _memory| {
                registers.l = unary_operation(registers.l, &mut registers.f, dec_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x3D, Instruction::new(
            "DEC A", |registers, _memory| {
                registers.a = unary_operation(registers.a, &mut registers.f, dec_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x27, Instruction::new(
            "DAA", |registers, _memory| {
                registers.a = unary_operation(registers.a, &mut registers.f, daa_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x37, Instruction::new(
            "SCF", |registers, _memory| {
                operation(&mut registers.f, scf_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x2F, Instruction::new(
            "CPL", |registers, _memory| {
                registers.a = unary_operation(registers.a, &mut registers.f, cpl_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x3F, Instruction::new(
            "CCF", |registers, _memory| {
                operation(&mut registers.f, ccf_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x80, Instruction::new(
            "ADD A, B", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.b, &mut registers.f, add_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x81, Instruction::new(
            "ADD A, C", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.c, &mut registers.f, add_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x82, Instruction::new(
            "ADD A, D", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.d, &mut registers.f, add_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x83, Instruction::new(
            "ADD A, E", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.e, &mut registers.f, add_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x84, Instruction::new(
            "ADD A, H", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.h, &mut registers.f, add_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x85, Instruction::new(
            "ADD A, L", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.l, &mut registers.f, add_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x86, Instruction::new(
            "ADD A, (HL)", |registers, memory| {
                registers.a = binary_operation(registers.a, memory.read_byte(registers.get_hl()), &mut registers.f, add_operator);
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x87, Instruction::new(
            "ADD A, A", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.a, &mut registers.f, add_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x88, Instruction::new(
            "ADC A, B", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.b, &mut registers.f, adc_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x89, Instruction::new(
            "ADC A, C", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.c, &mut registers.f, adc_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x8A, Instruction::new(
            "ADC A, D", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.d, &mut registers.f, adc_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x8B, Instruction::new(
            "ADC A, E", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.e, &mut registers.f, adc_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x8C, Instruction::new(
            "ADC A, H", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.h, &mut registers.f, adc_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x8D, Instruction::new(
            "ADC A, L", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.l, &mut registers.f, adc_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x8E, Instruction::new(
            "ADC A, (HL)", |registers, memory| {
                registers.a = binary_operation(registers.a, memory.read_byte(registers.get_hl()), &mut registers.f, adc_operator);
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x8F, Instruction::new(
            "ADC A, A", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.a, &mut registers.f, adc_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x90, Instruction::new(
            "SUB A, B", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.b, &mut registers.f, sub_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x91, Instruction::new(
            "SUB A, C", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.c, &mut registers.f, sub_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x92, Instruction::new(
            "SUB A, D", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.d, &mut registers.f, sub_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x93, Instruction::new(
            "SUB A, E", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.e, &mut registers.f, sub_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x94, Instruction::new(
            "SUB A, H", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.h, &mut registers.f, sub_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x95, Instruction::new(
            "SUB A, L", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.l, &mut registers.f, sub_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x96, Instruction::new(
            "SUB A, (HL)", |registers, memory| {
                registers.a = binary_operation(registers.a, memory.read_byte(registers.get_hl()), &mut registers.f, sub_operator);
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x97, Instruction::new(
            "SUB A, A", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.a, &mut registers.f, sub_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x98, Instruction::new(
            "SBC A, B", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.b, &mut registers.f, sbc_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x99, Instruction::new(
            "SBC A, C", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.c, &mut registers.f, sbc_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x9A, Instruction::new(
            "SBC A, D", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.d, &mut registers.f, sbc_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x9B, Instruction::new(
            "SBC A, E", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.e, &mut registers.f, sbc_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x9C, Instruction::new(
            "SBC A, H", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.h, &mut registers.f, sbc_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x9D, Instruction::new(
            "SBC A, L", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.l, &mut registers.f, sbc_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x9E, Instruction::new(
            "SBC A, (HL)", |registers, memory| {
                registers.a = binary_operation(registers.a, memory.read_byte(registers.get_hl()), &mut registers.f, sbc_operator);
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x9F, Instruction::new(
            "SBC A, A", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.a, &mut registers.f, sbc_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xA0, Instruction::new(
            "AND A, B", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.b, &mut registers.f, and_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xA1, Instruction::new(
            "AND A, C", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.c, &mut registers.f, and_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xA2, Instruction::new(
            "AND A, D", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.d, &mut registers.f, and_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xA3, Instruction::new(
            "AND A, E", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.e, &mut registers.f, and_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xA4, Instruction::new(
            "AND A, H", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.h, &mut registers.f, and_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xA5, Instruction::new(
            "AND A, L", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.l, &mut registers.f, and_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xA6, Instruction::new(
            "AND A, (HL)", |registers, memory| {
                registers.a = binary_operation(registers.a, memory.read_byte(registers.get_hl()), &mut registers.f, and_operator);
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0xA7, Instruction::new(
            "AND A, A", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.a, &mut registers.f, and_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xA8, Instruction::new(
            "XOR A, B", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.b, &mut registers.f, xor_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xA9, Instruction::new(
            "XOR A, C", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.c, &mut registers.f, xor_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xAA, Instruction::new(
            "XOR A, D", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.d, &mut registers.f, xor_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xAB, Instruction::new(
            "XOR A, E", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.e, &mut registers.f, xor_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xAC, Instruction::new(
            "XOR A, H", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.h, &mut registers.f, xor_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xAD, Instruction::new(
            "XOR A, L", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.l, &mut registers.f, xor_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xAE, Instruction::new(
            "XOR A, (HL)", |registers, memory| {
                registers.a = binary_operation(registers.a, memory.read_byte(registers.get_hl()), &mut registers.f, xor_operator);
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0xAF, Instruction::new(
            "XOR A, A", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.a, &mut registers.f, xor_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xB0, Instruction::new(
            "OR A, B", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.b, &mut registers.f, or_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xB1, Instruction::new(
            "OR A, C", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.c, &mut registers.f, or_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xB2, Instruction::new(
            "OR A, D", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.d, &mut registers.f, or_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xB3, Instruction::new(
            "OR A, E", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.e, &mut registers.f, or_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xB4, Instruction::new(
            "OR A, H", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.h, &mut registers.f, or_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xB5, Instruction::new(
            "OR A, L", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.l, &mut registers.f, or_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xB6, Instruction::new(
            "OR A, (HL)", |registers, memory| {
                registers.a = binary_operation(registers.a, memory.read_byte(registers.get_hl()), &mut registers.f, or_operator);
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0xB7, Instruction::new(
            "OR A, A", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.a, &mut registers.f, or_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xB8, Instruction::new(
            "CP A, B", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.b, &mut registers.f, cp_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xB9, Instruction::new(
            "CP A, C", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.c, &mut registers.f, cp_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xBA, Instruction::new(
            "CP A, D", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.d, &mut registers.f, cp_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xBB, Instruction::new(
            "CP A, E", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.e, &mut registers.f, cp_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xBC, Instruction::new(
            "CP A, H", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.h, &mut registers.f, cp_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xBD, Instruction::new(
            "CP A, L", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.l, &mut registers.f, cp_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xBE, Instruction::new(
            "CP A, (HL)", |registers, memory| {
                registers.a = binary_operation(registers.a, memory.read_byte(registers.get_hl()), &mut registers.f, cp_operator);
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0xBF, Instruction::new(
            "CP A, A", |registers, _memory| {
                registers.a = binary_operation(registers.a, registers.a, &mut registers.f, cp_operator);
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xC6, Instruction::new(
            "ADD A, d8", |registers, memory| {
                registers.a = binary_operation(registers.a, memory.read_byte(registers.pc + 1), &mut registers.f, add_operator);
                (true, true)
            }, Cycles::new(8), 2,
        ),
    );

    instructions_map.insert(
        0xD6, Instruction::new(
            "SUB A, d8", |registers, memory| {
                registers.a = binary_operation(registers.a, memory.read_byte(registers.pc + 1), &mut registers.f, sub_operator);
                (true, true)
            }, Cycles::new(8), 2,
        ),
    );

    instructions_map.insert(
        0xE6, Instruction::new(
            "AND A, d8", |registers, memory| {
                registers.a = binary_operation(registers.a, memory.read_byte(registers.pc + 1), &mut registers.f, and_operator);
                (true, true)
            }, Cycles::new(8), 2,
        ),
    );

    instructions_map.insert(
        0xF6, Instruction::new(
            "OR A, d8", |registers, memory| {
                registers.a = binary_operation(registers.a, memory.read_byte(registers.pc + 1), &mut registers.f, or_operator);
                (true, true)
            }, Cycles::new(8), 2,
        ),
    );

    instructions_map.insert(
        0xCE, Instruction::new(
            "ADC A, d8", |registers, memory| {
                registers.a = binary_operation(registers.a, memory.read_byte(registers.pc + 1), &mut registers.f, adc_operator);
                (true, true)
            }, Cycles::new(8), 2,
        ),
    );

    instructions_map.insert(
        0xDE, Instruction::new(
            "SBC A, d8", |registers, memory| {
                registers.a = binary_operation(registers.a, memory.read_byte(registers.pc + 1), &mut registers.f, sbc_operator);
                (true, true)
            }, Cycles::new(8), 2,
        ),
    );

    instructions_map.insert(
        0xEE, Instruction::new(
            "XOR A, d8", |registers, memory| {
                registers.a = binary_operation(registers.a, memory.read_byte(registers.pc + 1), &mut registers.f, xor_operator);
                (true, true)
            }, Cycles::new(8), 2,
        ),
    );

    instructions_map.insert(
        0xFE, Instruction::new(
            "CP A, d8", |registers, memory| {
                registers.a = binary_operation(registers.a, memory.read_byte(registers.pc + 1), &mut registers.f, cp_operator);
                (true, true)
            }, Cycles::new(8), 2,
        ),
    );
}

fn instructions_map_16_bit_arithmetic_logical_instructions(instructions_map: &mut HashMap<u8, Instruction>) -> () {
    fn unary_operation(data: u16, unary_operator: fn(u16) -> u16) -> u16 { *data = unary_operator(*data) }
    fn binary_operation(left: u16, right: u16, flags: &mut Flags, binary_operator: fn(u16, u16, &mut Flags) -> u16) -> u16 { *left = binary_operator(*left, *right, flags) }

    fn inc_operator(data: u16) -> u16 {
        data.wrapping_add(1)
    }

    fn dec_operator(data: u16) -> u16 {
        data.wrapping_sub(1)
    }
    fn add_operator(left: u16, right: u16, flags: &mut Flags) -> u16 {
        let result = left.wrapping_add(right);
        flags.n = false;
        flags.h = ((left & 0x0FFF) + (right & 0x0FFF)) > 0x0FFF;
        flags.c = result < left;
        result
    }

    instructions_map.insert(
        0x04, Instruction::new(
            "INC BC", |registers, _memory| {
                registers.set_bc(unary_operation(registers.get_bc(), inc_operator));
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x14, Instruction::new(
            "INC DE", |registers, _memory| {
                registers.set_de(unary_operation(registers.get_de(), inc_operator));
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x24, Instruction::new(
            "INC HL", |registers, _memory| {
                registers.set_hl(unary_operation(registers.get_hl(), inc_operator));
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x34, Instruction::new(
            "INC SP", |registers, memory| {
                registers.sp = unary_operation(registers.sp, inc_operator);
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x09, Instruction::new(
            "ADD HL, BC", |registers, _memory| {
                registers.set_hl(binary_operation(registers.get_hl(), registers.get_bc(), &mut registers.f, add_operator));
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x19, Instruction::new(
            "ADD HL, DE", |registers, _memory| {
                registers.set_hl(binary_operation(registers.get_hl(), registers.get_de(), &mut registers.f, add_operator));
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x29, Instruction::new(
            "ADD HL, HL", |registers, _memory| {
                registers.set_hl(binary_operation(registers.get_hl(), registers.get_hl(), &mut registers.f, add_operator));
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x39, Instruction::new(
            "ADD HL, SP", |registers, memory| {
                registers.set_hl(binary_operation(registers.get_hl(), registers.sp, &mut registers.f, add_operator));
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x0B, Instruction::new(
            "DEC BC", |registers, _memory| {
                registers.set_bc(unary_operation(registers.get_bc(), dec_operator));
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x1B, Instruction::new(
            "DEC DE", |registers, _memory| {
                registers.set_de(unary_operation(registers.get_de(), dec_operator));
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x2B, Instruction::new(
            "DEC HL", |registers, _memory| {
                registers.set_hl(unary_operation(registers.get_hl(), dec_operator));
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x3B, Instruction::new(
            "DEC SP", |registers, memory| {
                registers.sp = unary_operation(registers.sp, dec_operator);
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );
}