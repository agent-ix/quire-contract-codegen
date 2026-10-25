---
id: SR-16301
title: IR-983 CG 553 gap analysis
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; CG #553; FR-024-AC-44, FR-024-AC-46, TC-035; src/replay/function.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-024", type: references }
---

## Summary

Plan completion: not assessed. I compared CG #553's tagged value-parity tests with the merged FR-024-AC-44/46 and TC-035 step 30. The tests exercise Input-arm admission, request/readback of a complete declaration key, typed mismatch refusals, and a QSL value-parity report. The independent checked-claim requirement is not exercised.

## Verdict

**CONDITIONAL** — the tagged value-parity evidence cannot fail when packet and request are both given the same invented obligation identity.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Both value-parity tests construct the claimed obligation with `ObligationIdentity::from_digest([0x35; 32])` and pass that same value to the producer and request. Their mismatch controls mutate only the packet after this setup. No test selects an original checked parity claim, derives its obligation independently, and substitutes a second valid claim's packet or obligation while holding that selection fixed. Thus the FR-024-AC-46 trace tag credits a comparison against an unverified caller token. | src/replay/function.rs:1597; src/replay/function.rs:1646; src/replay/function.rs:1710 |

## Coverage

The new tests are tagged `FR-024-AC-44`, `FR-024-AC-46`, and `TC-035`. The code has FR-024 ownership; no reverse ownership gap or stub was found. I read the merged current-main FR-024/TC-035 text because the stacked #552 base predates the merged #551 spec and lacks AC-46. The authentic two-occurrence operator-parity control is in #552 and is outside this diff. A computed full matrix on this older stacked base would omit AC-46, so this diff review used the merged criterion text and direct trace-tag inspection. The author reports focused tests 2/2; this is not an independent gate result. Plan completion: not assessed.

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

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The tagged value tests take QSL's `FunctionSite.value_parity.obligation_identity` from a checked non-Boolean function, and the substitution test admits the second checked function's packet before verifying that the fixed selection refuses it. |

## Continuation verdict

**PASS for the previously reviewed identity trace** at `f19849bce067b4f04824c6b2fcca624648167d6c`: FND-001 is fixed. `quire matrix --scope . --format json` reports five binders for FR-024-AC-44 and six for FR-024-AC-46; the two value tests appear on both rows and now exercise a QSL-minted identity and a second admitted claim's packet. The new request-field defect is recorded in SR-16300 FND-002. Plan completion: not assessed.
