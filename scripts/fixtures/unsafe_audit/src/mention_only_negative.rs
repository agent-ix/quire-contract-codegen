// Fixture: a line that is ONLY a mention of the word unsafe, immediately
// quote-prefixed, inside a string literal -- the same shape as
// tests/exact_scalar_generation.rs:1118 in this repository. Must NOT be
// flagged.
fn case_mention_only() {
    let forbidden = ["unsafe {", "unsafe fn"];
}
