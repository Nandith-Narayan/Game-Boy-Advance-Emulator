use crate::cpu::enums::ARMCondition::{CC, CS, EQ, GE, GT, HI, LE, LS, LT, MI, NE, PL, VC, VS};
use crate::memory::Memory;
use super::Cpu;

impl Cpu {
    pub fn conditional_branch(&mut self, inst: u16, mem: &mut Memory) {
        let condition_bits = (inst >> 8) & 0xF;
        let offset = (inst & 0xFF) as i8 as i32;

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
            self.r[15] = (self.r[15] as i32 + offset) as u32;
            self.flush_pipeline();
        }
    }
}