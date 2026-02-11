use super::Cpu;

impl Cpu{

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

}