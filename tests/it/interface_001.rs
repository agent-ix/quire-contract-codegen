//! interface-001: the API surface `spec/interface/interface-001-codegen-api.md` declares matches
//! what the crate actually exports and emits. Each test below reads the crate's own declaration
//! rather than trusting prose to stay in sync with code.
//!
//! Every test also parses the contract document's own fenced YAML block and compares the parsed
//! vocabulary against the Rust vocabulary. For `GenerationTerminalState` the Rust side is
//! `GenerationTerminalState::ALL`, the census the enum carries beside its own definition in
//! `src/oracle.rs`. `census_enum_variants` counts the variant identifiers the enum declaration
//! carries and asserts that count equals `ALL.len()`, so `ALL`, `label()` and the declaration
//! cannot drift out of step. `interface-001-AC-1`, `AC-2` and `AC-4` are traced `Test (TC-028)` and
//! backed by this file.

use std::{fs, path::Path};

use quire_contract_codegen::GenerationTerminalState;

fn crate_source(relative_path: &str) -> String {
    fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path))
        .unwrap_or_else(|_| panic!("{relative_path} must be readable from the crate root"))
}

fn lib_source() -> String {
    crate_source("src/lib.rs")
}

fn oracle_source() -> String {
    crate_source("src/oracle.rs")
}

/// Counts the variant identifiers declared between `pub enum <name> {` and its matching closing
/// `}` in `source`. `label()`'s own exhaustive match already fails the build on an added variant,
/// but nothing carries that variant into `ALL` too -- this closes that gap the other direction,
/// against the enum declaration itself rather than against `label()` or `ALL`, so the three
/// cannot drift into agreement with each other while all three disagree with the real variant
/// count.
///
/// Not a general Rust parser: it only counts bare unit variants (`Ident,`), which is the only
/// shape `GenerationTerminalState` uses, and it skips `//`-prefixed and
/// `#`-prefixed lines (comments, doc comments and attributes) entirely -- before looking for
/// either a variant or a brace on that line -- so a brace or comma mentioned in a comment or an
/// attribute cannot desynchronize its `{`/`}` depth tracking or be miscounted as a variant. A
/// future data-carrying variant (`Foo(String),` or `Foo { x: i32 },`) would be silently skipped
/// rather than counted -- correct today, since `ALL: [Self; N]` cannot hold a data-carrying
/// variant either, so both sides would already be wrong the same way -- but is not a shape the
/// enum uses now.
fn census_enum_variants(source: &str, name: &str) -> usize {
    let marker = format!("pub enum {name} {{");
    let start = source
        .find(&marker)
        .unwrap_or_else(|| panic!("`pub enum {name} {{` must appear in src/oracle.rs"));
    let mut depth = 1i32;
    let mut count = 0usize;
    for line in source[start + marker.len()..].lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with('#') {
            continue;
        }
        for ch in trimmed.chars() {
            match ch {
                '{' => depth += 1,
                '}' => depth -= 1,
                _ => {}
            }
        }
        if depth == 0 {
            break;
        }
        let is_unit_variant = trimmed.strip_suffix(',').is_some_and(|ident| {
            !ident.is_empty() && ident.chars().all(|c| c.is_alphanumeric() || c == '_')
        });
        if is_unit_variant {
            count += 1;
        }
    }
    assert!(
        count > 0,
        "census_enum_variants found no variants for {name}; the parser likely desynchronized"
    );
    count
}

fn contract_source() -> String {
    crate_source("spec/interface/interface-001-codegen-api.md")
}

/// The contract document's one fenced ```yaml block, as text.
fn contract_yaml() -> String {
    let source = contract_source();
    let fence_start = source
        .find("```yaml\n")
        .expect("the contract document must carry a fenced yaml block");
    let body_start = fence_start + "```yaml\n".len();
    let rest = &source[body_start..];
    let body_end = rest
        .find("\n```")
        .expect("the contract document's yaml block must be closed");
    rest[..body_end].to_owned()
}

/// Parses every `operations` entry into `(name, declared_planned)`, reading each `- name:` line
/// and the `status:` line that follows it (if any) up to the next `- name:` or the next
/// unindented top-level key. `declared_planned` is true only when a `status:` value starts with
/// `planned`, matching interface-001-AC-1's and AC-2's own wording.
fn parse_operations(yaml: &str) -> Vec<(String, bool)> {
    let mut operations = Vec::new();
    let mut in_operations = false;
    let mut current: Option<(String, bool)> = None;
    for line in yaml.lines() {
        if line == "operations:" {
            in_operations = true;
            continue;
        }
        if !in_operations {
            continue;
        }
        if !line.is_empty() && !line.starts_with(' ') {
            break;
        }
        let trimmed = line.trim_start();
        if let Some(name) = trimmed.strip_prefix("- name: ") {
            if let Some(entry) = current.take() {
                operations.push(entry);
            }
            current = Some((name.trim().to_owned(), false));
        } else if let Some(status) = trimmed.strip_prefix("status: ") {
            if let Some((name, _)) = current.take() {
                current = Some((name, status.trim_start().starts_with("planned")));
            }
        }
    }
    if let Some(entry) = current.take() {
        operations.push(entry);
    }
    operations
}

/// Parses a `key: [a, b, c]` flow-sequence line into its comma-separated items. `marker` must
/// include the `: [` so a bare `key:` block-list line elsewhere in the document cannot match.
fn parse_flow_list(yaml: &str, marker: &str) -> Vec<String> {
    let line = yaml
        .lines()
        .find(|line| line.trim_start().contains(marker))
        .unwrap_or_else(|| panic!("contract yaml must declare a `{marker}...]` flow list"));
    let start = line
        .find('[')
        .expect("marker guarantees an opening bracket");
    let end = line[start..]
        .find(']')
        .map(|offset| start + offset)
        .unwrap_or_else(|| panic!("`{marker}` flow list must be closed on one line"));
    line[start + 1..end]
        .split(',')
        .map(|item| item.trim().to_owned())
        .filter(|item| !item.is_empty())
        .collect()
}

fn sorted(mut items: Vec<String>) -> Vec<String> {
    items.sort();
    items
}

/// One public function the crate exposes, found by walking its own source with `syn`.
struct PublicFunction {
    /// The path a consumer names it by: a bare name at the crate root, or `module::name`.
    path: String,
    /// Whether it is a `pub use` re-export rather than a `pub fn` definition.
    reexport: bool,
}

/// Walks one module's items: every `pub fn` it defines, every lower-snake-case leaf its `pub use`
/// items re-export (a renamed leaf under its new name), and, recursively, every `pub mod` it
/// declares, whether inline or in its own file. The crate's types are UpperCamelCase and its
/// constants SCREAMING_SNAKE_CASE, so a lowercase re-exported leaf is a function or a module; a
/// module re-export would be listed too, and fail the census rather than pass it.
fn walk_module(items: &[syn::Item], prefix: &str, dir: &Path, out: &mut Vec<PublicFunction>) {
    fn leaves(tree: &syn::UseTree, out: &mut Vec<String>) {
        match tree {
            syn::UseTree::Path(path) => leaves(&path.tree, out),
            syn::UseTree::Name(name) => out.push(name.ident.to_string()),
            syn::UseTree::Rename(rename) => out.push(rename.rename.to_string()),
            syn::UseTree::Group(group) => group.items.iter().for_each(|item| leaves(item, out)),
            syn::UseTree::Glob(_) => {
                panic!("a public glob re-export makes the crate's surface unenumerable")
            }
        }
    }
    let public = |vis: &syn::Visibility| matches!(vis, syn::Visibility::Public(_));
    for item in items {
        match item {
            syn::Item::Fn(function) if public(&function.vis) => out.push(PublicFunction {
                path: format!("{prefix}{}", function.sig.ident),
                reexport: false,
            }),
            syn::Item::Use(item) if public(&item.vis) => {
                let mut names = Vec::new();
                leaves(&item.tree, &mut names);
                out.extend(
                    names
                        .into_iter()
                        .filter(|name| name.starts_with(|c: char| c.is_ascii_lowercase()))
                        .map(|name| PublicFunction {
                            path: format!("{prefix}{name}"),
                            reexport: true,
                        }),
                );
            }
            syn::Item::Mod(module) if public(&module.vis) => {
                let name = module.ident.to_string();
                let child_prefix = format!("{prefix}{name}::");
                let child_dir = dir.join(&name);
                if let Some((_, inline)) = &module.content {
                    walk_module(inline, &child_prefix, &child_dir, out);
                } else {
                    let file = [dir.join(format!("{name}.rs")), child_dir.join("mod.rs")]
                        .into_iter()
                        .find(|candidate| candidate.is_file())
                        .unwrap_or_else(|| panic!("`pub mod {name}` must have a source file"));
                    let source = fs::read_to_string(&file).expect("module source is readable");
                    let parsed = syn::parse_file(&source).expect("module source parses");
                    walk_module(&parsed.items, &child_prefix, &child_dir, out);
                }
            }
            _ => {}
        }
    }
}

/// Every public function the crate exposes, read from its own source rather than retyped: the
/// crate root's `pub fn`s and `pub use` leaves, and everything reachable through a `pub mod`.
/// A function is counted once, under its shortest public path, so a submodule re-export of a
/// name the crate root also exports is the root's entry, not a second one.
fn exported_functions() -> Vec<String> {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let root = syn::parse_file(&lib_source()).expect("src/lib.rs must parse");
    let mut found = Vec::new();
    walk_module(&root.items, "", &src, &mut found);
    let at_root = found
        .iter()
        .filter(|function| !function.path.contains("::"))
        .map(|function| function.path.clone())
        .collect::<Vec<_>>();
    sorted(
        found
            .into_iter()
            .filter(|function| {
                let leaf = function.path.rsplit("::").next().unwrap_or_default();
                !(function.reexport
                    && function.path.contains("::")
                    && at_root.iter().any(|name| name == leaf))
            })
            .map(|function| function.path)
            .collect(),
    )
}

/// The crate's public function set, read from its own source, equals the contract's non-planned
/// `operations` entries in both directions: a new public function no entry declares fails,
/// whether a root `pub fn`, a root re-export or a function in a `pub mod`, and so does a declared
/// entry the crate stops exposing. Neither side is a list typed into this file.
///
/// The `## Features` table is the interface's feature order (the interface archetype requires
/// it) and restates the Contract yaml's operations, so each operation must have exactly one row
/// and no row may name anything else.
///
/// Trace: interface-001-AC-1, TC-028
#[test]
fn it_001_features_table_lists_exactly_the_contract_operations() {
    let declared = sorted(
        parse_operations(&contract_yaml())
            .into_iter()
            .map(|(name, _)| name)
            .collect(),
    );
    let source = contract_source();
    let section = source
        .split("\n## Features\n")
        .nth(1)
        .expect("the contract document must carry a `## Features` section")
        .split("\n## ")
        .next()
        .expect("split always yields a first part");
    let rows = sorted(
        section
            .lines()
            .filter(|line| line.starts_with('|'))
            .skip(2)
            .map(|line| {
                let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
                assert_eq!(
                    cells.len(),
                    2,
                    "a Features row has Feature and Kind: {line}"
                );
                assert_eq!(cells[1], "operation", "unexpected feature kind: {line}");
                cells[0].to_owned()
            })
            .collect(),
    );
    assert_eq!(
        rows, declared,
        "the Features table must list each Contract operation exactly once and nothing else"
    );
}

/// Trace: interface-001-AC-1, TC-028
#[test]
fn it_001_implemented_operations_are_exported_under_their_declared_names() {
    let declared_implemented = sorted(
        parse_operations(&contract_yaml())
            .into_iter()
            .filter_map(|(name, planned)| (!planned).then_some(name))
            .collect(),
    );
    let exported = exported_functions();
    let undeclared = exported
        .iter()
        .filter(|name| !declared_implemented.contains(name))
        .collect::<Vec<_>>();
    let unexported = declared_implemented
        .iter()
        .filter(|name| !exported.contains(name))
        .collect::<Vec<_>>();
    assert!(
        undeclared.is_empty() && unexported.is_empty(),
        "exported but undeclared: {undeclared:?}; declared but not exported: {unexported:?}"
    );
}

/// Every `operations` entry interface-001 marks `status: planned` is absent from the crate's
/// public functions, so an implementation cannot silently outrun the status the contract declares
/// for it. The planned set is parsed from the contract, not typed into this file.
///
/// Trace: interface-001-AC-2, TC-028
#[test]
fn it_001_planned_operations_are_not_exported() {
    let exported = exported_functions();
    let declared_planned = parse_operations(&contract_yaml())
        .into_iter()
        .filter_map(|(name, is_planned)| is_planned.then_some(name))
        .collect::<Vec<_>>();
    assert!(
        !declared_planned.is_empty(),
        "the contract declares planned operations"
    );
    for name in &declared_planned {
        assert!(
            !exported.contains(name),
            "the crate must not export {name}: interface-001 declares it status: planned"
        );
    }
}

/// `diagnostics.terminal_states` names exactly the six `GenerationTerminalState` variants, as the
/// contract's serde wire form -- what a `GenerationDiagnostic` actually emits -- not
/// `GenerationTerminalState::label`'s independent hand-maintained string, which a
/// `#[serde(rename = ...)]` could drift away from silently. The Rust side reads
/// `GenerationTerminalState::ALL`, the census declared beside the enum in `src/oracle.rs`, rather
/// than a second hand-typed list local to this file; the expected list is parsed from the contract
/// document itself, not hand-copied.
///
/// Trace: interface-001-AC-4, TC-028
#[test]
fn it_001_terminal_states_are_exactly_the_declared_six() {
    let labels = GenerationTerminalState::ALL
        .into_iter()
        .map(|state| {
            let wire_form = serde_json::to_value(state)
                .expect("GenerationTerminalState must serialize")
                .as_str()
                .expect("GenerationTerminalState serializes as a bare string")
                .to_owned();
            // label() has no other caller once this file compares the wire form instead, so this
            // keeps it from silently drifting unnoticed.
            assert_eq!(
                state.label(),
                wire_form,
                "GenerationTerminalState::label() must agree with its own #[serde(rename_all)] output"
            );
            wire_form
        })
        .collect::<Vec<_>>();
    let terminal_states = parse_flow_list(&contract_yaml(), "terminal_states: [");
    assert_eq!(sorted(labels), sorted(terminal_states));
    assert_eq!(
        census_enum_variants(&oracle_source(), "GenerationTerminalState"),
        GenerationTerminalState::ALL.len(),
        "GenerationTerminalState::ALL must name every variant src/oracle.rs declares"
    );
}
