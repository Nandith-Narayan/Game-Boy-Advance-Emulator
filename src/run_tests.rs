#[cfg(test)]
mod run_tests {
    use crate::{cpu, memory};

    #[test]
    pub fn test_arm_rom(){
        let mut mem = memory::init();
        mem.load_rom(std::string::String::from("C:/GBA roms/arm.gba"));
        let mut cpu = cpu::init();

        for _ in 1..1000{

            cpu.tick_cycle(&mut mem);
        }
        println!("R[12] = {:#4x} ({})", cpu.r[12], cpu.r[12]);
        assert_eq!(cpu.r[12], 0);
    }



}