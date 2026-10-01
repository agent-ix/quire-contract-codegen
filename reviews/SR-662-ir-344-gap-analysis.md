---
id: "SR-662"
title: "CG PR 215 gap analysis: AD-004 migration plan against the code and FR-015"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@bfaaa849a100aee4a49959554b4607f0410227b1; spec/assurance/AD-004-cg-crate-layout.md; measured against src/ and tests/it/ at origin/main 2fad745"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
---

# SR-662: CG PR 215 gap analysis

## Summary

Ticket: IR-344. PR: agent-ix/quire-contract-codegen#215 at bfaaa84.

The PR is spec-only, so this gap analysis asks whether AD-004's migration plan covers what
the code at the base actually does and what the cited requirements need. It also checks
whether any step drops a requirement, reopens a vacuity gap (IR-464) or keeps a pin or digest
that CLAUDE.md forbids. Plan completion: not assessed (no plan supplied).

Checked clean: no step deletes a requirement or rewrites a criterion (FR-015-AC-22/AC-25
"stay verbatim"; rows go planned or unbacked). The arithmetic control (L-5) is required at
every step, and its package comes through QSL's facade, never a copied fixture. 4f waits on 4e
and on the QSL move of the quire-integration exemplars. GitHub code search confirms
quire-integration's `tests/support/mod.rs` calls `generate_kani_bundle`. `RUNTIME_REVISION`
is deleted, not respelled. The report path decision (question c) matches the runner's
existing `CARGO_TARGET_DIR` ownership.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The plan has no step that gives the precondition, postcondition and invariant (contract) families a V2 input. Yet 4a, 4e, 4f and L-5 all rest on the claim that "V1 has no consumer once the contract family exists". At the base the contract family is the V1 arm. `lower_clause` takes `&BoundClause` (`kani_obligations.rs:1069-1092`, through `generate_boolean_oracle`). `render_precondition` and `render_contract` take that `LoweredClause` (`:2180`, `:2204`). `ObligationKind::Precondition`/`Postcondition` come only from V1 `ClauseKind` (`:1062-1063`). The only V2 arm, `ScalarClaim`, reaches `render_scalar` alone. `generate_kani_bundle` is V1 too (`KaniRequest` takes `TypedExpression`, `kani.rs:156-176`). So 4f, "delete the `BoundClause` arm", deletes the contract family that 4e made the public entry and that the QSL exemplars will have moved onto. 4a cannot put the control on a V2 FR-015 contract path that does not exist, so L-5 cannot hold "at every step". Needed: a step before 4e that adds a V2 contract arm (over `CheckedPackageV2`), with the criteria it needs, or an honest statement that 4f waits for it | spec/assurance/AD-004-cg-crate-layout.md:282-292, :337-339, :455-469 |
| FND-002 | medium | Step 4f retires "`ProofDependency*`, validation" with `generate_kani_bundle` (map row). The corpus survives until 4g, which is gated on QSL-353, and it imports `ProofDependencyEdge`, `ProofDependencyKind`, `ProofDependencyRequest`, `ProofReadiness`, `normalize_dependencies` and `dependency_readiness` from `kani` (`bounded_kani_corpus.rs:19-22`, `:442`, `:598`). It also publicly exports `CorpusProofDependencyGraph` and `CORPUS_PROOF_GRAPH_SCHEMA`. Those request and readiness types are also the FR-015 census input that AC-22/AC-25 need. The AD should retire only `ProofDependencyGraph` and the V1 bundle at 4f, and name where the census types live (for example `kani/identity.rs`) | spec/assurance/AD-004-cg-crate-layout.md:200, :463-469, :529 |
| FND-003 | medium | "Its constructor refuses an empty cover list ... That closes IR-464 for every family at once" overclaims. A non-empty cover list is not a non-vacuity check. IR-464 asks for a cover "reached only when the property's precondition is satisfiable". A `kani::cover!(true, ..)` placed before the assumptions would satisfy L-4 and still be vacuous. The AD states no render rule placing covers after every `kani::assume` and the subject call. It also does not say what cover the corpus family gets, though it has none today. Add the placement rule to `render.rs`, with a test that a harness whose assumptions are unsatisfiable reads as vacuous | spec/assurance/AD-004-cg-crate-layout.md:256-260, :417 |
| FND-004 | medium | The AD says "No tool, version or file digest is added" and lists the carried digests as `package_id` and source byte digests. It misses the tool digest CG already carries: `ReplayInputs::backend_manifest`, "The digest of the tool manifest of the backend that found the counterexample" (`spine_replay.rs:197-198`, public API). CLAUDE.md says to remove such a digest where it is found. Step 5 depends on QSL-351 with "the tool pin gone", but no step deletes CG's `backend_manifest` field and its plumbing when QSL drops it | spec/assurance/AD-004-cg-crate-layout.md:353-358, :474-477 |
| FND-005 | low | `SourceProbe` and `SourceRegion` (`oracle.rs`, imported by `vacuity.rs:7` and `bound_coverage.rs`) are named only as "source regions" in the `oracle` split row, with no target file in `core/`. The target tree lists no file for them | spec/assurance/AD-004-cg-crate-layout.md:126-133, :186 |

## Verdict

The plan keeps every requirement and the QSL arithmetic control, and its ordering of 4e/4f
behind the QSL exemplar move is right. But it cannot run as written. The contract family it
keeps alive is the V1 arm it deletes (FND-001). The census types it retires are used by the
corpus that outlives them (FND-002). The cover rule is weaker than IR-464 asks (FND-003). And
one existing tool digest is left without a removal step (FND-004). Not mergeable as an
approved AD until FND-001 to FND-004 are resolved.
