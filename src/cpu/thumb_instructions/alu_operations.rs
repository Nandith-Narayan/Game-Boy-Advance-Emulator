use crate::cpu::enums::ShiftType::*;
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
            self.r[rd as usize] = result as u32;
        }

    }

    pub fn alu_operation(&mut self, inst: u16, mem: &mut Memory){
        let rd = inst & 0b111;
        let rs = (inst >> 3) & 0b111;
        let op = (inst >> 6) & 0xF;

        let operand1 = self.r[rd as usize];
        let operand2 = self.r[rs as usize];

        let result: u32;

        match op{
            0 => {
                // AND
                result = operand1 & operand2;

                self.z = result == 0;
                self.n = (result & 0x80000000) != 0;
            },
            1 => {
                // XOR
                result = operand1 ^ operand2;

                self.z = result == 0;
                self.n = (result & 0x80000000) != 0;
            },
            2 => {
                // Logical Shift Left
                if operand2 !=0 {
                    result = self.perform_shift_op_immediate_shift(LogicalShiftLeft, operand2, operand1, true);
                }else {
                    result = operand1;
                }

                self.z = result == 0;
                self.n = (result & 0x80000000) != 0;
            },
            3 => {
                // Logical Shift Right
                if operand2 !=0 {
                    result = self.perform_shift_op_immediate_shift(LogicalShiftRight, operand2, operand1, true);
                }else {
                    result = operand1;
                }

                self.z = result == 0;
                self.n = (result & 0x80000000) != 0;
            },
            4 => {
                // Arithmetic Shift Right
                if operand2 !=0 {
                    result = self.perform_shift_op_immediate_shift(ArithmeticShiftRight, operand2, operand1, true);
                }else {
                    result = operand1;
                }
                self.z = result == 0;
                self.n = (result & 0x80000000) != 0;
            },
            5 => {
                // ADC
                let mut carry = 0;
                if self.c{
                    carry = 1;
                }
                let result_64_bit = operand1 as i64 + operand2 as i64 + carry;

                self.z = result_64_bit as u32 == 0;
                self.n = (result_64_bit & 0x80000000) != 0;
                self.c = (result_64_bit & 0x100000000) != 0;
                self.v = (((operand1 as i64 & 0x7FFFFFFF) + (operand2 as i64 & 0x7FFFFFFF) + carry) & 0x80000000) != 0;

                result = result_64_bit as u32;
            },
            6 => {
            // SBC
                let mut carry = 0;
                if self.c{
                    carry = 1;
                }
                let result_64_bit =  operand1 as i64 - operand2 as i64 + carry - 1;

                self.z = result_64_bit as u32 == 0;
                self.n = (result_64_bit & 0x80000000) != 0;
                self.c = (result_64_bit & 0x100000000) == 0; // SBC Carry flag is reversed
                self.v = (((operand1 as i64 & 0x7FFFFFFF) - (operand2 as i64 & 0x7FFFFFFF) + carry -1) & 0x80000000) != 0;

                result = result_64_bit as u32;
            },
            7 => {
                // Rotate Right
                if operand2 !=0 {
                    result = self.perform_shift_op_immediate_shift(RotateRight, operand2, operand1, true);
                }else {
                    result = operand1;
                }
                self.z = result == 0;
                self.n = (result & 0x80000000) != 0;
            },
            8 => {
                // TST
                let temp_result = operand1 & operand2;

                self.z = temp_result == 0;
                self.n = (temp_result & 0x80000000) != 0;

                result = operand1;
            },
            9 => {
                // NEG
                let result_64_bit = 0 - operand2 as i64;

                self.z = result_64_bit as u32 == 0;
                self.n = (result_64_bit & 0x80000000) != 0;
                self.c = (result_64_bit & 0x100000000) != 0;
                self.v = ((operand2 as i64 & 0x7FFFFFFF) & 0x80000000) != 0;

                result = result_64_bit as u32;
            },
            10 => {
                // CMP
                let result_64_bit = operand1 as i64 - operand2 as i64;

                self.z = result_64_bit as u32 == 0;
                self.n = (result_64_bit & 0x80000000) != 0;
                self.c = !((result_64_bit & 0x100000000) != 0); // Carry Flag on subtraction is reversed
                self.v = (((operand1 as i64 & 0x7FFFFFFF) - (operand2 as i64 & 0x7FFFFFFF)) & 0x80000000) != 0;

                result = operand1;
            },
            11 => {
                // CMN
                let result_64_bit = operand1 as i64 + operand2 as i64;

                self.z = result_64_bit as u32 == 0;
                self.n = (result_64_bit & 0x80000000) != 0;
                self.c = (result_64_bit & 0x100000000) != 0;
                self.v = (((operand1 as i64 & 0x7FFFFFFF) + (operand2 as i64 & 0x7FFFFFFF)) & 0x80000000) != 0;

                result = operand1;
            },
            12 => {
                // OR
                result = operand1 | operand2;

                self.z = result == 0;
                self.n = (result & 0x80000000) != 0;
            },
            13 => {
                // MUL
                result = ((operand1 as u64 * operand2 as u64) & 0xFFFFFFFF) as u32;

                self.z = result == 0;
                self.n = (result & 0x80000000) != 0;
            },
            14 => {
                // BIC
                result = operand1 & (!operand2);

                self.z = result == 0;
                self.n = (result & 0x80000000) != 0;
            },
            15 => {
                // MVN
                result = !operand2;

                self.z = result == 0;
                self.n = (result & 0x80000000) != 0;
            },

            _ => { result = operand1;},
        };

        self.r[rd as usize] = result;

    }

    pub fn high_register_operation(&mut self, inst: u16, mem: &mut Memory){
        let rd = ((inst >> 4) & 0b1000) | (inst & 0b111);
        let rs = ((inst >> 3) & 0b1000) | ((inst >> 3) & 0b111);
        let op = (inst >> 8) & 0b11;

        match op{
            0 => {
                // ADD
                self.r[rd as usize] = self.r[rd as usize].wrapping_add(self.r[rs as usize]);
                if rd == 15{
                    self.r[15] &= !0x1;
                    self.flush_pipeline();
                }
            },
            1 => {
                // CMP
                let operand1 = self.r[rd as usize];
                let operand2 = self.r[rs as usize];
                let result = operand1 as i64 - operand2 as i64;

                self.z = result as u32 == 0;
                self.n = (result & 0x80000000) != 0;
                self.c = !((result & 0x100000000) != 0); // Carry Flag on subtraction is reversed
                self.v = (((operand1 as i64 & 0x7FFFFFFF) - (operand2 as i64 & 0x7FFFFFFF)) & 0x80000000) != 0;
            },
            2 => {
                // MOV
                self.r[rd as usize] = self.r[rs as usize];
                if rd == 15{
                    self.r[15] &= !0x1;
                    self.flush_pipeline();
                }
            },
            _ => {}
        };
    }
}