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

## Findings and dispositions

Fourteen findings from the independent adversarial review of `00aa054`.

| ID | Severity | Finding | Disposition |
|---|---|---|---|
| FND-001 | critical | `tc_013` still required `scripts/legacy_evidence_view.py` in the `make -n ci` plan | **FIXED** — entry removed. `make ci` also caught this |
| FND-002 | critical | `tc_009` asserted five proof obligations; the declaration names four | **FIXED** — `5` → `4` |
| FND-003 | high | `attested-malformed` is `attested-failed` relabelled; the "both halves asserted" claim is false | **FIXED** — scenario, control and `derive_malformed_stream` removed |
| FND-004 | high | the paired control for `attested-malformed` cannot fail independently | **FIXED** — removed with the scenario |
| FND-005 | high | asymmetric treatment of `unsupported` and `malformed`, with gate preservation as the stated motive | **FIXED** — `malformed` withdrawn; FR-006-AC-5 requires ten |
| FND-006 | high | the sealed record kept issue-13's identity at revision 1 for a different change; `subject.scope` never named `evidence`; the deletion was declared in no entry | **FIXED** — record is now issue-16, `evidence` in scope, `PRESERVE-no-retained-evidence` declares the deletion and its authority |
| FND-007 | medium | `spec/test-matrix.md` still titled TC-012 "twelve" | **FIXED** |
| FND-008 | medium | an open human decision document had review obligations added to it | **FIXED** — the unsatisfiable item is struck and nothing substituted |
| FND-009 | low | the deleted-artifact census listed three basenames and exempted this file wholesale | **FIXED** — nine names; exempts only the byte range of its own declaration, which is how FND-001 survived |
| FND-010 | low | "three suites, four concerns" in `suites.md` | **FIXED** |
| FND-011 | low | the TC-001 trace claimed an assertion on manifest `inputs` that is only a schema requirement | **FIXED** |
| FND-012 | low | prose naming a tree that no longer exists | **FIXED** in `assurance_chain.py` and `AD-001`. `planning/foundation-gap-analysis.md` is **ACCEPTED** unchanged: it is a dated historical record and delete-never-rewrite forbids editing it |
| FND-013 | low | the branch had no commits, so `tc_001` failed on `generator_source_dirty` | **ACCEPTED** — an artifact of the uncommitted state at review time, not of the change. The gate is revision-bound and is green on the committed tree |
| FND-014 | low | `engineering-assurance#21` declared closed as moot without evidence | **FIXED** — not claimed closed. The epic records it closes as moot once the campaign repositories have dropped their records; it names four, so this repository does not close it unilaterally |

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
| assurance chain | 14 scenarios, 6 controls, 7 adapter probes, all matched; 4 proofs attested `passed` |
| `make pins` | `accepted: true`, no artifact mismatches; probed red by a one-byte upstream edit |

## Limitations

`make ci` is a statement about the tree as committed. `.IGNORE:`, a `-` recipe prefix or an
assignment to `SHELL` each make every recipe report success, and the measurement behind that is in
the Makefile header and in issue #14. No guard was re-added; its absence is recorded by owner
decision, not closed.

Seven mutation probes were deleted with the compatibility view. All seven degraded the mapper's view
of retained records and had no other subject, so they went with the material they guarded. The
chain's own seven adapter probes are unconditional inside `assurance_chain.py`, fold into
`report["matched"]`, and remain reachable from `ci` through `assurance-chain`. No gate vanished
without its subject.
