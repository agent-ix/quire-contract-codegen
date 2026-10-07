---
id: "SR-3161"
title: "IR-635 PR 326 gap analysis: original Eq constructor traceability"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen PR 326, branch code/ir-635-original-eq-builder (frozen head recorded in the IR-635 Linear review marker, not here); spec/replay/functional/FR-033-composite-parity-replay-binding.md, spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/replay/matrix/TC-048-composite-parity-replay-binding.md, src/replay/composite_builder.rs, tests/it/composite_original_eq.rs"
---

# SR-3161: IR-635 PR 326 gap analysis

## Summary

Ticket: IR-635. Planless run over the PR's criteria. Static: the computed matrix was read with
`quire matrix --format tsv` on the PR head; no tests were executed by the reviewer. Semantic
review (intent, test, code) was done by hand for the constructor criteria, not through the
optional fan-out.

Computed matrix for the criteria this PR touches:

- FR-033-AC-1: tagged by six new constructor controls (parameter/parameter report, literal
  operands, wrong source/package, wrong node/occurrence/function, stage-limit recompile, imported
  refusal). Status PARTIAL.
- FR-033-AC-9: tagged by five new controls plus the existing IR-666 converter tests. PARTIAL.
- FR-033-AC-11: tagged by four new controls (positional report, literal operands, self
  comparison, duplicate bound). PARTIAL.
- FR-033-AC-12: tagged by the wrong node/occurrence/function control plus IR-666. PARTIAL.
- FR-033-AC-2 to AC-6, AC-8, AC-10, FR-029-AC-17, AC-19, AC-21 to AC-27 and FR-025-AC-9:
  untagged, each PLANNED or GATED in its text. These were untagged before the PR; the PR adds no
  untagged criterion and loses no binder.
- The repository-wide `--strict` failure (untagged planned criteria) is pre-existing and is not
  made worse by this PR.

Reverse gap: every new public item (`OriginalCompositeEqContext`, `OriginalCompositeEqRequest`,
`OriginalCompositeEqReport`, `CompositeBuildError`) is owned by interface-001 and FR-033; no
unowned behaviour was found. No stubs, placeholder returns or `todo!` in the new code. Tests
assert measured values (positional identities, bound entries, report claim equals sent identity,
terminal `Tested`), not constants they set.

Plan completion: not assessed

## Verdict

CONDITIONAL. Tagged criteria are honestly PARTIAL and the untagged ones are PLANNED and
pre-existing. Two gaps: refusal branches credited to AC-1 and AC-9 have no control (FND-001), and
the AC-1/AC-11 clause that a QSL common-step mismatch through the constructor yields a
binding-checked `Refused` report is reachable but unexercised and not named as remaining in the
status rows (FND-002).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-9's status credits "Eq-only verified source/package/node/occurrence prechecks", but the package and occurrence prechecks have no control: no test produces `PackageMismatch`, `ContextMismatch` or `OccurrenceOutsideFunction` (the "another package" control asserts `SourceMismatch`; the occurrence control reaches only IR's `MissingOccurrence`). Deleting any of those checks leaves every tagged test green. Same root as SR-3160 FND-002. | spec/replay/functional/FR-033-composite-parity-replay-binding.md (FR-033-AC-9 row); tests/it/composite_original_eq.rs:326-374; src/replay/composite_builder.rs:176-184, 307-326, 363-385 |
| FND-002 | medium | FR-033-AC-1 requires that a QSL common-step mismatch return a binding-checked `Refused` report with its terminal value, and AC-11 names the HarnessUnknownKey `Refused` mapping. Through the constructor this is reachable (a Node-keyed harness bound for a node that is not an operand passes CG's filter into the claim and QSL refuses it in `prepare`), but no constructor control asserts it, and neither the AC-1 PARTIAL label nor the replay matrix row lists it as remaining. | spec/replay/functional/FR-033-composite-parity-replay-binding.md (FR-033-AC-1, FR-033-AC-11 rows); spec/replay/matrix/tests.md (FR-033-AC-1/AC-11 row); src/replay/composite_builder.rs:266-270 |
| FND-003 | low | FR-033-AC-12 is tagged by the wrong node/occurrence/function control, which never supplies Disagreed evidence. Precedence holds by construction (no evidence can be supplied before `request` succeeds), so the binding is weak but not wrong; the test's doc should say it demonstrates precedence structurally, not by a Disagreed input. | tests/it/composite_original_eq.rs:352-374 |
