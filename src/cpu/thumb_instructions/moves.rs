use crate::cpu::enums::ShiftType::{ArithmeticShiftRight, LogicalShiftLeft, LogicalShiftRight};
use crate::memory::Memory;
use super::Cpu;

impl Cpu {
    pub fn move_shifted_register(&mut self, inst: u16, mem: &mut Memory) {
        let rd = inst & 0b111;
        let rs = (inst >> 3) & 0b111;
        let offset = (inst >> 6) & 0x1F;
        let op = (inst >> 11) & 0b11;

        let op1 = self.get_r(rs as usize);

        match op{
            // Logical Shift Left
            0 =>{
                let val = self.perform_shift_op_immediate_shift(LogicalShiftLeft, offset as u32, op1, true);
                self.z = val == 0;
                self.set_r(rd as usize, val);
            },

            // Logical Shift Right
            1 =>{
                let val = self.perform_shift_op_immediate_shift(LogicalShiftRight, offset as u32, op1, true);
                self.z = val == 0;
                self.set_r(rd as usize, val);
            },

            // Arithmetic Shift Right
            2 =>{
                let val = self.perform_shift_op_immediate_shift(ArithmeticShiftRight, offset as u32, op1, true);
                self.z = val == 0;
                self.set_r(rd as usize, val);
            },
            _ => {}
        }
    }
}