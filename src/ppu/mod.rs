use std::time::Instant;
use crate::cpu::enums::InterruptType;
use crate::memory::Memory;

mod mode_3;
mod mode_4;
mod mode_5;
mod mode_0;

pub struct Ppu{
    x: usize,

    pub r: Vec<u8>,
    pub g: Vec<u8>,
    pub b: Vec<u8>,
    pub max:usize,
}

pub fn init() -> Ppu{

    return Ppu{
        x: 0,
        r: vec![0; 240*160],
        g: vec![0; 240*160],
        b: vec![0; 240*160],
        max:0,
    }
}

impl Ppu {
    pub fn tick_cycle(&mut self, mem: &mut Memory){
        // Scanline start
        if self.x == 0 && mem.vertical_count < 160{
            match mem.lcd_bg_mode{
                0 => {self.render_mode_0_scanline(mem);},
                
                3 => {self.render_mode_3_scanline(mem);},
                4 => {self.render_mode_4_scanline(mem);},
                5 => {self.render_mode_5_scanline(mem);},
                _ => {}
            }
        }

        // Enter H-Blank
        if self.x == 240{
            mem.h_blank = true;
            if mem.enable_h_blank_irq {
                mem.request_interrupt(InterruptType::HBLANK);
            }
            self.x += 1;
            return;
        }
        // Exit H-Blank
        if self.x >= (240+68){
            mem.h_blank = false;
            self.x = 0;
            mem.vertical_count += 1;
            mem.v_count_triggered = mem.vertical_count == mem.v_count_trigger;
            // Check if VCOUNT interrupt should occur
            if mem.enable_v_count_irq && mem.vertical_count == mem.v_count_trigger{
                mem.request_interrupt(InterruptType::VCOUNT);
            }

            // Enter V-Blank
            if mem.vertical_count == 160{
                mem.v_blank = true;
                if mem.enable_v_blank_irq {
                    mem.request_interrupt(InterruptType::VBLANK);
                }
                println!("DISPSTAT: {:x}, IF: {:x}", mem.read_16(0x04000004), mem.read_16(0x04000202));
            }

            // Exit V-Blank
            if mem.vertical_count >= (160+68){
                mem.v_blank = false;
                mem.vertical_count = 0;
            }

            return;
        }

        self.x += 1;
    }
}