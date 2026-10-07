---
id: "SR-2523"
title: "spec-failure-domain-analysis of IR-675 FR-034 cross-role cause representation"
type: "SpecReview"
analysis: "failure-domain"
scope: "agent-ix/quire-contract-codegen@47dd57f79207eb4517e318752e316e696c9974bb; base 5ab6249658d9bd5f8977b95991b55cb9af1e7522; spec/kani/functional/FR-034-caller-death-ownership.md; spec/kani/matrix/TC-049-caller-death-ownership.md; spec/kani/matrix/tests.md"
review_set: "subset"
---

## Summary

Ticket: IR-675. Reviewer session dcb5e3e7-8fe4-422e-aef1-3ca57d78bee2, model claude-opus-5-5, run e86ea724-594f-49cb-ad01-8568cc4c64d8. Reviewed head 47dd57f79207eb4517e318752e316e696c9974bb against measured main 5ab6249658d9bd5f8977b95991b55cb9af1e7522; SPEC-only diff of three files. Method: spec-review/spec-failure-domain-analysis.

Unstated failure modes in capturing, encoding, transporting and projecting a cross-role cause: OS errno mismatch, no-payload kinds, finite typed variants, Other-wrapped and Reservation-wrapped TryReserveError, other opaque payloads, diagnostic capture failure, malformed/unauthenticated custody, and refusal precedence. Source context is read-only at agent-ix/quire-contract-codegen@8fbf08f907f31f7768c5644dfe17481b0684e361 (unmerged IR-639 WIP, read via git show only) and installed rust 1.98.1 library source.

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
| FND-001 | medium | The section lists the diagnostic-capture failures ContextExceeded and Formatting as reconstructible cross-role cause variants (lines 838-839) yet requires that diagnostic truncation or formatting failure not supply a cause or classification (lines 862-864); it does not state whether an over-bound or failing Display during capture is truncated (keeping the original typed cause) or refuses, nor which cause then reaches the public refusal. Scenario: A seccompiler::Error::Seccomp(EPERM) whose Display exceeds CONTEXT_BYTES, or a Display impl returning fmt::Error, is captured. In the WIP producer, capture_io/capture_seccompiler return Err(RepresentationError::ContextExceeded/Formatting) after the typed cause was already captured, so the transported cause becomes the representation error and EPERM is lost: diagnostic size selects the public cause, which lines 862-864 forbid but the variant list at 838-839 sanctions. Another implementer truncates and keeps EPERM; both cite the spec. | spec/kani/functional/FR-034-caller-death-ownership.md:862-864; spec/kani/functional/FR-034-caller-death-ownership.md:838-839; src/kani/run/startup_cause.rs:292-299,314-336,338-354 (WIP) |
| FND-002 | low | Row 2 requires retaining and reconstructing 'the actual ErrorKind' of a no-errno, no-payload I/O error, but io::ErrorKind is #[non_exhaustive] and a finite cross-role encoding can only name the kinds it knows; the spec gives no outcome for a no-errno kind the encoding cannot name. Scenario: After a toolchain update stabilizes InProgress or TooManyOpenFiles (both #[unstable] in the installed 1.98.1 core/src/io/error.rs), a dependency returns io::Error::from(ErrorKind::InProgress) with no errno across a role. The WIP encoder maps this to RepresentationError::UnnamedIoKind. One implementation refuses with that representation error replacing the original cause; another projects to Other; the spec permits neither explicitly, so they disagree on the public cause. | spec/kani/functional/FR-034-caller-death-ownership.md:832; core/src/io/error.rs:753 (#[non_exhaustive] ErrorKind; InProgress/TooManyOpenFiles #[unstable]) |

## Verdict

Errno mismatch refuses with no fabricated errno; malformed or unauthenticated custody refuses through existing typed paths without Dispatch or evidence, retaining owners and original deadlines; diagnostic absence cannot select a mapping; opaque payload loss is explicit. Two gaps: what happens when capturing the diagnostic for an otherwise representable cause fails, and what happens for a no-errno ErrorKind the finite encoding cannot name.
