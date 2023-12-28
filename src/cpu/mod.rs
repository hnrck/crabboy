use crate::cpu::instructions::{Instruction, InstructionsMapsManager};
use crate::cpu::registers::{CpuState, Registers};
use crate::mmu::MMU;

mod instructions;
mod registers;

pub(crate) struct CPU {
    registers: Registers,
    instructions_maps_manager: InstructionsMapsManager,
}

impl CPU {
    pub(crate) fn new() -> Self {
        CPU {
            registers: Registers::new(),
            instructions_maps_manager: InstructionsMapsManager::new(),
        }
    }

    fn fetch(&mut self, mmu: &MMU) -> u8 { mmu.read_byte(self.registers.pc) }

    fn decode(&mut self, byte: u8) -> Option<Instruction> {
        let instruction = self.instructions_maps_manager.get_instruction_map().get(&byte).cloned();
        if byte == 0xCB && self.instructions_maps_manager.is_default_map() {
            self.instructions_maps_manager.set_prefix_cb_state();
        } else {
            self.instructions_maps_manager.reset_state();
        };
        instruction
    }

    fn execute(&mut self, instruction: &Instruction, mmu: &mut MMU) -> (bool, u8) {
        let (pc_update, action_taken) = (instruction.execute)(&mut self.registers, mmu);
        match action_taken {
            true => (pc_update, instruction.cycles.taken),
            false => (pc_update, instruction.cycles.not_taken),
        }
    }

    pub(crate) fn step(&mut self, mmu: &mut MMU) {
        println!("{:?}", self.registers);

        if self.registers.cpu_state == CpuState::Running {
            let pc = self.registers.pc;
            let byte = self.fetch(mmu);
            println!("Fetch:   @0x{:0>4x} -> 0x{:0>2x}", pc, byte);

            let instruction = self.decode(byte).expect("Unknown instruction");
            println!("Decode:  0x{:0>2x} = {:?}", byte, instruction.mnemonic);

            let (pc_update, cycles) = self.execute(&instruction, mmu);

            println!("Execute: {:?} : {} cycles", instruction.mnemonic, cycles);

            if pc_update {
                self.registers.pc += instruction.bytes as u16;
            }
        }

        println!("---");
    }
}