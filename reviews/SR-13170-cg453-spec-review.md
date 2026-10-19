---
id: "SR-13170"
title: "CG PR 453 spec review (base): FR-019-AC-45 fidelity to QSpec FR-290, FR-272 and FR-331"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@bf69bb4f23a976e8a858ac16460f1c740ab491ae; spec/routed/functional/FR-019-capability-settlement.md, spec/routed/matrix/TC-030-capability-settlement.md, spec/routed/matrix/tests.md (diff origin/main...HEAD); sources read: quire-specification@6fd431e FR-290, FR-272, FR-331; quire-spec-language origin/main qsl-route/src/lib.rs; quire-driver origin/main spec/functional/FR-001, FR-005, src/registry.rs; CG src/routed/capability.rs, src/routed/generate.rs"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-030
    type: reviews
---

# SR-13170: CG PR 453 spec review (base)

## Summary

Ticket: IR-887 (parent IR-878). PR: agent-ix/quire-contract-codegen#453 at bf69bb4. Spec-only
diff: FR-019 gains a FR-272 reference, an Outputs bullet, three Behavior bullets, a trimmed
FR-019-AC-10 and a new FR-019-AC-45; TC-030 and `spec/routed/matrix/tests.md` gain AC-45.

Checked against the sources, not the PR prose:

- QSpec FR-290:248-274 and 311-317 (origin/main 6fd431e). Byte-equal RFC 8785 repeats are held
  once. Differing manifests refuse every registration of that identity, "once per distinct
  manifest", hold none of them, and never refuse another identity. Refusals are ordered by
  identity, then by the manifest's RFC 8785 bytes.
- QSpec FR-272:94. The `duplicate-backend` payload is "the repeated identity and the refused
  registration's manifest".
- QSpec FR-331:57. The manifest holds one descriptor per registered backend in the snapshot, and
  "Backend identities are unique".
- QSL `qsl_route::RegistrationRefusal { backend, manifest_digest, cause }`, with one refusal per
  distinct digest and equality over the whole typed descriptor, origin included.
- quire-driver FR-005-AC-4: "an unequal descriptor for an existing `BackendId`, including one
  with a different origin, returns the typed duplicate-backend refusal".

What holds:

- AC-10 keeps "Only an item settled `supported` routes." verbatim.
- The per-identity scope matches FR-290: byte-equal repeats are held once, the refusal is
  isolated to its identity, and other identities still settle.
- The refusal is a typed variant, not a string cause in `EnvelopeRefusal::InvalidCapability`.
- The PR body and the spec both say this resolves a conflict between two specifications.
- The author's FR-331 claim is confirmed. FR-331:57 ("one backend descriptor per registered
  backend in the registry snapshot ... Backend identities are unique") is consistent with
  FR-290:258-265, which leaves at most one held registration per identity in the snapshot. No
  QSpec edit is needed.

What does not hold is below.

## Verdict

CONDITIONAL. Not mergeable as-is. Four defects:

- FND-001 (high): the equality basis ignores CG's `ProviderOrigin`. That contradicts FR-019's own
  IR-629 item 6, QSL PV-1 and driver FR-005-AC-4.
- FND-002 (high): the payload and cardinality depart from FR-272:94 and FR-290:264 while reusing
  their code and cause.
- FND-003 (medium) and FND-004 (medium): the refusal has no stated return surface, and the
  disposition of an item that uses the refused identity is not stated. Without both, AC-45
  cannot be written as a failing test in tests-first PR #451.

FND-002 needs a planner decision. The binding ruling says "no manifest of the refused repeat is
kept (as FR-272)", but FR-272:94 requires the refused registration's manifest in the
`duplicate-backend` payload. The ruling most plausibly means "no descriptor of that identity is
held". Under that reading the payload should still carry the manifest, or a digest of it as QSL
does.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AC-45 and Behavior define equality as byte-equal RFC 8785 encodings of FR-331 manifest descriptors. CG's `BackendDescriptor` carries `ProviderOrigin`, which is not on the FR-331 wire (FR-019-AC-15). So two descriptors of one identity that differ only in origin count as equal: one is held, with an unstated origin, and no refusal is raised. This contradicts FR-019's own IR-629 item 6 (PV-1 withdraws linked and process descriptors that conflict on one BackendId), quire-driver FR-005-AC-4, and QSL `Registry::register`, which compares the whole typed descriptor. Define equality over the whole CG descriptor, origin, domains and bounds included, or over the FR-331 bytes plus origin | spec/routed/functional/FR-019-capability-settlement.md:126-133, spec/routed/functional/FR-019-capability-settlement.md:410-412, spec/routed/functional/FR-019-capability-settlement.md:426, src/routed/capability.rs:206-218 |
| FND-002 | high | The refusal reuses FR-272 `invalid_capability`/`duplicate-backend` but its payload is `{ identity: String }` only, and the spec says so ("at CG negotiation this refusal carries the identity only"). FR-272:94 requires the repeated identity and the refused registration's manifest for that cause. FR-290:264 and 316-317 require one refusal per distinct manifest, ordered by identity and then by manifest RFC 8785 bytes. AC-45 says only "refuse every registration of that identity" and "the same refused identities regardless of order", so the refusal count and order are unstated, and an identity-only payload cannot tell two refusals apart. The QSL analogue carries `manifest_digest`. Carry the manifest or its digest, one refusal per distinct manifest, in FR-290's order, or name a distinct CG cause. Needs planner confirmation of what the ruling's "no manifest kept" means | spec/routed/functional/FR-019-capability-settlement.md:135-143, spec/routed/functional/FR-019-capability-settlement.md:426 |
| FND-003 | medium | The spec names `RegistrationRefusal::DuplicateBackend { identity: String }` but not where it is returned. `negotiate_backend_provider` returns `Result<Vec<ItemSettlement>, EnvelopeRefusal>`, which has no slot for registration refusals beside settlements. AC-45 is not testable until the result shape is stated. The name also collides with `qsl_route::RegistrationRefusal`, a struct with a different shape that quire-driver already imports beside CG. Name the return shape, and rename the type or state how it relates to QSL's | spec/routed/functional/FR-019-capability-settlement.md:135-137, src/routed/capability.rs:1284-1286 |
| FND-004 | medium | AC-45 says the refused identity "route[s] no item through it" but not how an item whose `candidates` contain it, or that names it, settles. CG does not route (generate.rs:3-7), so that clause is not observable in CG. FR-290:271-273 makes such an identity unregistered: a candidate for no item, and the unknown-backend mark when named. At CG, a candidate missing from the effective manifest hits FR-290's `inconsistent-candidates` row. State the disposition and cause that AC-45 asserts for such an item | spec/routed/functional/FR-019-capability-settlement.md:426, src/routed/generate.rs:3-7 |
