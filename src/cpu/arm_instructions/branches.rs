use crate::cpu::enums::InstructionSet::THUMB;
use super::Cpu;

impl Cpu{

    pub fn branch_and_exchange(&mut self, inst: u32) {
        let mut rn: usize = (inst & 0xF) as usize;
        self.r[15] = self.r[rn]&0xFFFFFFFE;
        if self.r[rn] & 0x1 != 0{
            self.r[15] -= 4;
            self.instruction_set = THUMB;
        }

        self.flush_pipeline();

    }

    pub fn branch(&mut self, inst: u32) {
        let mut offset: i32 = (((inst & 0x00FFFFFF) << 8) as i32) >> 8;
        offset <<= 2;
        self.r[15] = ((self.r[15] as i32) + offset) as u32;
        self.flush_pipeline();
    }

    pub fn branch_with_link(&mut self, inst: u32) {
        let mut offset: i32 = (((inst & 0x00FFFFFF) << 8) as i32) >> 8;
        offset <<= 2;
        // Compensate for instruction prefetching.
        // The link register should have the address of the instruction right after the branch instruction
        self.r[14] = self.r[15]-4;
        self.r[15] = ((self.r[15] as i32) + offset) as u32;
        self.flush_pipeline();
    }
    
}