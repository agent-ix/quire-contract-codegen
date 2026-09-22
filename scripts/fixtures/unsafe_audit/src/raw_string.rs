// Fixture: `unsafe` on the same line as a raw string literal containing a
// quote. Must be flagged. Regression target: #118.
fn case_raw_string() {
    let e = r#"x "y"#; unsafe { f("z") };
}
