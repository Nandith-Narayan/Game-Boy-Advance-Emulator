use crate::memory::dma_transfers::{DMADestinationAdjustment, DMASourceAdjustment, DMATransfer};
use crate::memory::dma_transfers::DMATimingMode::{HBLANK, IMMEDIATE, SPECIAL, VBLANK};
use super::Memory;

impl Memory{

    pub fn read_io_register(&self, address: usize) -> u8{
        return match address {
            // DISPCNT - Display Control Register
            0x0 => {
                (self.lcd_bg_mode & 0b111) | // B0-B2 - video mode
                ((self.frame_select as u8) << 4) // B4 - bitmap frame select
            },
            0x1 => {
                (self.bg_enabled[0] as u8) |
                ((self.bg_enabled[1] as u8) << 1) |
                ((self.bg_enabled[2] as u8) << 2) |
                ((self.bg_enabled[3] as u8) << 3)
            },

            // DISPSTAT - Display Status Register
            0x4 => {
                (self.v_blank as u8) |
                ((self.h_blank as u8) << 1)
            },
            0x5 => {0},
            0x6 => {self.vertical_count}, 0x7 => {0},

            // Background Control Registers
            0x8..=0xF=>{
                let bg_idx = (address - 0x8) / 2;
                return if address % 2 == 0 {
                    self.bg_control[bg_idx].get_lower_byte()
                } else {
                    self.bg_control[bg_idx].get_upper_byte()
                }
            }

            // DMA Registers
            0xB0..=0xDF => {self.read_dma_register(address)},


            0x130 => {(self.keypad & 0xFF) as u8},
            0x131 => {((self.keypad >> 8 )& 0xFF) as u8},
            0x132 | 0x133 => {0},

            // Interrupt Master Enable
            0x208 => {
              self.interrupt_master_enable as u8
            },
            0x209 => {0},

            _ => {println!("Reading from unimplemented IO Reg @ {:#4x}", address); 0},
        }
    }
    pub fn write_io_register(&mut self, address: usize, val: u8){
        match address {
            0x0 => {
                self.frame_select = (val & 0b10000) != 0;
                self.lcd_bg_mode = val & 0b111;

                println!("BG mode: {}",self.lcd_bg_mode);
            },
            0x1 => {
                self.bg_enabled[0] = (val & 0b1) != 0;
                self.bg_enabled[1] = (val & 0b10) != 0;
                self.bg_enabled[2] = (val & 0b100) != 0;
                self.bg_enabled[3] = (val & 0b1000) != 0;
            },

            // Background Control Registers
            0x8..=0xF => {
                let bg_idx = (address - 0x8) / 2;
                if address % 2 == 0 {
                    self.bg_control[bg_idx].set_lower_byte(val);
                } else {
                    self.bg_control[bg_idx].set_upper_byte(val);
                }
            }
            // Background Scroll Registers
            0x10..=0x1E => {
                if address % 2 == 0 {
                    self.bg_scroll_regs[(address-0x10)/2] &= 0xFF00;
                    self.bg_scroll_regs[(address-0x10)/2] |= val as u16;
                }else{
                    self.bg_scroll_regs[(address-0x10)/2] &= 0x00FF;
                    self.bg_scroll_regs[(address-0x10)/2] |= (val as u16)<<8;
                }

            }

            // DMA Registers
            0xB0..=0xDF => {self.write_dma_register(address, val);},

            // Interrupt Enable Register
            0x200 => {
                self.interrupt_enable = (self.interrupt_enable & 0xFF00) | val as u16;
                self.check_for_interrupt();
            },
            0x201 => {
                self.interrupt_enable = (self.interrupt_enable & 0x00FF) | ((val as u16) << 8);
                self.check_for_interrupt();
            },

            // Interrupt Flags Register
            0x202 => {
                self.interrupt_flags &= !(val as u16);
                self.check_for_interrupt();
            },
            0x203 => {
                self.interrupt_flags &= !((val as u16) << 8);
                self.check_for_interrupt();
                println!("Interrupt flags: {:b}", self.interrupt_flags);
                println!("Interrupt enable: {:b}", self.interrupt_enable);
                println!("Interrupt master enable: {}", self.interrupt_master_enable);
            },

            // Interrupt Master Enable
            0x208 => {
                self.interrupt_master_enable = (val & 0x1) != 0;
                self.check_for_interrupt();
                println!("Interrupt master enable: {}", self.interrupt_master_enable);
            },

            _ => {println!("Writing to unimplemented IO Reg @ {:#4x}, with value {:#x} ({})", address, val, val);},
        };
    }

    pub fn read_dma_register(&self, address: usize) -> u8{
        let dma_channel = (address - 0xB0) / 0xC;

        match (address - 0xB0) % 0xC{
            0x0 => { (self.dma_source_addr[dma_channel] & 0xFF) as u8 },
            0x1 => { ((self.dma_source_addr[dma_channel] >> 8) & 0xFF) as u8 },
            0x2 => { ((self.dma_source_addr[dma_channel] >> 16) & 0xFF) as u8 },
            0x3 => { ((self.dma_source_addr[dma_channel] >> 24) & 0xFF) as u8 },

            0x4 => { (self.dma_destination_addr[dma_channel] & 0xFF) as u8 },
            0x5 => { ((self.dma_destination_addr[dma_channel] >> 8) & 0xFF) as u8 },
            0x6 => { ((self.dma_destination_addr[dma_channel] >> 16) & 0xFF) as u8 },
            0x7 => { ((self.dma_destination_addr[dma_channel] >> 24) & 0xFF) as u8 },

            0x8 => { (self.dma_control[dma_channel] & 0xFF) as u8 },
            0x9 => { ((self.dma_control[dma_channel] >> 8) & 0xFF) as u8 },
            0xA => { ((self.dma_control[dma_channel] >> 16) & 0xFF) as u8 },
            0xB => { ((self.dma_control[dma_channel] >> 24) & 0xFF) as u8 },

            _ => {eprintln!("Impossible to reach!"); 0}

        }
    }

    pub fn write_dma_register(&mut self, address: usize, value: u8){
        let dma_channel = (address - 0xB0) / 0xC;

        match (address - 0xB0) % 0xC{
            0x0 => { self.dma_source_addr[dma_channel] = (self.dma_source_addr[dma_channel] & !0xFF) & (value as u32); },
            0x1 => { self.dma_source_addr[dma_channel] = (self.dma_source_addr[dma_channel] & !0xFF00) & ((value as u32) << 8); },
            0x2 => { self.dma_source_addr[dma_channel] = (self.dma_source_addr[dma_channel] & !0xFF0000) & ((value as u32) << 16); },
            0x3 => { self.dma_source_addr[dma_channel] = (self.dma_source_addr[dma_channel] & !0xFF000000) & ((value as u32) << 24); },

            0x4 => { self.dma_destination_addr[dma_channel] = (self.dma_destination_addr[dma_channel] & !0xFF) & (value as u32); },
            0x5 => { self.dma_destination_addr[dma_channel] = (self.dma_destination_addr[dma_channel] & !0xFF00) & ((value as u32) << 8); },
            0x6 => { self.dma_destination_addr[dma_channel] = (self.dma_destination_addr[dma_channel] & !0xFF0000) & ((value as u32) << 16); },
            0x7 => { self.dma_destination_addr[dma_channel] = (self.dma_destination_addr[dma_channel] & !0xFF000000) & ((value as u32) << 24); },

            0x8 => { self.dma_control[dma_channel] = (self.dma_control[dma_channel] & !0xFF) & (value as u32); },
            0x9 => { self.dma_control[dma_channel] = (self.dma_control[dma_channel] & !0xFF00) & ((value as u32) << 8); },
            0xA => { self.dma_control[dma_channel] = (self.dma_control[dma_channel] & !0xFF0000) & ((value as u32) << 16); },
            0xB => {
                // Highest bit of dma_control register initiates DMA transfer
                self.dma_control[dma_channel] = (self.dma_control[dma_channel] & !0xFF000000) & ((value as u32) << 24);

                let dma_control = self.dma_control[dma_channel];

                let enable_bit = (dma_control & (1 << 31)) != 0;
                println!("DMA TRANSFER: {:32b}", dma_control);
                if enable_bit{
                    let num_transfers = dma_control & 0xFFFF;
                    let destination_adjustment = (dma_control >> 21) & 0b11;
                    let source_adjustment = (dma_control >> 23) & 0b11;
                    let repeat = (dma_control & (1 << 25)) != 0;
                    let transfer_word = (dma_control & (1 << 26)) != 0;
                    let timing_mode = (dma_control >> 28) & 0b11;
                    let raise_irq = (dma_control & (1 << 30)) != 0;

                    let dma_transfer = DMATransfer{
                        channel: dma_channel,
                        num_transfers: num_transfers as usize,
                        destination_adjustment: match destination_adjustment{
                            0 => DMADestinationAdjustment::INCREMENT,
                            1 => DMADestinationAdjustment::DECREMENT,
                            2 => DMADestinationAdjustment::FIXED,
                            3 => DMADestinationAdjustment::RELOAD,
                            _ => DMADestinationAdjustment::INCREMENT
                        },
                        source_adjustment: match source_adjustment{
                            0 => DMASourceAdjustment::INCREMENT,
                            1 => DMASourceAdjustment::DECREMENT,
                            2 => DMASourceAdjustment::FIXED,
                            _ => DMASourceAdjustment::INCREMENT
                        },
                        repeat,
                        transfer_word,
                        timing_mode: match timing_mode {
                            0 => IMMEDIATE,
                            1 => VBLANK,
                            2 => HBLANK,
                            _ => SPECIAL
                        },
                        raise_irq,
                    };

                    println!("Initiated DMA Transfer {:?}",dma_transfer);

                    if dma_transfer.timing_mode == IMMEDIATE{
                        dma_transfer.start(self);
                    }

                }

            },

            _ => {eprintln!("Impossible to reach!");}

        }
    }


    pub fn get_io_reg_16bit(&self, address: usize) -> u16{
        let low = self.read_io_register(address);
        let high = self.read_io_register(address + 1);
        return ((high as u16) << 8) | (low as u16);
    }

    pub fn get_io_reg_32bit(&self, address: usize) -> u32{
        let a = self.read_io_register(address);
        let b = self.read_io_register(address + 1);
        let c = self.read_io_register(address + 2);
        let d = self.read_io_register(address + 3);
        return ((d as u32) << 24) | ((c as u32) << 16) | ((b as u32) << 8) | (a as u32);
    }

    pub fn set_io_reg_16bit(&mut self, address: usize, val: u16){
        self.write_io_register(address, (val & 0xFF) as u8);
        self.write_io_register(address + 1, ((val >> 8) & 0xFF) as u8);
    }

    pub fn set_io_reg_32bit(&mut self, address: usize, val: u32){
        self.write_io_register(address, (val & 0xFF) as u8);
        self.write_io_register(address + 1, ((val >> 8) & 0xFF) as u8);
        self.write_io_register(address + 2, ((val >> 16) & 0xFF) as u8);
        self.write_io_register(address + 3, ((val >> 24) & 0xFF) as u8);
    }
}