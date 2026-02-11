use crate::cpu::enums::ARMInstruction;
use crate::cpu::enums::ARMInstruction::*;

pub fn decode_instruction(inst: u32) -> ARMInstruction {

    if (inst & 0x0F_F0_00_F0) == 0b0000_0001__0010_0000__0000_0000__0001_0000 {return BranchAndExchange;}
    if (inst & 0x0F_00_00_00) == 0b0000_1010__0000_0000__0000_0000__0000_0000 {return Branch;}
    if (inst & 0x0F_00_00_00) == 0b0000_1011__0000_0000__0000_0000__0000_0000 {return BranchWithLink;}

    if (inst & 0x0F_B0_00_F0) == 0b0000_0001__0000_0000__0000_0000__1001_0000 {return SingleDataSwap;}
    if (inst & 0x0E_00_00_00) == 0b0000_0110__0000_0000__0000_0000__0000_0000 {return SingleDataTransferReg;}
    if (inst & 0x0E_00_00_00) == 0b0000_0100__0000_0000__0000_0000__0000_0000 {return SingleDataTransferImmediate;}

    if (inst & 0x0F_C0_00_F0) == 0b0000_0000__0000_0000__0000_0000__1001_0000 {return Multiply;}
    if (inst & 0x0F_80_00_F0) == 0b0000_0000__1000_0000__0000_0000__1001_0000 {return MultiplyLong;}

    if (inst & 0x0D_B0_F0_00) == 0b0000_0001__0010_0000__1111_0000__0000_0000 {return TransferToPSR;}
    if (inst & 0x0F_BF_0F_FF) == 0b0000_0001__0000_1111__0000_0000__0000_0000 {return TransferFromPSR;}
    
    if (inst & 0x0E_00_00_00) == 0b0000_0000__0000_0000__0000_0000__0000_0000 {return DataProcessingReg;}
    if (inst & 0x0C_00_00_00) == 0b0000_0000__0000_0000__0000_0000__0000_0000 {return DataProcessingImmediate;}

    return UNREACHABLE;


}