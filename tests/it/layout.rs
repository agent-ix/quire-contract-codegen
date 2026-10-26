//! AD-004 layout decisions L-1 and L-2 and FR-024-AC-10, checked over `src/`.
//!
//! L-1: `src/` holds the eight subsystem directories and `lib.rs`, and every module the AD's
//! module-to-subsystem map names sits in the directory the map names.
//!
//! L-2: no file imports an item through the crate root, an import names a directory the
//! importing directory may reach, the module graph has no cycle (`#[cfg(test)]` modules
//! included, because the scan reads whole files), and the order inside `kani/` holds.
//!
//! The scan reads `crate::` paths in code. Comments and string and character literals are
//! blanked first, so doc prose and generated-source templates that spell `crate::` are not
//! imports. A `super::` path, a path built in a macro and an intra-doc link escape it, as the
//! AD's Risks section records.

use ix_trace_rs::trace;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
use syn::{spanned::Spanned, visit::Visit, Item, UseTree};

const QSL_OWNED_TYPES: [&str; 5] = [
    "Witness",
    "ReplaySource",
    "WitnessEnvelope",
    "TerminalRecord",
    "ObligationIdentity",
];

/// Parses every source file and locates `Witness::parse` paths, including qualified paths and
/// function values. The scan sees normal Rust syntax, including nested modules; paths assembled
/// by macros or through aliases are outside it.
#[trace("TC-035", "FR-024-AC-2")]
#[test]
fn witness_parse_calls_stay_in_the_one_adapter_renderer() {
    struct ParseScan<'a> {
        file: &'a str,
        function: Option<String>,
        calls: &'a mut Vec<(String, String)>,
    }
    impl Visit<'_> for ParseScan<'_> {
        fn visit_item_fn(&mut self, item: &syn::ItemFn) {
            let previous = self.function.replace(item.sig.ident.to_string());
            syn::visit::visit_item_fn(self, item);
            self.function = previous;
        }
        fn visit_expr_path(&mut self, path: &syn::ExprPath) {
            let mut segments = path.path.segments.iter().rev();
            if segments.next().is_some_and(|part| part.ident == "parse")
                && segments.next().is_some_and(|part| part.ident == "Witness")
            {
                self.calls.push((
                    self.file.to_owned(),
                    self.function.clone().unwrap_or_default(),
                ));
            }
            syn::visit::visit_expr_path(self, path);
        }
    }
    let mut calls = Vec::new();
    for (file, source) in source_files() {
        let parsed = syn::parse_file(&source).unwrap_or_else(|error| panic!("{file}: {error}"));
        ParseScan {
            file: &file,
            function: None,
            calls: &mut calls,
        }
        .visit_file(&parsed);
    }
    assert_eq!(
        calls,
        vec![("replay/function.rs".to_owned(), "render_witness".to_owned())]
    );
}

type AliasMap = BTreeMap<Vec<String>, Vec<String>>;

fn use_paths(
    tree: &UseTree,
    prefix: &mut Vec<String>,
    found: &mut Vec<(Vec<String>, String, bool)>,
) {
    match tree {
        UseTree::Path(item) => {
            prefix.push(item.ident.to_string());
            use_paths(&item.tree, prefix, found);
            prefix.pop();
        }
        UseTree::Name(item) => {
            let mut target = prefix.clone();
            if item.ident != "self" {
                target.push(item.ident.to_string());
            }
            let binding = if item.ident == "self" {
                prefix.last().cloned().unwrap_or_default()
            } else {
                item.ident.to_string()
            };
            found.push((target, binding, false));
        }
        UseTree::Rename(item) => {
            let mut target = prefix.clone();
            if item.ident != "self" {
                target.push(item.ident.to_string());
            }
            found.push((target, item.rename.to_string(), false));
        }
        UseTree::Glob(_) => found.push((prefix.clone(), String::new(), true)),
        UseTree::Group(item) => {
            for tree in &item.items {
                use_paths(tree, prefix, found);
            }
        }
    }
}

fn resolve_path(
    path: &[String],
    module: &[String],
    aliases: &AliasMap,
    depth: usize,
) -> Vec<String> {
    if depth > 32 || path.is_empty() {
        return path.to_vec();
    }
    let mut relative = path.to_vec();
    let mut base = module.to_vec();
    if relative.first().is_some_and(|part| part == "crate") {
        relative.remove(0);
        base.clear();
    } else {
        while relative
            .first()
            .is_some_and(|part| part == "self" || part == "super")
        {
            if relative.remove(0) == "super" {
                base.pop();
            }
        }
    }
    if relative
        .first()
        .is_some_and(|part| part == "quire_contract_ir")
    {
        return relative;
    }
    for count in (0..=base.len()).rev() {
        let mut candidate = base[..count].to_vec();
        candidate.extend(relative.iter().cloned());
        for end in (1..=candidate.len()).rev() {
            if let Some(target) = aliases.get(&candidate[..end]) {
                let mut expanded = resolve_path(target, &candidate[..end - 1], aliases, depth + 1);
                expanded.extend_from_slice(&candidate[end..]);
                return expanded;
            }
        }
    }
    let mut result = base;
    result.extend(relative);
    result
}

fn banned_name(path: &[String]) -> Option<&str> {
    if path.first().is_some_and(|part| part == "quire_contract_ir") {
        path.iter()
            .skip(1)
            .map(String::as_str)
            .find(|name| QSL_OWNED_TYPES.contains(name))
    } else {
        None
    }
}

fn macro_definitions(
    tokens: proc_macro2::TokenStream,
    names: &mut Vec<(String, proc_macro2::Span)>,
) {
    let mut previous = None;
    for token in tokens {
        match token {
            proc_macro2::TokenTree::Ident(ident) => {
                if previous.as_deref().is_some_and(|word| {
                    ["struct", "enum", "union", "trait", "type"].contains(&word)
                }) && QSL_OWNED_TYPES.contains(&ident.to_string().as_str())
                {
                    names.push((ident.to_string(), ident.span()));
                }
                previous = Some(ident.to_string());
            }
            proc_macro2::TokenTree::Group(group) => {
                macro_definitions(group.stream(), names);
                previous = None;
            }
            _ => previous = None,
        }
    }
}

/// Scans every parsed Rust file under `src/` for the five QSL-owned declarations,
/// Contract IR imports and qualified paths, including alias chains across files.
/// Macro tokens are checked for literal declaration keywords followed by a banned name;
/// imports and qualified paths inside opaque macro tokens, macro expansion,
/// generated files outside `src/`, and constructed identifiers are outside this scan.
#[trace("TC-035", "FR-024-AC-10")]
#[test]
fn qsl_replay_types_are_neither_defined_nor_imported_from_contract_ir() {
    struct AliasScan<'a> {
        module: Vec<String>,
        aliases: &'a mut AliasMap,
    }
    impl Visit<'_> for AliasScan<'_> {
        fn visit_item_mod(&mut self, item: &syn::ItemMod) {
            if let Some((_, content)) = &item.content {
                self.module.push(item.ident.to_string());
                for item in content {
                    self.visit_item(item);
                }
                self.module.pop();
            }
        }
        fn visit_item_use(&mut self, item: &syn::ItemUse) {
            let mut paths = Vec::new();
            use_paths(&item.tree, &mut Vec::new(), &mut paths);
            for (target, binding, glob) in paths {
                if !glob && !binding.is_empty() {
                    let mut key = self.module.clone();
                    key.push(binding);
                    self.aliases.insert(key, target);
                }
            }
        }
        fn visit_item_extern_crate(&mut self, item: &syn::ItemExternCrate) {
            if item.ident == "quire_contract_ir" {
                let mut key = self.module.clone();
                key.push(
                    item.rename
                        .as_ref()
                        .map_or_else(|| item.ident.to_string(), |(_, name)| name.to_string()),
                );
                self.aliases
                    .insert(key, vec!["quire_contract_ir".to_owned()]);
            }
        }
    }

    struct BoundaryScan<'a> {
        violations: &'a mut Vec<String>,
        file: &'a str,
        module: Vec<String>,
        aliases: &'a AliasMap,
    }
    impl BoundaryScan<'_> {
        fn report(&mut self, name: &str, span: proc_macro2::Span, action: &str) {
            self.violations.push(format!(
                "{}:{}: {action} {name}",
                self.file,
                span.start().line
            ));
        }
        fn check(&mut self, path: Vec<String>, span: proc_macro2::Span) {
            let resolved = resolve_path(&path, &self.module, self.aliases, 0);
            if let Some(name) = banned_name(&resolved) {
                self.report(name, span, "uses Contract IR type");
            }
        }
    }
    impl Visit<'_> for BoundaryScan<'_> {
        fn visit_item_mod(&mut self, item: &syn::ItemMod) {
            if let Some((_, content)) = &item.content {
                self.module.push(item.ident.to_string());
                for item in content {
                    self.visit_item(item);
                }
                self.module.pop();
            }
        }
        fn visit_item(&mut self, item: &Item) {
            let name = match item {
                Item::Struct(item) => Some(&item.ident),
                Item::Enum(item) => Some(&item.ident),
                Item::Union(item) => Some(&item.ident),
                Item::Trait(item) => Some(&item.ident),
                Item::Type(item) => Some(&item.ident),
                _ => None,
            };
            if let Some(name) =
                name.filter(|name| QSL_OWNED_TYPES.contains(&name.to_string().as_str()))
            {
                self.report(&name.to_string(), name.span(), "defines");
            }
            if let Item::Use(item) = item {
                let mut paths = Vec::new();
                use_paths(&item.tree, &mut Vec::new(), &mut paths);
                for (path, _, glob) in paths {
                    if glob {
                        let resolved = resolve_path(&path, &self.module, self.aliases, 0);
                        if resolved
                            .first()
                            .is_some_and(|part| part == "quire_contract_ir")
                        {
                            for name in QSL_OWNED_TYPES {
                                self.report(
                                    name,
                                    item.use_token.span,
                                    "may import Contract IR type via glob",
                                );
                            }
                        }
                    } else {
                        self.check(path, item.use_token.span);
                    }
                }
            }
            syn::visit::visit_item(self, item);
        }
        fn visit_path(&mut self, path: &syn::Path) {
            self.check(
                path.segments
                    .iter()
                    .map(|segment| segment.ident.to_string())
                    .collect(),
                path.segments
                    .first()
                    .map_or(path.span(), |segment| segment.ident.span()),
            );
            syn::visit::visit_path(self, path);
        }
        fn visit_macro(&mut self, item: &syn::Macro) {
            let mut names = Vec::new();
            macro_definitions(item.tokens.clone(), &mut names);
            for (name, span) in names {
                self.report(&name, span, "macro tokens define");
            }
            syn::visit::visit_macro(self, item);
        }
    }

    let files: Vec<_> = source_files()
        .into_iter()
        .map(|(file, source)| {
            let parsed = syn::parse_file(&source).unwrap_or_else(|error| panic!("{file}: {error}"));
            (file, parsed)
        })
        .collect();
    let mut aliases = AliasMap::new();
    for (file, parsed) in &files {
        let mut scan = AliasScan {
            module: module_of(file),
            aliases: &mut aliases,
        };
        scan.visit_file(parsed);
    }
    let mut violations = Vec::new();
    for (file, parsed) in &files {
        let mut scan = BoundaryScan {
            violations: &mut violations,
            file,
            module: module_of(file),
            aliases: &aliases,
        };
        scan.visit_file(parsed);
    }
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

/// The directories of `src/`, one per subsystem.
const DIRECTORIES: [&str; 8] = [
    "core",
    "evidence",
    "kani",
    "oracle",
    "publication",
    "replay",
    "routed",
    "strategy",
];

/// Every module file of the AD's module-to-subsystem map that exists in this layout. The
/// one-generator spec and renderer, and the routed adapter, are created by later steps.
const MODULE_FILES: &[&str] = &[
    "core/artifact.rs",
    "core/diagnostic.rs",
    "core/identity.rs",
    "core/naming.rs",
    "core/profile.rs",
    "core/source_map.rs",
    "core/test_support.rs",
    "oracle/boolean_v1.rs",
    "oracle/bound_v1.rs",
    "oracle/claim.rs",
    "oracle/equality/mod.rs",
    "oracle/function/mod.rs",
    "oracle/scalar/mod.rs",
    "oracle/scalar/derive.rs",
    "oracle/scalar/lower.rs",
    "oracle/scalar/render.rs",
    "strategy/campaign.rs",
    "strategy/harness.rs",
    "strategy/bound/mod.rs",
    "evidence/bound_coverage.rs",
    "evidence/vacuity.rs",
    "kani/abi.rs",
    "kani/census.rs",
    "kani/classify.rs",
    "kani/identity.rs",
    "kani/terminal.rs",
    "kani/test_support.rs",
    "kani/generate/census_validation.rs",
    "kani/generate/clause.rs",
    "kani/generate/collection.rs",
    "kani/generate/contract.rs",
    "kani/generate/corpus/bounded_kani_corpus.rs",
    "kani/generate/frame.rs",
    "kani/generate/lower/bounded_collections.rs",
    "kani/generate/lower/bounded_kani_profile.rs",
    "kani/generate/lower/finite_reference_graphs.rs",
    "kani/generate/negotiate.rs",
    "kani/generate/outcome.rs",
    "kani/generate/precondition.rs",
    "kani/generate/record.rs",
    "kani/generate/scalar.rs",
    "kani/generate/v1_bundle.rs",
    "kani/output/diagnostics.rs",
    "kani/output/playback.rs",
    "kani/output/report.rs",
    "kani/run/execute.rs",
    "kani/run/harness.rs",
    "kani/run/launch.rs",
    "kani/run/report_file.rs",
    "kani/run/tool.rs",
    "replay/frame.rs",
    "replay/function.rs",
    "replay/state_clause.rs",
    "replay/witness.rs",
    "routed/capability.rs",
    "routed/generate.rs",
    "publication/publish.rs",
    // The `mod.rs` of each directory of the tree.
    "core/mod.rs",
    "oracle/mod.rs",
    "strategy/mod.rs",
    "evidence/mod.rs",
    "kani/mod.rs",
    "kani/generate/mod.rs",
    "kani/generate/corpus/mod.rs",
    "kani/generate/lower/mod.rs",
    "kani/output/mod.rs",
    "kani/run/mod.rs",
    "replay/mod.rs",
    "routed/mod.rs",
    "publication/mod.rs",
    // The files of `strategy/bound/` (the five `bound_strategy` files).
    "strategy/bound/census.rs",
    "strategy/bound/generation.rs",
    "strategy/bound/population.rs",
    "strategy/bound/relation.rs",
];

/// The directories a directory may import, besides itself: AD-004's dependency direction.
fn may_import(directory: &str) -> &'static [&'static str] {
    match directory {
        "core" => &[],
        "oracle" | "publication" => &["core"],
        "strategy" | "evidence" | "kani" => &["oracle", "core"],
        "replay" => &["kani", "core"],
        "routed" => &["replay", "kani", "strategy", "oracle", "core"],
        other => panic!("`{other}` is not a directory of the layout"),
    }
}

/// The order of the top-level modules of `kani/`: a file imports only earlier names. `generate`
/// and `output` share a rank, so neither imports the other.
fn kani_rank(name: &str) -> Option<u8> {
    match name {
        "abi" => Some(0),
        "census" => Some(1),
        "identity" => Some(2),
        "generate" | "output" => Some(3),
        "classify" => Some(4),
        "run" => Some(5),
        "terminal" => Some(6),
        _ => None,
    }
}

fn source_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// Every `.rs` file under `src/`: its path relative to `src/` with `/` separators, and its text.
pub(crate) fn source_files() -> BTreeMap<String, String> {
    fn walk(root: &Path, directory: &Path, files: &mut BTreeMap<String, String>) {
        for entry in fs::read_dir(directory).expect("a readable source directory") {
            let path = entry.expect("a readable directory entry").path();
            if path.is_dir() {
                walk(root, &path, files);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                let relative = path
                    .strip_prefix(root)
                    .expect("a path under src")
                    .components()
                    .map(|part| part.as_os_str().to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
                    .join("/");
                let text = fs::read_to_string(&path).expect("a readable source file");
                files.insert(relative, text);
            }
        }
    }
    let root = source_root();
    let mut files = BTreeMap::new();
    walk(&root, &root, &mut files);
    files
}

/// The module path of a file: `kani/generate/mod.rs` is `kani::generate`, `lib.rs` is empty.
fn module_of(file: &str) -> Vec<String> {
    let stem = file.strip_suffix(".rs").expect("a Rust file");
    let stem = stem.strip_suffix("/mod").unwrap_or(stem);
    if stem == "lib" {
        return Vec::new();
    }
    stem.split('/').map(str::to_owned).collect()
}

fn is_identifier_char(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

/// The source with comments and string and character literals replaced by spaces.
fn code_only(source: &str) -> String {
    let text: Vec<char> = source.chars().collect();
    let mut out = String::with_capacity(source.len());
    let mut index = 0;
    while index < text.len() {
        index = match blank_extent(&text, index) {
            Some(end) => {
                out.push(' ');
                end
            }
            None => {
                out.push(text[index]);
                index + 1
            }
        };
    }
    out
}

/// The end of the comment or literal that starts at `index`, or `None` if code starts there.
fn blank_extent(text: &[char], index: usize) -> Option<usize> {
    let at = |offset: usize| text.get(index + offset).copied();
    match text[index] {
        '/' if at(1) == Some('/') => Some(line_end(text, index)),
        '/' if at(1) == Some('*') => Some(block_comment_end(text, index)),
        '"' => Some(string_end(text, index + 1)),
        'r' if starts_raw_string(text, index) => Some(raw_string_end(text, index)),
        '\'' => char_literal_end(text, index),
        _ => None,
    }
}

/// The string literals of `source`, as written between their delimiters' extent, outside every
/// `#[cfg(test)]` item. It reads with the same lexer as [`code_only`], so a literal inside a
/// comment or a character literal is not one, and a `#[cfg(test)]` spelled in a literal or a
/// comment gates nothing. The gated item ends at its first `;` or at the `}` closing its first
/// `{`, which holds for the `mod tests` blocks and test functions of `src/`.
pub(crate) fn non_test_string_literals(source: &str) -> Vec<String> {
    const GATE: &str = "#[cfg(test)]";
    let text: Vec<char> = source.chars().collect();
    let gate: Vec<char> = GATE.chars().collect();
    let mut literals = Vec::new();
    let mut index = 0;
    while index < text.len() {
        if text.get(index..index + gate.len()) == Some(&gate[..]) {
            index = gated_item_end(&text, index + gate.len());
            continue;
        }
        match blank_extent(&text, index) {
            Some(end) => {
                if text[index] == '"' || text[index] == 'r' {
                    literals.push(text[index..end.min(text.len())].iter().collect());
                }
                index = end;
            }
            None => index += 1,
        }
    }
    literals
}

/// The index after the item that starts at `index`: past the first `;` outside every bracket
/// (an array type or length such as `[u8; 2]` holds one), or past the `}` that closes the first
/// `{` outside brackets, skipping comments and literals.
fn gated_item_end(text: &[char], mut index: usize) -> usize {
    let mut depth = 0_usize;
    while index < text.len() {
        if let Some(end) = blank_extent(text, index) {
            index = end;
            continue;
        }
        match text[index] {
            ';' if depth == 0 => return index + 1,
            '{' | '[' | '(' => depth += 1,
            '}' | ']' | ')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 && text[index] == '}' {
                    return index + 1;
                }
            }
            _ => {}
        }
        index += 1;
    }
    index
}

fn line_end(text: &[char], mut index: usize) -> usize {
    while index < text.len() && text[index] != '\n' {
        index += 1;
    }
    index
}

fn block_comment_end(text: &[char], mut index: usize) -> usize {
    let mut depth = 0_usize;
    while index < text.len() {
        match (text[index], text.get(index + 1).copied()) {
            ('/', Some('*')) => {
                depth += 1;
                index += 2;
            }
            ('*', Some('/')) => {
                depth -= 1;
                index += 2;
                if depth == 0 {
                    return index;
                }
            }
            _ => index += 1,
        }
    }
    index
}

/// The index after the closing quote of the string whose body starts at `index`.
fn string_end(text: &[char], mut index: usize) -> usize {
    while index < text.len() {
        match text[index] {
            '\\' => index += 2,
            '"' => return index + 1,
            _ => index += 1,
        }
    }
    index
}

fn starts_raw_string(text: &[char], index: usize) -> bool {
    if index > 0 && is_identifier_char(text[index - 1]) && text[index - 1] != 'b' {
        return false;
    }
    let mut cursor = index + 1;
    while text.get(cursor) == Some(&'#') {
        cursor += 1;
    }
    text.get(cursor) == Some(&'"')
}

fn raw_string_end(text: &[char], index: usize) -> usize {
    let mut cursor = index + 1;
    let mut hashes = 0;
    while text[cursor] == '#' {
        hashes += 1;
        cursor += 1;
    }
    cursor += 1;
    while cursor < text.len() {
        if text[cursor] == '"' && (1..=hashes).all(|offset| text.get(cursor + offset) == Some(&'#'))
        {
            return cursor + hashes + 1;
        }
        cursor += 1;
    }
    cursor
}

/// The end of a character literal, or `None` for a lifetime.
fn char_literal_end(text: &[char], index: usize) -> Option<usize> {
    match (text.get(index + 1), text.get(index + 2)) {
        (Some('\\'), _) => {
            let mut cursor = index + 2;
            while cursor < text.len() && text[cursor] != '\'' {
                cursor += 1;
            }
            Some(cursor + 1)
        }
        (Some(_), Some('\'')) => Some(index + 3),
        _ => None,
    }
}

/// Every `crate::` path in the code, each expanded to its full segments after `crate`.
fn crate_paths(code: &str) -> Vec<Vec<String>> {
    let text: Vec<char> = code.chars().collect();
    let mut paths = Vec::new();
    let marker: Vec<char> = "crate::".chars().collect();
    let mut index = 0;
    while index + marker.len() <= text.len() {
        let starts_here = text[index..index + marker.len()] == marker[..]
            && (index == 0 || !(is_identifier_char(text[index - 1]) || text[index - 1] == '$'));
        if starts_here {
            let mut cursor = index + marker.len();
            expand_tree(&text, &mut cursor, &[], &mut paths);
            index += marker.len();
        } else {
            index += 1;
        }
    }
    paths
}

fn skip_whitespace(text: &[char], cursor: &mut usize) {
    while text
        .get(*cursor)
        .is_some_and(|character| character.is_whitespace())
    {
        *cursor += 1;
    }
}

/// Parses one use-tree or path from `cursor`, pushing every full path it names onto `out`.
fn expand_tree(text: &[char], cursor: &mut usize, prefix: &[String], out: &mut Vec<Vec<String>>) {
    skip_whitespace(text, cursor);
    if text.get(*cursor) == Some(&'{') {
        *cursor += 1;
        loop {
            skip_whitespace(text, cursor);
            match text.get(*cursor) {
                None | Some('}') => break,
                Some(',') => *cursor += 1,
                Some(_) => expand_tree(text, cursor, prefix, out),
            }
        }
        *cursor += 1;
        return;
    }
    let start = *cursor;
    while text.get(*cursor).copied().is_some_and(is_identifier_char) {
        *cursor += 1;
    }
    if start == *cursor {
        // A glob, or a path that ends before something that is not an identifier.
        if *cursor < text.len() && text[*cursor] != '}' && text[*cursor] != ',' {
            *cursor += 1;
        }
        if !prefix.is_empty() {
            out.push(prefix.to_vec());
        }
        return;
    }
    let mut path = prefix.to_vec();
    path.push(text[start..*cursor].iter().collect());
    if text.get(*cursor) == Some(&':') && text.get(*cursor + 1) == Some(&':') {
        *cursor += 2;
        expand_tree(text, cursor, &path, out);
        return;
    }
    skip_alias(text, cursor);
    out.push(path);
}

/// Consumes an `as <name>` rename.
fn skip_alias(text: &[char], cursor: &mut usize) {
    let mut look = *cursor;
    skip_whitespace(text, &mut look);
    if text.get(look) == Some(&'a') && text.get(look + 1) == Some(&'s') && look > *cursor {
        look += 2;
        if text
            .get(look)
            .is_some_and(|character| character.is_whitespace())
        {
            skip_whitespace(text, &mut look);
            while text.get(look).copied().is_some_and(is_identifier_char) {
                look += 1;
            }
            *cursor = look;
        }
    }
}

/// One `crate::` edge: the importing file, and the full path it names.
struct Edge {
    file: String,
    path: Vec<String>,
}

fn edges(files: &BTreeMap<String, String>) -> Vec<Edge> {
    let mut found = Vec::new();
    for (file, text) in files {
        for path in crate_paths(&code_only(text)) {
            found.push(Edge {
                file: file.clone(),
                path,
            });
        }
    }
    found
}

/// The module a path resolves to: its longest prefix that is a module of this crate.
fn resolve(path: &[String], modules: &BTreeSet<Vec<String>>) -> Option<Vec<String>> {
    (1..=path.len())
        .rev()
        .map(|length| path[..length].to_vec())
        .find(|prefix| modules.contains(prefix))
}

/// L-1: the top of `src/` is the eight directories and `lib.rs`, and the modules the map names
/// are where the map puts them.
///
/// Architecture evidence: AD-004 L-1
#[test]
fn l_1_src_holds_the_layout_directories_and_every_mapped_module_in_its_place() {
    let mut expected: BTreeSet<String> =
        DIRECTORIES.iter().map(|name| (*name).to_owned()).collect();
    expected.insert("lib.rs".to_owned());
    let listed: BTreeSet<String> = fs::read_dir(source_root())
        .expect("a readable src directory")
        .map(|entry| {
            entry
                .expect("a readable directory entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    assert_eq!(
        listed, expected,
        "src/ must hold exactly the layout directories and lib.rs"
    );

    let files = source_files();
    let missing: Vec<&str> = MODULE_FILES
        .iter()
        .copied()
        .filter(|module| !files.contains_key(*module))
        .collect();
    assert!(
        missing.is_empty(),
        "modules missing from their place: {missing:?}"
    );
}

/// L-2, the first clause: no `crate::` path names a root item. Every path starts at a directory
/// of the layout.
///
/// Architecture evidence: AD-004 L-2
#[test]
fn l_2_no_file_imports_an_item_through_the_crate_root() {
    let bare: Vec<String> = edges(&source_files())
        .into_iter()
        .filter(|edge| !DIRECTORIES.contains(&edge.path[0].as_str()))
        .map(|edge| format!("{}: crate::{}", edge.file, edge.path.join("::")))
        .collect();
    assert!(bare.is_empty(), "a root path in code: {bare:#?}");
}

/// L-2, the second clause: every import points at a directory its own directory may reach, and
/// inside `kani/` only at an earlier name.
///
/// Architecture evidence: AD-004 L-2
#[test]
fn l_2_every_import_follows_the_dependency_direction() {
    let files = source_files();
    let modules: BTreeSet<Vec<String>> = files.keys().map(|file| module_of(file)).collect();
    let mut violations = Vec::new();
    for edge in edges(&files) {
        let from = module_of(&edge.file);
        let (Some(directory), Some(target)) = (from.first(), edge.path.first()) else {
            continue;
        };
        let reach = may_import(directory);
        if directory != target && !reach.contains(&target.as_str()) {
            violations.push(format!(
                "{}: crate::{} leaves `{directory}` for `{target}`",
                edge.file,
                edge.path.join("::")
            ));
        }
        if let Some(allowed) = file_import_allowlist(&edge.file) {
            if !allowed.iter().any(|prefix| {
                edge.path.len() >= prefix.len()
                    && edge.path.iter().zip(prefix.iter()).all(|(a, b)| a == b)
            }) {
                violations.push(format!(
                    "{}: crate::{} is outside the file's allowed set {allowed:?}",
                    edge.file,
                    edge.path.join("::")
                ));
            }
        }
        if let Some(resolved) = resolve(&edge.path, &modules) {
            if let Some(reason) = kani_order_violation(&from, &resolved) {
                violations.push(format!(
                    "{}: crate::{}: {reason}",
                    edge.file,
                    edge.path.join("::")
                ));
            }
        }
    }
    assert!(
        violations.is_empty(),
        "imports against the direction: {violations:#?}"
    );
}

/// L-2, the third clause: no file but the crate root re-exports an item. A `pub use` or
/// `pub(crate) use` in a directory would hand another directory an item of a directory it may not
/// import (AD-004: the arrows are not transitive), and the second clause reads only the
/// importer's own paths, so it could not see it.
///
/// Architecture evidence: AD-004 L-2
#[test]
fn l_2_no_file_but_the_crate_root_re_exports_an_item() {
    let reexports: Vec<String> = source_files()
        .iter()
        .filter(|(file, _)| file.as_str() != "lib.rs")
        .flat_map(|(file, source)| {
            code_only(source)
                .lines()
                .map(str::trim_start)
                .filter(|line| {
                    line.starts_with("pub use ")
                        || (line.starts_with("pub(") && line.contains(") use "))
                })
                .map(|line| format!("{file}: {line}"))
                .collect::<Vec<_>>()
        })
        .collect();
    assert!(
        reexports.is_empty(),
        "a re-export in a directory: {reexports:#?}"
    );
}

/// The reason an import inside `kani/` breaks the AD's order, if it does: first among the
/// top-level names of `kani/`, then among the files of `kani/generate/`.
fn kani_order_violation(from: &[String], to: &[String]) -> Option<String> {
    if from.first().map(String::as_str) != Some("kani")
        || to.first().map(String::as_str) != Some("kani")
    {
        return None;
    }
    let (importer, imported) = (from.get(1)?, to.get(1)?);
    if importer == imported {
        if importer == "generate" {
            return generate_order_violation(from.get(2)?, to.get(2)?);
        }
        return None;
    }
    if imported == "test_support" {
        return None;
    }
    if let Some(allowed) = kani_exact_imports(importer) {
        return (!allowed.contains(&imported.as_str()))
            .then(|| format!("`kani::{importer}` may import only {allowed:?}, not `{imported}`"));
    }
    let (Some(importer_rank), Some(imported_rank)) = (kani_rank(importer), kani_rank(imported))
    else {
        return None;
    };
    (imported_rank >= importer_rank)
        .then(|| format!("`kani::{importer}` imports `kani::{imported}`, which is not earlier"))
}

/// What a file of `kani/generate/` imports from its own directory: an earlier name only, a
/// family never imports `negotiate`, and `negotiate`, `frame` and the leaf `outcome` take only
/// the names the AD lists for them.
fn generate_order_violation(importer: &str, imported: &str) -> Option<String> {
    if importer == imported {
        return None;
    }
    if let Some(allowed) = generate_exact_imports(importer) {
        return (!allowed.contains(&imported)).then(|| {
            format!("`generate::{importer}` may import only {allowed:?}, not `{imported}`")
        });
    }
    let (Some(importer_rank), Some(imported_rank)) =
        (generate_rank(importer), generate_rank(imported))
    else {
        return None;
    };
    (imported_rank >= importer_rank).then(|| {
        format!("`generate::{importer}` imports `generate::{imported}`, which is not earlier")
    })
}

/// The crate paths a single file may import, where AD-004 names the set: the collection family
/// imports `core`, `kani::abi`, `kani::identity`, and `outcome` and `record` of its own directory.
fn file_import_allowlist(file: &str) -> Option<&'static [&'static [&'static str]]> {
    match file {
        "kani/generate/collection.rs" => Some(&[
            &["core"],
            &["kani", "abi"],
            &["kani", "identity"],
            &["kani", "generate", "outcome"],
            &["kani", "generate", "record"],
            &["kani", "generate", "render"],
            &["kani", "generate", "spec"],
        ]),
        _ => None,
    }
}

/// Importers inside `kani/` the AD restricts to a named set of top-level names.
fn kani_exact_imports(importer: &str) -> Option<&'static [&'static str]> {
    match importer {
        "output" | "test_support" => Some(&["identity", "abi"]),
        "run" => Some(&["identity", "output", "classify", "abi"]),
        _ => None,
    }
}

/// Importers inside `kani/generate/` the AD restricts to a named set of its files.
fn generate_exact_imports(importer: &str) -> Option<&'static [&'static str]> {
    match importer {
        "outcome" => Some(&[]),
        "collection" => Some(&["outcome", "record", "spec", "render"]),
        "frame" => Some(&["outcome"]),
        "negotiate" => Some(&[
            "outcome",
            "record",
            "scalar",
            "collection",
            "clause",
            "precondition",
            "contract",
            "frame",
        ]),
        _ => None,
    }
}

/// The order of the files of `kani/generate/`, earliest first; a file imports only earlier names.
/// `precondition` and `contract` share a rank, so neither imports the other.
fn generate_rank(name: &str) -> Option<u8> {
    match name {
        "outcome" => Some(0),
        "record" => Some(1),
        "census_validation" => Some(2),
        "scalar" | "collection" => Some(3),
        "clause" => Some(4),
        "precondition" | "contract" => Some(5),
        "frame" => Some(6),
        "lower" => Some(7),
        "corpus" => Some(8),
        "v1_bundle" => Some(9),
        "negotiate" => Some(10),
        _ => None,
    }
}

/// L-2, the third clause: the module graph has no cycle.
///
/// Architecture evidence: AD-004 L-2
#[test]
fn l_2_the_module_graph_is_acyclic() {
    let files = source_files();
    let modules: BTreeSet<Vec<String>> = files.keys().map(|file| module_of(file)).collect();
    let mut graph: BTreeMap<Vec<String>, BTreeSet<Vec<String>>> = BTreeMap::new();
    for edge in edges(&files) {
        let from = module_of(&edge.file);
        if from.is_empty() {
            continue;
        }
        if let Some(to) = resolve(&edge.path, &modules) {
            if to != from {
                graph.entry(from).or_default().insert(to);
            }
        }
    }
    // The scan must have read real imports, or an empty graph would pass for acyclic.
    assert!(
        graph.len() >= 30,
        "the scan found imports in only {} modules",
        graph.len()
    );
    let negotiate = module_of("kani/generate/negotiate.rs");
    let outcome = module_of("kani/generate/outcome.rs");
    assert!(
        graph
            .get(&negotiate)
            .is_some_and(|imports| imports.contains(&outcome)),
        "the scan must see `negotiate` import `outcome`"
    );
    let mut state: BTreeMap<Vec<String>, bool> = BTreeMap::new();
    let cycle = graph.keys().find_map(|start| {
        let mut trail = Vec::new();
        find_cycle(start, &graph, &mut state, &mut trail)
    });
    let names: Option<Vec<String>> =
        cycle.map(|cycle| cycle.iter().map(|module| module.join("::")).collect());
    assert_eq!(names, None, "an import cycle");
}

/// Depth-first search; `state` maps a module to `false` while on the trail and `true` once done.
fn find_cycle(
    module: &Vec<String>,
    graph: &BTreeMap<Vec<String>, BTreeSet<Vec<String>>>,
    state: &mut BTreeMap<Vec<String>, bool>,
    trail: &mut Vec<Vec<String>>,
) -> Option<Vec<Vec<String>>> {
    match state.get(module) {
        Some(true) => return None,
        Some(false) => {
            let start = trail.iter().position(|seen| seen == module).unwrap_or(0);
            let mut cycle = trail[start..].to_vec();
            cycle.push(module.clone());
            return Some(cycle);
        }
        None => {}
    }
    state.insert(module.clone(), false);
    trail.push(module.clone());
    for next in graph.get(module).into_iter().flatten() {
        if let Some(cycle) = find_cycle(next, graph, state, trail) {
            return Some(cycle);
        }
    }
    trail.pop();
    state.insert(module.clone(), true);
    None
}

/// The scan reads code, not comments or literals, and expands a use tree to its full paths.
///
/// Architecture evidence: AD-004 L-2
#[test]
fn l_2_the_scan_reads_code_paths_and_ignores_comments_and_literals() {
    let source = "\
//! docs name crate::Item
use crate::{core::artifact::Artifact, kani::{abi::KaniSolver, identity as ident}, Bare};
const TEMPLATE: &str = \"use crate::Hidden;\";
const RAW: &str = r#\"crate::AlsoHidden\"#;
/* crate::InComment */
fn call() { crate::oracle::claim::build(); let _ = $crate::Skipped; }
";
    let mut found: Vec<String> = crate_paths(&code_only(source))
        .into_iter()
        .map(|path| path.join("::"))
        .collect();
    found.sort();
    assert_eq!(
        found,
        [
            "Bare",
            "core::artifact::Artifact",
            "kani::abi::KaniSolver",
            "kani::identity",
            "oracle::claim::build",
        ]
    );
}
