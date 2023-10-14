mod opcodes;
mod registers;

use std::collections::HashMap;
use crate::mmu::MMU;
use crate::cpu::registers::Registers;
use crate::cpu::opcodes::{Instruction, Opcode, initialize_opcodes_instructions_map};

struct CPU {
    registers: Registers,
    opcodes_instructions_map: HashMap<Opcode, Instruction>,
}

impl CPU {
    pub(crate) fn new() -> Self {
        CPU {
            registers: Registers::new(),
            opcodes_instructions_map: initialize_opcodes_instructions_map(),
        }
    }

    fn fetch(&self, mmu: &MMU) -> u8 { mmu.read_byte(self.registers.pc) }

    fn decode(&self, byte: u8) -> Opcode { Opcode::from_byte(byte) }

    fn execute(&mut self, opcode: Opcode, mmu: &mut MMU) -> u8 {
        let instruction: &Instruction = self.opcodes_instructions_map.get(&opcode).unwrap();
        println!("{}", instruction.mnemonic);
        let execute_fn = instruction.execute;
        let _taken = execute_fn(&mut self.registers, mmu);
        // TODO(henrick) cycles simulation taken / not taken
        return instruction.bytes;
    }

    fn step(&mut self, mmu: &mut MMU) {
        let byte = self.fetch(mmu);
        let opcode = self.decode(byte);
        let bytes = self.execute(opcode, mmu);
        // TODO(henrick) update program counter, handle interrupts, etc.
        self.registers.pc += bytes as u16;
    }
}