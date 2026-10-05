//! Identity newtypes of a generated Kani harness (AD-004 `core/identity`, L-10).
//!
//! A harness is named by a module symbol and a harness symbol, and the two are different kinds of
//! name: swapping them at a call site must not compile. Each is a validated Rust identifier,
//! built once where a harness is generated, and [`HarnessPath`] is the pair. They serialize as
//! the bare string, so a persisted identity record is unchanged, and they deserialize from it
//! through the same validation.

use std::fmt;

use serde::{Deserialize, Serialize};

/// A string offered as a harness or module symbol is not a Rust identifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SymbolError {
    /// The rejected text.
    pub symbol: String,
}

impl fmt::Display for SymbolError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "`{}` is not a Rust identifier", self.symbol)
    }
}

impl std::error::Error for SymbolError {}

/// `text` when it is a non-keyword ASCII Rust identifier.
fn validated(text: String) -> Result<String, SymbolError> {
    let mut characters = text.chars();
    let ascii_identifier = characters
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        && characters.all(|next| next.is_ascii_alphanumeric() || next == '_');
    // `syn` refuses keywords and the lone `_`, which the character check above admits.
    if ascii_identifier && syn::parse_str::<syn::Ident>(&text).is_ok() {
        Ok(text)
    } else {
        Err(SymbolError { symbol: text })
    }
}

macro_rules! symbol {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
        #[serde(try_from = "String")]
        pub struct $name(String);

        impl $name {
            /// The symbol as text.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = SymbolError;

            fn try_from(text: String) -> Result<Self, SymbolError> {
                validated(text).map(Self)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = SymbolError;

            fn try_from(text: &str) -> Result<Self, SymbolError> {
                Self::try_from(text.to_owned())
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }
    };
}

symbol! {
    /// The generated module that holds a harness.
    ModuleSymbol
}

symbol! {
    /// The `kani::proof` function of a harness.
    HarnessSymbol
}

/// The `module::harness` path Kani records for a harness.
///
/// The two halves are different types, so a swapped pair does not compile:
///
/// ```compile_fail
/// use quire_contract_codegen::{HarnessPath, HarnessSymbol, ModuleSymbol};
///
/// let module = ModuleSymbol::try_from("m").unwrap();
/// let harness = HarnessSymbol::try_from("check").unwrap();
/// let _ = HarnessPath { module: harness, harness: module };
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HarnessPath {
    /// The module that holds the harness.
    pub module: ModuleSymbol,
    /// The harness function.
    pub harness: HarnessSymbol,
}

impl fmt::Display for HarnessPath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}::{}", self.module, self.harness)
    }
}

#[cfg(test)]
mod tests {
    use super::{HarnessPath, HarnessSymbol, ModuleSymbol};

    /// Trace: AD-004 L-10.
    #[test]
    fn l10_a_symbol_is_a_non_keyword_ascii_identifier() {
        for accepted in ["kob_scalar_add_1_proof", "_private", "check", "post_ab12"] {
            assert_eq!(
                ModuleSymbol::try_from(accepted).map(|symbol| symbol.to_string()),
                Ok(accepted.to_owned()),
                "{accepted} is an identifier"
            );
        }
        for refused in [
            "", "_", "1st", "a-b", "a::b", "fn", "self", "naïve", "a b", "r#fn",
        ] {
            assert!(
                HarnessSymbol::try_from(refused).is_err(),
                "`{refused}` is not an identifier"
            );
        }
    }

    /// Trace: AD-004 L-10.
    #[test]
    fn l10_a_harness_path_displays_module_then_harness_and_serializes_symbols_bare() {
        let path = HarnessPath {
            module: ModuleSymbol::try_from("m").unwrap(),
            harness: HarnessSymbol::try_from("check").unwrap(),
        };
        assert_eq!(path.to_string(), "m::check");
        assert_eq!(serde_json::to_string(&path.module).unwrap(), "\"m\"");
        assert_eq!(serde_json::to_string(&path.harness).unwrap(), "\"check\"");
    }
}
