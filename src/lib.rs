//! Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// Placeholder entry point.
pub fn hello() -> &'static str {
    "hello from quire_contract_codegen"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_returns_greeting() {
        assert!(hello().contains("quire_contract_codegen"));
    }
}
