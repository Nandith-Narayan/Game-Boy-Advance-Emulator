use crate::cpu::enums::THUMBInstruction;
use crate::cpu::enums::THUMBInstruction::{ALUOperation, AddImmediate, AddOrSubtract, BranchAndExchange, CompareImmediate, ConditionalBranch, HighRegisterOperation, MoveImmediate, MoveShiftedRegister, SubtractImmediate, UNREACHABLE};

pub fn decode_instruction(inst: u16) -> THUMBInstruction{

    if inst & 0b1110_0000__0000_0000 == 0b0000_0000__0000_0000 {return MoveShiftedRegister;}
    if inst & 0b1111_1000__0000_0000 == 0b0001_1000__0000_0000 {return AddOrSubtract;}
    if inst & 0b1111_1000__0000_0000 == 0b0010_0000__0000_0000 {return MoveImmediate;}
    if inst & 0b1111_1000__0000_0000 == 0b0010_1000__0000_0000 {return CompareImmediate;}
    if inst & 0b1111_1000__0000_0000 == 0b0011_0000__0000_0000 {return AddImmediate;}
    if inst & 0b1111_1000__0000_0000 == 0b0011_1000__0000_0000 {return SubtractImmediate;}
    if inst & 0b1111_1100__0000_0000 == 0b0100_0000__0000_0000 {return ALUOperation;}
    if inst & 0b1111_1111__0000_0000 == 0b0100_0111__0000_0000 {return BranchAndExchange;}
    if inst & 0b1111_1100__0000_0000 == 0b0100_0100__0000_0000 {return HighRegisterOperation;}



    if inst & 0b1111_0000__0000_0000 == 0b1101_0000__0000_0000 {return ConditionalBranch;}


    return UNREACHABLE;
}