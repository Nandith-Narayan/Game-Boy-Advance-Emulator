use crate::cpu::helper_functions::{compute_overflow_on_add, compute_overflow_on_sub};
use crate::memory::Memory;
use super::Cpu;

impl Cpu {
    pub fn move_immediate(&mut self, inst: u16, mem: &mut Memory) {
        let offset = inst & 0xFF;
        let rd = ((inst >> 8) & 0b111) as usize;

        let result = offset as u32;
        self.z = result == 0;
        self.n = (result & 0x80000000) != 0;
        self.r[rd] = result;
    }

    pub fn compare_immediate(&mut self, inst: u16, mem: &mut Memory) {
        let operand2 = inst & 0xFF;
        let rd = ((inst >> 8) & 0b111) as usize;

        let operand1 = self.r[rd];

        let result = operand1 as i64 - operand2  as i64;
        self.z = result as u32 == 0;
        self.n = (result & 0x80000000) != 0;
        self.c = !((result & 0x100000000) != 0); // Carry Flag on subtraction (CMP) is reversed
        self.v = compute_overflow_on_sub(operand1 as i32, operand2 as i32, result as i32);
    }

    pub fn add_immediate(&mut self, inst: u16, mem: &mut Memory) {
        let operand2 = inst & 0xFF;
        let rd = ((inst >> 8) & 0b111) as usize;

        let operand1 = self.r[rd];

        let result = operand1 as i64 + operand2 as i64;
        self.z = result as u32 == 0;
        self.n = (result & 0x80000000) != 0;
        self.c = (result & 0x100000000) != 0;
        self.v = compute_overflow_on_add(operand1 as i32, operand2 as i32, result as i32);
        self.r[rd] = result as u32;
    }

    pub fn sub_immediate(&mut self, inst: u16, mem: &mut Memory) {
        let operand2 = inst & 0xFF;
        let rd = ((inst >> 8) & 0b111) as usize;

        let operand1 = self.r[rd];

        let result = operand1 as i64 - operand2 as i64;
        self.z = result as u32 == 0;
        self.n = (result & 0x80000000) != 0;
        self.c = !((result & 0x100000000) != 0); // Carry Flag on subtraction is reversed
        self.v = compute_overflow_on_sub(operand1 as i32, operand2 as i32, result as i32);
        self.r[rd] = result as u32;
    }
}