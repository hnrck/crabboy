use crate::cpu::instructions::{Cycles, ExecutionResult, Instruction, InstructionsMap};
use crate::cpu::registers::Registers;
use crate::mmu::MMU;

pub(super) fn instructions_map_jump_commands(instructions_map: &mut InstructionsMap) -> () {
    instructions_map_jump_commands_jp(instructions_map);
    instructions_map_jump_commands_jr(instructions_map);
    instructions_map_jump_commands_ret(instructions_map);
    instructions_map_jump_commands_call(instructions_map);
    instructions_map_jump_commands_rst(instructions_map);
}

fn instructions_map_jump_commands_jp(instructions_map: &mut InstructionsMap) -> () {
    fn jp(registers: &mut Registers, address: u16) { registers.pc = address }

    instructions_map.insert(
        0xc2, Instruction::new(
            "JP NZ, a16", |registers, memory| {
                if !registers.f.z {
                    jp(registers, memory.read_word(registers.pc + 1));
                    ExecutionResult::default().without_pc_update()
                } else {
                    ExecutionResult::default().without_action()
                }
            }, Cycles::new(16).with_not_taken(12), 3,
        ),
    );

    instructions_map.insert(
        0xc3, Instruction::new(
            "JP a16", |registers, memory| {
                jp(registers, memory.read_word(registers.pc + 1));
                ExecutionResult::default().without_pc_update()
            }, Cycles::new(16), 3,
        ),
    );

    instructions_map.insert(
        0xca, Instruction::new(
            "JP Z, a16", |registers, memory| {
                if registers.f.z {
                    jp(registers, memory.read_word(registers.pc + 1));
                    ExecutionResult::default().without_pc_update()
                } else {
                    ExecutionResult::default().without_action()
                }
            }, Cycles::new(16).with_not_taken(12), 3,
        ),
    );

    instructions_map.insert(
        0xd2, Instruction::new(
            "JP NC, a16", |registers, memory| {
                if !registers.f.c {
                    jp(registers, memory.read_word(registers.pc + 1));
                    ExecutionResult::default().without_pc_update()
                } else {
                    ExecutionResult::default().without_action()
                }
            }, Cycles::new(16).with_not_taken(12), 3,
        ),
    );

    instructions_map.insert(
        0xda, Instruction::new(
            "JP C, a16", |registers, memory| {
                if registers.f.c {
                    jp(registers, memory.read_word(registers.pc + 1));
                    ExecutionResult::default().without_pc_update()
                } else {
                    ExecutionResult::default().without_action()
                }
            }, Cycles::new(16).with_not_taken(12), 3,
        ),
    );

    instructions_map.insert(
        0xe9, Instruction::new(
            "JP (HL)", |registers, memory| {
                jp(registers, memory.read_word(registers.get_hl()));
                ExecutionResult::default().without_pc_update()
            }, Cycles::new(4), 1,
        ),
    );
}

fn instructions_map_jump_commands_jr(instructions_map: &mut InstructionsMap) -> () {
    fn jr(registers: &mut Registers, offset: i8) { registers.pc = ((registers.pc as i16) + (offset as i16)) as u16 }

    instructions_map.insert(
        0x18, Instruction::new(
            "JR, r8", |registers, memory| {
                jr(registers, memory.read_byte(registers.pc + 1) as i8);
                ExecutionResult::default().without_pc_update()
            }, Cycles::new(12), 2,
        ),
    );

    instructions_map.insert(
        0x20, Instruction::new(
            "JR NZ, r8", |registers, memory| {
                if !registers.f.z {
                    jr(registers, memory.read_byte(registers.pc + 1) as i8);
                    ExecutionResult::default().without_pc_update()
                } else {
                    ExecutionResult::default().without_action()
                }
            }, Cycles::new(12).with_not_taken(8), 2,
        ),
    );

    instructions_map.insert(
        0x28, Instruction::new(
            "JR Z, r8", |registers, memory| {
                if registers.f.z {
                    jr(registers, memory.read_byte(registers.pc + 1) as i8);
                    ExecutionResult::default().without_pc_update()
                } else {
                    ExecutionResult::default().without_action()
                }
            }, Cycles::new(12).with_not_taken(8), 2,
        ),
    );

    instructions_map.insert(
        0x30, Instruction::new(
            "JR NC, r8", |registers, memory| {
                if !registers.f.c {
                    jr(registers, memory.read_byte(registers.pc + 1) as i8);
                    ExecutionResult::default().without_pc_update()
                } else {
                    ExecutionResult::default().without_action()
                }
            }, Cycles::new(12).with_not_taken(8), 2,
        ),
    );

    instructions_map.insert(
        0x38, Instruction::new(
            "JR C, r8", |registers, memory| {
                if registers.f.c {
                    jr(registers, memory.read_byte(registers.pc + 1) as i8);
                    ExecutionResult::default().without_pc_update()
                } else {
                    ExecutionResult::default().without_action()
                }
            }, Cycles::new(12).with_not_taken(8), 2,
        ),
    );
}

fn instructions_map_jump_commands_ret(instructions_map: &mut InstructionsMap) -> () {
    fn ret(registers: &mut Registers, memory: &mut MMU) {
        let new_pc = memory.read_word(registers.sp);
        registers.sp += 2;
        registers.pc = new_pc;
    }

    instructions_map.insert(
        0xc9, Instruction::new(
            "RET", |registers, memory| {
                ret(registers, memory);
                ExecutionResult::default().without_pc_update()
            }, Cycles::new(16), 1,
        ),
    );

    instructions_map.insert(
        0xd9, Instruction::new(
            "RETI", |registers, memory| {
                ret(registers, memory);
                registers.enable_interrupts();
                ExecutionResult::default().without_pc_update()
            }, Cycles::new(16), 1,
        ),
    );

    instructions_map.insert(
        0xc0, Instruction::new(
            "RET NZ", |registers, memory| {
                if !registers.f.z {
                    ret(registers, memory);
                    ExecutionResult::default().without_pc_update()
                } else {
                    ExecutionResult::default().without_action()
                }
            }, Cycles::new(20).with_not_taken(8), 1,
        ),
    );

    instructions_map.insert(
        0xc8, Instruction::new(
            "RET Z", |registers, memory| {
                if registers.f.z {
                    ret(registers, memory);
                    ExecutionResult::default().without_pc_update()
                } else {
                    ExecutionResult::default().without_action()
                }
            }, Cycles::new(20).with_not_taken(8), 1,
        ),
    );

    instructions_map.insert(
        0xd0, Instruction::new(
            "RET NC", |registers, memory| {
                if !registers.f.c {
                    ret(registers, memory);
                    ExecutionResult::default().without_pc_update()
                } else {
                    ExecutionResult::default().without_action()
                }
            }, Cycles::new(20).with_not_taken(8), 1,
        ),
    );

    instructions_map.insert(
        0xd8, Instruction::new(
            "RET C", |registers, memory| {
                if registers.f.c {
                    ret(registers, memory);
                    ExecutionResult::default().without_pc_update()
                } else {
                    ExecutionResult::default().without_action()
                }
            }, Cycles::new(20).with_not_taken(8), 1,
        ),
    );
}

fn instructions_map_jump_commands_call(instructions_map: &mut InstructionsMap) -> () {
    fn call(registers: &mut Registers, memory: &mut MMU, address: u16) {
        registers.sp -= 2;
        memory.write_word(registers.sp, registers.pc + 3);
        registers.pc = address;
    }

    instructions_map.insert(
        0xcd, Instruction::new(
            "CALL a16", |registers, memory| {
                call(registers, memory, memory.read_word(registers.pc + 1));
                ExecutionResult::default().without_pc_update()
            }, Cycles::new(24), 3,
        ),
    );

    instructions_map.insert(
        0xcc, Instruction::new(
            "CALL Z, a16", |registers, memory| {
                if registers.f.z {
                    call(registers, memory, memory.read_word(registers.pc + 1));
                    ExecutionResult::default().without_pc_update()
                } else {
                    ExecutionResult::default().without_action()
                }
            }, Cycles::new(24).with_not_taken(12), 3,
        ),
    );

    instructions_map.insert(
        0xdc, Instruction::new(
            "CALL C, a16", |registers, memory| {
                if registers.f.c {
                    call(registers, memory, memory.read_word(registers.pc + 1));
                    ExecutionResult::default().without_pc_update()
                } else {
                    ExecutionResult::default().without_action()
                }
            }, Cycles::new(24).with_not_taken(12), 3,
        ),
    );
}

fn instructions_map_jump_commands_rst(instructions_map: &mut InstructionsMap) -> () {
    fn rst(registers: &mut Registers, memory: &mut MMU, address: u16) {
        registers.sp -= 2;
        memory.write_word(registers.sp, registers.pc + 1);
        registers.pc = address;
    }

    instructions_map.insert(
        0xc7, Instruction::new(
            "RST 00H", |registers, memory| {
                rst(registers, memory, 0x00);
                ExecutionResult::default().without_pc_update()
            }, Cycles::new(16), 1,
        ),
    );

    instructions_map.insert(
        0xd7, Instruction::new(
            "RST 10H", |registers, memory| {
                rst(registers, memory, 0x10);
                ExecutionResult::default().without_pc_update()
            }, Cycles::new(16), 1,
        ),
    );

    instructions_map.insert(
        0xe7, Instruction::new(
            "RST 20H", |registers, memory| {
                rst(registers, memory, 0x20);
                ExecutionResult::default().without_pc_update()
            }, Cycles::new(16), 1,
        ),
    );

    instructions_map.insert(
        0xf7, Instruction::new(
            "RST 30H", |registers, memory| {
                rst(registers, memory, 0x30);
                ExecutionResult::default().without_pc_update()
            }, Cycles::new(16), 1,
        ),
    );

    instructions_map.insert(
        0xcf, Instruction::new(
            "RST 08H", |registers, memory| {
                rst(registers, memory, 0x08);
                ExecutionResult::default().without_pc_update()
            }, Cycles::new(16), 1,
        ),
    );

    instructions_map.insert(
        0xdf, Instruction::new(
            "RST 18H", |registers, memory| {
                rst(registers, memory, 0x18);
                ExecutionResult::default().without_pc_update()
            }, Cycles::new(16), 1,
        ),
    );

    instructions_map.insert(
        0xef, Instruction::new(
            "RST 28H", |registers, memory| {
                rst(registers, memory, 0x28);
                ExecutionResult::default().without_pc_update()
            }, Cycles::new(16), 1,
        ),
    );

    instructions_map.insert(
        0xff, Instruction::new(
            "RST 38H", |registers, memory| {
                rst(registers, memory, 0x38);
                ExecutionResult::default().without_pc_update()
            }, Cycles::new(16), 1,
        ),
    );
}