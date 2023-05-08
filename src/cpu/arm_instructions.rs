use super::Cpu;


// Implementation of functions related to ARM mode of the CPU
impl Cpu {
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
            println!("TODO: IMPLEMENT WORD-SIZED SINGLE DATA TRANSFERS");
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
        let shift = (inst & 0xFF0) >> 4;
        let shift_amount;
        if shift & 0x1 == 1 {
            let rs = (shift & 0xF0) >> 4;
            shift_amount = self.r[rs as usize] & 0xF;
        } else {
            shift_amount = (shift & 0xF8) >> 3;
        }
        // Handle 4 shift types
        match (shift & 0b110) >> 1 {
            0 => {
                // Logical Shift Left
                let mut shifted_val: u64 = operand2 as u64;
                if self.c {
                    shifted_val |= 1 << 32;
                }
                shifted_val <<= shift_amount;
                operand2 = shifted_val as u32;
                if set_flags {
                    self.c = (shifted_val & 0x100000000) != 0;
                }
            }
            1 => {
                // Logical Shift Right
                let mut shifted_val: u64 = (operand2 as u64) << 1;
                if self.c {
                    shifted_val |= 1;
                }
                shifted_val >>= shift_amount;
                operand2 = (shifted_val >> 1) as u32;
                if set_flags {
                    self.c = (shifted_val & 0x1) != 0;
                }
            }
            2 => {
                // Arithmetic Shift Right
                let mut shifted_val: i64 = (((operand2 as i64) << 32) >> 32) << 1;
                if self.c {
                    shifted_val |= 1;
                }
                shifted_val >>= shift_amount;
                operand2 = (shifted_val >> 1) as u32;
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
                        self.c = (operand2 & 0x1) != 0;
                    }
                    operand2 = operand2 >> 1;
                    operand2 = operand2 | carry_in;
                } else {
                    // Normal Rotate Right
                    operand2 = operand2.rotate_right(shift_amount);
                    if set_flags {
                        self.c = (operand2 & 0x80000000) != 0;
                    }
                }
            }
            _ => {}
        }
        self.alu_operations(inst, operand1, operand2, rd, set_flags);

    }

    pub fn alu_operations(&mut self, inst:u32, operand1:u32, operand2:u32, rd:u32, set_flags:bool){
        match (inst & 0x1E00000) >> 1 {
            0 => {
                // AND
                let result = operand1 & operand2;
                if result == 0 {
                    self.z = true;
                }
                self.n = (result & 0x80000000) != 0;
                self.r[rd as usize] = result;
            }
            1 => {
                // XOR
                let result = operand1 ^ operand2;
                if result == 0 {
                    self.z = true;
                }
                self.n = (result & 0x80000000) != 0;

            }
            2 => {
                // SUB
                let result = operand1 as i64 - operand2 as i64;
                if set_flags {
                    if result == 0 {
                        self.z = true;
                    }
                    self.n = (result & 0x80000000) != 0;
                    self.c = (result & 0x100000000) != 0;
                    self.v = (((operand1 as i64 & 0x7FFFFFFF) - (operand2 as i64 & 0x7FFFFFFF)) & 0x80000000) != 0;
                }
                self.r[rd as usize] = result as u32;
            },
            3 => {
                // RSB
                let result = operand2 as i64 - operand1 as i64;
                if set_flags {
                    if result == 0 {
                        self.z = true;
                    }
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
                    if result == 0 {
                        self.z = true;
                    }
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
                    if result == 0 {
                        self.z = true;
                    }
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
                    if result == 0 {
                        self.z = true;
                    }
                    self.n = (result & 0x80000000) != 0;
                    self.c = (result & 0x100000000) != 0;
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
                    if result == 0 {
                        self.z = true;
                    }
                    self.n = (result & 0x80000000) != 0;
                    self.c = (result & 0x100000000) != 0;
                    self.v = (((operand2 as i64 & 0x7FFFFFFF) - (operand1 as i64 & 0x7FFFFFFF) + carry -1) & 0x80000000) != 0;
                }
                self.r[rd as usize] = result as u32;
            },
            8 => {
                // TST
                let result = operand1 & operand2;
                if result == 0 {
                    self.z = true;
                }
                self.n = (result & 0x80000000) != 0;
            }
            9 => {
                // TEQ
                let result = operand1 ^ operand2;
                if result == 0 {
                    self.z = true;
                }
                self.n = (result & 0x80000000) != 0;
            }
            10 => {
                // CMP
                let result = operand1 as i64 - operand2 as i64;
                if set_flags {
                    if result == 0 {
                        self.z = true;
                    }
                    self.n = (result & 0x80000000) != 0;
                    self.c = (result & 0x100000000) != 0;
                    self.v = (((operand1 as i64 & 0x7FFFFFFF) - (operand2 as i64 & 0x7FFFFFFF)) & 0x80000000) != 0;
                }
            },
            11 => {
                // CMN
                let result = operand1 as i64 + operand2 as i64;
                if set_flags {
                    if result == 0 {
                        self.z = true;
                    }
                    self.n = (result & 0x80000000) != 0;
                    self.c = (result & 0x100000000) != 0;
                    self.v = (((operand1 as i64 & 0x7FFFFFFF) + (operand2 as i64 & 0x7FFFFFFF)) & 0x80000000) != 0;
                }
            },
            12 => {
                // ORR
                let result = operand1 | operand2;
                if result == 0 {
                    self.z = true;
                }
                self.n = (result & 0x80000000) != 0;
                self.r[rd as usize] = result;
            }
            13 => {
                // MOV
                let result = operand2;
                if result == 0 {
                    self.z = true;
                }
                self.n = (result & 0x80000000) != 0;
                self.r[rd as usize] = result;
            }
            14 => {
                // BIC (Bit Clear)
                let result = operand1 & (!operand2);
                if result == 0 {
                    self.z = true;
                }
                self.n = (result & 0x80000000) != 0;
                self.r[rd as usize] = result;
            }
            15 => {
                // MVN
                let result = !operand2;
                if result == 0 {
                    self.z = true;
                }
                self.n = (result & 0x80000000) != 0;
                self.r[rd as usize] = result;
            }
            _ => {}
        }
    }
}
