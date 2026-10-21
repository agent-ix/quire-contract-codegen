<!-- reviewer-dispositions repo=agent-ix/quire-contract-codegen visibility=public quoin=0.28.4 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-15235 round=2 model=gpt-6.1-sol run=53c49625-af99-46bb-a313-114eaa6d8f5e -->

Same-reviewer narrow rebase disposition for PR497. Exact current head 3f1e7668ff49abaa439998bcb98572b0d4ae1b98; base 76dea88f2d22f6fe15208552a24bfdb44fe2038c. Independent binary git diff and every one of 16 owned files are byte-identical across the ordinary rebase (two commits range-diff equal). PR494 changed family/corpus outcome lowering and tests; PR496 changed composite admission stage allocation. Neither alters namespace owner/launch/poll/cancellation source, FR017 diagnostic semantics or TC043 planned controls. The diagnostic remains observational only; no timing, retries, KILL ordering, pidfd identity, errno priority or Drop behavior changes are introduced. No Cargo/fmt/metadata/native/test/proof command ran in this review.

Eight non-gap methods SR15233/15234/15236–15241 remain PASS. Current base884:523 tagged,293 untagged,50 method-without-symbol,18 ignored-only. Candidate887:523 tagged,295 untagged,51 method-without-symbol,18 ignored-only. All884 base statements/methods/statuses/binder identities are preserved by this PR apart from line offsets. Computed open gaps313 =311 existing IR878/program +2 planned IR877 tests. Additionally, tagged FR018AC8 still lacks its newly required authentic duplicate-source admission control: the unchanged synthetic duplicate test admits corpus_package and calls public generate; it does not call compile_admitted or assert typed Package/CompileRefusal::code(AmbiguousDeclaration)/cause(None). Therefore semantic coverage must retain that separate IR17 gap. Current open findings314 =311 program +1 IR17 +2 IR877. Two historical IR621 findings are fixed by landed PR494, not by this rebase. AC56 Inspection is still explicitly pending IR877 implementation/source inspection. Full gap remains FAIL.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-076 | fixed | Implemented by landed CG PR494 (53fa532794b26f89d641f6d689b6230f140aa140); actual public signature/success/adverse controls and private oversized-render refusal inspected. Current native binder exists; no new proof verdict claimed. |
| FND-077 | fixed | Implemented by landed CG PR494 (53fa532794b26f89d641f6d689b6230f140aa140); actual public signature/success/adverse controls and private oversized-render refusal inspected. Current native binder exists; no new proof verdict claimed. |
| FND-316 | deferred | IR-17 explicit actual decision 7e6d6fef-f764-4869-ae69-bebeb321e713 owns the genuine compile_admitted duplicate-source control and separate exact private defenses; reviewer current-base disposition 18645a93-eb81-403e-b6ab-eeff4c2a926b independently retains it. This PR does not implement, qualify, close or waive that gap. |

+++ [reviewer data]

```yaml
dispositions:
- fnd: FND-076
  outcome: fixed
  artifact_id: FR-015-AC-123
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  after_excerpt: External public-consumer calls to `prepare_bounded_collection_query`, `prepare_finite_graph_reaches`
    and `generate_bounded_kani_corpus_case` have the direct result signatures specified by `interface-001`, with
    `KaniOutcome` as their error type and no public `FamilyLoweringError`, `BoundedCorpusError`, `OutcomeConstruction`
    branch, or replacement alias or wrapper.
  reason: Implemented by landed CG PR494 (53fa532794b26f89d641f6d689b6230f140aa140); actual public signature/success/adverse
    controls and private oversized-render refusal inspected. Current native binder exists; no new proof verdict
    claimed.
- fnd: FND-077
  outcome: fixed
  artifact_id: FR-015-AC-124
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  after_excerpt: After infallible-constructor adoption, each retained family/corpus adverse control returns the
    original exact non-success kind, typed cause, source identity and selected profile/bound context, with no Boolean
    claim, partial artifact or premature identity claim; successful outputs and failure ordering remain unchanged.
  reason: Implemented by landed CG PR494 (53fa532794b26f89d641f6d689b6230f140aa140); actual public signature/success/adverse
    controls and private oversized-render refusal inspected. Current native binder exists; no new proof verdict
    claimed.
- fnd: FND-316
  outcome: deferred
  artifact_id: FR-018-AC-8
  reason: IR-17 explicit actual decision 7e6d6fef-f764-4869-ae69-bebeb321e713 owns the genuine compile_admitted
    duplicate-source control and separate exact private defenses; reviewer current-base disposition 18645a93-eb81-403e-b6ab-eeff4c2a926b
    independently retains it. This PR does not implement, qualify, close or waive that gap.
current_head: 3f1e7668ff49abaa439998bcb98572b0d4ae1b98
current_base: 76dea88f2d22f6fe15208552a24bfdb44fe2038c
method_verdicts:
  SR-15233: PASS
  SR-15234: PASS
  SR-15235: FAIL
  SR-15236: PASS
  SR-15237: PASS
  SR-15238: PASS
  SR-15239: PASS
  SR-15240: PASS
  SR-15241: PASS
current_open:
  program:
  - FR-002-AC-10
  - FR-002-AC-11
  - FR-002-AC-12
  - FR-002-AC-13
  - FR-002-AC-14
  - FR-002-AC-15
  - FR-002-AC-16
  - FR-002-AC-17
  - FR-002-AC-18
  - FR-002-AC-7
  - FR-002-AC-8
  - FR-004-AC-10
  - FR-004-AC-11
  - FR-004-AC-12
  - FR-004-AC-13
  - FR-004-AC-14
  - FR-004-AC-15
  - FR-004-AC-18
  - FR-004-AC-6
  - FR-004-AC-8
  - FR-005-AC-3
  - FR-008-AC-7
  - FR-008-AC-9
  - FR-009-AC-7
  - FR-015-AC-105
  - FR-015-AC-106
  - FR-015-AC-107
  - FR-015-AC-108
  - FR-015-AC-110
  - FR-015-AC-111
  - FR-015-AC-117
  - FR-015-AC-119
  - FR-015-AC-120
  - FR-015-AC-121
  - FR-015-AC-122
  - FR-015-AC-19
  - FR-015-AC-21
  - FR-015-AC-22
  - FR-015-AC-23
  - FR-015-AC-24
  - FR-015-AC-30
  - FR-015-AC-31
  - FR-015-AC-32
  - FR-015-AC-38
  - FR-015-AC-39
  - FR-015-AC-40
  - FR-015-AC-41
  - FR-015-AC-42
  - FR-015-AC-43
  - FR-015-AC-44
  - FR-015-AC-45
  - FR-015-AC-46
  - FR-015-AC-47
  - FR-015-AC-48
  - FR-015-AC-49
  - FR-015-AC-51
  - FR-015-AC-52
  - FR-015-AC-56
  - FR-015-AC-57
  - FR-015-AC-69
  - FR-015-AC-70
  - FR-015-AC-71
  - FR-015-AC-72
  - FR-015-AC-73
  - FR-015-AC-74
  - FR-015-AC-75
  - FR-015-AC-76
  - FR-015-AC-80
  - FR-015-AC-84
  - FR-015-AC-85
  - FR-015-AC-87
  - FR-015-AC-89
  - FR-015-AC-90
  - FR-015-AC-91
  - FR-015-AC-92
  - FR-015-AC-93
  - FR-015-AC-95
  - FR-015-AC-98
  - FR-017-AC-19
  - FR-017-AC-26
  - FR-017-AC-27
  - FR-017-AC-30
  - FR-017-AC-47
  - FR-017-AC-52
  - FR-017-AC-53
  - FR-017-AC-7
  - FR-017-CON-1
  - FR-018-AC-21
  - FR-018-AC-22
  - FR-018-AC-23
  - FR-018-AC-33
  - FR-018-AC-34
  - FR-018-AC-35
  - FR-018-AC-36
  - FR-018-AC-37
  - FR-018-AC-38
  - FR-018-AC-39
  - FR-018-AC-40
  - FR-018-AC-41
  - FR-018-AC-42
  - FR-018-AC-43
  - FR-019-AC-15
  - FR-019-AC-24
  - FR-019-AC-3
  - FR-019-AC-34
  - FR-019-AC-35
  - FR-019-AC-36
  - FR-019-AC-37
  - FR-019-AC-38
  - FR-019-AC-39
  - FR-019-AC-40
  - FR-019-AC-41
  - FR-021-AC-30
  - FR-022-AC-26
  - FR-022-AC-27
  - FR-022-AC-28
  - FR-024-AC-1
  - FR-024-AC-18
  - FR-024-AC-34
  - FR-024-AC-35
  - FR-024-AC-36
  - FR-024-AC-37
  - FR-024-AC-38
  - FR-024-AC-39
  - FR-024-AC-40
  - FR-024-AC-7
  - FR-024-AC-8
  - FR-025-AC-1
  - FR-025-AC-2
  - FR-025-AC-3
  - FR-025-AC-4
  - FR-025-AC-5
  - FR-025-AC-6
  - FR-025-AC-7
  - FR-025-AC-8
  - FR-025-AC-9
  - FR-026-AC-1
  - FR-026-AC-4
  - FR-028-AC-13
  - FR-028-AC-14
  - FR-028-AC-15
  - FR-028-AC-16
  - FR-028-AC-17
  - FR-028-AC-18
  - FR-028-AC-19
  - FR-028-AC-20
  - FR-028-AC-22
  - FR-028-AC-23
  - FR-028-AC-24
  - FR-028-AC-5
  - FR-028-AC-6
  - FR-028-AC-7
  - FR-028-AC-8
  - FR-028-AC-9
  - FR-029-AC-17
  - FR-029-AC-19
  - FR-029-AC-21
  - FR-029-AC-22
  - FR-029-AC-23
  - FR-029-AC-24
  - FR-029-AC-25
  - FR-029-AC-26
  - FR-029-AC-27
  - FR-029-AC-30
  - FR-029-AC-31
  - FR-030-AC-19
  - FR-030-AC-20
  - FR-030-AC-21
  - FR-030-AC-22
  - FR-031-AC-16
  - FR-031-AC-17
  - FR-031-AC-2
  - FR-031-AC-20
  - FR-031-AC-22
  - FR-031-AC-23
  - FR-031-AC-24
  - FR-031-AC-25
  - FR-031-AC-26
  - FR-031-AC-27
  - FR-031-AC-6
  - FR-032-AC-1
  - FR-032-AC-10
  - FR-032-AC-11
  - FR-032-AC-12
  - FR-032-AC-13
  - FR-032-AC-14
  - FR-032-AC-4
  - FR-032-AC-5
  - FR-032-AC-6
  - FR-032-AC-7
  - FR-032-AC-8
  - FR-032-AC-9
  - FR-033-AC-10
  - FR-033-AC-14
  - FR-033-AC-15
  - FR-033-AC-17
  - FR-033-AC-2
  - FR-033-AC-3
  - FR-033-AC-4
  - FR-033-AC-5
  - FR-033-AC-6
  - FR-033-AC-8
  - FR-034-AC-1
  - FR-034-AC-10
  - FR-034-AC-100
  - FR-034-AC-11
  - FR-034-AC-12
  - FR-034-AC-13
  - FR-034-AC-14
  - FR-034-AC-15
  - FR-034-AC-16
  - FR-034-AC-17
  - FR-034-AC-18
  - FR-034-AC-19
  - FR-034-AC-2
  - FR-034-AC-20
  - FR-034-AC-21
  - FR-034-AC-22
  - FR-034-AC-23
  - FR-034-AC-24
  - FR-034-AC-25
  - FR-034-AC-26
  - FR-034-AC-27
  - FR-034-AC-28
  - FR-034-AC-29
  - FR-034-AC-3
  - FR-034-AC-31
  - FR-034-AC-32
  - FR-034-AC-33
  - FR-034-AC-34
  - FR-034-AC-35
  - FR-034-AC-36
  - FR-034-AC-37
  - FR-034-AC-38
  - FR-034-AC-39
  - FR-034-AC-4
  - FR-034-AC-40
  - FR-034-AC-5
  - FR-034-AC-51
  - FR-034-AC-52
  - FR-034-AC-53
  - FR-034-AC-54
  - FR-034-AC-57
  - FR-034-AC-58
  - FR-034-AC-59
  - FR-034-AC-6
  - FR-034-AC-60
  - FR-034-AC-61
  - FR-034-AC-62
  - FR-034-AC-63
  - FR-034-AC-65
  - FR-034-AC-66
  - FR-034-AC-67
  - FR-034-AC-69
  - FR-034-AC-7
  - FR-034-AC-70
  - FR-034-AC-71
  - FR-034-AC-73
  - FR-034-AC-74
  - FR-034-AC-75
  - FR-034-AC-76
  - FR-034-AC-77
  - FR-034-AC-78
  - FR-034-AC-79
  - FR-034-AC-8
  - FR-034-AC-80
  - FR-034-AC-81
  - FR-034-AC-82
  - FR-034-AC-83
  - FR-034-AC-84
  - FR-034-AC-85
  - FR-034-AC-86
  - FR-034-AC-87
  - FR-034-AC-88
  - FR-034-AC-89
  - FR-034-AC-9
  - FR-034-AC-90
  - FR-034-AC-91
  - FR-034-AC-92
  - FR-034-AC-94
  - FR-034-AC-95
  - FR-034-AC-96
  - FR-034-AC-97
  - FR-034-AC-98
  - FR-034-AC-99
  - FR-035-AC-1
  - FR-035-AC-2
  - FR-035-AC-3
  - FR-035-AC-4
  - FR-035-AC-5
  - FR-035-AC-6
  - FR-036-AC-1
  - FR-036-AC-2
  - FR-036-AC-3
  - FR-036-AC-4
  - FR-036-AC-5
  - FR-036-AC-6
  - FR-036-AC-7
  - FR-039-AC-4
  - FR-041-AC-4
  - FR-042-AC-1
  - NFR-001-M-1
  - NFR-002-M-1
  - NFR-002-M-3
  - NFR-004-M-1
  - NFR-004-M-2
  - NFR-004-M-3
  - NFR-004-M-4
  - NFR-006-M-1
  - NFR-006-M-2
  - NFR-006-M-3
  IR-17:
  - FR-018-AC-8
  IR-877:
  - FR-017-AC-54
  - FR-017-AC-55
pending_inspection:
  id: FR-017-AC-56
  owner: IR-877
  status: pending implementation and actual source inspection
```

+++