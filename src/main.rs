mod cpu;
mod memory;
use std::time::Instant;
use std::thread;
use std::time::Duration;

fn main() {

    let mut cpu = cpu::init("D:/gba_emu/roms/ARM_Any.gba");
    let now = Instant::now();
    let n = 10_000_000;
    for _ in 1..n{
        cpu.tick_cycle();
    }
    let val = now.elapsed().as_nanos();
    println!("Took {} ns to execute {} instructions. ({} ns per instruction)", val, n, val/n);
    for address in 0..=0x30{
        print!("{:#4x}", cpu.memory.wram_on_board[address])
    }
    println!();
}
