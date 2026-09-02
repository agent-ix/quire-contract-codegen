---
id: SR-005
title: Drop legacy evidence code review
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen#16, branch chore/drop-legacy-evidence; the deletion of the retained evidence tree and everything that served it, and the disposition of every finding from the independent adversarial review"
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

The per-filename census, run across `src/ scripts/ tests/ assurance/ examples/ Makefile Cargo.toml
build.rs .github/` at `origin/main`:

| Schema | Hits | Where |
|---|---|---|
| `pgm01-derivation-evidence-envelope-v1` | **11** | `src/oracle.rs:35` (`include_bytes!` as `PGM_SCHEMA`); `tests/oracle_generation.rs` ×3 including the `executable_digest` input at :101; `tests/harness_generation.rs:164`; `tests/strategy_generation.rs:103`; plus the pins register and prose |
| `generated-rust-oracle-v1` | **6** | `src/oracle.rs:37` (`include_bytes!`); `tests/oracle_generation.rs` ×3; the pins register; the live-schema assertion |
| `oracle-source-map-v1` | **5** | `src/oracle.rs:36` (`include_bytes!`); `tests/oracle_generation.rs` ×2; the pins register; the live-schema assertion |
| `foundation-evidence-input-v1` | **2** | `assurance/pins.json:50` (the freeze register) and `tests/shared_assurance.rs:1021` (the freeze test's own digest pin). **Zero executable consumers** |
| `foundation-evidence-manifest-v1` | **2** | `assurance/pins.json:55` and `tests/shared_assurance.rs:1025`. **Zero executable consumers** |

The two deleted schemas have exactly one hit in the register that records the freeze and one in the
test that enforces it, and nothing else — the signature of an artifact kept alive only by the
bookkeeping about it. The three kept schemas are each loaded at compile time by the generator.

Separately, all 15 distinct `include_str!`/`include_bytes!` targets in the crate were resolved
against the tree at the final head; every one exists. No include site was orphaned by the deletion.

## Findings

Fourteen from the independent adversarial review of `00aa054`, plus three raised by the campaign
coordinator against the *fix* rather than the deletion, plus two this repository found in its own
correction.

Severities are the reviewers'. Three were graded **critical** by a reviewer and are recorded as
`high` here because the SpecReview vocabulary is low/medium/high — FND-501 and FND-502 from the first
round, both live references to deleted material that also made `make ci` fail, and FND-701 from the
re-review, which found the reference census still defeatable after it was reported fixed. The grade
is stated here rather than quietly lost.

The FND-7xx block came from an independent re-review commissioned on the exact final head *because*
the first round's fixes had themselves introduced defects. It found two blockers.

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
| FND-519 | high | `collect_sources` filtered on extension, and `Makefile` has no extension — so the reference census never saw the one file a Make target can live in. Appending the deleted `compat-view` target verbatim left the census green. `.yaml` was also absent, and GitHub accepts `.github/workflows/*.yaml` | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-520 | medium | three claims corrected everywhere a reader looks were left stale in `assurance/change-assurance.json`, which is the file that gets sealed and travels into the receipt | `assurance/change-assurance.json` | correct-requirement-no-evidence |
| FND-521 | medium | the re-derived floor was total-only. `scripts` and `tests` are five files each and `src` is four, so a whole directory could vanish and move the total by less than ordinary churn | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-522 | high | the first per-directory guard read its floors from a hardcoded list, so deleting an entry removed the directory from the check and left the test green. A guard built from the same list the walk uses cannot catch that list shrinking | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-523 | low | the floor derivation credited `.yaml` with part of the population increase; both added files are extensionless | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-524 | medium | the census counted from the directory walk, so untracked scratch could inflate the population and restore headroom that real deletions had consumed. `proptest-regressions/*.txt` and a stray root `.json` are both admitted by the extension filter | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-525 | low | the floor comment stated a margin as though it were derived, when no rule fixes it | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-526 | high | the census filter was an allow-list, so it was blind to an extensionless `scripts/reintroduced_reader` and to a `.yaml` one. Naming `Makefile` back by hand fixed one file and left the class | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-527 | medium | the untracked half of the scan had no probe. Narrowing `sources` to tracked files left every assertion passing — a property that had already been lost once could be lost again silently | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-701 | high | the byte-range exemption was still a whole-file exemption. The census matched with `find`, which returns the *first* occurrence only, and every forbidden name occurs first inside the exempt declarations — so this file was never examined past them. Six stale references appended to the end were measured green | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-702 | high | the untracked-scan control was a tautology for the third time. `scanned` was recorded at the top of the loop, so a narrowing placed between there and the check that matters left the census blind and the control green | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-703 | medium | `inspected` counted 30, not the 31 the comment and the assert message both claimed, because the change-declaration exemption ran before the increment. The real margin was 3, not 4 — the same class of defect the commit before it set out to remove | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-704 | high | the new probe's line condition was inert. `adapt_conformance` decodes one line at a time, so the decoder always reports "line 1" and a bare substring test for it is free. An adapter naming no row, and one naming a fixed wrong row, both passed | `scripts/assurance_chain.py` | correct-requirement-no-evidence |
| FND-705 | medium | SR-005's mutation table credited the reference census with a red that was actually produced by the `make -n ci` plan assertion. The census did not see that defect | `reviews/SR-005-…` | wrong-requirement |
| FND-706 | medium | `subject.scope` gained `evidence` but still omitted `reviews`, `README.md` and `CLAUDE.md`, all touched by this change. One instance of FND-506 was fixed out of four | `assurance/change-assurance.json` | correct-requirement-no-evidence |
| FND-707 | medium | TC-012 gained a procedure step — that each control could fail independently of its scenario — which nothing implements | `spec/test/TC-012-outcome-vocabulary.md` | correct-requirement-no-evidence |
| FND-708 | low | the TC-001 trace comment and SR-006 claimed "both output schema digests"; only the first output's is asserted | `tests/oracle_generation.rs`, `reviews/SR-006-…` | correct-requirement-no-evidence |
| FND-709 | low | the census probe was not removed when the loop panicked, and was not gitignored — a failing gate littering the tree with a file `git add -A` would commit | `tests/shared_assurance.rs`, `.gitignore` | correct-requirement-no-evidence |
| FND-710 | medium | SR-005 and SR-006 carried a stale claim boundary and superseded measurements | `reviews/SR-005-…`, `reviews/SR-006-…` | wrong-requirement |
| FND-711 | low | AA-001's Sufficiency Decision **redefined** "retained measurements" onto a weaker referent while `planning/release-decision.md` struck the identical item, and neither disclosed the inconsistency | `spec/assurance/AA-001-codegen-argument.md` | wrong-requirement |
| FND-801 | high | one non-UTF-8 byte made a file invisible to the census *and* to the count. `read_to_string` returned `Err` and the loop `continue`d — an unnamed silent exclusion that no deny-list entry covers, already dropping three `__pycache__` files, and eroding the floors at the same time | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-802 | high | `gone` named the reader by filename, so a Python import of the same module — the most likely form of reintroduction — spelled it without the suffix and passed | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-803 | high | the undecodable-row probe pinned rows 1 and 2 of a nine-row stream, so an adapter enforcing only `number <= 2` and dropping the rest passed all eight probes while silently transcribing 8 entries from 9 | `scripts/assurance_chain.py` | correct-requirement-no-evidence |
| FND-804 | medium | nothing asserted the census probe was untracked. Committing it restored the FND-528 defect exactly | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-805 | medium | the probe carried one of nine forbidden names, so `gone` could be reduced to that one entry and the control still agreed — FND-522's shape, fixed for directories and left standing for names | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-806 | medium | the change-declaration exemption compared a bare basename, so any `change-assurance.json` anywhere in the tree bought a whole-file exemption | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-807 | medium | SR-005 and SR-006 still carried superseded census figures and a claim boundary naming a parent commit — which is what FND-710 records as fixed | `reviews/SR-005-…`, `reviews/SR-006-…` | wrong-requirement |
| FND-808 | low | the FND-708 correction was appended rather than substituted, leaving a sentence asserting both readings | `reviews/SR-006-…` | wrong-requirement |
| FND-809 | low | the exempt span ran ~131 lines and covered executable statements, so a live invocation of the deleted reader inserted among them was exempt | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-810 | low | the Makefile directive scan reads one file, so `.IGNORE:` reached through an `include` left the test green while `make` swallowed a failing recipe | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-901 | high | the adapter could validate every row and then drop one before the transcript. All eight probes stayed matched and the chain stayed green while eight entries of nine were sealed. The count that would have caught it was printed in `accepts-the-real-run`'s detail and compared to nothing | `scripts/assurance_chain.py` | correct-requirement-no-evidence |
| FND-902 | medium | `override SHELL :=` split to the target `"override SHELL"` and bypassed the SHELL guard. `override` is this Makefile's own spelling for six variables | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-903 | low | the no-`include` assertion matched `"include "` with a trailing space; make honours `include<TAB>file.mk`, which put `.IGNORE:` back through an include | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-904 | medium | FND-801 was fixed for unreadable files and not for unreadable directories; `collect_sources` still returned silently. A FIFO also blocked the walk with no bound | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-905 | medium | the stem rule was applied to one of eleven names. The deleted test spelled as a Rust function name, the two schemas spelled by stem, and the underscore forms were all invisible | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-906 | medium | SR-005 and SR-006 described code that does not exist at this head — an allow-list and a `scanned` binding, both superseded — plus one stale figure pair and one wrong count. Third consecutive round | `reviews/SR-005-…`, `reviews/SR-006-…` | wrong-requirement |
| FND-907 | low | the untracked control's disposition claimed "nothing can narrow the census while the control still agrees"; the probe is identifiable by its literal path, so a narrowing that excepts it passes | `reviews/SR-005-…` | wrong-requirement |
| FND-908 | low | `from_utf8_lossy` fixed the stray-byte case and not the whole-file-encoding case; a UTF-16 live import matched nothing | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-811 | low | `subject.scope` is a strict superset of the diff — `examples`, `plan` and `src` are declared and untouched | `assurance/change-assurance.json` | wrong-requirement |
| FND-528 | high | the control written to close FND-527 was itself a tautology, in two successive forms. It asserted a *fresh* `collect_sources` call could see an untracked file, which is a fact about the walker; narrowing the census left it green. Rewritten to assert on `sources`, it was still green, because anything narrowing the set between collection and use slips underneath | `tests/shared_assurance.rs` | correct-requirement-no-evidence |

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
| FND-516 | **FIXED**. Floor re-derived from the tree as it stands: `>= 27` against a measured 31, with the arithmetic in the comment and every input taken from the walk |
| FND-517, FND-518 | **FIXED**. Both were defects in the FND-503..FND-509 fixes and both were caught by `make ci` before merge |
| FND-519 | **FIXED**. `collect_sources` was changed to see the Makefile. It has since become a deny-list — everything is scanned unless it is non-text or a generated lock/licence — which subsumes the extensionless and `.yaml` cases the first fix handled by name. Relayed by the campaign coordinator from a sibling's reviewer; reproduced here before fixing, and mutation-tested after |
| FND-520 | **FIXED**. See "The sealed record, re-read last" above |
| FND-521 | **FIXED**. A per-directory guard was added alongside the total, then rebuilt on discovery after probing found the defect below |
| FND-522 | **FIXED**. The first per-directory guard read its floors from a hardcoded list and looked each directory up with `unwrap_or(0)`. Deleting the `scripts` entry from that list left `tc_013` green. The guard now compares the **discovered** directory set against a declared one with `assert_eq!` before any floor is applied, so a directory that stops being walked and an entry removed from the declaration both land on the same assertion |
| FND-526 | **FIXED**. The filter is now a deny-list: everything is scanned unless it is non-text or a generated lock/licence, each exclusion named individually and the count in the comment taken from the array length so it cannot drift. Probed: an extensionless reader, a `.yaml` reader and a `Makefile` target all go red |
| FND-527, FND-528 | **FIXED**. The control writes an untracked `scripts/.census-probe.py` carrying every forbidden name and requires the census's own `match_indices` branch to detect each one — the detection path itself, with no separate accounting to keep in step. Two weaker forms preceded it, a second walk and an assertion on the collected `sources`, and both were measured green while the census was blind. Probed red two ways: narrowing the set between collection and use, and narrowing `collect_sources` itself to tracked files. It is not absolute: the probe is identifiable by its literal path, so a narrowing written to except it would pass |
| FND-524 | **FIXED**. Scanned broadly, counted narrowly. The reference scan still runs over tracked *and* untracked files, because untracked is exactly the state a reintroduced reader is in before anyone `git add`s it; the floors count `git ls-files` only, so scratch cannot vote on whether the census is large enough to prove anything. Probed in both directions |
| FND-525 | **FIXED**. The margin of 4 is now stated as chosen rather than derived, with the reason it is safe: whole-directory loss is caught by the set comparison and the per-directory floors, not by this number. Printing arithmetic that does not close is the failure this avoids |
| FND-701 | **FIXED**. The census matches with `match_indices` and judges every occurrence on its own. Probed with the reviewer's own mutation — six stale references appended to the end of the file: **red** |
| FND-702 | **FIXED**, and the form changed rather than the placement. The probe is now an untracked file **carrying a real deleted name**, and it is detected by the same `match_indices` branch that panics on every other file — there is no separate accounting to keep in step, so nothing can narrow the census while the control still agrees. Probed with the reviewer's mutation: **red** |
| FND-703 | **FIXED**. `inspected` increments where the file is read, before the change-declaration exemption, so it is the walked tracked count. Measured 31, matching the printed derivation |
| FND-704 | **FIXED**. The probe truncates two different rows in two runs and requires each refusal to carry the adapter's own sentence for *its* row and not the other's. Matching the adapter's sentence rather than a bare `line N` was itself necessary — the decoder's appended message always says "line 1", which is the same trap one layer down. Both reviewer mutations, no-row and fixed-wrong-row, are now **red**, as is dropping the row silently |
| FND-705, FND-710 | **FIXED**. The mutation row is re-attributed to the assertion that actually produced the red, and the stale figures and claim boundary are corrected |
| FND-706 | **FIXED**. `reviews`, `README.md`, `CLAUDE.md` and `.gitignore` added to `subject.scope`, which now covers every top-level path in `git diff --name-only origin/main..HEAD`. The first pass of this fix added three of the four and missed `.gitignore` — a fourth instance of the same one-at-a-time habit the finding was about, caught by checking the declaration against the diff rather than against the finding's list |
| FND-707 | **FIXED**. The unimplemented clause is removed rather than left declared. Adding an independence requirement immediately after a finding about a control that could not fail independently was the wrong direction, and implementing it was not in scope for this change |
| FND-708 | **FIXED** in both places |
| FND-709 | **FIXED**. An RAII drop guard removes the probe even when the loop panics, and `.gitignore` names it. Verified: after a deliberately failing run the file is absent |
| FND-711 | **FIXED**. AA-001 now strikes the item exactly as `planning/release-decision.md` does, and says so |
| FND-801 | **FIXED**. Files are read as bytes and decoded with `from_utf8_lossy`; a file the filter admitted but that cannot be *read* now panics rather than being skipped. Probed: the same reader invocation with one `\xe9` in a comment is red |
| FND-802 | **FIXED**. `gone` carries the module **stem**, which subsumes the suffixed spelling. Probed: a Python import of the stem is red |
| FND-803 | **FIXED**. The probe truncates **every** row in turn — nine runs — and requires each refusal to name that row and no other. Probed: an adapter enforcing only rows 1–2 now reports 2/9 and the chain goes red; so does a fixed wrong row |
| FND-804 | **FIXED**. One assertion that the probe is untracked, immediately after the `git ls-files` block. Probed with the reviewer's exact mutation: red |
| FND-805 | **FIXED**. The probe carries every name in `gone`, the control requires each to have been matched, and `EXPECTED_GONE` is an independent second statement of the list that a single-line deletion lands on. It does not defend against a coordinated edit of both, and the comment says so — nothing in one file can |
| FND-806 | **FIXED**. Compared by full path |
| FND-807, FND-808 | **FIXED**. Figures corrected to 31 / floor 27; the schema-digest sentence is substituted rather than appended. The claim boundary now names the branch and states the one-commit lag explicitly, because a document cannot name the commit that contains it and naming a superseded parent is worse than naming none |
| FND-809 | **FIXED**. Four disjoint ranges over the literal declarations, with an ordering assertion. Probed: a live invocation inserted where the old span reached is red. Prose in this file is no longer exempt either, which caught four of my own comments |
| FND-810 | **FIXED**. The Makefile is asserted to declare no `include`. Probed: `.IGNORE:` reached through one is red |
| FND-811 | **ACCEPTED**. `subject.scope` declares the *subject area* of the record, not a changelog. `examples`, `plan` and `src` are in scope for a change to this repository's assurance surface whether or not this particular diff touched them, and under-declaring was the finding — over-declaring costs nothing and survives a rebase |
| FND-901 | **FIXED**. `adapt_conformance` refuses a transcript shorter than its input, and `accepts-the-real-run` now compares the entry count it was already printing. Probed with the reviewer's mutation — every row validated, row 5 dropped: the chain exits non-zero |
| FND-902 | **FIXED**. `override ` and `export ` are stripped before the target comparison. Probed: red |
| FND-903 | **FIXED**. Matched on the directive word via `split_whitespace`, so any whitespace separator counts. Probed with a tab: red |
| FND-904 | **FIXED**. `read_dir` failure panics with the same message shape as the file arm, and non-regular files are skipped by `file_type` so a FIFO cannot block the walk. Probed: a live import inside a chmod-000 directory is red |
| FND-905 | **FIXED**. Suffixes stripped from the schema entries and both hyphen and underscore spellings carried, 14 names in place of 11. Probed: a workflow running the deleted test by its Rust function-name spelling is red. The fix also found its own drift — the exempt-range marker embedded the array length, so changing the count silently moved the range, and the ordering assertion caught it |
| FND-906, FND-907 | **FIXED**. The three superseded dispositions now describe the deny-list and the detection-path control that actually exist; the floor row reads 31 → 26; the range count reads four; and the over-claim is narrowed to what was measured |
| FND-908 | **FIXED**. Files whose bytes carry interleaved NULs are refused as a wide encoding rather than scanned as though empty. Probed with UTF-16: red. It also caught three `__pycache__/*.pyc` files the old silent skip had been dropping, which are now deny-listed with `__pycache__` itself |
| FND-523 | **FIXED**. The floor derivation credited `.yaml` with part of the population increase. Measured: both added files are `Makefile` and `.gitignore`, and `.yaml` adds nothing here because this repository's only workflow is `.yml`. It is admitted so a rename cannot carry a workflow out of the census, and it is no longer credited with any of the 2 |

## Assurance Context

**Claim boundary.** That this repository no longer retains evidence, that nothing surviving needs
what was removed, and that no claim resting on the deleted records was restated more weakly. It is a
claim about the final head of `chore/drop-legacy-evidence` and about nothing else. That head is recorded in the merge comment on PR #17 rather than here: a document cannot name the commit that contains it, and three earlier drafts of this line named a parent that had already been superseded, which is worse than naming none.

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

## Gate results at the final head

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

The same rule was applied to every other check this change added or repointed. Each was given the
specific defect it exists to catch, and each was observed to go red:

| Check | Defect introduced | Result |
|---|---|---|
| `tc_013` reference census | the FND-501 stale `legacy_evidence_view.py` put back in the `make -n ci` list | **red** |
| `tc_013` reference census | the deleted `compat-view` target appended to the `Makefile` | **red** — and **green** before FND-519 was fixed, which is how FND-519 was found |
| `tc_013` reference census | a `.github/workflows/probe.yaml` naming the deleted reader | **red** |
| `tc_013` per-directory floor | two files moved out of `scripts/`, 5 → 3 | **red**, via the directory floor |
| `tc_013` directory-set guard | the `scripts` entry deleted from the declared set | **red** — and **green** against the first version of the guard, which is how FND-522 was found |
| `tc_013` total floor | the whole of `scripts/` moved out (30 → 24) | **red** |
| `tc_013` removed-name check | an **untracked** `scripts/legacy_evidence_view.py` written back, never `git add`ed | **red** |
| `tc_013` tracked-only counting | two real files removed from `scripts/`, then four untracked scratch `.json` added to restore the count | **red** — the scratch does not count, which is the point |
| `tc_013` reference census | an **extensionless** `scripts/reintroduced_reader` naming the deleted reader | **red** — **green** under the allow-list, which is how FND-526 was found |
| `tc_013` reference census | `scripts/reintroduced_reader.yaml` naming the deleted reader | **red** |
| `tc_013` untracked-scan control | the census set narrowed between collection and use | **red** — **green** against both earlier forms of the control, which is how FND-528 was found |
| `tc_013` untracked-scan control | `collect_sources` itself narrowed to tracked files | **red** |
| `make pins` digest check | one byte appended to `engineering_assurance/compatibility.py` | **exit 1**, naming the digest |

It counts no verification state, so it does not put `malformed` back in the census by way of an error
message. It closes a real gap instead: the `JSONDecodeError` arm of `adapt_conformance` was reached by
no case before it.

## The sealed record, re-read last

`assurance/change-assurance.json` is the one file in this change that is **sealed and travels into
the verification receipt**, and the census that would otherwise catch a stale reference in it exempts
it by design — because it is also the file whose job is to name what was deleted. So it was re-read
last, line by line, against every claim corrected elsewhere. Three were stale:

| Claim | Was | Now |
|---|---|---|
| `PRESERVE-planned-matrix` | "The rows this change adds for FR-006 are the only rows it claims" | This change adds no row. It **removes two** — the FR-006-AC-4 row and the TC-011 row — and says so |
| `UNKNOWN-stacked-branch-divergence` | "This change supersedes PRs #9, #10 and #12 … machinery this change deletes" | **Removed.** It was an accepted disposition of the issue-13 migration record. Once the record id moved to issue-16 its "this change" named the wrong change, and carrying it forward would have sealed a false claim |
| `UNKNOWN-make-is-not-a-trust-root` | "Measured on this repository **at this candidate revision**" | attributed to the revision it was actually measured at (`bbd5e67`, issue #13), with the reason the counts still describe this tree: `ci` keeps the same eleven prerequisites and none of the seven that failed |

The `#14` measurement itself is untouched and no execution-control guard was re-added, per the owner
decision recorded there. Correcting who a measurement belongs to is not restating it.

## What a fix stops catching

Three of the findings above — FND-519, FND-524 and FND-528 — were introduced *by* the fix for the
finding before them. The pattern is the same each time, and it is worth stating as the rule this
review ended up working to:

> **Each fix was checked for what it started catching and not for what it stopped catching.**

- The allow-list filter was extended to admit `Makefile` by name. That closed the one file and left
  the class: an extensionless `scripts/reintroduced_reader` was still invisible. Replaced with a
  deny-list.
- Counting was moved off the walk to fix scratch inflating the population. That silently stopped the
  census seeing untracked files at all — the state a reintroduced reader is in first. Split into a
  broad scan and a narrow count.
- A control was added for that property. It re-derived its own answer with a second walk, so it
  verified the walker rather than the census. Rewritten against `sources`, it still missed a
  narrowing between collection and use. Only asserting on `scanned`, recorded inside the loop at the
  moment each file is read, actually holds.

Every check in the table above was run against the specific defect it exists to catch, and **eighteen**
of them were green until they were not — three found by the first fix cycle and three more by the
independent re-review of the fixes themselves (FND-701, FND-702, FND-704). The re-review's summary of
its own finding is the sharpest statement of it:

> The two checks that carry TC-013 — the reference census and its untracked-scan control — are both
> still defeatable, and both were reported as fixed.

That is the reason this document lists the mutation for every check rather than asserting that the
checks work.

## Empty populations

The clause that changed shape is TC-013's. It previously asserted the frozen schemas were *present*
and referenced by nothing; it now asserts two schemas are *absent*. An assertion of absence over a
population that was just deleted is satisfied by a repository that deleted everything, so it cannot
carry the criterion alone.

It does not. Three live schemas are asserted **present and still named by `src/oracle.rs`** — a
non-empty positive population — and the reference census runs over 31 tracked non-markdown readable
files against a re-derived floor of 27. The deleted-schema clause is a corroborating check, not the
load-bearing one.
