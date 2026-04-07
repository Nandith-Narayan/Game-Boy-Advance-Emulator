use crate::memory::Memory;
use super::Cpu;

impl Cpu {
    pub fn pc_relative_load(&mut self, inst: u16, mem: &mut Memory){
        let offset = (inst & 0xFF) << 2;
        let rd = (inst >> 8) & 0b111;

        // Note: PC with bit 1 cleared, is used to maintain word alignment
        self.r[rd as usize] = mem.read_32((self.r[15] & 0xFFFFFFFD) + offset as u32);
    }

    pub fn single_data_transfer(&mut self, inst: u16, mem: &mut Memory){
        let rd = inst & 0b111;
        let rb = (inst >> 3) & 0b111;
        let ro = (inst >> 6) & 0b111;
        let transfer_byte = (inst >> 10) & 0x1 != 0;
        let is_load = (inst >> 11) & 0x1 != 0;

        let address = self.r[rb as usize] + self.r[ro as usize];

        if is_load {
            if transfer_byte {
                self.r[rd as usize] = mem.read_8(address) as u32;
            }else {
                self.r[rd as usize] = mem.read_32(address);
            }
        }else{
            if transfer_byte {
                mem.write_8(address, self.r[rd as usize] as u8);
            }else{
                mem.write_32(address, self.r[rd as usize]);
            }
        }
    }

    pub fn load_or_store_sign_extended_halfword_or_byte(&mut self, inst: u16, mem: &mut Memory){
        let rd = inst & 0b111;
        let rb = (inst >> 3) & 0b111;
        let ro = (inst >> 6) & 0b111;
        let s_bit = (inst >> 10) & 0x1 != 0;
        let h_bit = (inst >> 11) & 0x1 != 0;

        let address = self.r[rb as usize] + self.r[ro as usize];

        match(s_bit, h_bit){
            (false, false)=>{
                mem.write_16(address, self.r[rd as usize] as u16);
            },
            (false, true)=>{
                self.r[rd as usize] = mem.read_16(address) as u32;
            },
            (true, false)=>{
                self.r[rd as usize] = mem.read_8(address) as i8 as i32 as u32;
            },
            (true, true)=>{
                self.r[rd as usize] = mem.read_16(address) as i16 as i32 as u32;
            },
        }
    }

    pub fn load_or_store_with_immediate_offset(&mut self, inst: u16, mem: &mut Memory){
        let rd = inst & 0b111;
        let rb = (inst >> 3) & 0b111;
        let offset = (inst >> 6) & 0x1F;
        let is_load = (inst >> 11) & 0x1 != 0;
        let transfer_byte = (inst >> 12) & 0x1 != 0;

        let address = self.r[rb as usize] + offset as u32;

        if is_load {
            if transfer_byte {
                self.r[rd as usize] = mem.read_8(address) as u32;
            }else {
                self.r[rd as usize] = mem.read_32(address);
            }
        }else{
            if transfer_byte {
                mem.write_8(address, self.r[rd as usize] as u8);
            }else{
                mem.write_32(address, self.r[rd as usize]);
            }
        }
    }

    pub fn load_or_store_halfword(&mut self, inst: u16, mem: &mut Memory){
        let rd = inst & 0b111;
        let rb = (inst >> 3) & 0b111;
        let offset = (inst >> 6) & 0x1F;
        let is_load = (inst >> 11) & 0x1 != 0;

        let address = self.r[rb as usize] + offset as u32;

        if is_load {
                self.r[rd as usize] = mem.read_16(address) as u32;

        }else{
            mem.write_16(address, self.r[rd as usize] as u16);

        }
    }
}