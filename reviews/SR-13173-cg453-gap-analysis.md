---
id: "SR-13173"
title: "CG PR 453 gap analysis: AC-10 and AC-45 backing and the tests PR 451 needs"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@bf69bb4f23a976e8a858ac16460f1c740ab491ae; FR-019-AC-10, FR-019-AC-45 (spec/routed/functional/FR-019-capability-settlement.md, spec/routed/matrix/tests.md); src/routed/capability.rs, src/routed/generate.rs read for implementation status"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: reviews
---

# SR-13173: CG PR 453 gap analysis

## Summary

Ticket: IR-887. PR: agent-ix/quire-contract-codegen#453 at bf69bb4. The run was planless (plan
completion not assessed) and scoped to the two criteria the diff touches. Both are untagged and
🚧 Planned at the head. On main, `negotiate_backend_provider` makes no repeat check and
`descriptor()` takes the first entry whose identity matches (src/routed/capability.rs:1506-1513).
That is IR-887's known gap, which tests-first PR #451 is to close.

PR #451 as last reviewed (SR-13130) pins `tc_030_identical_repeated_descriptors_refuse_the_envelope`.
That test contradicts AC-45 (byte-equal repeats are held, not refused), so #451 must be reworked
to AC-45 once this PR settles.

Tests #451 needs for AC-45, once SR-13170 FND-001 to FND-004 are resolved:

1. Byte-equal adjacent repeat [A, A]: one descriptor, no refusal, and the item settles as with [A].
2. Byte-equal non-adjacent repeat [A, B, A]: the same, with B unaffected.
3. Differing adjacent repeat [A1, A2] and [A2, A1]: a typed DuplicateBackend for A in both
   orders, no A descriptor held, and the item over A settles with the cause AC-45 names
   (SR-13170 FND-004).
4. Differing non-adjacent repeat [A1, B, A2], in every permutation. This kills the adjacent-only
   (`windows(2)`) mutant.
5. Isolation. With [A1, B, A2], an item using B settles exactly as with [B] alone, in every
   order.
6. Two conflicting identities [A1, C1, A2, C2]: both refused, reported in bytewise identity order
   whatever the input order. This kills the `.max()` mutant.
7. Mixed repeat [A1, A1, A2]: refused. Under FR-290:264 there is one refusal per distinct
   manifest, so two refusals, not three (SR-13170 FND-002).
8. Origin-only difference: [A(Linked), A(Process)] with otherwise equal descriptors is refused
   (SR-13170 FND-001).
9. Identities that differ only in case are distinct. The existing test carries over.
10. Precedence. A foreign `capability_vocabulary` together with a repeat refuses the envelope as
    `unsupported-version`, with no registration refusal and no kind read (AC-7).
11. No retention. The returned result exposes no descriptor of the refused identity. This needs
    the return surface from SR-13170 FND-003.

## Verdict

CONDITIONAL. AC-45 is testable and can fail once SR-13170 FND-001 to FND-004 give it an
equality basis, a payload, a return surface and an item disposition. The finding below is the
remaining spec gap.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-019-AC-10 ("Only an item settled `supported` routes") and AC-45's "route no item through it" and "still settles and routes" make claims about routing. CG does not route: QSL `route` picks the backend (generate.rs:3-7), CG takes no `qsl-route` dependency (FR-019:377-378), and `RoutedGenerationItem::from_descriptor` and `generate_routed` take no settlement (generate.rs:79, 295). `RequiresBound` also carries a backend, so a backend in a disposition is no marker of routing either. No CG test can fail on these clauses as written, so this is a spec gap, not only a missing test. Restate them as something CG can observe (for example, only a `Supported` settlement yields a value FR-022 accepts), or move routing to QSL or the driver as a referenced precondition with Verification "Analysis" | spec/routed/functional/FR-019-capability-settlement.md:425-426, src/routed/generate.rs:3-7, src/routed/generate.rs:79-101, src/routed/generate.rs:295 |
