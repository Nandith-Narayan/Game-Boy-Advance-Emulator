use crate::cpu::arm_decode::decode_instruction;
use crate::cpu::enums::ARMCondition::*;
use crate::cpu::enums::ARMInstruction::*;
use crate::memory::Memory;
use super::Cpu;

// Implementation of functions related to ARM mode of the CPU
impl Cpu{
    // Fetch ARM instruction
    pub fn fetch_arm(&mut self, mem: &mut Memory) {
        if self.fetch_arm != 0{
            return;
        }
        self.fetch_arm = mem.read_32(self.r[15]);
        self.r[15] += 4;
    }
    // Decode ARM instruction
    pub fn decode_arm(&mut self, mem: &mut Memory){
        if self.fetch_arm == 0 {
            return;
        }
        let inst = self.fetch_arm;
        self.fetch_arm = 0;
        // decode condition
        self.condition_arm = match (inst & 0xF0000000)>>28{
            0 => EQ,
            1 => NE,
            2 => CS,
            3 => CC,
            4 => MI,
            5 => PL,
            6 => VS,
            7 => VC,
            8 => HI,
            9 => LS,
            10 => GE,
            11 => LT,
            12 => GT,
            13 => LE,
            14 => AL,
            _ => {println!("UNDEFINED CONDITION ON INSTRUCTION {}", inst); AL},
        };
        /*let low = (inst & 0xF0) >> 4;
        let high = (inst & 0x0FF00000) >> 20;
        self.decode_arm = self.arm_decode_table[((high << 4) | low) as usize];*/
        self.decode_arm = decode_instruction(inst);
        if inst == 0{
            self.decode_arm = NOP;
        }
        self.inst_arm = inst;
        let mut x = self.inst_arm;
        let str = format!("{:032b}",x);

        /*print!("Instruction: {:?} ( ", self.decode_arm);
        for i in 0..8{
            print!("{} ", str.get(i*4..i*4+4).unwrap());
        }
        println!(") ({:#x})", x);*/
        //println!("{:?}", self.r);
    }
    // Execute ARM instruction
    pub fn execute_arm(&mut self, mem: &mut Memory){
        if self.decode_arm == EMPTY{
            return;
        }
        //println!("{:?}", self.condition_arm);
        let condition = match self.condition_arm{
            EQ => self.z,
            NE => !self.z,
            CS => self.c,
            CC => !self.c,
            MI => self.n,
            PL => !self.n,
            VS => self.v,
            VC => !self.v,
            HI => self.c && !self.z,
            LS => !self.c || self.z,
            GE => self.n == self.v,
            LT => self.n != self.v,
            GT => !self.z && (self.n == self.v),
            LE => self.z || (self.n != self.v),
            AL => true,
        };
        if !condition{
            self.decode_arm = CondIsFalse;
        }
        println!("{:?}", self.decode_arm);
        let mut x = self.inst_arm;
        let str = format!("{:032b}",x);

        print!("Instruction: {:?} ( ", self.decode_arm);
        for i in 0..8{
            print!("{} ", str.get(i*4..i*4+4).unwrap());
        }
        println!(") ({:#x})", x);
        match self.decode_arm{
            CondIsFalse => {}
            NOP => {},
            BranchAndExchange => self.branch_and_exchange(self.inst_arm),
            Branch => self.branch(self.inst_arm),
            BranchWithLink => self.branch_with_link(self.inst_arm),
            DataProcessingReg => self.data_processing_register_operand(self.inst_arm),
            DataProcessingImmediate => self.data_processing_immediate_operand(self.inst_arm),
            SingleDataSwap => self.single_data_swap(self.inst_arm, mem),
            SingleDataTransferReg => self.single_data_transfer_register_operand(self.inst_arm, mem),
            SingleDataTransferImmediate => self.single_data_transfer_immediate_operand(self.inst_arm, mem),
            TransferToPSR => self.transfer_to_program_status_register(self.inst_arm),
            TransferFromPSR => self.transfer_from_program_status_register(self.inst_arm),
            Multiply => self.multiply(self.inst_arm),
            MultiplyLong => self.multiply_long(self.inst_arm),
            _ => println!("Unimplemented ARM instruction: {:?}", self.decode_arm),
        };
    }

    // Flush the pipeline
    pub fn flush_pipeline(&mut self){
        self.fetch_arm = 0;
        self.inst_arm = 0;
        self.decode_arm = EMPTY;
        self.condition_arm = AL;
    }

    // Helper function to populate CPU flags & control bits based on CPSR bytes
    pub fn set_cpsr(&mut self, cpsr: u32){
        self.n = (cpsr & (1 << 31)) != 0;
        self.z = (cpsr & (1 << 30)) != 0;
        self.c = (cpsr & (1 << 29)) != 0;
        self.v = (cpsr & (1 << 28)) != 0;


    }
    // Helper function to convert CPU status into a bit field
    pub fn get_cpsr(&mut self) -> u32{
        let mut cpsr = 0;

        if self.n {cpsr |= 1<<31;}
        if self.z {cpsr |= 1<<30;}
        if self.c {cpsr |= 1<<29;}
        if self.v {cpsr |= 1<<28;}

        cpsr |= self.mode as u32;

        return cpsr;
    }
 }