use crate::cpu::thumb_decode::decode_instruction;
use crate::cpu::Cpu;
use crate::cpu::enums::THUMBInstruction::*;
use crate::memory::Memory;

// Implementation of functions related to THUMB mode of the CPU
impl Cpu{
    pub fn fetch_thumb(&mut self, mem: &mut Memory){
        if self.fetch_thumb != 0{
            return;
        }
        self.fetch_thumb = mem.read_16(self.r[15]);
        self.r[15] += 2;
    }

    pub fn decode_thumb(&mut self, mem: &mut Memory){
        if self.fetch_thumb == 0 {
            return;
        }
        let inst = self.fetch_thumb;
        self.fetch_thumb = 0;

        self.decode_thumb = decode_instruction(inst);
        if inst == 0{
            self.decode_thumb = NOP;
        }
        self.inst_thumb = inst;

    }

    pub fn execute_thumb(&mut self, mem: &mut Memory){
        if self.decode_thumb == EMPTY{
            return;
        }

        println!("{:?}", self.decode_thumb);
        let x = self.inst_thumb;
        let str = format!("{:016b}",x);

        print!("Instruction: {:?} ( ", self.decode_thumb);
        for i in 0..4{
            print!("{} ", str.get(i*4..i*4+4).unwrap());
        }
        println!(") ({:#x})", x);

        match self.decode_thumb {
            MoveShiftedRegister => self.move_shifted_register(self.inst_thumb, mem),
            AddOrSubtract => self.add_or_subtract(self.inst_thumb, mem),
            MoveImmediate => self.move_immediate(self.inst_thumb, mem),
            CompareImmediate => self.compare_immediate(self.inst_thumb, mem),
            SubtractImmediate => self.sub_immediate(self.inst_thumb, mem),
            AddImmediate => self.add_immediate(self.inst_thumb, mem),
            ALUOperation => self.alu_operation(self.inst_thumb, mem),
            BranchAndExchange => self.thumb_branch_and_exchange(self.inst_thumb, mem),
            HighRegisterOperation => self.high_register_operation(self.inst_thumb, mem),

            ConditionalBranch => self.conditional_branch(self.inst_thumb, mem),
            _ => println!("Unimplemented THUMB instruction: {:?}", self.decode_thumb),
        };
    }
}