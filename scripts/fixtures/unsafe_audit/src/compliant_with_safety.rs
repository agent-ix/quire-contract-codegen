// Fixture: an unsafe block WITH a proper `// SAFETY:` comment. Must NOT be
// flagged, confirming the scanner correctly leaves compliant blocks alone.
fn case_compliant() {
    // SAFETY: test fixture only; no real invariant is being asserted, this
    // exists to prove the scanner does not flag a compliant block.
    unsafe { f() };
}
