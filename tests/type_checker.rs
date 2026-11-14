use vyron::{check_return_type, Literal, Type};

#[test]
fn accepts_integer_literal_for_i64_return() {
    assert_eq!(check_return_type(&Type::I64, &Literal::Integer(42)), Ok(Type::I64));
}

#[test]
fn rejects_string_literal_for_integer_return() {
    let error = check_return_type(&Type::I64, &Literal::String("wrong".into())).unwrap_err();
    assert!(error.message.contains("expected i64"));
    assert!(error.message.contains("found string"));
}

#[test]
fn accepts_boolean_literal_for_bool_return() {
    assert_eq!(check_return_type(&Type::Bool, &Literal::Boolean(true)), Ok(Type::Bool));
}

#[test]
fn rejects_integer_overflow_for_i64_literal() {
    let error = check_return_type(&Type::I64, &Literal::IntegerText("9223372036854775808".into())).unwrap_err();
    assert!(error.message.contains("outside the i64 range"));
}

#[test]
fn unit_functions_only_accept_unit_literal() {
    assert!(check_return_type(&Type::Unit, &Literal::Unit).is_ok());
    assert!(check_return_type(&Type::Unit, &Literal::Integer(1)).is_err());
}
