---
id: "SR-1540"
title: "IR-624 spec review: state field ranges read from the package QSL emits (CG #285)"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@82ba95ff370dc5b6caac85d1623f7764a0f2ac1d; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md, spec/tests.md (PR #285 diff against main 13fc2d0)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: reviews
---

# SR-1540: IR-624 spec review (base)

## Summary

Ticket: IR-624. PR: agent-ix/quire-contract-codegen#285, head 82ba95f, against main 13fc2d0.
Method: spec-review (base checklist plus soundness of the design). The integrity,
criterion-strength, scope-boundary and EARS lenses are in SR-1541 to SR-1544.

Re-measured, not taken from the PR text:

- **Emitted shape.** A scratch probe over `Twin::emitted_package` (QSL bcca433, IR 6fb6e97)
  dumped the package. The `model`/`object_type` node has body `{"members":[],"term":"aggregate"}`.
  Each `balance` and `audit` read is an `expression`/`query` `quire.op.record.project` over
  the `deref`, with `member {kind: field, name, declaration: <object>}` and `result_type` the
  one `bounded_domain`/`integer_range` node (0, 1000). The packages for the two clauses are
  byte-identical, so the whole unit is emitted. All confirmed.
- **QSpec FR-322 "Model-owned members".** Step 2 requires the model node's body to be empty.
  Step 4 gives the member type. A projection's `result_type` "is admitted exactly when it names
  the node of that type" (FR-322-AC-31, AC-34). Confirmed.
- **IR `check_model_member`** (operations.rs:1773). It resolves the field and compares
  `result_type.digest` with `member_type.node_key()`, and only on that digest. It runs only when
  `is_model_declaration_node` holds. Otherwise `check_field_member` checks only that the name
  is declared. `model_members`, `DomainModel`, `resolve` and `field_type` are `pub(super)`, and
  `CheckedPackageV2` holds only `wire`, `kinds` and `bytes`. Confirmed.
- **Scratch probe of the design** (field_range reads the common `result_type` of the reads).
  The emitted package generates, so tc_035_..._reads_no_field_range_... fails as expected.
  Generation also succeeds for the fixture's RationalBound, WideRange and Literal shapes (see
  FND-002).
- **Tamper probe.** In the emitted package, set the `integer_range` node's `max` to 10 or to
  5000 and keep its node id. Patch `identity_projection` to match and recompute `package_id`
  (an unkeyed SHA-256). `CheckedPackageV2::read` with the domain evidence returns **Admitted**
  in both cases (see FND-001).

Verdict: **changes requested**. The direction is right: read the type from the field's read
and not from the empty model body. The two soundness claims that make it safe are not true of
the IR reader as it stands.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The claim "in an admitted package the result_type of a field's read is the field's declared type, and an integer_range there is its declared range" is false for the bounds. IR compares only the `result_type` digest with the derived member type's node key. It never re-derives an anonymous structural node's key from its body: `validate_application_keys` covers application nodes and `validate_nominal_nodes` covers nominal nodes, and structural.rs:51 says structural keys are not re-derived. A package whose `integer_range` node carries the key of Int[0,1000] but bounds 0..10 or 0..5000 is admitted (reproduced). `field_range` reads the bounds from that node's body, so the harness would `assume` a range the model does not declare. A narrower range is an unsound Verified. The spec must make the range depend on an IR guarantee that exists: IR re-derives anonymous `bounded_domain` keys, or the Q-5 accessor returns the bounds. Until then, state the dependency as a stated assumption. | spec/kani/functional/FR-015-bounded-kani-obligations.md:431-444, FR-015-AC-77 |
| FND-002 | high | The design trusts a read's `result_type` for any framed object, but IR checks `result_type` against the member type only for a model declaration node. For any other object (the hand-built fixture's `model`/`object_type`, which carries a `declaration` and body members), `check_field_member` checks only the name. The fixture today is admitted with reads typed by `audit`'s Int[0,1000] while the `balance` member is a rational bound, an Int[0, 2^63] range or a literal. With the probe reader, all three generate a harness assuming 0..1000. FR-015-AC-77 even requires that "over one whose object body declares a different range for a field, it still returns the range of that field's read". That turns an IR-unchecked type into the harness precondition. Fix: trust a read's `result_type` only when the object is a model declaration node (empty body, IR check ran). For an object with body members, require the read to agree with the member and otherwise refuse (`ConflictingFieldReads` or similar). Drop the AC-77 clause. | FR-015-AC-77, spec/kani/functional/FR-015-bounded-kani-obligations.md:456-459 |
| FND-003 | medium | The unranged-field fallback is called "sound for the proof and wider than the model", but its consequence is not specified. In a frame-effect harness the ungranted fields are usually read by no clause, so an unconstrained draw is the main case, not an edge. A falsified harness can then carry a playback value outside the model's range. Per QSL's out-of-range ruling as relayed in PR #286 (untrusted PR text, author the IR-465 coder), an out-of-range pre state stays refused, so that counterexample can never reproduce and settles Inconclusive. Also, `domains` cannot distinguish "the model declares no range" from "the package did not carry it". State how such a run settles, and record the missing-range cause in the identity, or refuse frame generation when an ungranted model field has no range. | FR-015-AC-78, spec/kani/functional/FR-015-bounded-kani-obligations.md:471-474, FR-024-AC-23 |

## Dispositions

Round 1, reviewed at 2fac5ac6394887b1d617c64350d7886e057f2e4c.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 2fac5ac: Limit now stated truthfully (re-measured: the tamper with max=10 is still Admitted at IR 6fb6e97). IR-627 and IR-628 exist in Linear. Criteria are marked GATED. The gating gaps are filed as new SR-1541 FND-003. |
| FND-002 | fixed | 2fac5ac: The split matches IR. is_model_declaration_node (Model tag with no declaration) plus recover (empty body, else stale-node-key) means an admitted model/object_type with an empty body and no declaration always went through check_model_member. Other objects read the body and refuse MemberDisagreesWithRead. |
| FND-003 | fixed | 2fac5ac: FR-015-AC-81 records NoRead or TypeNotRange in the identity. FR-024-AC-35 states the Inconclusive(ReplayRefused) settlement. |
