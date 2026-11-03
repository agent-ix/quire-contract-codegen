---
id: SR-1121023
title: "IR-1120 CG #643 rust-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; PR #643 head 3f40b53ab057c52d72fdf49ca29faf2bcef89572; rust-review over changed Rust files"
review_set: subset
---

## Summary

Independent static Rust idiom and API review of `src/routed/smt/mod.rs`, `src/routed/smt/refinement.rs` and `src/routed/smt/refinement_source_tests.rs`. There are no prior Rust-review findings for this PR.

## Verdict

PASS with one low style note. Test soundness is carried by SR-1121021 FND-002.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | `cvc5_proof_dump_selected` spells `std::ffi::OsString` inline while the same file imports from `std::ffi` at the top. | src/routed/smt/refinement.rs:5; src/routed/smt/refinement.rs:263 |

## Coverage

Checked with no finding:

- Slice indexing in `captured_alethe_proof` is bounded by `sexpr_end`, and depth arithmetic is checked.
- `Option::is_none_or` and `trim_ascii` fit `rust-version = "1.98.1"`.
- Private `RefinementRun` fields with `#[must_use]` accessors are a sound narrowing of the prior public fields.
- The two `.expect` calls now in non-test `serialize_verified_fields` sit behind `validate_records`, which refuses absent fields first; `.agent/rules/writing_rust.md` has no rule against them.
- `#[path]` for the unit-test module matches repo precedent and is needed for private-field mutation; this module does not need to move to `tests/it/`.
- Direct `quire_canonical` calls in test code match existing test sites, though the `Cargo.toml` comment says `core::canonical` only. That comment was already inaccurate before this PR.

This reviewer ran no compiler, clippy, test or Quire validation. The clean `-D warnings` result is the lead's log. The `/rust-review` and `rust-style` skills were not loadable here, so the review used repo conventions and general Rust idiom. Plan completion: not assessed.

## Examined scope and bindings

```yaml
scope:
- id: FR-045-AC-4
  path: spec/routed/functional/FR-045-hyper-smt-induction-map.md
  role: examined
  excerpt: For a refinement with seven step rows, CG retains one `Base` and exactly
    seven typed `Step { row }` records numbered 0 through 6 from the recompiled, canonically
    ordered `steps` array, even when records arrive out of order or the source rows
    were written in another order. Each record binds the refinement item, subjects,
    distinct invocation attempt, role and row, and exact FR-315 `Initial` or `Step
    { row }` query bytes sent on stdin; changing a query byte or exchanging the base
    and a step role refuses the handoff. With zero step rows, the complete shape has
    only `Base`.
bindings: []
```

## Dispositions

| Finding | Outcome | Evidence |
| --- | --- | --- |
| FND-001 | accepted-no-change | Style only, with no behavior effect; it can be fixed in passing with the test change. |
