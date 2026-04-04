use crate::memory::Memory;
use super::Cpu;

impl Cpu{
    pub fn block_data_transfer_load(&mut self, inst: u32, mem: &mut Memory){
        let rn = (inst >> 16) & 0x0F;
        let is_pre_indexing = inst & (1 << 24) != 0;
        let is_increment = inst & (1 << 23) != 0;
        let write_back = inst & (1 << 21) != 0;


        let mut register_list = inst & 0xFFFF;
        let reg_count = register_list.count_ones();

        let base_address = self.r[rn as usize];
        let mut address = base_address;
        if !is_increment {
            address -= reg_count*4;
        }
        if is_pre_indexing == is_increment {
            address += 4;
        }

        for i in 0..16{
            if register_list & 0x1 != 0{
                self.r[i] = mem.read_32(address);
                address += 4;
            }
            register_list >>= 1;
        }

        if write_back {
            let mut new_address = base_address;
            if is_increment{
                new_address += reg_count*4;
            }else{
                new_address -= reg_count*4;
            }
            self.r[rn as usize] = new_address;
        }
    }

    pub fn block_data_transfer_store(&mut self, inst: u32, mem: &mut Memory){
        let rn = (inst >> 16) & 0x0F;
        let is_pre_indexing = inst & (1 << 24) != 0;
        let is_increment = inst & (1 << 23) != 0;
        let write_back = inst & (1 << 21) != 0;


        let mut register_list = inst & 0xFFFF;
        let reg_count = register_list.count_ones();

        let base_address = self.r[rn as usize];
        let mut address = base_address;
        if !is_increment {
            address -= reg_count*4;
        }
        if is_pre_indexing == is_increment {
            address += 4;
        }

        for i in 0..15{
            if register_list & 0x1 != 0{
                mem.write_32(address, self.r[i]);
                address += 4;
            }
            register_list >>= 1;
        }
        // Handle R[15] (PC)
        if register_list & 0x1 != 0{
            mem.write_32(address, self.r[15]+4);
        }

        if write_back {
            let mut new_address = base_address;
            if is_increment{
                new_address += reg_count*4;
            }else{
                new_address -= reg_count*4;
            }
            self.r[rn as usize] = new_address;
        }
    }
}