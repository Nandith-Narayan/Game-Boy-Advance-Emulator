pub enum InstructionSet {
    ARM,
    THUMB,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ARMInstruction {
    UNREACHABLE,
    CondIsFalse,
    NOP,
    DataProcessingReg,
    DataProcessingImmediate,
    BranchAndExchange,
    Branch,
    BranchWithLink,
    Multiply,
    MultiplyLong,
    SingleDataSwap,
    SingleDataTransferReg,
    SingleDataTransferImmediate,
}

pub enum ARMCondition {
    EQ, // z set
    NE, // z clear
    CS, // C set
    CC, // C clear
    MI, // N set
    PL, // N clear
    VS, // V set
    VC, // V clear
    HI, // C set and z clear
    LS, // C clear or z set
    GE, // N equals V
    LT, // N not equal to V
    GT, // z clear and N equals V
    LE, // z set or N not equal to V
    AL, // Always
}
