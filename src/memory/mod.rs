mod writes;
mod reads;
mod io_registers;

pub struct Memory{
    // General Memory
    bios: [u8; 1<<14],
    pub wram_on_board: Vec<u8>,
    pub wram_on_chip: [u8; 1<<15],

    // Display Memory
    pub palette_ram: [u8; 1<<10],
    pub vram: [u8; 3*(1<<15)],
    oam: [u8; 1<<10],

    // IO Control Registers
    // LCD Control (0x4000000)
    pub lcd_bg_mode: u8,
    pub v_blank: bool,
    pub h_blank: bool,

    // Vertical Counter (0x4000006)
    pub vertical_count: u8,

    // Game Pak
    pub rom: Vec<u8>,
    sram: [u8; 1<<16],
}

pub fn init() -> Memory{
    println!("Initializing Memory...");
    return Memory{
        // General Memory
        bios: [0; 1<<14],
        wram_on_board: vec![0; 1<<18],
        wram_on_chip: [0; 1<<15],

        // Display Memory
        palette_ram: [0; 1<<10],
        vram: [0; 3*(1<<15)],
        oam: [0; 1<<10],

        lcd_bg_mode: 4,
        vertical_count: 0,
        v_blank: false,
        h_blank: false,

        // Game Pak Memory
        rom: vec![0; 3*(1<<25)],
        sram: [0; 1<<16],
    };
}

impl Memory{
    pub fn load_rom(&mut self, rom_file: String) {
        println!("Loading Rom File [{}]", rom_file);
        let data = std::fs::read(rom_file).unwrap();
        for (i,val) in data.iter().enumerate(){
            self.rom[i] = *val;
        }
    }
}