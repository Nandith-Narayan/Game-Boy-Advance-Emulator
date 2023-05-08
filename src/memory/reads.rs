use super::Memory;

impl Memory{
    pub fn read_32(&mut self, address:u32) -> u32{
        match address{
            // =============== General Memory ===============
            // 0x00000000-0x00003FFF   BIOS - System ROM         (16 KBytes)
            0x00000000..=0x00003FFF => get_u32(&self.bios, address as usize),
            // 0x02000000-0x0203FFFF   WRAM - On-board Work RAM  (256 KBytes)
            0x02000000..=0x0203FFFF => get_u32(&self.wram_on_board, (address - 0x02000000) as usize),
            // 0x03000000-0x03007FFF   WRAM - On-chip Work RAM   (32 KBytes)
            0x03000000..=0x03007FFF => get_u32(&self.wram_on_chip, (address - 0x03000000) as usize),

            // =============== Display Memory ===============
            // 0x05000000-0x050003FF   BG/OBJ Palette RAM        (1 KByte)
            0x05000000..=0x050003FF => get_u32(&self.palette_ram, (address - 0x05000000) as usize),
            // 0x06000000-0x06017FFF   VRAM - Video RAM          (96 KBytes)
            0x06000000..=0x06017FFF => get_u32(&self.vram, (address - 0x06000000) as usize),
            // 0x07000000-0x070003FF   OAM - OBJ Attributes      (1 KByte)
            0x07000000..=0x070003FF => get_u32(&self.oam, (address - 0x07000000) as usize),

            // =============== Game Pak Memory ===============
            // 0x08000000-0x0DFFFFFF   Game Pak ROM
            0x08000000..=0x0DFFFFFF => get_u32(&self.rom, (address - 0x08000000) as usize),
            // 0x0E000000-0x0E00FFFF   Game Pak SRAM    (max 64 KBytes)
            0x0E000000..=0x0E00FFFF => get_u32(&self.sram, (address - 0x0E000000) as usize),
            _ => {eprintln!("32-bit Read at {:#08x}", address); 0},
        }
    }
    pub fn read_16(&mut self, address:u32) -> u16{
        match address{
            // =============== General Memory ===============
            // 0x00000000-0x00003FFF   BIOS - System ROM         (16 KBytes)
            0x00000000..=0x00003FFF => get_u16(&self.bios, address as usize),
            // 0x02000000-0x0203FFFF   WRAM - On-board Work RAM  (256 KBytes)
            0x02000000..=0x0203FFFF => get_u16(&self.wram_on_board, (address - 0x02000000) as usize),
            // 0x03000000-0x03007FFF   WRAM - On-chip Work RAM   (32 KBytes)
            0x03000000..=0x03007FFF => get_u16(&self.wram_on_chip, (address - 0x03000000) as usize),

            // =============== Display Memory ===============
            // 0x05000000-0x050003FF   BG/OBJ Palette RAM        (1 KByte)
            0x05000000..=0x050003FF => get_u16(&self.palette_ram, (address - 0x05000000) as usize),
            // 0x06000000-0x06017FFF   VRAM - Video RAM          (96 KBytes)
            0x06000000..=0x06017FFF => get_u16(&self.vram, (address - 0x06000000) as usize),
            // 0x07000000-0x070003FF   OAM - OBJ Attributes      (1 KByte)
            0x07000000..=0x070003FF => get_u16(&self.oam, (address - 0x07000000) as usize),

            // =============== Game Pak Memory ===============
            // 0x08000000-0x0DFFFFFF   Game Pak ROM
            0x08000000..=0x0DFFFFFF => get_u16(&self.rom, (address - 0x08000000) as usize),
            // 0x0E000000-0x0E00FFFF   Game Pak SRAM    (max 64 KBytes)
            0x0E000000..=0x0E00FFFF => get_u16(&self.sram, (address - 0x0E000000) as usize),
            _ => {eprintln!("16-bit Read at {:#08x}", address); 0},
        }
    }
    pub fn read_8(&mut self, address:u32) -> u8{
        match address{
            // =============== General Memory ===============
            // 0x00000000-0x00003FFF   BIOS - System ROM         (16 KBytes)
            0x00000000..=0x00003FFF => get_u8(&self.bios, address as usize),
            // 0x02000000-0x0203FFFF   WRAM - On-board Work RAM  (256 KBytes)
            0x02000000..=0x0203FFFF => get_u8(&self.wram_on_board, (address - 0x02000000) as usize),
            // 0x03000000-0x03007FFF   WRAM - On-chip Work RAM   (32 KBytes)
            0x03000000..=0x03007FFF => get_u8(&self.wram_on_chip, (address - 0x03000000) as usize),

            // =============== Display Memory ===============
            // 0x05000000-0x050003FF   BG/OBJ Palette RAM        (1 KByte)
            0x05000000..=0x050003FF => get_u8(&self.palette_ram, (address - 0x05000000) as usize),
            // 0x06000000-0x06017FFF   VRAM - Video RAM          (96 KBytes)
            0x06000000..=0x06017FFF => get_u8(&self.vram, (address - 0x06000000) as usize),
            // 0x07000000-0x070003FF   OAM - OBJ Attributes      (1 KByte)
            0x07000000..=0x070003FF => get_u8(&self.oam, (address - 0x07000000) as usize),

            // =============== Game Pak Memory ===============
            // 0x08000000-0x0DFFFFFF   Game Pak ROM
            0x08000000..=0x0DFFFFFF => get_u8(&self.rom, (address - 0x08000000) as usize),
            // 0x0E000000-0x0E00FFFF   Game Pak SRAM    (max 64 KBytes)
            0x0E000000..=0x0E00FFFF => get_u8(&self.sram, (address - 0x0E000000) as usize),
            _ => {eprintln!("8-bit Read at {:#08x}", address); 0},
        }
    }
}

// Helper functions for memory access

// Combines 4 elements of a byte array into a single u32 value
fn get_u32(arr:&[u8], index:usize) -> u32{
    let mut bytes:[u8 ; 4] = [0; 4];
    bytes.copy_from_slice(&arr[index..(index+4)]);
    return u32::from_le_bytes(bytes);
}
// Combines 2 elements of a byte array into a single u16 value
fn get_u16(arr:&[u8], index:usize) -> u16{
    let mut bytes:[u8 ; 2] = [0; 2];
    bytes.copy_from_slice(&arr[index..(index+2)]);
    return u16::from_le_bytes(bytes);
}
// Returns value in array
fn get_u8(arr:&[u8], index:usize) -> u8{
    return arr[index];
}

