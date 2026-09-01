use quire_contract_codegen::hello;

#[test]
fn hello_is_non_empty() {
    assert!(!hello().is_empty());
}

#[test]
fn hello_is_deterministic() {
    assert_eq!(hello(), hello());
}
