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
        self.v = (((operand1 as i64 & 0x7FFFFFFF) - (operand2 as i64 & 0x7FFFFFFF)) & 0x80000000) != 0;
    }

    pub fn add_immediate(&mut self, inst: u16, mem: &mut Memory) {
        let operand2 = inst & 0xFF;
        let rd = ((inst >> 8) & 0b111) as usize;

        let operand1 = self.r[rd];

        let result = operand1 as i64 + operand2 as i64;
        self.z = result as u32 == 0;
        self.n = (result & 0x80000000) != 0;
        self.c = (result & 0x100000000) != 0;
        self.v = (((operand1 as i64 & 0x7FFFFFFF) + (operand2 as i64 & 0x7FFFFFFF)) & 0x80000000) != 0;
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
        self.v = (((operand1 as i64 & 0x7FFFFFFF) - (operand2 as i64 & 0x7FFFFFFF)) & 0x80000000) != 0;
        self.r[rd] = result as u32;
    }
}