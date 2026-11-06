use crate::memory::Memory;

#[derive(Debug)]
pub struct DMATransfer {
    pub channel: usize,
    pub num_transfers: usize,
    pub destination_adjustment: DMADestinationAdjustment,
    pub source_adjustment: DMASourceAdjustment,
    pub repeat: bool,
    pub transfer_word: bool,
    pub timing_mode: DMATimingMode,
    pub raise_irq: bool,
}


impl DMATransfer {
    pub fn start(&self, mem: &mut Memory){
        let initial_dst_addr = mem.dma_destination_addr[self.channel];
        for _ in 0..self.num_transfers {
            let src_addr = mem.dma_source_addr[self.channel];
            let dst_addr = mem.dma_destination_addr[self.channel];
            let offset;
            if self.transfer_word{
                offset = 4;
                let val = mem.read_32(src_addr);
                mem.write_32(dst_addr, val);
            }else{
                offset = 2;
                let val = mem.read_16(src_addr);
                mem.write_16(dst_addr, val);
            }

            match self.source_adjustment{
                DMASourceAdjustment::INCREMENT => {
                    mem.dma_source_addr[self.channel] += offset;
                },
                DMASourceAdjustment::DECREMENT => {
                    mem.dma_source_addr[self.channel] -= offset;
                },
                DMASourceAdjustment::FIXED => {},
            }
            match self.destination_adjustment{
                DMADestinationAdjustment::INCREMENT | DMADestinationAdjustment::RELOAD => {
                    mem.dma_destination_addr[self.channel] += offset;
                },
                DMADestinationAdjustment::DECREMENT => {
                    mem.dma_destination_addr[self.channel] -= offset;
                },
                DMADestinationAdjustment::FIXED => {},
            }

        }
        if self.destination_adjustment == DMADestinationAdjustment::RELOAD{
            mem.dma_destination_addr[self.channel] = initial_dst_addr;
        }
    }

}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DMADestinationAdjustment {
    INCREMENT,
    DECREMENT,
    FIXED,
    RELOAD,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DMASourceAdjustment {
    INCREMENT,
    DECREMENT,
    FIXED,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DMATimingMode {
    IMMEDIATE,
    VBLANK,
    HBLANK,
    SPECIAL,
}