---
id: "SR-3160"
title: "IR-635 PR 326 code review: original admitted structural.eq constructor"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen PR 326, branch code/ir-635-original-eq-builder (frozen head recorded in the IR-635 Linear review marker, not here); Cargo.lock, src/lib.rs, src/replay/mod.rs, src/replay/composite_builder.rs, tests/it/composite_original_eq.rs, tests/it/main.rs"
---

# SR-3160: IR-635 PR 326 code review

## Summary

Ticket: IR-635. Code review with the Rust lane (rust-review idiom and smell checklist) folded
in. Static review only: no cargo, test, clippy or Kani run by the reviewer. The author reports
full make ci and real Kani passing on the reviewed head; that is the author's claim and was not
re-measured here.

Examined:

- `Cargo.lock`: only the two IR-repository entries (`quire-contract-ir`, `quire-contract-model`)
  move, both to one IR revision. Checked against GitHub: that revision is identical to current IR
  `main` (compare: identical, 0 ahead, 0 behind) and contains the IR-651
  `composite_application_operands` accessor merge (accessor commit is 2 behind it, 0 ahead). One
  IR source line remains in the lock, so the one-copy rule holds. QSL and every other revision are
  unchanged. The refresh is minimal and truthful.
- `src/replay/composite_builder.rs`: `OriginalCompositeEqContext::new`, `request`,
  `check_recompiled_context`, `check_function_membership`, `OriginalCompositeEqRequest::settle_verified`,
  `OriginalCompositeEqReport::settlement`, `CompositeBuildError`.
- `src/lib.rs`, `src/replay/mod.rs`: re-export and module wiring.
- `tests/it/composite_original_eq.rs`: eight controls; `tests/it/main.rs`: module registration.
- Upstream context read only (not changed): QSL `settle_verified_shadow`, `prepare`,
  `identity_tie` and `composite_site::locate`; IR `composite_application_operands`.

What is sound:

- Authenticity. Operands come only from IR's public accessor over an admitted
  `CheckedPackageV2`; the original package is then authenticated by recompiling the retained
  source under the real request's decoded stage limits, re-reading through IR, and requiring
  both the semantic package id and full package equality before any request is returned. No node,
  occurrence or package id is invented; `WireNodeId` is parsed from IR's digest and refuses on
  failure. O-09 goes through QSL's owning `parity_obligation`; bound ordering/duplicate refusal
  goes through QSL `BoundEntries::new`. QSL independently recomputes O-09 in `identity_tie`.
- The report path retains the sent `CompositeIdentity` before invoking the public facade and
  feeds the genuine report to the existing IR-666 converter; no report is fabricated.
- Imported contexts refuse with typed `ImportedContextUnsupported` in `new`, before any
  recompilation or invocation, and after the dependency/lock comparison.
- No `unwrap`, `expect`, `panic!`, indexing or `unsafe` in library code; no compatibility layer,
  vendored file, depth cap or new hash/pin record. Tests carry `Trace:` tags that resolve.
- Library code is SHA/hex free (a seven-plus hex grep over the added non-lock lines is empty).

## Verdict

CONDITIONAL. The constructor is authentic and the pin refresh is clean, but two medium issues
should be fixed in this PR before merge: the function/occurrence membership precheck is a second
CG-local implementation of a rule QSL owns (FND-001), and several typed refusal branches the spec
credits have no control, so their mutants survive (FND-002). Three low findings are idiom and
precedence notes. Not merge-ready until FND-001 and FND-002 are fixed or dispositioned; gates must
be re-run on the final head before merge.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `check_function_membership` re-derives whether a node/occurrence belongs to the selected function with a CG-local heuristic: a graph-closure walk over `dependencies` that stops at nested `function` nodes, plus source-map region containment against the function's regions. QSL owns this rule in `composite_site::locate` (`in_body` over the checked body plus the occurrence's `Body { function }` origin), and IR exposes no membership accessor. Two implementations of one rule will diverge: an occurrence QSL places in the body but whose source region is not textually contained in the function's region (or lies in another source unit) is falsely refused pre-invocation, and one QSL rejects can pass the precheck. Either rely on QSL's binding-checked `Refused` report for membership (FR-033-AC-1 permits a QSL common-step refusal) or obtain an owning IR accessor; do not keep a third definition in CG. | src/replay/composite_builder.rs:330-387 |
| FND-002 | medium | Typed refusal branches without a control, so the mutant deleting each survives: (a) `PackageMismatch` at all three sites (non-PackageSemanticV2 domain, hex parse, recompiled package id differs); the test named for "another package" actually asserts `SourceMismatch`; (b) `ContextMismatch` from the dependency comparison, reachable by retaining an extra dependency whose lock has empty `dependency_selections`, and from full recompiled-package inequality; (c) `OccurrenceOutsideFunction`, the enclosing-function-substitution guard FR-033-AC-1 names, likely reachable with a content-addressed closed literal/literal Eq node shared by two functions and the other function's occurrence; (d) the literal arm's empty Bounds: the literal control passes `&[]` harness bounds, so a mutant applying the parameter filter to literals is identical; (e) the claim's Node-only harness-bound filter (Population keys dropped) has no control. | src/replay/composite_builder.rs:155-184, 238-243, 266-270, 307-326, 363-385; tests/it/composite_original_eq.rs:326-374 |
| FND-003 | low | Error precedence runs derivation before authentication: `request` calls the accessor and the membership walk on the caller-supplied package before `check_recompiled_context` authenticates it, so an inauthentic context surfaces as `Operands` or `NodeOutsideFunction` rather than `ContextMismatch`. No wrong success is possible (the request is returned only after the equality check), but the reported cause depends on the node chosen. | src/replay/composite_builder.rs:228-232, 281 |
| FND-004 | low | `entry.regions.iter().all(..)` is vacuously true for a source-map entry with no regions, so such an occurrence passes the `OccurrenceOutsideFunction` precheck for any function. QSL would still refuse later, so this degrades a CG precheck into a QSL report rather than producing a wrong value. | src/replay/composite_builder.rs:371-381 |
| FND-005 | low | Idiom: public `OriginalCompositeEqRequest` has no `Debug` while every sibling public type derives it; `Display` for `PackageRead` and `PackageReadLimit` prints the cause with `{:?}` instead of a message; the membership walk does a linear `find` over all graph nodes per visited id (quadratic, unmetered, while the IR projection it follows is work-metered). | src/replay/composite_builder.rs:96-97, 346-352, 406-413 |
