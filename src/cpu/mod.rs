use std::thread;
use std::time::{Duration, Instant};

use log::{debug, error, warn};

use crate::cpu::instructions::{Instruction, InstructionsMapsManager};
use crate::cpu::registers::{CpuState, Registers};
use crate::mmu::MMU;

mod instructions;
mod registers;

fn cycles_to_time(cycles: u8) -> Duration {
    let clock_speed = 4_194_304.0;
    let time = cycles as f64 / clock_speed;
    let seconds = time.floor() as u64;
    let nanoseconds = ((time - seconds as f64) * 1_000_000_000.0) as u32;
    Duration::new(seconds, nanoseconds)
}

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
        debug!("{:?}", self.registers);

        if self.registers.cpu_state == CpuState::Running {
            let start_time = Instant::now();
            let pc = self.registers.pc;
            let byte = self.fetch(mmu);
            debug!("Fetch:   @0x{:0>4x} -> 0x{:0>2x}", pc, byte);

            let instruction = match self.decode(byte) {
                Some(instruction) => instruction,
                None => {
                    error!("Fatal: Unknown instruction for byte 0x{:0>2x}", byte);
                    std::process::exit(1);
                }
            };
            debug!("Decode:  0x{:0>2x} = {:?}", byte, instruction.mnemonic);

            let (pc_update, cycles) = self.execute(&instruction, mmu);

            debug!("Execute: {:?} : {} cycles", instruction.mnemonic, cycles);

            if pc_update {
                self.registers.pc += instruction.bytes as u16;
            }
            let cur_time = Instant::now();
            let delta_time = cur_time - start_time;
            let step_duration = cycles_to_time(cycles);
            if step_duration < delta_time {
                warn!("Overshoot: {} ns > {} ns", delta_time.as_nanos(), step_duration.as_nanos());
            } else {
                let wait_for = step_duration - delta_time;
                thread::sleep(wait_for);
            }
        }
    }
}