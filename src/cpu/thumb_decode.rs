use crate::cpu::enums::THUMBInstruction;
use crate::cpu::enums::THUMBInstruction::*;

pub fn decode_instruction(inst: u16) -> THUMBInstruction{

    if inst & 0b1111_1000__0000_0000 == 0b0001_1000__0000_0000 {return AddOrSubtract;}

    if inst & 0b1110_0000__0000_0000 == 0b0000_0000__0000_0000 {return MoveShiftedRegister;}

    if inst & 0b1111_1000__0000_0000 == 0b0010_0000__0000_0000 {return MoveImmediate;}
    if inst & 0b1111_1000__0000_0000 == 0b0010_1000__0000_0000 {return CompareImmediate;}
    if inst & 0b1111_1000__0000_0000 == 0b0011_0000__0000_0000 {return AddImmediate;}
    if inst & 0b1111_1000__0000_0000 == 0b0011_1000__0000_0000 {return SubtractImmediate;}

    if inst & 0b1111_1100__0000_0000 == 0b0100_0000__0000_0000 {return ALUOperation;}

    if inst & 0b1111_1111__0000_0000 == 0b0100_0111__0000_0000 {return BranchAndExchange;}
    if inst & 0b1111_1100__0000_0000 == 0b0100_0100__0000_0000 {return HighRegisterOperation;}

    if inst & 0b1111_1000__0000_0000 == 0b0100_1000__0000_0000 {return PCRelativeLoad;}

    if inst & 0b1111_0010__0000_0000 == 0b0101_0000__0000_0000 {return SingleDataTransfer;}

    if inst & 0b1111_0010__0000_0000 == 0b0101_0010__0000_0000 {return LoadOrStoreSignExtendedHalfWord;}

    if inst & 0b1110_0000__0000_0000 == 0b0110_0000__0000_0000 {return SingleDataTransferWithImmediateOffset;}

    if inst & 0b1111_0000__0000_0000 == 0b1000_0000__0000_0000 {return HalfWordTransferWithImmediateOffset;}

    if inst & 0b1111_0000__0000_0000 == 0b1001_0000__0000_0000 {return SPRelativeLoadOrStore;}

    if inst & 0b1111_0000__0000_0000 == 0b1010_0000__0000_0000 {return LoadAddress;}

    if inst & 0b1111_1111__0000_0000 == 0b1011_0000__0000_0000 {return AddOffsetToSP;}

    if inst & 0b1111_1110__0000_0000 == 0b1011_0100__0000_0000 {return PushRegisters;}
    if inst & 0b1111_1110__0000_0000 == 0b1011_1100__0000_0000 {return PopRegisters;}

    if inst & 0b1111_1000__0000_0000 == 0b1100_0000__0000_0000 {return MultipleStore;}
    if inst & 0b1111_1000__0000_0000 == 0b1100_1000__0000_0000 {return MultipleLoad;}

    if inst & 0b1111_0000__0000_0000 == 0b1101_0000__0000_0000 {return ConditionalBranch;}



    if inst & 0b1111_1000__0000_0000 == 0b1110_0000__0000_0000 {return Branch;}

    if inst & 0b1111_0000__0000_0000 == 0b1111_0000__0000_0000 {return BranchWithLink;}


    return UNREACHABLE;
}