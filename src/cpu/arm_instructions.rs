mod branches;
mod alu_operations;
mod psr_transfers;
mod multiply;
mod single_data_transfers;
mod half_word_transfers;
mod block_data_transfers;

use crate::cpu::enums::CPUMode;
use crate::cpu::enums::CPUMode::*;
use crate::cpu::enums::RegisterName::*;
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

        match new_mode{
            USER | SYSTEM => {
                self.reg_map[8] = R8 as usize;
                self.reg_map[9] = R9 as usize;
                self.reg_map[10] = R10 as usize;
                self.reg_map[11] = R11 as usize;
                self.reg_map[12] = R12 as usize;
                self.reg_map[13] = R13 as usize;
                self.reg_map[14] = R14 as usize;
            }
            FIQ => {
                self.reg_map[8] = R8FIQ as usize;
                self.reg_map[9] = R9FIQ as usize;
                self.reg_map[10] = R10FIQ as usize;
                self.reg_map[11] = R11FIQ as usize;
                self.reg_map[12] = R12FIQ as usize;
                self.reg_map[13] = R13FIQ as usize;
                self.reg_map[14] = R14FIQ as usize;

                let temp = self.get_cpsr();
                self.set_cpsr(self.spsr_fiq);
                self.spsr_fiq = temp;
            },
            IRQ => {
                self.reg_map[8] = R8 as usize;
                self.reg_map[9] = R9 as usize;
                self.reg_map[10] = R10 as usize;
                self.reg_map[11] = R11 as usize;
                self.reg_map[12] = R12 as usize;
                self.reg_map[13] = R13IRQ as usize;
                self.reg_map[14] = R14IRQ as usize;

                let temp = self.get_cpsr();
                self.set_cpsr(self.spsr_fiq);
                self.spsr_fiq = temp;
            },
            SUPERVISOR => {
                self.reg_map[8] = R8 as usize;
                self.reg_map[9] = R9 as usize;
                self.reg_map[10] = R10 as usize;
                self.reg_map[11] = R11 as usize;
                self.reg_map[12] = R12 as usize;
                self.reg_map[13] = R13SVC as usize;
                self.reg_map[14] = R14SVC as usize;

                let temp = self.get_cpsr();
                self.set_cpsr(self.spsr_svc);
                self.spsr_svc = temp;
            },
            ABORT => {
                self.reg_map[8] = R8 as usize;
                self.reg_map[9] = R9 as usize;
                self.reg_map[10] = R10 as usize;
                self.reg_map[11] = R11 as usize;
                self.reg_map[12] = R12 as usize;
                self.reg_map[13] = R13ABT as usize;
                self.reg_map[14] = R14ABT as usize;

                let temp = self.get_cpsr();
                self.set_cpsr(self.spsr_abt);
                self.spsr_abt = temp;
            },
            UNDEFINED => {
                self.reg_map[8] = R8 as usize;
                self.reg_map[9] = R9 as usize;
                self.reg_map[10] = R10 as usize;
                self.reg_map[11] = R11 as usize;
                self.reg_map[12] = R12 as usize;
                self.reg_map[13] = R13UND as usize;
                self.reg_map[14] = R14UND as usize;

                let temp = self.get_cpsr();
                self.set_cpsr(self.spsr_und);
                self.spsr_und = temp;
            },

            _ => {println!("Failed to switch to mode {:?}", new_mode);}
        }

        self.mode = new_mode;
    }

}
