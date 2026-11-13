use crate::memory::Memory;
use crate::ppu::Ppu;

impl Ppu{
    pub fn render_mode_5_scanline(&mut self, mem: &mut Memory){
        let base_idx = (mem.vertical_count as usize)*240;

        if mem.vertical_count >= 128{
            for i in base_idx..(base_idx+240) {

                self.r[i] = 0;
                self.g[i] = 0;
                self.b[i] = 0;
            }
            return;
        }


        let vram_base_idx = (mem.vertical_count as usize)*160;
        let frame_offset = if mem.frame_select {0xA000} else {0};


        for i in 0..160 {
            let p_low = mem.vram[(vram_base_idx+i)*2 + frame_offset];
            let p_high = mem.vram[(vram_base_idx+i)*2 + 1 + frame_offset];
            let p = ((p_high as u16)<<8) | (p_low as u16);

            self.r[base_idx+i] = ((p & 0b011111) as u8) << 3;
            self.g[base_idx+i] = (((p>>5) & 0b011111) as u8) << 3;
            self.b[base_idx+i] = (((p>>10) & 0b011111) as u8) << 3;
        }
        for i in (base_idx+160)..(base_idx+240) {

            self.r[i] = 0;
            self.g[i] = 0;
            self.b[i] = 0;
        }
    }
}