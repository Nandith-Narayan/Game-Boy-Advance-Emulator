pub struct BackgroundControl {
    pub priority: usize,
    pub tile_block: usize,
    pub mosaic: bool,
    pub use_8_bits_per_pixel: bool,
    pub map_block: usize,
    pub wraparound: bool,
    pub map_size: usize,
}

impl BackgroundControl {

    pub fn new() -> BackgroundControl {
        BackgroundControl{
            priority: 0,
            tile_block: 0,
            mosaic: false,
            use_8_bits_per_pixel: false,
            map_block: 0,
            wraparound: false,
            map_size: 0,
        }
    }

    pub fn set_lower_byte(&mut self, byte: u8){
        self.priority = (byte & 0b11) as usize;
        self.tile_block = ((byte & 0b1100) >> 2) as usize;
        self.mosaic = (byte & 0b100_0000) != 0;
        self.use_8_bits_per_pixel = (byte & 0b1000_0000) != 0;
    }

    pub fn set_upper_byte(&mut self, byte: u8){
        self.map_block = (byte & 0b1_1111) as usize;
        self.wraparound = (byte & 0b10_0000) != 0;
        self.map_size = ((byte & 0b1100_0000) >> 6) as usize;
    }

    pub fn get_lower_byte(&self) -> u8{
        let mut byte = 0;
        byte |= self.priority as u8;
        byte |= (self.tile_block as u8) << 2;
        byte |= (self.mosaic as u8) << 6;
        byte |= (self.use_8_bits_per_pixel as u8) << 7;

        return byte;
    }

    pub fn get_upper_byte(&self) -> u8{
        let mut byte = 0;
        byte |= self.map_block as u8;
        byte |= (self.wraparound as u8) << 5;
        byte |= (self.map_size as u8) << 6;

        return byte;
    }
}