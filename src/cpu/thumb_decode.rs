use crate::cpu::enums::THUMBInstruction;
use crate::cpu::enums::THUMBInstruction::{AddImmediate, AddOrSubtract, CompareImmediate, ConditionalBranch, MoveImmediate, MoveShiftedRegister, SubtractImmediate, UNREACHABLE};

pub fn decode_instruction(inst: u16) -> THUMBInstruction{

    if inst & 0b1110_0000__0000_0000 == 0b0000_0000__0000_0000 {return MoveShiftedRegister;}
    if inst & 0b1111_1000__0000_0000 == 0b0001_1000__0000_0000 {return AddOrSubtract;}
    if inst & 0b1111_1000__0000_0000 == 0b0010_0000__0000_0000 {return MoveImmediate;}
    if inst & 0b1111_1000__0000_0000 == 0b0010_1000__0000_0000 {return CompareImmediate;}
    if inst & 0b1111_1000__0000_0000 == 0b0011_0000__0000_0000 {return AddImmediate;}
    if inst & 0b1111_1000__0000_0000 == 0b0011_1000__0000_0000 {return SubtractImmediate;}



    if inst & 0b1111_0000__0000_0000 == 0b1101_0000__0000_0000 {return ConditionalBranch;}


    return UNREACHABLE;
}