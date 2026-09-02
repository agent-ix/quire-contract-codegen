---
id: SR-006
title: Drop legacy evidence gap analysis
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen#16 at the final head; FR-006 coverage after the deletion, the coverage arithmetic, and the honest reach of every claim that survives"
review_set: all
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-006
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TM-001
    type: references
---

# SR-006: Drop legacy evidence gap analysis

## Summary

SR-004 asked whether each FR-006 criterion had a test that would fail if the criterion stopped
holding. This document asks a narrower and more dangerous question: after deleting 2,220 files, does
any criterion still have a test that *passes without checking what it claims to check*.

Two did, and both were caught before merge — one by the independent adversarial review and one by
`make ci`. A third, subtler case was caught by the pre-deletion census the brief mandated: `tc_012`
would have gone on passing at ten states while asserting twelve, because it took the union of two
sources and only one of them was deleted.

## The coverage arithmetic

The gate is not "zero unbacked rows" — most rows in this repository are unbacked by design, because
FR-003 and FR-004 are specified and unimplemented. The gate is that **no row becomes unbacked as a
side effect**.

| | Before (`bbd5e67`) | After | Delta |
|---|---|---|---|
| backed | 18 | 16 | −2 |
| total | 56 | 53 | −3 |
| status lies | 0 | 0 | 0 |

Every unit of that delta is accounted for by material this change deleted:

| Group | Before | After | Why |
|---|---|---|---|
| `spec/evidence/suites.md` | 0/7 | 0/6 | SUITE-005 removed. It was unbacked before and after, so no backed row was lost |
| `spec/functional/FR-006-...` | 6/6 | 5/5 | FR-006-AC-4 removed. It was backed, so backed −1 and total −1 |
| `spec/test-matrix.md` | 10/13 | 9/12 | the TC-011 row removed. It was backed, so backed −1 and total −1 |
| `spec/nonfunctional/NFR-002-...` | 2/5 | **2/5** | unchanged — see below |
| all other groups | — | — | unchanged |

Arithmetic: total 56 − 1 − 1 − 1 = 53. Backed 18 − 0 − 1 − 1 = 16. Both close.

**New unbacked rows: none. Rows that stopped being unbacked: none.** The unbacked set after is
identical to the unbacked set before.

## The row that nearly became unbacked

`NFR-002-AC-1` — "Every emitted artifact records the required tool, input, schema, backend,
configuration, output, and digest identity" — was backed by exactly one tag, on the deleted
`tc_011_retained_evidence_is_read_through_the_shared_mapping_without_moving_a_byte`. Deleting that
test would have taken NFR-002 from 2/5 to 1/5, which is a row becoming unbacked as a side effect.

The tag was not recreated to keep a gate green. It was moved to the test the requirement itself has
always named. NFR-002's own acceptance table declares AC-1's verification method as **Test (TC-001)**,
and `spec/test-matrix.md` already listed `NFR-002-AC-1` under TC-001's Traces To. The tag had been
attached to an evidence-compatibility test that read identity out of historical records, rather than
to the test that reads it out of an artifact this crate emits.

`tc_001_boolean_oracle_bundle_is_deterministic_traceable_and_schema_valid` asserts the emitted
manifest records the producer's source revision and executable digest, the backend kind, the
parameters digest, the dependencies digest, both output content digests, and the first output's
schema digest — and validates the whole manifest against the PGM-01 envelope schema plus a negative
case.
That is AC-1. The independent reviewer checked this line by line and found the tag honest and better
placed than before.

One overclaim was corrected: the trace comment initially said the test asserts manifest `inputs`
field by field. It does not; `inputs` is required by the envelope schema the assertion validates
against. The comment now says so.

## Criterion coverage after the deletion

| Criterion | Test | Would it fail if the criterion stopped holding? |
|---|---|---|
| FR-006-AC-1 | `tc_008` | Yes. A component not classifying `compatible`; a consumed-artifact digest that moved; a mirror reference; an install line naming a matrix-forbidden version. Both scans are probed by injection. Newly re-probed: a one-byte edit to `engineering_assurance/compatibility.py` takes the gate to exit 1 naming the digest, and restoring it returns 0 |
| FR-006-AC-2 | `tc_009` ×5 | Yes. Any scenario, control or adapter probe not matching; a declared argv absent from `make -n assurance-inputs`; a producer that cannot report anything but pass; a producer invocation by the driver; the driver writing into its own input directory. The obligation count assertion is now 4, matching the declaration |
| FR-006-AC-3 | `tc_010` | Yes. An impact snapshot that is not the export's digest; a `status_lies` entry; an export attested as anything but `passed` |
| ~~FR-006-AC-4~~ | — | **Removed.** The retained bytes it asserted over are deleted. Not restated over a smaller tree |
| FR-006-AC-5 | `tc_012` | Yes, and in both directions — see below |
| FR-006-AC-6 | `tc_013` | Yes. Any named file present; the retained tree present; a live schema absent or no longer named by the generator; any code or configuration file referencing a deleted artifact; `make -n ci` not naming a replacement gate |

## FR-006-AC-5: the gap this analysis exists to record

The criterion previously claimed twelve distinguishable verification states. It now claims ten, and
the two-state reduction is the most load-bearing judgement in the change.

**Measured on the pre-deletion tree, per state and per source** — not "does the surviving path still
reach twelve", which is the question that would have missed it:

- the assurance chain alone demonstrated ten
- the compatibility census over `evidence/` supplied `unsupported` and `malformed`, and only those

`tc_012` asserted the **union**. Deleting the census alone would have left it green at ten while
claiming twelve. That is a gate weakening silently, and it is the specific failure the brief's
pre-deletion census requirement exists to catch.

**`unsupported` is withdrawn.** It is not in `CONFORMANCE_OUTCOMES` or `ROW_RESULTS`, Quoin's
attestation `result` has no such value, and nothing on the intake path produces it. It was a property
of the compatibility mapping — a retained record carrying an unknown PGM-01 schema version. The
generation corpus does reach an `unsupported` *Interface-001 terminal state*, and borrowing it would
collapse two vocabularies this very test exists to keep apart.

**`malformed` is withdrawn too, and the first attempt got this wrong.** It *is* a declared key in
both tables, and a scenario was added feeding the adapter a producer stream declaring that outcome.
Measured against the existing `fail` scenario, the receipt reasons were byte-identical, because both
tables map `malformed` onto `fail`. The chain's own `non-success-states-stay-distinguishable`
scenario could not have included it. The scenario was `attested-failed` under a second name; its
paired control was implied by an existing control and could never fail independently. It was removed.

A declared key is not a distinguishable state. The honest count is ten.

**What replaces the lost coverage is an assertion of absence.** `tc_012` requires `unsupported` and
`malformed` to be *missing* from `states_demonstrated`. A later change that re-acquires either — by
relabelling a refusal, by borrowing the Interface-001 terminal state, or by adding back a scenario
that is another state under a second name — turns the test red and has to argue for itself. That is a
stronger guarantee than a demonstration that was never distinguishing anything.

## Gaps that remain open, and are not closed by this change

- **`unsupported` and `malformed` have no demonstration in this repository.** This is a real
  reduction in what the intake path is shown to distinguish, recorded in FR-006-AC-5, MP-001,
  `assurance/README.md` and `CLAUDE.md` rather than papered over.
- **FR-003 and FR-004** remain specified and unimplemented. No proof obligation was created for
  either, and issues #2 and #5 stay open. A proof obligation whose subject does not exist is the most
  complete false green available.
- **`agent-ix/engineering-assurance#21`** has no subject in this repository any more. It is not
  claimed closed: the epic records it closes as moot once the campaign repositories have dropped
  their retained records, and it names four.
- **`make ci` is not a trust root** and no guard was re-added. Issue #14 carries the measurement.
- **Seven mutation probes were deleted** with the compatibility view. All seven degraded the mapper's
  view of retained records and had no other subject. The chain's adapter probes are unconditional and
  still reachable from `ci`, so the total fell from fourteen to eight without a gate losing a subject
  it still had — seven chain probes, plus one added here for the adapter's undecodable-bytes arm,
  which no case reached before.

## Verification methods that no longer resolve

Every argument in the specification tree that rested on retained evidence was swept, not just the one
acceptance criterion:

| Document | What rested on retained evidence | Disposition |
|---|---|---|
| `FR-006` | AC-4, one input, one output, two behaviour clauses | removed |
| `spec/test-matrix.md` | the FR-006-AC-4 row and the TC-011 row | removed |
| `spec/test/TC-011-...` | the whole document | deleted |
| `spec/test/TC-012-...` | the union with the compatibility census | rewritten to the chain alone plus an absence assertion |
| `spec/test/TC-013-...` | "frozen artifacts are present and unchanged" | rewritten to "deleted and unreferenced", plus the live-schema assertion |
| `spec/evidence/suites.md` | SUITE-005 and its prose | removed; identifiers deliberately not renumbered |
| `MP-001` | the reader, the read-only claim, the mutation-probe paragraph, the refusal statement | removed and replaced with the state-vocabulary limitation |
| `NFR-002` | "retained evidence" in Scope, and a Verification section gating on the deleted TC-011 | struck. This is the unsatisfiable-gate pattern a sibling repository left behind, and it was present here |
| `CAC-001` | "retained evidence" in the controls surface list | removed |
| `AA-001` | "complete evidence" in Reasoning, and "retained measurements" in the Sufficiency Decision | de-referenced; the Sufficiency Decision now says explicitly that no retained tree exists to review |
| `AD-001` | the evidence-verifier surface in the present tense | rewritten to past tense; the boundary observation kept |
| `planning/release-decision.md` | "retained measurement/evidence manifests" in an **open human decision** | struck, with nothing substituted — adding review obligations to a pending decision is not an agent's to do |
| `planning/foundation-gap-analysis.md` | present-tense references to `evidence/ANCHORS` | **left unchanged**. It is a dated historical record and delete-never-rewrite forbids editing it |

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-601 | high | `tc_012` asserted the union of two sources and only one was deleted, so the criterion would have kept passing at ten states while claiming twelve | `tests/shared_assurance.rs`, `spec/functional/FR-006-shared-assurance-intake.md` | correct-requirement-no-evidence |
| FND-602 | high | `NFR-002-AC-1` was backed by exactly one tag, on the deleted `tc_011`, so the deletion would have taken NFR-002 from 2/5 to 1/5 | `spec/nonfunctional/NFR-002-provenance-boundary.md` | correct-requirement-no-evidence |
| FND-603 | high | `NFR-002`'s Verification section gated on TC-011, which this change deletes — an unsatisfiable verification method, the same pattern a sibling repository left behind | `spec/nonfunctional/NFR-002-provenance-boundary.md` | correct-requirement-no-evidence |
| FND-604 | medium | five further documents argued from retained evidence beyond the one acceptance criterion the brief named | `spec/assurance/AA-001-codegen-argument.md`, `spec/assurance/MP-001-codegen-measurements.md`, `spec/assurance/CAC-001-codegen-contract.md`, `spec/evidence/suites.md`, `planning/release-decision.md` | wrong-requirement |
| FND-605 | medium | TC-013's clause changed from asserting frozen schemas present to asserting deleted schemas absent, which over a just-deleted population is satisfied by a repository that deleted everything | `spec/test/TC-013-no-local-framework.md` | wrong-requirement |
| FND-606 | medium | the census floor was inherited unchanged while its walked population fell 42 → 28, and once re-derived was still total-only over a tree whose directories hold four or five files each | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-609 | high | the reference census could not see `Makefile`, which is the only file a reintroduced Make target can live in | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-610 | medium | three claims corrected in the spec, the README and `CLAUDE.md` were left stale in the sealed change-assurance record | `assurance/change-assurance.json` | correct-requirement-no-evidence |
| FND-607 | low | `unsupported` and `malformed` have no demonstration in this repository, which is a real reduction in what the intake path is shown to distinguish | `spec/functional/FR-006-shared-assurance-intake.md` | correct-requirement-no-evidence |
| FND-608 | low | FR-003 and FR-004 remain specified and unimplemented, with no suite and no proof obligation | `spec/functional/FR-003-kani-lowering.md`, `spec/functional/FR-004-vacuity-evidence.md` | correct-requirement-no-evidence |

## Dispositions

| Finding | Disposition |
| --- | --- |
| FND-601 | **FIXED**. Measured per state and per source on the pre-deletion tree, then both affected states withdrawn and asserted absent. The first attempt manufactured a demonstration for `malformed` and was itself withdrawn — SR-005 FND-003 |
| FND-602 | **FIXED**. The tag moved to `tc_001_boolean_oracle_bundle_is_deterministic_traceable_and_schema_valid`, which is the test NFR-002's own table has always declared as AC-1's verification method. NFR-002 stays 2/5 and no row became unbacked |
| FND-603 | **FIXED**. The TC-011 sentence is struck and no weaker substitute is put in its place |
| FND-604 | **FIXED** in all five. `planning/foundation-gap-analysis.md` is **ACCEPTED** unchanged as a dated historical record |
| FND-605 | **FIXED**. The load-bearing clause is the live-schema assertion over three present files, plus a reference census over 31 tracked files; the absence clause corroborates rather than carries |
| FND-606 | **FIXED**. Floor re-derived at `>= 27` against a measured 31, plus a directory-set guard and per-directory floors. The first per-directory guard read its floors from a hardcoded list and could not catch that list shrinking; it was probed, found green on a deleted entry, and rebuilt on discovery — SR-005 FND-522. Every figure was taken from the walk, with both sides on the same deny-list filter: 45 − 14 + 0 = 31, and the census counts 31 |
| FND-609 | **FIXED**. `collect_sources` admits extensionless sources by name and `.yaml` alongside `.yml`. Reproduced before fixing: the deleted target appended to the `Makefile` left the census green |
| FND-610 | **FIXED**. The sealed record was re-read last against every claim corrected elsewhere; SR-005 tabulates the three |
| FND-607 | **ACCEPTED**. Recorded in FR-006-AC-5, MP-001, `assurance/README.md`, `CLAUDE.md` and a TC-012 absence assertion. It is a reduction, and it is stated as one |
| FND-608 | **ACCEPTED**. Issues #2 and #5 own it. No proof obligation was created for either: a proof obligation whose subject does not exist is the most complete false green available |

## Verdict

The deletion is complete, the coverage arithmetic closes, and the one criterion that lost coverage
lost it visibly. Two states were withdrawn rather than restated, and the attempt to restate one of
them was caught and removed before merge.

## Conclusion

The deletion is complete and nothing surviving needs what was removed. The one criterion that
genuinely lost coverage lost it visibly, in three documents and one test assertion, and the count
went down rather than being held up by a demonstration that was not demonstrating anything.
