use crate::memory::Memory;
use super::Cpu;

impl Cpu {

    pub fn sp_relative_load_or_store(&mut self, inst: u16, mem: &mut Memory){
        let offset = inst & 0xFF;
        let rd = (inst >> 8) & 0b111;
        let is_load = (inst >> 11) & 0x1 != 0;

        let address = self.get_r(13) + ((offset as u32) << 2) ;

        if is_load {
            self.set_r(rd as usize, mem.read_32(address & 0xFFFFFFFE).rotate_right((address & 0b11) * 8));
        }else{
            mem.write_32(address, self.get_r(rd as usize));
        }
    }

    pub fn load_address(&mut self, inst: u16, mem: &mut Memory){
        let offset = inst & 0xFF;
        let rd = (inst >> 8) & 0b111;
        let use_sp = (inst >> 11) & 0x1 != 0;

        let address = if use_sp {
            self.get_r(13) + ((offset as u32) << 2)
        }else {
            (self.get_r(15) & (!0b10)) + ((offset as u32) << 2)
        };

        self.set_r(rd as usize, address);

    }

    pub fn add_offset_to_sp(&mut self, inst: u16, mem: &mut Memory){
        let offset = (inst & 0x7F) << 2;
        let sign_flag = (inst >> 7) & 0x1 != 0;

        if sign_flag{
            self.set_r(13, self.get_r(13) - offset as u32);
        }else {
            self.set_r(13, self.get_r(13) + offset as u32);
        }
    }

    pub fn push_registers(&mut self, inst: u16, mem: &mut Memory){
        let mut register_list = inst & 0xFF;
        let r_bit = (inst >> 8) & 0x1 != 0;

        let mut address = self.get_r(13);
        if r_bit{
            address -= 4;
            mem.write_32(address, self.get_r(14));
        }
        for i in (0..=7).rev(){
            if register_list & 0x80 != 0{
                address -= 4;
                mem.write_32(address, self.get_r(i));
            }
            register_list <<= 1;
        }


        self.set_r(13, address);
    }

    pub fn pop_registers(&mut self, inst: u16, mem: &mut Memory){
        let mut register_list = inst & 0xFF;
        let r_bit = (inst >> 8) & 0x1 != 0;

        let mut address = self.get_r(13);
        if r_bit{
            self.set_r(15, mem.read_32(address) & (!0x1));
            address += 4;
            self.flush_pipeline();

        }
        for i in 0..=7{
            if register_list & 0x1 != 0{
                self.set_r(i, mem.read_32(address));
                address += 4;

            }
            register_list >>= 1;
        }


        self.set_r(13, address);
    }

    pub fn store_registers(&mut self, inst: u16, mem: &mut Memory){
        let mut register_list = inst & 0xFF;
        let rb = (inst >> 8) & 0b111;

        // Handle special case if the register list is empty
        if register_list == 0{
            mem.write_32(self.get_r(rb as usize), self.get_r(15)+2);
            self.set_r(rb as usize, self.get_r(rb as usize) + 0x40);
            return;
        }
        let reg_count = register_list.count_ones();
        let mut address = self.get_r(rb as usize);
        let mut is_first = true;
        for i in 0..=7{
            if register_list & 0x1 != 0{
                mem.write_32(address, self.get_r(i));
                address += 4;
                if is_first{
                    self.increment_r(rb as usize, reg_count*4);
                }
                is_first = false;
            }
            register_list >>= 1;
        }
    }

    pub fn load_registers(&mut self, inst: u16, mem: &mut Memory){
        let mut register_list = inst & 0xFF;
        let rb = (inst >> 8) & 0b111;

        // Handle special case if the register list is empty
        if register_list == 0{
            // Load PC
            self.set_r(15, mem.read_32(self.get_r(rb as usize)));
            // Increment base register as if the register list was full
            self.increment_r(rb as usize, 0x40);
            // PC has been updated, so the pipeline has to be flushed
            self.flush_pipeline();
            return;
        }

        let mut address = self.get_r(rb as usize);

        for i in 0..=7{
            if register_list & 0x1 != 0{
                self.set_r(i, mem.read_32(address));
                address += 4;

            }
            register_list >>= 1;
        }

        self.set_r(rb as usize, address);
    }
}