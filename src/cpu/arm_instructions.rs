mod branches;
mod alu_operations;
mod psr_transfers;
mod multiply;
mod single_data_transfers;
mod half_word_transfers;
mod block_data_transfers;

use crate::cpu::enums::CPUMode;
use crate::cpu::enums::CPUMode::*;
use crate::cpu::enums::InstructionSet::{ARM, THUMB};
use crate::cpu::enums::RegisterName::*;
use crate::cpu::helper_functions::num_to_cpu_mode;
use super::Cpu;


// Implementation of functions related to ARM mode of the CPU
impl Cpu {

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
            },
            FIQ => {
                self.reg_map[8] = R8FIQ as usize;
                self.reg_map[9] = R9FIQ as usize;
                self.reg_map[10] = R10FIQ as usize;
                self.reg_map[11] = R11FIQ as usize;
                self.reg_map[12] = R12FIQ as usize;
                self.reg_map[13] = R13FIQ as usize;
                self.reg_map[14] = R14FIQ as usize;

            },
            IRQ => {
                self.reg_map[8] = R8 as usize;
                self.reg_map[9] = R9 as usize;
                self.reg_map[10] = R10 as usize;
                self.reg_map[11] = R11 as usize;
                self.reg_map[12] = R12 as usize;
                self.reg_map[13] = R13IRQ as usize;
                self.reg_map[14] = R14IRQ as usize;

                self.irq_disable = true; // IRQ disable flag is always set when entering IRQ mode.
            },
            SUPERVISOR => {
                self.reg_map[8] = R8 as usize;
                self.reg_map[9] = R9 as usize;
                self.reg_map[10] = R10 as usize;
                self.reg_map[11] = R11 as usize;
                self.reg_map[12] = R12 as usize;
                self.reg_map[13] = R13SVC as usize;
                self.reg_map[14] = R14SVC as usize;

            },
            ABORT => {
                self.reg_map[8] = R8 as usize;
                self.reg_map[9] = R9 as usize;
                self.reg_map[10] = R10 as usize;
                self.reg_map[11] = R11 as usize;
                self.reg_map[12] = R12 as usize;
                self.reg_map[13] = R13ABT as usize;
                self.reg_map[14] = R14ABT as usize;

            },
            UNDEFINED => {
                self.reg_map[8] = R8 as usize;
                self.reg_map[9] = R9 as usize;
                self.reg_map[10] = R10 as usize;
                self.reg_map[11] = R11 as usize;
                self.reg_map[12] = R12 as usize;
                self.reg_map[13] = R13UND as usize;
                self.reg_map[14] = R14UND as usize;

            }
        }

        self.mode = new_mode;
    }

    // Helper function to convert CPU status into a bit field
    pub fn get_cpsr(&self) -> u32{
        let mut cpsr = 0;

        if self.n {cpsr |= 1<<31;}
        if self.z {cpsr |= 1<<30;}
        if self.c {cpsr |= 1<<29;}
        if self.v {cpsr |= 1<<28;}

        if self.instruction_set == THUMB {cpsr |= 1<<5;}
        if self.irq_disable {cpsr |= 1<<7;}

        cpsr |= self.mode as u32;

        return cpsr;
    }

    // Helper function to populate CPU flags & control bits based on CPSR bytes
    pub fn set_cpsr_and_switch_mode(&mut self, cpsr: u32){
        self.n = (cpsr & (1 << 31)) != 0;
        self.z = (cpsr & (1 << 30)) != 0;
        self.c = (cpsr & (1 << 29)) != 0;
        self.v = (cpsr & (1 << 28)) != 0;

        if (cpsr & (1<<5)) != 0{
            self.instruction_set = THUMB;
        }else{
            self.instruction_set = ARM;
        }

        self.irq_disable = (cpsr & (1 << 7)) != 0;

        let new_mode = cpsr & 0x1F;
        self.switch_mode(num_to_cpu_mode(new_mode));
    }

    pub fn get_spsr(&mut self, cpu_mode: CPUMode) -> u32{
        match cpu_mode{
            USER | SYSTEM => {
                println!("Attempted to read from SPSR while in USER/SYSTEM mode");
                // If in USER/SYSTEM mode, there is no SPSR, so use the CPSR instead.
                self.get_cpsr()
            },
            FIQ => {self.spsr_fiq},
            IRQ => {self.spsr_irq},
            SUPERVISOR => {self.spsr_svc},
            ABORT => {self.spsr_abt},
            UNDEFINED => {self.spsr_und},
        }
    }

    pub fn set_spsr(&mut self, cpu_mode: CPUMode, value: u32){
        match cpu_mode{
            USER | SYSTEM => {
                println!("Attempted to write to SPSR while in USER/SYSTEM mode");
            },
            FIQ => {self.spsr_fiq = value;},
            IRQ => {self.spsr_irq = value;},
            SUPERVISOR => {self.spsr_svc = value;},
            ABORT => {self.spsr_abt = value;},
            UNDEFINED => {self.spsr_und = value;}
        };
    }

}
