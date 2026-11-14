use vyron::{check, lower_to_ir, Instruction};

#[test]
fn parses_operator_precedence_and_lowers_multiplication_before_addition() {
    let module = lower_to_ir("fn main() -> i64 { return 2 + 3 * 4; }").unwrap();
    assert_eq!(module.functions[0].instructions, vec![
        Instruction::ConstI64(2),
        Instruction::ConstI64(3),
        Instruction::ConstI64(4),
        Instruction::MulI64,
        Instruction::AddI64,
        Instruction::Return,
    ]);
}

#[test]
fn parses_parenthesized_expressions() {
    let module = lower_to_ir("fn main() -> i64 { return (2 + 3) * 4; }").unwrap();
    assert_eq!(module.functions[0].instructions, vec![
        Instruction::ConstI64(2),
        Instruction::ConstI64(3),
        Instruction::AddI64,
        Instruction::ConstI64(4),
        Instruction::MulI64,
        Instruction::Return,
    ]);
}

#[test]
fn supports_unary_negative_integer_literals() {
    let module = lower_to_ir("fn main() -> i64 { return -7; }").unwrap();
    assert_eq!(module.functions[0].instructions, vec![
        Instruction::ConstI64(7),
        Instruction::NegI64,
        Instruction::Return,
    ]);
}

#[test]
fn rejects_mixed_integer_and_boolean_arithmetic() {
    let error = check("fn main() -> i64 { return 1 + true; }").unwrap_err();
    assert!(error.message.contains("operator '+' requires i64 operands"));
}

#[test]
fn rejects_division_by_zero_when_constant() {
    let error = check("fn main() -> i64 { return 9 / (3 - 3); }").unwrap_err();
    assert!(error.message.contains("division by zero"));
}

#[test]
fn rejects_integer_overflow_in_constant_expression() {
    let error = check("fn main() -> i64 { return 9223372036854775807 + 1; }").unwrap_err();
    assert!(error.message.contains("overflows i64"));
}

#[test]
fn rejects_unexpected_trailing_expression_tokens() {
    let error = check("fn main() -> i64 { return 1 2; }").unwrap_err();
    assert!(error.message.contains("expected ';' after return statement"));
}
