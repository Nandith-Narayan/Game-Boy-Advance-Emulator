use crate::cpu::enums::InstructionSet::{ARM};
use crate::memory::Memory;
use super::Cpu;

impl Cpu {
    pub fn conditional_branch(&mut self, inst: u16, mem: &mut Memory) {
        let condition_bits = (inst >> 8) & 0xF;
        let offset = (inst & 0xFF) as i8 as i64;

        let condition = match condition_bits {
            0 => self.z,
            1 => !self.z,
            2 => self.c,
            3 => !self.c,
            4 => self.n,
            5 => !self.n,
            6 => self.v,
            7 => !self.v,
            8 => self.c && !self.z,
            9 => !self.c || self.z,
            10 => self.n == self.v,
            11 => self.n != self.v,
            12 => !self.z && (self.n == self.v),
            13 => self.z || (self.n != self.v),
            _ => true,
        };

        if condition {
            self.set_r(15, ((self.get_r(15)) as i64 + (offset << 1)) as u32);
            self.flush_pipeline();
        }
    }
    pub fn thumb_branch_and_exchange(&mut self, inst: u16, mem: &mut Memory) {
        let rs = ((inst >> 3) & 0b1000)| ((inst >> 3) & 0b111);
        self.set_r(15, self.get_r(rs as usize) & 0xFFFFFFFE);
        if self.get_r(rs as usize) & 0x1 == 0{

            self.decrement_r(15, 2);

            self.instruction_set = ARM;
        }
        self.flush_pipeline();

    }

    pub fn thumb_branch(&mut self, inst: u16, mem: &mut Memory) {
        let mut offset = (inst & 0x7FF) << 1;
        // Sign Extend 12 bit value
        if offset & 0x0800 != 0{
            offset |= 0xF000;
        }

        self.set_r(15, ((self.get_r(15) as i32) + (offset as i16 as i32)) as u32);
        self.flush_pipeline();
    }

    pub fn branch_and_link(&mut self, inst: u16, mem: &mut Memory){
        let offset = inst & 0x7FF;
        let h_bit = (inst >> 11) & 0x1 != 0;

        if h_bit {
            let address = ((offset as u32) << 1) + self.get_r(14);
            self.set_r(14, self.get_r(15)-2 | 0x1);
            self.set_r(15, address);
            self.flush_pipeline();
        }else{
            let mut sign_extended_offset = offset;
            if offset & 0x400 !=0{
                sign_extended_offset |= 0xF800;
            }
            let address = ((sign_extended_offset as i16 as i32) << 12 )+ (self.get_r(15) as i32);
            self.set_r(14, address as u32);
        }
    }
}