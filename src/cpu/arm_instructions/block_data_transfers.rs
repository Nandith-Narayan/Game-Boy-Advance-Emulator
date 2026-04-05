use crate::cpu::enums::CPUMode;
use crate::memory::Memory;
use super::Cpu;

impl Cpu{
    pub fn block_data_transfer_load(&mut self, inst: u32, mem: &mut Memory){
        let rn = (inst >> 16) & 0x0F;
        let is_pre_indexing = inst & (1 << 24) != 0;
        let is_increment = inst & (1 << 23) != 0;
        let s_bit = inst & (1 << 22) != 0;
        let write_back = inst & (1 << 21) != 0;


        let mut register_list = inst & 0xFFFF;
        // Handle special case if the register list is empty
        if register_list == 0{
            // Load PC
            self.r[15] = mem.read_32(self.r[rn as usize]);
            // Increment base register as if the register list was full
            self.r[rn as usize] += 0x40;
            // PC has been updated, so the pipeline has to be flushed
            self.flush_pipeline();
            return;
        }
        let reg_count = register_list.count_ones();

        let base_address = self.r[rn as usize];
        let mut address = base_address;
        if !is_increment {
            address -= reg_count*4;
        }
        if is_pre_indexing == is_increment {
            address += 4;
        }

        // Perform write back first, because if the write back register is in the list,
        // then write back register will be overwritten by the load
        if write_back {
            let mut new_address = base_address;
            if is_increment{
                new_address += reg_count*4;
            }else{
                new_address -= reg_count*4;
            }
            self.r[rn as usize] = new_address;
        }

        for i in 0..15{
            if register_list & 0x1 != 0{
                if !s_bit {
                    self.r[i] = mem.read_32(address);
                }else{
                    self.set_mode_specific_reg(i, mem.read_32(address));
                }
                address += 4;
            }
            register_list >>= 1;
        }
        // Handle R[15] (PC)
        if register_list & 0x1 != 0{
            self.r[15] = mem.read_32(address);
            self.flush_pipeline();
        }


    }

    pub fn block_data_transfer_store(&mut self, inst: u32, mem: &mut Memory){
        let rn = (inst >> 16) & 0x0F;
        let is_pre_indexing = inst & (1 << 24) != 0;
        let is_increment = inst & (1 << 23) != 0;
        let s_bit = inst & (1 << 22) != 0;
        let write_back = inst & (1 << 21) != 0;


        let mut register_list = inst & 0xFFFF;
        // Handle special case if the register list is empty
        if register_list == 0{
            match (is_increment, is_pre_indexing){
                (false, false) =>{ // Decrement After
                    mem.write_32(self.r[rn as usize]-0x3C, self.r[15]+4);
                    self.r[rn as usize] -= 0x40;
                },
                (false, true) =>{ // Decrement Before
                    mem.write_32(self.r[rn as usize]-0x40, self.r[15]+4);
                    self.r[rn as usize] -= 0x40;
                },
                (true, false) =>{ // Increment After
                    mem.write_32(self.r[rn as usize], self.r[15]+4);
                    self.r[rn as usize] += 0x40;
                },
                (true, true) =>{ // Increment Before
                    mem.write_32(self.r[rn as usize]+4, self.r[15]+4);
                    self.r[rn as usize] += 0x40;
                },
            }
            return;
        }
        let reg_count = register_list.count_ones();

        let base_address = self.r[rn as usize];
        let mut address = base_address;
        if !is_increment {
            address -= reg_count*4;
        }
        if is_pre_indexing == is_increment {
            address += 4;
        }

        let mut base_reg_in_list = false;
        let mut is_first = true;
        let mut base_reg_destination_addr = base_address;
        for i in 0..15{
            if register_list & 0x1 != 0{
                if i == rn as usize {
                    base_reg_in_list = true;
                    base_reg_destination_addr = address;
                }

                if !s_bit {
                    mem.write_32(address, self.r[i]);
                    // To handle weird edge cases when the base register is first in the list,
                    // write back immediately after storing the first register
                    // Cases:
                    //     Rn not in list: Write back occurs after storing unrelated register
                    //     Rn first in list: Write back occurs after storing Rn,
                    //                       stack has og value
                    //     Rn not first in list: Write back occurs before storing Rn,
                    //                           stack has new value
                    if write_back && is_first{
                        if is_increment {
                            self.r[rn as usize] = base_address + reg_count*4;
                        }else{
                            self.r[rn as usize] = base_address - reg_count*4;
                        }
                    }

                }else{
                    mem.write_32(address, self.get_mode_specific_reg(i));
                }

                address += 4;

                is_first = false;
            }
            register_list >>= 1;
        }
        // Handle R[15] (PC)
        if register_list & 0x1 != 0{
            mem.write_32(address, self.r[15]+4);
        }

    }

    fn get_mode_specific_reg(&mut self, reg: usize) -> u32{
        return match (self.mode, reg) {
            (_, 0..=7) => self.r[reg],
            (CPUMode::USER|CPUMode::SYSTEM, _) => self.r[reg],
            (CPUMode::FIQ, 8..=14) => self.r_fiq[reg],
            (CPUMode::IRQ, 13|14) => self.r_irq[reg],
            _ => self.r[reg],
        };
    }

    fn set_mode_specific_reg(&mut self, reg: usize, value: u32){
        match (self.mode, reg) {
            (_, 0..=7) => self.r[reg] = value,
            (CPUMode::USER|CPUMode::SYSTEM, _) => self.r[reg] = value,
            (CPUMode::FIQ, 8..=14) => self.r_fiq[reg] = value,
            (CPUMode::IRQ, 13|14) => self.r_irq[reg] = value,
            _ => self.r[reg] = value,
        };
    }
}