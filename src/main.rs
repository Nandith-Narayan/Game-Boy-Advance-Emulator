mod cpu;
mod memory;
mod ppu;

#[cfg(test)]
mod run_tests;


use std::time::Instant;
use std::thread;
use std::time::Duration;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 240;
const HEIGHT: usize = 160;

fn main() {

    let mut buffer: Vec<u32> = vec![0x0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "GBA Emulator",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
    window.set_target_fps(60);
    let mut mem = memory::init();
    mem.load_rom(String::from("C:/GBA roms/arm.gba"));
    let mut cpu = cpu::init();
    let mut ppu = ppu::init();
    //let now = Instant::now();
    let n = 2000;
    /*for address in 0..4{
        print!("{:#4x}", cpu.memory.read_8(address+0x8000000))
    }
    println!("\n {:#x}",cpu.memory.read_32(0x8000000));*/

    for _ in 1..n {
        println!("R[12] = {:#4x} ({})", cpu.r[12], cpu.r[12]);
        println!("R[0] = {:#4x}, R[1] = {:#4x}, R[2] = {:#4x}, R[3] = {:#4x}, R[4] = {:#4x}, R[5] = {:#4x}, PC = {:#4x}, Carry Flag: {}", cpu.r[0], cpu.r[1], cpu.r[2], cpu.r[3], cpu.r[4], cpu.r[5], cpu.r[15], cpu.c);
        //println!("R[0] = {:#4x} ({})", cpu.r[0], cpu.r[0]);
        cpu.tick_cycle(&mut mem);
        ppu.tick_cycle(&mut mem);
        if (cpu.r[12] > 100) {
            break;
        }
    }
    for i in 0..(240*160){
        buffer[i] = ((ppu.r[i] as u32) << 24)| ((ppu.g[i] as u32) << 16) | ((ppu.g[i] as u32) << 8) | 0x00000000;

    }
    /*while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }*/

    /*let val = now.elapsed().as_nanos();
    println!("Took {} ns to execute {} instructions. ({} ns per instruction)", val, n, val/n);*/
    println!("R[12] = {:#4x} ({})", cpu.r[12], cpu.r[12]);
    for address in 0..=0x30{
        print!("{:#4x}", mem.wram_on_board[address])
    }
    println!();
}
