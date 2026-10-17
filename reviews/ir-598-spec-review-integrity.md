---
id: SR-12201
title: "Integrity review of IR-598 per-family corpus retirement (CG #423 delta)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; PR #423 delta f22c59d9..1163a768 (IR-598): spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/assurance/AD-001-codegen-architecture.md, spec/assurance/AD-004-cg-crate-layout.md, spec/core/functional/interface-001-codegen-api.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/spec.md, spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md, spec/kani/matrix/tests.md"
review_set: subset
date: 2026-10-17
---

## Summary

Ticket: IR-598. Integrity and completeness check of the deletion lists in FR-015-AC-109/110/111 against the corpus dependency closure on CG origin/main 0e0f92e (src/lib.rs:45-60, lower/mod.rs, corpus imports, replay/function.rs, census_validation.rs). The per-family arm lists match the code. The final slice's list leaves out files and exports that become dead once all three arms are gone, and the bounded-profile classifier has an escape clause that the code on main does not need.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The AC-111 and AC-109 deletion lists leave residue that the 'search for each deleted name' check cannot catch. AC-111 omits census_validation.rs, whose `validate_dependencies` has only the corpus as caller (bounded_kani_corpus.rs:32) and which AD-004 4f says 4g deletes; lower/mod.rs (`FamilyLoweringError`, `admit_family`, `refusal`); schemas/kani-corpus-proof-graph-v1.schema.json and `CORPUS_PROOF_GRAPH_SCHEMA`; and the exports `BoundedCorpusArtifacts`, `BoundedCorpusFamily`, `CorpusProofDependencyGraph` and `EmittedCorpusIdentities` (lib.rs:45-48). AC-109 omits `BoundedCorpusFamily::DefinednessArithmetic` and does not name the replay helpers (`ReplayHarnessIdentity`, `ReplayPackage::harness_identity`, `replay_harness_counterexample(_through)`). | FR-015-AC-111, FR-015-AC-109, AD-004 (spec/kani/functional/FR-015-bounded-kani-obligations.md:802) |
| FND-002 | medium | The phrase 'unless a V2 path still uses it' for the bounded-profile classifier makes this deletion unfalsifiable, and interface-001 contradicts it ('deleted by the final retirement slice'). On main, nothing in src/ uses `classify_bounded_kani_profile` or `BoundedKaniProfile` except the lib.rs:53-54 re-export, and the corpus does not import it. Delete it unconditionally, in any slice; it belongs to no family. | FR-015-AC-111, interface-001 (spec/kani/functional/FR-015-bounded-kani-obligations.md:802) |
| FND-003 | low | FR-024 says the corpus path 'is removed' in the present tense while it is live, and AD-001 says its replay 'is live today'. The spec.md `kani/generate/lower/{...}` row (spec.md:114) still says 'pending the AD-004 V2 migration' and was not given per-slice retirement like the corpus row next to it. | FR-024, AD-001, spec.md (spec/replay/functional/FR-024-counterexample-envelope-intake.md:416-418) |

## Verdict

The arithmetic and collection arm lists match the code: lower/definedness_arithmetic.rs and bounded_collections.rs have no caller outside the corpus and lib.rs. The interface-001 status edits are consistent with the slices, except for the classifier. Fix the AC-111 residue list and remove the conditional.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | Introduced by the fix round. AC-111 and AD-004 4g now delete all of `kani/generate/census_validation.rs`, but that file also holds the public `KaniDiagnostic` and `KaniErrorCode` (lib.rs:135 re-export). `v1_bundle.rs` uses them (:23-25, :146, :252 and on), and it stays live until step 4f. AC-111 is not gated on 4f, so its deletion list can contradict live code. Either gate AC-111 on step 4f, or delete only `validate_dependencies` at AC-111 and remove the file once 4f has landed, as AD-004 4f (:1008-1016) already describes. | FR-015-AC-111, AD-004 (spec/kani/functional/FR-015-bounded-kani-obligations.md:802) |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Verified at 455b1b1 (round 1) |
| FND-002 | fixed | Verified at 455b1b1 (round 1) |
| FND-003 | fixed | Verified at 455b1b1 (round 1) |
