---
id: SR-14122
title: IR-936 observer spec-review/base review
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-codegen; spec/kani/functional/FR-017-kani-execution-evidence.md;
  spec/kani/matrix/TC-027-kani-execution-evidence.md
review_set: subset
---

## Summary

Ticket: IR-936. PR: quire-contract-codegen#475. Frozen head 496233a16ccfc3570bca0fe20bc8136de35f6312; assigned base 7c8e62910cd56387a4cdfc5c32a7b1f39693982c.

Reviewed all changed normative prose and AC-29/45/46/48/49/51/52/53 with the surrounding receipt, error and execution contracts, owning TC-027 and StR-001. Four emitted originals, source map, adapter, harness/record, production lock and two observer files form ten closed entries. UTF-8 sorting, canonical JCS/SHA-256, independent literal golden plus schema assertion, original-byte comparison, operand/limit/outcome exclusions, opaque custody and cleanup diagnostics remain coherent. Automated full and focused Quire validation pass with inherited advisories. New obligations are Planned; no implementation, Unicode fix or Kani qualification is credited.

## Verdict

PASS. No new finding. Review approves specification readiness only; implementation, nested Cargo qualification, Unicode resolution and IR-17/705/844 proof delivery remain uncredited. Root retains final verification and reviewed merge administration.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Model gpt-6.1-sol verified in this native turn context; run 0beef413-c37e-427c-91a3-6368dc2bb83e. Quoin 0.28.4; Quire 0.36.2/engine 0.50.2; spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502. No compiler, Cargo, rustc, fmt, Kani, aggregate or heavy reservation acquired. Scope is 103 ordered records; bindings are 48 records. Structural binding inventory is not semantic coverage or run discharge.

Applicable methods: code review, planless whole_repo gap, base specification, EARS, integrity, failure-domain, scope-boundary, evidence advisor and risk-complexity. Rust/React/compiler review is inapplicable to this spec-only diff. Object review is not a separate lens: no new domain object is introduced and existing type ownership is covered by scope-boundary/integrity. Dependency lens skipped because no relationship edge changes. Criterion-strength not selected; a calibrated Jev run was not attempted; correctness/fuzz authoring, app and security-profile reviews are outside this change. No applicable AssuranceProfile or required review-selection profile is installed in spec/.

Primary Cargo/Rust documents support examples with dev dependencies, resolver-2 isolation and source-relative module paths: [Cargo targets](https://doc.rust-lang.org/cargo/reference/cargo-targets.html), [features](https://doc.rust-lang.org/cargo/reference/features.html#feature-resolver-version-2), [module paths](https://doc.rust-lang.org/reference/items/modules.html#the-path-attribute). These do not qualify this nested staging layout.

## Scope and bindings

```yaml
clean: true
scope:
- id: FR-017:L20
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '## Description'
  assessment: targeted contract and inherited invariant context
  line: 20
- id: FR-017:L23
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'When a caller runs an FR-015, FR-022/FR-014 or FR-035 harness, the code generator shall invoke the

    installed Kani backend and retain the backend''s own reported outcome as typed

    execution evidence. A caller may hand the generator several harnesses to run together

    (FR-017-AC-21 to FR-017-AC-25); the generator then forms groups, runs each

    group in one launcher process, and retains one evidence record per harness. Unless a

    statement says "batch" or "group", it describes a run of one harness.'
  assessment: targeted contract and inherited invariant context
  line: 23
- id: FR-017:L30
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '[FR-040](./FR-040-cancellation-producer.md) adds the one launcher-observed cancellation outcome:

    the backend did not report a verdict for a run stopped by caller cancellation. Its typed source

    comes from the live execution control, and its group refusal produces no member evidence.'
  assessment: targeted contract and inherited invariant context
  line: 30
- id: FR-017:L34
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Two words are used for two things below. A **batch** is the list of harnesses the caller hands to

    the batch entry, one call. A **group** is the harnesses of one batch that share one launcher

    process, so G groups are G processes; a group of one is a single run. A rule that says "the

    batch" is about the call: a batch refusal (a member not in the crate) refuses the call and starts

    no process. A rule that says "the group" is about one process: a group refusal (an output stream

    over its limit or unread, an outer bound that elapsed, a missing report after a clean exit, a

    report'
  assessment: targeted contract and inherited invariant context
  line: 34
- id: FR-017:L44
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'FR-015 and FR-035 emit harnesses and typed refusals and assert nothing about

    whether one ever ran; this requirement owns the run and everything read back

    from it. The FR-035 caller-ingress harness is a distinct executable kind, separate

    from both checked-package harness kinds. Its evidence keeps the caller

    Text-admission identity rather than claiming a checked node or contract role.'
  assessment: targeted contract and inherited invariant context
  line: 44
- id: FR-017:L50
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '## Inputs'
  assessment: targeted contract and inherited invariant context
  line: 50
- id: FR-017:L52
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: "- One harness of any kind this requirement executes: an FR-015 contract\n  harness (`KaniObligationHarness`,\
    \ contract role in\n  `ObligationKind`) or an FR-022/FR-014 exact-scalar harness\n  (`KaniScalarObligationHarness`,\
    \ no contract role), or the distinct FR-035\n  caller Text-admission harness (no contract role, with a typed\
    \ caller claim\n  identity naming profile, bounds and finite payload class). Each identity\n  carries the option\
    \ vector to invoke, the unwind bound and the solver. This\n  requirement reads the same handful of facts from\
    \ whichever one the caller\n  hands it,"
  assessment: targeted contract and inherited invariant context
  line: 52
- id: FR-017:L83
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '## Outputs'
  assessment: targeted contract and inherited invariant context
  line: 83
- id: FR-017:L85
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: "- Execution evidence naming the harness path, the exact invocation, and the\n  backend-reported outcome.\
    \ The obligation-kind field is the contract harness's\n  role for a contract harness, and `None` for an exact-scalar\
    \ or caller\n  Text-admission harness, neither of which has a contract role. Caller\n  Text-admission evidence\
    \ carries its typed caller-ingress identity so it\n  cannot be attributed to a checked-package scalar claim.\n\
    - For a `NoVerdict` run only, a diagnostic record in that evidence: a typed failure phase and a\n  bounded,\
    \ redacted tail excerpt of each of the"
  assessment: targeted contract and inherited invariant context
  line: 85
- id: FR-017:L105
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '## Behavior'
  assessment: targeted contract and inherited invariant context
  line: 105
- id: FR-017:L107
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: "- If the launcher is absent, then the generator shall refuse with a typed\n  reason naming its path,\
    \ and shall run nothing.\n- When a caller requests a paired routed-scalar proof/native receipt, the runner shall\
    \ bind the\n  retained scalar harness to its complete generated oracle package and validate the selected\n \
    \ public oracle, its arity and argument order, the concrete operands against their retained\n  ranges before\
    \ starting either child process. It shall pass the request's exact `ScalarLimits` to\n  `KaniScalarNativeRequest`,\
    \ initialize the native meter from those values,"
  assessment: targeted contract and inherited invariant context
  line: 107
- id: FR-017:L692
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '### Scalar proof-content identity as the FR-357 observation digest'
  assessment: targeted contract and inherited invariant context
  line: 692
- id: FR-017:L695
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The measured proof-content identity of FR-017-AC-29 is the value QSL FR-357 carries as the

    **canonical generated-content identity of the observation**

    ([QSL FR-357](ix://agent-ix/quire-spec-language/FR-357), Description and AC-14). This section fixes its digest
    domain, its exact preimage, who mints it and

    who authenticates it, so the IR-773 driver can recompute it. It extends FR-017-AC-29; FR-032 refers here for
    the representation.'
  assessment: targeted contract and inherited invariant context
  line: 695
- id: FR-017:L700
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The v3 companion execution/custody behavior and its new or extended verification obligations

    are **Planned / UNRUN** until implemented and qualified. This amendment claims no companion

    compile, sealed invocation, proof success, independent-resolution fix or IR-17 completion.'
  assessment: targeted contract and inherited invariant context
  line: 700
- id: FR-017:L704
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: "- **Domain.** The scalar runner shall expose the identity as a `qsl_replay::DigestRecord`, which the\
    \ crate re-exports at its root (`quire_contract_codegen::DigestRecord`) so a consumer names one type and CG\
    \ defines no twin, whose domain is\n  `sha256-jcs` and whose bytes are the 32-byte SHA-256 of the RFC 8785 (JCS)\
    \ canonical encoding of\n  the structural preimage below. `sha256-jcs` is chosen because the preimage is a structured\
    \ JSON\n  document of several artifacts, not one byte string; QSL's `GeneratedArtifactBytesV1` is\n  raw-byte-addressed\
    \ and would be correct only if CG"
  assessment: targeted contract and inherited invariant context
  line: 704
- id: FR-017:L818
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '## Constraints'
  assessment: targeted contract and inherited invariant context
  line: 818
- id: FR-017-CON-1
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-CON-1 | The generator SHALL NOT convert a non-verified outcome into a proof claim. | Integrity
    | Test (TC-027) |'
  assessment: targeted contract and inherited invariant context
  line: 822
- id: FR-017-CON-2
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-CON-2 | The generator SHALL NOT report a generation-time classification as an execution outcome.
    | Integrity | Test (TC-027) |'
  assessment: targeted contract and inherited invariant context
  line: 822
- id: FR-017:L825
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '## Acceptance Criteria'
  assessment: targeted contract and inherited invariant context
  line: 825
- id: FR-017-AC-2
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-2 | An absent launcher is refused with a typed reason naming its path before anything runs.
    | Test (TC-027) |'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-4
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-4 | A run is verified only when the process exited successfully, the report states success,
    and every cover property is reported satisfied; a successful run with an unsatisfied cover is cover-unsatisfied
    with its satisfied and total counts; a report of success from an unsuccessfully exited process, a report with
    no cover property and a report with no successful check are each inconclusive with their own reason; console
    banners never decide the verdict. | Test (TC-027) |'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-5
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-5 | A failed non-unwinding check with a concrete playback is falsified carrying that playback
    verbatim and never the playback of a satisfied cover; a failure with no playback is inconclusive for that reason;
    a failed unwinding assertion is inconclusive as an exhausted bound rather than falsified, and a succeeded unwinding
    check in a results listing is not a failure. | Test (TC-027) |'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-6
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-6 | The execution evidence of a single run carries the obligation kind, the harness path,
    the launcher path, the complete argument vector, the unwind bound, the solver, the exit code and the outcome.
    | Test (TC-027) |'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-7
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-7 | A crate whose library source does not contain the harness source byte for byte is refused,
    and no backend runs. | Test (TC-027) |'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-11
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-11 | A routed FR-022/FR-014 exact-scalar harness (`KaniScalarObligationHarness`) runs through
    `execute_kani_obligation` and `kani_launch_command` the same way an FR-015 contract harness does: a crate whose
    library source lacks its generated source byte for byte is `HarnessNotInCrate`, its covers classify a run identically
    (all satisfied is verified, an unsatisfied one is cover-unsatisfied, none printed is inconclusive), and its
    evidence carries `None` for obligation kind, since an exact-scalar claim carries no contract role. | Test (TC-027)
    |'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-12
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-12 | Real Kani captures of a verified run, a falsified run with a playback, an exhausted
    unwind bound, a run whose only check is unreachable, a partly satisfied cover and a run with no cover each parse
    into the expected typed report of harness status, successful checks, cover counts and failed checks, and classify
    to the expected outcome; the falsifying playback block passes through verbatim. | Test (TC-027) |'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-13
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-13 | A run whose process exited successfully and whose backend reported success with zero
    successful checks is inconclusive with the vacuous-proof reason, never verified, except that a precondition
    harness, which asserts nothing beyond its cover, is decided by its cover summary. | Test (TC-027) |'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-14
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-14 | A launcher whose stdout or stderr carries more than 8 MiB times its harness count is
    stopped, its process group is killed, and the run is refused with `kani_output_over_limit` naming the stream,
    the limit and the count, with no outcome and no retained text; a launcher whose stream is exactly the limit
    completes with its real exit status and the whole stream retained. No stream is ever truncated and then classified.
    | Test (TC-043) |'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-15
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-15 | A timeout too large to add to the current instant never elapses and does not panic.
    | Test (TC-027) |'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-16
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-16 | The launcher''s capture threads are stopped and joined on every outcome: they return
    what the launcher wrote before it ended, stop while a write end is still held open, and stop within their drain
    limit while a straggler keeps writing, so a process holding a pipe open does not delay the return. | Test (TC-027)
    |'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-17
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-17 | A run that times out has the whole group the launcher leads killed, a real grandchild
    included. | Test (TC-027) |'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-18
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-18 | A report that is malformed, of an unknown check or harness status, of another schema
    version, that holds other than one harness result in a single run, or whose harness states success while it
    lists a failed, errored, undetermined or unknown check (of any class, including an unwinding assertion) is refused
    with its own typed cause and never classified, for every obligation kind; a run that exited successfully and
    exported no report is refused, and one that exited unsuccessfully and exported none is `NoVerdict`. | Test (TC-027)
    |'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-19
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-19 | The generator appends `-Z unstable-options --export-json /proc/self/fd/N` after unchanged
    harness options, with actual child-only anonymous pipe writer N >= 5, and records the actual argv bytes. It
    uses per-run unnamed report authority, never a named target-directory report or fallback. FR-034 AC-32/33 own
    collector bounds, resource-stop classification, kernel lifetime and immutable final handoff. Zero-byte EOF means
    no exported report; nonempty partial/malformed content is refused under AC-18 unless an independently established
    resource/deadline stop'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-20
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-20 | The evidence and the classified run list every check of a real run with its id, class,
    source file and line and status, a line stated as unknown is absent, and a non-numeric line or a check with
    no location is a refused report; the view serializes as `id`, `class`, `location { file, line }` and `status`
    and is not deserializable. | Test (TC-027) |'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-21
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-21 | N harnesses with equal option vectors (the harness selection removed) and equal identity
    `ProofCeilings` (`wall_clock` T and `memory_bytes` M) start exactly one launcher process whose argument vector
    holds one `--harness <module::harness> --exact` pair per member in request order, `--harness-timeout` T, then
    the shared options; N harnesses in G such groups start G processes; so N > 1 compatible harnesses start fewer
    than N processes; one harness starts one process with the single-run argument vector. Requests whose wall-clock
    or memory ceilings differ'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-22
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-22 | Over a real or captured batch report, members are matched by `module::harness` path
    (two members with the bare symbol `check` stay distinct), and each member''s outcome, checks and SUCCESS count
    come from its own entry: a falsified member beside a verified member leaves the verified member verified; a
    Success member in a non-zero exit with no Failure entry is inconclusive, and with one Failure entry is verified;
    a Failure entry with no checks and exit status `timeout` is inconclusive timed-out naming T while the others
    keep their results; a Failure entry'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-23
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-23 | A group''s report that lacks a requested harness, holds one twice or holds an unrequested
    one refuses the group with a typed cause and classifies no member; a member whose source is not in the crate
    refuses the whole batch with `HarnessNotInCrate` and launches nothing; a falsified member''s playback is the
    first counterexample block headed for its own path and never another member''s block, a member that fails two
    property checks (two counterexample blocks under its path, as real Kani prints) takes the first, as the same
    harness alone does, and neither it nor'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-24
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-24 | A launcher that exits on its own after leaving a real grandchild in its process group
    has that grandchild killed by the time the run returns, the same as on a timeout. | Test (TC-043) |'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-25
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-25 | A capture thread that panics, or whose poll or read of the stream errs, refuses the
    run with `kani_output_unread`; the run is never classified from empty text. | Test (TC-043) |'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-26
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-26 | A generated FR-035 caller Text-admission harness enters the same production execution
    path in single and compatible batch runs; its evidence carries the exact typed caller identity (profile, bounds
    and finite payload class) supplied with that harness and `None` for contract obligation kind. A matching successful
    report with a satisfied cover and successful agreement assertion classifies `Verified`; an unsatisfied cover
    classifies `CoverUnsatisfied` with counts; a failed agreement assertion with its own concrete playback classifies
    `Falsified` with that'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-27
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-27 | A genuine routed `x + 1` scalar harness from `generate_routed`, its complete emitted
    oracle package and one in-range concrete operand are passed to the paired runner. One real Kani run reports
    `Verified`, and the runner invokes the public oracle from the same staged library source at that operand with
    the request''s exact `ScalarLimits`; the receipt contains the Kani evidence and the actual typed runtime result
    of that invocation. A Kani verdict alone or a result reconstructed by the caller is insufficient. | Test (TC-027)
    |'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-28
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-28 | Before either child starts, a package or harness whose path or bytes differ from the
    retained routed output, a selected oracle symbol or argument order that differs from the retained scalar identity,
    or an operand count/value outside the retained ordered ranges is refused with a typed cause and no receipt.
    For an accepted request, the exact supplied `ScalarLimits` are passed unchanged to `KaniScalarNativeRequest`,
    used to initialize the native meter, and returned in the receipt; no hidden or substituted limit value is permitted.
    CG attests these supplied,'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-29
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-29 | The runner measures one canonical proof-content identity over the exact paths and bytes
    of every staged execution input: the complete generated package, retained harness, canonical native invocation
    adapter, observer manifest and both independently generated dependency locks. This identity binds the content
    actually proved and executed; it is not a tool-version or file-version tracking identity, and no separate identity
    is added for any staged item. The Kani and native invocations cannot write the staged source tree. A changed,
    replaced or additional'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-30
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-30 | The public receipt exposes the measured identity, scalar identity, ordered operands,
    exact supplied and meter-used `ScalarLimits`, verified Kani evidence and actual typed native outcome through
    read-only access. Callers cannot construct, deserialize or mutate a receipt, set its identity or outcome, or
    turn any non-verified, refused, inconclusive, faulted or malformed run into a receipt. | Test (TC-027) |'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-31
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-31 | The paired runner executes an order-sensitive routed scalar operation with two symbolic
    operands in two concrete operand orders. Each real Kani run is verified; the native invocation returns the result
    for that order; and each receipt binds its exact ordered operand vector and typed native result. Swapping the
    operands changes the observed result and receipt context while retaining the same generated proof-content identity.
    | Test (TC-027) |'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-32
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-32 | In the `make kani` lane, a crate whose library source holds the generated harness byte
    for byte and, beside it, an item that does not compile (a reference to an undefined name) is run through `execute_kani_obligation`:
    the run exits unsuccessfully, exports no report and is `Inconclusive` `NoVerdict` with its exit code, and its
    evidence carries a diagnostic record whose phase is `compile`, whose stdout excerpt holds the rustc error code
    and Kani''s `error: Failed to execute cargo` line and whose stderr excerpt holds Cargo''s `error: could not
    compile` line'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-33
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-33 | Over single-run launcher stand-ins that exit 1: a report of success beside the unsuccessful
    exit is `unknown` even when its stdout holds `Solving with CaDiCaL 3.0.0`; with no report, stdout holding the
    Kani banner, `Checking harness proofs::check...` and `Solving with CaDiCaL 3.0.0` is `solver`; the banner and
    the `Checking harness` line alone is `backend`; the banner and `error: Failed to execute cargo (exit status:
    101). Found 1 compilation errors.` is `compile`, as is stderr `error: could not compile` alone with the banner;
    a usage error on stderr and'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-34
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-34 | A redacted stream of exactly 8192 bytes is kept whole with `omittedBytes` 0; one of
    8193 bytes whose final line starts within the window keeps the longest line-aligned suffix of at most 8192 bytes
    with `omittedBytes` the byte count before it; a stream whose final line alone exceeds 8192 bytes keeps at most
    8192 bytes ending at its last byte, starting on a character boundary; a stream that is one 10000-byte line followed
    by `\n` keeps the last 8192 bytes of it, ending in `\n`, never an empty excerpt; a stream of a 10000-byte line,
    `\n`, `error: last\n`'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-35
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-35 | With crate directory `/h/u/ws/crate`, target directory `/h/u/ws/crate/target` and home
    directory `/h/u` supplied in the request, a stand-in whose streams print `(/h/u/ws/crate)`, `/h/u/ws/crate/src/lib.rs:6:21`,
    `/h/u/ws/crate/target/debug/x.out`, `/h/u/.kani/bin`, `/h/u/ws/crate2/src/lib.rs`, `/opt/tool/bin`, `https://host/x/y`,
    `and/or`, `/`, `2026-10-16T22:27:04.275975036Z`, `Runtime Symex: 0.00331318s`, `Runtime Solver: 1.385e-05s`,
    `in 0.02s`, `check_ttl_30s`, `u32s`, `pid 4242` and `thread ''main'' (4242) panicked` yields, in order, `(<CRATE_DIR>)`,'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-36
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-36 | The diagnostic record never feeds classification: two runs with the same report and
    exit and different stream text classify to the same outcome, checks, SUCCESS count and playback, and two `NoVerdict`
    runs with equal exit and no report whose streams differ have equal evidence in every field but `diagnostics`,
    `memory` (the observed peak resident bytes vary run to run) and the run''s report-path argument (the last argument,
    unique to each run); the FR-029 terminal value of a `NoVerdict` evidence carrying a record is `Failed`, the
    value FR-029-AC-4 gives the'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-37
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-37 | Over single-run launcher stand-ins that exit 1, print `Checking harness proofs::check...`
    and `CBMC failed with status 64` on stdout and `Unknown option: --no-such-option` on stderr, and export a report
    whose one result `proofs::check` is `Failure` with no checks and whose one `error_details` entry for `proofs::check`
    has the exit status shown: `exit_code_64` is `Inconclusive` `FailedWithoutCounterexample` with `failureCause`
    `{ "kind": "exit_code", "code": 64 }` and a `diagnostics` record of phase `unknown` whose stderr excerpt holds
    `Unknown option:'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-38
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-38 | A single-run stand-in that exits 1 and exports a `Failure` report listing a failed
    assertion check with no playback on the console, whose entry for the harness has exit status `properties_failed`,
    is `FailedWithoutCounterexample` with `failureCause` `{ "kind": "properties_failed" }` and no `diagnostics`
    member. The same report with no `error_details` member, with an entry that names another harness only, with
    two entries naming the harness, with an entry whose `exit_status` is null, or whose one result names no `harness_id`,
    is'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-39
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-39 | A batch stand-in of members `a::check`, `b::check`, `c::check`, `d::check` and `e::check`
    whose process exits 1 and whose report has `a::check` `Success` with a passing assertion, every cover satisfied
    and no exit status, `b::check` `Failure` with no checks and exit status `exit_code_64`, `c::check` `Failure`
    with no checks and exit status `timeout`, `d::check` `Failure` listing a failed assertion with no playback block
    headed for its path and exit status `timeout`, and `e::check` `Failure` listing a failed assertion with no playback
    block headed for its'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-40
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-40 | The cause never feeds classification: three single-run stand-ins with the same `Failure`
    report, no checks and the same exit, whose entries give `exit_code_64`, `out_of_memory` and `cbmc_exploded`
    and whose streams differ, classify to the same outcome, checks, SUCCESS count and playback, and their evidence
    is equal in every field but `failureCause`, `diagnostics`, `memory` and the report-path argument; the cause
    is the only input of the FR-029 terminal value that differs: `exit_code_64` and `cbmc_exploded` give `Failed`,
    equal to the terminal value of the'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-41
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-41 | Every `KaniScalarNativeError` variant has a `code()` that is a total `&''static str`
    from exactly this set: `invalid_request`, `artifact_package`, `snapshot_io`, `kani_execution`, `proof_not_verified`,
    `oracle_stop`, `native_launch`, `dependency_lock`, `native_result`, `snapshot_changed`, `identity_encode`, `codegen_lock`.
    The set is closed and the codes are pairwise distinct. The type is `#[non_exhaustive]` (a rustdoc `compile_fail`
    exhaustive match without a wildcard arm, beside a compiling control with one), and `class()` returns a closed'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-42
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-42 | `class()` is `CompletionRefusal` for `invalid_request`, `artifact_package` and `oracle_stop`,
    and `EnvironmentFault` for `snapshot_io`, `native_launch`, `dependency_lock`, `native_result`, `snapshot_changed`,
    `identity_encode` and `codegen_lock`. For `proof_not_verified` it is `CompletionRefusal` for evidence outcome
    `Falsified`, `CoverUnsatisfied`, and `Inconclusive` `MissingCoverSummary`, `VacuousProof`, `UnwindBoundExhausted`
    or `FailedWithoutCounterexample` with no `failureCause` or `properties_failed`, and `EnvironmentFault` for `Cancelled`,'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-43
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-43 | A canonical-identity encoding failure is `IdentityEncode` with code `identity_encode`
    and class `EnvironmentFault`; no path returns `NativeResult` for it. `NativeResult` is produced only for the
    helper''s malformed, missing or unsupported structured result, and a forced identity-encode failure through
    an in-crate test seam returns `IdentityEncode`, not `NativeResult`. | Test (TC-027) |'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-44
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-44 | The code table and the mapping to class are frozen: a test lists every variant with
    its expected code and its mapping to class (one class for a fixed variant; for `proof_not_verified` the class
    of each `KaniRunOutcome`, `KaniInconclusiveReason` and `KaniFailureCause` value; for `kani_execution` the class
    of each nested refusal), fails if a variant, outcome, reason, cause or nested refusal exists that the list lacks
    or if a code value or a mapped class differs, and fails if two variants share a code. Adding a variant, code,
    class or mapping entry is a spec'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-45
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-45 | The receipt exposes its proof-content identity as a `DigestRecord`, the `qsl_replay::DigestRecord`
    re-exported at the crate root (no CG-defined twin type; a compile test names it by the CG path and by the `qsl_replay`
    path as the same type), with domain `sha256-jcs` and 32 bytes equal to `ScalarProofContentIdentity::as_bytes()`.
    An independent computation in the test, SHA-256 of the RFC 8785 encoding of `{"schema": "agent-ix/cg-kani-scalar-proof-content/v3",
    "artifacts": [...]}` over the entries of AC-46, gives the same 32 bytes. The domain is not'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-46
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-46 | For a genuine routed `x + 1` package the v3 preimage has exactly ten artifacts, in
    UTF-8 byte order: `Cargo.lock`, `Cargo.toml`, `claim-map.json`, the only admitted record `kani-obligations/<module>.json`,
    `observer/Cargo.lock`, `observer/Cargo.toml`, `source-maps/lib.json`, `src/bin/ir791_native.rs`, `src/lib.rs`
    (harness), and `src/oracle-artifacts-lib.rs` (emitted library). All four original emitted artifacts, harness
    and record compare byte-identically under the rename map. Removing, renaming, reordering, duplicating or changing
    one byte of any entry'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-47
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-47 | Two receipts for the same package and harness with different operands, different `ScalarLimits`
    and different native outcomes have equal `DigestRecord`s; operand order, limits, ranges, outcome and Kani evidence
    appear nowhere in the preimage. This holds for the swapped-operand runs of FR-017-AC-31. | Test (TC-027) |'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-48
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-48 | The receipt exposes read-only the ordered staged `(path, bytes)` entries the digest
    was built from, including both manifests, both independently generated locks, the source map and the canonical
    `src/bin/ir791_native.rs`. All ten entries remain measured, sealed and compared across both execution boundaries.
    Recomputing the digest from only those exposed entries equals the receipt''s `DigestRecord`. A recomputation
    that substitutes one byte in the package, harness or harness record (which a driver holding different bytes
    would do) differs from the receipt''s'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-49
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-49 | A change to the preimage schema (artifact set, path names, ordering, encoding, canonicalization
    or hash) is made only by changing the schema string, with the golden digest of AC-46 updated in the same change;
    the golden is computed over fixed literal entries, so dependency bumps do not move it, and is asserted together
    with the schema string in one assertion; a golden change without a schema bump is rejected in review (a test
    cannot enforce it). A change to either manifest, either lock or adapter bytes changes the digest with the schema
    string unchanged.'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-50
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-50 | The `NativeMeterObservation` of a receipt exposes read-only the `ScalarLimits` the
    native meter was constructed with. They are read back from the native invocation''s own output, not copied from
    the request: a test seam that makes the adapter report a well-formed but different limit value makes the helper''s
    structured result unsupported in FR-017-AC-43''s sense and yields `NativeResult` / `native_result` / `EnvironmentFault`
    and no receipt. For an accepted request the request limits, the meter-observed limits and `scalar_limits()`
    are equal, and the'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-51
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-51 | A consumer compile test names and constructs `quire_contract_codegen::ScalarLimits`
    using only codegen crate paths, a typed accessor to `KaniScalarNativeRequest::scalar_limits` compiles with that
    same return type, and assignment to the owning `quire_exact::ScalarLimits` compiles without conversion, wrapper
    or compatibility shim. The CG public name and field shape remain unchanged. Planned with IR-17 kernel adoption.
    | Test |'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-52
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-52 | Production emitted bytes and feature-free exact dependency remain unchanged. The isolated
    private observer uses resolver 2 and its sole example compiles the single canonical adapter in place through
    `../src/bin/ir791_native.rs`, with runtime `exact` and kernel `test-support` exposed only by dev-dependencies;
    no wrapper, copied source, symlink, normal/build logging edge or CLI feature override is allowed. Native invocation
    explicitly names `observer/Cargo.toml` and the example; the real production proof retains its harness, ordered
    options, claims, covers,'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017-AC-53
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-53 | Both package locks are independently freshly resolved with the existing bounded offline
    generation/update workflow, without copying a parent/other lock or substituting a cached resolution. Each resolves
    both owning runtime and exact identities to the build''s authoritative lock identities; the independent observer
    regression asserts both literal package names and repository identities independently of the renderer''s list.
    Both manifests, locks and all other AC-46 entries are sealed and compared as a complete path/byte set before
    proof, after proof and after'
  assessment: targeted contract and inherited invariant context
  line: 827
- id: FR-017:L878
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '## Dependencies'
  assessment: targeted contract and inherited invariant context
  line: 878
- id: FR-017:L880
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: "- **Upstream**: [FR-015](./FR-015-bounded-kani-obligations.md),\n  [FR-022](../../routed/functional/FR-022-routed-generation.md),\
    \ whose `generate_routed` is the source\n  of the exact-scalar harness this requirement also runs,\n  [FR-035](../../oracle/functional/FR-035-caller-text-admission.md),\
    \ which supplies the distinct\n  caller Text-admission harness and identity,\n  [interface-001](../../core/functional/interface-001-codegen-api.md).\n\
    - **Downstream**: [TC-027](../matrix/TC-027-kani-execution-evidence.md),\n  [TC-043](../matrix/TC-043-kani-batching-and-output-bounds.md),\n\
    \ "
  assessment: targeted contract and inherited invariant context
  line: 880
- id: TC-027:L10
  path: spec/kani/matrix/TC-027-kani-execution-evidence.md
  role: examined
  excerpt: '## Description'
  assessment: targeted contract and inherited invariant context
  line: 10
- id: TC-027:L13
  path: spec/kani/matrix/TC-027-kani-execution-evidence.md
  role: examined
  excerpt: 'Verify that a bounded Kani obligation runs against the installed backend, that

    its outcome is read from the backend rather than defaulted, and that the

    returned evidence describes the invocation that actually happened.'
  assessment: targeted contract and inherited invariant context
  line: 13
- id: TC-027:L17
  path: spec/kani/matrix/TC-027-kani-execution-evidence.md
  role: examined
  excerpt: '## Test Procedure'
  assessment: targeted contract and inherited invariant context
  line: 17
- id: TC-027:L175
  path: spec/kani/matrix/TC-027-kani-execution-evidence.md
  role: examined
  excerpt: whose library source does not contain it.
  assessment: targeted contract and inherited invariant context
  line: 175
- id: TC-027:L177
  path: spec/kani/matrix/TC-027-kani-execution-evidence.md
  role: examined
  excerpt: 'ScalarLimits owner identity (FR-017-AC-51), **Planned / UNRUN** with IR-17 kernel adoption:

    compile a consumer that constructs `quire_contract_codegen::ScalarLimits` using only CG paths

    and its unchanged fields, types the `KaniScalarNativeRequest::scalar_limits` accessor with that

    same CG type, and assigns the value directly to `quire_exact::ScalarLimits`. Require actual type

    identity without conversion, wrapper or compatibility shim; a source import alone is insufficient.

    Current-main delivery is not credited by the specification amendment.'
  assessment: targeted contract and inherited invariant context
  line: 177
- id: TC-027:L184
  path: spec/kani/matrix/TC-027-kani-execution-evidence.md
  role: examined
  excerpt: 'Private native observer custody (FR-017-AC-45/46/48/49/52/53), **Planned / UNRUN**:

    construct the isolated resolver-2 observer package inside the private snapshot, with the example

    root at `../src/bin/ir791_native.rs`. Verify that this is the single canonical adapter, loading

    `../lib.rs`, with no wrapper, copy or symlink. Compare all four original emitted artifacts

    byte for byte under FR-017''s rename map, explicitly including `source-maps/lib.json` and the

    unmodified production manifest, plus the original harness and record. Inspect the production

    and companion dependency'
  assessment: targeted contract and inherited invariant context
  line: 184
- id: TC-027:L199
  path: spec/kani/matrix/TC-027-kani-execution-evidence.md
  role: examined
  excerpt: 'Run the actual bounded native command with `--manifest-path observer/Cargo.toml --example

    ir791_native` from the sealed root and its target outside that root. Verify actual nonzero ordered

    charges, limits, counters and typed incomplete outcomes from the real meter. Run the real production

    proof with its retained harness/ordered options and all claims/covers, including the existing

    non-verified/no-native controls; no `--all-targets` compile assumption or skipped proof substitutes

    for those checks. Exercise cancellation and limits at the companion resolution/native'
  assessment: targeted contract and inherited invariant context
  line: 199
- id: TC-027:L394
  path: spec/kani/matrix/TC-027-kani-execution-evidence.md
  role: examined
  excerpt: proof evidence is confined to the exhaustive classification table.
  assessment: targeted contract and inherited invariant context
  line: 394
- id: TC-027:L396
  path: spec/kani/matrix/TC-027-kani-execution-evidence.md
  role: examined
  excerpt: 'Snapshot controls (FR-017-AC-29 and AC-30): measure the staged path/byte set for the complete

    generated package (including source map), retained harness, canonical adapter, observer manifest

    and both independently generated dependency locks;

    attempt a source-tree write from each child execution boundary, and require that neither child can

    change it. Inject a changed byte in each execution-relevant staged item, a replaced path and an

    unexpected file after staging and across the Kani/native boundary;

    each must produce a typed failure and no receipt carrying the earlier'
  assessment: targeted contract and inherited invariant context
  line: 396
- id: TC-027:L409
  path: spec/kani/matrix/TC-027-kani-execution-evidence.md
  role: examined
  excerpt: 'Every generation-time refusal above exposes no harness, so `execute_kani_obligation` — which

    takes a `KaniObligationHarness` — has nothing to run for it: a generation-time classification can

    never surface as one of FR-017''s execution outcomes because no code path converts one into the

    other.'
  assessment: targeted contract and inherited invariant context
  line: 409
- id: TC-027:L504
  path: spec/kani/matrix/TC-027-kani-execution-evidence.md
  role: examined
  excerpt: 'compile/adaptation gaps are reported, not bypassed with named compatibility, helper identity

    override, skipped tests or invented coverage. All guarantees precede complete IR-639/MVP acceptance.'
  assessment: targeted contract and inherited invariant context
  line: 504
- id: TC-027:L507
  path: spec/kani/matrix/TC-027-kani-execution-evidence.md
  role: examined
  excerpt: 'Scalar proof-content digest (FR-017-AC-45 to AC-50): the v3 extensions below are

    **Planned / UNRUN**; existing v2 controls do not establish them. Run the paired runner on a genuine routed

    `x + 1` package. (1) Assert the receipt''s `DigestRecord` has domain `sha256-jcs` and 32 bytes equal

    to `as_bytes()`, and recompute SHA-256 of the RFC 8785 encoding of the explicit `agent-ix/cg-kani-scalar-proof-content/v3`
    schema object and the sorted

    artifact list in the test, independently of the runner''s encoder. (2) Assert the exposed entry

    paths are exactly the AC-46 set in UTF-8'
  assessment: targeted contract and inherited invariant context
  line: 507
- id: StR-001
  path: spec/core/stakeholder/StR-001-traceable-generation.md
  role: context_only
  excerpt: 'Assurance engineers require that the generator shall derive executable, property-test, proof, and

    coverage artifacts reproducibly from one validated contract without hiding unsupported semantics,

    vacuity, rejection, or assumptions.'
  assessment: source/ownership grounding; no implementation credit
- id: AD-004:run
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: context_only
  excerpt: '| Launch, capture cap, timeout, process-group kill, harness-in-crate check | `kani/run/` | process only
    |'
  assessment: source/ownership grounding; no implementation credit
- id: AD-004:profile
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: context_only
  excerpt: "- L-11. The oracle crate manifest template, the runtime dependency spelling and every contract\n  and\
    \ schema spelling are defined in `core/profile.rs` once."
  assessment: source/ownership grounding; no implementation credit
- id: CG:ScalarLimits
  path: src/lib.rs
  role: context_only
  excerpt: pub use quire_contract_runtime::exact::ScalarLimits;
  assessment: source/ownership grounding; no implementation credit
- id: IR17:ScalarLimits
  path: /home/peter/dev/worktrees/kani-lead-codex/ir17-observer-spec/review/context/ir17/src/lib.rs
  role: context_only
  excerpt: pub use quire_exact::ScalarLimits;
  assessment: source/ownership grounding; no implementation credit
- id: IR17:adapter
  path: /home/peter/dev/worktrees/kani-lead-codex/ir17-observer-spec/review/context/ir17/src/kani/run/scalar.rs
  role: context_only
  excerpt: '#[path = "../src/lib.rs"] mod generated;

    use quire_exact as rt;'
  assessment: source/ownership grounding; no implementation credit
- id: CG:adapter
  path: src/kani/run/scalar.rs
  role: context_only
  excerpt: '#[path = "../lib.rs"] mod generated;

    use quire_contract_runtime::exact as rt;'
  assessment: source/ownership grounding; no implementation credit
- id: CG:schema
  path: src/kani/run/scalar.rs
  role: context_only
  excerpt: 'const SCALAR_PROOF_SCHEMA: &str = "agent-ix/cg-kani-scalar-proof-content/v2";'
  assessment: source/ownership grounding; no implementation credit
- id: EXACT:feature
  path: /home/peter/dev/quire-exact/Cargo.toml
  role: context_only
  excerpt: '# The ordered charge log (`Meter::admitted_charges`). A production

    # meter keeps a count only; tests that assert exact charge sequences enable

    # this through a dev-dependency, which resolver 2 turns on only for test

    # builds. A normal or build dependency must never enable it.

    test-support = []'
  assessment: source/ownership grounding; no implementation credit
- id: ADR-013:O21
  path: /home/peter/dev/quire-spec-language/spec/decisions/ADR-013-canonical-type-package-conversion-ownership.md
  role: context_only
  excerpt: '| Evaluation resource limit | `quire-exact` `Meter`, `ChargePoint`, `Incomplete` under `quire.value.accounting/v1`
    | FR-323 `limits` (`ScalarLimitsV1`, `PopulationAdmissionLimitsV1`) |'
  assessment: source/ownership grounding; no implementation credit
- id: CG:v2-literal
  path: src/kani/run/scalar.rs
  role: context_only
  excerpt: "            (\"Cargo.lock\", \"lock\\n\"),\n            (\"Cargo.toml\", \"manifest\"),\n          \
    \  (\"claim-map.json\", \"{}\"),\n            (\"kani-obligations/m.json\", \"record\"),\n            (\"src/bin/ir791_native.rs\"\
    , \"adapter\"),\n            (\"src/lib.rs\", \"harness\"),\n            (\"src/oracle-artifacts-lib.rs\", \"\
    oracle\"),\n        ]\n        .into_iter()\n        .map(|(path, contents)| (path.to_owned(), contents.as_bytes().to_vec()))\n\
    \        .collect()\n    }\n"
  assessment: source inspection only; existing test tags do not credit v3
- id: CG:eight-retained
  path: tests/it/kani_obligations.rs
  role: context_only
  excerpt: "        vec![\n            \"Cargo.lock\",\n            \"Cargo.toml\",\n            \"claim-map.json\"\
    ,\n            &harness.record.path,\n            \"source-maps/lib.json\",\n            \"src/bin/ir791_native.rs\"\
    ,\n            \"src/lib.rs\",\n            \"src/oracle-artifacts-lib.rs\"\n        ]\n    );\n    let expected\
    \ = independent_digest(entries);\n    for index in 0..entries.len() {\n        for mutation in 0..5 {\n    \
    \        let mut changed = entries.to_vec();"
  assessment: source inspection only; existing test tags do not credit v3
- id: CG:consumer
  path: tests/it/kani_obligations.rs
  role: context_only
  excerpt: "#[test]\n#[trace(\"TC-027\", \"FR-017-AC-51\")]\nfn tc_027_scalar_limits_is_available_at_codegen_root_with_request_type_identity()\
    \ {\n    use quire_contract_codegen::{KaniScalarNativeRequest, ScalarLimits};\n\n    // This function's return\
    \ type proves that the public root export is the\n    // exact type of the existing request field, without naming\
    \ Runtime.\n    fn request_limits<'a>(request: &'a KaniScalarNativeRequest<'a>) -> &'a ScalarLimits {\n    \
    \    &request.scalar_limits\n    }\n\n    let limits = ScalarLimits {\n        integer_bits: 64,\n        decimal_digits:\
    \ 20,\n       "
  assessment: source inspection only; existing test tags do not credit v3
- id: CG:profile
  path: src/core/profile.rs
  role: context_only
  excerpt: '//! The version profile: spellings this build emits (AD-004 `core/profile`, L-11).

    //!

    //! The emitted oracle crate''s `Cargo.toml` is written here once and called by the three oracle

    //! emitters. Its runtime dependency names the runtime source the way this crate''s own `Cargo.toml`

    //! does: the git URL and `branch = "main"`. The commit is not spelled in the emitted manifest; a

    //! consumer''s lockfile records it, as this repository''s `Cargo.lock` does for its own build.


    /// The runtime source every emitted manifest depends on, spelled as in this crate''s `Cargo.toml`:

    ///'
  assessment: source inspection only; existing test tags do not credit v3
- id: CG:embedded-oracle
  path: src/kani/generate/render.rs
  role: context_only
  excerpt: "        KaniPrimitiveType::I64 => i64::try_from(value).map(i64_literal).map_err(|error| {\n        \
    \    UnsupportedObligation::InvalidGeneratedSyntax {\n                error: error.to_string(),\n          \
    \  }\n        }),\n        KaniPrimitiveType::Boolean => Ok((value != 0).to_string()),\n    }\n}\npub(super)\
    \ fn render(spec: &HarnessSpec) -> Result<Artifact, UnsupportedObligation> {\n    let path = spec.path();\n\
    \    let mut source = format!(\"// SPDX-License-Identifier: MIT OR Apache-2.0\\n// @generated by quire-contract-codegen.\
    \ Do not edit.\\n// Obligation:"
  assessment: source inspection only; existing test tags do not credit v3
- id: CG:doc-scope
  path: tests/it/kani_gate.rs
  role: context_only
  excerpt: "    }\n    for path in [\n        \"spec/kani/functional/FR-017-kani-execution-evidence.md\",\n    \
    \    \"reviews/REV-018-bound-coverage-observations.md\",\n        \"src/strategy/mod.rs\",\n        \"src/evidence/mod.rs\"\
    ,\n        \"Makefile\",\n        \".github/workflows/ci.yml\",\n    ] {"
  assessment: source inspection only; existing test tags do not credit v3
- id: ADR-013:O13
  path: /home/peter/dev/quire-spec-language/spec/decisions/ADR-013-canonical-type-package-conversion-ownership.md
  role: context_only
  excerpt: '| Owner | `quire-exact` kernel crate in the QSL repository (AD-016 Owner decision 2), execution stage.
    Consumers: QSL evaluator, RT host ABI, and CG oracles for kernel types only. A CG oracle never computes its
    expectation with the kernel operation under proof, or with a kernel helper that operation calls (ADR-011 §2.3
    rule 8). The expectation for a kernel operation is derived from the QSpec operation vectors or a checked-in
    specification model that calls no `quire-exact` operation. AD-016 arrow 4 and its crate-table line 406 are stale
    here. Remaining work: agent-'
  assessment: contextual claim or review instructions; not independent approval
- id: Research
  path: /home/peter/dev/worktrees/kani-lead-codex/ir17/resume-2026-10-20/observer-design-grounding.md
  role: context_only
  excerpt: A private companion package can plausibly compile the canonical native adapter **in place as an example**,
    with ordered-log activation exclusively through its own dev-dependency. The actual imports support this design;
    no new kernel API is required. It cannot satisfy the existing exact v2 artifact-set contract unchanged. It requires
    an explicit versioned custody amendment, implementation and confined validation. Neither a build success nor
    readiness is claimed.
  assessment: contextual claim or review instructions; not independent approval
- id: Request
  path: /home/peter/dev/worktrees/kani-lead-codex/ir17-observer-spec/request.md
  role: context_only
  excerpt: 'Desired capability: a reviewed, explicit execution/custody specification for dev-only native observation
    that retains every original production artifact unchanged and binds every actual companion build input under
    an appropriately versioned canonical identity. Read-only grounding found a companion package can reference the
    canonical adapter in place as an example with both exact/runtime dev-dependencies; this is a candidate, not
    a compiled result or product waiver.'
  assessment: contextual claim or review instructions; not independent approval
- id: ReviewBrief
  path: /home/peter/dev/worktrees/kani-lead-codex/ir17-observer-spec/reviewer-brief.md
  role: context_only
  excerpt: Frozen specworktree /home/peter/dev/worktrees/ir17-observer-spec, branchcode/ir-936-observer-custody-spec,
    exacthead496233a16ccfc3570bca0fe20bc8136de35f6312, base7c8e62910cd56387a4cdfc5c32a7b1f39693982c, PR475 https://github.com/agent-ix/quire-contract-codegen/pull/475,
    IR936 childIR17. Read request.md, author-brief.md and actualdiff; rootnewestsate KANI-CODE-LEAD-STATE. Research
    ir17/resume-2026-10-20/observer-design-grounding.md and frozen implementationparentir17b21e40e READONLY are
    contextualclaims toverify, not independentapproval. Requirements newbehaviorPla
  assessment: contextual claim or review instructions; not independent approval
- id: AuthorBrief
  path: /home/peter/dev/worktrees/kani-lead-codex/ir17-observer-spec/author-brief.md
  role: context_only
  excerpt: Root authorizes a focused technical specification reconciliation required by existing IR17 adoption,
    under normal spec-first/review process, preserving all current product goals and stronger evidence boundaries.
    This is NOT an exception/waiver/resource or feature-policy change. Define the appropriate explicit new canonical
    identity schema/custody for a private observer companion package if actual source/contracts support it. Existing
    AC49 explicitly requires a schema change when artifact inputs/paths change. Retain original emitted manifest/source/claim-map/source
  assessment: contextual claim or review instructions; not independent approval
- id: LeadState
  path: /home/peter/dev/worktrees/KANI-CODE-LEAD-STATE.md
  role: context_only
  excerpt: '# LATEST ACTIVE UPDATE — 2026-10-20T20:43:51.129625+00:00'
  assessment: contextual claim or review instructions; not independent approval
findings: []
bindings:
- test_id: tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  path: tests/it/kani_obligations.rs
  line: 3054
  ac_id: FR-017-AC-27
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::prepared_source_bind_is_read_only_to_the_backend
  path: src/kani/run/namespace.rs
  line: 544
  ac_id: FR-017-AC-28
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::tc_027_changed_staged_bytes_and_unexpected_files_refuse_identity
  path: src/kani/run/scalar.rs
  line: 1422
  ac_id: FR-017-AC-28
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  path: tests/it/kani_obligations.rs
  line: 3054
  ac_id: FR-017-AC-28
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tc_027_attested_scalar_rejects_preflight_mismatches_without_launch
  path: tests/it/kani_obligations.rs
  line: 3316
  ac_id: FR-017-AC-28
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::tc_027_scalar_proof_content_identity_tracks_staged_bytes_only
  path: src/kani/run/scalar.rs
  line: 1443
  ac_id: FR-017-AC-29
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  path: tests/it/kani_obligations.rs
  line: 3054
  ac_id: FR-017-AC-29
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tc_027_attested_scalar_rejects_preflight_mismatches_without_launch
  path: tests/it/kani_obligations.rs
  line: 3316
  ac_id: FR-017-AC-29
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  path: tests/it/kani_obligations.rs
  line: 3054
  ac_id: FR-017-AC-30
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tc_027_attested_scalar_subtraction_fixture_has_two_bounded_operands
  path: tests/it/kani_obligations.rs
  line: 3134
  ac_id: FR-017-AC-31
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tc_027_attested_scalar_subtraction_receipts_bind_both_operand_orders
  path: tests/it/kani_obligations.rs
  line: 3147
  ac_id: FR-017-AC-31
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::frozen_outer_code_and_class_table_has_exactly_twelve_distinct_codes
  path: src/kani/run/scalar.rs
  line: 1847
  ac_id: FR-017-AC-41
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::lock_failures_distinguish_codegen_configuration_from_staged_lock
  path: src/kani/run/scalar.rs
  line: 1683
  ac_id: FR-017-AC-42
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::runtime_revision_read_preserves_dependency_mismatch_and_unreadable_lock_faults
  path: src/kani/run/scalar.rs
  line: 1703
  ac_id: FR-017-AC-42
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::frozen_outer_code_and_class_table_has_exactly_twelve_distinct_codes
  path: src/kani/run/scalar.rs
  line: 1847
  ac_id: FR-017-AC-42
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::frozen_evidence_classification_covers_all_outcomes_reasons_and_causes_unchanged
  path: src/kani/run/scalar.rs
  line: 1887
  ac_id: FR-017-AC-42
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::frozen_nested_refusal_table_preserves_nested_output_codes
  path: src/kani/run/scalar.rs
  line: 1975
  ac_id: FR-017-AC-42
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::native_launch_failure_is_an_environment_fault_without_a_receipt
  path: src/kani/run/scalar.rs
  line: 2047
  ac_id: FR-017-AC-42
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: bounded_single_run_standins_preserve_cancelled_and_timed_out_evidence
  path: tests/it/kani_scalar_native_private.rs
  line: 121
  ac_id: FR-017-AC-42
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: seeded_false_bound_harness_produces_a_paired_completion_refusal
  path: tests/it/kani_scalar_native_private.rs
  line: 202
  ac_id: FR-017-AC-42
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: bounded_paired_runner_propagates_the_native_helpers_oracle_stop
  path: tests/it/kani_scalar_native_private.rs
  line: 246
  ac_id: FR-017-AC-42
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::native_decoder_retains_actual_limits_and_rejects_each_mismatch
  path: src/kani/run/scalar.rs
  line: 1480
  ac_id: FR-017-AC-43
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::literal_seven_entry_jcs_golden_and_mutation_controls
  path: src/kani/run/scalar.rs
  line: 1565
  ac_id: FR-017-AC-43
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::regression_bare_native_result_records_are_counted_and_refused
  path: src/kani/run/scalar.rs
  line: 2073
  ac_id: FR-017-AC-43
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::lock_failures_distinguish_codegen_configuration_from_staged_lock
  path: src/kani/run/scalar.rs
  line: 1683
  ac_id: FR-017-AC-44
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::frozen_outer_code_and_class_table_has_exactly_twelve_distinct_codes
  path: src/kani/run/scalar.rs
  line: 1847
  ac_id: FR-017-AC-44
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::frozen_evidence_classification_covers_all_outcomes_reasons_and_causes_unchanged
  path: src/kani/run/scalar.rs
  line: 1887
  ac_id: FR-017-AC-44
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::frozen_nested_refusal_table_preserves_nested_output_codes
  path: src/kani/run/scalar.rs
  line: 1975
  ac_id: FR-017-AC-44
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::literal_seven_entry_jcs_golden_and_mutation_controls
  path: src/kani/run/scalar.rs
  line: 1565
  ac_id: FR-017-AC-45
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  path: tests/it/kani_obligations.rs
  line: 3054
  ac_id: FR-017-AC-45
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: cleanup_success_and_failure_preserve_real_receipt_after_snapshot_removal
  path: tests/it/kani_scalar_native_private.rs
  line: 53
  ac_id: FR-017-AC-45
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::literal_seven_entry_jcs_golden_and_mutation_controls
  path: src/kani/run/scalar.rs
  line: 1565
  ac_id: FR-017-AC-46
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::owned_measurement_survives_explicit_cleanup_and_reports_invalid_utf8
  path: src/kani/run/scalar.rs
  line: 1628
  ac_id: FR-017-AC-46
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::runtime_revision_read_preserves_dependency_mismatch_and_unreadable_lock_faults
  path: src/kani/run/scalar.rs
  line: 1703
  ac_id: FR-017-AC-46
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::regression_non_utf8_lock_uses_the_early_runtime_revision_read_path
  path: src/kani/run/scalar.rs
  line: 2055
  ac_id: FR-017-AC-46
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  path: tests/it/kani_obligations.rs
  line: 3054
  ac_id: FR-017-AC-46
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  path: tests/it/kani_obligations.rs
  line: 3054
  ac_id: FR-017-AC-47
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tc_027_attested_scalar_subtraction_receipts_bind_both_operand_orders
  path: tests/it/kani_obligations.rs
  line: 3147
  ac_id: FR-017-AC-47
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::owned_measurement_survives_explicit_cleanup_and_reports_invalid_utf8
  path: src/kani/run/scalar.rs
  line: 1628
  ac_id: FR-017-AC-48
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  path: tests/it/kani_obligations.rs
  line: 3054
  ac_id: FR-017-AC-48
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: cleanup_success_and_failure_preserve_real_receipt_after_snapshot_removal
  path: tests/it/kani_scalar_native_private.rs
  line: 53
  ac_id: FR-017-AC-48
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::literal_seven_entry_jcs_golden_and_mutation_controls
  path: src/kani/run/scalar.rs
  line: 1565
  ac_id: FR-017-AC-49
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::native_decoder_retains_actual_limits_and_rejects_each_mismatch
  path: src/kani/run/scalar.rs
  line: 1480
  ac_id: FR-017-AC-50
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tests::regression_bare_native_result_records_are_counted_and_refused
  path: src/kani/run/scalar.rs
  line: 2073
  ac_id: FR-017-AC-50
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  path: tests/it/kani_obligations.rs
  line: 3054
  ac_id: FR-017-AC-50
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tc_027_attested_scalar_subtraction_receipts_bind_both_operand_orders
  path: tests/it/kani_obligations.rs
  line: 3147
  ac_id: FR-017-AC-50
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: cleanup_success_and_failure_preserve_real_receipt_after_snapshot_removal
  path: tests/it/kani_scalar_native_private.rs
  line: 53
  ac_id: FR-017-AC-50
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
- test_id: tc_027_scalar_limits_is_available_at_codegen_root_with_request_type_identity
  path: tests/it/kani_obligations.rs
  line: 3534
  ac_id: FR-017-AC-51
  binding_kind: test_function
  assessment: structural association only; semantic validity and execution not credited
```

## Source-location clarification

The original scope `line` values are paragraph/table anchors. The exact excerpt start locations below clarify those anchors without rewriting any original scope, finding, verdict or private comment. All excerpts remain verbatim at the recorded paths; scope ordering and binding records are unchanged.

```yaml
source_locations:
- id: FR-017:L20
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '## Description'
  paragraph_anchor: 20
  excerpt_start_line: 21
  applies_to:
  - artifact: SR-14120
    scope_index: 0
  - artifact: SR-14121
    scope_index: 0
  - artifact: SR-14122
    scope_index: 0
  - artifact: SR-14123
    scope_index: 0
  - artifact: SR-14124
    scope_index: 0
  - artifact: SR-14125
    scope_index: 0
  - artifact: SR-14126
    scope_index: 0
  - artifact: SR-14127
    scope_index: 0
  - artifact: SR-14128
    scope_index: 0
- id: FR-017:L692
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '### Scalar proof-content identity as the FR-357 observation digest'
  paragraph_anchor: 692
  excerpt_start_line: 693
  applies_to:
  - artifact: SR-14120
    scope_index: 11
  - artifact: SR-14121
    scope_index: 11
  - artifact: SR-14122
    scope_index: 11
  - artifact: SR-14123
    scope_index: 11
  - artifact: SR-14124
    scope_index: 11
  - artifact: SR-14125
    scope_index: 11
  - artifact: SR-14126
    scope_index: 11
  - artifact: SR-14127
    scope_index: 11
  - artifact: SR-14128
    scope_index: 11
- id: FR-017-CON-2
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-CON-2 | The generator SHALL NOT report a generation-time classification as an execution outcome.
    | Integrity | Test (TC-027) |'
  paragraph_anchor: 822
  excerpt_start_line: 823
  applies_to:
  - artifact: SR-14120
    scope_index: 17
  - artifact: SR-14121
    scope_index: 17
  - artifact: SR-14122
    scope_index: 17
  - artifact: SR-14123
    scope_index: 17
  - artifact: SR-14124
    scope_index: 17
  - artifact: SR-14125
    scope_index: 17
  - artifact: SR-14126
    scope_index: 17
  - artifact: SR-14127
    scope_index: 17
  - artifact: SR-14128
    scope_index: 17
- id: FR-017-AC-2
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-2 | An absent launcher is refused with a typed reason naming its path before anything runs.
    | Test (TC-027) |'
  paragraph_anchor: 827
  excerpt_start_line: 829
  applies_to:
  - artifact: SR-14120
    scope_index: 19
  - artifact: SR-14121
    scope_index: 19
  - artifact: SR-14122
    scope_index: 19
  - artifact: SR-14123
    scope_index: 19
  - artifact: SR-14124
    scope_index: 19
  - artifact: SR-14125
    scope_index: 19
  - artifact: SR-14126
    scope_index: 19
  - artifact: SR-14127
    scope_index: 19
  - artifact: SR-14128
    scope_index: 19
- id: FR-017-AC-4
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-4 | A run is verified only when the process exited successfully, the report states success,
    and every cover property is reported satisfied; a successful run with an unsatisfied cover is cover-unsatisfied
    with its satisfied and total counts; a report of success from an unsuccessfully exited process, a report with
    no cover property and a report with no successful check are each inconclusive with their own reason; console
    banners never decide the verdict. | Test (TC-027) |'
  paragraph_anchor: 827
  excerpt_start_line: 830
  applies_to:
  - artifact: SR-14120
    scope_index: 20
  - artifact: SR-14121
    scope_index: 20
  - artifact: SR-14122
    scope_index: 20
  - artifact: SR-14123
    scope_index: 20
  - artifact: SR-14124
    scope_index: 20
  - artifact: SR-14125
    scope_index: 20
  - artifact: SR-14126
    scope_index: 20
  - artifact: SR-14127
    scope_index: 20
  - artifact: SR-14128
    scope_index: 20
- id: FR-017-AC-5
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-5 | A failed non-unwinding check with a concrete playback is falsified carrying that playback
    verbatim and never the playback of a satisfied cover; a failure with no playback is inconclusive for that reason;
    a failed unwinding assertion is inconclusive as an exhausted bound rather than falsified, and a succeeded unwinding
    check in a results listing is not a failure. | Test (TC-027) |'
  paragraph_anchor: 827
  excerpt_start_line: 831
  applies_to:
  - artifact: SR-14120
    scope_index: 21
  - artifact: SR-14121
    scope_index: 21
  - artifact: SR-14122
    scope_index: 21
  - artifact: SR-14123
    scope_index: 21
  - artifact: SR-14124
    scope_index: 21
  - artifact: SR-14125
    scope_index: 21
  - artifact: SR-14126
    scope_index: 21
  - artifact: SR-14127
    scope_index: 21
  - artifact: SR-14128
    scope_index: 21
- id: FR-017-AC-6
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-6 | The execution evidence of a single run carries the obligation kind, the harness path,
    the launcher path, the complete argument vector, the unwind bound, the solver, the exit code and the outcome.
    | Test (TC-027) |'
  paragraph_anchor: 827
  excerpt_start_line: 832
  applies_to:
  - artifact: SR-14120
    scope_index: 22
  - artifact: SR-14121
    scope_index: 22
  - artifact: SR-14122
    scope_index: 22
  - artifact: SR-14123
    scope_index: 22
  - artifact: SR-14124
    scope_index: 22
  - artifact: SR-14125
    scope_index: 22
  - artifact: SR-14126
    scope_index: 22
  - artifact: SR-14127
    scope_index: 22
  - artifact: SR-14128
    scope_index: 22
- id: FR-017-AC-7
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-7 | A crate whose library source does not contain the harness source byte for byte is refused,
    and no backend runs. | Test (TC-027) |'
  paragraph_anchor: 827
  excerpt_start_line: 833
  applies_to:
  - artifact: SR-14120
    scope_index: 23
  - artifact: SR-14121
    scope_index: 23
  - artifact: SR-14122
    scope_index: 23
  - artifact: SR-14123
    scope_index: 23
  - artifact: SR-14124
    scope_index: 23
  - artifact: SR-14125
    scope_index: 23
  - artifact: SR-14126
    scope_index: 23
  - artifact: SR-14127
    scope_index: 23
  - artifact: SR-14128
    scope_index: 23
- id: FR-017-AC-11
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-11 | A routed FR-022/FR-014 exact-scalar harness (`KaniScalarObligationHarness`) runs through
    `execute_kani_obligation` and `kani_launch_command` the same way an FR-015 contract harness does: a crate whose
    library source lacks its generated source byte for byte is `HarnessNotInCrate`, its covers classify a run identically
    (all satisfied is verified, an unsatisfied one is cover-unsatisfied, none printed is inconclusive), and its
    evidence carries `None` for obligation kind, since an exact-scalar claim carries no contract role. | Test (TC-027)
    |'
  paragraph_anchor: 827
  excerpt_start_line: 834
  applies_to:
  - artifact: SR-14120
    scope_index: 24
  - artifact: SR-14121
    scope_index: 24
  - artifact: SR-14122
    scope_index: 24
  - artifact: SR-14123
    scope_index: 24
  - artifact: SR-14124
    scope_index: 24
  - artifact: SR-14125
    scope_index: 24
  - artifact: SR-14126
    scope_index: 24
  - artifact: SR-14127
    scope_index: 24
  - artifact: SR-14128
    scope_index: 24
- id: FR-017-AC-12
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-12 | Real Kani captures of a verified run, a falsified run with a playback, an exhausted
    unwind bound, a run whose only check is unreachable, a partly satisfied cover and a run with no cover each parse
    into the expected typed report of harness status, successful checks, cover counts and failed checks, and classify
    to the expected outcome; the falsifying playback block passes through verbatim. | Test (TC-027) |'
  paragraph_anchor: 827
  excerpt_start_line: 835
  applies_to:
  - artifact: SR-14120
    scope_index: 25
  - artifact: SR-14121
    scope_index: 25
  - artifact: SR-14122
    scope_index: 25
  - artifact: SR-14123
    scope_index: 25
  - artifact: SR-14124
    scope_index: 25
  - artifact: SR-14125
    scope_index: 25
  - artifact: SR-14126
    scope_index: 25
  - artifact: SR-14127
    scope_index: 25
  - artifact: SR-14128
    scope_index: 25
- id: FR-017-AC-13
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-13 | A run whose process exited successfully and whose backend reported success with zero
    successful checks is inconclusive with the vacuous-proof reason, never verified, except that a precondition
    harness, which asserts nothing beyond its cover, is decided by its cover summary. | Test (TC-027) |'
  paragraph_anchor: 827
  excerpt_start_line: 836
  applies_to:
  - artifact: SR-14120
    scope_index: 26
  - artifact: SR-14121
    scope_index: 26
  - artifact: SR-14122
    scope_index: 26
  - artifact: SR-14123
    scope_index: 26
  - artifact: SR-14124
    scope_index: 26
  - artifact: SR-14125
    scope_index: 26
  - artifact: SR-14126
    scope_index: 26
  - artifact: SR-14127
    scope_index: 26
  - artifact: SR-14128
    scope_index: 26
- id: FR-017-AC-14
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-14 | A launcher whose stdout or stderr carries more than 8 MiB times its harness count is
    stopped, its process group is killed, and the run is refused with `kani_output_over_limit` naming the stream,
    the limit and the count, with no outcome and no retained text; a launcher whose stream is exactly the limit
    completes with its real exit status and the whole stream retained. No stream is ever truncated and then classified.
    | Test (TC-043) |'
  paragraph_anchor: 827
  excerpt_start_line: 837
  applies_to:
  - artifact: SR-14120
    scope_index: 27
  - artifact: SR-14121
    scope_index: 27
  - artifact: SR-14122
    scope_index: 27
  - artifact: SR-14123
    scope_index: 27
  - artifact: SR-14124
    scope_index: 27
  - artifact: SR-14125
    scope_index: 27
  - artifact: SR-14126
    scope_index: 27
  - artifact: SR-14127
    scope_index: 27
  - artifact: SR-14128
    scope_index: 27
- id: FR-017-AC-15
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-15 | A timeout too large to add to the current instant never elapses and does not panic.
    | Test (TC-027) |'
  paragraph_anchor: 827
  excerpt_start_line: 838
  applies_to:
  - artifact: SR-14120
    scope_index: 28
  - artifact: SR-14121
    scope_index: 28
  - artifact: SR-14122
    scope_index: 28
  - artifact: SR-14123
    scope_index: 28
  - artifact: SR-14124
    scope_index: 28
  - artifact: SR-14125
    scope_index: 28
  - artifact: SR-14126
    scope_index: 28
  - artifact: SR-14127
    scope_index: 28
  - artifact: SR-14128
    scope_index: 28
- id: FR-017-AC-16
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-16 | The launcher''s capture threads are stopped and joined on every outcome: they return
    what the launcher wrote before it ended, stop while a write end is still held open, and stop within their drain
    limit while a straggler keeps writing, so a process holding a pipe open does not delay the return. | Test (TC-027)
    |'
  paragraph_anchor: 827
  excerpt_start_line: 839
  applies_to:
  - artifact: SR-14120
    scope_index: 29
  - artifact: SR-14121
    scope_index: 29
  - artifact: SR-14122
    scope_index: 29
  - artifact: SR-14123
    scope_index: 29
  - artifact: SR-14124
    scope_index: 29
  - artifact: SR-14125
    scope_index: 29
  - artifact: SR-14126
    scope_index: 29
  - artifact: SR-14127
    scope_index: 29
  - artifact: SR-14128
    scope_index: 29
- id: FR-017-AC-17
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-17 | A run that times out has the whole group the launcher leads killed, a real grandchild
    included. | Test (TC-027) |'
  paragraph_anchor: 827
  excerpt_start_line: 840
  applies_to:
  - artifact: SR-14120
    scope_index: 30
  - artifact: SR-14121
    scope_index: 30
  - artifact: SR-14122
    scope_index: 30
  - artifact: SR-14123
    scope_index: 30
  - artifact: SR-14124
    scope_index: 30
  - artifact: SR-14125
    scope_index: 30
  - artifact: SR-14126
    scope_index: 30
  - artifact: SR-14127
    scope_index: 30
  - artifact: SR-14128
    scope_index: 30
- id: FR-017-AC-18
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-18 | A report that is malformed, of an unknown check or harness status, of another schema
    version, that holds other than one harness result in a single run, or whose harness states success while it
    lists a failed, errored, undetermined or unknown check (of any class, including an unwinding assertion) is refused
    with its own typed cause and never classified, for every obligation kind; a run that exited successfully and
    exported no report is refused, and one that exited unsuccessfully and exported none is `NoVerdict`. | Test (TC-027)
    |'
  paragraph_anchor: 827
  excerpt_start_line: 841
  applies_to:
  - artifact: SR-14120
    scope_index: 31
  - artifact: SR-14121
    scope_index: 31
  - artifact: SR-14122
    scope_index: 31
  - artifact: SR-14123
    scope_index: 31
  - artifact: SR-14124
    scope_index: 31
  - artifact: SR-14125
    scope_index: 31
  - artifact: SR-14126
    scope_index: 31
  - artifact: SR-14127
    scope_index: 31
  - artifact: SR-14128
    scope_index: 31
- id: FR-017-AC-19
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-19 | The generator appends `-Z unstable-options --export-json /proc/self/fd/N` after unchanged
    harness options, with actual child-only anonymous pipe writer N >= 5, and records the actual argv bytes. It
    uses per-run unnamed report authority, never a named target-directory report or fallback. FR-034 AC-32/33 own
    collector bounds, resource-stop classification, kernel lifetime and immutable final handoff. Zero-byte EOF means
    no exported report; nonempty partial/malformed content is refused under AC-18 unless an independently established
    resource/deadline stop'
  paragraph_anchor: 827
  excerpt_start_line: 842
  applies_to:
  - artifact: SR-14120
    scope_index: 32
  - artifact: SR-14121
    scope_index: 32
  - artifact: SR-14122
    scope_index: 32
  - artifact: SR-14123
    scope_index: 32
  - artifact: SR-14124
    scope_index: 32
  - artifact: SR-14125
    scope_index: 32
  - artifact: SR-14126
    scope_index: 32
  - artifact: SR-14127
    scope_index: 32
  - artifact: SR-14128
    scope_index: 32
- id: FR-017-AC-20
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-20 | The evidence and the classified run list every check of a real run with its id, class,
    source file and line and status, a line stated as unknown is absent, and a non-numeric line or a check with
    no location is a refused report; the view serializes as `id`, `class`, `location { file, line }` and `status`
    and is not deserializable. | Test (TC-027) |'
  paragraph_anchor: 827
  excerpt_start_line: 843
  applies_to:
  - artifact: SR-14120
    scope_index: 33
  - artifact: SR-14121
    scope_index: 33
  - artifact: SR-14122
    scope_index: 33
  - artifact: SR-14123
    scope_index: 33
  - artifact: SR-14124
    scope_index: 33
  - artifact: SR-14125
    scope_index: 33
  - artifact: SR-14126
    scope_index: 33
  - artifact: SR-14127
    scope_index: 33
  - artifact: SR-14128
    scope_index: 33
- id: FR-017-AC-21
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-21 | N harnesses with equal option vectors (the harness selection removed) and equal identity
    `ProofCeilings` (`wall_clock` T and `memory_bytes` M) start exactly one launcher process whose argument vector
    holds one `--harness <module::harness> --exact` pair per member in request order, `--harness-timeout` T, then
    the shared options; N harnesses in G such groups start G processes; so N > 1 compatible harnesses start fewer
    than N processes; one harness starts one process with the single-run argument vector. Requests whose wall-clock
    or memory ceilings differ'
  paragraph_anchor: 827
  excerpt_start_line: 844
  applies_to:
  - artifact: SR-14120
    scope_index: 34
  - artifact: SR-14121
    scope_index: 34
  - artifact: SR-14122
    scope_index: 34
  - artifact: SR-14123
    scope_index: 34
  - artifact: SR-14124
    scope_index: 34
  - artifact: SR-14125
    scope_index: 34
  - artifact: SR-14126
    scope_index: 34
  - artifact: SR-14127
    scope_index: 34
  - artifact: SR-14128
    scope_index: 34
- id: FR-017-AC-22
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-22 | Over a real or captured batch report, members are matched by `module::harness` path
    (two members with the bare symbol `check` stay distinct), and each member''s outcome, checks and SUCCESS count
    come from its own entry: a falsified member beside a verified member leaves the verified member verified; a
    Success member in a non-zero exit with no Failure entry is inconclusive, and with one Failure entry is verified;
    a Failure entry with no checks and exit status `timeout` is inconclusive timed-out naming T while the others
    keep their results; a Failure entry'
  paragraph_anchor: 827
  excerpt_start_line: 845
  applies_to:
  - artifact: SR-14120
    scope_index: 35
  - artifact: SR-14121
    scope_index: 35
  - artifact: SR-14122
    scope_index: 35
  - artifact: SR-14123
    scope_index: 35
  - artifact: SR-14124
    scope_index: 35
  - artifact: SR-14125
    scope_index: 35
  - artifact: SR-14126
    scope_index: 35
  - artifact: SR-14127
    scope_index: 35
  - artifact: SR-14128
    scope_index: 35
- id: FR-017-AC-23
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-23 | A group''s report that lacks a requested harness, holds one twice or holds an unrequested
    one refuses the group with a typed cause and classifies no member; a member whose source is not in the crate
    refuses the whole batch with `HarnessNotInCrate` and launches nothing; a falsified member''s playback is the
    first counterexample block headed for its own path and never another member''s block, a member that fails two
    property checks (two counterexample blocks under its path, as real Kani prints) takes the first, as the same
    harness alone does, and neither it nor'
  paragraph_anchor: 827
  excerpt_start_line: 846
  applies_to:
  - artifact: SR-14120
    scope_index: 36
  - artifact: SR-14121
    scope_index: 36
  - artifact: SR-14122
    scope_index: 36
  - artifact: SR-14123
    scope_index: 36
  - artifact: SR-14124
    scope_index: 36
  - artifact: SR-14125
    scope_index: 36
  - artifact: SR-14126
    scope_index: 36
  - artifact: SR-14127
    scope_index: 36
  - artifact: SR-14128
    scope_index: 36
- id: FR-017-AC-24
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-24 | A launcher that exits on its own after leaving a real grandchild in its process group
    has that grandchild killed by the time the run returns, the same as on a timeout. | Test (TC-043) |'
  paragraph_anchor: 827
  excerpt_start_line: 847
  applies_to:
  - artifact: SR-14120
    scope_index: 37
  - artifact: SR-14121
    scope_index: 37
  - artifact: SR-14122
    scope_index: 37
  - artifact: SR-14123
    scope_index: 37
  - artifact: SR-14124
    scope_index: 37
  - artifact: SR-14125
    scope_index: 37
  - artifact: SR-14126
    scope_index: 37
  - artifact: SR-14127
    scope_index: 37
  - artifact: SR-14128
    scope_index: 37
- id: FR-017-AC-25
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-25 | A capture thread that panics, or whose poll or read of the stream errs, refuses the
    run with `kani_output_unread`; the run is never classified from empty text. | Test (TC-043) |'
  paragraph_anchor: 827
  excerpt_start_line: 848
  applies_to:
  - artifact: SR-14120
    scope_index: 38
  - artifact: SR-14121
    scope_index: 38
  - artifact: SR-14122
    scope_index: 38
  - artifact: SR-14123
    scope_index: 38
  - artifact: SR-14124
    scope_index: 38
  - artifact: SR-14125
    scope_index: 38
  - artifact: SR-14126
    scope_index: 38
  - artifact: SR-14127
    scope_index: 38
  - artifact: SR-14128
    scope_index: 38
- id: FR-017-AC-26
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-26 | A generated FR-035 caller Text-admission harness enters the same production execution
    path in single and compatible batch runs; its evidence carries the exact typed caller identity (profile, bounds
    and finite payload class) supplied with that harness and `None` for contract obligation kind. A matching successful
    report with a satisfied cover and successful agreement assertion classifies `Verified`; an unsatisfied cover
    classifies `CoverUnsatisfied` with counts; a failed agreement assertion with its own concrete playback classifies
    `Falsified` with that'
  paragraph_anchor: 827
  excerpt_start_line: 849
  applies_to:
  - artifact: SR-14120
    scope_index: 39
  - artifact: SR-14121
    scope_index: 39
  - artifact: SR-14122
    scope_index: 39
  - artifact: SR-14123
    scope_index: 39
  - artifact: SR-14124
    scope_index: 39
  - artifact: SR-14125
    scope_index: 39
  - artifact: SR-14126
    scope_index: 39
  - artifact: SR-14127
    scope_index: 39
  - artifact: SR-14128
    scope_index: 39
- id: FR-017-AC-27
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-27 | A genuine routed `x + 1` scalar harness from `generate_routed`, its complete emitted
    oracle package and one in-range concrete operand are passed to the paired runner. One real Kani run reports
    `Verified`, and the runner invokes the public oracle from the same staged library source at that operand with
    the request''s exact `ScalarLimits`; the receipt contains the Kani evidence and the actual typed runtime result
    of that invocation. A Kani verdict alone or a result reconstructed by the caller is insufficient. | Test (TC-027)
    |'
  paragraph_anchor: 827
  excerpt_start_line: 850
  applies_to:
  - artifact: SR-14120
    scope_index: 40
  - artifact: SR-14121
    scope_index: 40
  - artifact: SR-14122
    scope_index: 40
  - artifact: SR-14123
    scope_index: 40
  - artifact: SR-14124
    scope_index: 40
  - artifact: SR-14125
    scope_index: 40
  - artifact: SR-14126
    scope_index: 40
  - artifact: SR-14127
    scope_index: 40
  - artifact: SR-14128
    scope_index: 40
- id: FR-017-AC-28
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-28 | Before either child starts, a package or harness whose path or bytes differ from the
    retained routed output, a selected oracle symbol or argument order that differs from the retained scalar identity,
    or an operand count/value outside the retained ordered ranges is refused with a typed cause and no receipt.
    For an accepted request, the exact supplied `ScalarLimits` are passed unchanged to `KaniScalarNativeRequest`,
    used to initialize the native meter, and returned in the receipt; no hidden or substituted limit value is permitted.
    CG attests these supplied,'
  paragraph_anchor: 827
  excerpt_start_line: 851
  applies_to:
  - artifact: SR-14120
    scope_index: 41
  - artifact: SR-14121
    scope_index: 41
  - artifact: SR-14122
    scope_index: 41
  - artifact: SR-14123
    scope_index: 41
  - artifact: SR-14124
    scope_index: 41
  - artifact: SR-14125
    scope_index: 41
  - artifact: SR-14126
    scope_index: 41
  - artifact: SR-14127
    scope_index: 41
  - artifact: SR-14128
    scope_index: 41
- id: FR-017-AC-29
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-29 | The runner measures one canonical proof-content identity over the exact paths and bytes
    of every staged execution input: the complete generated package, retained harness, canonical native invocation
    adapter, observer manifest and both independently generated dependency locks. This identity binds the content
    actually proved and executed; it is not a tool-version or file-version tracking identity, and no separate identity
    is added for any staged item. The Kani and native invocations cannot write the staged source tree. A changed,
    replaced or additional'
  paragraph_anchor: 827
  excerpt_start_line: 852
  applies_to:
  - artifact: SR-14120
    scope_index: 42
  - artifact: SR-14121
    scope_index: 42
  - artifact: SR-14122
    scope_index: 42
  - artifact: SR-14123
    scope_index: 42
  - artifact: SR-14124
    scope_index: 42
  - artifact: SR-14125
    scope_index: 42
  - artifact: SR-14126
    scope_index: 42
  - artifact: SR-14127
    scope_index: 42
  - artifact: SR-14128
    scope_index: 42
- id: FR-017-AC-30
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-30 | The public receipt exposes the measured identity, scalar identity, ordered operands,
    exact supplied and meter-used `ScalarLimits`, verified Kani evidence and actual typed native outcome through
    read-only access. Callers cannot construct, deserialize or mutate a receipt, set its identity or outcome, or
    turn any non-verified, refused, inconclusive, faulted or malformed run into a receipt. | Test (TC-027) |'
  paragraph_anchor: 827
  excerpt_start_line: 853
  applies_to:
  - artifact: SR-14120
    scope_index: 43
  - artifact: SR-14121
    scope_index: 43
  - artifact: SR-14122
    scope_index: 43
  - artifact: SR-14123
    scope_index: 43
  - artifact: SR-14124
    scope_index: 43
  - artifact: SR-14125
    scope_index: 43
  - artifact: SR-14126
    scope_index: 43
  - artifact: SR-14127
    scope_index: 43
  - artifact: SR-14128
    scope_index: 43
- id: FR-017-AC-31
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-31 | The paired runner executes an order-sensitive routed scalar operation with two symbolic
    operands in two concrete operand orders. Each real Kani run is verified; the native invocation returns the result
    for that order; and each receipt binds its exact ordered operand vector and typed native result. Swapping the
    operands changes the observed result and receipt context while retaining the same generated proof-content identity.
    | Test (TC-027) |'
  paragraph_anchor: 827
  excerpt_start_line: 854
  applies_to:
  - artifact: SR-14120
    scope_index: 44
  - artifact: SR-14121
    scope_index: 44
  - artifact: SR-14122
    scope_index: 44
  - artifact: SR-14123
    scope_index: 44
  - artifact: SR-14124
    scope_index: 44
  - artifact: SR-14125
    scope_index: 44
  - artifact: SR-14126
    scope_index: 44
  - artifact: SR-14127
    scope_index: 44
  - artifact: SR-14128
    scope_index: 44
- id: FR-017-AC-32
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-32 | In the `make kani` lane, a crate whose library source holds the generated harness byte
    for byte and, beside it, an item that does not compile (a reference to an undefined name) is run through `execute_kani_obligation`:
    the run exits unsuccessfully, exports no report and is `Inconclusive` `NoVerdict` with its exit code, and its
    evidence carries a diagnostic record whose phase is `compile`, whose stdout excerpt holds the rustc error code
    and Kani''s `error: Failed to execute cargo` line and whose stderr excerpt holds Cargo''s `error: could not
    compile` line'
  paragraph_anchor: 827
  excerpt_start_line: 855
  applies_to:
  - artifact: SR-14120
    scope_index: 45
  - artifact: SR-14121
    scope_index: 45
  - artifact: SR-14122
    scope_index: 45
  - artifact: SR-14123
    scope_index: 45
  - artifact: SR-14124
    scope_index: 45
  - artifact: SR-14125
    scope_index: 45
  - artifact: SR-14126
    scope_index: 45
  - artifact: SR-14127
    scope_index: 45
  - artifact: SR-14128
    scope_index: 45
- id: FR-017-AC-33
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-33 | Over single-run launcher stand-ins that exit 1: a report of success beside the unsuccessful
    exit is `unknown` even when its stdout holds `Solving with CaDiCaL 3.0.0`; with no report, stdout holding the
    Kani banner, `Checking harness proofs::check...` and `Solving with CaDiCaL 3.0.0` is `solver`; the banner and
    the `Checking harness` line alone is `backend`; the banner and `error: Failed to execute cargo (exit status:
    101). Found 1 compilation errors.` is `compile`, as is stderr `error: could not compile` alone with the banner;
    a usage error on stderr and'
  paragraph_anchor: 827
  excerpt_start_line: 856
  applies_to:
  - artifact: SR-14120
    scope_index: 46
  - artifact: SR-14121
    scope_index: 46
  - artifact: SR-14122
    scope_index: 46
  - artifact: SR-14123
    scope_index: 46
  - artifact: SR-14124
    scope_index: 46
  - artifact: SR-14125
    scope_index: 46
  - artifact: SR-14126
    scope_index: 46
  - artifact: SR-14127
    scope_index: 46
  - artifact: SR-14128
    scope_index: 46
- id: FR-017-AC-34
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-34 | A redacted stream of exactly 8192 bytes is kept whole with `omittedBytes` 0; one of
    8193 bytes whose final line starts within the window keeps the longest line-aligned suffix of at most 8192 bytes
    with `omittedBytes` the byte count before it; a stream whose final line alone exceeds 8192 bytes keeps at most
    8192 bytes ending at its last byte, starting on a character boundary; a stream that is one 10000-byte line followed
    by `\n` keeps the last 8192 bytes of it, ending in `\n`, never an empty excerpt; a stream of a 10000-byte line,
    `\n`, `error: last\n`'
  paragraph_anchor: 827
  excerpt_start_line: 857
  applies_to:
  - artifact: SR-14120
    scope_index: 47
  - artifact: SR-14121
    scope_index: 47
  - artifact: SR-14122
    scope_index: 47
  - artifact: SR-14123
    scope_index: 47
  - artifact: SR-14124
    scope_index: 47
  - artifact: SR-14125
    scope_index: 47
  - artifact: SR-14126
    scope_index: 47
  - artifact: SR-14127
    scope_index: 47
  - artifact: SR-14128
    scope_index: 47
- id: FR-017-AC-35
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-35 | With crate directory `/h/u/ws/crate`, target directory `/h/u/ws/crate/target` and home
    directory `/h/u` supplied in the request, a stand-in whose streams print `(/h/u/ws/crate)`, `/h/u/ws/crate/src/lib.rs:6:21`,
    `/h/u/ws/crate/target/debug/x.out`, `/h/u/.kani/bin`, `/h/u/ws/crate2/src/lib.rs`, `/opt/tool/bin`, `https://host/x/y`,
    `and/or`, `/`, `2026-10-16T22:27:04.275975036Z`, `Runtime Symex: 0.00331318s`, `Runtime Solver: 1.385e-05s`,
    `in 0.02s`, `check_ttl_30s`, `u32s`, `pid 4242` and `thread ''main'' (4242) panicked` yields, in order, `(<CRATE_DIR>)`,'
  paragraph_anchor: 827
  excerpt_start_line: 858
  applies_to:
  - artifact: SR-14120
    scope_index: 48
  - artifact: SR-14121
    scope_index: 48
  - artifact: SR-14122
    scope_index: 48
  - artifact: SR-14123
    scope_index: 48
  - artifact: SR-14124
    scope_index: 48
  - artifact: SR-14125
    scope_index: 48
  - artifact: SR-14126
    scope_index: 48
  - artifact: SR-14127
    scope_index: 48
  - artifact: SR-14128
    scope_index: 48
- id: FR-017-AC-36
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-36 | The diagnostic record never feeds classification: two runs with the same report and
    exit and different stream text classify to the same outcome, checks, SUCCESS count and playback, and two `NoVerdict`
    runs with equal exit and no report whose streams differ have equal evidence in every field but `diagnostics`,
    `memory` (the observed peak resident bytes vary run to run) and the run''s report-path argument (the last argument,
    unique to each run); the FR-029 terminal value of a `NoVerdict` evidence carrying a record is `Failed`, the
    value FR-029-AC-4 gives the'
  paragraph_anchor: 827
  excerpt_start_line: 859
  applies_to:
  - artifact: SR-14120
    scope_index: 49
  - artifact: SR-14121
    scope_index: 49
  - artifact: SR-14122
    scope_index: 49
  - artifact: SR-14123
    scope_index: 49
  - artifact: SR-14124
    scope_index: 49
  - artifact: SR-14125
    scope_index: 49
  - artifact: SR-14126
    scope_index: 49
  - artifact: SR-14127
    scope_index: 49
  - artifact: SR-14128
    scope_index: 49
- id: FR-017-AC-37
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-37 | Over single-run launcher stand-ins that exit 1, print `Checking harness proofs::check...`
    and `CBMC failed with status 64` on stdout and `Unknown option: --no-such-option` on stderr, and export a report
    whose one result `proofs::check` is `Failure` with no checks and whose one `error_details` entry for `proofs::check`
    has the exit status shown: `exit_code_64` is `Inconclusive` `FailedWithoutCounterexample` with `failureCause`
    `{ "kind": "exit_code", "code": 64 }` and a `diagnostics` record of phase `unknown` whose stderr excerpt holds
    `Unknown option:'
  paragraph_anchor: 827
  excerpt_start_line: 860
  applies_to:
  - artifact: SR-14120
    scope_index: 50
  - artifact: SR-14121
    scope_index: 50
  - artifact: SR-14122
    scope_index: 50
  - artifact: SR-14123
    scope_index: 50
  - artifact: SR-14124
    scope_index: 50
  - artifact: SR-14125
    scope_index: 50
  - artifact: SR-14126
    scope_index: 50
  - artifact: SR-14127
    scope_index: 50
  - artifact: SR-14128
    scope_index: 50
- id: FR-017-AC-38
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-38 | A single-run stand-in that exits 1 and exports a `Failure` report listing a failed
    assertion check with no playback on the console, whose entry for the harness has exit status `properties_failed`,
    is `FailedWithoutCounterexample` with `failureCause` `{ "kind": "properties_failed" }` and no `diagnostics`
    member. The same report with no `error_details` member, with an entry that names another harness only, with
    two entries naming the harness, with an entry whose `exit_status` is null, or whose one result names no `harness_id`,
    is'
  paragraph_anchor: 827
  excerpt_start_line: 861
  applies_to:
  - artifact: SR-14120
    scope_index: 51
  - artifact: SR-14121
    scope_index: 51
  - artifact: SR-14122
    scope_index: 51
  - artifact: SR-14123
    scope_index: 51
  - artifact: SR-14124
    scope_index: 51
  - artifact: SR-14125
    scope_index: 51
  - artifact: SR-14126
    scope_index: 51
  - artifact: SR-14127
    scope_index: 51
  - artifact: SR-14128
    scope_index: 51
- id: FR-017-AC-39
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-39 | A batch stand-in of members `a::check`, `b::check`, `c::check`, `d::check` and `e::check`
    whose process exits 1 and whose report has `a::check` `Success` with a passing assertion, every cover satisfied
    and no exit status, `b::check` `Failure` with no checks and exit status `exit_code_64`, `c::check` `Failure`
    with no checks and exit status `timeout`, `d::check` `Failure` listing a failed assertion with no playback block
    headed for its path and exit status `timeout`, and `e::check` `Failure` listing a failed assertion with no playback
    block headed for its'
  paragraph_anchor: 827
  excerpt_start_line: 862
  applies_to:
  - artifact: SR-14120
    scope_index: 52
  - artifact: SR-14121
    scope_index: 52
  - artifact: SR-14122
    scope_index: 52
  - artifact: SR-14123
    scope_index: 52
  - artifact: SR-14124
    scope_index: 52
  - artifact: SR-14125
    scope_index: 52
  - artifact: SR-14126
    scope_index: 52
  - artifact: SR-14127
    scope_index: 52
  - artifact: SR-14128
    scope_index: 52
- id: FR-017-AC-40
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-40 | The cause never feeds classification: three single-run stand-ins with the same `Failure`
    report, no checks and the same exit, whose entries give `exit_code_64`, `out_of_memory` and `cbmc_exploded`
    and whose streams differ, classify to the same outcome, checks, SUCCESS count and playback, and their evidence
    is equal in every field but `failureCause`, `diagnostics`, `memory` and the report-path argument; the cause
    is the only input of the FR-029 terminal value that differs: `exit_code_64` and `cbmc_exploded` give `Failed`,
    equal to the terminal value of the'
  paragraph_anchor: 827
  excerpt_start_line: 863
  applies_to:
  - artifact: SR-14120
    scope_index: 53
  - artifact: SR-14121
    scope_index: 53
  - artifact: SR-14122
    scope_index: 53
  - artifact: SR-14123
    scope_index: 53
  - artifact: SR-14124
    scope_index: 53
  - artifact: SR-14125
    scope_index: 53
  - artifact: SR-14126
    scope_index: 53
  - artifact: SR-14127
    scope_index: 53
  - artifact: SR-14128
    scope_index: 53
- id: FR-017-AC-41
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-41 | Every `KaniScalarNativeError` variant has a `code()` that is a total `&''static str`
    from exactly this set: `invalid_request`, `artifact_package`, `snapshot_io`, `kani_execution`, `proof_not_verified`,
    `oracle_stop`, `native_launch`, `dependency_lock`, `native_result`, `snapshot_changed`, `identity_encode`, `codegen_lock`.
    The set is closed and the codes are pairwise distinct. The type is `#[non_exhaustive]` (a rustdoc `compile_fail`
    exhaustive match without a wildcard arm, beside a compiling control with one), and `class()` returns a closed'
  paragraph_anchor: 827
  excerpt_start_line: 864
  applies_to:
  - artifact: SR-14120
    scope_index: 54
  - artifact: SR-14121
    scope_index: 54
  - artifact: SR-14122
    scope_index: 54
  - artifact: SR-14123
    scope_index: 54
  - artifact: SR-14124
    scope_index: 54
  - artifact: SR-14125
    scope_index: 54
  - artifact: SR-14126
    scope_index: 54
  - artifact: SR-14127
    scope_index: 54
  - artifact: SR-14128
    scope_index: 54
- id: FR-017-AC-42
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-42 | `class()` is `CompletionRefusal` for `invalid_request`, `artifact_package` and `oracle_stop`,
    and `EnvironmentFault` for `snapshot_io`, `native_launch`, `dependency_lock`, `native_result`, `snapshot_changed`,
    `identity_encode` and `codegen_lock`. For `proof_not_verified` it is `CompletionRefusal` for evidence outcome
    `Falsified`, `CoverUnsatisfied`, and `Inconclusive` `MissingCoverSummary`, `VacuousProof`, `UnwindBoundExhausted`
    or `FailedWithoutCounterexample` with no `failureCause` or `properties_failed`, and `EnvironmentFault` for `Cancelled`,'
  paragraph_anchor: 827
  excerpt_start_line: 865
  applies_to:
  - artifact: SR-14120
    scope_index: 55
  - artifact: SR-14121
    scope_index: 55
  - artifact: SR-14122
    scope_index: 55
  - artifact: SR-14123
    scope_index: 55
  - artifact: SR-14124
    scope_index: 55
  - artifact: SR-14125
    scope_index: 55
  - artifact: SR-14126
    scope_index: 55
  - artifact: SR-14127
    scope_index: 55
  - artifact: SR-14128
    scope_index: 55
- id: FR-017-AC-43
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-43 | A canonical-identity encoding failure is `IdentityEncode` with code `identity_encode`
    and class `EnvironmentFault`; no path returns `NativeResult` for it. `NativeResult` is produced only for the
    helper''s malformed, missing or unsupported structured result, and a forced identity-encode failure through
    an in-crate test seam returns `IdentityEncode`, not `NativeResult`. | Test (TC-027) |'
  paragraph_anchor: 827
  excerpt_start_line: 866
  applies_to:
  - artifact: SR-14120
    scope_index: 56
  - artifact: SR-14121
    scope_index: 56
  - artifact: SR-14122
    scope_index: 56
  - artifact: SR-14123
    scope_index: 56
  - artifact: SR-14124
    scope_index: 56
  - artifact: SR-14125
    scope_index: 56
  - artifact: SR-14126
    scope_index: 56
  - artifact: SR-14127
    scope_index: 56
  - artifact: SR-14128
    scope_index: 56
- id: FR-017-AC-44
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-44 | The code table and the mapping to class are frozen: a test lists every variant with
    its expected code and its mapping to class (one class for a fixed variant; for `proof_not_verified` the class
    of each `KaniRunOutcome`, `KaniInconclusiveReason` and `KaniFailureCause` value; for `kani_execution` the class
    of each nested refusal), fails if a variant, outcome, reason, cause or nested refusal exists that the list lacks
    or if a code value or a mapped class differs, and fails if two variants share a code. Adding a variant, code,
    class or mapping entry is a spec'
  paragraph_anchor: 827
  excerpt_start_line: 867
  applies_to:
  - artifact: SR-14120
    scope_index: 57
  - artifact: SR-14121
    scope_index: 57
  - artifact: SR-14122
    scope_index: 57
  - artifact: SR-14123
    scope_index: 57
  - artifact: SR-14124
    scope_index: 57
  - artifact: SR-14125
    scope_index: 57
  - artifact: SR-14126
    scope_index: 57
  - artifact: SR-14127
    scope_index: 57
  - artifact: SR-14128
    scope_index: 57
- id: FR-017-AC-45
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-45 | The receipt exposes its proof-content identity as a `DigestRecord`, the `qsl_replay::DigestRecord`
    re-exported at the crate root (no CG-defined twin type; a compile test names it by the CG path and by the `qsl_replay`
    path as the same type), with domain `sha256-jcs` and 32 bytes equal to `ScalarProofContentIdentity::as_bytes()`.
    An independent computation in the test, SHA-256 of the RFC 8785 encoding of `{"schema": "agent-ix/cg-kani-scalar-proof-content/v3",
    "artifacts": [...]}` over the entries of AC-46, gives the same 32 bytes. The domain is not'
  paragraph_anchor: 827
  excerpt_start_line: 868
  applies_to:
  - artifact: SR-14120
    scope_index: 58
  - artifact: SR-14121
    scope_index: 58
  - artifact: SR-14122
    scope_index: 58
  - artifact: SR-14123
    scope_index: 58
  - artifact: SR-14124
    scope_index: 58
  - artifact: SR-14125
    scope_index: 58
  - artifact: SR-14126
    scope_index: 58
  - artifact: SR-14127
    scope_index: 58
  - artifact: SR-14128
    scope_index: 58
- id: FR-017-AC-46
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-46 | For a genuine routed `x + 1` package the v3 preimage has exactly ten artifacts, in
    UTF-8 byte order: `Cargo.lock`, `Cargo.toml`, `claim-map.json`, the only admitted record `kani-obligations/<module>.json`,
    `observer/Cargo.lock`, `observer/Cargo.toml`, `source-maps/lib.json`, `src/bin/ir791_native.rs`, `src/lib.rs`
    (harness), and `src/oracle-artifacts-lib.rs` (emitted library). All four original emitted artifacts, harness
    and record compare byte-identically under the rename map. Removing, renaming, reordering, duplicating or changing
    one byte of any entry'
  paragraph_anchor: 827
  excerpt_start_line: 869
  applies_to:
  - artifact: SR-14120
    scope_index: 59
  - artifact: SR-14121
    scope_index: 59
  - artifact: SR-14122
    scope_index: 59
  - artifact: SR-14123
    scope_index: 59
  - artifact: SR-14124
    scope_index: 59
  - artifact: SR-14125
    scope_index: 59
  - artifact: SR-14126
    scope_index: 59
  - artifact: SR-14127
    scope_index: 59
  - artifact: SR-14128
    scope_index: 59
- id: FR-017-AC-47
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-47 | Two receipts for the same package and harness with different operands, different `ScalarLimits`
    and different native outcomes have equal `DigestRecord`s; operand order, limits, ranges, outcome and Kani evidence
    appear nowhere in the preimage. This holds for the swapped-operand runs of FR-017-AC-31. | Test (TC-027) |'
  paragraph_anchor: 827
  excerpt_start_line: 870
  applies_to:
  - artifact: SR-14120
    scope_index: 60
  - artifact: SR-14121
    scope_index: 60
  - artifact: SR-14122
    scope_index: 60
  - artifact: SR-14123
    scope_index: 60
  - artifact: SR-14124
    scope_index: 60
  - artifact: SR-14125
    scope_index: 60
  - artifact: SR-14126
    scope_index: 60
  - artifact: SR-14127
    scope_index: 60
  - artifact: SR-14128
    scope_index: 60
- id: FR-017-AC-48
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-48 | The receipt exposes read-only the ordered staged `(path, bytes)` entries the digest
    was built from, including both manifests, both independently generated locks, the source map and the canonical
    `src/bin/ir791_native.rs`. All ten entries remain measured, sealed and compared across both execution boundaries.
    Recomputing the digest from only those exposed entries equals the receipt''s `DigestRecord`. A recomputation
    that substitutes one byte in the package, harness or harness record (which a driver holding different bytes
    would do) differs from the receipt''s'
  paragraph_anchor: 827
  excerpt_start_line: 871
  applies_to:
  - artifact: SR-14120
    scope_index: 61
  - artifact: SR-14121
    scope_index: 61
  - artifact: SR-14122
    scope_index: 61
  - artifact: SR-14123
    scope_index: 61
  - artifact: SR-14124
    scope_index: 61
  - artifact: SR-14125
    scope_index: 61
  - artifact: SR-14126
    scope_index: 61
  - artifact: SR-14127
    scope_index: 61
  - artifact: SR-14128
    scope_index: 61
- id: FR-017-AC-49
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-49 | A change to the preimage schema (artifact set, path names, ordering, encoding, canonicalization
    or hash) is made only by changing the schema string, with the golden digest of AC-46 updated in the same change;
    the golden is computed over fixed literal entries, so dependency bumps do not move it, and is asserted together
    with the schema string in one assertion; a golden change without a schema bump is rejected in review (a test
    cannot enforce it). A change to either manifest, either lock or adapter bytes changes the digest with the schema
    string unchanged.'
  paragraph_anchor: 827
  excerpt_start_line: 872
  applies_to:
  - artifact: SR-14120
    scope_index: 62
  - artifact: SR-14121
    scope_index: 62
  - artifact: SR-14122
    scope_index: 62
  - artifact: SR-14123
    scope_index: 62
  - artifact: SR-14124
    scope_index: 62
  - artifact: SR-14125
    scope_index: 62
  - artifact: SR-14126
    scope_index: 62
  - artifact: SR-14127
    scope_index: 62
  - artifact: SR-14128
    scope_index: 62
- id: FR-017-AC-50
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-50 | The `NativeMeterObservation` of a receipt exposes read-only the `ScalarLimits` the
    native meter was constructed with. They are read back from the native invocation''s own output, not copied from
    the request: a test seam that makes the adapter report a well-formed but different limit value makes the helper''s
    structured result unsupported in FR-017-AC-43''s sense and yields `NativeResult` / `native_result` / `EnvironmentFault`
    and no receipt. For an accepted request the request limits, the meter-observed limits and `scalar_limits()`
    are equal, and the'
  paragraph_anchor: 827
  excerpt_start_line: 873
  applies_to:
  - artifact: SR-14120
    scope_index: 63
  - artifact: SR-14121
    scope_index: 63
  - artifact: SR-14122
    scope_index: 63
  - artifact: SR-14123
    scope_index: 63
  - artifact: SR-14124
    scope_index: 63
  - artifact: SR-14125
    scope_index: 63
  - artifact: SR-14126
    scope_index: 63
  - artifact: SR-14127
    scope_index: 63
  - artifact: SR-14128
    scope_index: 63
- id: FR-017-AC-51
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-51 | A consumer compile test names and constructs `quire_contract_codegen::ScalarLimits`
    using only codegen crate paths, a typed accessor to `KaniScalarNativeRequest::scalar_limits` compiles with that
    same return type, and assignment to the owning `quire_exact::ScalarLimits` compiles without conversion, wrapper
    or compatibility shim. The CG public name and field shape remain unchanged. Planned with IR-17 kernel adoption.
    | Test |'
  paragraph_anchor: 827
  excerpt_start_line: 874
  applies_to:
  - artifact: SR-14120
    scope_index: 64
  - artifact: SR-14121
    scope_index: 64
  - artifact: SR-14122
    scope_index: 64
  - artifact: SR-14123
    scope_index: 64
  - artifact: SR-14124
    scope_index: 64
  - artifact: SR-14125
    scope_index: 64
  - artifact: SR-14126
    scope_index: 64
  - artifact: SR-14127
    scope_index: 64
  - artifact: SR-14128
    scope_index: 64
- id: FR-017-AC-52
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-52 | Production emitted bytes and feature-free exact dependency remain unchanged. The isolated
    private observer uses resolver 2 and its sole example compiles the single canonical adapter in place through
    `../src/bin/ir791_native.rs`, with runtime `exact` and kernel `test-support` exposed only by dev-dependencies;
    no wrapper, copied source, symlink, normal/build logging edge or CLI feature override is allowed. Native invocation
    explicitly names `observer/Cargo.toml` and the example; the real production proof retains its harness, ordered
    options, claims, covers,'
  paragraph_anchor: 827
  excerpt_start_line: 875
  applies_to:
  - artifact: SR-14120
    scope_index: 65
  - artifact: SR-14121
    scope_index: 65
  - artifact: SR-14122
    scope_index: 65
  - artifact: SR-14123
    scope_index: 65
  - artifact: SR-14124
    scope_index: 65
  - artifact: SR-14125
    scope_index: 65
  - artifact: SR-14126
    scope_index: 65
  - artifact: SR-14127
    scope_index: 65
  - artifact: SR-14128
    scope_index: 65
- id: FR-017-AC-53
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  excerpt: '| FR-017-AC-53 | Both package locks are independently freshly resolved with the existing bounded offline
    generation/update workflow, without copying a parent/other lock or substituting a cached resolution. Each resolves
    both owning runtime and exact identities to the build''s authoritative lock identities; the independent observer
    regression asserts both literal package names and repository identities independently of the renderer''s list.
    Both manifests, locks and all other AC-46 entries are sealed and compared as a complete path/byte set before
    proof, after proof and after'
  paragraph_anchor: 827
  excerpt_start_line: 876
  applies_to:
  - artifact: SR-14120
    scope_index: 66
  - artifact: SR-14121
    scope_index: 66
  - artifact: SR-14122
    scope_index: 66
  - artifact: SR-14123
    scope_index: 66
  - artifact: SR-14124
    scope_index: 66
  - artifact: SR-14125
    scope_index: 66
  - artifact: SR-14126
    scope_index: 66
  - artifact: SR-14127
    scope_index: 66
  - artifact: SR-14128
    scope_index: 66
- id: TC-027:L10
  path: spec/kani/matrix/TC-027-kani-execution-evidence.md
  excerpt: '## Description'
  paragraph_anchor: 10
  excerpt_start_line: 11
  applies_to:
  - artifact: SR-14120
    scope_index: 69
  - artifact: SR-14121
    scope_index: 69
  - artifact: SR-14122
    scope_index: 69
  - artifact: SR-14123
    scope_index: 69
  - artifact: SR-14124
    scope_index: 69
  - artifact: SR-14125
    scope_index: 69
  - artifact: SR-14126
    scope_index: 69
  - artifact: SR-14127
    scope_index: 69
  - artifact: SR-14128
    scope_index: 69
```

## Source-location correction 2

This append-only metadata correction supersedes only the 16 `excerpt_start_line` values below from the first source-location clarification. Repeated signatures must resolve to the intended original source unit, not the first global occurrence. Every original artifact prefix, scope record, finding, verdict, binding and private comment remains unchanged. These mappings apply only to the explicitly listed SR-14121 scope indices; the other methods have no affected source units. No specification finding or semantic verdict changes.

```yaml
source_location_correction_2:
- id: src/kani/generate/spec.rs:L264
  path: src/kani/generate/spec.rs
  excerpt: '    pub(super) fn new('
  context_excerpt: "}\nimpl HarnessSpec {\n    pub(super) fn new(\n        label: String,\n        path: HarnessPath,\n\
    \        oracle_source: String,"
  superseded_excerpt_start_line: 49
  excerpt_start_line: 264
  applies_to:
  - artifact: SR-14121
    scope_index: 266
  basis: Inspected the intended source unit at its original anchor; repeated signatures elsewhere are different
    units.
- id: src/kani/identity.rs:L379
  path: src/kani/identity.rs
  excerpt: '    pub(crate) fn generated('
  context_excerpt: "impl KaniScalarObligationHarness {\n    /// Construct the immutable generator-origin seal for\
    \ one scalar harness.\n    pub(crate) fn generated(\n        identity: ScalarObligationIdentity,\n        rust:\
    \ Artifact,\n        record: Artifact,"
  superseded_excerpt_start_line: 322
  excerpt_start_line: 379
  applies_to:
  - artifact: SR-14121
    scope_index: 281
  basis: Inspected the intended source unit at its original anchor; repeated signatures elsewhere are different
    units.
- id: src/kani/identity.rs:L397
  path: src/kani/identity.rs
  excerpt: '    pub(crate) fn has_valid_provenance(&self) -> bool {'
  context_excerpt: "    /// Whether the public retained fields still match the generator-origin seal.\n    #[must_use]\n\
    \    pub(crate) fn has_valid_provenance(&self) -> bool {\n        self.identity == self.generated_identity\n\
    \            && self.rust == self.generated_rust\n            && self.record == self.generated_record"
  superseded_excerpt_start_line: 339
  excerpt_start_line: 397
  applies_to:
  - artifact: SR-14121
    scope_index: 282
  basis: Inspected the intended source unit at its original anchor; repeated signatures elsewhere are different
    units.
- id: src/kani/identity.rs:L436
  path: src/kani/identity.rs
  excerpt: '    pub fn harness_path(&self) -> HarnessPath {'
  context_excerpt: "    /// The `module::harness` path of this harness.\n    #[must_use]\n    pub fn harness_path(&self)\
    \ -> HarnessPath {\n        HarnessPath {\n            module: self.module_symbol.clone(),\n            harness:\
    \ self.harness_symbol.clone(),"
  superseded_excerpt_start_line: 425
  excerpt_start_line: 436
  applies_to:
  - artifact: SR-14121
    scope_index: 286
  basis: Inspected the intended source unit at its original anchor; repeated signatures elsewhere are different
    units.
- id: src/kani/identity.rs:L631
  path: src/kani/identity.rs
  excerpt: '    pub fn harness_path(&self) -> HarnessPath {'
  context_excerpt: "    /// The `module::harness` path of this harness.\n    #[must_use]\n    pub fn harness_path(&self)\
    \ -> HarnessPath {\n        HarnessPath {\n            module: self.module_symbol.clone(),\n            harness:\
    \ self.harness_symbol.clone(),"
  superseded_excerpt_start_line: 425
  excerpt_start_line: 631
  applies_to:
  - artifact: SR-14121
    scope_index: 287
  basis: Inspected the intended source unit at its original anchor; repeated signatures elsewhere are different
    units.
- id: src/kani/run/execute.rs:L1159
  path: src/kani/run/execute.rs
  excerpt: '    pub(super) fn single('
  context_excerpt: "\n    #[cfg(not(target_os = \"linux\"))]\n    pub(super) fn single(\n        request: &KaniExecutionRequest<'_>,\n\
    \    ) -> Result<FixtureExecution, KaniExecutionRefusal> {\n        require_in_crate(request)?;"
  superseded_excerpt_start_line: 1137
  excerpt_start_line: 1159
  applies_to:
  - artifact: SR-14121
    scope_index: 321
  basis: Inspected the intended source unit at its original anchor; repeated signatures elsewhere are different
    units.
- id: src/kani/run/execute.rs:L1181
  path: src/kani/run/execute.rs
  excerpt: '    pub(super) fn groups('
  context_excerpt: "\n    #[cfg(not(target_os = \"linux\"))]\n    pub(super) fn groups(\n        requests: &[KaniExecutionRequest<'_>],\n\
    \    ) -> Result<Vec<ReportGroup>, KaniExecutionRefusal> {\n        for request in requests {"
  superseded_excerpt_start_line: 1144
  excerpt_start_line: 1181
  applies_to:
  - artifact: SR-14121
    scope_index: 322
  basis: Inspected the intended source unit at its original anchor; repeated signatures elsewhere are different
    units.
- id: src/kani/run/namespace.rs:L276
  path: src/kani/run/namespace.rs
  excerpt: '    pub(super) fn dispatch('
  context_excerpt: "    /// Unsupported targets cannot dispatch a PID-namespace backend.\n    #[cfg(not(target_os\
    \ = \"linux\"))]\n    pub(super) fn dispatch(\n        &mut self,\n        _: Option<Instant>,\n        _: &mut\
    \ super::memory::MemoryObserver,"
  superseded_excerpt_start_line: 212
  excerpt_start_line: 276
  applies_to:
  - artifact: SR-14121
    scope_index: 354
  basis: Inspected the intended source unit at its original anchor; repeated signatures elsewhere are different
    units.
- id: src/oracle/bound_v1.rs:L75
  path: src/oracle/bound_v1.rs
  excerpt: '    pub fn informational(&self) -> &[ClauseRef] {'
  context_excerpt: "    /// Complete informational population, never counted as executable coverage.\n    #[must_use]\n\
    \    pub fn informational(&self) -> &[ClauseRef] {\n        &self.informational\n    }\n    /// Validated complete\
    \ set; publication requires a separate explicit call."
  superseded_excerpt_start_line: 34
  excerpt_start_line: 75
  applies_to:
  - artifact: SR-14121
    scope_index: 416
  basis: Inspected the intended source unit at its original anchor; repeated signatures elsewhere are different
    units.
- id: src/oracle/equality/mod.rs:L3615
  path: src/oracle/equality/mod.rs
  excerpt: '        assert_eq!(lib.matches("pub fn environment_").count(), 1);'
  context_excerpt: "            Some((\"environment_equality_equal\", \"oracle_equality_equal\"))\n        );\n\
    \        assert_eq!(lib.matches(\"pub fn environment_\").count(), 1);\n\n        // Neither renders: both are\
    \ refused, with no code.\n        let (result, claims, lib) = emit_two([true, true], quantity);"
  superseded_excerpt_start_line: 3605
  excerpt_start_line: 3615
  applies_to:
  - artifact: SR-14121
    scope_index: 437
  basis: Inspected the intended source unit at its original anchor; repeated signatures elsewhere are different
    units.
- id: src/replay/composite.rs:L67
  path: src/replay/composite.rs
  excerpt: '    pub fn terminal_value(&self) -> TerminalValue {'
  context_excerpt: "\n    /// The value QSL assigned to this result.\n    pub fn terminal_value(&self) -> TerminalValue\
    \ {\n        self.result.terminal_value()\n    }\n}"
  superseded_excerpt_start_line: 48
  excerpt_start_line: 67
  applies_to:
  - artifact: SR-14121
    scope_index: 496
  basis: Inspected the intended source unit at its original anchor; repeated signatures elsewhere are different
    units.
- id: src/replay/function.rs:L1024
  path: src/replay/function.rs
  excerpt: pub fn replay_counterexample(
  context_excerpt: "/// [`SpineReplayError`] when the adapter cannot build an admitted request or QSL refuses it;\n\
    /// those are not verdicts on the evidence.\npub fn replay_counterexample(\n    identity: &KaniObligationIdentity,\n\
    \    transcript: &str,\n    package: &ReplayPackage,"
  superseded_excerpt_start_line: 726
  excerpt_start_line: 1024
  applies_to:
  - artifact: SR-14121
    scope_index: 532
  basis: Inspected the intended source unit at its original anchor; repeated signatures elsewhere are different
    units.
- id: src/routed/contract_package_tests.rs:L2470
  path: src/routed/contract_package_tests.rs
  excerpt: '            .contains("pub fn oracle_"));'
  context_excerpt: "            .expect(\"generated source\")\n            .contents\n            .contains(\"pub\
    \ fn oracle_\"));\n    }\n\n    #[trace(\"FR-039-AC-2\", \"FR-039-AC-7\", \"TC-053\")]"
  superseded_excerpt_start_line: 2351
  excerpt_start_line: 2470
  applies_to:
  - artifact: SR-14121
    scope_index: 572
  basis: Inspected the intended source unit at its original anchor; repeated signatures elsewhere are different
    units.
- id: src/strategy/campaign.rs:L391
  path: src/strategy/campaign.rs
  excerpt: '    pub const fn expects_rejection(self) -> bool {{\n\'
  context_excerpt: "    /// Returns false because emitted enum populations contain admissible members only.\\n\\\
    \n    #[must_use]\\n\\\n    pub const fn expects_rejection(self) -> bool {{\\n\\\n        false\\n\\\n    }}\\\
    n\\\n\\n\\"
  superseded_excerpt_start_line: 227
  excerpt_start_line: 391
  applies_to:
  - artifact: SR-14121
    scope_index: 630
  basis: Inspected the intended source unit at its original anchor; repeated signatures elsewhere are different
    units.
- id: src/strategy/campaign.rs:L396
  path: src/strategy/campaign.rs
  excerpt: '    pub fn verify(\n\'
  context_excerpt: "\\n\\\n    /// Checks this generated domain expectation against a tri-state harness verdict.\\\
    n\\\n    pub fn verify(\\n\\\n        self,\\n\\\n        verdict: quire_contract_runtime::VerdictKind,\\n\\\
    \n    ) -> proptest::test_runner::TestCaseResult {{\\n\\"
  superseded_excerpt_start_line: 232
  excerpt_start_line: 396
  applies_to:
  - artifact: SR-14121
    scope_index: 631
  basis: Inspected the intended source unit at its original anchor; repeated signatures elsewhere are different
    units.
- id: src/strategy/campaign.rs:L427
  path: src/strategy/campaign.rs
  excerpt: pub fn {function}() -> proptest::strategy::BoxedStrategy<{case_type}> {{\n\
  context_excerpt: "\\n\\\n/// Builds generated enum strategy `{}`.\\n\\\npub fn {function}() -> proptest::strategy::BoxedStrategy<{case_type}>\
    \ {{\\n\\\n    use proptest::strategy::Strategy as _;\\n\\\n{body}\\\n}}\\n\","
  superseded_excerpt_start_line: 266
  excerpt_start_line: 427
  applies_to:
  - artifact: SR-14121
    scope_index: 632
  basis: Inspected the intended source unit at its original anchor; repeated signatures elsewhere are different
    units.
```
