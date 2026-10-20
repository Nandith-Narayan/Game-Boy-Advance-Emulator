use crate::memory::Memory;
use crate::ppu::Ppu;

impl Ppu{
    pub fn draw_mode_5(&mut self, mem: &mut Memory){
        let frame_offset = if mem.frame_select {0xA000} else {0};
        let p_low = mem.vram[self.idx*2 + frame_offset];
        let p_high = mem.vram[self.idx*2 + 1 + frame_offset];
        let p = ((p_high as u16)<<8) | (p_low as u16);





        if self.x >= 160 || mem.vertical_count >= 128{
            self.r[self.idx+self.m5_idx] = 0;
            self.g[self.idx+self.m5_idx] = 0;
            self.b[self.idx+self.m5_idx] = 0;
            self.m5_idx += 1;
        }else{
            self.r[self.idx+self.m5_idx] = ((p & 0b011111) as u8) << 3;
            self.g[self.idx+self.m5_idx] = (((p>>5) & 0b011111) as u8) << 3;
            self.b[self.idx+self.m5_idx] = (((p>>10) & 0b011111) as u8) << 3;
            self.idx += 1;
        }
    }
}