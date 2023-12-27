use std::collections::HashMap;

use crate::cpu::instructions::{Cycles, Instruction};
use crate::cpu::registers::Registers;
use crate::mmu::MMU;

pub(super) fn instructions_map_load_instructions(instructions_map: &mut HashMap<u8, Instruction>) -> () {
    instructions_map_8_bit_load_instructions(instructions_map);
    instructions_map_16_bit_load_instructions(instructions_map);
}

fn instructions_map_8_bit_load_instructions(instructions_map: &mut HashMap<u8, Instruction>) -> () {
    instructions_map.insert(
        0x02, Instruction::new(
            "LD (BC), A", |registers, memory| {
                memory.write_byte(registers.get_bc(), registers.a);
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x12, Instruction::new(
            "LD (DE), A", |registers, memory| {
                memory.write_byte(registers.get_de(), registers.a);
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x22, Instruction::new(
            "LD (HL+), A", |registers, memory| {
                memory.write_byte(registers.get_hl(), registers.a);
                registers.set_hl(registers.get_hl() + 1);
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x32, Instruction::new(
            "LD (HL-), A", |registers, memory| {
                memory.write_byte(registers.get_hl(), registers.a);
                registers.set_hl(registers.get_hl() - 1);
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x06, Instruction::new(
            "LD (BC), d8", |registers, memory| {
                memory.write_byte(registers.get_bc(), memory.read_byte(registers.pc + 1));
                (true, true)
            }, Cycles::new(8), 2,
        ),
    );

    instructions_map.insert(
        0x16, Instruction::new(
            "LD (DE), d8", |registers, memory| {
                memory.write_byte(registers.get_de(), memory.read_byte(registers.pc + 1));
                (true, true)
            }, Cycles::new(8), 2,
        ),
    );

    instructions_map.insert(
        0x26, Instruction::new(
            "LD (HL+), d8", |registers, memory| {
                memory.write_byte(registers.get_hl(), memory.read_byte(registers.pc + 1));
                registers.set_hl(registers.get_hl() + 1);
                (true, true)
            }, Cycles::new(8), 2,
        ),
    );

    instructions_map.insert(
        0x36, Instruction::new(
            "LD (HL-), d8", |registers, memory| {
                memory.write_byte(registers.get_hl(), memory.read_byte(registers.pc + 1));
                registers.set_hl(registers.get_hl() - 1);
                (true, true)
            }, Cycles::new(8), 2,
        ),
    );

    instructions_map.insert(
        0x0A, Instruction::new(
            "LD A, (BC)", |registers, memory| {
                registers.a = memory.read_byte(registers.get_bc());
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x1A, Instruction::new(
            "LD A, (DE)", |registers, memory| {
                registers.a = memory.read_byte(registers.get_de());
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x2A, Instruction::new(
            "LD A, (HL+)", |registers, memory| {
                registers.a = memory.read_byte(registers.get_hl());
                registers.set_hl(registers.get_hl() + 1);
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x3A, Instruction::new(
            "LD A, (HL-)", |registers, memory| {
                registers.a = memory.read_byte(registers.get_hl());
                registers.set_hl(registers.get_hl() - 1);
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x0E, Instruction::new(
            "LD C, d8", |registers, memory| {
                registers.c = memory.read_byte(registers.pc + 1);
                (true, true)
            }, Cycles::new(8), 2,
        ),
    );

    instructions_map.insert(
        0x1E, Instruction::new(
            "LD E, d8", |registers, memory| {
                registers.e = memory.read_byte(registers.pc + 1);
                (true, true)
            }, Cycles::new(8), 2,
        ),
    );

    instructions_map.insert(
        0x2E, Instruction::new(
            "LD L, d8", |registers, memory| {
                registers.l = memory.read_byte(registers.pc + 1);
                (true, true)
            }, Cycles::new(8), 2,
        ),
    );

    instructions_map.insert(
        0x3E, Instruction::new(
            "LD A, d8", |registers, memory| {
                registers.a = memory.read_byte(registers.pc + 1);
                (true, true)
            }, Cycles::new(8), 2,
        ),
    );

    instructions_map.insert(
        0x40, Instruction::new(
            "LD B, B", |registers, memory| {
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x50, Instruction::new(
            "LD D, B", |registers, memory| {
                registers.d = registers.b;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x60, Instruction::new(
            "LD H, B", |registers, memory| {
                registers.h = registers.b;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x70, Instruction::new(
            "LD (HL), B", |registers, memory| {
                memory.write_byte(registers.get_hl(), registers.b);
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x41, Instruction::new(
            "LD B, C", |registers, memory| {
                registers.b = registers.c;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x51, Instruction::new(
            "LD D, C", |registers, memory| {
                registers.d = registers.c;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x61, Instruction::new(
            "LD H, C", |registers, memory| {
                registers.h = registers.c;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x71, Instruction::new(
            "LD (HL), C", |registers, memory| {
                memory.write_byte(registers.get_hl(), registers.c);
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x42, Instruction::new(
            "LD B, D", |registers, memory| {
                registers.b = registers.d;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x52, Instruction::new(
            "LD D, D", |registers, memory| {
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x62, Instruction::new(
            "LD H, D", |registers, memory| {
                registers.h = registers.d;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x72, Instruction::new(
            "LD (HL), D", |registers, memory| {
                memory.write_byte(registers.get_hl(), registers.d);
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x43, Instruction::new(
            "LD B, E", |registers, memory| {
                registers.b = registers.e;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x53, Instruction::new(
            "LD D, E", |registers, memory| {
                registers.d = registers.e;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x63, Instruction::new(
            "LD H, E", |registers, memory| {
                registers.h = registers.e;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x73, Instruction::new(
            "LD (HL), E", |registers, memory| {
                memory.write_byte(registers.get_hl(), registers.e);
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x44, Instruction::new(
            "LD B, H", |registers, memory| {
                registers.b = registers.h;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x54, Instruction::new(
            "LD D, H", |registers, memory| {
                registers.d = registers.h;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x64, Instruction::new(
            "LD H, H", |registers, memory| {
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x74, Instruction::new(
            "LD (HL), H", |registers, memory| {
                memory.write_byte(registers.get_hl(), registers.h);
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );


    instructions_map.insert(
        0x45, Instruction::new(
            "LD B, L", |registers, memory| {
                registers.b = registers.l;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x55, Instruction::new(
            "LD D, L", |registers, memory| {
                registers.d = registers.l;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x65, Instruction::new(
            "LD H, L", |registers, memory| {
                registers.h = registers.l;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x75, Instruction::new(
            "LD (HL), L", |registers, memory| {
                memory.write_byte(registers.get_hl(), registers.l);
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x46, Instruction::new(
            "LD B, (HL)", |registers, memory| {
                registers.b = memory.read_byte(registers.get_hl());
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x56, Instruction::new(
            "LD D, (HL)", |registers, memory| {
                registers.d = memory.read_byte(registers.get_hl());
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x66, Instruction::new(
            "LD H, (HL)", |registers, memory| {
                registers.h = memory.read_byte(registers.get_hl());
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x47, Instruction::new(
            "LD B, A", |registers, memory| {
                registers.b = registers.a;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x57, Instruction::new(
            "LD D, A", |registers, memory| {
                registers.d = registers.a;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x67, Instruction::new(
            "LD H, A", |registers, memory| {
                registers.h = registers.a;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x77, Instruction::new(
            "LD (HL), A", |registers, memory| {
                memory.write_byte(registers.get_hl(), registers.a);
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x48, Instruction::new(
            "LD C, B", |registers, memory| {
                registers.c = registers.b;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x58, Instruction::new(
            "LD E, B", |registers, memory| {
                registers.e = registers.b;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x68, Instruction::new(
            "LD L, B", |registers, memory| {
                registers.l = registers.b;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x78, Instruction::new(
            "LD A, B", |registers, memory| {
                registers.a = registers.b;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x49, Instruction::new(
            "LD C, C", |registers, memory| {
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x59, Instruction::new(
            "LD E, C", |registers, memory| {
                registers.e = registers.c;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x69, Instruction::new(
            "LD L, C", |registers, memory| {
                registers.l = registers.c;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x79, Instruction::new(
            "LD A, C", |registers, memory| {
                registers.a = registers.c;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x4A, Instruction::new(
            "LD C, D", |registers, memory| {
                registers.c = registers.d;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x5A, Instruction::new(
            "LD E, D", |registers, memory| {
                registers.e = registers.d;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x6A, Instruction::new(
            "LD L, D", |registers, memory| {
                registers.l = registers.d;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x7A, Instruction::new(
            "LD A, D", |registers, memory| {
                registers.a = registers.d;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x4B, Instruction::new(
            "LD C, E", |registers, memory| {
                registers.c = registers.e;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x5B, Instruction::new(
            "LD E, E", |registers, memory| {
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x6B, Instruction::new(
            "LD L, E", |registers, memory| {
                registers.l = registers.e;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x7B, Instruction::new(
            "LD A, E", |registers, memory| {
                registers.a = registers.e;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x4C, Instruction::new(
            "LD C, H", |registers, memory| {
                registers.c = registers.h;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x5C, Instruction::new(
            "LD E, H", |registers, memory| {
                registers.e = registers.h;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x6C, Instruction::new(
            "LD L, H", |registers, memory| {
                registers.l = registers.h;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x7C, Instruction::new(
            "LD A, H", |registers, memory| {
                registers.a = registers.h;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x4D, Instruction::new(
            "LD C, L", |registers, memory| {
                registers.c = registers.l;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x5D, Instruction::new(
            "LD E, L", |registers, memory| {
                registers.e = registers.l;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x6D, Instruction::new(
            "LD L, L", |registers, memory| {
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x7D, Instruction::new(
            "LD A, L", |registers, memory| {
                registers.a = registers.c;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x4E, Instruction::new(
            "LD C, (HL)", |registers, memory| {
                registers.c = memory.read_byte(registers.get_hl());
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x5E, Instruction::new(
            "LD E, (HL)", |registers, memory| {
                registers.e = memory.read_byte(registers.get_hl());
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x6E, Instruction::new(
            "LD L, (HL)", |registers, memory| {
                registers.l = memory.read_byte(registers.get_hl());
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x7E, Instruction::new(
            "LD A, (HL)", |registers, memory| {
                registers.a = memory.read_byte(registers.get_hl());
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );

    instructions_map.insert(
        0x4F, Instruction::new(
            "LD C, A", |registers, memory| {
                registers.c = registers.a;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x5F, Instruction::new(
            "LD E, A", |registers, memory| {
                registers.e = registers.a;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x6F, Instruction::new(
            "LD L, A", |registers, memory| {
                registers.l = registers.a;
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0x7F, Instruction::new(
            "LD A, A", |registers, memory| {
                (true, true)
            }, Cycles::new(4), 1,
        ),
    );

    instructions_map.insert(
        0xE0, Instruction::new(
            "LDH (a8), A", |registers, memory| {
                registers.a = memory.read_byte(registers.pc + 1);
                memory.write_byte(0xFF00 | memory.read_byte(registers.pc + 1) as u16, registers.a);
                (true, true)
            }, Cycles::new(12), 2,
        ),
    );

    instructions_map.insert(
        0xF0, Instruction::new(
            "LDH A, (a8)", |registers, memory| {
                registers.a = memory.read_byte(0xFF00 | memory.read_byte(registers.pc + 1) as u16);
                (true, true)
            }, Cycles::new(12), 2,
        ),
    );

    instructions_map.insert(
        0xE2, Instruction::new(
            "LDH (C), A", |registers, memory| {
                memory.write_byte(0xFF00 | registers.c as u16, registers.a);
                (true, true)
            }, Cycles::new(8), 1,
            // NOTE(henrick): https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html said 2 bytes,
            // but https://stackoverflow.com/questions/41353869/length-of-instruction-ld-a-c-in-gameboy-z80-processor
            // said first one is wrong and it should be 1 byte
        ),
    );

    instructions_map.insert(
        0xF2, Instruction::new(
            "LDH A, (C)", |registers, memory| {
                registers.a = memory.read_byte(0xFF00 | registers.c as u16);
                (true, true)
            }, Cycles::new(8), 1,
            // NOTE(henrick): https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html said 2 bytes,
            // but https://stackoverflow.com/questions/41353869/length-of-instruction-ld-a-c-in-gameboy-z80-processor
            // said first one is wrong and it should be 1 byte
        ),
    );

    instructions_map.insert(
        0xEA, Instruction::new(
            "LD (a16), A", |registers, memory| {
                memory.write_byte(memory.read_word(registers.pc + 1), registers.a);
                (true, true)
            }, Cycles::new(16), 3,
        ),
    );

    instructions_map.insert(
        0xFA, Instruction::new(
            "LD A, (a16)", |registers, memory| {
                registers.a = memory.read_byte(memory.read_word(registers.pc + 1));
                (true, true)
            }, Cycles::new(16), 3,
        ),
    );
}

fn instructions_map_16_bit_load_instructions(instructions_map: &mut HashMap<u8, Instruction>) -> () {
    instructions_map_16_bit_load_ld_instructions(instructions_map);
    instructions_map_16_bit_load_pop_instructions(instructions_map);
    instructions_map_16_bit_load_push_instructions(instructions_map);
}

fn instructions_map_16_bit_load_ld_instructions(instructions_map: &mut HashMap<u8, Instruction>) -> () {
    instructions_map.insert(
        0x01, Instruction::new(
            "LD BC, d16", |registers, memory| {
                registers.set_bc(memory.read_word(registers.pc + 1));
                (true, true)
            }, Cycles::new(12), 3,
        ),
    );

    instructions_map.insert(
        0x11, Instruction::new(
            "LD DE, d16", |registers, memory| {
                registers.set_de(memory.read_word(registers.pc + 1));
                (true, true)
            }, Cycles::new(12), 3,
        ),
    );

    instructions_map.insert(
        0x21, Instruction::new(
            "LD HL, d16", |registers, memory| {
                registers.set_hl(memory.read_word(registers.pc + 1));
                (true, true)
            }, Cycles::new(12), 3,
        ),
    );

    instructions_map.insert(
        0x31, Instruction::new(
            "LD SP, d16", |registers, memory| {
                memory.write_word(registers.sp, memory.read_word(registers.pc + 1));
                (true, true)
            }, Cycles::new(12), 3,
        ),
    );

    instructions_map.insert(
        0x08, Instruction::new(
            "LD (a16), SP", |registers, memory| {
                memory.write_word(memory.read_word(registers.pc + 1), registers.sp);
                (true, true)
            }, Cycles::new(20), 3,
        ),
    );

    instructions_map.insert(
        0xF8, Instruction::new(
            "LD HL, SP+r8", |registers, memory| {
                let r8 = memory.read_byte(registers.pc + 1) as i16;
                let sp = registers.sp as i16;
                let sp_plus_r8 = sp.wrapping_add(r8);
                registers.f.h = ((sp & 0x0F) + (r8 & 0x0F)) & 0x10 != 0;
                registers.f.c = ((sp as u16) + (r8 as u16)) & 0x100 != 0;
                registers.set_hl(sp_plus_r8 as u16);
                (true, true)
            }, Cycles::new(12), 2,
        ),
    );

    instructions_map.insert(
        0xF9, Instruction::new(
            "LD SP, HL", |registers, memory| {
                memory.write_word(registers.sp, registers.get_hl());
                (true, true)
            }, Cycles::new(8), 1,
        ),
    );
}

fn instructions_map_16_bit_load_pop_instructions(instructions_map: &mut HashMap<u8, Instruction>) -> () {
    fn pop(registers: &mut Registers, memory: &MMU) -> u16 {
        let sp = memory.read_word(registers.sp);
        registers.sp += 2;
        sp
    }

    instructions_map.insert(
        0xC1, Instruction::new(
            "POP BC", |registers, memory| {
                let d16 = pop(registers, memory);
                registers.set_bc(d16);
                (true, true)
            }, Cycles::new(12), 1,
        ),
    );

    instructions_map.insert(
        0xD1, Instruction::new(
            "POP DE", |registers, memory| {
                let d16 = pop(registers, memory);
                registers.set_de(d16);
                (true, true)
            }, Cycles::new(12), 1,
        ),
    );

    instructions_map.insert(
        0xE1, Instruction::new(
            "POP HL", |registers, memory| {
                let d16 = pop(registers, memory);
                registers.set_hl(d16);
                (true, true)
            }, Cycles::new(12), 1,
        ),
    );

    instructions_map.insert(
        0xF1, Instruction::new(
            "POP AF", |registers, memory| {
                let d16 = pop(registers, memory);
                registers.set_af(d16);
                (true, true)
            }, Cycles::new(12), 1,
        ),
    );
}

fn instructions_map_16_bit_load_push_instructions(instructions_map: &mut HashMap<u8, Instruction>) -> () {
    fn push(registers: &mut Registers, memory: &mut MMU, d16: u16) {
        memory.write_word(registers.sp, d16);
        registers.sp += 2;
    }

    instructions_map.insert(
        0xC5, Instruction::new(
            "PUSH BC", |registers, memory| {
                push(registers, memory, registers.get_bc());
                (true, true)
            }, Cycles::new(16), 1,
        ),
    );

    instructions_map.insert(
        0xD5, Instruction::new(
            "PUSH DE", |registers, memory| {
                push(registers, memory, registers.get_de());
                (true, true)
            }, Cycles::new(16), 1,
        ),
    );

    instructions_map.insert(
        0xE5, Instruction::new(
            "PUSH HL", |registers, memory| {
                push(registers, memory, registers.get_hl());
                (true, true)
            }, Cycles::new(16), 1,
        ),
    );

    instructions_map.insert(
        0xF5, Instruction::new(
            "PUSH AF", |registers, memory| {
                push(registers, memory, registers.get_af());
                (true, true)
            }, Cycles::new(16), 1,
        ),
    );
}