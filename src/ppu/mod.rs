use crate::memory::Memory;

mod mode_3;
mod mode_4;

pub struct Ppu{
    x: usize,
    idx: usize,

    pub r: Vec<u8>,
    pub g: Vec<u8>,
    pub b: Vec<u8>,
    pub max:usize,
}

pub fn init() -> Ppu{

    return Ppu{
        x: 0,
        idx: 0,
        r: vec![0; 240*160],
        g: vec![0; 240*160],
        b: vec![0; 240*160],
        max:0,
    }
}

impl Ppu {
    pub fn tick_cycle(&mut self, mem: &mut Memory){
        if self.x < 240 && mem.vertical_count < 160{
            match mem.lcd_bg_mode {
                3 => self.draw_mode_3(mem),
                4 => self.draw_mode_4(mem),
                _ => {},
            }
        }
        if self.x >= 240 && self.x < (240+68){
            mem.h_blank = true;
        }
        if self.x >= (240+68){
            self.x = 0;
            mem.h_blank = false;
            mem.vertical_count+=1;
            if mem.vertical_count < 160{
                mem.v_blank = false;
            }
            if mem.vertical_count > 160 && mem.vertical_count < 227{
                mem.h_blank = true;
            }
            if mem.vertical_count >= 227{
                mem.v_blank = false;
                mem.vertical_count = 0;
                self.idx = 0;

            }
        }else{
            self.x += 1;
        }


    }

}