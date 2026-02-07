use crate::cpu::enums::CPUMode;
use crate::cpu::enums::CPUMode::*;
use super::Cpu;


// Implementation of functions related to ARM mode of the CPU
impl Cpu {
    pub fn branch_and_exchange(&mut self, inst: u32) {
        let mut rn: usize = (inst & 0xF) as usize;
        self.r[15] = self.r[rn]&0xFFFFFFFC;
        self.flush_pipeline();
    }
    pub fn branch(&mut self, inst: u32) {
        let mut offset: i32 = (((inst & 0x00FFFFFF) << 8) as i32) >> 8;
        offset <<= 2;
        self.r[15] = ((self.r[15] as i32) + offset) as u32;
        self.flush_pipeline();
    }
    pub fn branch_with_link(&mut self, inst: u32) {
        let mut offset: i32 = (((inst & 0x00FFFFFF) << 8) as i32) >> 8;
        offset <<= 2;
        self.r[14] = self.r[15];
        self.r[15] = ((self.r[15] as i32) + offset) as u32;
        self.flush_pipeline();
    }
    pub fn single_data_swap(&mut self, inst: u32){
        let rn = (inst & 0xF0000) >> 16;
        let rd = (inst & 0xF000) >> 12;
        let rm = inst & 0xF;
        if inst&(1<<22) != 0{
            // Swap 8-bit quantity
            let value = self.memory.read_8(self.r[rn as usize]);
            self.memory.write_8(self.r[rn as usize], self.r[rm as usize] as u8);
            self.r[rd as usize] = value as u32;
        }else{
            // Swap 32-bit quantity
            let value = self.memory.read_32(self.r[rn as usize]);
            self.memory.write_32(self.r[rn as usize], self.r[rm as usize]);
            self.r[rd as usize] = value;
        }
    }
    pub fn single_data_transfer_immediate_operand(&mut self, inst: u32){
        let offset = inst & 0xFFF;
        self.single_data_transfer_with_offset(inst, offset);
    }
    pub fn single_data_transfer_register_operand(&mut self, inst: u32){
        // compute offset
        let rm = inst & 0xF;

        let set_flags = true;

        // register shift
        let shift = (inst & 0xFF0) >> 4;
        let shift_amount = (shift & 0xF8) >> 3;
        let mut offset = self.r[rm as usize];

        // Handle 4 shift types
        match (shift & 0b110) >> 1 {
            0 => {
                // Logical Shift Left
                let mut shifted_val: u64 = offset as u64;
                if self.c {
                    shifted_val |= 1 << 32;
                }
                shifted_val <<= shift_amount;
                offset = shifted_val as u32;
                if set_flags {
                    self.c = (shifted_val & 0x100000000) != 0;
                }
            }
            1 => {
                // Logical Shift Right
                let mut shifted_val: u64 = (offset as u64) << 1;
                if self.c {
                    shifted_val |= 1;
                }
                shifted_val >>= shift_amount;
                offset = (shifted_val >> 1) as u32;
                if set_flags {
                    self.c = (shifted_val & 0x1) != 0;
                }
            }
            2 => {
                // Arithmetic Shift Right
                let mut shifted_val: i64 = (((offset as i64) << 32) >> 32) << 1;
                if self.c {
                    shifted_val |= 1;
                }
                shifted_val >>= shift_amount;
                offset = (shifted_val >> 1) as u32;
                if set_flags {
                    self.c = (shifted_val & 0x1) != 0;
                }
            }
            3 => {
                // Rotate Right
                if shift_amount == 0 {
                    // Rotate Right Extended
                    let mut carry_in = 0;
                    if self.c {
                        carry_in = 0x80000000;
                    }
                    if set_flags {
                        self.c = (offset & 0x1) != 0;
                    }
                    offset = offset >> 1;
                    offset = offset | carry_in;
                } else {
                    // Normal Rotate Right
                    offset = offset.rotate_right(shift_amount);
                    if set_flags {
                        self.c = (offset & 0x80000000) != 0;
                    }
                }
            }
            _ => {}
        }
        // execute the rest of the instruction using the offset
        self.single_data_transfer_with_offset(inst, offset);

    }
    pub fn single_data_transfer_with_offset(&mut self, inst: u32, offset:u32){
        let rn = (inst & 0xF0000) >> 16;
        let rd = (inst & 0xF000) >> 12;

        let mut address = self.r[rn as usize];

        let add_before_transfer = (inst & (1 << 24)) != 0;
        let add_offset = (inst & (1 << 23)) != 0;
        let transfer_byte = (inst & (1 << 22)) != 0;
        let write_back = (inst & (1 << 21)) != 0;
        let load_data = (inst & (1 << 20)) != 0;
        if add_before_transfer{
            if add_offset{
                address += offset;
            }else{
                address -= offset;
            }
        }
        // Do transfer
        if transfer_byte{
            if load_data{
                self.r[rd as usize] = self.memory.read_8(address) as u32;
            }else{
                self.memory.write_8(address, self.r[rd as usize] as u8);
            }

        }else{
            if load_data{
                self.r[rd as usize] = self.memory.read_32(address);
            }else{
                self.memory.write_32(address, self.r[rd as usize]);
            }
        }
        // Write address back to base register
        if write_back{
            self.r[rn as usize] = address;
        }

        if !add_before_transfer{
            if add_offset{
                address += offset;
            }else{
                address -= offset;
            }
        }

    }
    pub fn data_processing_register_operand(&mut self, inst: u32) {
        let rn = (inst & 0xF0000) >> 16;
        let rd = (inst & 0xF000) >> 12;
        let rm = inst & 0xF;

        let operand1 = self.r[rn as usize];
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
            // shift by register value
            operand2 = self.perform_shift_op_register_shift(shift_inst, operand2);
        }


        self.alu_operations(inst, operand1, operand2, rd, set_flags);

    }

    pub fn perform_shift_op_immediate_shift(&mut self,shift_type: u32, shift_amount:u32, value: u32) -> u32{
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

    pub fn data_processing_immediate_operand(&mut self, inst: u32){
        let rn = (inst & 0xF0000) >> 16;
        let rd = (inst & 0xF000) >> 12;
        let mut immediate = inst & 0xFF;
        let rotate = (inst & 0xF00)>> 8;

        let operand1 = self.r[rn as usize];
        let operand2 = immediate.rotate_right(rotate*2);
        // update the carry flag only if the shifter was used to rotate the immediate value
        if rotate != 0{
            self.c = (operand2 & 0x80000000) != 0;
        }

        let set_flags = (inst & (1 << 20)) != 0;

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
                let result = operand1 as i64 - operand2 as i64;
                if set_flags {

                    self.z = result as u32 == 0;
                    self.n = (result & 0x80000000) != 0;
                    self.c = (result & 0x100000000) != 0;
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

                self.z = result == 0;
                self.n = (result & 0x80000000) != 0;
                self.r[rd as usize] = result;
            },
            13 => {
                // MOV
                let result = operand2;

                self.z = result == 0;
                self.n = (result & 0x80000000) != 0;
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
            self.flush_pipeline();
            }
        }

    }

    pub fn transfer_to_program_status_register(&mut self, inst: u32) {
        let operand_is_immediate = (inst & (1 << 25)) != 0;
        let set_flags = (inst & (1 << 19)) != 0;
        let write_to_control_bits = (inst & (1 << 16)) != 0;
        let operand;
        println!("set_flags {}", set_flags);
        if operand_is_immediate {
            let rotate = (inst & 0xF00) >> 8;
            let immediate = inst & 0xFF;
            operand = immediate.rotate_right(rotate * 2);
        }else{
            let rm = inst & 0xF;
            operand = self.r[rm as usize];
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
                _ => {println!("failed to set CPU {:?} mode's SPSR.", self.mode);},
            }
        }
    }

    pub fn program_status_register_transfer_from_register(&mut self, inst: u32) {
        println!("EEEE");
        let destination_is_cpsr = (inst & (1 << 22)) == 0;
        let rm = inst & 0xF;
        let operand = self.r[rm as usize];

        if destination_is_cpsr{
            self.n = (operand & (1 << 31)) != 0;
            self.z = (operand & (1 << 30)) != 0;
            self.c = (operand & (1 << 29)) != 0;
            self.v = (operand & (1 << 28)) != 0;

            let new_mode = self.num_to_cpu_mode(operand & 0x1F);

            self.switch_mode(new_mode);


        }else{
            match self.mode{
                USER => {println!("CPU user mode doesn't have a SPSR");},
                FIQ => {
                    self.spsr_fiq = operand;
                },
                IRQ => {
                    self.spsr_irq = operand;
                },
                _ => {println!("failed to set CPU {:?} mode's SPSR.", self.mode);},
            }
        }

    }

    pub fn num_to_cpu_mode(&mut self, n: u32) -> CPUMode{
       return match n{
            0b10000 => USER,
            0b10001 => FIQ,
            0b10010 => IRQ,
            0b10011 => SUPERVISOR,
            0b10111 => ABORT,
            0b11011 => UNDEFINED,
            0b11111 => SYSTEM,
            _ => UNDEFINED,
        }
    }

    pub fn switch_mode(&mut self, new_mode: CPUMode){

        println!("Mode swap triggered: {:?} -> {:?}", self.mode, new_mode);

        let mode_swap = if new_mode == USER || new_mode == SYSTEM {self.mode} else {new_mode};

        match mode_swap{
            FIQ => {
                std::mem::swap(&mut self.r[8], &mut self.r_fiq[8]);
                std::mem::swap(&mut self.r[9], &mut self.r_fiq[9]);
                std::mem::swap(&mut self.r[10], &mut self.r_fiq[10]);
                std::mem::swap(&mut self.r[11], &mut self.r_fiq[11]);
                std::mem::swap(&mut self.r[12], &mut self.r_fiq[12]);
                std::mem::swap(&mut self.r[13], &mut self.r_fiq[13]);
                std::mem::swap(&mut self.r[14], &mut self.r_fiq[14]);

                let temp = self.get_cpsr();
                self.set_cpsr(self.spsr_fiq);
                self.spsr_fiq = temp;
            },
            IRQ => {
                std::mem::swap(&mut self.r[13], &mut self.r_fiq[13]);
                std::mem::swap(&mut self.r[14], &mut self.r_fiq[14]);

                let temp = self.get_cpsr();
                self.set_cpsr(self.spsr_fiq);
                self.spsr_fiq = temp;
            },

            _ => {println!("Failed to switch to mode {:?}", new_mode);}
        }

        self.mode = new_mode;
    }

}
