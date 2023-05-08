use super::Memory;

impl Memory{
    pub fn write_32(&mut self, address:u32, value:u32) {
        match address{
            // =============== General Memory ===============
            // 0x00000000-0x00003FFF   BIOS - System ROM         (16 KBytes)
            0x00000000..=0x00003FFF => set_u32(&mut self.bios, address as usize, value),
            // 0x02000000-0x0203FFFF   WRAM - On-board Work RAM  (256 KBytes)
            0x02000000..=0x0203FFFF => set_u32(&mut self.wram_on_board, (address - 0x02000000) as usize, value),
            // 0x03000000-0x03007FFF   WRAM - On-chip Work RAM   (32 KBytes)
            0x03000000..=0x03007FFF => set_u32(&mut self.wram_on_chip, (address - 0x03000000) as usize, value),

            // =============== Display Memory ===============
            // 0x05000000-0x050003FF   BG/OBJ Palette RAM        (1 KByte)
            0x05000000..=0x050003FF => set_u32(&mut self.palette_ram, (address - 0x05000000) as usize, value),
            // 0x06000000-0x06017FFF   VRAM - Video RAM          (96 KBytes)
            0x06000000..=0x06017FFF => set_u32(&mut self.vram, (address - 0x06000000) as usize, value),
            // 0x07000000-0x070003FF   OAM - OBJ Attributes      (1 KByte)
            0x07000000..=0x070003FF => set_u32(&mut self.oam, (address - 0x07000000) as usize, value),

            // =============== Game Pak Memory ===============
            // 0x08000000-0x0DFFFFFF   Game Pak ROM
            0x08000000..=0x0DFFFFFF => set_u32(&mut self.rom, (address - 0x08000000) as usize, value),
            // 0x0E000000-0x0E00FFFF   Game Pak SRAM    (max 64 KBytes)
            0x0E000000..=0x0E00FFFF => set_u32(&mut self.sram, (address - 0x0E000000) as usize, value),
            _ => eprintln!("32-bit Write at {:#08x} with {:#08x}", address, value),
        };
    }
    pub fn write_16(&mut self, address:u32, value:u16) {
        match address{
            // =============== General Memory ===============
            // 0x00000000-0x00003FFF   BIOS - System ROM         (16 KBytes)
            0x00000000..=0x00003FFF => set_u16(&mut self.bios, address as usize, value),
            // 0x02000000-0x0203FFFF   WRAM - On-board Work RAM  (256 KBytes)
            0x02000000..=0x0203FFFF => set_u16(&mut self.wram_on_board, (address - 0x02000000) as usize, value),
            // 0x03000000-0x03007FFF   WRAM - On-chip Work RAM   (32 KBytes)
            0x03000000..=0x03007FFF => set_u16(&mut self.wram_on_chip, (address - 0x03000000) as usize, value),

            // =============== Display Memory ===============
            // 0x05000000-0x050003FF   BG/OBJ Palette RAM        (1 KByte)
            0x05000000..=0x050003FF => set_u16(&mut self.palette_ram, (address - 0x05000000) as usize, value),
            // 0x06000000-0x06017FFF   VRAM - Video RAM          (96 KBytes)
            0x06000000..=0x06017FFF => set_u16(&mut self.vram, (address - 0x06000000) as usize, value),
            // 0x07000000-0x070003FF   OAM - OBJ Attributes      (1 KByte)
            0x07000000..=0x070003FF => set_u16(&mut self.oam, (address - 0x07000000) as usize, value),

            // =============== Game Pak Memory ===============
            // 0x08000000-0x0DFFFFFF   Game Pak ROM
            0x08000000..=0x0DFFFFFF => set_u16(&mut self.rom, (address - 0x08000000) as usize, value),
            // 0x0E000000-0x0E00FFFF   Game Pak SRAM    (max 64 KBytes)
            0x0E000000..=0x0E00FFFF => set_u16(&mut self.sram, (address - 0x0E000000) as usize, value),
            _ => eprintln!("16-bit Write at {:#08x} with {:#04x}", address, value),
        };
    }
    pub fn write_8(&mut self, address:u32, value:u8) {
        match address{
            // =============== General Memory ===============
            // 0x00000000-0x00003FFF   BIOS - System ROM         (16 KBytes)
            0x00000000..=0x00003FFF => set_u8(&mut self.bios, address as usize, value),
            // 0x02000000-0x0203FFFF   WRAM - On-board Work RAM  (256 KBytes)
            0x02000000..=0x0203FFFF => set_u8(&mut self.wram_on_board, (address - 0x02000000) as usize, value),
            // 0x03000000-0x03007FFF   WRAM - On-chip Work RAM   (32 KBytes)
            0x03000000..=0x03007FFF => set_u8(&mut self.wram_on_chip, (address - 0x03000000) as usize, value),

            // =============== Display Memory ===============
            // 0x05000000-0x050003FF   BG/OBJ Palette RAM        (1 KByte)
            0x05000000..=0x050003FF => set_u8(&mut self.palette_ram, (address - 0x05000000) as usize, value),
            // 0x06000000-0x06017FFF   VRAM - Video RAM          (96 KBytes)
            0x06000000..=0x06017FFF => set_u8(&mut self.vram, (address - 0x06000000) as usize, value),
            // 0x07000000-0x070003FF   OAM - OBJ Attributes      (1 KByte)
            0x07000000..=0x070003FF => set_u8(&mut self.oam, (address - 0x07000000) as usize, value),

            // =============== Game Pak Memory ===============
            // 0x08000000-0x0DFFFFFF   Game Pak ROM
            0x08000000..=0x0DFFFFFF => set_u8(&mut self.rom, (address - 0x08000000) as usize, value),
            // 0x0E000000-0x0E00FFFF   Game Pak SRAM    (max 64 KBytes)
            0x0E000000..=0x0E00FFFF => set_u8(&mut self.sram, (address - 0x0E000000) as usize, value),
            _ => eprintln!("8-bit Write at {:#08x} with {:#02x}", address, value),
        };
    }

}


// Helper functions for memory access

// Writes a 32 bit value to 4 bytes of an array
fn set_u32(arr: &mut [u8], index:usize, value:u32) {
    arr[index] = (value &0xFF)as u8;
    arr[index+1] = ((value &0xFF00) >> 8)as u8;
    arr[index+2] = ((value &0xFF0000) >> 16)as u8;
    arr[index+3] = ((value &0xFF000000) >> 24)as u8;
}
// Writes a 16 bit value to 2 bytes of an array
fn set_u16(arr: &mut [u8], index:usize, value:u16) {
    arr[index] = (value &0xFF)as u8;
    arr[index+1] = ((value &0xFF00) >> 8)as u8;
}
// Writes an 8 bit value to an array
fn set_u8(arr:&mut [u8], index:usize, value:u8) {
    arr[index] = value;
}