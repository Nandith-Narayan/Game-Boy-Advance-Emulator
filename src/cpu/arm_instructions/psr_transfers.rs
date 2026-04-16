use crate::cpu::enums::CPUMode::{FIQ, IRQ, SUPERVISOR, USER};
use super::Cpu;

impl Cpu{

    pub fn transfer_to_program_status_register(&mut self, inst: u32) {
        let operand_is_immediate = (inst & (1 << 25)) != 0;
        let set_flags = (inst & (1 << 19)) != 0;
        let write_to_control_bits = (inst & (1 << 16)) != 0;
        let operand;
        if operand_is_immediate {
            let rotate = (inst & 0xF00) >> 8;
            let immediate = inst & 0xFF;
            operand = immediate.rotate_right(rotate * 2);
        }else{
            let rm = inst & 0xF;
            operand = self.get_r(rm as usize);
        }

        let destination_is_cpsr = (inst & (1 << 22)) == 0;

        if destination_is_cpsr {
            if set_flags {
                self.n = (operand & (1 << 31)) != 0;
                self.z = (operand & (1 << 30)) != 0;
                self.c = (operand & (1 << 29)) != 0;
                self.v = (operand & (1 << 28)) != 0;
            }
            if write_to_control_bits {
                println!("operand = {:#4x}", operand);
                let new_mode = self.num_to_cpu_mode(operand & 0x01F);
                self.switch_mode(new_mode);
            }

        }else{
            // Mask off bits we want to set. Use the negation of this mask to select the bits we want to keep.
            let mut mask = 0u32;
            if set_flags {
                mask |= 0xF0000000;
            }
            if write_to_control_bits {
                mask |= 0x000000FF
            }


            match self.mode{
                USER => {println!("CPU user mode doesn't have a SPSR");},
                FIQ => {
                    self.spsr_fiq = (self.spsr_fiq & (!mask)) | (operand & mask);
                },
                IRQ => {
                    self.spsr_irq = (self.spsr_irq & (!mask)) | (operand & mask);
                },
                SUPERVISOR => {
                    self.spsr_svc = (self.spsr_svc & (!mask)) | (operand & mask);
                },
                _ => {println!("failed to set CPU {:?} mode's SPSR.", self.mode);},
            }
        }
    }

    pub fn transfer_from_program_status_register(&mut self, inst: u32) {
        let source_is_cpsr = (inst & (1 << 22)) == 0;
        let rd = (inst >> 12) & 0xF;

        if !source_is_cpsr {
            //println!("TRANSFER FROM SPSR NOT IMPLEMENTED");
            if self.mode == IRQ{
                self.set_r(rd as usize, self.spsr_irq);
            }else if self.mode == FIQ {
                self.set_r(rd as usize, self.spsr_fiq);
            }else if self.mode == SUPERVISOR {
                self.set_r(rd as usize, self.spsr_svc);
            }else{
                println!("TRANSFER FROM SPSR NOT IMPLEMENTED FOR MODE {:?}", self.mode);
            }
        }else {
            self.set_r(rd as usize, self.get_cpsr());
        }

    }
}