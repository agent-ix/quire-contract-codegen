//! Generate an epoch during actual library compilation, independent of source/tool digests.
//!
//! Cargo reuses the library's compiled epoch when reusing that actual library artifact. There is
//! no always-rerun build script. A separate helper build must use the consumer's manifest and
//! package/feature selection so it links that same normal library artifact.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use std::{fs::File, io::Read};

/// Expand to a fresh 32-byte artifact epoch on actual library compilation.
#[proc_macro]
pub fn artifact_epoch(input: TokenStream) -> TokenStream {
    if !input.is_empty() {
        return compilation_error("artifact_epoch accepts no caller-supplied identity");
    }
    let mut bytes = [0; 32];
    if File::open("/dev/urandom")
        .and_then(|mut file| file.read_exact(&mut bytes))
        .is_err()
    {
        return compilation_error("cannot acquire guardian library-compilation epoch");
    }
    let mut tokens = TokenStream::new();
    for byte in bytes {
        tokens.extend([
            TokenTree::Literal(Literal::u8_suffixed(byte)),
            TokenTree::Punct(Punct::new(',', Spacing::Alone)),
        ]);
    }
    TokenTree::Group(Group::new(Delimiter::Bracket, tokens)).into()
}

fn compilation_error(message: &str) -> TokenStream {
    [
        TokenTree::Ident(Ident::new("compile_error", Span::call_site())),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenTree::Literal(Literal::string(message)).into(),
        )),
        TokenTree::Punct(Punct::new(';', Spacing::Alone)),
    ]
    .into_iter()
    .collect()
}
