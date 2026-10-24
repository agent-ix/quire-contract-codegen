---
id: SR-16280
title: IR-968 CG 552 code and Rust review
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; CG #552; src/lib.rs, src/replay/composite_builder.rs, src/replay/function.rs, tests/it/composite_original_eq.rs, tests/it/skeleton_spine.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-024", type: references }
---

## Summary

Independent diff-scoped code and Rust review of CG #552 at `f3016e97b03439cd0e39e6f6922ce10139a35c0d` on stacked base `ac6e3c92e55acb73c84cc02a0b46e587b81093da`. The diff changes function and original operator-parity replay request construction to admit QSL packets and bind target, occurrence, package, source arm, and obligation before producing a wire. I examined all five changed paths, public API shape, call sites, test assertions, and merged FR-024-AC-43/44/46 intent.

## Verdict

**FAIL** — the public Kani-obligation replay path can send a replay request for a different selected function than the supplied proving-run identity because its packet is completed from the request package after no package/identity binding check.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | `replay_counterexample_through` accepts a `KaniObligationIdentity` for a different function when its argument names bind in `package`. It recomputes the obligation and builds every packet claim member from `package.site`, so the new packet comparison necessarily passes. A playback for function `g` can therefore be replayed against function `f` and reported under `f` rather than refused before request construction. The sibling `ReplayHarnessIdentity` path performs `matches_package`, showing the missing boundary check. | src/replay/function.rs:1332; src/replay/function.rs:1354; src/replay/function.rs:1364; src/replay/function.rs:1263; src/kani/identity.rs:115 |

## Review evidence

The packet builder in `src/replay/function.rs:883` takes subject and occurrence but the `KaniObligationIdentity` route at lines 1364–1369 passes the selected package's values, not independent proving-run values. `obligation_identity` at lines 851–879 derives the function and declaration from the package and joins only the identity's kind and argument names. No use of `identity.contract_symbol` or `identity.subject_path` appears in this route. Thus a cross-function identity with the same argument names can reach `execute` under the other package. `ReplayHarnessIdentity::matches_package` at lines 67–72 and 1263–1266 guards the separate corpus route.

No vendored source, unsafe code, dependency change, workflow narrowing, or changed spec file appears in this PR. `git diff --check` passed. The author reports focused function-packet, composite and skeleton checks plus Clippy and fmt; I did not run the full gates, which belong on a reviewed final head.

## Examined scope and bindings

```yaml
scope:
  - id: FR-024-AC-43
    path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
    role: examined
    excerpt: "A function-contract adapter supplies a completed `WitnessPacket<NoPayload>` carrying its decoded witness and retained proving-run members. CG admits it through `WitnessEnvelope::reconstruct` before request construction and requires its `Witness` source arm."
  - id: FR-024-AC-44
    path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
    role: examined
    excerpt: "An input-arm parity claim producer supplies a completed `WitnessPacket<NoPayload>` before its request builder. CG admits it through `WitnessEnvelope::reconstruct` and requires its `Input` source arm."
  - id: FR-024-AC-46
    path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
    role: examined
    excerpt: "Before constructing a function-contract, operator-parity or value-parity request, CG compares the admitted packet's `function` target, complete occurrence key, subject `clause_node`, `package_id`, source arm and opaque obligation identity with the independently selected claim from the admitted package and proving-run identity or original checked parity claim, never from that packet."
```

## Same-reviewer disposition at a8fe44c3

**FND-001 resolved for the replay boundary; final verdict PASS.** The new `KaniProofBinding` retains the full `KaniObligationIdentity` and the selected QSL package, source, function, and declaration site when `with_kani_identity` is called. `ReplayPackage::request` refuses a different retained identity with `ProvingRunMismatch`; `replay_counterexample_through` refuses it as `PackageIdentity` before decoding or invoking the executor. The new `tc_035_function_replay_refuses_another_proving_run_identity_before_execution` changes the contract symbol and subject path while preserving argument names, checks both refusal routes, and asserts zero executor calls. I inspected the exact committed delta `f3016e97..a8fe44c3`; `git diff --check` passed.

The hand-mirrored QSL twin's initial pairing with the Kani proof is explicitly a caller responsibility in the new API documentation. That declaration does not independently prove semantic equivalence of the twin and proof. The disposition credits the enforced replay-time binding, not such a proof. The author reports focused skeleton 31/31, composite 19/19, function packet 1/1, and Kani stdout 2/2 plus fmt, diff, and scoped Clippy passes; aggregate final-head gates remain with the author.
