use crate::cpu::enums::CPUMode;
use crate::cpu::enums::CPUMode::*;

pub fn compute_overflow_on_add(op1: i32, op2: i32, result: i32) -> bool{
    // If the sign(op1) == sign(op2) && sign(op1) != sign(result), then set the overflow flag.
    return (op1.is_negative() == op2.is_negative()) && (op1.is_negative() != result.is_negative());
}

pub fn compute_overflow_on_sub(op1: i32, op2: i32, result: i32) -> bool{
    // If the sign(op1) != sign(op2) && sign(op1) != sign(result), then set the overflow flag.
    return (op1.is_negative() != op2.is_negative()) && (op1.is_negative() != result.is_negative());
}

pub fn num_to_cpu_mode(n: u32) -> CPUMode{
    return match n{
        0b10000 => USER,
        0b10001 => FIQ,
        0b10010 => IRQ,
        0b10011 => SUPERVISOR,
        0b10111 => ABORT,
        0b11011 => UNDEFINED,
        0b11111 => SYSTEM,
        _ => UNDEFINED,
    }
}