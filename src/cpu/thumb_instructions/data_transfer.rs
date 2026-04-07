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
}