---
id: SR-202610310162
title: IR-1047 PR701 ears-conformance scoped correction review
type: SpecReview
analysis: ears-conformance
scope: agent-ix/quire-contract-codegen; IR-1047; spec/kani/functional/FR-041-claimed-module-proof-gate.md,
  spec/kani/matrix/TC-056-claimed-module-proof-gate.md, spec/kani/non-functional/NFR-006-real-kani-lane-gating.md;
  exact proposed IR-1047 and IR-1049 descriptions
review_set: subset
---

## Summary

Ticket: IR-1047. Head a5814c1e8dbb0bc4efac3ab472cf54fb8a5307e3 against base 3da9460cc97a61c2de2cd4ae2ad3f115a781e5c2. Engine quire validation reports no grammar warning on these changed files. Changed SHALL statements name concrete generator/module/gate outcomes; sequencing prose is verifiable against the two deliveries. No grammar defect found in the scoped correction.

## Verdict

PASS — Engine quire validation reports no grammar warning on these changed files. Changed SHALL statements name concrete generator/module/gate outcomes; sequencing prose is verifiable against the two deliveries. No grammar defect found in the scoped correction.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Examined scope

```yaml
scope:
- id: FR-041-AC-1
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: A nonempty set of current-run generated module claims is checked against the current compiled-module census. An absent compiled module or empty set names the claim or condition and fails. Open pending a genuine current-build claim source. The former merge-base declaration comparison is retired by IR-1105.
- id: FR-041-AC-2
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: With two claimed modules sharing one `src/lib.rs`, a SUCCESS location inside only the first and an UNREACHABLE check inside the second reports the second `unreached` and fails. A SUCCESS check inside each passes this census; a SATISFIED cover, unknown location or non-SUCCESS status cannot supply the missing check.
- id: FR-041-AC-3
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: The current build's transitive expectation/subject reachability yields the shared-helper intersection; the gate fails when the checked-in generated evidence omits or adds a helper, and records an explicit empty set when none is shared. For an exact operation, the expectation uses QSpec vectors or a model with no `quire-exact` call.
- id: FR-041-AC-4
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: Each claimed module and computed shared helper has a positive unmodified real-Kani proof and a source mutation that compiles, executes, and reports a failed proposition check. Deleting a control or making its mutant verify, fail to compile, time out, skip execution, or merely trip the dirty-tree guard fails the gate; evidence names the exact diff, command and failed check from this run.
- id: FR-041-AC-5
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: A stubbed executor, predetermined outcome or build-only run produces no qualifying module evidence and cannot pass the gate.
- id: FR-041-AC-6
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: The V2 claim is produced by the admitted V2 Boolean implication request through the production scalar renderer; its generated module and harness identities match the current-build claim source and compiled census. A V1 source, hand-authored module, missing module, changed identity or uncompiled claim is rejected by name. Open pending a genuine current-build claim source.
- id: FR-041-AC-7
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: The V2 module exposes separate named subject and expectation roots in the current compiled call graph. The subject reaches the runtime implication operation; the expectation evaluates `!left || right` over the same Boolean inputs and reaches neither subject, oracle nor `quire-exact`. The observed helper intersection exactly matches the declared helper mutations, including an explicit empty set.
- id: FR-041-AC-8
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: The unmodified generated V2 module has a location-attributed SUCCESS proposition and satisfied final cover with installed Kani. Replacing only its parsed subject-call expression with `false` compiles and executes the same harness, then yields an in-module FAILURE at the subject-versus-expectation assertion with in-domain playback where `left` is false; each computed shared helper has its own positive and falsifying control.
- id: FR-041-AC-9
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: After AC-6 through AC-8 pass in the current lane, the V2 implication module is included in the current-build claim census. The required `make kani-gate` run executes its real-Kani control and fails on any missing, skipped or failing claim or helper control. This completes the IR-1047 machinery delivery independently of the new Boolean state-transition claim in AC-10; implication proof credit does not discharge that transition obligation. Open pending a genuine current-build claim source.
- id: FR-041-AC-10
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: A production V2 module implements the Boolean state-transition obligation originally described by FR-003@4, with precondition `input_current \|\| state_pre`, postcondition `state_post`, the actual transition subject, authentic checked-package/source provenance and a separately named independent expectation. The new claim has its own stable V2 module and obligation identities and is registered in the current compiled census without restoring a V1 row or asserting V1 identity continuity. Installed Kani gives that module an in-module SUCCESS proposition and satisfied cover; its in-module source m
- id: FR-041 / V1 claim retirement
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: 'The V1-sourced FR-003@4 claim ended with the V1 generator deletion and retirement

    of its inventory and hold in [CG #636](https://github.com/agent-ix/quire-contract-codegen/pull/636)

    (IR-1105), as confirmed by the

    [owner ruling on IR-1049](https://linear.app/agent-ix/issue/IR-1049#comment-813eda1b).

    No V1 module identity, obligation identity or proof credit carries into a V2 claim.

    The Boolean state-transition obligation is implemented by a new V2 claim under

    AC-10. All current V2 claims remain subject to AC-1 through AC-5: an empty claim

    set, absent compiled module, missing independent expecta'
- id: FR-041 / Current-build V2 claim machinery (IR-1047, planned)
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: "- When an additional V2 claim is declared, the generator SHALL obtain an admitted V2 Boolean\n  implication scalar claim from the production `KaniObligationRequest` path and render its\n  module through `src/kani/generate/scalar.rs::render_boolean_implies`. A future\n  current-build claim control SHALL assemble the generated Rust artifact, its required\n  runtime dependency and its separate scratch-crate subject source; it SHALL\n  derive the declared module and harness identities from the generated identity record,\n  never from a copied V1 source or a hand-authored module.\n- The generated module S"
- id: FR-041 / New V2 Boolean state-transition claim (IR-1049, planned)
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: 'After the IR-1047 machinery is available, a production V2 module SHALL implement

    the Boolean state-transition obligation originally described by FR-003@4:

    precondition `input_current || state_pre`, postcondition `state_post`, and the

    actual transition subject. It SHALL have its own stable V2 module and obligation

    identities, authentic checked-package/source provenance, and a separately named

    independent expectation rooted in the current compiled call graph. It SHALL be

    registered as a new claim in the current-build census, without restoring a V1 row,

    reusing a V1 identity or copying V1 generat'
- id: FR-041 / Status
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: 'Open. IR-1047 delivers the genuine current-build claim source, compiled-module

    census, subject/expectation helper intersection and per-helper controls, using the

    V2 implication claim. The new V2 Boolean state-transition claim follows in

    IR-1049. Neither delivery receives proof credit from this specification correction;

    current-run installed-Kani positive and falsifying evidence remains required.'
- id: TC-056 / Test Procedure
  path: spec/kani/matrix/TC-056-claimed-module-proof-gate.md
  role: examined
  excerpt: "The [retirement record](../functional/FR-041-claimed-module-proof-gate.md#v1-claim-retirement)\nprovides no V2 proof credit. These procedures remain Open until production generated\nmodules provide a genuine current-build claim source and installed-Kani controls.\nIR-1047's V2 implication and IR-1049's new V2 Boolean state-transition claim are\nseparate obligations, each with its own generated identity.\n\n1. Obtain a nonempty set of claims from the current production generated artifact.\n   Build the candidate and compare claimed identities with compiled modules. In isolated\n   negative controls, om"
- id: TC-056 / Expected Results
  path: spec/kani/matrix/TC-056-claimed-module-proof-gate.md
  role: examined
  excerpt: 'The required real-Kani lane passes only when every current-build claimed module

    has an in-module SUCCESS proposition, the build-derived shared-helper set agrees

    with the generated evidence, and each claimed module and shared helper has a

    compiling, executed mutation with a real failed check. No build-only result,

    stub, process refusal or stored verdict counts. A current V2 claim cannot be

    removed or left uncompiled to satisfy these checks.'
- id: TC-056 / Status
  path: spec/kani/matrix/TC-056-claimed-module-proof-gate.md
  role: examined
  excerpt: 'Open. A genuine current-build claim source and installed-Kani proof and mutation

    controls are still needed for IR-1047. IR-1049''s new V2 transition claim follows

    that prerequisite and is not a condition for landing the machinery.'
- id: NFR-006-AC-19
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: On a clean current candidate, `make kani-gate` runs FR-041's current-build module census and real mutation controls in its one required lane, and a failure of any claim or shared-helper control makes the gate exit non-zero. An optional mutation target, retired fixed inventory or saved earlier result cannot substitute. Open until a genuine current-build claim source exists.
- id: IR-1047 proposal paragraph 1
  path: /tmp/titan-ir1047-reviewed-ticket-proposals.json
  role: examined
  excerpt: Filed by luna:IR:plan
- id: IR-1047 proposal paragraph 2
  path: /tmp/titan-ir1047-reviewed-ticket-proposals.json
  role: examined
  excerpt: '## Need'
- id: IR-1047 proposal paragraph 3
  path: /tmp/titan-ir1047-reviewed-ticket-proposals.json
  role: examined
  excerpt: 'Deliver the current-build generated V2 claim source, compiled-module census,

    subject/expectation helper intersection, and per-helper controls required by

    FR-041 and NFR-006. The first client is the production V2 Boolean implication

    claim specified by FR-041-AC-6..9; its expectation remains independent of the

    subject, oracle and quire-exact.'
- id: IR-1047 proposal paragraph 4
  path: /tmp/titan-ir1047-reviewed-ticket-proposals.json
  role: examined
  excerpt: 'The V1 retirement and the distinct new V2 transition obligation are governed by

    FR-041''s reviewed retirement record and the ruling on IR-1049 (813eda1b).

    IR-1047 replaces no V1 row and is a prerequisite of IR-1049, not the reverse.'
- id: IR-1047 proposal paragraph 5
  path: /tmp/titan-ir1047-reviewed-ticket-proposals.json
  role: examined
  excerpt: '## Acceptance'
- id: IR-1047 proposal paragraph 6
  path: /tmp/titan-ir1047-reviewed-ticket-proposals.json
  role: examined
  excerpt: '1. Spec first: preserve FR-041-AC-1 through FR-041-AC-5 and NFR-006-AC-19 verbatim. Specify a V2 generated harness with an independent expectation suitable for current-build subject/expectation reachability and the module census. State its exact owner and verification method.

    2. Generate the V2 module through the production path. Its declared claim appears in the current compiled module census; the expectation is independent of the subject and of `quire-exact` where FR-041 requires that. An absent, removed or unreachable claim fails with a named cause.

    3. Run the installed real Kani backend on'
- id: IR-1047 proposal paragraph 7
  path: /tmp/titan-ir1047-reviewed-ticket-proposals.json
  role: examined
  excerpt: '## Sequencing'
- id: IR-1047 proposal paragraph 8
  path: /tmp/titan-ir1047-reviewed-ticket-proposals.json
  role: examined
  excerpt: 'IR-1047''s machinery is delivered first. IR-1049 subsequently produces the Boolean

    state-transition obligation as a new V2 claim with its own stable identities and

    registers it through that machinery. Current V2 claims retain all FR-041-AC-1..5

    and NFR-006-AC-19 requirements verbatim; no absent-compiled or removed-claim

    exception is introduced.

    '
- id: IR-1049 proposal paragraph 1
  path: /tmp/titan-ir1047-reviewed-ticket-proposals.json
  role: examined
  excerpt: Filed by luna:IR:plan
- id: IR-1049 proposal paragraph 2
  path: /tmp/titan-ir1047-reviewed-ticket-proposals.json
  role: examined
  excerpt: '## Need'
- id: IR-1049 proposal paragraph 3
  path: /tmp/titan-ir1047-reviewed-ticket-proposals.json
  role: examined
  excerpt: 'After IR-1047''s current-build claim machinery lands, produce the Boolean

    state-transition obligation originally described by FR-003@4 as a new V2 claim.

    It has its own stable V2 module and obligation identities and authentic current

    checked-package/source provenance. The V2 FR-014 implication proof is a different

    obligation and does not discharge this transition.'
- id: IR-1049 proposal paragraph 4
  path: /tmp/titan-ir1047-reviewed-ticket-proposals.json
  role: examined
  excerpt: 'FR-041''s reviewed retirement record and the ruling on this ticket (813eda1b)

    govern the ended V1 claim; no V1 identity continuity or held-row replacement is

    claimed by this delivery.'
- id: IR-1049 proposal paragraph 5
  path: /tmp/titan-ir1047-reviewed-ticket-proposals.json
  role: examined
  excerpt: '## Acceptance'
- id: IR-1049 proposal paragraph 6
  path: /tmp/titan-ir1047-reviewed-ticket-proposals.json
  role: examined
  excerpt: '1. Specify and implement a production V2 generated harness for the original FR-003@4 Boolean state-transition obligation: precondition `input_current || state_pre`, postcondition `state_post`, and the actual transition subject. Give this new V2 claim its own stable module and obligation identities and preserve its authentic checked-package/source provenance, without asserting V1 identity continuity or copying the V1 generator or source.

    2. Provide a separately named independent expectation rooted in the current compiled call graph. It must not call the subject, its oracle, or quire-exact where'
- id: IR-1049 proposal paragraph 7
  path: /tmp/titan-ir1047-reviewed-ticket-proposals.json
  role: examined
  excerpt: '## Scope and dependency'
- id: IR-1049 proposal paragraph 8
  path: /tmp/titan-ir1047-reviewed-ticket-proposals.json
  role: examined
  excerpt: 'The current V2 scalar implication renderer proves a different FR-014 operator. Current V2 state/frame generation is integer-field scoped and does not supply this Boolean transition. This ticket blocks IR-1047''s Acceptance 4 and V1 hold expiry; IR-1047''s additive V2 implication claim can land separately while this remains open. No compatibility layer, vendoring, or invented identity continuity.

    ## Sequencing'
- id: IR-1049 proposal paragraph 9
  path: /tmp/titan-ir1047-reviewed-ticket-proposals.json
  role: examined
  excerpt: 'Blocked by IR-1047''s current-build claim source, census, helper intersection and controls. IR-1047''s machinery delivery does not depend on this ticket.

    '
- id: IR-1049 proposed Scope and dependency
  path: /tmp/titan-ir1047-reviewed-ticket-proposals.json
  role: examined
  excerpt: This ticket blocks IR-1047's Acceptance 4 and V1 hold expiry; IR-1047's additive V2 implication claim can land separately while this remains open.
```

## Method and limitations

No applicable AssuranceProfile exists under spec/. Direct subset review per scoped reviewer dispatch. Full spec Quire validation passed with existing unrelated advisory warnings. Protected criteria compared byte-for-byte to base and qualification JSON checked. No implementation or proof completion is claimed. Rust, Cargo/Kani and production gap-analysis are inapplicable to this docs-only correction; future source PRs still require full gates and actual small-bound installed Kani, never the 16 GiB proof. Criterion-strength/risk/object lenses not selected: no changed domain object, new risk model or requested calibration. Matrix is diagnostic only; planned machinery remains Open.
