use std::collections::HashMap;

use crate::cpu::instructions::Instruction;
use crate::cpu::registers::Flags;

pub(super)
fn instructions_map_arithmetic_logical_instructions(instructions_map: &mut HashMap<u8, Instruction>) -> () {
    instructions_map_8_bit_arithmetic_logical_instructions(instructions_map);
    instructions_map_16_bit_arithmetic_logical_instructions(instructions_map);
}

fn instructions_map_8_bit_arithmetic_logical_instructions(_instructions_map: &mut HashMap<u8, Instruction>) -> () {
    fn operation(flags: &mut Flags, operator: fn(&mut Flags)) { operator(flags) }
    fn unary_operation(data: &mut u8, flags: &mut Flags, unary_operator: fn(u8, &mut Flags) -> u8) { *data = unary_operator(*data, flags) }
    fn binary_operation(left: &mut u8, right: &u8, flags: &mut Flags, binary_operator: fn(u8, u8, &mut Flags) -> u8) { *left = binary_operator(*left, *right, flags) }

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

    // TODO(henrick) Implementation
}

fn instructions_map_16_bit_arithmetic_logical_instructions(_instructions_map: &mut HashMap<u8, Instruction>) -> () {
    fn unary_operation(data: &mut u16, unary_operator: fn(u16) -> u16) { *data = unary_operator(*data) }
    fn binary_operation(left: &mut u16, right: &u16, flags: &mut Flags, binary_operator: fn(u16, u16, &mut Flags) -> u16) { *left = binary_operator(*left, *right, flags) }

    fn inc_operator(data: u16, _flags: &mut Flags) -> u16 {
        data.wrapping_add(1)
    }

    fn dec_operator(data: u16, _flags: &mut Flags) -> u16 {
        data.wrapping_sub(1)
    }
    fn add_operator(left: u16, right: u16, flags: &mut Flags) -> u16 {
        let result = left.wrapping_add(right);
        flags.n = false;
        flags.h = ((left & 0x0FFF) + (right & 0x0FFF)) > 0x0FFF;
        flags.c = result < left;
        result
    }

    // TODO(henrick) Implementation
}