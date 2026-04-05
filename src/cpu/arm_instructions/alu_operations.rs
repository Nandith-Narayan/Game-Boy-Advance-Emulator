use crate::cpu::enums::CPUMode::FIQ;
use super::Cpu;

impl Cpu{

    pub fn data_processing_register_operand(&mut self, inst: u32) {
        let rn = (inst & 0xF0000) >> 16;
        let rd = (inst & 0xF000) >> 12;
        let rm = inst & 0xF;

        let mut operand1 = self.r[rn as usize];
        let mut operand2 = self.r[rm as usize];

        let set_flags = (inst & (1 << 20)) != 0;

        // register shift
        let shift_inst = (inst & 0xFF0) >> 4;
        if shift_inst & 0x1 == 0{
            // shift by immediate value
            let shift_type = (shift_inst>>1) & 0b11;
            let shift_amount = (shift_inst >> 3) & 0x1F;
            operand2 = self.perform_shift_op_immediate_shift(shift_type, shift_amount, operand2);
        }else{
            // Shifting normally takes longer to execute,
            // so the instruction pipeline would've incremented the PC by 4.
            // If the PC (R[15]) is one of the operands,
            // then increase the operand by 4 to compensate.
            if rn == 15{
                operand1 += 4;

            }else if rm == 15{
                operand2 += 4;
            }
            // shift by register value
            operand2 = self.perform_shift_op_register_shift(shift_inst, operand2);
        }

        self.alu_operations(inst, operand1, operand2, rd, set_flags);
    }

    pub fn data_processing_immediate_operand(&mut self, inst: u32){
        let set_flags = (inst & (1 << 20)) != 0;
        let rn = (inst & 0xF0000) >> 16;
        let rd = (inst & 0xF000) >> 12;
        let immediate = inst & 0xFF;
        let rotate = (inst & 0xF00)>> 8;

        let operand1 = self.r[rn as usize];
        let operand2 = immediate.rotate_right(rotate*2);
        // update the carry flag only if the shifter was used to rotate the immediate value
        if rotate != 0{
            self.c = (operand2 & 0x80000000) != 0;
        }

        self.alu_operations(inst, operand1, operand2, rd, set_flags);
    }

    pub fn alu_operations(&mut self, inst:u32, operand1:u32, operand2:u32, rd:u32, set_flags:bool){
        match (inst & 0x1E00000) >> 21 {
            0 => {
                // AND
                let result = operand1 & operand2;

                self.z = result == 0;
                self.n = (result & 0x80000000) != 0;
                self.r[rd as usize] = result;
            }
            1 => {
                // XOR
                let result = operand1 ^ operand2;

                self.z = result == 0;
                self.n = (result & 0x80000000) != 0;
                self.r[rd as usize] = result;
            }
            2 => {
                // SUB
                let result = operand1 as i64 - operand2 as i64;
                if set_flags {

                    self.z = result as u32 == 0;
                    self.n = (result & 0x80000000) != 0;
                    self.c = !((result & 0x100000000) != 0); // Carry Flag on subtraction is reversed
                    self.v = (((operand1 as i64 & 0x7FFFFFFF) - (operand2 as i64 & 0x7FFFFFFF)) & 0x80000000) != 0;
                }
                self.r[rd as usize] = result as u32;
            },
            3 => {
                // RSB
                let result = operand2 as i64 - operand1 as i64;
                if set_flags {

                    self.z = result as u32 == 0;
                    self.n = (result & 0x80000000) != 0;
                    self.c = (result & 0x100000000) != 0;
                    self.v = (((operand2 as i64 & 0x7FFFFFFF) - (operand1 as i64 & 0x7FFFFFFF)) & 0x80000000) != 0;
                }
                self.r[rd as usize] = result as u32;
            },
            4 => {
                // ADD
                let result = operand1 as i64 + operand2 as i64;
                if set_flags {

                    self.z = result as u32 == 0;
                    self.n = (result & 0x80000000) != 0;
                    self.c = (result & 0x100000000) != 0;
                    self.v = (((operand1 as i64 & 0x7FFFFFFF) + (operand2 as i64 & 0x7FFFFFFF)) & 0x80000000) != 0;
                }
                self.r[rd as usize] = result as u32;
            },
            5 => {
                // ADC
                let mut carry = 0;
                if self.c{
                    carry = 1;
                }
                let result = operand1 as i64 + operand2 as i64 + carry;
                if set_flags {

                    self.z = result as u32 == 0;
                    self.n = (result & 0x80000000) != 0;
                    self.c = (result & 0x100000000) != 0;
                    self.v = (((operand1 as i64 & 0x7FFFFFFF) + (operand2 as i64 & 0x7FFFFFFF) + carry) & 0x80000000) != 0;
                }
                self.r[rd as usize] = result as u32;
            },
            6 => {
                // SBC
                let mut carry = 0;
                if self.c{
                    carry = 1;
                }
                let result = operand1 as i64 - operand2 as i64 + carry - 1;
                if set_flags {

                    self.z = result as u32 == 0;
                    self.n = (result & 0x80000000) != 0;
                    self.c = (result & 0x100000000) == 0; // SBC Carry flag is reversed
                    self.v = (((operand1 as i64 & 0x7FFFFFFF) - (operand2 as i64 & 0x7FFFFFFF) + carry -1) & 0x80000000) != 0;
                }
                self.r[rd as usize] = result as u32;
            },
            7 => {
                // RSC
                let mut carry = 0;
                if self.c{
                    carry = 1;
                }
                let result = operand2 as i64 - operand1 as i64 + carry - 1;
                if set_flags {

                    self.z = result as u32 == 0;
                    self.n = (result & 0x80000000) != 0;
                    self.c = (result & 0x100000000) != 0;
                    self.v = (((operand2 as i64 & 0x7FFFFFFF) - (operand1 as i64 & 0x7FFFFFFF) + carry -1) & 0x80000000) != 0;
                }
                self.r[rd as usize] = result as u32;
            },
            8 => {
                // TST
                let result = operand1 & operand2;

                self.z = result == 0;
                self.n = (result & 0x80000000) != 0;
            },
            9 => {
                // TEQ
                let result = operand1 ^ operand2;

                self.z = result == 0;
                self.n = (result & 0x80000000) != 0;
            },
            10 => {
                // CMP
                let result = operand1 as i64 - operand2  as i64;
                if set_flags {
                    self.z = result as u32 == 0;
                    self.n = (result & 0x80000000) != 0;
                    self.c = !((result & 0x100000000) != 0); // Carry Flag on subtraction (CMP) is reversed
                    self.v = (((operand1 as i64 & 0x7FFFFFFF) - (operand2 as i64 & 0x7FFFFFFF)) & 0x80000000) != 0;
                }
            },
            11 => {
                // CMN
                let result = operand1 as i64 + operand2 as i64;
                if set_flags {

                    self.z = result as u32 == 0;
                    self.n = (result & 0x80000000) != 0;
                    self.c = (result & 0x100000000) != 0;
                    self.v = (((operand1 as i64 & 0x7FFFFFFF) + (operand2 as i64 & 0x7FFFFFFF)) & 0x80000000) != 0;
                }
            },
            12 => {
                // ORR
                let result = operand1 | operand2;
                if set_flags {
                    self.z = result == 0;
                    self.n = (result & 0x80000000) != 0;
                }
                self.r[rd as usize] = result;
            },
            13 => {
                // MOV
                let result = operand2;
                if set_flags{
                    self.z = result == 0;
                    self.n = (result & 0x80000000) != 0;
                }
                self.r[rd as usize] = result;
            },
            14 => {
                // BIC (Bit Clear)
                let result = operand1 & (!operand2);

                self.z = result == 0;
                self.n = (result & 0x80000000) != 0;
                self.r[rd as usize] = result;
            },
            15 => {
                // MVN
                let result = !operand2;
                self.z = result == 0;
                self.n = (result & 0x80000000) != 0;
                self.r[rd as usize] = result;
            },
            _ => {}
        }

        if rd == 15{
            if set_flags{
                match self.mode{
                    FIQ =>{
                        self.set_cpsr(self.spsr_fiq);
                        let new_mode = self.num_to_cpu_mode(self.spsr_fiq & 0x1F);
                        self.switch_mode(new_mode);
                    },
                    _ => {println!("Unimplemented situation: Rd = 15, S-bit is set, and CPU is in {:?} mode.", self.mode);}
                }

            }
            self.flush_pipeline();
        }

    }

    pub fn perform_shift_op_immediate_shift(&mut self, shift_type: u32, shift_amount:u32, value: u32) -> u32{
        // Handle 4 shift types
        match shift_type {
            0 => {
                // Logical Shift Left
                let mut shifted_val: u64 = value as u64;
                shifted_val <<= shift_amount;

                // carry flag is maintained for special case of LSL #0
                self.c = (shifted_val & 0x100000000) != 0;
                return shifted_val as u32;
            }
            1 => {
                // Logical Shift Right
                let mut shifted_val: u64 = (value as u64) << 1;
                /*if self.c {
                    shifted_val |= 1 << 33;
                }*/

                shifted_val >>= shift_amount;
                // special case LSR #0 encodes LSR #32
                if shift_amount == 0{
                    self.c = (shifted_val& (1<<32)) != 0;
                    return 0;
                }

                self.c = (shifted_val & 0x1) != 0;

                return (shifted_val>>1) as u32;
            }
            2 => {
                // Arithmetic Shift Right
                let mut shifted_val: i64 = (((value as i64) << 32) >> 32) << 1;
                if self.c {
                    shifted_val |= 1;
                }
                shifted_val >>= shift_amount;
                // special case ASR #0 encodes ASR #32
                if shift_amount == 0{
                    self.c = (shifted_val& (1<<32)) != 0;
                    return if (shifted_val& (1<<32)) != 0 {0xFFFFFFFF} else {0};
                }

                self.c = (shifted_val & 0x1) != 0;

                return (shifted_val >> 1) as u32;
            }
            3 => {
                // Rotate Right
                if shift_amount == 0 {
                    // Rotate Right Extended
                    let mut carry_in = 0;
                    if self.c {
                        carry_in = 0x80000000;
                    }

                    self.c = (value & 0x1) != 0;
                    return (value >> 1) | carry_in;
                } else {
                    // Normal Rotate Right
                    self.c = (value.rotate_right(shift_amount) & 0x80000000) != 0;
                    return value.rotate_right(shift_amount);
                }
            }
            _ => {
                return value;
            }
        };

    }

    pub fn perform_shift_op_register_shift(&mut self, shift_inst: u32, value: u32) -> u32{
        let shift_type = (shift_inst>>1) & 0b11;
        let selected_reg = (shift_inst>>4) & 0xF;
        let mut shift_amount = self.r[selected_reg as usize] & 0xFF;

        // special case if shift type is rotate right and n >32, result is the same as n-32.
        if shift_type == 3{
            while shift_amount > 32{
                shift_amount -= 32;
            }
        }

        // do nothing if shift amount is 0
        if shift_amount == 0{
            return value;
        }
        // if shift amount is between 1 and 31, then shift as normal
        if shift_amount < 32{
            return self.perform_shift_op_immediate_shift(shift_type, shift_amount, value);
        }

        // special case when shift amount >= 32
        match shift_type {
            // Logical Left Shift
            0 => {
                self.c = if shift_amount == 32 {(value & 0x1) != 0} else {false};
                return 0;
            },
            // Logical Right Shift
            1 => {
                self.c = if shift_amount == 32 {(value & (1<<31)) != 0} else {false};
                return 0;
            }
            // Arithmetic Right Shift
            2 => {
                self.c = (value & (1<<31)) != 0;
                return if (value & (1<<31)) != 0 {0xFFFFFFFF} else {0};
            }
            // Rotate Right
            3 => {
                self.c = (value & (1<<31)) != 0;
                return value;
            }

            _ => {return value;}
        }

    }

}