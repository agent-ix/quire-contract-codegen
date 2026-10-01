---
id: "SR-751"
title: "CG PR 223 gap analysis: AD-004 step 2d-0, core/naming.rs"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@c6e5186329824f3ef725be50ef1cb85688941842; src/ (the 17 files the PR touches), spec/assurance/AD-004-cg-crate-layout.md (step 2d-0, module map rows for oracle naming and MAX_GENERATED_SOURCE_BYTES, Risks), spec/routed (FR-022, TC-033, matrix)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: references
---

# SR-751: CG PR 223 gap analysis

## Summary

Ticket: IR-348. PR: agent-ix/quire-contract-codegen#223 at c6e5186. Planless run, scoped to the
change: a definition move, so the questions are whether the move lost a spec-test-code link,
whether it does what AD-004 step 2d-0 asks, and whether the rest of the AD still reads
consistently.

- Matrix. `quire coverage --scope . --json` (quire 0.33.0) at base 224ca6e and at head:
  identical totals (210 of 319 rows backed, 284 criteria, 66 unbacked, 0 status lies, 4
  untracked symbols, 75 unmatched tags). The `implements` list gains one entry, the
  `// Implements: FR-022` line on `mod naming` in `src/core/mod.rs`, and loses none. The only
  `unmatched_tags` change is a two-line shift of a pre-existing `exact_scalar.rs` test. The three
  FR-022-AC-9 / TC-033 tests moved with their tags unchanged.
- Step 2d-0 as written (AD-004:609-619 and module-map rows :232-233). Items: the nine named
  helpers moved to `core/naming.rs` and `MAX_GENERATED_SOURCE_BYTES` to `core/artifact.rs`, as
  listed. Visibility: `observation_name` became `pub(crate)`, the one edit the AD allows. Tests:
  moved with trace tags unchanged. The four items the AD keeps in `oracle` stayed. No behaviour
  change: byte-identical output, measured independently in SR-750.
- Importers. The AD says "points every importer at the new module path"; it carries no
  enumerated importer list. `bound.rs` and the inline `crate::oracle::readable_name_component`
  in `kani.rs` are covered by "every importer", so the PR body's "AD importer-list correction"
  is not needed: the committed AD is already right. (The list the coder calls incomplete is not
  in the AD.)
- Rest of the AD. Rule 4 ("every user sits above `core`") holds for every user found. The
  module-map row for the naming helpers already says `reference_key`, `dependency_key`,
  `dependency_parameters` and `typed_dependency_parameters` stay for `oracle/boolean_v1.rs` at
  step 2d; nothing in 2d to 2g depends on a helper still being in `oracle`. The Risks line naming
  2d-0 as a definition move that must be byte-identical is satisfied.
- Spec text. No spec, matrix or interface text names `oracle::unique_names`,
  `oracle::oracle_symbol` or the constant's old path. AD-001:198 mentions `src/oracle.rs` as a V1
  path, which is still true.
- Stubs and hollow evidence: none; the PR adds no logic.
- Open PRs. CG #222 (step 1a, head 34fec3f) is based on 041c5ae, before 2c, and already conflicts
  with main in `src/bounded_kani_corpus.rs`, `src/kani.rs`, `src/lib.rs` and `src/oracle.rs`.
  Against this head the same four files conflict; the only addition is a second `oracle.rs` hunk
  where #222 deletes `deterministic_json` next to the removed helpers, and the `use` block hunk.
  Both are mechanical. Report only.

## Verdict

PASS. The PR does what AD-004 step 2d-0 says and loses no trace link. No gap found.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
