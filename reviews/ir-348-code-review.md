---
id: SR-601001
title: "IR-348 PR #601 code-review and rust-review (V1 Kani generator and BoundClause retirement)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; PR #601 head 64992f5be8aebb7d036fba5491fa9ef61af70d29 against merge-base 46dbaccacbd96f55654fc4b331a9d42883e70a2c; Makefile, README.md, scripts/cg_gates.rs, scripts/kani_claims.json, src/kani/**, src/lib.rs, src/oracle/boolean_v1.rs, src/routed/capability.rs, tests/common/**, tests/it/**; IR-348"
review_set: subset
---

## Summary

Ticket: IR-348. This is a static review of the 37-file PR diff, read with `rust-style` in mind.
It also covers the callers of every removed item. No cargo, make, test or Clippy run was made; the
brief says focused compile, Clippy and tests passed on this head.

The removed V1 surface has no live callers. `generate_kani_bundle`, `v1_bundle.rs`,
`precondition.rs`, `contract.rs`, the V1 `clause.rs` lowering, `ObligationItem::BoundClause`,
`KaniForm::V1BoundClause`, `KaniIntegerBounds::from_model`, `OracleShape::KaniBundle`,
`measure_named_boolean_oracle` and `withdraw_fixture` are all gone, and a repo-wide grep finds no
code reference to them. The only remaining `BoundClause` uses are the separately owned strategy
and coverage readers (`src/strategy/bound/generation.rs`, `src/evidence/bound_coverage.rs`). The PR
adds no file, so nothing was copied from another repository, and it adds no compatibility shim.
The V2 scalar real-Kani test moved, renamed, into the `kani_generation` lane and still runs there,
so the lane did not lose it.

Merge blocker, not a code finding: GitHub reports the PR `CONFLICTING` (`mergeStateStatus=DIRTY`).
`git merge-tree origin/main HEAD` conflicts in `src/kani/generate/negotiate.rs`. Resolving it is
the lead's job after #593 merges. This review covers the frozen head only.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | When every claim row is held, the claim test returns before `claim_census`, and `cg_gates.rs::claim_counts` never reads the merge-base declaration. Nothing then compares the declaration with the merge base, which FR-041 requires for the hold. Once main carries a non-held claim, a branch that deletes that row and keeps only the held row passes both the claim test and the gate. | tests/it/kani_generation_claims.rs:519-532; scripts/cg_gates.rs:243-281 |
| FND-002 | medium | `KaniErrorCode` keeps six variants that nothing produces any more: `UnsupportedBinding`, `ClauseGenerationFailed`, `InvalidUnwind`, `InvalidGeneratedSyntax`, `SerializationFailed` and `ResourceLimitExceeded`. The `KaniDiagnostic.generation_code` and `source_span` fields are bundle-only too. The corpus's `validate_dependencies` emits only `InvalidIdentity` and `InvalidDependency`. AD-004 step 4f says 4f deletes every error variant the bundle alone used. | src/kani/generate/census_validation.rs:16-33, 54-71 |
| FND-003 | medium | The public `KaniObligationHarness` and `KaniExecutableHarness::Contract` (documented as "A V1 frozen-clause contract harness") remain, with their `From` impl and execution arms. Nothing in `src` or `tests` constructs a `KaniObligationHarness` any more, so this is a V1 execution entry with no producer, left in place despite the no-retention ruling. | src/kani/run/harness.rs:20-35; src/kani/identity.rs:146-155; src/lib.rs:239 |
| FND-004 | low | The `CorpusProofDependencyGraph` rustdoc now ends mid-sentence: "See [`CORPUS_PROOF_GRAPH_SCHEMA`] for why this is a distinct envelope from". The PR also deleted the rationale that sentence points to from the schema constant's doc. | src/kani/generate/corpus/bounded_kani_corpus.rs:114-119, 39-43 |
| FND-005 | low | The `assign_names` doc still describes naming "clause oracle functions (any may be embedded beside another as an assumed precondition), then the module and harness names of every clause and scalar obligation". Only scalar modules are named now, and `NameKey` has a single variant. | src/kani/generate/negotiate.rs:713-723 |
| FND-006 | low | `generate/mod.rs` still labels `census_validation` "interim, see the file header", but the PR rewrote that header and it no longer says interim. | src/kani/generate/mod.rs:4 |

## Verdict

Changes requested. FND-001 is a real hole in the FR-041 hold. FND-002 and FND-003 leave V1-only
public surface the ruling says to delete. The other changes look correct:

- The retirement is complete and has no live callers.
- `Outcome` and `ItemState` are simplified correctly now that they no longer borrow `BoundPackage`.
- `boolean_v1` keeps a single retained-source path, and the `PlainBool` arithmetic refusal
  remains, now with an explicit `Numeric` arm.
- The `cover_last`, `layout`, `kani_gate` and `main.rs` tables match the deleted files.
- `skeleton_spine` now uses a direct test-only `KaniObligationIdentity`, and nothing claims it is
  a generator.
- The hold literals in `kani_claims.json`, `cg_gates.rs` and the claim test match FR-041 exactly.

The merge conflict in `negotiate.rs` blocks merge.

## Coverage

```yaml
scope:
- {id: src/kani/generate/v1_bundle.rs, path: src/kani/generate/v1_bundle.rs, role: examined, excerpt: "deleted (742 lines); no remaining reference in src/ or tests/"}
- {id: src/kani/generate/negotiate.rs, path: src/kani/generate/negotiate.rs, role: examined, excerpt: "struct ItemState { kind, subject, identity, outcome: Outcome }; reject_duplicates_and_mixtures(&mut states); assign_names(request.items, &mut states);"}
- {id: src/kani/generate/outcome.rs, path: src/kani/generate/outcome.rs, role: examined, excerpt: "BoundClause item/subject, V1 UnsupportedObligation and InvalidObligationItem variants and Emitted.harnesses removed"}
- {id: src/oracle/boolean_v1.rs, path: src/oracle/boolean_v1.rs, role: examined, excerpt: "ExpressionKind::Numeric { .. } => Err(unsupported_node(self.request, expression, \"integer arithmetic requires the native outcome oracle\"))"}
- {id: src/routed/capability.rs, path: src/routed/capability.rs, role: examined, excerpt: "KaniForm::V1BoundClause and KaniFormName::V1BoundClause removed"}
- {id: scripts/cg_gates.rs, path: scripts/cg_gates.rs, role: examined, excerpt: "let (active_claims, _held_claims) = claim_counts()?; ... && (active_claims == 0 || proof_reports_present())"}
- {id: tests/it/kani_generation_claims.rs, path: tests/it/kani_generation_claims.rs, role: examined, excerpt: "if held == declaration.modules.len() { ... println!(\"kani-claims: active=0 held={held}\"); return; }"}
- {id: tests/it/skeleton_spine.rs, path: tests/it/skeleton_spine.rs, role: examined, excerpt: "fn spine_identity() -> KaniObligationIdentity { identity_fixture(ObligationKind::Postcondition, \"balance-never-grows\", 20) }"}
- {id: tests/it/kani_stdout_capture.rs, path: tests/it/kani_stdout_capture.rs, role: examined, excerpt: "V1 tc_026_installed_kani_falsification_is_bound_to_process_stdout removed; observer tests retained"}
- {id: tests/it/kani_obligations.rs, path: tests/it/kani_obligations.rs, role: examined, excerpt: "pub(crate) fn verify_routed_scalar_harness_real_kani() (moved from tc_027_a_routed_scalar_harness_verifies)"}
- {id: tests/it/kani_generation.rs, path: tests/it/kani_generation.rs, role: examined, excerpt: "pub(crate) fn guard_sources() -> Vec<(&'static str, String)> { crate::kani_obligations::guard_sources() }"}
- {id: tests/it/cover_last.rs, path: tests/it/cover_last.rs, role: examined, excerpt: "const FAMILIES: [(&str, &str); 7] = [(\"scalar\", ...), (\"v2 precondition\", \"kani/generate/render.rs\"), ...]"}
- {id: Makefile, path: Makefile, role: examined, excerpt: "KANI_FILTERS := kani_obligations kani_obligations_symbolic_domains skeleton_spine kani_stdout_capture bounded_kani_corpus kani_generation kani_batching"}
- {id: src/kani/generate/census_validation.rs, path: src/kani/generate/census_validation.rs, role: examined, excerpt: "pub enum KaniErrorCode { InvalidIdentity, InvalidDependency, UnsupportedBinding, ClauseGenerationFailed, InvalidUnwind, InvalidGeneratedSyntax, SerializationFailed, ResourceLimitExceeded }"}
- {id: src/kani/run/harness.rs, path: src/kani/run/harness.rs, role: examined, excerpt: "/// A V1 frozen-clause contract harness (FR-015). Contract(&'a KaniObligationHarness),"}
- {id: AD-004:4f-census, path: spec/assurance/AD-004-cg-crate-layout.md, role: context_only, excerpt: "Step 4f deletes any helper and error variant used only by the bundle; later steps remove the remaining items when the corpus stops calling them."}
- {id: FR-041, path: spec/kani/functional/FR-041-claimed-module-proof-gate.md, role: context_only, excerpt: "The gate SHALL retain that exact row in the reviewed declaration and compare it with the merge-base declaration, even after the V1 module is absent from the compiled build."}
```
