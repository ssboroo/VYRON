use vyron::check;

#[test]
fn rejects_a_string_returned_from_an_i64_function() {
    let error = check("fn main() -> i64 { return \"wrong\"; }").unwrap_err();
    assert!(error.message.contains("expected i64"));
    assert!(error.message.contains("found string"));
}

#[test]
fn accepts_matching_integer_return_expression() {
    let module = check("fn main() -> i64 { return 42; }").unwrap();
    assert_eq!(module.functions.len(), 1);
}

#[test]
fn accepts_boolean_return_expression() {
    check("fn enabled() -> bool { return true; }").unwrap();
}

#[test]
fn requires_return_value_for_non_unit_function() {
    let error = check("fn main() -> i64 {}").unwrap_err();
    assert!(error.message.contains("must return a value"));
}

#[test]
fn rejects_unknown_return_type() {
    let error = check("fn main() -> mystery { return 1; }").unwrap_err();
    assert!(error.message.contains("unknown return type 'mystery'"));
}

#[test]
fn rejects_return_value_from_untyped_function() {
    let error = check("fn main() { return 1; }").unwrap_err();
    assert!(error.message.contains("does not declare a return type"));
}
