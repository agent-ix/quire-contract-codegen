// Fixture: a plain uncommented unsafe block with no escape-adjacent construct
// anywhere near it. Must be flagged. This is the baseline positive case that
// every one of the escape-adjacent fixtures is a harder variant of.
fn case_trivial_positive() {
    unsafe { f() };
}
