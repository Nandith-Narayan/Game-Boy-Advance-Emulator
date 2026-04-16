use crate::memory::Memory;
use super::Cpu;

impl Cpu{

    pub fn half_word_transfer_register(&mut self, inst: u32, mem: &mut Memory){
        let rm = inst & 0xF;
        let rn = (inst & 0xF0000) >> 16;
        let rd = (inst & 0xF000) >> 12;

        let update_offset_before_transfer = (inst & (1<<24)) != 0;
        let add_offset = (inst & (1<<23)) != 0;
        let write_back = (inst & (1<<21)) != 0;
        let is_load = (inst & (1<<20)) != 0;
        let op_type = (inst >> 5) & 0b011;

        let offset = self.get_r(rm as usize);
        let base_address = self.get_r(rn as usize);
        let new_address = if add_offset{ base_address + offset } else { base_address - offset };

        let address = if update_offset_before_transfer { new_address } else { base_address };

        if is_load{
            self.half_word_loads(rd as usize, op_type, address, mem);
        }else{
            self.half_word_stores(rd as usize, op_type, address, mem);
        }

        if write_back || !update_offset_before_transfer{
            if !is_load || rd != rn{
                self.set_r(rn as usize, new_address);
            }
        }

    }

    pub fn half_word_transfer_immediate(&mut self, inst: u32, mem: &mut Memory){
        let rn = (inst & 0xF0000) >> 16;
        let rd = (inst & 0xF000) >> 12;

        let update_offset_before_transfer = (inst & (1<<24)) != 0;
        let add_offset = (inst & (1<<23)) != 0;
        let write_back = (inst & (1<<21)) != 0;
        let is_load = (inst & (1<<20)) != 0;
        let op_type = (inst >> 5) & 0b011;

        let offset = (((inst >> 4) & 0xF0) | (inst & 0xF)) & 0xFF;
        let base_address = self.get_r(rn as usize);
        let new_address = if add_offset{ base_address + offset } else { base_address - offset };

        let address = if update_offset_before_transfer { new_address } else { base_address };

        if is_load{
            self.half_word_loads(rd as usize, op_type, address, mem);
        }else{
            self.half_word_stores(rd as usize, op_type, address, mem);
        }

        if write_back || !update_offset_before_transfer{
            if !is_load || rd != rn{
                self.set_r(rn as usize, new_address);
            }
        }

    }

    fn half_word_loads(&mut self, rd: usize, op_type: u32, address: u32, mem: &mut Memory){
        match op_type{
            0 => {println!("This should be a SWP, but was decoded as a halfword transfer");},
            1 => {
                self.set_r(rd, mem.read_16(address & 0xFFFFFFFE) as u32);
                if address & 0x1 != 0{
                    self.set_r(rd, self.get_r(rd).rotate_right(8));
                }
            },
            2 => {
                let mut val = mem.read_8(address & 0xFFFFFFFE) as u32;
                if val & 0x80 != 0{
                    val |= 0xFFFFFF00;
                }
                self.set_r(rd, val);
            },
            3 => {
                if address & 0x1 ==0 {
                    let mut val = mem.read_16(address & 0xFFFFFFFE) as u32;
                    if val & 0x8000 != 0 {
                        val |= 0xFFFF0000;
                    }
                    self.set_r(rd, val);
                }else{
                    // Handle Misaligned signed halfword load
                    let mut val = mem.read_16(address & 0xFFFFFFFE) as u32;
                    val >>= 8; // Use top byte and treat as 8bit signed load
                    if val & 0x80 != 0 {
                        val |= 0xFFFFFF00;
                    }
                    self.set_r(rd, val);
                }
            },
            _ => {}
        }
    }

    fn half_word_stores(&mut self, rd: usize, op_type: u32, address: u32, mem: &mut Memory){
        match op_type{
            0 => {println!("This should be a SWP, but was decoded as a halfword transfer");},
            1 | 3 => {mem.write_16(address & 0xFFFFFFFE, (self.get_r(rd) & 0xFFFF) as u16)},
            2 => {
                mem.write_8(address & 0xFFFFFFFE, (self.get_r(rd) & 0xFF) as u8)
            },
            _ => {}
        }
    }

}