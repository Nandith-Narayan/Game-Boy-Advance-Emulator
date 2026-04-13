use super::Memory;

impl Memory{

    pub fn read_io_register(&self, address: usize) -> u8{
        return match address {

            0x6 => {self.vertical_count}, 0x7 => {0},

            _ => {println!("Reading from unimplemented IO Reg @ {:#4x}", address); 0},
        }
    }
    pub fn write_io_register(&mut self, address: usize, val: u8){
        match address {

            0x6 => {self.vertical_count = val;}, 0x7 => {},

            _ => {println!("Writing to unimplemented IO Reg @ {:#4x}, with value {:#x} ({})", address, val, val);},
        };
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