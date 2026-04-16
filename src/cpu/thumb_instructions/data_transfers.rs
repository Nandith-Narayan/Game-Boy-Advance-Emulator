use crate::memory::Memory;
use super::Cpu;

impl Cpu {
    pub fn pc_relative_load(&mut self, inst: u16, mem: &mut Memory){
        let offset = (inst & 0xFF) << 2;
        let rd = (inst >> 8) & 0b111;

        // Note: PC with bit 1 cleared, is used to maintain word alignment
        self.set_r(rd as usize, mem.read_32((self.get_r(15) & 0xFFFFFFFD) + offset as u32));
    }

    pub fn single_data_transfer(&mut self, inst: u16, mem: &mut Memory){
        let rd = inst & 0b111;
        let rb = (inst >> 3) & 0b111;
        let ro = (inst >> 6) & 0b111;
        let transfer_byte = (inst >> 10) & 0x1 != 0;
        let is_load = (inst >> 11) & 0x1 != 0;

        let address = self.get_r(rb as usize) + self.get_r(ro as usize);

        if is_load {
            if transfer_byte {
                self.set_r(rd as usize, mem.read_8(address) as u32);
            }else {
                self.set_r(rd as usize, mem.read_32(address & 0xFFFFFFFC).rotate_right((address & 0b11) * 8));
            }
        }else{
            if transfer_byte {
                mem.write_8(address, self.get_r(rd as usize) as u8);
            }else{
                mem.write_32(address, self.get_r(rd as usize));
            }
        }
    }

    pub fn load_or_store_sign_extended_halfword_or_byte(&mut self, inst: u16, mem: &mut Memory){
        let rd = inst & 0b111;
        let rb = (inst >> 3) & 0b111;
        let ro = (inst >> 6) & 0b111;
        let s_bit = (inst >> 10) & 0x1 != 0;
        let h_bit = (inst >> 11) & 0x1 != 0;

        let address = self.get_r(rb as usize) + self.get_r(ro as usize);

        match(s_bit, h_bit){
            (false, false)=>{
                mem.write_16(address, self.get_r(rd as usize) as u16);
            },
            (false, true)=>{
                self.set_r(rd as usize, (mem.read_16(address & 0xFFFFFFFE) as u32).rotate_right((address & 0b11) * 8));
            },
            (true, false)=>{
                self.set_r(rd as usize, mem.read_8(address) as i8 as i32 as u32);
            },
            (true, true)=>{
                if address & 0x1 == 0 {
                    self.set_r(rd as usize, mem.read_16(address) as i16 as i32 as u32);
                }else {
                    self.set_r(rd as usize, (mem.read_16(address & 0xFFFFFFFE) >> 8) as i8 as i16 as i32 as u32);
                }
            },
        }
    }

    pub fn load_or_store_with_immediate_offset(&mut self, inst: u16, mem: &mut Memory){
        let rd = inst & 0b111;
        let rb = (inst >> 3) & 0b111;
        let offset = (inst >> 6) & 0x1F;
        let is_load = (inst >> 11) & 0x1 != 0;
        let transfer_byte = (inst >> 12) & 0x1 != 0;

        let address = if transfer_byte {
            self.get_r(rb as usize) + offset as u32
        }else{
            self.get_r(rb as usize) + ((offset as u32) << 2)
        };

        if is_load {
            if transfer_byte {
                self.set_r(rd as usize, mem.read_8(address) as u32);
            }else {
                self.set_r(rd as usize, mem.read_32(address & 0xFFFFFFFC).rotate_right((address & 0b11) * 8));
            }
        }else{
            if transfer_byte {
                mem.write_8(address, self.get_r(rd as usize) as u8);
            }else{
                mem.write_32(address, self.get_r(rd as usize));
            }
        }
    }

    pub fn load_or_store_halfword(&mut self, inst: u16, mem: &mut Memory){
        let rd = inst & 0b111;
        let rb = (inst >> 3) & 0b111;
        let offset = (inst >> 6) & 0x1F;
        let is_load = (inst >> 11) & 0x1 != 0;

        let address = self.get_r(rb as usize) + ((offset as u32) << 1);

        if is_load {
                self.set_r(rd as usize, (mem.read_16(address & 0xFFFFFFFE) as u32).rotate_right((address & 0b1) * 8));

        }else{
            mem.write_16(address, self.get_r(rd as usize) as u16);

        }
    }
}