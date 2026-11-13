use crate::memory::Memory;
use crate::ppu::Ppu;

impl Ppu{
    pub fn render_mode_4_scanline(&mut self, mem: &mut Memory){
        let base_idx = (mem.vertical_count as usize)*240;

        for i in base_idx..(base_idx+240) {
            let pixel_palette_idx = if mem.frame_select {mem.vram[i + 0xA000]} else {mem.vram[i]};
            let p_low = mem.palette_ram[(pixel_palette_idx as usize) * 2];
            let p_high = mem.palette_ram[(pixel_palette_idx as usize) * 2 + 1];


            let p = ((p_high as u16)<<8) | (p_low as u16);
            self.r[i] = ((p & 0b011111) as u8) << 3;
            self.g[i] = (((p>>5) & 0b011111) as u8) << 3;
            self.b[i] = (((p>>10) & 0b011111) as u8) << 3;
        }
    }
}