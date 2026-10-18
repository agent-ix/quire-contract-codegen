---
id: SR-12861
title: IR-858 step 4b spec-review/ears-analysis
type: SpecReview
analysis: ears-conformance
scope: 'agent-ix/quire-contract-codegen; IR-858 PR #421 own specification delta in
  AD-004, FR-015, TC-025 and matrix index'
review_set: subset
---

## Summary

Ticket IR-858; independently examined the frozen #421 own delta against its prepared #417 parent. Grammar check: quire validate --scope . "spec/**/*.md" --summary exited 0. Result: 107/109 grammar-clean documents, three advisory findings solely in unchanged FR-017 (unclassifiable, missing-subject) and FR-014 (non-canonical-trigger). EARS governs requirement statements, not AC tables; no shall rewrite is required for six AC rows. Architecture and test-procedure prose are outside the grammar lens. See ears-engine.log for complete deterministic output.

## Verdict

PASS — no changed FR/NFR/StR requirement statement introduces an EARS defect. This patch adds six planned acceptance criteria, without changing the FR-015 requirement statement. Their event/result semantics are concrete: constructor refusal, emission order, determinism, equivalence, generic rendering and backend smoke. The engine reports three advisory warnings in unchanged FR-017 and FR-014; these are baseline context, not #421 findings.

## Scope

Every changed paragraph/table row examined by this method is recorded below with its exact text excerpt (up to 600 characters). Context-only entries are not assertions of implementation completion.

```yaml
scope:
- id: FR-015-AC-112
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: '| FR-015-AC-112 | PLANNED (IR-858, AD-004 step 4b). `HarnessSpec::new` is the only constructor
    and refuses with a typed `HarnessSpecError`, never a panic and never a harness: a spec with no cover
    or more than one cover; a cover whose condition is the literal `true`; no `Call` or more than one;
    an `Input` or `Assume` after the `Call`; an `Assert` before it; a binding name bound twice (other
    than an input''s own rebinding); a name, `result` or `callee` that is not a valid Rust identifier;
    an input domain with `minimum` above `maximum` or outside its type; an `unwind` outside `1..=MAX_OBLIGATION_U'
- id: FR-015-AC-113
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: '| FR-015-AC-113 | PLANNED (IR-858). For every spec that constructs, `render` emits the header,
    the oracle source, the module wrapper and the proof attribute, then the spec''s statements in spec
    order, then its one cover as the last statement of the function: after every assumption, the subject
    call and every assertion, whatever order a family listed them in. A hand-built spec in each of several
    statement orders renders its cover last; a cover is not a field of `body` and no statement order moves
    it (AD-004 "The HarnessSpec and render contract (step 4b)", R-1; FR-015-AC-7). | Test (TC-025) |'
- id: FR-015-AC-114
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: '| FR-015-AC-114 | PLANNED (IR-858). `render` of one spec returns equal bytes on repeated calls
    and in a second process, and a spec whose `Input` list is reordered renders the reordered source and
    not the same bytes (AD-004 "The HarnessSpec and render contract (step 4b)", R-1). | Test (TC-025)
    |'
- id: FR-015-AC-115
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: '| FR-015-AC-115 | PLANNED (IR-858). After the scalar port, for each claim of a fixed set (each
    of `quire.op.integer.add`, `sub`, `mul` and `negate`; a claim with a literal operand; a claim with
    an inferred operand; a claim with each operand at an `i64` extreme; claims at two unwind values including
    `MAX_OBLIGATION_UNWIND`; claims at two ceilings values) the ported path renders the same artifact
    (path and source bytes), `ScalarObligationIdentity`, `options` and `ObligationRecord`, and the same
    digests derived from them, as a test-only reference copy of the pre-port `render_scalar` computes
    in t'
- id: FR-015-AC-116
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: '| FR-015-AC-116 | PLANNED (IR-858). `render` holds no family-specific template text: for hand-built
    specs whose label, expressions and messages are not the scalar family''s, the rendered source consists
    of the fixed header, the spec''s `oracle_source`, the module wrapper, the proof attribute, the spec''s
    statements rendered from their typed fields, and the one cover, and contains no other text; and the
    ported scalar harness is emitted by `render`, so that the FR-015-AC-58 inspection drives `render.rs`
    for it (AD-004 "The HarnessSpec and render contract (step 4b)", R-1 item 3). | Test (TC-025) |'
- id: FR-015-AC-117
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: '| FR-015-AC-117 | PLANNED (IR-858). With the installed backend, after the scalar port, the
    unmutated scalar harness classifies `Verified` with its cover satisfied and the existing scalar mutation
    control (the oracle''s `Add` swapped for `Subtract`, FR-015-AC-37) classifies `Falsified`; and a hand-built
    `HarnessSpec` with an unsatisfiable `Assume`, rendered by `render`, does not classify `Verified` (L-4).
    This run is a smoke over the ported source and not the evidence of equivalence, which FR-015-AC-115
    is (AD-004 "The HarnessSpec and render contract (step 4b)", P-3). | Test (TC-025) |'
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
