use vyron::{lower_to_ir, Instruction, Type};

#[test]
fn lowers_integer_return_to_typed_ir() {
    let module = lower_to_ir("fn answer() -> i64 { return 42; }").unwrap();
    assert_eq!(module.functions[0].return_type, Type::I64);
    assert_eq!(module.functions[0].instructions, vec![
        Instruction::ConstI64(42),
        Instruction::Return,
    ]);
}

#[test]
fn lowers_boolean_return_to_typed_ir() {
    let module = lower_to_ir("fn enabled() -> bool { return true; }").unwrap();
    assert_eq!(module.functions[0].instructions, vec![
        Instruction::ConstBool(true),
        Instruction::Return,
    ]);
}

#[test]
fn lowers_unit_function_to_unit_return_instruction() {
    let module = lower_to_ir("fn start() {}").unwrap();
    assert_eq!(module.functions[0].instructions, vec![Instruction::ReturnUnit]);
}

#[test]
fn refuses_to_lower_a_type_error() {
    assert!(lower_to_ir("fn wrong() -> i64 { return false; }").is_err());
}
