mod cpu;
mod memory;
mod ppu;

#[cfg(test)]
mod run_tests;



use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 240;
const HEIGHT: usize = 160;

fn main() {

    let mut buffer: Vec<u32> = vec![0x0; WIDTH * HEIGHT];
    let window_options = WindowOptions {
        borderless: false,
        title: true,
        resize: false,
        scale: Scale::X4,
        scale_mode: ScaleMode::Stretch,
        topmost: false,
        transparency: false,
        none: false,
    };
    let mut window = Window::new(
        "GBA Emulator",
        WIDTH,
        HEIGHT,
        window_options,
    )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
    window.set_target_fps(60);
    let mut mem = memory::init();
    mem.load_rom(String::from("C:/GBA roms/pageflip.gba"));
    let mut cpu = cpu::init();
    let mut ppu = ppu::init();
    //let now = Instant::now();
    let n = 200_000;//447795;//447795;
    /*for address in 0..4{
        print!("{:#4x}", cpu.memory.read_8(address+0x8000000))
    }
    println!("\n {:#x}",cpu.memory.read_32(0x8000000));*/
    let mut flag=0;
    while true {
        for _i in 1..n {
            //println!("R[7] = {:#4x} ({})", cpu.r[7], cpu.r[7]);
            //println!("R[12] = {:#4x} ({})", cpu.r[12], cpu.r[12]);
            /*if cpu.r[15] == 0x0800048A {
                flag = 5;
            }*/
            if flag > 0 {
                flag -= 1;
                //println!("R[0] = {:#4x}, R[1] = {:#4x}, R[2] = {:#4x}, R[3] = {:#4x}, R[4] = {:#4x}, R[5] = {:#4x}, SP = {:#4x}, LR = {:#4x}, PC = {:#4x}, Carry Flag: {}, Neg Flag: {}, V Flag: {}", cpu.r[0], cpu.r[1], cpu.r[2], cpu.r[3], cpu.r[4], cpu.r[5], cpu.r[13], cpu.r[14], cpu.r[15], cpu.c, cpu.n, cpu.v);
                //println!("{}", mem.vertical_count);
            }
            /*if cpu.r[7]>200{
                break;
            }*/
            //println!("R[0] = {:#4x} ({})", cpu.r[0], cpu.r[0]);

            cpu.tick_cycle(&mut mem);
            ppu.tick_cycle(&mut mem);
        }
        for i in 0..(240 * 160) {
            buffer[i] = ((ppu.r[i] as u32) << 16) | ((ppu.g[i] as u32) << 8) | ((ppu.b[i] as u32)) | 0xFF000000;

        }
        //break;
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
    /*while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }*/

    /*let val = now.elapsed().as_nanos();
    println!("Took {} ns to execute {} instructions. ({} ns per instruction)", val, n, val/n);*/
    //println!("R[12] = {:#4x} ({})", cpu.r[12], cpu.r[12]);
    //println!("R[7] = {:#4x} ({})", cpu.r[7], cpu.r[7]);
    for address in 0..=0x30{
        print!("{:#4x}", mem.wram_on_board[address])
    }
    println!();
}
