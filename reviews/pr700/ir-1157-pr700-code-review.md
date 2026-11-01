---
id: SR-202610310153
title: "code-review review of constrained method implementation"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; IR-1157 PR700 changed paths"
review_set: subset
---

## Summary

Ticket IR-1157, CG PR #700 frozen head bcc6436780d39ee712717e722518d0aa202adbd1 against origin/main 78e725a8. One independent review; no source edits or Cargo execution. FR-048 AC1..11, TC-062 and changed AD-004 boundaries were checked. No applicable AssuranceProfile exists. The generation entry is negotiate-only, renderer remains private/shared, identities sit below runner, and terminal conversion above runner. QSL UnsupportedTerminalRepresentation fallback is allowed and emits no forged terminal.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The method identity preimage retains only graph-local clause node IDs and optional native capacity decisions for its conditions. A general-oracle fallback stores None and omits the checked condition body or generated oracle bytes; admitted IDs serialize operand indices without the referenced bodies. Separately admitted same-label sources with a changed modulo literal can therefore reuse a method identity despite different executed oracle source. Bind complete canonical condition content for safety, constraint and preconditions and add a same-label changed-general-body substitution control. | src/kani/generate/constrained.rs:329-345 |
| FND-002 | medium | The sealed public backend carrier exposes checks, playback and Inconclusive reasons only. Cancelled is collapsed to generic Inconclusive by classification and its original KaniCancellationSource has no typed public accessor; original failure_cause and diagnostics are likewise only recoverable by serializing the private backend. Expose narrow typed non-proof/cause accessors without leaking ordinary Verified, and test Requested and Deadline cancellation plus original backend failure payloads. | src/kani/run/constrained.rs:19-61 |
| FND-003 | medium | Boundary reachability is computed before the owner CallFailure override and returned unchanged with RuntimeInvariant. A completed healthy boundary report paired with owner CallFailure therefore returns RuntimeInvariant and Reached boundary credit, although FR-048 requires no boundary credit on that fault. Clear or make boundary observation Unknown for owner faults and test both safety and constraint CallFailure against complete reached/not-reached backend reports. | src/kani/terminal/constrained.rs:176-194,258-263 |

## Verdict

REQUEST_CHANGES

## Coverage

Plan completion: not assessed. Parent owns final-head gates after fixes. Inspected qualification logs under qualification-logs/titan-ir1157: focused 9 integration controls pass, cover audit 7 pass, fmt and all-target/all-feature Clippy pass, installed lane four controls pass in 2.29 seconds. Full integration gate pending. No 16 GiB proof run.

## Examined scope

```json
[
  {
    "id": "FR-048-AC-1",
    "path": "spec/kani/functional/FR-048-constrained-state-clause-method.md",
    "role": "examined",
    "excerpt": "A selected constraint resolves in the obligation's admitted package and state context; absent, ambiguous, wrong-kind, wrong-package and wrong-context selections each yield a distinct typed refusal before harness generation or backend execution."
  },
  {
    "id": "FR-048-AC-2",
    "path": "spec/kani/functional/FR-048-constrained-state-clause-method.md",
    "role": "examined",
    "excerpt": "The same safety obligation run without a constraint, with constraint A, and with constraint B has three distinct method execution identities and retains the same semantic obligation identity. Cross-method evidence or replay substitution refuses before conversion; equal requests regenerate identically."
  },
  {
    "id": "FR-048-AC-3",
    "path": "spec/kani/functional/FR-048-constrained-state-clause-method.md",
    "role": "examined",
    "excerpt": "An initial boundary state and a later boundary state each undergo safety evaluation. A seeded safety violation at either boundary is falsified with in-domain playback; replacing the boundary branch with an early assumption would fail these controls. Neither boundary expands, stutters or counts as deadlock."
  },
  {
    "id": "FR-048-AC-4",
    "path": "spec/kani/functional/FR-048-constrained-state-clause-method.md",
    "role": "examined",
    "excerpt": "A healthy bounded method reaching a boundary satisfies its separate boundary cover and ordinary cover, yields ConstraintReached with Reached observation, and produces no ordinary Verified or proved terminal. A boundary cover records reachability rather than an invented distinct-state count."
  },
  {
    "id": "FR-048-AC-5",
    "path": "spec/kani/functional/FR-048-constrained-state-clause-method.md",
    "role": "examined",
    "excerpt": "A healthy method reaching no boundary retains a completed NotReached observation and, only with complete exhaustive evidence, a method-qualified Proved/Exhaustive/Uncertified terminal retaining the constraint under QSL FR-160; it receives no unconditional proof credit. An interrupted or missing boundary check retains Unknown and an inconclusive result. An unmet boundary cover alone cannot classify the ordinary cover as unsatisfied."
  },
  {
    "id": "FR-048-AC-6",
    "path": "spec/kani/functional/FR-048-constrained-state-clause-method.md",
    "role": "examined",
    "excerpt": "A constraint that is Undefined at a reached state retains the evaluation violation rather than classifying that state as a boundary. Typed Refused and Incomplete evaluations retain their original causes and cannot become a Boolean assumption or proof. CallFailure instead stops with runtime_invariant / established-invariant-broken, without being recast as Undefined, Refused or a boundary."
  },
  {
    "id": "FR-048-AC-7",
    "path": "spec/kani/functional/FR-048-constrained-state-clause-method.md",
    "role": "examined",
    "excerpt": "Removing ordinary non-vacuity evidence, exhausting unwind or resources, cancelling, or supplying malformed backend checks preserves the corresponding non-proof outcome even if the boundary cover was satisfied. Reordering check output does not change classification."
  },
  {
    "id": "FR-048-AC-8",
    "path": "spec/kani/functional/FR-048-constrained-state-clause-method.md",
    "role": "examined",
    "excerpt": "The public CG conversion implements every mapping row above using typed evidence and exact method binding. A boundary safety violation follows same-method replay and FR-029 precedence; replay disagreement, refusal, absent playback and identity mismatch never produce refuted or proved by default."
  },
  {
    "id": "FR-048-AC-9",
    "path": "spec/kani/functional/FR-048-constrained-state-clause-method.md",
    "role": "examined",
    "excerpt": "A public QSL terminal representation preserves the constraint method, the ConstraintReached non-proof status and the no-boundary method-qualified proof status; where that representation is unavailable, conversion returns a typed unsupported refusal. No private wire variant, missing method or unconditional proved fallback is emitted."
  },
  {
    "id": "FR-048-AC-10",
    "path": "spec/kani/functional/FR-048-constrained-state-clause-method.md",
    "role": "examined",
    "excerpt": "Installed-backend healthy-boundary, no-boundary and seeded-boundary-violation controls exercise the generated method and retain their exact typed check evidence. Independent QSL evaluation agrees on the bounded reached states and boundary safety observations. Neither generation-only inspection, saved verdict nor timeout substitutes for this evidence."
  },
  {
    "id": "FR-048-AC-11",
    "path": "spec/kani/functional/FR-048-constrained-state-clause-method.md",
    "role": "examined",
    "excerpt": "A standalone collection constraint remains unsupported under FR-015-AC-128 with no harness or proof credit. A method constraint never rescues an otherwise unbounded or unsupported state domain."
  },
  {
    "id": "Cargo.lock",
    "path": "Cargo.lock",
    "role": "examined",
    "excerpt": "# This file is automatically @generated by Cargo."
  },
  {
    "id": "spec/assurance/AD-004-cg-crate-layout.md",
    "path": "spec/assurance/AD-004-cg-crate-layout.md",
    "role": "examined",
    "excerpt": "---"
  },
  {
    "id": "spec/kani/matrix/TC-062-constrained-state-method.md",
    "path": "spec/kani/matrix/TC-062-constrained-state-method.md",
    "role": "examined",
    "excerpt": "---"
  },
  {
    "id": "src/core/ir/clause.rs",
    "path": "src/core/ir/clause.rs",
    "role": "examined",
    "excerpt": "use std::collections::{BTreeMap, BTreeSet};"
  },
  {
    "id": "src/kani/generate/clause/v2.rs",
    "path": "src/kani/generate/clause/v2.rs",
    "role": "examined",
    "excerpt": "use quire_contract_model::{CheckedNodeId, CheckedPackageV2, ContractSourceV1};"
  },
  {
    "id": "src/kani/generate/constrained.rs",
    "path": "src/kani/generate/constrained.rs",
    "role": "examined",
    "excerpt": "use super::outcome::constrained::*;"
  },
  {
    "id": "src/kani/generate/mod.rs",
    "path": "src/kani/generate/mod.rs",
    "role": "examined",
    "excerpt": "pub(crate) mod census_validation;"
  },
  {
    "id": "src/kani/generate/negotiate.rs",
    "path": "src/kani/generate/negotiate.rs",
    "role": "examined",
    "excerpt": "#[cfg(test)]"
  },
  {
    "id": "src/kani/generate/outcome.rs",
    "path": "src/kani/generate/outcome.rs",
    "role": "examined",
    "excerpt": "use quire_contract_model::{"
  },
  {
    "id": "src/kani/generate/outcome/constrained.rs",
    "path": "src/kani/generate/outcome/constrained.rs",
    "role": "examined",
    "excerpt": "use qsl_replay::SnapshotValue;"
  },
  {
    "id": "src/kani/generate/render.rs",
    "path": "src/kani/generate/render.rs",
    "role": "examined",
    "excerpt": "mod collection_operation;"
  },
  {
    "id": "src/kani/generate/render/constrained.rs",
    "path": "src/kani/generate/render/constrained.rs",
    "role": "examined",
    "excerpt": "use crate::core::artifact::{Artifact, MAX_GENERATED_SOURCE_BYTES};"
  },
  {
    "id": "src/kani/generate/spec.rs",
    "path": "src/kani/generate/spec.rs",
    "role": "examined",
    "excerpt": "use super::outcome::{UnsupportedObligation, MAX_OBLIGATION_UNWIND};"
  },
  {
    "id": "src/kani/identity.rs",
    "path": "src/kani/identity.rs",
    "role": "examined",
    "excerpt": "use std::{num::NonZeroU64, time::Duration};"
  },
  {
    "id": "src/kani/identity/constrained.rs",
    "path": "src/kani/identity/constrained.rs",
    "role": "examined",
    "excerpt": "use super::{ProofCeilings, SymbolicBounds};"
  },
  {
    "id": "src/kani/run/constrained.rs",
    "path": "src/kani/run/constrained.rs",
    "role": "examined",
    "excerpt": "use crate::kani::identity::constrained::*;"
  },
  {
    "id": "src/kani/run/execute.rs",
    "path": "src/kani/run/execute.rs",
    "role": "examined",
    "excerpt": "use std::{"
  },
  {
    "id": "src/kani/run/harness.rs",
    "path": "src/kani/run/harness.rs",
    "role": "examined",
    "excerpt": "use crate::{"
  },
  {
    "id": "src/kani/run/mod.rs",
    "path": "src/kani/run/mod.rs",
    "role": "examined",
    "excerpt": "pub(crate) mod execute;"
  },
  {
    "id": "src/kani/run/scalar.rs",
    "path": "src/kani/run/scalar.rs",
    "role": "examined",
    "excerpt": "use std::{"
  },
  {
    "id": "src/kani/terminal.rs",
    "path": "src/kani/terminal.rs",
    "role": "examined",
    "excerpt": "use qsl_replay::{"
  },
  {
    "id": "src/kani/terminal/constrained.rs",
    "path": "src/kani/terminal/constrained.rs",
    "role": "examined",
    "excerpt": "use super::ReplaySettlement;"
  },
  {
    "id": "src/kani/terminal/constrained/tests.rs",
    "path": "src/kani/terminal/constrained/tests.rs",
    "role": "examined",
    "excerpt": "use super::*;"
  },
  {
    "id": "src/lib.rs",
    "path": "src/lib.rs",
    "role": "examined",
    "excerpt": "#![forbid(unsafe_code)]"
  },
  {
    "id": "src/oracle/clause.rs",
    "path": "src/oracle/clause.rs",
    "role": "examined",
    "excerpt": "mod native;"
  },
  {
    "id": "src/oracle/clause/native.rs",
    "path": "src/oracle/clause/native.rs",
    "role": "examined",
    "excerpt": "use quire_exact::{NativeIntegerInterval, NativeIntegerOperation, NativeIntegerPlan};"
  },
  {
    "id": "src/oracle/scalar/render.rs",
    "path": "src/oracle/scalar/render.rs",
    "role": "examined",
    "excerpt": "use super::derive::{family_of_identity, Family};"
  },
  {
    "id": "tests/it/constrained_method.rs",
    "path": "tests/it/constrained_method.rs",
    "role": "examined",
    "excerpt": "mod support;"
  },
  {
    "id": "tests/it/constrained_method/support.rs",
    "path": "tests/it/constrained_method/support.rs",
    "role": "examined",
    "excerpt": "use qsl_replay::{"
  },
  {
    "id": "tests/it/cover_last.rs",
    "path": "tests/it/cover_last.rs",
    "role": "examined",
    "excerpt": "use std::collections::{BTreeMap, BTreeSet};"
  },
  {
    "id": "tests/it/kani_obligations.rs",
    "path": "tests/it/kani_obligations.rs",
    "role": "examined",
    "excerpt": "#[allow(clippy::duplicate_mod)]"
  },
  {
    "id": "tests/it/main.rs",
    "path": "tests/it/main.rs",
    "role": "examined",
    "excerpt": "#[path = \"../checked_package_support/evidence.rs\"]"
  }
]
```

## Dispositions

Disposition round 1 inspected fix head 043d398c9dc520e04c504a92455f67ceda75bce6 against the frozen reviewed head bcc64367.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Full owner oracle content is canonically bound before the method preimage; same-label safety/constraint/precondition modulo controls demonstrate the old collision and differing new identities. |
| FND-002 | fixed | Narrow typed cancellation, failure_cause, diagnostics and memory accessors preserve original evidence without exposing raw Verified; Requested/Deadline and original diagnostic payload controls pass. |
| FND-003 | fixed | Original CallFailure and facade ClauseRefusal::Fault clear boundary credit; complete reached/not-reached report controls exercise safety and constraint faults. |

## Final disposition verdict

PASS. Every finding is fixed within the original PR; no new finding was found in the fixes. The source is mergeable subject to the parent-owned final integration gate. Focused fixes pass 10 integration controls plus the conversion unit; installed four-control authored-clause lane passes in 1.94s. Final fmt and all-target/all-feature Clippy logs pass. Reviewer ran no Cargo or Kani and did not edit source.

## Merge synchronization

This PR700 artifact was remapped from SR-202610310145 to SR-202610310153 because main PR699 independently introduced the prior id. Original findings and round-1 dispositions above are unchanged. At rebased head 8054ccc3c5c666dd8a5e142aa01193a372ac5009 against main 0989d178, the previously dispositioned constrained generation, identity, execution, terminal and authored-clause test source is byte-identical to 043d398c. The prior PASS disposition therefore applies unchanged to this source. QSL dependency synchronization and the parent-owned rerun gates remain pending qualification; the stopped earlier aggregate gate is not a pass. This is an id correction, not a second broad review.

## New findings (disposition pass 2)

The first completed full integration gate at 596977bf reported 772 passed, 3 failed, 35 ignored in 1470.93s. Two PR regressions were measured and fixed; the third is the explicitly allowed unchanged upstream TC-025 intake baseline.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | The original constrained symbolic metadata introduced expect on the production generation/analysis path, violating NFR-005-AC-1 and failing tc_042_ac1_src_holds_no_panic_token in the first full integration gate. | src/kani/run/harness.rs:106-107 at 596977bf |
| FND-005 | medium | The existing admitted inline Boolean fixture and consumer were not migrated with the Boolean literal/direct-oracle ABI change, so admitted_inline_clause_keeps_the_clause_identity failed with UnsupportedCondition before retaining its identity and compiling the generated oracle. | tests/it/kani_obligations_state_frame.rs:264-280 at 596977bf |

## Dispositions — round 2

| FND | outcome | reason |
| --- | --- | --- |
| FND-004 | fixed | d55a5a2d lowers a checked u32 operation choice once and uses that same choice in renderer and metadata; metadata uses infallible i64::from. All 7 no-generation-panic controls pass. |
| FND-005 | fixed | d55a5a2d uses canonical JSON Boolean in the two local fixture sites and direct bool ABI for this zero-operation oracle. Identity/provenance/empty-input/source-map assertions remain; native -Dwarnings compilation/execution passes. |

## Gate-fix disposition verdict

PASS on the narrowly inspected d55a5a2d7532ed188a6912246642ed1be5eaa349 changes; all five code findings and the earlier gap finding have fixed dispositions. No new finding in the fixes. Source is mergeable subject to the parent-owned final integration rerun.

The first gate is not green: its remaining `tc_025_model_member_types_without_i64_ranges_remain_unranged` failure occurs at QSL ModelIntake before IR/CG: UnsupportedConstruct, `quire.meaning.model.value-type/v1:profile` has no reader. Its native_twin.rs source is unchanged against 0989d178 and the QSL lock is the same e9e7cdfc; this is the user-authorized baseline, retained explicitly rather than counted as passing.

Read qualification-logs/titan-ir1157/{full-it-first-complete.log,no-panic-fix.log,inline-fix.log,gate-fixes-clippy.log,gate-fixes-focused.log,gate-fixes-installed.log}: no-panic 7 pass; inline 1 pass; focused integration 10 pass; Clippy passes; installed four-control lane passes in 1.93s with ConstraintReached, BoundedSuccess and actual initial/later Falsified playback. Reviewer ran no Cargo and edited no source.
