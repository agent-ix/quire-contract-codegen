// Fixture: `unsafe` on the same line as a string literal containing an escaped
// quote. Must be flagged (no SAFETY comment). Regression target: #118, where a
// string-deletion approach paired quote characters positionally and silently
// swallowed this construct.
fn case_escaped_quote() {
    let b = "x \" y"; unsafe { f("z") };
}
