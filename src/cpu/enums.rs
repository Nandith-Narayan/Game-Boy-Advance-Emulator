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

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum THUMBInstruction {
    UNREACHABLE,
    NOP,
    EMPTY,
    MoveShiftedRegister,
    AddOrSubtract,
    MoveImmediate,
    CompareImmediate,
    AddImmediate,
    SubtractImmediate,
    ALUOperation,
    BranchAndExchange,
    HighRegisterOperation,
    PCRelativeLoad,
    SingleDataTransfer,
    LoadOrStoreSignExtendedHalfWord,
    SingleDataTransferWithImmediateOffset,
    HalfWordTransferWithImmediateOffset,
    SPRelativeLoadOrStore,
    LoadAddress,
    AddOffsetToSP,
    PushRegisters,
    PopRegisters,
    MultipleStore,
    MultipleLoad,
    ConditionalBranch,

    Branch,
    BranchWithLink,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ShiftType {
    LogicalShiftLeft,
    LogicalShiftRight,
    ArithmeticShiftRight,
    RotateRight,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RegisterName {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    R8FIQ,
    R9FIQ,
    R10FIQ,
    R11FIQ,
    R12FIQ,
    R13FIQ,
    R14FIQ,
    R13SVC,
    R14SVC,
    R13IRQ,
    R14IRQ,
}
