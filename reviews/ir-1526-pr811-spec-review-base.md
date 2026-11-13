---
id: SR-17940486709564847047748
title: "IR-1526 PR #811 FR-056 tractability refusal spec base review"
type: SpecReview
analysis: base
date: 2026-11-13
scope: "agent-ix/quire-contract-codegen; PR #811 @ f524805c; IR-1526; spec/kani/functional/FR-056-kani-tractability-refusal.md, spec/kani/matrix/TC-070-kani-tractability-refusal.md, spec/kani/matrix/tests.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-056
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-070
    type: reviews
---

## Summary

Ticket: IR-1526. Spec-only PR (head f524805c), reviewed in a detached worktree scoped to
`git diff origin/main...HEAD` (4 files, +179). It adds FR-056 (decider seam, measured table as
runtime-loaded data, typed refusal `UnsupportedObligation::V2ClauseNotTractable`, corpus and scoring
test), TC-070 and one index row each in `spec/kani/matrix/tests.md` and `spec/spec.md`.
FR-015 and FR-017 text is untouched. `quire validate` (quire 0.44.0) passes on all four files with
one grammar warning (AC-4 is given-when-then shaped). `quire matrix` lists all 13 ACs, untagged,
which is right for a PLANNED row.

Code read without changes: `src/kani/generate/outcome.rs` (`V2ClauseOperatorNotSupported { node_id,
operation_identity }`), `src/kani/generate/clause/v2.rs` (`lower_with_census`,
`supported_clause_operator`), `src/oracle/clause.rs` (`V2ClauseDescriptor.operation_identities`),
`src/oracle/clause/native.rs` (interval propagation), `src/routed/capability.rs`
(`Cause::KaniFormRefused` carries an `UnsupportedObligation`, so a new arm flows through it with no
new cause). Also read: FR-015-AC-23/40/41/42 and FR-028-AC-2/12/25.

Checked and clean: the refusal is an arm of the existing `UnsupportedObligation`, not a parallel
family. Every AC has a TC-070 step. No titan:IR:proofs table rows are copied into CG: the table is
loaded at run time and FR-056 Status says so. There are no hashes, digests or SHA pins. The
generator and kernel versions on each row are a real staleness key for measured data, not ceremony.
There is no compatibility layer. No QSpec is cited. AC-13 is labelled `Test (installed Kani)`, and
"a probe record does not substitute" scopes it honestly. Its target operation is the problem
(FND-002).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AC-9 and the Description cite FR-028-AC-25 as the `TimedOut` reason, but FR-028-AC-25 is the memory-kill classification (`out_of_memory` to `Incomplete(ResourceExhausted)`, "never to `TimedOut`"). The wall-clock ceiling is FR-028-AC-2 (and AC-12 for groups). TC-070 step 9 has the same wrong reference. | spec/kani/functional/FR-056-kani-tractability-refusal.md:21,91; spec/kani/matrix/TC-070-kani-tractability-refusal.md:39-40; spec/kani/functional/FR-028-bounded-proof-ceilings.md:214,235 |
| FND-002 | high | AC-13, AC-4 (through TC-070 step 4) and TC-070 step 13 use Decimal multiply. The V2 clause arm admits only integer add, sub, mul and negate (`supported_clause_operator`, v2.rs:574-593; FR-015-AC-42). A decimal-multiply clause is therefore refused as `V2ClauseOperatorNotSupported` before any `operation_identities` exist, and the decider never runs. "Refused before any Kani launch" passes with no decider at all, and AC-4's own mutant describes today's behaviour. The Dependencies hedge ("nearest delivered measured-bad row") contradicts AC-13 as written. | spec/kani/functional/FR-056-kani-tractability-refusal.md:86,95,104-106; spec/kani/matrix/TC-070-kani-tractability-refusal.md:26-28,48-50; src/kani/generate/clause/v2.rs:127-139,574-593 |
| FND-003 | medium | The rows' "generator version" and "kernel version", and "CG's" own, are never defined. CG has no such identity today (no `generator_version`, `kernel_version` or `CARGO_PKG_VERSION` in src). The stale-row rule and AC-7 depend on what they compare. | spec/kani/functional/FR-056-kani-tractability-refusal.md:44,66,74,89 |
| FND-004 | medium | "Width in bits after range propagation" has no definition. CG propagates `NativeIntegerInterval` bounds, not bit widths (oracle/clause/native.rs:71-208). Signed or unsigned, and the bit count of an interval, are not stated: `Int[0, 2^k]` needs k+1 unsigned bits. AC-2's three widths and AC-5's 64/65 boundary depend on this. | spec/kani/functional/FR-056-kani-tractability-refusal.md:36-38,84,87; src/oracle/clause/native.rs:71 |
| FND-005 | medium | FR-056 never says how the table file reaches CG (API argument, CLI flag, config path) or what happens when none is supplied or the file is missing or unreadable. AC-3 covers only empty and malformed tables. | spec/kani/functional/FR-056-kani-tractability-refusal.md:43,67,85 |
| FND-006 | low | AC-1 says the table lookup is "the only implementation", but AC-12 and TC-070 step 1 require a second, always-`Run` decider behind the same interface. Say "the only production implementation". | spec/kani/functional/FR-056-kani-tractability-refusal.md:83,94; spec/kani/matrix/TC-070-kani-tractability-refusal.md:18 |
| FND-007 | low | AC-1's mutant "fails the feature-only check" names no check. A decider that reads the package is excluded only by the interface signature, which is checked by inspection or the compiler, not by a runtime test. | spec/kani/functional/FR-056-kani-tractability-refusal.md:83 |
| FND-008 | low | AC-4's refusal fields omit the clause node id. The sibling arm `V2ClauseOperatorNotSupported` carries `node_id` (outcome.rs:302-307), and FR-015-AC-42 requires a refusal "naming the node". | spec/kani/functional/FR-056-kani-tractability-refusal.md:86; src/kani/generate/outcome.rs:302 |
| FND-009 | low | AC-11's "the baseline is updated in the same PR" is a process step, not a test outcome, and the AC does not say where the baseline is recorded. | spec/kani/functional/FR-056-kani-tractability-refusal.md:93 |

## Verdict

FAIL: not mergeable as written. Fix FND-001 by citing FR-028-AC-2 (the `KaniRunOutcome::Inconclusive`
timed-out reason naming the ceiling) in AC-9, the Description and TC-070 step 9. Fix FND-002 by
moving AC-13 and TC-070 steps 4 and 13 to a measured-bad row of an operation the V2 clause arm admits
(for example `quire.op.integer.mul` at a measured width). Assert that the refusal is
`V2ClauseNotTractable`, not just a pre-launch refusal, and state that the decider runs after
operator admission. Drop or rescope the decimal hedge: decimal in a V2 clause needs an FR-015
operator extension, and FR-056 says it makes none. The medium findings are definitions that two
implementers would read differently. The low findings are wording.
