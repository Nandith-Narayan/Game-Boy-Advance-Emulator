use crate::cpu::enums::{ARMCondition, ARMInstruction, InstructionSet};
use InstructionSet::{ARM, THUMB};
use crate::cpu::enums::ARMCondition::AL;
use crate::cpu::enums::ARMInstruction::NOP;
use crate::memory;
mod arm_instructions;
mod arm;
mod enums;
mod arm_decode_table;

pub struct Cpu{
    pub r: [u32; 16],
    pub memory: memory::Memory,
    pub fetch_arm: u32,
    pub inst_arm: u32,
    pub decode_arm: ARMInstruction,
    pub condition_arm: ARMCondition,
    pub instruction_set: InstructionSet,
    // Flags (CPSR)
    pub z: bool,
    pub c: bool,
    pub n: bool,
    pub v: bool,
    // ARM instruction decode table
    arm_decode_table: [ARMInstruction; 4096],
}

pub fn init(path: &str) -> Cpu{
    println!("Initializing CPU...");
    let mut cpu = Cpu{
        r: [0; 16],
        memory: memory::init(),
        fetch_arm: 0,
        inst_arm: 0,
        decode_arm: NOP,
        condition_arm: AL,
        instruction_set: ARM,
        // Flags cleared
        z: false,
        c: false,
        n: false,
        v: false,
        // ARM instruction decode table
        arm_decode_table: arm_decode_table::generate_decode_table(),
    };

    cpu.load_rom(path.to_string());

    // Set PC to cartridge entry point
    cpu.r[15] = 0x8000000;
    return cpu;
}
impl Cpu{
    pub fn tick_cycle(&mut self){
        match self.instruction_set {
            ARM => {
                self.execute_arm();
                self.decode_arm();
                self.fetch_arm();


                self.r[15] += 4;
            },
            THUMB => {
                println!("THUMB Mode is unsupported");
            }
        }
    }
    pub fn load_rom(&mut self, rom_file: String) {
        println!("Loading Rom File [{}]", rom_file);
        let data = std::fs::read(rom_file).unwrap();
        for (i,val) in data.iter().enumerate(){
            self.memory.rom[i] = *val;
        }
    }

}

