// Fixture: a quote-prefixed mention of unsafe, and a real unsafe block, on the
// SAME line. The real block must still be flagged after the mention is
// stripped. Regression target: #118.
fn case_mention_then_block() {
    let m = "unsafe {"; unsafe { f("z") };
}
