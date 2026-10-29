---
id: SR-2026102762
title: "CG PR 607 combined gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; src/strategy/v2_selected_claim.rs, src/strategy/v2_comparison.rs, src/strategy/v2_population.rs, src/strategy/v2_numeric.rs, src/strategy/bound/{population,census}.rs, src/oracle/integer_comparison.rs, tests/it/v2_*.rs; FR-011-AC-13..18, FR-004-AC-26"
review_set: subset
---

## Summary

The computed Quoin matrix at frozen PR #607 head was checked against the changed tests and code. FR-011-AC-13, AC-14, AC-15 and FR-004-AC-26 have direct test binders. FR-011-AC-16 and AC-17 are untagged despite this PR implementing their runner and refusal paths. FR-011-AC-18 is still Planned for later live producer binding; source/map bytes and a report alone are not campaign qualification.

## Scope

- examined: FR-011-AC-13: PLANNED (IR-1072). The public V2 selected-claim entry accepts an admitted `CheckedPackageV2` and a typed complete claim key (checked clause id, claim owner node id, authentic `claim` `CheckedOccurrence`) without a second caller-supplied owner; a matching key selects only that claim and its exact checked point. With the same clause and occurrence but a different claim owner, the entry returns a distinct typed V2 diagnostic retaining Contract IR `CheckedExecutionPointRefusal::MissingClaimOccurrence` and the supplied clause, owner, and occurrence, and publishes no artifact. The V1 `BoundPackage` entry and `StrategyDiagnostic` are unchanged.
- examined: FR-011-AC-14: PLANNED (IR-1070). For selected authentic `claim` occurrences of distinct V2 invariant clause nodes whose condition and anchor are equal but whose accepted points are respectively `initialization{name: "startup"}` and `handler{name: "tick"}`, each constructed verdict carries the exact point of its selected node through the FR-058 accessor and injective runtime rendering. The nodes and otherwise identical packages have distinct identities; changing only the accepted handler name changes its rederived node and package identities and runtime point. Precondition and postcondition claims preserve their accepted `pre` and `post` points, including their operation names (QSpec FR-322-AC-131, FR-341-AC-14, FR-342-AC-10; IR FR-058-AC-9). Two authentic claims of one pointed node return its one identical point; no claim occurrence supplies a second point.
- examined: FR-011-AC-15: PLANNED (IR-1070). An absent checked clause returns `StrategyErrorCode::UnknownClause`, a non-state clause returns `StrategyErrorCode::UnsupportedClauseKind`, and a wrong-role, wrong-ordinal, or other-node occurrence returns `StrategyErrorCode::MissingClaimOccurrence`, each before artifact publication and without substituting another claim or node's point. An authentic claim of a separate V2 clause node whose optional point is absent returns `StrategyErrorCode::MissingExecutionPoint` without borrowing a pointed sibling node's point. A renderer mutant changing only the selected point returns `StrategyErrorCode::ExecutionPointMismatch` before publication. Two authentic claims of one pointed node use the same point; clause kind and anchor never synthesize a missing point.
- examined: FR-011-AC-16: PLANNED (IR-1124; depends on IR-1072 and FR-008). A public V2 strategy request over an admitted checked package and one authentic selected claim produces a complete source, map, census and numeric runner bundle whose selected clause, owner, occurrence, package revision, comparison and runner identities agree exactly. The runner executes the FR-009 constructive population and FR-010 census with the selected V2 oracle; source inspection finds no V1 `BoundPackage`, V1 `ClauseRef`, `generate_bound_strategy` call or V1-to-V2 adapter in this V2 path, while the selected V2 checked clause key is retained.
- examined: FR-011-AC-17: PLANNED (IR-1124). Independently injected unknown clause, unsupported kind, wrong owner/role/ordinal, missing accepted point, FR-008 comparison refusal, empty population, identity mismatch, point-render mutant, invalid generated syntax and resource overflow each return a typed diagnostic with the selected key and available structural locus, publish zero artifacts and start no campaign. Pairwise overlapping defects return the first cause in Behavior order; a valid sibling still publishes its complete bundle.
- context_only: FR-011-AC-18: PLANNED (IR-1124; FR-004 live binding remains separate). A campaign run from the emitted V2 runner retains the selected claim and package identity with its own source/map and result for same-invocation producer binding. Published bytes or a detached/copied result alone yield no authenticated campaign, measured coverage or proof credit. A refused selection yields no campaign evidence, including when another claim of the same package succeeds.
- examined: FR-004-AC-26: PLANNED (IR-1070). For each selected executable V2 clause, the generator uses the exact accepted point of that point-qualified clause node and authentic claim through IR FR-058; separate nodes with equal condition and anchor but different points produce distinct rendered points when selected separately, while two authentic claims of one pointed node produce the same point. The analyzer retains one expected population entry per executable clause node and grants no campaign evidence for a refused selection. Wrong-role, wrong-ordinal, other-node, absent-point-node, and renderer-mutant selections each receive the corresponding typed no-artifact refusal of FR-011-AC-15, with no sibling substitution or inferred point.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-011-AC-16 has no direct acceptance-criterion test binder in the computed matrix | tests/it/v2_numeric_strategy.rs:37 |
| FND-002 | high | FR-011-AC-17 has no direct acceptance-criterion test binder and its prior-discard identity case is untested | tests/it/v2_numeric_strategy.rs:289 |

## Coverage

- FR-011-AC-13: tagged; typed key and exact point/refusal assertions.
- FR-011-AC-14: tagged; distinct point-qualified nodes, pre/post variants, and UTF-8 byte length.
- FR-011-AC-15: tagged; wrong role/ordinal/owner, unpointed sibling, and renderer mutant controls.
- FR-011-AC-16: untagged; runner/source/map integration test uses only TC-020.
- FR-011-AC-17: untagged; refusal test uses only TC-020, and no runner test exercises prior discarded report plus identity mismatch.
- FR-011-AC-18: untagged and Planned for a later live same-invocation binding; no credit claimed.
- FR-004-AC-26: tagged for the point and refusal portions; existing V2 LLVM analyzer retains clause-node population, but campaign qualification has no live V2 binding seam in this PR.
- Reverse gap: new source and tests map to FR-011/FR-004 scope; no unowned new behavior identified.
- Plan completion: not assessed
- Runtime evidence: unavailable under the planner's build-lane hold.

## Verdict

FAIL for static traceability of the two acceptance criteria this PR implements. The computed matrix has no test binders for AC-16 or AC-17. Runtime evidence and AC-18 live binding remain pending.
