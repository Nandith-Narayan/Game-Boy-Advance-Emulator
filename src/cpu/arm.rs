use crate::cpu::enums::ARMCondition::*;
use crate::cpu::enums::ARMInstruction::*;
use super::Cpu;

// Implementation of functions related to ARM mode of the CPU
impl Cpu{
    // Fetch ARM instruction
    pub fn fetch_arm(&mut self){
        self.fetch_arm = self.memory.read_32(self.r[15]);
    }
    // Decode ARM instruction
    pub fn decode_arm(&mut self){
        let inst = self.fetch_arm;
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
        let low = (inst & 0xF0) >> 4;
        let high = (inst & 0x0FF00000) >> 20;
        self.decode_arm = self.arm_decode_table[((high << 4) | low) as usize];
        if inst == 0{
            self.decode_arm = NOP;
        }
        self.inst_arm = inst;
    }
    // Execute ARM instruction
    pub fn execute_arm(&mut self){


        let condition = match self.condition_arm{
            EQ => self.z,
            NE => !self.z,
            CS => self.c,
            CC => !self.c,
            MI => self.n,
            PL => !self.c,
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
        //println!("{:?}", self.decode_arm);
        match self.decode_arm{
            CondIsFalse => {}
            NOP => {},
            Branch => self.branch(self.inst_arm),
            BranchWithLink => self.branch_with_link(self.inst_arm),
            DataProcessingReg => self.data_processing_register_operand(self.inst_arm),
            SingleDataSwap => self.single_data_swap(self.inst_arm),
            SingleDataTransferReg => self.single_data_transfer_register_operand(self.inst_arm),
            SingleDataTransferImmediate => self.single_data_transfer_immediate_operand(self.inst_arm),
            _ => println!("Unimplemented ARM instruction: {:?}", self.decode_arm),
        };
    }

    // Flush the pipeline
    pub fn flush_pipeline(&mut self){
        self.fetch_arm = 0;
        self.inst_arm = 0;
        self.decode_arm = NOP;
        self.condition_arm = AL;
    }
}