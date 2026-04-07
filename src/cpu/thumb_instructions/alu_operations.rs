use crate::memory::Memory;
use super::Cpu;

impl Cpu{

    pub fn add_or_subtract(&mut self, inst: u16, mem: &mut Memory){
        let rd = inst & 0b111;
        let rs = (inst >> 3) & 0b111;
        let reg_or_offset = (inst >> 6) & 0b111;
        let is_sub = (inst >> 9) & 0x1 != 0;
        let is_immediate = (inst >> 10) & 0x1 != 0;

        let operand1 = self.r[rs as usize];
        let operand2 = if is_immediate { reg_or_offset as u32 } else { self.r[reg_or_offset as usize] };

        if is_sub {
            let result = operand1 as i64 - operand2 as i64;
            self.z = result as u32 == 0;
            self.n = (result & 0x80000000) != 0;
            self.c = !((result & 0x100000000) != 0); // Carry Flag on subtraction is reversed
            self.v = (((operand1 as i64 & 0x7FFFFFFF) - (operand2 as i64 & 0x7FFFFFFF)) & 0x80000000) != 0;
            self.r[rd as usize] = result as u32;
        }else{
            let result = operand1 as i64 + operand2 as i64;
            self.z = result as u32 == 0;
            self.n = (result & 0x80000000) != 0;
            self.c = (result & 0x100000000) != 0;
            self.v = (((operand1 as i64 & 0x7FFFFFFF) + (operand2 as i64 & 0x7FFFFFFF)) & 0x80000000) != 0;
        }

    }
}