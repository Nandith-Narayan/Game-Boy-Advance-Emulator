use crate::memory::Memory;
use crate::ppu::Ppu;

impl Ppu{
    pub fn draw_mode_4(&mut self, mem: &mut Memory){
        let pixel_palette_idx = mem.vram[self.idx];
        let p_low = mem.palette_ram[(pixel_palette_idx as usize) * 2];
        let p_high = mem.palette_ram[(pixel_palette_idx as usize) * 2 + 1];

        let p = ((p_high as u16)<<8) | (p_low as u16);
        self.r[self.idx] = ((p & 0b011111) as u8) << 3;
        self.g[self.idx] = (((p>>5) & 0b011111) as u8) << 3;
        self.b[self.idx] = (((p>>10) & 0b011111) as u8) << 3;
        self.idx += 1;

    }
}