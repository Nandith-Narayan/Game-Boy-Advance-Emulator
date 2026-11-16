use crate::memory::background_control::BackgroundControl;

mod writes;
mod reads;
mod io_registers;
mod dma_transfers;
mod interrupts;
mod background_control;

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
    pub frame_select: bool,
    pub bg_enabled: [bool; 4],

    // LCD Status (0x4000002)
    pub v_blank: bool,
    pub h_blank: bool,

    // Vertical Counter (0x4000006)
    pub vertical_count: u8,

    // Display Control Registers (0x4000008 to 0x400000F)
    pub bg_control: [BackgroundControl; 4],

    pub bg_scroll_regs: [u16; 8],

    pub dma_source_addr: [u32; 4],
    pub dma_destination_addr: [u32; 4],
    pub dma_control: [u32; 4],

    // Keypad Inputs
    pub keypad: u16,

    // Interrupt Control Registers
    pub interrupt_master_enable: bool,
    pub interrupt_enable: u16,
    pub interrupt_flags: u16,
    pub interrupt_ready: bool,

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

        lcd_bg_mode: 3,
        frame_select: false,
        bg_enabled: [false; 4],
        vertical_count: 0,
        v_blank: false,
        h_blank: false,

        bg_control: [BackgroundControl::new(),
                     BackgroundControl::new(),
                     BackgroundControl::new(),
                     BackgroundControl::new()],
        bg_scroll_regs: [0; 8],

        dma_source_addr: [0; 4],
        dma_destination_addr: [0; 4],
        dma_control: [0; 4],

        keypad: 0xFFFF,

        interrupt_master_enable: false,
        interrupt_enable: 0,
        interrupt_flags: 0,
        interrupt_ready: false,

        // Game Pak Memory
        rom: vec![0; 3*(1<<25)],
        sram: [0; 1<<16],
    };
}

impl Memory{
    pub fn load_bios(&mut self, rom_file: String) {
        println!("Loading Open Source BIOS File [{}]", rom_file);
        let data = std::fs::read(rom_file).unwrap();
        for (i,val) in data.iter().enumerate(){
            self.bios[i] = *val;
        }
    }
    pub fn load_rom(&mut self, rom_file: String) {
        println!("Loading Rom File [{}]", rom_file);
        let data = std::fs::read(rom_file).unwrap();
        for (i,val) in data.iter().enumerate(){
            self.rom[i] = *val;
        }
    }
}