---
id: "SR-2526"
title: "code-review (source-backed SPEC allocation) of IR-675 FR-034 cross-role cause representation"
type: "SpecReview"
analysis: "code-review"
scope: "agent-ix/quire-contract-codegen@47dd57f79207eb4517e318752e316e696c9974bb; base 5ab6249658d9bd5f8977b95991b55cb9af1e7522; spec/kani/functional/FR-034-caller-death-ownership.md; spec/kani/matrix/TC-049-caller-death-ownership.md; spec/kani/matrix/tests.md"
review_set: "subset"
---

## Summary

Ticket: IR-675. Reviewer session dcb5e3e7-8fe4-422e-aef1-3ca57d78bee2, model claude-opus-5-5, run e86ea724-594f-49cb-ad01-8568cc4c64d8. Reviewed head 47dd57f79207eb4517e318752e316e696c9974bb against measured main 5ab6249658d9bd5f8977b95991b55cb9af1e7522; SPEC-only diff of three files. Method: code-review.

SPEC-only diff: no Rust or other code changed, so there is no Rust diff to review; the rust-review checklist was loaded before any source reading and applied only to judge the source claims the SPEC allocates. This artifact verifies those claims against agent-ix/quire-contract-codegen@8fbf08f907f31f7768c5644dfe17481b0684e361 (unmerged IR-639 WIP, read via git show only), the installed seccompiler 0.5.0 crate source and the installed rust 1.98.1 library source. No build, test, Cargo, Kani or probe was run.

Examined units (role examined):

- `FR-034#cross-role-L814-819` (spec/kani/functional/FR-034-caller-death-ownership.md:814-819)
- `FR-034#cross-role-L821-824` (spec/kani/functional/FR-034-caller-death-ownership.md:821-824)
- `FR-034#cross-role-L825-827` (spec/kani/functional/FR-034-caller-death-ownership.md:825-827)
- `FR-034#cross-role-table-row1-L829-831` (spec/kani/functional/FR-034-caller-death-ownership.md:829-831)
- `FR-034#cross-role-table-row2-L832` (spec/kani/functional/FR-034-caller-death-ownership.md:832)
- `FR-034#cross-role-table-row3-L833` (spec/kani/functional/FR-034-caller-death-ownership.md:833)
- `FR-034#cross-role-table-row4-L834` (spec/kani/functional/FR-034-caller-death-ownership.md:834)
- `FR-034#cross-role-table-row5-L835` (spec/kani/functional/FR-034-caller-death-ownership.md:835)
- `FR-034#cross-role-L837-842` (spec/kani/functional/FR-034-caller-death-ownership.md:837-842)
- `FR-034#cross-role-L843-846` (spec/kani/functional/FR-034-caller-death-ownership.md:843-846)
- `FR-034#cross-role-L847-851` (spec/kani/functional/FR-034-caller-death-ownership.md:847-851)
- `FR-034#cross-role-L853-858` (spec/kani/functional/FR-034-caller-death-ownership.md:853-858)
- `FR-034#cross-role-L860-864` (spec/kani/functional/FR-034-caller-death-ownership.md:860-864)
- `FR-034#cross-role-L865-868` (spec/kani/functional/FR-034-caller-death-ownership.md:865-868)
- `FR-034-AC-40#s1` (spec/kani/functional/FR-034-caller-death-ownership.md:1165)
- `FR-034-AC-40#s2` (spec/kani/functional/FR-034-caller-death-ownership.md:1165)
- `FR-034-AC-40#s3` (spec/kani/functional/FR-034-caller-death-ownership.md:1165)
- `TC-049#coverage-AC-40-L106` (spec/kani/matrix/TC-049-caller-death-ownership.md:106)
- `TC-049#step27-L655-660` (spec/kani/matrix/TC-049-caller-death-ownership.md:655-660)
- `TC-049#step27-L661-666` (spec/kani/matrix/TC-049-caller-death-ownership.md:661-666)
- `TC-049#step27-L667-672` (spec/kani/matrix/TC-049-caller-death-ownership.md:667-672)
- `TC-049#step27-L673-678` (spec/kani/matrix/TC-049-caller-death-ownership.md:673-678)
- `TC-049#step27-L679-681` (spec/kani/matrix/TC-049-caller-death-ownership.md:679-681)
- `TC-049#blank-L682-684` (spec/kani/matrix/TC-049-caller-death-ownership.md:682-684)
- `TC-049#blank-L755-758` (spec/kani/matrix/TC-049-caller-death-ownership.md:755-758)
- `TC-049#expected-AC-40-L781` (spec/kani/matrix/TC-049-caller-death-ownership.md:781)
- `tests.md#FR-034-row-L54` (spec/kani/matrix/tests.md:54)
- `tests.md#TC-049-row-L85` (spec/kani/matrix/tests.md:85)

Context-only units:

- `FR-034#abi-scope-L659-664` (spec/kani/functional/FR-034-caller-death-ownership.md:659-664)
- `FR-034#admission-original-cause-L789-794` (spec/kani/functional/FR-034-caller-death-ownership.md:789-794)
- `FR-034#mapping-row-L1086` (spec/kani/functional/FR-034-caller-death-ownership.md:1086)
- `TC-049#slice-terms-L35-37` (spec/kani/matrix/TC-049-caller-death-ownership.md:35-37)

Not run, by policy: Cargo, build, tests, full make ci, Kani, runtime probes, strict matrix re-run. All CODE/runtime criteria remain PLANNED/UNRUN; this review gives no CODE or proof credit.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-034 lines 855-856 and AC-40 require C to preserve the standard From<TryReserveError> conversion's 'already absent source', and TC-049 step 27 states that conversion 'drops the source', but the installed authority documents this as unstable behaviour: alloc/src/io/error.rs:238-240 says 'TryReserveError won't be available as the error source(), but this may change in the future.' Scenario: A later toolchain, as its own doc permits, keeps the TryReserveError as the converted error's source. A producer using `?` then yields OutOfMemory with a source; the SPEC simultaneously requires preserving an 'already absent' source and forbids inventing one, and step 27's Analysis records a fact the toolchain contradicts. Keying the requirement on the producer's observed source presence rather than on a toolchain property avoids this. | spec/kani/functional/FR-034-caller-death-ownership.md:855-856; spec/kani/functional/FR-034-caller-death-ownership.md:1165; spec/kani/matrix/TC-049-caller-death-ownership.md:661-662; alloc/src/io/error.rs:238-240 (rust 1.98.1) |

## Verdict

Verified: creator.rs:146-157 (WIP) maps both try_reserve_exact failures through io::Error::other, giving Other, raw_os_error None and a boxed TryReserveError payload; startup_cause.rs:97-128 (WIP) keeps kind/errno, validates OS-derived kinds and projects raw-None to io::Error::from(kind) with KindOnly (the CODE gap the SPEC names); startup_cause.rs:174-186 lists exactly the seven no-payload variants plus Reservation(TryReserveError); outer_sampling.rs:1090-1091 wraps RepresentationError in SamplingError::Observation(io::Error::other(..)); seccompiler 0.5.0 src/lib.rs:228-243 has Backend, EmptyFilter, Prctl, Seccomp, ThreadSync and feature-gated JsonFrontend, and the WIP Cargo.toml uses default-features = false; alloc/src/io/error.rs:236-246 implements From<TryReserveError> as ErrorKind::OutOfMemory.into(); alloc/src/collections/mod.rs:71-110 keeps TryReserveError::kind and TryReserveErrorKind #[unstable(try_reserve_kind)]. The SPEC's typed-discriminant, no-message-parsing and explicit-loss stance matches the rust-review error idioms. One medium finding on a std behaviour the SPEC treats as fixed.

## Dispositions

Round 1, reviewed at agent-ix/quire-contract-codegen@16572f234284384a6117ea15b9ee0aed5aa204b5 (fix diff 47dd57f..16572f2; source fix commit 16572f234284384a6117ea15b9ee0aed5aa204b5). Reviewer session dcb5e3e7-8fe4-422e-aef1-3ca57d78bee2, model claude-opus-5-5, run a5df61d0-d398-40ac-b2a5-5eacdbacb452. Changed lines re-checked for regressions of each finding.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 16572f2: The SPEC no longer treats source absence as a timeless property of From<TryReserveError>: C preserves the actual resulting kind and observed source presence/absence and must not infer absence from the conversion's name; TC-049 step 27 reads the selected toolchain's source-presence contract without assuming every version drops the source. |
