---
id: SR-003
title: Shared assurance migration closing code review
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen#13 at efb04d3; the independent adversarial review of a63b387's parent, and the disposition of every SR-001 and adversarial finding"
review_set: all
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-006
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/SR-001
    type: references
---

# SR-003: Shared assurance migration closing code review

## Summary

SR-001 was a self-review and it missed a high. An independent agent was given one instruction —
find false greens — and ran 22 probes against `8246110`. It returned six findings, one of them high,
and every one is of the same class: a gate that reports success without the thing it claims to check
having happened.

The high is worth stating plainly because it defeated the central architectural claim of the
migration. `make assurance-inputs` runs five producers through four programs, and the
producer-isolation test shimmed three of them. The reviewer injected a `quire coverage` call into the
driver and the test said `ok`; then deleted `target/assurance/quire-static-export.json`, made the
driver regenerate it by running `quire coverage` itself, and got a green chain and ten green tests.
Then did the same through `python3 scripts/check_upstream_pins.py`. A driver that can produce its own
inputs can produce a green run out of nothing, and for three of the five proofs it could.

## Verdict

**APPROVED** at `efb04d3`, with the dispositions below.

## Gates at efb04d3

| Gate | Result |
| --- | --- |
| `make ci` | **exit 0** on a clean tree; eleven prerequisites |
| Rust tests | **29 passed, 0 failed, 0 ignored**, twice — once on the default toolchain and once on 1.75.0. 4 harness, 1 integration, 9 oracle, 10 shared-assurance, 5 strategy |
| `make spec` | `quire validate` clean, zero warnings, over `spec/`, `planning/`, `plan/` and `reviews/` |
| `quire coverage` | 18/56 rows backed; **0 contradicted statuses**. The 38 unbacked rows are the FR-001..FR-005, NFR and StR rows this change deliberately leaves 🚧 Planned, plus FR-003 (3) and FR-004 (4), which have no implementation at all |
| Generation conformance | **9/9 rows pass**, each at or above its declared check floor; terminal states reached: `generated`, `unsupported`, `invalid-input` |
| Upstream identity | 2/2 upstreams agree across constant, manifest and lockfile |
| MSRV | `rustup run 1.75.0 cargo check --locked --all-targets` clean |
| Shared pins | 4/4 compatible; 0 artifact digest mismatches; 0 mirror references; **0 incompatible install pins** (was 1); acceptance state `pending_human_acceptance`, reported and not gated on |
| Assurance chain | **14 scenarios, 6 controls, 7 adapter probes, all matched**; exit 0 |
| Record digest | `824dcb39c3cce12956114ce66577beb8405f8eb463d85cdcb18de2fc8cb1ac6f` |
| Receipt | `incomplete`, reason `decision_missing` + `unresolved_unknown`. Correct: no human decision exists and none was synthesized |
| Compatibility census | **16/16 cases matched**; 44 retained envelopes, all `incompatible`; 2,205 evidence files read; 0 bytes moved; 0 uncommitted differences; floor 2,205/44 met |
| Compatibility mutation probes | **7/7 detected** |
| `cargo deny` | clean |
| `audit-unsafe` | passed; and exits 2 rather than 0 when it has nothing to audit |
| Hosted CI | `workflow_dispatch` only; **not dispatched** |

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-301 | high | The producer-isolation gate covered 2 of 5 producers. It shimmed `cargo`, `rustup` and `rustc`; the input target also runs two producers through `python3`, and `quire` was exempted on a subcommand test that permitted the driver to run the producer command itself. The driver was made to regenerate two deleted inputs and everything stayed green | `tests/shared_assurance.rs`, `scripts/assurance_chain.py` | correct-requirement-no-evidence |
| FND-302 | medium | The `make -n ci` test-runner check matched the bare string `test --locked`, which `msrv:` supplies on its own, so deleting `test` from `ci:` was invisible — the exact failure the comment above it claimed to prevent | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-303 | medium | The twelve-state census counted `kind`, a free-text label in the fixture declaration that nothing cross-checked. Two of the twelve states came only from labels, and relabelling two healthy records "demonstrated" both | `scripts/legacy_evidence_view.py`, `tests/fixtures/legacy-compat/expectations.json` | correct-requirement-no-evidence |
| FND-304 | medium | The retained-evidence census had no floor: it compared the tree to itself, so it agreed at any size. 2,203 of 2,205 files and 42 of 44 envelopes could be dropped in a commit with every gate green | `scripts/legacy_evidence_view.py` | correct-requirement-no-evidence |
| FND-305 | low | `make assurance` exited 0 over a fabricated Quire export: the rule was "a populated object" and `{"junk":[1]}` satisfied it | `scripts/assurance_chain.py` | correct-requirement-no-evidence |
| FND-306 | low | A positive control read the same boolean as the scenario it paired with, so it was a second printing of that scenario's measurement rather than an independent observation | `scripts/assurance_chain.py` | correct-requirement-no-evidence |
| FND-307 | low | The input census could be walked around by regenerating an input on the line above it, because a census only speaks for the interval it brackets. Found by re-probing the FND-301 fix | `scripts/assurance_chain.py` | correct-requirement-no-evidence |

## Dispositions

| Finding | Disposition | Evidence |
| --- | --- | --- |
| SR-001 FND-101 (incompatible quoin pin) | **FIXED** | repinned to the matrix version; `incompatible_install_references` makes it a gate, probed by injection in `tc_008` |
| SR-001 FND-102 (ix-flow absent) | **FIXED** | added to the hosted install list; classified 4/4 |
| SR-001 FND-103 (unsafe audit false green) | **FIXED** | carried from the divergent branch; measured 0→2 exit change |
| SR-001 FND-104 (old path not revision-portable) | **ACCEPTED** | a property of the deleted path; recorded in the dual run rather than repaired, because polishing machinery scheduled for deletion is out of scope |
| SR-001 FND-105 (`--strict` returns 0) | **DEFERRED** | agent-ix/quire-contract-ir#21. Mitigated locally: `tc_010` gates on the export's own `status_lies`, which is 0 |
| SR-001 FND-106 (FR-003/FR-004 unimplemented) | **ACCEPTED** | no suite and no proof obligation created; issues #2 and #5 own the gap |
| SR-001 FND-107 (Make is not a trust root) | **DEFERRED** | agent-ix/quire-contract-codegen#14, with measured numbers in four places |
| SR-001 FND-108 (frozen set is two, not four) | **FIXED** | measured on this repository; `tc_013` asserts both the frozen pair by digest and the three live ones by the generator including them |
| SR-001 FND-109 (`evidence/ANCHORS` unread) | **ACCEPTED** | left byte-identical; recorded in `assurance/README.md` and `CLAUDE.md` |
| FND-301 | **FIXED** | shim list extended to `python3`/`python`; driver launched by absolute path; shims log `observe` vs `work` and record the caller from `/proc`, so a coverage export requested by Quoin is told from one requested by the driver; and a fourth check digests `target/assurance/` around the run, which does not depend on anyone having shimmed the right tool. Re-probed: the reviewer's injection now fails the test, and the regeneration probes now exit 2 |
| FND-302 | **FIXED** | the two runners are distinguished by the toolchain selector and both are required. Re-probed: deleting `test` from `ci:` now fails `tc_013` |
| FND-303 | **FIXED** | `KIND_OBLIGATIONS` requires the observation to support the label; an unlisted kind is refused. The check found a live mislabel in the shipped tree (`retained-historical-envelope-refused` declared `stale` while observing a schema refusal) and it is corrected. Re-probed: the relabelling now fails 2 cases |
| FND-304 | **FIXED** | `retained_floor` of 2,205 files and 44 envelopes, declared in the fixture and enforced in the census. A floor and not an equality, because retained evidence may only grow. Re-probed: the shrink now reports `below_retained_floor` and `matched: false` |
| FND-305 | **FIXED** | the chain requires the fields a coverage export declares. Re-probed: `{"junk":[1]}` now takes the chain to exit 1 |
| FND-306 | **FIXED** | the control observes intake's exit status |
| FND-307 | **FIXED** | the census is taken immediately after argument parsing. Re-probed: exit 2 |

Eleven FIXED, four ACCEPTED, two DEFERRED.

## Residuals

`FND-104` and `FND-109` are accepted properties of the tree as it stands rather than open work.
`FND-105` is agent-ix/quire-contract-ir#21 and `FND-107` is agent-ix/quire-contract-codegen#14. Both
are recorded as open unknowns in `assurance/change-assurance.json`, so they travel into every
verification receipt this repository asks for rather than living only in a review document.
