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

## Dispositions

Round 1, reviewed at agent-ix/quire-contract-codegen@16572f234284384a6117ea15b9ee0aed5aa204b5 (fix diff 47dd57f..16572f2; source fix commit 16572f234284384a6117ea15b9ee0aed5aa204b5). Reviewer session dcb5e3e7-8fe4-422e-aef1-3ca57d78bee2, model claude-opus-5-5, run a5df61d0-d398-40ac-b2a5-5eacdbacb452. Changed lines re-checked for regressions of each finding.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 16572f2: The section now orders typed-cause retention before optional diagnostic capture, requires omitting detail on over-bound/formatting failure without substituting a context/formatting error, and separates required-representation failures from optional diagnostic failure; TC-049 step 27 lines 678-680 and the AC-40 row check it. |
| FND-002 | fixed | 16572f2: An unnameable no-errno kind now has a defined outcome: the sender stops the cause transaction without Other normalization and C refuses the actual transport/representation failure without fake replay, evidence or Dispatch. The rule introduced a new contradiction, recorded as FND-003 below. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | The round-1 unnamed-kind rule makes the public classification of a listed cross-role admission failure depend on its ErrorKind: when the sender cannot name a no-errno kind it stops the cause transaction, and C must report the transport/representation failure through Tool(KaniToolError::Io) or another control mapping and may not publish it as a MemoryMechanismUnavailable cause; this contradicts line 793 ('classify the admission failure as BoundedLaunchError::Unavailable at its site regardless of original errno/io::ErrorKind') and line 849 ('No public refusal variant/code, errno-independent admission ... is changed'). Scenario: An I-role PrivateProc admission check fails with a no-errno io::Error whose ErrorKind the finite encoding cannot name (for example a kind stabilized by a later toolchain, such as InProgress). Under lines 873-881 the caller receives Tool(KaniToolError::Io) or a control-failure mapping. Under lines 791-793 and 849-850 the same admission failure must be MemoryMechanismUnavailable with admission CapabilityUnavailable { PrivateProc }, regardless of kind. An implementer cannot satisfy both, and the AC-40 mutant oracle cannot say which classification is correct. Neither section says whether C, which already holds the authenticated role/operation, may still publish Unavailable with an explicitly unrepresented cause. | spec/kani/functional/FR-034-caller-death-ownership.md:873-882; spec/kani/functional/FR-034-caller-death-ownership.md:791-793; spec/kani/functional/FR-034-caller-death-ownership.md:849-850 |

## Dispositions (round 2)

Round 2, reviewed at agent-ix/quire-contract-codegen@a0c0e33a4a938787616e687198749e5f6f8233b4 (fix diff 16572f2..a0c0e33). Reviewer session dcb5e3e7-8fe4-422e-aef1-3ca57d78bee2, model claude-opus-5-5, run 26449637-58f7-48fd-a392-c34b3dd68c3e. Only FND-003 was open; FND-001 and FND-002 latest outcomes remain fixed 16572f2 and were re-checked against the changed lines for regressions: none.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-003 | fixed | a0c0e33: The unnamed-kind contradiction is removed: the no-errno producer/build domain is closed to the 39 named stable ErrorKinds with a producer-side CODE gate before delivery for any newly emitted kind (lines 892-905), and a supported genuine admission cause keeps the authenticated site-first Unavailable mapping regardless of kind/errno (lines 906-909); faulty metadata may not reclassify a genuine admission failure as Tool (lines 915-916). This matches line 793 and line 849. The fix introduced a residual gap, recorded as FND-004. |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | The round-2 integrity-fault rule keeps a genuine admission failure on Unavailable when its cause metadata is unknown or malformed, forbids fabricating an original io::Error and forbids claiming the integrity error is the original, and defers to 'existing ... integrity-failure mappings' that FR-034 does not define (the four uses of 'integrity' in FR-034 are all in this new paragraph); MemoryMechanismUnavailable carries a mandatory cause: std::io::Error documented as the original cause, so no rule says which io::Error that refusal carries in this case. Scenario: I reports a PrivateProc admission failure whose authenticated site resolves to CapabilityUnavailable { PrivateProc }, but its cause packet carries an unknown kind code. Line 915 keeps Unavailable, so C must build MemoryMechanismUnavailable { cause, admission }. It may not fabricate an original io::Error (line 917) or present the integrity error as the original (line 918). One implementer uses io::Error::new(InvalidData, integrity_fault) as the cause, which a caller reads as the original cause; another routes to the protocol-failure row 1170 ('otherwise ordinary I/O Tool refusal'), which line 916 forbids. The AC-40 oracle cannot tell which is correct, and no named mapping exists to settle it. | spec/kani/functional/FR-034-caller-death-ownership.md:911-920; spec/kani/functional/FR-034-caller-death-ownership.md:1169-1170 |

## Dispositions (round 3)

Round 3, reviewed at agent-ix/quire-contract-codegen@3225e12a45e54c379bddeb4c0e5ee93ae19a6869 (fix diff a0c0e33..3225e12). Reviewer session dcb5e3e7-8fe4-422e-aef1-3ca57d78bee2, model claude-opus-5-5, run 53bac61e-de87-45d6-a1e6-f89aee04777d. Only FND-004 was open; FND-001 and FND-002 (fixed 16572f2) and FND-003 (fixed a0c0e33) were re-checked against the changed lines for regressions: none.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-004 | fixed | 3225e12: The undefined 'existing integrity-failure mapping' is replaced by a defined rule: an unknown/malformed/incomplete cause packet at an independently authenticated negative admission site yields MemoryMechanismUnavailable with that site's admission context and the actual local io::Error::new(InvalidData, CauseMetadataIntegrityError), raw errno None, explicitly not the original producer cause, with no loss marker (lines 911-926, mapping row 1193); an unauthenticated site takes the existing startup/protocol path (lines 928-938). AC-40, TC-049 step 27 and the Expected Results row assert the mapping. The fix introduced a residual public-detectability gap, recorded as FND-005. |

## New findings (disposition pass 3)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | The round-3 integrity cause is io::Error::new(InvalidData, CauseMetadataIntegrityError) with a private payload type, and the public rustdoc must tell callers not to read it as an original producer error (lines 967-971), but no typed public way to recognise it is allocated: unlike projection loss, which callers detect through get_ref()/downcast_ref::<KaniCrossRoleCauseLoss>() without parsing Display (lines 866-868), the integrity payload cannot be downcast by callers, so the only distinguishing signal is the diagnostic Display that the spec forbids as a semantic API. Scenario: Two MemoryMechanismUnavailable results carry the same admission context and cause.kind()==InvalidData with raw_os_error()==None. In the first, a listed I-role check read a proc file with read_to_string and got std's payload-free Error::INVALID_UTF8 (core/src/io/error.rs:102-103), so it is a genuine original cause, unmarked. In the second, the cause packet was malformed, giving the round-3 integrity cause. A local-role admission whose original io::Error is io::Error::new(InvalidData, local_error) looks the same as the second. A caller obeying the rustdoc ('callers shall not read this integrity cause as an original producer error') can only tell them apart by an undocumented 'get_ref() is Some and not the marker' heuristic, which also matches local custom causes, or by parsing Display, which the spec forbids. The TC-049 step 27 oracle checks kind, raw errno and marker absence, and the genuine payload-free InvalidData case satisfies all three. | spec/kani/functional/FR-034-caller-death-ownership.md:967-971; spec/kani/functional/FR-034-caller-death-ownership.md:864-868; spec/kani/functional/FR-034-caller-death-ownership.md:1193; rust-1.98.1 core/src/io/error.rs:102-103 |

## Dispositions (round 4)

Round 4, reviewed at agent-ix/quire-contract-codegen@45da0787da71abb24b8122f48ea7012d274a5d7a (fix diff 3225e12..45da078). Reviewer session dcb5e3e7-8fe4-422e-aef1-3ca57d78bee2, model claude-opus-5-5, run 9ee1b302-75e7-4222-be42-3d38cc35f1b2. This was a single consolidated consistency pass over the whole amended cross-role text, failure mapping, AC-40, TC-049 step 27, the consumer matrix and the Expected Results row. Every remaining defect found is recorded now, in this artifact and SR-2522. FND-001..FND-004 latest outcomes remain fixed; no regression of them was found.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-005 | fixed | 45da078: The integrity cause now has a typed public identity: KaniCauseMetadataIntegrityError is a public opaque error detected only by get_ref()/downcast_ref, and the spec states that InvalidData, get_ref().is_some(), source presence, type-name text and Display do not distinguish it (lines 911-924). The rustdoc paragraph (lines 982-987), AC-40, the step 27 oracle and the new TC-049 consumer matrix (lines 744-759) require payload-free and local custom InvalidData controls to give None for that downcast. |

## New findings (disposition pass 4)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | Line 968 sends 'Required representation failures without a previously representable original cause' to 'the failure rule above', but no rule of that name exists any more: the R1 unnamed-kind failure rule was replaced in round 2 by the closed-kind domain, and the current integrity paragraph (lines 911-953) covers only C-observed unknown/malformed/incomplete metadata. The sender-side scalar categories that describe exactly these failures (required representation exceeding its bound, required representation formatting failure, unnamed I/O kind, OS-kind mismatch, lines 880-881) remain in the provenance set with no stated public cause or mapping. Scenario: O fails to encode a required cause because its required representation exceeds the bound, and it has no previously representable original cause. One implementer sends the declared scalar category 'required representation exceeding its bound' with outer Other and a KaniCrossRoleCauseLoss marker. Another sends an incomplete packet, so C applies the integrity rule and publishes InvalidData with KaniCauseMetadataIntegrityError. A third drops the cause and lets C report a protocol Tool failure. Line 968 points to a rule that no longer exists, so none of these can be called conforming, and the consumer matrix has no row for the case. | spec/kani/functional/FR-034-caller-death-ownership.md:967-968; spec/kani/functional/FR-034-caller-death-ownership.md:879-884 |
| FND-007 | low | KaniCauseMetadataIntegrityError is required to carry 'authenticated finite role/site provenance fields' (lines 914-915), but the same error is also published on the unauthenticated-site Tool path (lines 943-945), where line 939 forbids inventing admission context; the spec does not say whether those fields are optional, role-only, or absent in that case. Scenario: A malformed cause packet arrives on O's authenticated control, so the role is known, but its admission site cannot be independently authenticated. CODE must build KaniCauseMetadataIntegrityError for the Tool path. One implementation fills the private site field from the packet's label, which line 939 forbids. Another makes the field an Option, which lines 914-915 do not allow. A third rejects construction and reports a different error, which drops the public integrity downcast that matrix row 758 requires. | spec/kani/functional/FR-034-caller-death-ownership.md:914-915; spec/kani/functional/FR-034-caller-death-ownership.md:938-945 |
