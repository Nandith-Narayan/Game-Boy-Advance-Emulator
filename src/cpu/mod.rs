use crate::cpu::enums::{ARMCondition, ARMInstruction, CPUMode, InstructionSet, THUMBInstruction};
use InstructionSet::{ARM, THUMB};
use crate::cpu::enums::ARMCondition::AL;
use crate::cpu::enums::CPUMode::SUPERVISOR;
use crate::memory::Memory;

mod enums;
mod arm_instructions;
mod arm;
mod arm_decode;
mod thumb_instructions;
mod thumb;
mod thumb_decode;
mod helper_functions;

pub struct Cpu{
    pub r: [u32; 16],
    pub instruction_set: InstructionSet,

    // ARM
    pub fetch_arm: u32,
    pub inst_arm: u32,
    pub decode_arm: ARMInstruction,
    pub condition_arm: ARMCondition,

    // THUMB
    pub fetch_thumb: u16,
    pub inst_thumb: u16,
    pub decode_thumb: THUMBInstruction,

    // Flags (CPSR)
    pub z: bool,
    pub c: bool,
    pub n: bool,
    pub v: bool,
    pub mode: CPUMode,

    // Banked registers
    pub r_irq: [u32; 16],
    pub spsr_irq: u32,
    pub r_fiq: [u32; 16],
    pub spsr_fiq: u32,
    pub r_svc: [u32; 16],
    pub spsr_svc: u32,

}

pub fn init() -> Cpu{
    println!("Initializing CPU...");
    let mut cpu = Cpu{
        r: [0; 16],
        instruction_set: ARM,
        //memory: memory::init(),
        fetch_arm: 0,
        inst_arm: 0,
        decode_arm: ARMInstruction::EMPTY,
        condition_arm: AL,

        fetch_thumb: 0,
        inst_thumb: 0,
        decode_thumb: THUMBInstruction::EMPTY,

        // Flags cleared
        z: false,
        c: false,
        n: false,
        v: false,

        mode: SUPERVISOR,
        r_irq: [0; 16],
        spsr_irq: 0,
        r_fiq: [0; 16],
        spsr_fiq: 0,
        r_svc: [0; 16],
        spsr_svc: 0,
    };
    /*if(path.to_string().eq("skip")){
        //println!("Skipping ROM Loading");
    }else{
        cpu.load_rom(path.to_string());
    }*/

    // Set PC to cartridge entry point
    cpu.r[15] = 0x8000000;
    // Init Stack Pointer
    cpu.r[13] = 0x3007F00;
    return cpu;
}
impl Cpu{
    pub fn tick_cycle(&mut self, mem: &mut Memory){
        match self.instruction_set {
            ARM => {

                self.execute_arm(mem);

                self.decode_arm(mem);

                self.fetch_arm(mem);

            },
            THUMB => {
                self. execute_thumb(mem);

                self.decode_thumb(mem);

                self.fetch_thumb(mem);
            }
        }
    }


}

