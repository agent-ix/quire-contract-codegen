//! FR-017-AC-67 (TC-043 step 17, TC-039 step 26): a structural scan of the non-test, non-comment
//! source of `src/kani/run/` for constructs that tie the launch path to Linux or to a sandbox.
//!
//! The scan parses each file with `syn`, drops `#[cfg(test)]` items and every attribute (so
//! `#[doc]` text goes too; comments are not tokens at all) and searches the identifiers and string
//! literals that remain, including those inside macro arguments, for the one token list below.
//! It does not follow a `#[cfg(test)] mod name;` declaration to its file: the file set is the one
//! reachable from `src/kani/run/mod.rs` through non-test `mod` declarations.

use std::{
    fs,
    path::{Path, PathBuf},
};

use ix_trace_rs::trace;
use proc_macro2::{TokenStream, TokenTree};
use syn::{
    visit::{self, Visit},
    Attribute, Item, ItemMod,
};

/// The one token list (FR-017-AC-67). Matching is by case-insensitive substring.
const FORBIDDEN: [&str; 9] = [
    "bwrap",
    "bubblewrap",
    "unshare",
    "CLONE_NEW",
    "pidfd",
    "procfs",
    "/proc",
    "cgroup",
    "namespace",
];

/// What the scanner looks at; the controls switch one part off to show the part matters.
#[derive(Clone, Copy)]
struct Scope {
    strings: bool,
    test_items: bool,
}

const FULL: Scope = Scope {
    strings: true,
    test_items: false,
};

struct Scanner {
    scope: Scope,
    findings: Vec<String>,
}

impl Scanner {
    fn check(&mut self, text: &str) {
        let lowered = text.to_lowercase();
        for token in FORBIDDEN {
            if lowered.contains(&token.to_lowercase()) {
                self.findings.push(format!("{token} in {text}"));
            }
        }
    }

    fn tokens(&mut self, stream: TokenStream) {
        for tree in stream {
            match tree {
                TokenTree::Ident(ident) => self.check(&ident.to_string()),
                TokenTree::Literal(literal) if self.scope.strings => {
                    self.check(&literal.to_string());
                }
                TokenTree::Group(group) => self.tokens(group.stream()),
                TokenTree::Literal(_) | TokenTree::Punct(_) => {}
            }
        }
    }
}

fn is_cfg_test(attributes: &[Attribute]) -> bool {
    attributes.iter().any(|attribute| {
        attribute.path().is_ident("cfg") && attribute.meta.to_token_stream_string().contains("test")
    })
}

trait MetaText {
    fn to_token_stream_string(&self) -> String;
}

impl MetaText for syn::Meta {
    fn to_token_stream_string(&self) -> String {
        quote::ToTokens::to_token_stream(self).to_string()
    }
}

impl<'ast> Visit<'ast> for Scanner {
    fn visit_item(&mut self, item: &'ast Item) {
        let attributes = match item {
            Item::Mod(item) => &item.attrs,
            Item::Fn(item) => &item.attrs,
            Item::Impl(item) => &item.attrs,
            Item::Const(item) => &item.attrs,
            Item::Static(item) => &item.attrs,
            Item::Struct(item) => &item.attrs,
            Item::Enum(item) => &item.attrs,
            Item::Trait(item) => &item.attrs,
            Item::Use(item) => &item.attrs,
            Item::Type(item) => &item.attrs,
            _ => &Vec::new(),
        };
        if !self.scope.test_items && is_cfg_test(attributes) {
            return;
        }
        visit::visit_item(self, item);
    }

    fn visit_impl_item(&mut self, item: &'ast syn::ImplItem) {
        let attributes = match item {
            syn::ImplItem::Fn(item) => &item.attrs,
            syn::ImplItem::Const(item) => &item.attrs,
            _ => &Vec::new(),
        };
        if !self.scope.test_items && is_cfg_test(attributes) {
            return;
        }
        visit::visit_impl_item(self, item);
    }

    // Attributes (doc text, `trace` arguments, `cfg`) are not source the launch path runs.
    fn visit_attribute(&mut self, _attribute: &'ast Attribute) {}

    fn visit_ident(&mut self, ident: &'ast proc_macro2::Ident) {
        self.check(&ident.to_string());
    }

    fn visit_lit_str(&mut self, literal: &'ast syn::LitStr) {
        if self.scope.strings {
            self.check(&literal.value());
        }
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        self.tokens(mac.tokens.clone());
        visit::visit_macro(self, mac);
    }
}

/// The findings in `source` under `scope`.
fn scan_source(source: &str, scope: Scope) -> Vec<String> {
    let file = syn::parse_file(source).expect("the scanned source parses");
    let mut scanner = Scanner {
        scope,
        findings: Vec::new(),
    };
    scanner.visit_file(&file);
    scanner.findings
}

/// The non-test files reachable from `root` through `mod name;` declarations.
fn reachable_files(root: &Path) -> Vec<PathBuf> {
    let mut files = vec![root.to_path_buf()];
    let mut next = 0;
    while next < files.len() {
        let path = files[next].clone();
        next += 1;
        let source = fs::read_to_string(&path).expect("a reachable file reads");
        let file = syn::parse_file(&source).expect("a reachable file parses");
        let directory = if path.file_name().is_some_and(|name| name == "mod.rs") {
            path.parent().unwrap().to_path_buf()
        } else {
            path.with_extension("")
        };
        let parent = path.parent().unwrap().to_path_buf();
        for item in &file.items {
            let Item::Mod(ItemMod {
                attrs,
                ident,
                content: None,
                ..
            }) = item
            else {
                continue;
            };
            if is_cfg_test(attrs) {
                continue;
            }
            let candidates = [
                directory.join(format!("{ident}.rs")),
                directory.join(ident.to_string()).join("mod.rs"),
                parent.join(format!("{ident}.rs")),
            ];
            let found = candidates
                .into_iter()
                .find(|candidate| candidate.is_file())
                .unwrap_or_else(|| panic!("module {ident} of {} has a file", path.display()));
            if !files.contains(&found) {
                files.push(found);
            }
        }
    }
    files
}

const FIXTURE: &str = r##"
fn code() -> &'static Path { Path::new("/proc") }

// a cgroup in a comment is not a finding

#[cfg(test)]
mod tests {
    fn test_only() -> &'static Path { Path::new("/proc") }
}
"##;

/// The non-test, non-comment source of `src/kani/run/` holds none of the tokens.
#[trace("FR-017-AC-67", "FR-028-AC-21", "TC-043", "TC-039")]
#[test]
fn the_launch_path_source_has_no_linux_only_or_sandbox_construct() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/kani/run/mod.rs");
    let files = reachable_files(&root);
    assert!(
        files.len() >= 8,
        "the scan reaches the launch path files, found {files:?}"
    );
    for name in ["launch.rs", "owners.rs", "memory.rs", "execute.rs"] {
        assert!(
            files.iter().any(|file| file.ends_with(name)),
            "{name} is scanned"
        );
    }
    let mut findings = Vec::new();
    for file in &files {
        let source = fs::read_to_string(file).unwrap();
        for finding in scan_source(&source, FULL) {
            findings.push(format!("{}: {finding}", file.display()));
        }
    }
    assert!(findings.is_empty(), "forbidden constructs: {findings:#?}");
}

/// The scanner finds the code literal and nothing else in the fixture: a scanner that skips
/// string literals finds none, and one that includes test modules or comments finds more.
#[trace("FR-017-AC-67", "TC-043")]
#[test]
fn the_scan_finds_the_code_literal_and_not_the_test_module_or_the_comment() {
    assert_eq!(scan_source(FIXTURE, FULL).len(), 1);
    assert!(scan_source(
        FIXTURE,
        Scope {
            strings: false,
            ..FULL
        }
    )
    .is_empty());
    assert_eq!(
        scan_source(
            FIXTURE,
            Scope {
                test_items: true,
                ..FULL
            }
        )
        .len(),
        2,
        "a scanner that includes test modules finds the test literal too"
    );
    let raw: usize = FORBIDDEN
        .iter()
        .map(|token| {
            FIXTURE
                .to_lowercase()
                .matches(&token.to_lowercase())
                .count()
        })
        .sum();
    assert_eq!(raw, 3, "a text scan that includes comments finds all three");
}
