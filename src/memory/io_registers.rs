use super::Memory;

impl Memory{

    pub fn read_io_register(&self, address: usize) -> u8{
        return match address {
            // DISPCNT - Display Control Register
            0x0 => {
                (self.lcd_bg_mode & 0b111) | // B0-B2 - video mode
                ((self.frame_select as u8) << 4) // B4 - bitmap frame select
            },
            // DISPSTAT - Display Status Register
            0x4 => {
                (self.v_blank as u8) |
                ((self.h_blank as u8) << 1)
            },
            0x5 => {0},
            0x6 => {self.vertical_count}, 0x7 => {0},

            // DMA Registers
            0xB0..=0xDF => {self.read_dma_register(address)},


            0x130 => {(self.keypad & 0xFF) as u8},
            0x131 => {((self.keypad >> 8 )& 0xFF) as u8},
            0x132 | 0x133 => {0},

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
            //0x6 => {}, 0x7 => {},

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
            0x10 => { ((self.dma_control[dma_channel] >> 16) & 0xFF) as u8 },
            0x11 => { ((self.dma_control[dma_channel] >> 24) & 0xFF) as u8 },

            _ => {eprintln!("Impossible to reach!"); 0}

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