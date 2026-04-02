use crate::memory::Memory;
use crate::ppu::Ppu;

impl Ppu{
    pub fn draw_mode_3(&mut self, mem: &mut Memory){

        let p_low = mem.vram[self.idx*2];
        let p_high = mem.vram[self.idx*2 +1];

        let p = ((p_high as u16)<<8) | (p_low as u16);
        self.r[self.idx] = ((p & 0b011111) as u8) << 3;
        self.g[self.idx] = (((p>>5) & 0b011111) as u8) << 3;
        self.b[self.idx] = (((p>>10) & 0b011111) as u8) << 3;

        self.idx += 1;

    }
}