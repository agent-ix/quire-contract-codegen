// Fixture: `unsafe` on the same line as a char literal that is itself a double
// quote. Must be flagged. Regression target: #118, quote-parity swallowed this
// even though the quote count on the line is even.
fn case_char_quote() {
    let c = '"'; unsafe { f("z") };
}
