use crate::memory::Memory;
use super::Cpu;

impl Cpu {
    
    pub fn sp_relative_load_or_store(&mut self, inst: u16, mem: &mut Memory){
        let offset = inst & 0xFF;
        let rd = (inst >> 8) & 0b111;
        let is_load = (inst >> 11) & 0x1 != 0;

        let address = self.r[7] + ((offset as u32) << 2) ;

        if is_load {
            self.r[rd as usize] = mem.read_32(address);
        }else{
            mem.write_32(address, self.r[rd as usize]);
        }
    }

    pub fn load_address(&mut self, inst: u16, mem: &mut Memory){
        let offset = inst & 0xFF;
        let rd = (inst >> 8) & 0b111;
        let use_sp = (inst >> 11) & 0x1 != 0;

        let address = if use_sp {
            self.r[13] + ((offset as u32) << 2)
        }else {
            (self.r[15] & (!0b10)) + ((offset as u32) << 2)
        };

        self.r[rd as usize] = address;

    }

    pub fn add_offset_to_sp(&mut self, inst: u16, mem: &mut Memory){
        let offset = (inst & 0x7F) << 2;
        let sign_flag = (inst >> 7) & 0x1 != 0;

        if sign_flag{
            self.r[13] = self.r[15] - offset as u32;
        }else {
            self.r[13] = self.r[15] + offset as u32;
        }
    }
}