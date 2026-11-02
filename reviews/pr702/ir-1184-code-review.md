---
id: SR-202610310163
title: IR-1184 code-review review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; origin/main...f7b11de; src/, tests/, spec/;
  IR-1184
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-032
  type: references
- target: ix://agent-ix/quire-contract-codegen/FR-033
  type: references
---

## Summary

Ticket: IR-1184. PASS for the frozen IR-1184 implementation delta. Immutable generation authority rejects edits to public scalar identities/artifacts and public composite claim maps; exact staged path/bytes, positional draws, original limits, source and package are checked independently. Scalar request/report conversion checks every actual wire field and ReplayLimits before full actual-sent claim/report identity and terminal projection. The repeated x+x control executes actual emitted bytes with independent draws [2,4] and calls the public pinned QSL operator route. Composite controls execute genuine generated Eq and a changed native mutant, preserve actual Outcome<bool>, and reject changed staged bytes; they invent no pair count, shadow evidence or backend proof. Verified facade evidence remains explicitly independent. No new copied encoder, manifest/lock mutation, pin purge or CI weakening occurs. Driver honesty remains explicit; these products are checked assertions, not unforgeable execution receipts. IR-631 and IR-806 production/run-settlement pipelines remain gated and are not discharged by this review.

## Verdict

PASS

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - | correct-requirement-no-evidence |

## Coverage

Reviewed all 16 changed files at f7b11de8da0cc3c7f5e4b86539b7445104d6a756 against origin/main 1b28e456; read AGENTS.md, CLAUDE.md, IR-1184 and its comments, FR-032 and FR-033 boundaries, rust-style API/implementation/testing/safety/wire rules and installed gap-analysis references. No applicable AssuranceProfile was found. No tests, Cargo, Kani or backend proofs were run by this reviewer, as explicitly directed; pre-review fmt/clippy and focused 1 unit plus 37 IT passes were supplied by the lead. Full local integration target is pending the lead’s final single run. The source inventory is 135 Rust files and the test/support inventory is 129 files. Existing subsystem ownership is declared by core, strategy, oracle, publication, evidence, routed, Kani and replay requirements. No newly ownerless public behavior, concrete source/test stub or coverage inflation was confirmed. Arc ownership retains one admitted package per source within scalar generation and one immutable package/artifact set per composite call; scalar harness Eq excludes unrelated whole-call package siblings. Generated scalar Rust output is preserved; the proving record intentionally gains scalarReplayBody. No line coverage was measured. Broad optional whole-repository semantic analysis was skipped; the requested scoped intent/test/code review was performed.
## Examined scope and bindings

```yaml
scope:
- id: FR-032-AC-15
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED / CODE-GATED. The admitted IR reader supplies the original application
    structural identity and authentic expression occurrence. CG typed generation supplies
    and retains the scalar kind and one argument for each operand position in actual
    draw order, with typed graph-child or inline-literal provenance and the harness
    range. Two draws of one parameter remain two entries. Changing a package, path
    or producer ordinal changes the structural identity; changing an included body
    member while the identity stays equal is caught by complete retained-body comparison
    and refuses. A source-span or n
- id: FR-032-AC-16
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED / CODE-GATED. An independent fixture obtains the same application
    `NodeKey`/`CheckedNodeId` from IR admission and CG retention, checks the exact
    structural identity and complete positional body, and the actual QSL operator
    route accepts the matching claim. No CG-local identity derivation, copied encoder,
    stand-in verifier or synthetic acceptance passes.
- id: FR-032-AC-17
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED / CODE-GATED. With a valid original claim retained, altering only
    the claimed package, path or producer ordinal, or changing one otherwise membership-valid
    retained body member while reusing the old structural identity, produces a typed
    stale-dependency/revision-mismatch refusal before scalar comparison. A body-only
    edit cannot pass because the node identity is equal. No claimed or recomputed
    digest is required.
- id: FR-032-AC-18
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED / CODE-GATED. If the admitted IR read lacks a valid application
    identity or authentic occurrence, or CG typed generation lacks a required retained
    body member, the CG builder returns typed setup refusal without replay or fabricated
    identity. A caller string or usable sentinel cannot replace the absent authority.
- id: FR-033-AC-18
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED / CODE-GATED. CG obtains the original equality/inequality application
    structural identity from IR admission and retains the authentic occurrence, kind
    and one argument per operand position in original order. A self-comparison keeps
    two graph-child entries with each drawn Node-keyed bound; distinct parameters
    and composite literals keep their own graph-child entries, with empty Bounds for
    literals. Population-keyed metadata stays in the full proving record; an admitted
    QSL claim containing it refuses as HarnessUnknownKey before identity/body comparison.
    Bounds key ordering and duplicate
- id: FR-032-AC-1
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: context_only
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    A public consumer passes the routed harness's `ScalarObligationIdentity` directly
    to the scalar builder; its typed call constructs neither `KaniObligationIdentity`
    nor V1 `ReplayPackage`. After the ordered setup checks, an otherwise valid request
    returns unsupported-capability only while a named step-5 capability is unavailable;
    when all are available it returns an admitted plan, and authenticated completion
    enables replay/conversion. Conversion of a capability refusal returns no settlement
    rather than a synthe
- id: FR-032-AC-2
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: context_only
  excerpt: PLANNED (IR-631). The builder decodes actual routed-harness playback in
    persisted symbolic call order through the Kani output adapter; another harness
    name, missing/extra operand, wrong byte width, playback above the builder-owned
    8 MiB per-harness ceiling or malformed assertion block yields the distinct typed
    refusal under the stated precedence before replay.
- id: FR-032-AC-3
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: context_only
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    Each operand at its own lower or upper bound is admitted and the adjacent out-of-domain
    value is refused before replay. The admitted QSL inputs preserve the catalogued
    operator, operands, operand ranges, result range and harness limits. Another node/operation,
    altered operand schema or replay limit differing from the retained original generation
    context refuses; operand correspondence uses [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md)
    AC-16.
- id: FR-032-AC-4
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: context_only
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    The driver's actual native observation of the proved generated oracle is required
    and tied to this operand vector, limits and retained staged artifact bytes; checked
    completion requires driver authentication. A bare outcome echo, missing observation,
    another artifact's observation or fresh regeneration that removes the arithmetic
    mutation refuses before QSL evaluation. Playback bytes and Kani falsification
    alone cannot supply a generated outcome.
- id: FR-032-AC-5
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: context_only
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    The public consumer executes the same proved generated artifact and invokes QSL
    between the builder and converter. QSL compares its actual native observation
    with authoritative exact operator evaluation using the four-operator projection
    owned by [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) AC-37,
    without adding refusal-cause or accounting-counter equality; changing the generated
    arithmetic while keeping original node/operator identity changes the measured
    parity result. `Undefined`, `Inco
- id: FR-032-AC-6
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: context_only
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    For every `Diverged`, `Agrees`, `GeneratedFault`, `RefusedInput`, exact `Incomplete`
    and other `Refused` outcome, the converter compares the report's full claim to
    the retained sent claim and actual sent wire against the original proving context
    before conversion; a changed package/function/source/dependency or original limit
    refuses even with an equal claim. At the immutable-report/retained-binding boundary,
    an absent report, another result/run, or changing only the obligation, node, occurrence,
    kind, operator
- id: FR-033-AC-19
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: context_only
  excerpt: PLANNED / CODE-GATED (IR-806). Before falsified parity invocation, CG authenticates
    the retained proved generated artifact and original source/package/context, reconstructed
    operands and original execution limits against the driver's actual native observation;
    it preserves the owning same-valid-claim Disagreed precedence before treating
    a missing or failed observation as a refusal. Without a same-valid-claim Disagreed,
    a different artifact, changed retained bytes or context, altered operands or limits,
    missing observation, or fresh regeneration that removes a mutation refuses with
    a typed CG p
- id: FR-033-AC-20
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: context_only
  excerpt: PLANNED / CODE-GATED for the current QSL FR-358 claim shape; earlier IR-666
    converter and IR-635 verified-constructor controls remain partial evidence only
    for unchanged members. Before projecting any QSL outcome, CG compares a genuine
    `report.claim()` against `CompositeIdentity::new` over exactly the actual sent
    obligation, all remaining `CompositeParityClaim` members (node, occurrence, operator,
    obligation kind, harness bounds and limits) and complete `CompositeEvidence` (falsified
    operands, shadow verdict and pair count, native observation and cause, refinement;
    or verified SUCCESS count an
bindings: []
```
