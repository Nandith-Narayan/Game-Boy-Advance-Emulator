mod branches;
mod alu_operations;
mod psr_transfers;
mod multiply;
mod single_data_transfers;

use crate::cpu::enums::CPUMode;
use crate::cpu::enums::CPUMode::*;
use super::Cpu;


// Implementation of functions related to ARM mode of the CPU
impl Cpu {

    pub fn num_to_cpu_mode(&mut self, n: u32) -> CPUMode{
       return match n{
            0b10000 => USER,
            0b10001 => FIQ,
            0b10010 => IRQ,
            0b10011 => SUPERVISOR,
            0b10111 => ABORT,
            0b11011 => UNDEFINED,
            0b11111 => SYSTEM,
            _ => UNDEFINED,
        }
    }

    pub fn switch_mode(&mut self, new_mode: CPUMode){

        println!("Mode swap triggered: {:?} -> {:?}", self.mode, new_mode);

        let mode_swap = if new_mode == USER || new_mode == SYSTEM {self.mode} else {new_mode};

        match mode_swap{
            FIQ => {
                std::mem::swap(&mut self.r[8], &mut self.r_fiq[8]);
                std::mem::swap(&mut self.r[9], &mut self.r_fiq[9]);
                std::mem::swap(&mut self.r[10], &mut self.r_fiq[10]);
                std::mem::swap(&mut self.r[11], &mut self.r_fiq[11]);
                std::mem::swap(&mut self.r[12], &mut self.r_fiq[12]);
                std::mem::swap(&mut self.r[13], &mut self.r_fiq[13]);
                std::mem::swap(&mut self.r[14], &mut self.r_fiq[14]);

                let temp = self.get_cpsr();
                self.set_cpsr(self.spsr_fiq);
                self.spsr_fiq = temp;
            },
            IRQ => {
                std::mem::swap(&mut self.r[13], &mut self.r_fiq[13]);
                std::mem::swap(&mut self.r[14], &mut self.r_fiq[14]);

                let temp = self.get_cpsr();
                self.set_cpsr(self.spsr_fiq);
                self.spsr_fiq = temp;
            },

            _ => {println!("Failed to switch to mode {:?}", new_mode);}
        }

        self.mode = new_mode;
    }

}
