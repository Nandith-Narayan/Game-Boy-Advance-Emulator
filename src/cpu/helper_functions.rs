

pub fn compute_overflow_on_add(op1: i32, op2: i32, result: i32) -> bool{
    // If the sign(op1) == sign(op2) && sign(op1) != sign(result), then set the overflow flag.
    return (op1.is_negative() == op2.is_negative()) && (op1.is_negative() != result.is_negative());
}

pub fn compute_overflow_on_sub(op1: i32, op2: i32, result: i32) -> bool{
    // If the sign(op1) != sign(op2) && sign(op1) != sign(result), then set the overflow flag.
    return (op1.is_negative() != op2.is_negative()) && (op1.is_negative() != result.is_negative());
}