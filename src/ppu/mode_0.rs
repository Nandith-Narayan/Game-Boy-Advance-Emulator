use crate::memory::Memory;
use crate::ppu::Ppu;

impl Ppu{
    pub fn render_mode_0_scanline(&mut self, mem: &mut Memory){
        let base_idx = (mem.vertical_count as usize)*240;
        for i in base_idx..(base_idx+240) {
            let p_low = mem.vram[i*2];
            let p_high = mem.vram[i*2 +1];

            let p = ((p_high as u16)<<8) | (p_low as u16);
            self.r[i] = ((p & 0b011111) as u8) << 3;
            self.g[i] = (((p>>5) & 0b011111) as u8) << 3;
            self.b[i] = (((p>>10) & 0b011111) as u8) << 3;
        }
    }
}