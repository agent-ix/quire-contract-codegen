---
id: "SR-3162"
title: "IR-635 PR 326 spec review (base): Eq constructor status edits"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen PR 326, branch code/ir-635-original-eq-builder (frozen head recorded in the IR-635 Linear review marker, not here); spec/core/functional/interface-001-codegen-api.md, spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/kani/matrix/tests.md, spec/replay/functional/FR-033-composite-parity-replay-binding.md, spec/replay/matrix/TC-048-composite-parity-replay-binding.md, spec/replay/matrix/tests.md"
---

# SR-3162: IR-635 PR 326 spec review (base)

## Summary

Ticket: IR-635. Base checklist over the six changed spec files; review set is base plus the
integrity lens (SR-3163). Static; the author reports `make spec` passing on the head.

Examined: the three interface-001 operations added (`OriginalCompositeEqContext::new`,
`::request`, `OriginalCompositeEqRequest::settle_verified`); FR-033 description paragraphs and
the AC-1, AC-9, AC-11, AC-12 status prefixes; FR-029 composite heading, the new IR-635 paragraph
and AC-17; TC-048 scope paragraph and step 8a heading; both matrix index rows.

What is right: ID formats and links are intact; no criterion text other than status prefixes
changed; AC-2 to AC-8, AC-10 and AC-13 stay PLANNED/GATED; FR-029-AC-17 and AC-19 to AC-27 stay
planned and the new paragraph explicitly denies FR-028 strength or `run_terminal_value` credit;
Ne (IR-690), bounded-field literals (IR-691), collection literals, imported contexts and the
non-retention of dependency packages are recorded as gaps. No hash, digest, pin or tracking
record was added; the only digest named is the canonical proof-content identity, which the
repository allows. No SHA or hex in the changed spec text.

## Verdict

CONDITIONAL. The status edits are mostly honest, but the AC-9 and TC-048 wording credits package
and occurrence precheck refusals that no control exercises (FND-001), and the imported-context
sentence is repeated in four documents including one that does not own the constructor (FND-002).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Overclaimed AC credit: FR-033-AC-9's status says the Eq constructor's "source/package/node/occurrence prechecks ... covered", the replay matrix row says original "source/package/function/node/occurrence ... reach genuine QSL reports", and TC-048 lists "wrong source/package/node/occurrence/function refusals". The package refusal is measured only as `SourceMismatch`, and `PackageMismatch`, `ContextMismatch` and `OccurrenceOutsideFunction` are never produced by a test. Either add the controls or narrow the wording to what is measured. | spec/replay/functional/FR-033-composite-parity-replay-binding.md (FR-033-AC-9 row); spec/replay/matrix/tests.md (FR-033-AC-1/AC-11 row); spec/replay/matrix/TC-048-composite-parity-replay-binding.md (scope paragraph) |
| FND-002 | low | One fact, four places: the "Imported contexts remain PLANNED ..." sentence is inserted verbatim above the first section of FR-029, FR-033 and both matrix indexes. FR-029 owns terminal mapping, not the constructor, and a preamble before `## Description` sits outside any section. State it once in FR-033 (and in the TC-048 scope, where it already is) and reference it elsewhere. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:23-24; spec/kani/matrix/tests.md:9-10; spec/replay/matrix/tests.md:9-10; spec/replay/functional/FR-033-composite-parity-replay-binding.md:39-40 |
## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | e8cf5261ac45: Added concrete semantic package/context controls and QSL membership/unknown-bound controls; narrowed TC/matrix status wording to the measured outcomes. |
| FND-002 | fixed | e8cf5261ac45: Removed duplicate imported-context sentence from FR-029 and both matrix indexes; retained it under FR-033 Description and TC-048 scope. |
