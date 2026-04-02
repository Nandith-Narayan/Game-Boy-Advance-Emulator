mod cpu;
mod memory;
#[cfg(test)]
mod run_tests;

use std::time::Instant;
use std::thread;
use std::time::Duration;

fn main() {
    let mut mem = memory::init();
    mem.load_rom(String::from("C:/GBA roms/arm.gba"));
    let mut cpu = cpu::init();
    let now = Instant::now();
    let n = 1000;
    /*for address in 0..4{
        print!("{:#4x}", cpu.memory.read_8(address+0x8000000))
    }
    println!("\n {:#x}",cpu.memory.read_32(0x8000000));*/

    for _ in 1..n{
        println!("R[12] = {:#4x} ({})", cpu.r[12], cpu.r[12]);
        println!("R[0] = {:#4x}, R[1] = {:#4x}, PC = {:#4x}, Carry Flag: {}", cpu.r[0], cpu.r[1], cpu.r[15], cpu.c);
        //println!("R[0] = {:#4x} ({})", cpu.r[0], cpu.r[0]);
        cpu.tick_cycle(&mut mem);
        if (cpu.r[12] ==224){
            break;
        }
    }
    let val = now.elapsed().as_nanos();
    println!("Took {} ns to execute {} instructions. ({} ns per instruction)", val, n, val/n);
    println!("R[12] = {:#4x} ({})", cpu.r[12], cpu.r[12]);
    for address in 0..=0x30{
        print!("{:#4x}", mem.wram_on_board[address])
    }
    println!();
}
