use super::Cpu;

impl Cpu{

    pub fn multiply(&mut self, inst: u32){
        let accumulate = (inst & (1<<21)) != 0;
        let set_flags = (inst & (1<<20)) != 0;

        let rd = (inst >> 16) & 0xF;
        let rn = (inst >> 12) & 0xF;
        let rs = (inst >> 8) & 0xF;
        let rm = inst & 0xF;

        let result;
        if accumulate{
            result = (((self.r[rm as usize] as u64 * self.r[rs as usize] as u64) & 0xFFFFFFFF) as u32).wrapping_add(self.r[rn as usize]);
        }else{
            result = ((self.r[rm as usize] as u64 * self.r[rs as usize] as u64) & 0xFFFFFFFF) as u32;
        }

        if set_flags{
            self.z = result == 0;
            self.n = (result & 0x80000000) != 0;
        }

        self.r[rd as usize] = result;
    }

    pub fn multiply_long(&mut self, inst: u32){
        let is_signed = (inst & (1<<22)) != 0;
        let accumulate = (inst & (1<<21)) != 0;
        let set_flags = (inst & (1<<20)) != 0;

        let rd_high = (inst >> 16) & 0xF;
        let rd_low = (inst >> 12) & 0xF;
        let rs = (inst >> 8) & 0xF;
        let rm = inst & 0xF;

        let mut result;

        if is_signed{
            let mut signed_result: i64;
            // Need to first cast to i32 and then i64, to sign-extend the 32bit number to 64bits.
            signed_result = (self.r[rm as usize] as i32 as i64).wrapping_mul(self.r[rs as usize] as i32 as i64);
            if accumulate {
                signed_result = signed_result.wrapping_add((((self.r[rd_high as usize] as u64) << 32) | self.r[rd_low as usize] as u64) as i64);
            }
            result = signed_result as u64;

        }else {
            result = self.r[rm as usize] as u64 * self.r[rs as usize] as u64;
            if accumulate {
                result = result.wrapping_add(((self.r[rd_high as usize] as u64) << 32) | self.r[rd_low as usize] as u64);
            }
        }

        if set_flags{
            self.z = result == 0;
            self.n = (result & (1 << 31)) != 0;
        }

        self.r[rd_high as usize] = ((result >> 32) & 0xFFFFFFFF) as u32;
        self.r[rd_low as usize] = (result & 0xFFFFFFFF) as u32;
    }

}