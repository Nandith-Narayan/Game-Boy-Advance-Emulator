mod writes;
mod reads;

pub struct Memory{
    // General Memory
    bios: [u8; 1<<14],
    pub wram_on_board: [u8; 1<<18],
    pub wram_on_chip: [u8; 1<<15],

    // Display Memory
    palette_ram: [u8; 1<<10],
    vram: [u8; 3*(1<<15)],
    oam: [u8; 1<<10],

    // Game Pak
    pub rom: Vec<u8>,
    sram: [u8; 1<<16],
}

pub fn init() -> Memory{
    println!("Initializing Memory...");
    return Memory{
        // General Memory
        bios: [0; 1<<14],
        wram_on_board: [0; 1<<18],
        wram_on_chip: [0; 1<<15],

        // Display Memory
        palette_ram: [0; 1<<10],
        vram: [0; 3*(1<<15)],
        oam: [0; 1<<10],

        // Game Pak Memory
        rom: vec![0; 3*(1<<25)],
        sram: [0; 1<<16],
    };
}

