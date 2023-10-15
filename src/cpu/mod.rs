mod instructions;
mod registers;

use std::collections::HashMap;
use crate::mmu::MMU;
use crate::cpu::registers::Registers;
use crate::cpu::instructions::{Instruction, initialize_instructions_map};

pub(crate) struct CPU {
    registers: Registers,
    instructions_map: HashMap<u8, Instruction>,
}

impl CPU {
    pub(crate) fn new() -> Self {
        CPU {
            registers: Registers::new(),
            instructions_map: initialize_instructions_map(),
        }
    }

    fn fetch(&mut self, mmu: &MMU) -> u8 { mmu.read_byte(self.registers.pc) }

    fn decode(&self, byte: u8) -> Option<Instruction> {
        self.instructions_map.get(&byte).cloned()
    }

    fn execute(&mut self, instruction: &Instruction, mmu: &mut MMU) -> u8 {
        match (instruction.execute)(&mut self.registers, mmu) {
            true => instruction.cycles_taken,
            false => instruction.cycles_not_taken.unwrap_or(instruction.cycles_taken),
        }
    }

    pub(crate) fn step(&mut self, mmu: &mut MMU) {
        println!("{:?}", self.registers);

        let pc = self.registers.pc;
        let byte = self.fetch(mmu);
        println!("Fetch:   @0x{:0>4x} -> 0x{:0>2x}", pc, byte);

        let instruction = self.decode(byte).expect("Unknown instruction");
        println!("Decode:  0x{:0>2x} = {:?}", byte, instruction.mnemonic);

        let cycles = self.execute(&instruction, mmu);

        println!("Execute: {:?} : {} cycles", instruction.mnemonic, cycles);

        self.registers.pc += instruction.bytes as u16;

        println!("---");
    }
}