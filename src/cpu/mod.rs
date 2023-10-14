mod opcodes;
mod registers;

use std::collections::HashMap;
use crate::mmu::MMU;
use crate::cpu::registers::Registers;
use crate::cpu::opcodes::{Instruction, Opcode, initialize_opcodes_instructions_map};

pub(crate) struct CPU {
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
        let execute_fn = instruction.execute;
        let _taken = execute_fn(&mut self.registers, mmu);
        // TODO(henrick) cycles simulation taken / not taken
        return instruction.bytes;
    }

    pub(crate) fn step(&mut self, mmu: &mut MMU) {
        println!("{:?}", self.registers);
        let byte = self.fetch(mmu);
        println!("Fetch:   @0x{:0>4x} -> 0x{:0>2x}", self.registers.pc, byte);
        let opcode = self.decode(byte);
        println!("Decode:  0x{:0>2x} = {:?}", byte, opcode);
        let bytes = self.execute(opcode, mmu);
        // TODO(henrick) update program counter, handle interrupts, etc.
        println!("Execute: {:?} : {} bytes", opcode, bytes);
        self.registers.pc += bytes as u16;
        println!("---");
    }
}