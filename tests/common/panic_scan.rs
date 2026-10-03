//! The panic-token scan of FR-014-AC-39 and FR-018-AC-19, shared by the generator-source scans and
//! the emitted-source scans so that one list defines what "panic-free" means.

/// The method-call tokens the scan bans. `.unwrap_or_else(` does not match `.unwrap(`.
const PANIC_CALLS: [&str; 3] = [".unwrap(", ".expect(", ".unwrap_unchecked("];

/// The macros the scan bans, in any delimiter form: the name, optional whitespace, then `!`.
/// `debug_assert` is listed beside `assert` so that a hit names the macro actually written.
const PANIC_MACROS: [&str; 8] = [
    "panic",
    "unreachable",
    "todo",
    "unimplemented",
    "assert",
    "assert_eq",
    "assert_ne",
    "debug_assert",
];

/// The path token the scan bans.
const PANIC_PATH: &str = "process::abort";

/// Every banned token `text` contains, each named once, in the order the scan lists them.
pub fn panic_tokens_in(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    for call in PANIC_CALLS {
        if text.contains(call) {
            found.push(call.to_owned());
        }
    }
    for name in PANIC_MACROS {
        let written = text.match_indices(name).any(|(at, _)| {
            text.get(at + name.len()..)
                .is_some_and(|rest| rest.trim_start().starts_with('!'))
        });
        if written {
            found.push(format!("{name}!"));
        }
    }
    if text.contains(PANIC_PATH) {
        found.push(PANIC_PATH.to_owned());
    }
    found
}

/// The non-test code of a Rust source file: everything before its `#[cfg(test)]` module, with
/// comment lines dropped. String literals are kept: what a generator emits counts.
pub fn non_test_code(source: &str) -> String {
    source
        .split_once("#[cfg(test)]")
        .map_or(source, |(before, _)| before)
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n")
}
