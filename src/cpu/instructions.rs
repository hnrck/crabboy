use std::collections::HashMap;

use crate::cpu::registers::{CpuState, Flags, Registers};
use crate::mmu::MMU;

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

fn instructions_map_control_commands(instructions_map: &mut HashMap<u8, Instruction>) -> () {
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

fn instructions_map_jump_commands(instructions_map: &mut HashMap<u8, Instruction>) -> () {
    instructions_map_jump_commands_jp(instructions_map);
    instructions_map_jump_commands_jr(instructions_map);
    instructions_map_jump_commands_ret(instructions_map);
    instructions_map_jump_commands_call(instructions_map);
    instructions_map_jump_commands_rst(instructions_map);
}

fn instructions_map_jump_commands_jp(instructions_map: &mut HashMap<u8, Instruction>) -> () {
    fn jp(registers: &mut Registers, address: u16) { registers.pc = address }

    instructions_map.insert(
        0xc2, Instruction::new(
            "JP NZ, a16", |registers, memory| {
                if !registers.f.z {
                    jp(registers, memory.read_word(registers.pc + 1));
                    (false, true)
                } else {
                    (true, false)
                }
            }, Cycles::new(16).with_not_taken(12), 3,
        ),
    );

    instructions_map.insert(
        0xc3, Instruction::new(
            "JP a16", |registers, memory| {
                jp(registers, memory.read_word(registers.pc + 1));
                (false, true)
            }, Cycles::new(16), 3,
        ),
    );

    instructions_map.insert(
        0xca, Instruction::new(
            "JP Z, a16", |registers, memory| {
                if registers.f.z {
                    jp(registers, memory.read_word(registers.pc + 1));
                    (false, true)
                } else {
                    (true, false)
                }
            }, Cycles::new(16).with_not_taken(12), 3,
        ),
    );

    instructions_map.insert(
        0xd2, Instruction::new(
            "JP NC, a16", |registers, memory| {
                if !registers.f.c {
                    jp(registers, memory.read_word(registers.pc + 1));
                    (false, true)
                } else {
                    (true, false)
                }
            }, Cycles::new(16).with_not_taken(12), 3,
        ),
    );

    instructions_map.insert(
        0xda, Instruction::new(
            "JP C, a16", |registers, memory| {
                if registers.f.c {
                    jp(registers, memory.read_word(registers.pc + 1));
                    (false, true)
                } else {
                    (true, false)
                }
            }, Cycles::new(16).with_not_taken(12), 3,
        ),
    );

    instructions_map.insert(
        0xe9, Instruction::new(
            "JP (HL)", |registers, memory| {
                jp(registers, memory.read_word(registers.get_hl()));
                (false, true)
            }, Cycles::new(4), 1,
        ),
    );
}

fn instructions_map_jump_commands_jr(instructions_map: &mut HashMap<u8, Instruction>) -> () {
    fn jr(registers: &mut Registers, offset: i8) { registers.pc = ((registers.pc as i16) + (offset as i16)) as u16 }

    instructions_map.insert(
        0x18, Instruction::new(
            "JR, r8", |registers, memory| {
                jr(registers, memory.read_byte(registers.pc + 1) as i8);
                (false, true)
            }, Cycles::new(12), 2,
        ),
    );

    instructions_map.insert(
        0x20, Instruction::new(
            "JR NZ, r8", |registers, memory| {
                if !registers.f.z {
                    jr(registers, memory.read_byte(registers.pc + 1) as i8);
                    (false, true)
                } else {
                    (true, false)
                }
            }, Cycles::new(12).with_not_taken(8), 2,
        ),
    );

    instructions_map.insert(
        0x28, Instruction::new(
            "JR Z, r8", |registers, memory| {
                if registers.f.z {
                    jr(registers, memory.read_byte(registers.pc + 1) as i8);
                    (false, true)
                } else {
                    (true, false)
                }
            }, Cycles::new(12).with_not_taken(8), 2,
        ),
    );

    instructions_map.insert(
        0x30, Instruction::new(
            "JR NC, r8", |registers, memory| {
                if !registers.f.c {
                    jr(registers, memory.read_byte(registers.pc + 1) as i8);
                    (false, true)
                } else {
                    (true, false)
                }
            }, Cycles::new(12).with_not_taken(8), 2,
        ),
    );

    instructions_map.insert(
        0x38, Instruction::new(
            "JR C, r8", |registers, memory| {
                if registers.f.c {
                    jr(registers, memory.read_byte(registers.pc + 1) as i8);
                    (false, true)
                } else {
                    (true, false)
                }
            }, Cycles::new(12).with_not_taken(8), 2,
        ),
    );
}

fn instructions_map_jump_commands_ret(instructions_map: &mut HashMap<u8, Instruction>) -> () {
    fn ret(registers: &mut Registers, memory: &mut MMU) {
        let new_pc = memory.read_word(registers.sp);
        registers.sp += 2;
        registers.pc = new_pc;
    }

    instructions_map.insert(
        0xc9, Instruction::new(
            "RET", |registers, memory| {
                ret(registers, memory);
                (false, true)
            }, Cycles::new(16), 1,
        ),
    );

    instructions_map.insert(
        0xd9, Instruction::new(
            "RETI", |registers, memory| {
                ret(registers, memory);
                registers.enable_interrupts();
                (false, true)
            }, Cycles::new(16), 1,
        ),
    );

    instructions_map.insert(
        0xc0, Instruction::new(
            "RET NZ", |registers, memory| {
                if !registers.f.z {
                    ret(registers, memory);
                    (false, true)
                } else {
                    (true, false)
                }
            }, Cycles::new(20).with_not_taken(8), 1,
        ),
    );

    instructions_map.insert(
        0xc8, Instruction::new(
            "RET Z", |registers, memory| {
                if registers.f.z {
                    ret(registers, memory);
                    (false, true)
                } else {
                    (true, false)
                }
            }, Cycles::new(20).with_not_taken(8), 1,
        ),
    );

    instructions_map.insert(
        0xd0, Instruction::new(
            "RET NC", |registers, memory| {
                if !registers.f.c {
                    ret(registers, memory);
                    (false, true)
                } else {
                    (true, false)
                }
            }, Cycles::new(20).with_not_taken(8), 1,
        ),
    );

    instructions_map.insert(
        0xd8, Instruction::new(
            "RET C", |registers, memory| {
                if registers.f.c {
                    ret(registers, memory);
                    (false, true)
                } else {
                    (true, false)
                }
            }, Cycles::new(20).with_not_taken(8), 1,
        ),
    );
}

fn instructions_map_jump_commands_call(instructions_map: &mut HashMap<u8, Instruction>) -> () {
    fn call(registers: &mut Registers, memory: &mut MMU, address: u16) {
        registers.sp -= 2;
        memory.write_word(registers.sp, registers.pc + 3);
        registers.pc = address;
    }

    instructions_map.insert(
        0xcd, Instruction::new(
            "CALL a16", |registers, memory| {
                call(registers, memory, memory.read_word(registers.pc + 1));
                (false, true)
            }, Cycles::new(24), 3,
        ),
    );

    instructions_map.insert(
        0xcc, Instruction::new(
            "CALL Z, a16", |registers, memory| {
                if registers.f.z {
                    call(registers, memory, memory.read_word(registers.pc + 1));
                    (false, true)
                } else {
                    (true, false)
                }
            }, Cycles::new(24).with_not_taken(12), 3,
        ),
    );

    instructions_map.insert(
        0xdc, Instruction::new(
            "CALL C, a16", |registers, memory| {
                if registers.f.c {
                    call(registers, memory, memory.read_word(registers.pc + 1));
                    (false, true)
                } else {
                    (true, false)
                }
            }, Cycles::new(24).with_not_taken(12), 3,
        ),
    );
}

fn instructions_map_jump_commands_rst(instructions_map: &mut HashMap<u8, Instruction>) -> () {
    fn rst(registers: &mut Registers, memory: &mut MMU, address: u16) {
        registers.sp -= 2;
        memory.write_word(registers.sp, registers.pc + 1);
        registers.pc = address;
    }

    instructions_map.insert(
        0xc7, Instruction::new(
            "RST 00H", |registers, memory| {
                rst(registers, memory, 0x00);
                (false, true)
            }, Cycles::new(16), 1,
        ),
    );

    instructions_map.insert(
        0xd7, Instruction::new(
            "RST 10H", |registers, memory| {
                rst(registers, memory, 0x10);
                (false, true)
            }, Cycles::new(16), 1,
        ),
    );

    instructions_map.insert(
        0xe7, Instruction::new(
            "RST 20H", |registers, memory| {
                rst(registers, memory, 0x20);
                (false, true)
            }, Cycles::new(16), 1,
        ),
    );

    instructions_map.insert(
        0xf7, Instruction::new(
            "RST 30H", |registers, memory| {
                rst(registers, memory, 0x30);
                (false, true)
            }, Cycles::new(16), 1,
        ),
    );

    instructions_map.insert(
        0xcf, Instruction::new(
            "RST 08H", |registers, memory| {
                rst(registers, memory, 0x08);
                (false, true)
            }, Cycles::new(16), 1,
        ),
    );

    instructions_map.insert(
        0xdf, Instruction::new(
            "RST 18H", |registers, memory| {
                rst(registers, memory, 0x18);
                (false, true)
            }, Cycles::new(16), 1,
        ),
    );

    instructions_map.insert(
        0xef, Instruction::new(
            "RST 28H", |registers, memory| {
                rst(registers, memory, 0x28);
                (false, true)
            }, Cycles::new(16), 1,
        ),
    );

    instructions_map.insert(
        0xff, Instruction::new(
            "RST 38H", |registers, memory| {
                rst(registers, memory, 0x38);
                (false, true)
            }, Cycles::new(16), 1,
        ),
    );
}

fn instructions_map_8_bit_load_instructions(_instructions_map: &mut HashMap<u8, Instruction>) -> () {
    fn ld(from: &u8, to: &mut u8) { *to = *from; }

    // TODO
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

    // TODO
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

    // TODO
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
}

fn instructions_map_8_bit_shift_rot_bit_instructions(_instructions_map: &mut HashMap<u8, Instruction>) -> () {
    // TODO
}

pub(crate) fn initialize_instructions_map() -> HashMap<u8, Instruction> {
    let mut instructions_map = HashMap::new();

    instructions_map_control_commands(&mut instructions_map);
    instructions_map_jump_commands(&mut instructions_map);
    instructions_map_8_bit_load_instructions(&mut instructions_map);
    instructions_map_16_bit_load_instructions(&mut instructions_map);
    instructions_map_8_bit_arithmetic_logical_instructions(&mut instructions_map);
    instructions_map_16_bit_arithmetic_logical_instructions(&mut instructions_map);
    instructions_map_8_bit_shift_rot_bit_instructions(&mut instructions_map);

    instructions_map
}