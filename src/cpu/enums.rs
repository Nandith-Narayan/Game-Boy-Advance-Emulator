pub enum InstructionSet {
    ARM,
    THUMB,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CPUMode{
    USER = 0b10000,
    FIQ = 0b10001,
    IRQ = 0b10010,
    SUPERVISOR = 0b10011,
    ABORT = 0b10111,
    UNDEFINED = 0b11011,
    SYSTEM = 0b11111,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ARMInstruction {
    UNREACHABLE,
    CondIsFalse,
    NOP,
    EMPTY,
    DataProcessingReg,
    DataProcessingImmediate,
    TransferToPSR,
    TransferFromPSR,
    BranchAndExchange,
    Branch,
    BranchWithLink,
    Multiply,
    MultiplyLong,
    SingleDataSwap,
    SingleDataTransferReg,
    SingleDataTransferImmediate,
    HalfWordTransferReg,
    HalfWordTransferImmediate,
    BlockDataTransferLoad,
    BlockDataTransferStore,
}

#[derive(Debug)]
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
