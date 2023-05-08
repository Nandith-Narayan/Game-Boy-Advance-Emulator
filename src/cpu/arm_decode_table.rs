use std::time::Instant;
use crate::cpu::enums::ARMInstruction;
use crate::cpu::enums::ARMInstruction::*;

pub fn generate_decode_table() -> [ARMInstruction; 4096] {
    let now = Instant::now();

    let mut decode_table = [NOP; 4096];

    for i in 0..decode_table.len(){
        let low = i & 0xF;
        let high = (i & 0xFF0) >> 4;
        let inst = match (high, low){
            (0b00010010, 0b0001) => BranchAndExchange,
            _ if ((high & 0xF0) >> 4) == 0b1010 => Branch,
            _ if ((high & 0xF0) >> 4) == 0b1011 => BranchWithLink,
            (_, 0b1001) if (high & 0b11111011) == 0b10000 => SingleDataSwap,
            _ if (high & 0b11100000) == 0b01100000 => SingleDataTransferReg,
            _ if (high & 0b11100000) == 0b01000000 => SingleDataTransferImmediate,
            (_, 0b1001) if (high & 0xFC) == 0  => Multiply,
            (_, 0b1001) if ((high & 0xF8) >> 3) == 1  => MultiplyLong,
            _ if (high & 0b1110000000000000) == 0 => DataProcessingReg,
            _ if (high & 0b1100000000000000) == 0 => DataProcessingImmediate,
            _ => UNREACHABLE,
        };
        decode_table[i] = inst;
    }

    let elapsed_time = now.elapsed().as_nanos();
    println!("Generated ARM instruction decode table [{} ns]", elapsed_time);
    return decode_table;
}