---
id: SR-16300
title: IR-983 CG 553 code and Rust review
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; CG #553; src/replay/function.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-024", type: references }
---

## Summary

Independent diff-scoped code and Rust review of CG #553 at `3b871fee8edb783c33bcc1bbd1c5b02f6c2aeff4` against stacked #552 base `0f29a8120af013cf6ed17b0c0ed1bdccea839e14`. The change adds an Input-arm value-parity packet producer and request path in `src/replay/function.rs`, with two tagged unit tests. I examined the complete diff, the selected-function call site, packet admission and request readback, the sibling original operator-parity claim derivation, and the merged FR-024-AC-44/46 and TC-035 step 30.

## Verdict

**FAIL** — the value-parity obligation is a caller-chosen digest supplied independently to the packet producer and request method; no original checked parity claim selects or verifies it.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | `value_parity_packet` mints a packet from any caller-supplied `ObligationIdentity`, and `value_parity_request` compares it only with a second caller-supplied digest. A caller can give the same wrong digest to both methods, obtain an admitted request for this function, and have QSL report under the wrong obligation. No original checked parity claim is retained or read to make the comparison independent as FR-024-AC-46 requires. | src/replay/function.rs:806; src/replay/function.rs:828; src/replay/function.rs:914 |

## Review evidence

The value path calls `backend_function_packet(self, obligation, ...)` and then `request_from_packet_for_arm(packet, obligation, Input)`. The latter checks `envelope.obligation_identity() != obligation`, but both operands are caller-controlled at this API boundary. The test uses `ObligationIdentity::from_digest([0x35; 32])` for both calls, which is unrelated to the checked package. By contrast, `OriginalCompositeEqContext::selected_claim` derives `parity_obligation` from the original checked node and occurrence before comparing an admitted packet. The #552 Kani path separately retains a full proving-run identity at setup. The new value path has neither source of independent claim identity.

The source has no new vendored artifact, unsafe block, dependency change, or workflow change. The shared Input/Witness arm matching is exhaustive for the private two-variant enum and rejects the other arm. `git diff --check` passed. The author reports focused value-parity tests 2/2 and scoped Clippy/fmt passes; I did not run aggregate gates because this stacked PR is not on its final reviewed base.

## Examined scope and bindings

```yaml
scope:
  - id: FR-024-AC-44
    path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
    role: examined
    excerpt: "An input-arm parity claim producer supplies a completed `WitnessPacket<NoPayload>` before its request builder. CG admits it through `WitnessEnvelope::reconstruct` and requires its `Input` source arm. The operator-parity request copies that packet's checked scalar application occurrence key, and the value-parity request copies its function `declaration` occurrence key, target and opaque obligation identity."
  - id: FR-024-AC-46
    path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
    role: examined
    excerpt: "Before constructing a function-contract, operator-parity or value-parity request, CG compares the admitted packet's `function` target, complete occurrence key, subject `clause_node`, `package_id`, source arm and opaque obligation identity with the independently selected claim from the admitted package and proving-run identity or original checked parity claim, never from that packet."
  - id: TC-035
    path: spec/replay/matrix/TC-035-counterexample-envelope-intake.md
    role: examined
    excerpt: "Have the checked parity claim producer complete an input-arm `WitnessPacket<NoPayload>` before invoking CG's parity intake. Build operator-parity and value-parity requests from their admitted packets and independently selected checked claims."
```

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | The value-parity request leaves `supplied_limits` absent, and its new positive test requires that absence. QSL FR-484 requires the request to carry the complete effective A/H/B supplied ceilings from replay configuration, resolving omitted ceilings to published defaults; `ValueIdentity::sent` applying a default afterward does not make the request complete. | src/replay/function.rs:511; src/replay/function.rs:974; src/replay/function.rs:1700 |

QSL's pinned `b8d7138` FR-484 *Retained claim inputs* table explicitly names the request's complete effective `limits.supplied` object. `ReplayInputs::wire` sets `supplied_limits: None`; `request_from_envelope` changes backend and domains but leaves that member absent. The test asserts `wire.supplied_limits.is_none()` and reads the default only afterward. This is a request contract gap on the new value-parity path. QSL's current `ValueIdentity::sent` resolves the omission to default values, so the tested default replay settles as expected; the finding concerns the wire's required retained inputs.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | `value_parity_packet` and `value_parity_request` both read `self.site.site.value_parity.obligation_identity`; `call_site` mints that site from the compiled package and the request compares the admitted packet against it. The caller can no longer supply an arbitrary obligation digest. |
| FND-002 | still-open | The wire still carries `supplied_limits: None`; QSL FR-484 requires an explicit complete effective A/H/B object for a value-parity request, and the test asserts the omission. |

## Continuation verdict

**CONDITIONAL, not mergeable yet** at `f19849bce067b4f04824c6b2fcca624648167d6c`: FND-001 is fixed. New FND-002 remains open. The packet's QSL-minted identity, function declaration, target, package, Input arm and generator identity bind correctly. The value request still omits the required complete effective supplied ceilings. The final aggregate gate belongs to the code lead after the fix and review.

## Dispositions (round 2)

| FND | outcome | reason |
| --- | --- | --- |
| FND-002 | fixed | `ReplayInputs` now carries configured `SuppliedLimits`; Input packet intake writes `Some(self.inputs.supplied_limits.unwrap_or_default())` to the request. The tagged test checks omitted/default equivalence, three nondefault A/H/B values, QSL typed decode/re-encode, and report claim retention. Witness requests retain their existing configuration behavior. |

## Final review verdict

**PASS** at `e1783fdac9b6e79388e38936459c09209e7139c4`: latest outcomes for FND-001 and FND-002 are fixed. This is a scoped reviewer verdict; the code lead owns the one final aggregate gate. `git diff --check f19849bc..HEAD` passed; no branch changes were made by this review.

## Config-only continuation

**PASS** at `3df522f03a0bfb26f3c1675acbb91dca26d79abf`. The delta from `f718d822e045c44e7d3732dd516191f6cce94402` is two lines in `deny.toml`: a crate-specific `AGPL-3.0-or-later` exception for `quire-checked`. `Cargo.lock` resolves that crate from first-party `agent-ix/quire-eval` revision `5317524d2a040d8f030286ecd8f5d0edecd8e177`; its manifest inherits the workspace's `AGPL-3.0-or-later` license. QSL's locked `qsl-replay`, `qsl-eval` and `qsl-package` manifests declare this dependency. The global permissive license allow-list, git-source allow-list, bans and advisories are unchanged. Prior FND-001 and FND-002 remain fixed; no new finding. `git diff --check` passed. The coder reports focused `make deny` PASS; this reviewer did not rerun the aggregate gate.
