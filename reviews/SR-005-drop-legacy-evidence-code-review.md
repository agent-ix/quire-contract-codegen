---
id: SR-005
title: Drop legacy evidence code review
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen#16 at 2320c7f; the deletion of the retained evidence tree and everything that served it, and the disposition of every finding from the independent adversarial review"
review_set: all
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-006
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/SR-003
    type: references
---

# SR-005: Drop legacy evidence code review

## Summary

This change deletes 2,220 files: the retained `evidence/` tree, the reader that mapped it, its
fixtures, two schemas frozen only because retained records named them, one acceptance criterion, one
test case, one proof obligation and one Make target. The authority is the repository owner's
2026-09-02 release of the evidence-preservation constraint for the pre-stable phase, recorded in the
"Preservation constraint released for the pre-stable phase" section of
`agent-ix/engineering-assurance#7`. The epic's completion criterion and mandatory control were
amended before this work.

The deletion is irreversible, so the only question worth real scrutiny is whether anything still
needs what was removed. A self-review answered yes-it-is-clean and was wrong twice. An independent
adversarial reviewer, given one instruction — attack the deletion — returned fourteen findings, two
critical and four high. Both criticals were live references to deleted material, and one of them was
in the very test whose purpose is to catch exactly that.

The high worth stating plainly is that this change's first attempt at the state vocabulary was a
relabelling, and the review caught it.

## The measurement that drove the design

The brief required a state census taken from the **pre-deletion** tree, per state and per source,
rather than a check that the surviving path still reaches twelve. Taken at `bbd5e67`:

| Source | States demonstrated |
|---|---|
| `scripts/assurance_chain.py` alone | `pass`, `fail`, `unavailable`, `inconclusive`, `not-computed`, `partial`, `stale`, `suspect`, `vacuous`, `tampered` — ten |
| the compatibility census over `evidence/` | adds `unsupported` and `malformed` — and only those two |

`tc_012` asserted the **union**. Deleting the census without touching the test would therefore have
left it passing at ten and reporting the same green: a gate weakening silently rather than a claim
being withdrawn. That is the failure mode the census was measured to find, and it was present.

## The finding this review exists to record

The first fix was wrong, and it was wrong in the direction of keeping the number up.

`malformed` is a declared key in both `ROW_RESULTS` and `CONFORMANCE_OUTCOMES`, so an
`attested-malformed` scenario was added on the reasoning that a producer really reporting it is a
state that travels the chain. Measured against the existing `attested-failed` scenario:

```
attested-failed     fail       {"reasons": ["result_failed"], "receipt_outcome": "invalid"}
attested-malformed  malformed  {"reasons": ["result_failed"], "receipt_outcome": "invalid"}
```

Byte-identical, and structurally so: both tables map `malformed` onto `fail`. The word never leaves
the input file. The chain's own anti-collapse scenario, `non-success-states-stay-distinguishable`,
could not have included it — which is the tell. One of the scenario's three match conditions was a
tautology over a string constructed three lines earlier, and its paired control was strictly implied
by an existing control and could never fail independently.

A declared key is not a distinguishable state. The scenario, its control and `derive_malformed_stream`
are removed, and `malformed` is withdrawn alongside `unsupported`. FR-006-AC-5 requires ten.

The protection against a silent weakening is not a manufactured demonstration. It is `tc_012`
asserting that both states stay **absent**, so a later change that re-acquires either goes red and
has to argue for it. Ten states that are real beats eleven with one painted on.

## Schemas: checked individually, not inherited

Five files sat under `schemas/`. A sibling repository froze four artifacts including its own vendored
copy of the PGM-01 envelope schema, and inheriting that list would have deleted a live dependency.

| Schema | Verdict | Evidence |
|---|---|---|
| `pgm01-derivation-evidence-envelope-v1.schema.json` | **KEPT — live** | `src/oracle.rs:35` `include_bytes!` as `PGM_SCHEMA`; the generator validates every derivation manifest it emits against it; part of the generator's own `executable_digest`; `tests/oracle_generation.rs` compiles it and validates the emitted manifest plus a negative case |
| `generated-rust-oracle-v1.schema.json` | **KEPT — live** | `src/oracle.rs:37` `include_bytes!`; validated against in `tests/oracle_generation.rs` |
| `oracle-source-map-v1.schema.json` | **KEPT — live** | `src/oracle.rs:36` `include_bytes!`; validated against in `tests/oracle_generation.rs` |
| `foundation-evidence-input-v1.schema.json` | **DELETED — frozen only** | referenced only by `assurance/pins.json`, the freeze test, and its own `$id`; no executable consumer at any point |
| `foundation-evidence-manifest-v1.schema.json` | **DELETED — frozen only** | same |

The first row is the trap. A `pgm01` in the filename names the programme that governed the shape, not
the retained records that were deleted. A filename is not a dependency, and deleting that file breaks
every generation. The reviewer verified the split independently from scratch and confirmed all three
kept schemas are `include_bytes!` and feed `executable_digest`, so removing any one fails to compile.

## Findings

Fourteen from the independent adversarial review of `00aa054`, plus three raised by the campaign
coordinator against the *fix* rather than the deletion, plus two this repository found in its own
correction.

Severities are the reviewer's except for FND-501 and FND-502, which the reviewer graded **critical**.
The SpecReview vocabulary is low/medium/high, so they are recorded as `high` here and the reviewer's
grade is stated in this sentence rather than quietly lost. Both were live references to deleted
material and both made `make ci` fail.

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-501 | high | `tc_013` still required `scripts/legacy_evidence_view.py` in the `make -n ci` plan. The test whose purpose is "the gates that replaced them are reachable from `ci` rather than merely defined" was itself left dangling | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-502 | high | `tc_009` asserted five proof obligations against a declaration that names four | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-503 | high | `attested-malformed` was `attested-failed` under another name: both vocabulary tables map `malformed` onto `fail`, so the receipt reasons were byte-identical. One of its three conditions was a tautology over a string constructed three lines earlier | `scripts/assurance_chain.py` | correct-requirement-no-evidence |
| FND-504 | high | its paired control was strictly implied by an existing control and could never fail independently | `scripts/assurance_chain.py` | correct-requirement-no-evidence |
| FND-505 | high | `unsupported` and `malformed` were treated asymmetrically, with gate preservation as the stated motive in six files | `spec/functional/FR-006-shared-assurance-intake.md`, `tests/shared_assurance.rs` | wrong-requirement |
| FND-506 | high | the sealed record kept `issue-13` at revision 1 with a null parent while carrying content for a different change under a different authority; `subject.scope` never named `evidence`, so a 2,205-file deletion sat outside the declared subject and in no entry | `assurance/change-assurance.json` | correct-requirement-no-evidence |
| FND-507 | medium | `spec/test-matrix.md` still titled TC-012 "twelve" | `spec/test-matrix.md` | correct-requirement-no-evidence |
| FND-508 | medium | review obligations were added to an open human decision document | `planning/release-decision.md` | wrong-requirement |
| FND-509 | low | the deleted-artifact census listed three basenames and exempted `shared_assurance.rs` wholesale — which is how FND-501 survived it | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-510 | low | `suites.md` mapped four concerns onto three suites | `spec/evidence/suites.md` | correct-requirement-no-evidence |
| FND-511 | low | the TC-001 trace claimed an assertion on manifest `inputs` that is only a schema requirement | `tests/oracle_generation.rs` | correct-requirement-no-evidence |
| FND-512 | low | prose naming a tree that no longer exists | `scripts/assurance_chain.py`, `spec/assurance/AD-001-codegen-architecture.md` | correct-requirement-no-evidence |
| FND-513 | low | `tc_001` failed on `generator_source_dirty` because the branch was uncommitted at review time | — | wrong-requirement |
| FND-514 | low | `engineering-assurance#21` was declared closed as moot without evidence | `assurance/README.md` | correct-requirement-no-evidence |
| FND-515 | medium | the JSONDecodeError arm of `adapt_conformance` was reached by no case. Raised by the coordinator against sibling work in the same form | `scripts/assurance_chain.py` | correct-requirement-no-evidence |
| FND-516 | medium | the census floor `inspected > 20` was inherited unchanged while its walked population fell 42 → 28, cutting headroom from 22 to 8 | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-517 | low | `purpose` was moved into `record`, where it is an extra field; quoin refused to seal and four tests exited 2. Found by `make ci` | `assurance/change-assurance.json` | correct-requirement-no-evidence |
| FND-518 | low | the FND-509 fix exempted only the `gone` declaration, but this test names the deleted schemas twice. Found by `make ci` | `tests/shared_assurance.rs` | correct-requirement-no-evidence |

## Dispositions

| Finding | Disposition |
| --- | --- |
| FND-501 | **FIXED**. Entry removed. `make ci` reached the same failure independently |
| FND-502 | **FIXED**. `5` → `4` |
| FND-503, FND-504, FND-505 | **FIXED**. The scenario, its control and `derive_malformed_stream` are removed and `malformed` is withdrawn alongside `unsupported`. FR-006-AC-5 requires ten. Not repaired — deleted, because a probe that cannot fail and a probe that is absent are the same check, and the absent one does not also make a claim |
| FND-506 | **FIXED**. The record is `quire-contract-codegen/issue-16`, `evidence` is in `subject.scope`, and `PRESERVE-no-retained-evidence` declares the deletion and its authority. `revision` stays 1 with a null parent because this is a new record whose predecessor was sealed only into an ignored store under `target/` and never persisted; no lineage digest is invented |
| FND-507, FND-510, FND-511, FND-512 | **FIXED**. `planning/foundation-gap-analysis.md` is **ACCEPTED** unchanged within FND-512: it is a dated historical record and delete-never-rewrite forbids editing it |
| FND-508 | **FIXED**. The unsatisfiable item is struck and nothing is substituted for it |
| FND-509 | **FIXED**. Nine names rather than three, and the exemption is the byte range of the test's own declarations rather than the file that holds them |
| FND-513 | **ACCEPTED**. An artifact of the uncommitted tree at review time, not of the change. The gate is revision-bound and is green on the committed tree |
| FND-514 | **FIXED**. Not claimed closed. The epic records that it closes as moot once the campaign repositories have dropped their records, and it names four |
| FND-515 | **FIXED**. A new probe truncates a real row of the real producer stream mid-object and calls the real adapter, requiring an error that names the line. Mutation-tested in both directions — see below |
| FND-516 | **FIXED**. Floor re-derived from the tree as it stands: `>= 24` against a measured 28, with the derivation in the comment |
| FND-517, FND-518 | **FIXED**. Both were defects in the FND-503..009 fixes and both were caught by `make ci` before merge |

## Assurance Context

**Claim boundary.** That this repository no longer retains evidence, that nothing surviving needs
what was removed, and that no claim resting on the deleted records was restated more weakly. It is a
claim about the tree at `2320c7f` and about nothing else.

**Authoritative policy.** `agent-ix/engineering-assurance#7`, the "Preservation constraint released
for the pre-stable phase" section. The decision is the repository owner's, taken on 2026-09-02; an
agent transcribed it. The constraint re-applies unchanged at the move toward stable releases, and
evidence retained under it from that point is immutable.

**Trust inputs.** Engineering Assurance 0.2.0 by git tag; quire 0.31.0 / quire-rs 0.46.0; quoin
0.23.1. `assurance/pins.json` now carries one digest-pinned consumed artifact,
`engineering_assurance/compatibility.py`, which `scripts/check_shared_pins.py` imports on every
`make pins`.

**Failure posture.** Unchanged. The chain still distinguishes ten states, still refuses a foreign
protocol, an empty stream and an unnamed outcome, and still reports rather than collapses. Two states
are withdrawn and asserted absent rather than quietly dropped.

**Execution boundary.** Unchanged. `make assurance-inputs` is still the only target that runs a
producer; it now runs four rather than five. Quire exports and Quoin transcribes; neither executes a
producer, and the three-run PATH probe with its control still asserts it.

**Retained-output identity.** There is none, and that is the point of the change. The Quoin store
lives under `target/` and is ignored.

## Gate results at `2320c7f`

| Gate | Result |
|---|---|
| `make ci` | exit 0, all 11 prerequisites |
| `quire coverage --scope . --json` | 16/53 backed, 0 status lies, 0 new unbacked rows |
| assurance chain | 14 scenarios, 6 controls, 8 adapter probes, all matched; 4 proofs attested `passed` |
| `make pins` | `accepted: true`, no artifact mismatches; probed red by a one-byte upstream edit |

## Limitations

`make ci` is a statement about the tree as committed. `.IGNORE:`, a `-` recipe prefix or an
assignment to `SHELL` each make every recipe report success, and the measurement behind that is in
the Makefile header and in issue #14. No guard was re-added; its absence is recorded by owner
decision, not closed.

Seven mutation probes were deleted with the compatibility view. All seven degraded the mapper's view
of retained records and had no other subject, so they went with the material they guarded. The
chain's own adapter probes — now eight — are unconditional inside `assurance_chain.py`, fold into
`report["matched"]`, and remain reachable from `ci` through `assurance-chain`. No gate vanished
without its subject.

## The replacement probe, and why it is not the one that was deleted

The coordinator's warning is that finding a gap and filling it with something unfalsifiable reads
identically to filling it properly. This repository has form: SR-003 FND-303 found that the
twelve-state census counted `kind`, a free-text label nothing cross-checked, and the check found a
live mislabel in the shipped tree.

`attested-malformed` was that failure again, and it was deleted rather than repaired. What replaced
it is not a state demonstration at all:

| | deleted `attested-malformed` | new `refuses-an-undecodable-row-by-name` |
|---|---|---|
| input | a row's `outcome` field rewritten to `malformed` | a real row of the real stream truncated mid-object |
| observed | the value **after** the adapter mapped it — the one place `malformed` cannot survive | the bytes **before** any mapping |
| claims a state | yes, `malformed` | no, `state: None` |
| can it fail | no. Two of three conditions were implied by the vocabulary table; one was a tautology | yes, measured |

Applying the coordinator's test — name the defect, introduce it, watch it go red:

| Mutation | Probe | Chain |
|---|---|---|
| control, unmodified | matched | matched |
| `adapt_conformance` drops undecodable rows instead of raising | **not matched** | **not matched** |
| it raises but without naming the line | **not matched** | **not matched** |
| restored | matched | matched |

It counts no verification state, so it does not put `malformed` back in the census by way of an error
message. It closes a real gap instead: the `JSONDecodeError` arm of `adapt_conformance` was reached by
no case before it.

## Empty populations

The clause that changed shape is TC-013's. It previously asserted the frozen schemas were *present*
and referenced by nothing; it now asserts two schemas are *absent*. An assertion of absence over a
population that was just deleted is satisfied by a repository that deleted everything, so it cannot
carry the criterion alone.

It does not. Three live schemas are asserted **present and still named by `src/oracle.rs`** — a
non-empty positive population — and the reference census runs over 28 non-markdown readable files
against a re-derived floor of 24. The deleted-schema clause is a corroborating check, not the
load-bearing one.
