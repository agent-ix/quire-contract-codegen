// Fixture: `unsafe` on the physical line that closes a multi-line string
// literal opened on the line before it. Must be flagged. Regression: #118.
fn case_multiline_string() {
    let s = "line one
line two"; unsafe { f("y") };
}
