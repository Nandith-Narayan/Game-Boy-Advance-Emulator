use crate::cpu::enums::InterruptType;
use super::Memory;

impl Memory {
    pub fn request_interrupt(&mut self, interrupt: InterruptType){
        self.interrupt_flags |= interrupt as u16;
        self.check_for_interrupt();
    }
    pub fn check_for_interrupt(&mut self){
        self.interrupt_ready = self.interrupt_master_enable
                            && ((self.interrupt_enable & self.interrupt_flags)!= 0);
    }
}