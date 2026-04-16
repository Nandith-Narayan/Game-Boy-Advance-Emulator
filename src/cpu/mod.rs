use crate::cpu::enums::{ARMCondition, ARMInstruction, CPUMode, InstructionSet, THUMBInstruction};
use InstructionSet::{ARM, THUMB};
use crate::cpu::enums::ARMCondition::AL;
use crate::cpu::enums::CPUMode::SUPERVISOR;
use crate::cpu::enums::RegisterName::*;
use crate::memory::Memory;

pub(crate) mod enums;
mod arm_instructions;
mod arm;
mod arm_decode;
mod thumb_instructions;
mod thumb;
mod thumb_decode;
mod helper_functions;

pub struct Cpu{
    pub regs: [u32; 27],
    pub reg_map: [usize; 16],
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
    let reg_map: [usize; 16] = [R0 as usize, R1 as usize, R2 as usize, R3 as usize,
        R4 as usize, R5 as usize, R6 as usize, R7 as usize,
        R8 as usize, R9 as usize, R10 as usize, R11 as usize,
        R12 as usize, R13 as usize, R14 as usize, R15 as usize];

    let mut cpu = Cpu{
        regs: [0; 27],
        reg_map,
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
    cpu.set_r(15,  0x8000000);
    // Init Stack Pointer
    cpu.set_r(13,  0x3007F00);
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
    #[inline(always)]
    pub fn set_r(&mut self, r: usize, val: u32){
        self.regs[self.reg_map[r]] = val;
    }

    #[inline(always)]
    pub fn get_r(&self, r: usize) -> u32{
        return self.regs[self.reg_map[r]];
    }

    #[inline(always)]
    pub fn increment_r(&mut self, r: usize, val: u32){
        self.regs[self.reg_map[r]] += val;
    }
    #[inline(always)]
    pub fn decrement_r(&mut self, r: usize, val: u32){
        self.regs[self.reg_map[r]] -= val;
    }
}

