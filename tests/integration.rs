use quire_contract_codegen::hello;

#[test]
fn hello_is_non_empty() {
    assert!(!hello().is_empty());
}
