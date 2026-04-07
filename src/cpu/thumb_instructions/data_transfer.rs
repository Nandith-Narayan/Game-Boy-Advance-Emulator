use crate::memory::Memory;
use super::Cpu;

impl Cpu {
    pub fn pc_relative_load(&mut self, inst: u16, mem: &mut Memory){
        let offset = (inst & 0xFF) << 2;
        let rd = (inst >> 8) & 0b111;

        // Note: PC with bit 1 cleared, is used to maintain word alignment
        self.r[rd as usize] = mem.read_32((self.r[15] & 0xFFFFFFFD) + offset as u32);
    }
}