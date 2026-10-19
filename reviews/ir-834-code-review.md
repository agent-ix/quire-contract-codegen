---
id: SR-12970
title: IR-834 code-review of PR447 fixture migration
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; src/kani/run/scalar.rs, src/oracle/equality/mod.rs, src/oracle/function/schedule.rs,
  src/publication/publish.rs, tests/it/kani_batching.rs, tests/it/kani_generation_claims.rs, tests/it/kani_obligations.rs,
  tests/it/kani_obligations_symbolic_domains.rs, tests/it/kani_witness_join.rs, tests/it/skeleton_spine.rs, tests/it/scratch_crate.rs,
  src/kani/test_support.rs, tests/it/layout.rs
review_set: subset
---

## Summary

Ticket: IR-834. Independent code-review with the Rust lane examined the exact ten-file fixture migration on PR447, clean frozen `8b842a5020a3246effef19612f3e2eaf9af4d54e` over assigned base `ca21b99461b5075c4a805deca740683954830b04`. One new architectural defect blocks the candidate. This artifact is reviewer-authored outside the frozen branch; no source/spec/test changes, compiler/backend invocation, commit, push or public findings were made by this reviewer.

## Verdict

**FAIL** — FND-001 remains open. The migration preserves owner lifetimes and real oracle/assertion behavior, but creates three forbidden test-module dependencies. A passing format/lint/compile/Kani result cannot establish AD-004 L-2 conformance: the full layout test was not part of those checks. Root final aggregate and full actual Kani remain mandatory after fixes, independent review and latest artifact custody (NFR-006-AC-13/15); no waiver is inferred.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Cross-subsystem unit tests introduce Kani back-edges forbidden by AD-004 L-2. | src/oracle/equality/mod.rs:2873; src/oracle/function/schedule.rs:1279; src/publication/publish.rs:273; spec/assurance/AD-004-cg-crate-layout.md:839; tests/it/layout.rs:524 |

Scenario: the next full `layout::l_2_every_import_follows_the_dependency_direction` inspection discovers `oracle -> kani` twice and `publication -> kani` once. `may_import` permits only `core` for both directories. `edges` scans inline `crate::` calls after stripping strings/comments and retains cfg(test) source. The three new calls to `kani::test_support::discover_scratch` therefore violate the existing architecture gate. AD-004 explicitly applies the same direction to cfg(test), and author compile-only checks never execute this test. This is statically determined, not a claimed observed test failure. Kani's own `run/scalar -> test_support` use is valid.

Required fix: reuse one shared target/dead-PID allocator from a directionally valid cfg(test) location, preserve existing Kani callers and behavior, and introduce neither duplication nor a scanner exception. Existing helper searches covered `discover_scratch`, `TemporaryDirectory`, module declarations, `src/core`, `src/kani/test_support.rs`, `tests/it/scratch_crate.rs` and the complete layout gate. Recopying the allocator to each consumer would create a second implementation.

## Coverage

Methods run: code-review and its Rust lane, with focused intent/oracle inspection explicitly authorized by the dispatch. No spec/plan/normative artifact changed, so spec-review and its requirement-writing subanalyses do not apply. No React/Python lane applies. Separate whole-repository planless gap-analysis is SR-12971. No applicable AssuranceProfile was found under spec; ordinary rubric used. AD-004 is architecture context, not an AssuranceProfile.

The full diff contains exactly the ten source files listed in the machine record. No dependencies, manifests/templates, CI lanes, budgets, schema, staging, hashes/catalogs or production runtime paths change. Duplication/vendoring search confirms the new uses replace local allocators with existing owners; no new implementation or copied upstream artifact is introduced. No unsafe, new panic/error surface, casts, async/lock-across-await, unbounded retry, test logic bypass, suppression or weakened gate is introduced. Production PrivateSnapshot remains excluded and still allocates under /tmp; this is not a claim that all production /tmp use disappears.

| Fixture/call chain examined | Lifetime/layout/oracle conclusion |
| --- | --- |
| obligations write_crate, run, missing/nonexecutable launchers, no-verdict, compile failure, CBMC cause, timeout and missing-harness controls | Returns TemporaryDirectory, every caller holds owner and borrows path. Real execution is synchronous. Explicit `drop(scratch)` at 2572 occurs after timeout assertions and before reacquiring the exact same prefix at 2576. Loop iteration owners drop before the next iteration. Existing generated source/build.rs/workspace manifest/runtime dependency/lock seeding unchanged. |
| obligations write_scalar_crate, run_scalar_under_real_kani and missing scalar wrapper | Owner returned and held through execution; scalar name-specific prefixes retain existing distinct healthy/violating/missing/arithmetic populations. Emitted manifest already terminates with [workspace]. Real proof/native/digest/preflight assertions unchanged. |
| symbolic domains write_collection_scratch -> run_collection_kani | Collection-specific owner returned and kept through execution/evidence-file writing; manifest/workspace and lock unchanged. Real healthy claim, mutated claim and independent playback checks remain. |
| skeleton_spine prove -> Boolean spine | Borrowed directory stays owned through real proof; each prove call returns before next allocation. Healthy proof, mutated ensures, defective subject, independently decoded playback and native QSL replay remain real. |
| witness published schema plus run_falsifying crate | Separate live prefixes for publication and crate; published owner outlives persisted-record read and both dropped/retyped binding refusals. Real transcript positive-credit arithmetic and positional names remain asserted. |
| batching Lane before-N/after-N, mixed and maximum | Lane stores TemporaryDirectory, retains crate/counts/launcher paths, unchanged nested crate layout/permissions/real counting launcher. All simultaneously live lanes have different prefixes, including before/after at 1/10/50 and mixed/maximum. Removed manual Drop is replaced by owner Drop. No async work outlives Lane. |
| generation claims Git base-read fixture | Owner survives Git commits, absent/present declaration and deleted-loose-blob fail-closed assertions. Git-only fixture needs no Cargo workspace table. Only owned wrapper changes legacy trace to canonical imported attribute. |
| unit equality/schedule | discover_scratch preserves target/dead-PID placement and explicit cleanup. Equality still creates src/tests, seeds lock, writes emitted isolated manifest and invokes actual nested Cargo; schedule still writes complete module set and invokes actual rustc on 512 KiB worker. Default/exact/one-below budgets and refusal assertions remain. These uses have the FND-001 location defect. |
| publication/snapshot controls | Existing ten publication tests keep rollback, residue, symlink and bounds assertions and explicit cleanup; staged-byte control keeps healthy, changed-byte and unexpected-file distinctions. Publication has FND-001; scalar Kani-to-support use does not. |
| shared allocators | TemporaryDirectory creates src, acquires sibling lock before reclaim, holds it through Drop cleanup and detects same-thread reentry. discover_scratch creates its base, sweeps dead PID owners once per process, uses a unique call/PID name and creates the root; callers retain explicit src creation and cleanup. Neither helper is redesigned by this candidate. |

Focused semantic binding records deliberately state partial populations. The deep equality test checks a synthetic 100001-entry type closure and compares one emitted result to an independently shallow runtime environment; it is not a proof for all admitted packages. The schedule wrapper verifies 100000 linear call nodes, work/source/query boundaries and actual complete-crate compilation, but its FR-021-AC-27 tag supports the successful shape only; malformed/cyclic/shared/wrong-operation refusals live in other wrappers. The claims base-read wrapper supports one subpart of FR-041-AC-1, not the entire compiled census gate. Those inherited scopes are not inflated into new full discharges. Canonical attribute conversions in only claims and witness wrappers preserve exact existing criterion/TC membership; no tags are added to proof or production functions.

## Validation Evidence

Reviewer ran only static/read-only tools. Source identity and GitHub file/head metadata were independently read; frozen source remained clean. Actual author raw logs inspected: author/full-author-result.json, full-author-kani.log, full-author-outer.log, full-author-cleanup.json, rebase-check-events.jsonl, rebase-range-diff.log, confined-events.jsonl, consumers.log, scratch-controls.log and unit-{equality,schedule,publication,snapshot}.log. All paths are under /home/peter/dev/worktrees/kani-lead-codex/ir834/.

On this exact head, author actual `make kani-gate` exits 0, start 23:33:15Z, finish 23:46:34Z: 35 passed / expected 35, no failures/ignored, 799 gate seconds, 749.56 libtest seconds, Kani 0.68.0, clean/stable head before and after. Both target cleanup inventories contain only matching sibling lock files and no migrated fixture roots. Successful cleanup and existing helper stale/live-lock controls do not establish a killed real-backend recovery experiment. Rebased format, lint and lib+integration compile each exit 0 at this same head. Earlier seven helper controls, 87 default consumers / 15 ignored, equality1/schedule1/publication10/snapshot1 behavioral passes are attributed only to original 7a320ae3; all three rebased patches compare `=`. No unchanged behavioral refresh, author aggregate or reviewer heavy test was run. Initial formatting and unused-import failures are preserved, fixed and followed by actual passing checks. Exclusive heavy custody remains with the other assigned work; no probe is needed to establish FND-001.

## Examined Scope and Bindings

The following YAML is the complete machine-readable source/criterion scope (including clean units), original finding and actual focused bindings. Excerpts are verbatim actual source slices, capped at 600 characters. Context_only is not a semantic verification claim. Structural tags are not evidence-store discharge.

```yaml
repo: agent-ix/quire-contract-codegen
ticket: IR-834
pr: quire-contract-codegen#447
checked_head: 8b842a5020a3246effef19612f3e2eaf9af4d54e
assigned_base: ca21b99461b5075c4a805deca740683954830b04
run: ce89d6a6-ed82-4ef8-b019-5e78667251c3
model: gpt-6.1-sol
source_unchanged: true
artifact_id: SR-12970
method: code-review
clean: false
verdict: FAIL
scope:
- id: FR-005-AC-1
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: Failed generation leaves no partial published bundle or edited developer region.
- id: FR-005-AC-7
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: When publication fails before the destination is replaced, or fails after replacement began and the prior
    destination is restored, the publisher reports destination state `unchanged`, the destination holds its prior
    bytes or remains absent, an adjacent developer-owned file is byte-identical, and no staging or backup sibling
    remains.
- id: FR-005-AC-8
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: When removal of the replaced bundle's backup fails after the staged bundle has replaced the destination,
    the publisher reports destination state `published`, the destination holds every artifact of the new bundle,
    and the backup sibling remains as visible residue.
- id: FR-015-AC-1
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Pre, post, invariant and frame obligations of one contract produce separate harnesses with distinct identities.
- id: FR-015-AC-4
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: No harness assumption excludes an undefined, refused or incomplete runtime outcome.
- id: FR-017-AC-2
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: An absent launcher is refused with a typed reason naming its path before anything runs.
- id: FR-017-AC-4
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A run is verified only when the process exited successfully, the report states success, and every cover
    property is reported satisfied; a successful run with an unsatisfied cover is cover-unsatisfied with its satisfied
    and total counts; a report of success from an unsuccessfully exited process, a report with no cover property
    and a report with no successful check are each inconclusive with their own reason; console banners never decide
    the verdict.
- id: FR-017-AC-5
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A failed non-unwinding check with a concrete playback is falsified carrying that playback verbatim and
    never the playback of a satisfied cover; a failure with no playback is inconclusive for that reason; a failed
    unwinding assertion is inconclusive as an exhausted bound rather than falsified, and a succeeded unwinding check
    in a results listing is not a failure.
- id: FR-017-AC-6
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The execution evidence of a single run carries the obligation kind, the harness path, the launcher path,
    the complete argument vector, the unwind bound, the solver, the exit code and the outcome.
- id: FR-017-AC-7
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A crate whose library source does not contain the harness source byte for byte is refused, and no backend
    runs.
- id: FR-017-AC-21
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: N harnesses with equal option vectors (the harness selection removed) and equal identity `ProofCeilings`
    (`wall_clock` T and `memory_bytes` M) start exactly one launcher process whose argument vector holds one `--harness
    <module::harness> --exact` pair per member in request order, `--harness-timeout` T, then the shared options;
    N harnesses in G such groups start G processes; so N > 1 compatible harnesses start fewer than N processes;
    one harness starts one process with the single-run argument vector. Requests whose wall-clock or memory ceilings
    differ start separate processes. The group's proc
- id: FR-017-AC-22
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Over a real or captured batch report, members are matched by `module::harness` path (two members with
    the bare symbol `check` stay distinct), and each member''s outcome, checks and SUCCESS count come from its own
    entry: a falsified member beside a verified member leaves the verified member verified; a Success member in
    a non-zero exit with no Failure entry is inconclusive, and with one Failure entry is verified; a Failure entry
    with no checks and exit status `timeout` is inconclusive timed-out naming T while the others keep their results;
    a Failure entry with no checks and no timeout is inconcl'
- id: FR-017-AC-23
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A group's report that lacks a requested harness, holds one twice or holds an unrequested one refuses
    the group with a typed cause and classifies no member; a member whose source is not in the crate refuses the
    whole batch with `HarnessNotInCrate` and launches nothing; a falsified member's playback is the first counterexample
    block headed for its own path and never another member's block, a member that fails two property checks (two
    counterexample blocks under its path, as real Kani prints) takes the first, as the same harness alone does,
    and neither it nor its neighbours lose their evidence, a
- id: FR-017-AC-28
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Before either child starts, a package or harness whose path or bytes differ from the retained routed
    output, a selected oracle symbol or argument order that differs from the retained scalar identity, or an operand
    count/value outside the retained ordered ranges is refused with a typed cause and no receipt. For an accepted
    request, the exact supplied `ScalarLimits` are passed unchanged to `KaniScalarNativeRequest`, used to initialize
    the native meter, and returned in the receipt; no hidden or substituted limit value is permitted. CG attests
    these supplied, used and returned values are equal, bu
- id: FR-018-AC-28
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: With a caller-selected type-resolution work limit of at least 100,000 and `cg.equality.source_bytes`
    raised to the measured flat source size, an admitted acyclic type chain of 100,000 entries reconstructs and
    generates an oracle with the same result as an equivalent shallow type on a 512 KiB stack; with the same raised
    source-byte limit, the default type-resolution work limit refuses it at entry 65,537, and a selected work limit
    one below its entry count refuses it at the first denied entry. No depth setting or fixed depth refusal participates
    in these runs. PLANNED (IR-511).
- id: FR-018-AC-30
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The 100,000-entry acyclic closure renders as flat source without a type-depth-sized constructor expression;
    under the 1,048,576-byte default it returns `SourceTooLarge` naming that limit when the source exceeds it, and
    under a caller limit equal to its measured source bytes it emits a complete, compilable oracle within the publication
    artifact and bundle byte limits. One byte less refuses before that byte is appended and publishes no partial
    artifact. PLANNED (IR-511).
- id: FR-021-AC-7
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: A tree-shaped 100,000-node V2 `expression`/`call` graph whose nested children are separate admitted nodes
    reached by `reference` produces a flat SSA instruction table on a 512 KiB stack when caller work and source-byte
    limits fit it; no CG analysis or emission depth cap participates. The schedule crate compiles. Running a concrete
    oracle against a QSL evaluator is outside CG's boundary (IR-845); no Contract Runtime `MAX_CALL_DEPTH` behavior
    is required.
- id: FR-021-AC-15
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Each scheduled call point has one constant-size location record with function origin, stable occurrence
    ordinal, parent ordinal, child ordinal and instruction slot. A charged query reconstructs the same path as an
    independent walk of the request V2 reference graph; no record embeds the entire root-to-point path. A 100,000-link
    chain uses O(100,000) map storage and generation work.
- id: FR-021-AC-25
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'Across one body, the counter charges each node entry, child edge, emitted instruction and location record
    once in source traversal order. A setting equal to the required units emits a complete schedule; one less refuses
    that function as `GenerationWorkExhausted { limit, consumed: limit + 1 }` before the denied action, with no
    partial root or location record and no change to an unrelated function. A failed Contract IR lowering record
    keeps its lowering refusal and incurs no generation charge. PLANNED (IR-511).'
- id: FR-021-AC-27
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: An admitted nested body consists of separate V2 expression nodes connected only by `reference` leaves
    under QSpec FR-322, with each expression node reached from one parent in this CG subset; callee argument 0 binds
    an exact V2 function node, parameter references bind the declared parameter slots, and remaining arguments preserve
    source order. Missing, cyclic, shared-expression, unbound-parameter, ill-typed or wrong-operation edges refuse
    with a typed cause before schedule emission. PLANNED (IR-511).
- id: FR-021-AC-28
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Under the default 1,048,576-byte source limit, a 100,000-instruction body that needs more bytes returns
    `SourceTooLarge` naming the default; raising the caller limit to its measured flat source bytes emits a complete
    compilable schedule crate, with deterministic ordinal-range modules if one artifact would exceed its publication
    limit. One byte below the measured count refuses before append with no partial output. PLANNED (IR-511).
- id: FR-021-AC-29
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: A 100,000-call linear body produces 100,000 constant-size parent-linked location records; requesting
    its deepest path under sufficient query limits returns exactly 100,000 child ordinals, and a one-unit-below
    query limit refuses without a partial path. Generation does not materialize all paths. PLANNED (IR-511).
- id: FR-016-AC-8
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A witness schema binds the obligation's persisted argument bindings, in the order the obligation persists
    them, position for position to the harness's symbolic arguments, and each decoded value is named by the binding
    at its position; a binding that is not an argument is refused with a typed schema refusal that reports no failure
    and is none of the five replay results.
- id: FR-016-AC-9
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A decoded falsification whose witness makes the native function evaluate the clause to false settles
    as `reproduced-with-evaluated-witness` with category `violation`; the verdict depends on the witness value,
    so a witness at which the function holds does not settle so.
- id: FR-016-AC-10
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A decoded falsification at which the native function evaluates the clause to true settles `inconclusive`
    with the proved violation and the replayed success both named.
- id: FR-041-AC-1
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: A nonempty reviewed declaration is checked against the current compiled-module census and the merge-base
    declaration. An absent compiled module, a removed claim, and an empty list each name the claim or condition
    and fail; adding a compiled claim does not erase an older one.
- id: NFR-006-AC-13
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: For a Kani-touching pull request, the author runs the full gate on its clean, stable candidate before
    opening it, and the merger runs it again on the clean, stable current candidate before merge; both actual processes
    exit successfully with all expected tests run. A copied body line alone cannot satisfy either run.
- id: NFR-006-AC-15
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: A Kani-touching pull request whose required lane did not run or failed remains unmerged and does not
    claim real-Kani verification; a written explanation does not waive the gate.
- id: FR-005
  path: spec/core/functional/FR-005-cli-conformance.md
  role: context_only
  excerpt: 'When invoked through the library or CLI, the generator shall write a complete artifact bundle

    atomically and shall retain differential and cross-backend semantic-parity results.'
- id: FR-015
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: 'When a caller selects Kani generation for complete-V1 contracts whose scalar

    oracles FR-014 generated, the code generator shall emit one bounded Kani

    obligation per precondition, postcondition, invariant and frame condition,

    each bounded by its model-domain bounds. It is the Kani

    backend''s one generator

    ([ADR-001](../../decisions/ADR-001-overlapping-generators-and-input-models.md)),

    and every item it lowers is a claim over an admitted `quire.checked-package/v2`

    package.


    For production requests after the FR-330 consumer transition,

    [FR-039](../../core/functional/FR-039-contract-package-consump'
- id: FR-017
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: context_only
  excerpt: 'When a caller runs an FR-015, FR-022/FR-014 or FR-035 harness, the code generator shall invoke the

    installed Kani backend and retain the backend''s own reported outcome as typed

    execution evidence. A caller may hand the generator several harnesses to run together

    (FR-017-AC-21 to FR-017-AC-25); the generator then forms groups, runs each

    group in one launcher process, and retains one evidence record per harness. Unless a

    statement says "batch" or "group", it describes a run of one harness.


    [FR-040](./FR-040-cancellation-producer.md) adds the one launcher-observed cancellation outcome:

    the backe'
- id: FR-041
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: context_only
  excerpt: 'When the repository claims proof coverage for a generated-harness module, its existing

    [`make kani-gate`](../non-functional/NFR-006-real-kani-lane-gating.md) SHALL accept that claim only

    after a real Kani run discharges a proposition in the module and an executed mutation inside the

    module falsifies a proof check. The gate applies the proof-stage acceptance rule of QSL ADR-011

    §2.3 to CG''s claimed modules. It makes no claim about a module absent from its declared list or

    about a semantic stage the test input did not traverse.'
- id: NFR-006
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: context_only
  excerpt: "- The repository shall define the **Kani-touching set** as the changed paths matching\n  `src/kani/**`,\
    \ `src/oracle/**`, `src/routed/**`, `src/replay/**`, `src/core/**`,\n  `src/publication/**`, `tests/it/kani_*.rs`,\
    \ `tests/it/skeleton_spine.rs`,\n  `tests/it/bounded_kani_corpus.rs`, `tests/it/scratch_crate.rs`,\n  `tests/exact_scalar_support/package.rs`,\
    \ `tests/checked_package_support/base.rs`,\n  `tests/checked_package_support/rekey.rs`,\n  `tests/state_frame_support/**`,\
    \ `schemas/kani-*.schema.json`,\n  `schemas/generated-rust-kani-*.schema.json`, `Cargo.toml` and\n  `Cargo.lock`.\n\
    - The `make kani-"
- id: FR-018
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: 'When a caller supplies an admitted `quire.checked-package/v2` package and a set

    of equality expression nodes, the code generator shall emit a deterministic Rust

    oracle crate whose functions decide each node by reconstructing the item''s

    `TypeEnvironment` and calling the Contract Runtime

    `TypeEnvironment::check_equality` and `CheckedEquality::evaluate` with a

    caller-supplied `Meter`. It never compares two values structurally, and never

    copies a resource charge or a planned pair count.


    This is the composite/structural equality slice of complete-V1 oracle

    generation. [FR-014](./FR-014-exact-scala'
- id: FR-021
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: context_only
  excerpt: 'When a caller supplies an admitted `quire.checked-package/v2` package, function

    bindings and requested `call` nodes, the generator shall classify the admitted

    expression-node graph, build a deterministic, target-neutral instruction

    schedule for each admitted body, and record one generated or typed-refused

    disposition per item. The schedule preserves V2 operation identity, operand

    order, value/result types, function binding, and source occurrence identity;

    it states no charge amount and executes no operation itself. Its body analysis

    and schedule emission use charged iterative walks and no fixe'
- id: FR-016
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: context_only
  excerpt: 'When a FR-015 harness yields a counterexample, the code generator shall decode

    the witness into typed complete-V1 values, validate each value against its

    declared domain, and replay it through native runtime execution before any

    failure is reported.'
- id: StR-001
  path: spec/core/stakeholder/StR-001-traceable-generation.md
  role: context_only
  excerpt: 'Assurance engineers require that the generator shall derive executable, property-test, proof, and

    coverage artifacts reproducibly from one validated contract without hiding unsupported semantics,

    vacuity, rejection, or assumptions.'
- id: AD-004-L-2
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "- L-2. The import graph is acyclic and follows the dependency direction above, `#[cfg(test)]`\n  modules\
    \ included, and no file imports an item through the crate root. Test: a layout test that\n  reads `use crate::`\
    \ lines and flags a bare `crate::<Item>` path in code (inline types, calls;\n  not string literals or comments,\
    \ so intra-doc links are rewritten by step 2g-0 but not checked). It lands at the end of step 2, after the edge-removal\
    \ steps, so it can\n  pass when it lands."
- id: src/kani/run/scalar.rs::fn tc_027_changed_staged
  path: src/kani/run/scalar.rs
  role: examined
  excerpt: "    fn tc_027_changed_staged_bytes_and_unexpected_files_refuse_identity() {\n        let root = crate::kani::test_support::discover_scratch(\"\
    scalar-snapshot-control\");\n        fs::write(root.join(\"source.rs\"), b\"original\").unwrap();\n        let\
    \ expected = vec![(\"source.rs\".to_owned(), b\"original\".to_vec())];\n        assert!(ensure_snapshot_matches(&root,\
    \ &expected).is_ok());\n        fs::write(root.join(\"source.rs\"), b\"changed\").unwrap();\n        assert!(matches!(\n\
    \            ensure_snapshot_matches(&root, &expected),\n            Err(KaniScalarNativeError::SnapshotChanged)\n\
    \        ));\n        fs"
- id: src/oracle/equality/mod.rs::let scratch = crate::kani::test_support::discover_scratch
  path: src/oracle/equality/mod.rs
  role: examined
  excerpt: "        let scratch = crate::kani::test_support::discover_scratch(\"oracle-equality-deep\");\n     \
    \   std::fs::create_dir_all(scratch.join(\"src\")).expect(\"scratch source directory\");\n        std::fs::create_dir_all(scratch.join(\"\
    tests\")).expect(\"scratch test directory\");\n        std::fs::write(\n            scratch.join(\"Cargo.toml\"\
    ),\n            oracle_crate_manifest(COMPOSITE_EQUALITY_CRATE_NAME),\n        )\n        .expect(\"scratch\
    \ manifest\");\n        std::fs::copy(\n            concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/Cargo.lock\"),\n\
    \            scratch.join(\"Cargo.lock\"),\n        )"
- id: src/oracle/function/schedule.rs::crate::kani::test_support::discover_scratch
  path: src/oracle/function/schedule.rs
  role: examined
  excerpt: "                    crate::kani::test_support::discover_scratch(\"oracle-function-schedule\");\n   \
    \             let source_directory = directory.join(\"src\");\n                std::fs::create_dir_all(&source_directory).expect(\"\
    create source directory\");\n                std::fs::write(source_directory.join(\"lib.rs\"), source)\n   \
    \                 .expect(\"write schedule crate root\");\n                for module in modules {\n       \
    \             let path = directory.join(&module.path);\n                    std::fs::write(path, module.contents).expect(\"\
    write schedule module\");\n                }\n           "
- id: src/publication/publish.rs::fn temporary(
  path: src/publication/publish.rs
  role: examined
  excerpt: "    fn temporary(name: &str) -> PathBuf {\n        crate::kani::test_support::discover_scratch(&format!(\"\
    publication-{name}\"))\n    }\n\n    fn generated(path: &str, contents: &str) -> Artifact {\n        Artifact::new(path,\
    \ contents)\n    }\n\n    fn bundle(version: &str) -> ArtifactBundle {\n        ArtifactBundle::new(vec![\n\
    \            generated(\"src/generated.rs\", version),\n            generated(\"src/generated/nested.rs\", &format!(\"\
    nested {version}\")),"
- id: tests/it/kani_batching.rs::struct Lane
  path: tests/it/kani_batching.rs
  role: examined
  excerpt: "struct Lane {\n    _scratch: TemporaryDirectory,\n    crate_directory: PathBuf,\n    installation: KaniInstallation,\n\
    \    counts: PathBuf,\n    cancel: quire_exact::Cancel,\n}\n\nimpl Lane {\n    fn new(label: &str, harnesses:\
    \ &[StateFrameHarness]) -> Self {\n        let real = KaniInstallation::discover().expect(\"cargo-kani is installed\"\
    );\n        let scratch = TemporaryDirectory::new(&format!(\"quire-kani-batching-{label}\"));"
- id: tests/it/kani_generation_claims.rs::fn base_claims_allow
  path: tests/it/kani_generation_claims.rs
  role: examined
  excerpt: "fn base_claims_allow_proven_absence_but_refuse_failed_object_read() {\n    let scratch = crate::scratch_crate::TemporaryDirectory::new(\"\
    quire-kani-claims-base\");\n    let directory = &scratch.path;\n    let git = |args: &[&str]| {\n        let\
    \ output = Command::new(\"git\")\n            .arg(\"-C\")\n            .arg(directory)\n            .args(args)\n\
    \            .output()\n            .expect(\"scratch Git must launch\");\n        assert!(output.status.success(),\
    \ \"{}\", output_text(&output));\n        String::from_utf8(output.stdout).expect(\"UTF-8 Git output\")"
- id: tests/it/kani_obligations.rs::pub(crate) fn write_crate
  path: tests/it/kani_obligations.rs
  role: examined
  excerpt: "pub(crate) fn write_crate(harness: &KaniObligationHarness, subject: &str) -> TemporaryDirectory {\n\
    \    let scratch = TemporaryDirectory::new(\"quire-kani-obligations-crate\");\n    let directory = &scratch.path;\n\
    \    fs::write(\n        directory.join(\"src/lib.rs\"),\n        format!(\n            \"//! Generated obligation\
    \ check crate.\\n\\n{}\\n{subject}\",\n            harness.rust.contents\n        ),\n    )\n    .unwrap();\n\
    \    write_manifest("
- id: tests/it/kani_obligations_symbolic_domains.rs::fn write_collection_scratch
  path: tests/it/kani_obligations_symbolic_domains.rs
  role: examined
  excerpt: "fn write_collection_scratch(harness: &KaniCollectionObligationHarness) -> TemporaryDirectory {\n   \
    \ let scratch = TemporaryDirectory::new(\"quire-kani-symbolic-domains-collection\");\n    let directory = &scratch.path;\n\
    \    fs::write(directory.join(\"src/lib.rs\"), &harness.rust.contents)\n        .expect(\"write the generated\
    \ artifact byte for byte\");\n    write_manifest(\n        directory,\n        &format!(\n            \"[package]\\\
    nname = \\\"generated-kani-collection\\\"\\nversion = \\\"0.0.0\\\"\\nedition = \\\"2021\\\"\\npublish = false\\\
    n\\n[dependencies]\\n{}\\n\\n[workspace]\\n\",\n            runtime_dependency(&[\"exa"
- id: tests/it/kani_witness_join.rs::fn write_crate(
  path: tests/it/kani_witness_join.rs
  role: examined
  excerpt: "fn write_crate(harness: &KaniObligationHarness, subject: &str) -> TemporaryDirectory {\n    let scratch\
    \ = TemporaryDirectory::new(\"quire-kani-witness-join-crate\");\n    let directory = &scratch.path;\n    fs::write(\n\
    \        directory.join(\"src/lib.rs\"),\n        format!(\n            \"//! Generated obligation check crate.\\\
    n\\n{}\\n{subject}\",\n            harness.rust.contents\n        ),\n    )\n    .unwrap();\n    write_manifest("
- id: tests/it/skeleton_spine.rs::fn prove(
  path: tests/it/skeleton_spine.rs
  role: examined
  excerpt: "fn prove(harness: &KaniObligationHarness, subject: &str) -> KaniRunOutcome {\n    let cancel = quire_exact::Cancel::new();\n\
    \    let installation = KaniInstallation::discover().expect(\"cargo-kani is installed\");\n    let scratch =\
    \ write_crate(harness, subject);\n    let directory = &scratch.path;\n    let evidence = execute_kani_obligation(&KaniExecutionRequest\
    \ {\n        cancel: &cancel,\n        installation: &installation,\n        harness: harness.into(),\n    \
    \    crate_directory: directory,\n        target_directory: &PathBuf::from(env!(\"CARGO_TARGET_TMPDIR\")).join(\"\
    kani-spine\"),\n        home_direct"
- id: tests/it/scratch_crate.rs::pub(crate) struct TemporaryDirectory
  path: tests/it/scratch_crate.rs
  role: examined
  excerpt: "pub(crate) struct TemporaryDirectory {\n    pub(crate) path: PathBuf,\n    _lock: fs::File,\n}\n\nimpl\
    \ TemporaryDirectory {\n    pub(crate) fn new(prefix: &str) -> Self {\n        Self::in_directory(Path::new(env!(\"\
    CARGO_TARGET_TMPDIR\")), prefix)\n    }\n\n    fn in_directory(base: &Path, prefix: &str) -> Self {\n      \
    \  fs::create_dir_all(base).unwrap();"
- id: src/kani/test_support.rs::pub(crate) fn discover_scratch
  path: src/kani/test_support.rs
  role: examined
  excerpt: "pub(crate) fn discover_scratch(name: &str) -> PathBuf {\n    static SWEEP: std::sync::Once = std::sync::Once::new();\n\
    \    static CALLS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);\n    let executable\
    \ = env::current_exe().unwrap();\n    let target = executable\n        .ancestors()\n        .nth(3)\n     \
    \   .expect(\"the test executable sits under a target directory\");\n    let base = target.join(\"tmp\").join(\"\
    quire-kani-discover\");\n    fs::create_dir_all(&base).unwrap();\n    SWEEP.call_once(|| {\n        for entry\
    \ in fs::read_dir(&base).unwrap().flatten() {"
- id: tests/it/layout.rs::fn l_2_every_import
  path: tests/it/layout.rs
  role: examined
  excerpt: "fn l_2_every_import_follows_the_dependency_direction() {\n    let files = source_files();\n    let modules:\
    \ BTreeSet<Vec<String>> = files.keys().map(|file| module_of(file)).collect();\n    let mut violations = Vec::new();\n\
    \    for edge in edges(&files) {\n        let from = module_of(&edge.file);\n        let (Some(directory), Some(target))\
    \ = (from.first(), edge.path.first()) else {\n            continue;\n        };\n        let reach = may_import(directory);\n\
    \        if directory != target && !reach.contains(&target.as_str()) {\n            violations.push(format!("
findings:
- fnd: FND-001
  severity: high
  confidence: high
  method: code-review
  check: code-bug
  artifact_id: AD-004-L-2
  related_ids:
  - AD-004
  path: src/oracle/equality/mod.rs
  lines: '2873'
  excerpt: "- L-2. The import graph is acyclic and follows the dependency direction above, `#[cfg(test)]`\n  modules\
    \ included, and no file imports an item through the crate root. Test: a layout test that\n  reads `use crate::`\
    \ lines and flags a bare `crate::<Item>` path in code (inline types, calls;\n  not string literals or comments,\
    \ so intra-doc links are rewritten by step 2g-0 but not checked). It lands at the end of step 2, after the edge-removal\
    \ steps, so it can\n  pass when it lands."
  finding: Cross-subsystem unit tests introduce Kani back-edges forbidden by AD-004 L-2.
  scenario: The next full layout gate discovers oracle/equality -> kani::test_support, oracle/function/schedule
    -> kani::test_support and publication/publish -> kani::test_support. AD-004 permits oracle/publication -> core
    only, including cfg(test); tests/it/layout.rs:524 applies may_import to every scanned edge. These additions
    violate the existing gate even though fmt/lint/compile and the real-Kani lane pass.
  additional_locations:
  - src/oracle/function/schedule.rs:1279
  - src/publication/publish.rs:273
  required_fix: Reuse one shared target/dead-PID allocator from a directionally valid cfg(test) location; preserve
    its behavior and existing Kani callers, without duplication or a scanner exception. No new normative exception
    is authorized.
bindings:
- test_id: src/oracle/equality/mod.rs::deep_acyclic_closure_generates_flat_compilable_source_on_a_small_stack
  ac_id: FR-018-AC-28
  trace: correct
  support: 100001-entry synthetic closure, raised/default/one-below work, shallow runtime comparison on 512 KiB;
    one family, not every admitted package.
  complete_criterion_discharge: false
- test_id: src/oracle/equality/mod.rs::deep_acyclic_closure_generates_flat_compilable_source_on_a_small_stack
  ac_id: FR-018-AC-30
  trace: correct
  support: Flat modules, default/exact/one-below bytes and actual nested Cargo; no whole-source-budget proof.
  complete_criterion_discharge: false
- test_id: src/oracle/function/schedule.rs::ac7_hundred_thousand_call_nodes_schedule_on_a_512_kib_stack
  ac_id: FR-021-AC-7
  trace: correct
  support: 100000-call V2 reference graph; actual rustc of emitted full crate.
  complete_criterion_discharge: false
- test_id: src/oracle/function/schedule.rs::ac7_hundred_thousand_call_nodes_schedule_on_a_512_kib_stack
  ac_id: FR-021-AC-15
  trace: correct
  support: Linear parent-linked shape and deepest path; separate sibling tests cover the remaining independent-graph
    population.
  complete_criterion_discharge: false
- test_id: src/oracle/function/schedule.rs::ac7_hundred_thousand_call_nodes_schedule_on_a_512_kib_stack
  ac_id: FR-021-AC-25
  trace: correct
  support: Exact work 500003/one-below for linear graph; sibling isolation and failed lowering are not asserted
    by this wrapper.
  complete_criterion_discharge: false
- test_id: src/oracle/function/schedule.rs::ac7_hundred_thousand_call_nodes_schedule_on_a_512_kib_stack
  ac_id: FR-021-AC-27
  trace: correct
  support: Admitted nested reference success branch; malformed/cyclic/shared and wrong-operation refusals require
    other wrappers.
  complete_criterion_discharge: false
- test_id: src/oracle/function/schedule.rs::ac7_hundred_thousand_call_nodes_schedule_on_a_512_kib_stack
  ac_id: FR-021-AC-28
  trace: correct
  support: Default/one-below source refusals, deterministic module naming/size, complete rustc; not QSL evaluator
    agreement.
  complete_criterion_discharge: false
- test_id: src/oracle/function/schedule.rs::ac7_hundred_thousand_call_nodes_schedule_on_a_512_kib_stack
  ac_id: FR-021-AC-29
  trace: correct
  support: 100000-link deepest path and one-below work/output bytes.
  complete_criterion_discharge: false
- test_id: src/kani/run/scalar.rs::tc_027_changed_staged_bytes_and_unexpected_files_refuse_identity
  ac_id: FR-017-AC-28
  trace: correct
  support: Exact original bytes accepted, changed bytes and extra file each SnapshotChanged; does not independently
    discharge all paired native receipt semantics.
  complete_criterion_discharge: false
- test_id: tests/it/kani_witness_join.rs::tc_026_real_falsification_decodes_against_the_persisted_schema_and_mutation_refuses
  ac_id: FR-016-AC-8
  trace: correct
  support: Actual persisted two-i64 positional decode, positive credit witnesses defect, dropped/retyped bindings
    refuse; nonargument refusal population is elsewhere.
  complete_criterion_discharge: false
- test_id: tests/it/skeleton_spine.rs::tc_026_one_boolean_clause_goes_from_a_bound_package_through_kani_to_native_replay
  ac_id: FR-016-AC-9
  trace: correct
  support: Hand-built Boolean clause healthy/seeded defect, real Kani, independent playback values/bytes and QSL
    native replay; one case.
  complete_criterion_discharge: false
- test_id: tests/it/skeleton_spine.rs::tc_026_one_boolean_clause_goes_from_a_bound_package_through_kani_to_native_replay
  ac_id: FR-016-AC-10
  trace: correct
  support: Same violation reproduces through QSL twin; does not claim all replay results from one spine case.
  complete_criterion_discharge: false
- test_id: tests/it/kani_generation_claims.rs::base_claims_allow_proven_absence_but_refuse_failed_object_read
  ac_id: FR-041-AC-1
  trace: correct
  support: Git absent declaration -> empty, present -> claimed, deleted loose blob -> fail closed. This wrapper
    supports base-read distinction only; full census/removal/empty/additive controls are elsewhere.
  complete_criterion_discharge: false
- test_id: tests/it/kani_batching.rs::tc_043_real_kani_one_process_runs_n_harnesses_where_n_ran_before
  ac_id: FR-017-AC-21
  trace: correct
  support: Real counting launcher N=1/10/50 singles vs one group, exact member count and outcomes; shared target
    remains unchanged.
  complete_criterion_discharge: false
- test_id: tests/it/kani_obligations.rs::tc_025_real_kani_runs_verify_separate_obligations_and_falsify_a_seeded_defect
  ac_id: FR-017-AC-4
  trace: correct
  support: Three healthy obligation kinds verify with real exit/options.
  complete_criterion_discharge: false
- test_id: tests/it/kani_obligations.rs::tc_025_real_kani_runs_verify_separate_obligations_and_falsify_a_seeded_defect
  ac_id: FR-017-AC-5
  trace: correct
  support: Seeded addition defect falsifies with actual concrete playback and failing exit.
  complete_criterion_discharge: false
- test_id: tests/it/kani_obligations.rs::tc_025_real_kani_runs_verify_separate_obligations_and_falsify_a_seeded_defect
  ac_id: FR-017-AC-6
  trace: correct
  support: Unsatisfiable requires yields cover-unsatisfied 0/1, never Verified.
  complete_criterion_discharge: false
- test_id: tests/it/kani_obligations.rs::tc_025_real_kani_runs_verify_separate_obligations_and_falsify_a_seeded_defect
  ac_id: FR-017-AC-7
  trace: correct
  support: 1ms real invocation gives typed TimedOut/exit None; changed fixture drop precedes same-prefix missing-harness
    reacquisition. Timeout mutation evidence belongs to launch tests, not an elapsed-time assertion here.
  complete_criterion_discharge: false
```

## Dispositions

Round 1, checked at frozen clean final fix head `4e299c590644a2769b28b7015a1611111125e02f` over unchanged base `ca21b99461b5075c4a805deca740683954830b04`.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The one unchanged discover_scratch allocator and SWEEP/CALLS state now live in cfg(test) core/test_support.rs. Every caller imports core directly, including library-included private modules. Actual unchanged L-2 direction and graph-acyclic tests pass on 4e299c5. |

## Disposition Round 1 Verdict

**PASS for current independent code/Rust review.** FND-001 is fixed; zero new findings. Original Findings/Verdict and the entire original artifact prefix remain unchanged as historical review. Candidate still requires latest reviewer artifact custody and root's final aggregate/full actual Kani before merge; this is not a completed final-gate claim.

The full current diff has 23 Rust files, one AD-004 placement update and the two historical review artifacts (26 paths). Necessary helper relocation introduces no production interface or algorithm change: allocator docs/body/local SWEEP/CALLS state match original bytes, exactly one definition exists, all callers use core directly, and no compatibility wrapper, duplicate implementation, scanner exception or direction rule change exists. The sole layout edit is its actual module-file inventory entry. AD-004 preserves dependency direction and the original six helper identities as five Kani-specific helpers plus one core allocator. Cancellation bodies and all existing test/proof assertions, ignored status, generated manifests/workspace isolation, lock seeding and commands/options remain unchanged apart from necessary helper import paths. SR-12972 separately reviews the changed architecture placement prose.

Correction to static preparation: the earlier src-only caller search missed three references in tests/it/kani_render_private.rs and kani_scalar_native_private.rs, compiled into libtest via src/lib.rs #[path] modules. Actual initial lint found them. Final fix changes only the necessary import/two inline calls, preserving bodies/traces. The final scope includes both files and a recursive inventory of ten explicit library path sources, plus the whole Rust-tree caller search (39 reference lines, one definition, no old helper path). The one non-resolving lexical #[path] occurrence is text emitted inside the existing native helper raw string in scalar.rs, not a source module declaration. The static preparation was not compile proof.

Actual final-head author results independently read from fix1-check-events.jsonl and raw fix1-*.log: fmt/lint/lib+it no-run/docs each exit0; layout6 PASS including actual dependency direction, acyclicity, scanner and re-export checks; scratch7 PASS; affected Kani/private caller tests100 PASS/4 ignored; equality1 actual nested Cargo, schedule1 actual rustc, publication10 and snapshot1 PASS, serialized. The private test filter also includes existing package-fixture tests; 100 is the libtest reported selection, not 100 new allocator tests. Initial fmt exit2 and lint exit2 remain in fix1-fmt-check-initial.log/fix1-lint-initial.log and events; corrections preceded all final checks. Cleanup shows no migrated unit fixture roots; both targets retained. Explicit resource release is recorded at 00:17:40Z with exact process guard exit1. No reviewer compiler/backend/aggregate was run.

Evidence attribution: prior full author Kani35/35 and 799gate/749.56test seconds remain ONLY on 8b842a5. Original87consumer/15ignored and earlier7helper+unit passes remain ONLY on 7a320ae3 before unchanged rebase. They are not refreshed on 4e299c5. Final root full aggregate/Kani are pending after latest artifact custody; no waiver, killed-real-backend recovery or all-production-/tmp-elimination claim is made. Original exclusions remain.

## Disposition Round 1 Scope and Bindings

Current scope/excerpts below bind exactly to final fix 4e299c5 and supersede no original historical record. Full source/criterion contexts, all clean units and actual focused bindings are retained; local AD-004 L-2 verification bindings are explicitly distinguished from engine-minted AC targets.

```yaml
repo: agent-ix/quire-contract-codegen
ticket: IR-834
pr: quire-contract-codegen#447
checked_head: 4e299c590644a2769b28b7015a1611111125e02f
assigned_base: ca21b99461b5075c4a805deca740683954830b04
original_review_head: 8b842a5020a3246effef19612f3e2eaf9af4d54e
round: 1
model: gpt-6.1-sol
run: ce89d6a6-ed82-4ef8-b019-5e78667251c3
source_unchanged_during_review: true
no_new_findings: true
findings: []
bindings:
- test_id: src/oracle/equality/mod.rs::deep_acyclic_closure_generates_flat_compilable_source_on_a_small_stack
  ac_id: FR-018-AC-28
  trace: correct
  support: 100001-entry synthetic closure, raised/default/one-below work, shallow runtime comparison on 512 KiB;
    one family, not every admitted package.
  complete_criterion_discharge: false
- test_id: src/oracle/equality/mod.rs::deep_acyclic_closure_generates_flat_compilable_source_on_a_small_stack
  ac_id: FR-018-AC-30
  trace: correct
  support: Flat modules, default/exact/one-below bytes and actual nested Cargo; no whole-source-budget proof.
  complete_criterion_discharge: false
- test_id: src/oracle/function/schedule.rs::ac7_hundred_thousand_call_nodes_schedule_on_a_512_kib_stack
  ac_id: FR-021-AC-7
  trace: correct
  support: 100000-call V2 reference graph; actual rustc of emitted full crate.
  complete_criterion_discharge: false
- test_id: src/oracle/function/schedule.rs::ac7_hundred_thousand_call_nodes_schedule_on_a_512_kib_stack
  ac_id: FR-021-AC-15
  trace: correct
  support: Linear parent-linked shape and deepest path; separate sibling tests cover the remaining independent-graph
    population.
  complete_criterion_discharge: false
- test_id: src/oracle/function/schedule.rs::ac7_hundred_thousand_call_nodes_schedule_on_a_512_kib_stack
  ac_id: FR-021-AC-25
  trace: correct
  support: Exact work 500003/one-below for linear graph; sibling isolation and failed lowering are not asserted
    by this wrapper.
  complete_criterion_discharge: false
- test_id: src/oracle/function/schedule.rs::ac7_hundred_thousand_call_nodes_schedule_on_a_512_kib_stack
  ac_id: FR-021-AC-27
  trace: correct
  support: Admitted nested reference success branch; malformed/cyclic/shared and wrong-operation refusals require
    other wrappers.
  complete_criterion_discharge: false
- test_id: src/oracle/function/schedule.rs::ac7_hundred_thousand_call_nodes_schedule_on_a_512_kib_stack
  ac_id: FR-021-AC-28
  trace: correct
  support: Default/one-below source refusals, deterministic module naming/size, complete rustc; not QSL evaluator
    agreement.
  complete_criterion_discharge: false
- test_id: src/oracle/function/schedule.rs::ac7_hundred_thousand_call_nodes_schedule_on_a_512_kib_stack
  ac_id: FR-021-AC-29
  trace: correct
  support: 100000-link deepest path and one-below work/output bytes.
  complete_criterion_discharge: false
- test_id: src/kani/run/scalar.rs::tc_027_changed_staged_bytes_and_unexpected_files_refuse_identity
  ac_id: FR-017-AC-28
  trace: correct
  support: Exact original bytes accepted, changed bytes and extra file each SnapshotChanged; does not independently
    discharge all paired native receipt semantics.
  complete_criterion_discharge: false
- test_id: tests/it/kani_witness_join.rs::tc_026_real_falsification_decodes_against_the_persisted_schema_and_mutation_refuses
  ac_id: FR-016-AC-8
  trace: correct
  support: Actual persisted two-i64 positional decode, positive credit witnesses defect, dropped/retyped bindings
    refuse; nonargument refusal population is elsewhere.
  complete_criterion_discharge: false
- test_id: tests/it/skeleton_spine.rs::tc_026_one_boolean_clause_goes_from_a_bound_package_through_kani_to_native_replay
  ac_id: FR-016-AC-9
  trace: correct
  support: Hand-built Boolean clause healthy/seeded defect, real Kani, independent playback values/bytes and QSL
    native replay; one case.
  complete_criterion_discharge: false
- test_id: tests/it/skeleton_spine.rs::tc_026_one_boolean_clause_goes_from_a_bound_package_through_kani_to_native_replay
  ac_id: FR-016-AC-10
  trace: correct
  support: Same violation reproduces through QSL twin; does not claim all replay results from one spine case.
  complete_criterion_discharge: false
- test_id: tests/it/kani_generation_claims.rs::base_claims_allow_proven_absence_but_refuse_failed_object_read
  ac_id: FR-041-AC-1
  trace: correct
  support: Git absent declaration -> empty, present -> claimed, deleted loose blob -> fail closed. This wrapper
    supports base-read distinction only; full census/removal/empty/additive controls are elsewhere.
  complete_criterion_discharge: false
- test_id: tests/it/kani_batching.rs::tc_043_real_kani_one_process_runs_n_harnesses_where_n_ran_before
  ac_id: FR-017-AC-21
  trace: correct
  support: Real counting launcher N=1/10/50 singles vs one group, exact member count and outcomes; shared target
    remains unchanged.
  complete_criterion_discharge: false
- test_id: tests/it/kani_obligations.rs::tc_025_real_kani_runs_verify_separate_obligations_and_falsify_a_seeded_defect
  ac_id: FR-017-AC-4
  trace: correct
  support: Three healthy obligation kinds verify with real exit/options.
  complete_criterion_discharge: false
- test_id: tests/it/kani_obligations.rs::tc_025_real_kani_runs_verify_separate_obligations_and_falsify_a_seeded_defect
  ac_id: FR-017-AC-5
  trace: correct
  support: Seeded addition defect falsifies with actual concrete playback and failing exit.
  complete_criterion_discharge: false
- test_id: tests/it/kani_obligations.rs::tc_025_real_kani_runs_verify_separate_obligations_and_falsify_a_seeded_defect
  ac_id: FR-017-AC-6
  trace: correct
  support: Unsatisfiable requires yields cover-unsatisfied 0/1, never Verified.
  complete_criterion_discharge: false
- test_id: tests/it/kani_obligations.rs::tc_025_real_kani_runs_verify_separate_obligations_and_falsify_a_seeded_defect
  ac_id: FR-017-AC-7
  trace: correct
  support: 1ms real invocation gives typed TimedOut/exit None; changed fixture drop precedes same-prefix missing-harness
    reacquisition. Timeout mutation evidence belongs to launch tests, not an elapsed-time assertion here.
  complete_criterion_discharge: false
- test_id: tests/it/layout.rs::l_2_every_import_follows_the_dependency_direction
  ac_id: AD-004-L-2
  trace: correct
  binding_basis: AD-004 local L-2 verification prose and test documentation; this is not an engine-minted acceptance-criterion
    target.
  support: Actual final-head layout test passes while the unchanged scanner walks imports/inline crate paths, includes
    cfg(test), and applies existing directional restrictions. Source inspection additionally covers library-included
    external test modules.
  complete_criterion_discharge: false
- test_id: tests/it/layout.rs::l_2_the_module_graph_is_acyclic
  ac_id: AD-004-L-2
  trace: correct
  binding_basis: AD-004 local L-2 verification prose and test documentation; not a newly added trace tag.
  support: Actual final-head acyclicity test passes; combined with mechanical helper relocation and direct-call
    inspection. No producer/evidence-store semantic discharge is inferred.
  complete_criterion_discharge: false
actual_final_head_checks:
- label: fmt-check
  command:
  - make
  - fmt-check
  head: 4e299c590644a2769b28b7015a1611111125e02f
  started: '2026-10-19T00:16:00.793224+00:00'
  finished: '2026-10-19T00:16:01.403856+00:00'
  exit: 0
  log: /home/peter/dev/worktrees/kani-lead-codex/ir834/author/fix1-fmt-check.log
- label: lint
  command:
  - make
  - lint
  head: 4e299c590644a2769b28b7015a1611111125e02f
  started: '2026-10-19T00:16:01.405224+00:00'
  finished: '2026-10-19T00:16:03.178930+00:00'
  exit: 0
  log: /home/peter/dev/worktrees/kani-lead-codex/ir834/author/fix1-lint.log
- label: compile
  command:
  - /home/peter/.cargo/bin/cargo
  - test
  - --locked
  - --lib
  - --test
  - it
  - --no-run
  head: 4e299c590644a2769b28b7015a1611111125e02f
  started: '2026-10-19T00:16:03.180439+00:00'
  finished: '2026-10-19T00:16:06.748177+00:00'
  exit: 0
  log: /home/peter/dev/worktrees/kani-lead-codex/ir834/author/fix1-compile.log
- label: docs
  command:
  - quire
  - validate
  - --scope
  - .
  - spec/assurance/AD-004-cg-crate-layout.md
  - reviews/ir-834-code-review.md
  - reviews/ir-834-gap-analysis.md
  head: 4e299c590644a2769b28b7015a1611111125e02f
  started: '2026-10-19T00:16:06.749706+00:00'
  finished: '2026-10-19T00:16:06.833725+00:00'
  exit: 0
  log: /home/peter/dev/worktrees/kani-lead-codex/ir834/author/fix1-docs.log
- label: layout
  command:
  - /home/peter/.cargo/bin/cargo
  - test
  - --locked
  - --test
  - it
  - 'layout::'
  - --
  - --test-threads=1
  head: 4e299c590644a2769b28b7015a1611111125e02f
  started: '2026-10-19T00:16:06.835205+00:00'
  finished: '2026-10-19T00:16:07.164695+00:00'
  exit: 0
  log: /home/peter/dev/worktrees/kani-lead-codex/ir834/author/fix1-layout.log
- label: scratch-controls
  command:
  - /home/peter/.cargo/bin/cargo
  - test
  - --locked
  - --test
  - it
  - 'scratch_crate::tests::'
  - --
  - --test-threads=1
  head: 4e299c590644a2769b28b7015a1611111125e02f
  started: '2026-10-19T00:16:07.165879+00:00'
  finished: '2026-10-19T00:16:07.568333+00:00'
  exit: 0
  log: /home/peter/dev/worktrees/kani-lead-codex/ir834/author/fix1-scratch-controls.log
- label: unit-callers
  command:
  - /home/peter/.cargo/bin/cargo
  - test
  - --locked
  - --lib
  - --
  - --test-threads=1
  - 'kani::run::execute::'
  - 'kani::run::launch::'
  - 'kani::run::memory::'
  - 'kani::run::namespace::'
  - 'kani::run::report_file::'
  - 'kani::run::tool::'
  - kani::generate::render::tests::rendering_is_identical_in_another_process
  - 'kani_scalar_native_private::'
  - 'kani_render_private::'
  head: 4e299c590644a2769b28b7015a1611111125e02f
  started: '2026-10-19T00:16:07.569751+00:00'
  finished: '2026-10-19T00:16:32.496233+00:00'
  exit: 0
  log: /home/peter/dev/worktrees/kani-lead-codex/ir834/author/fix1-unit-callers.log
- label: unit-equality
  command:
  - /home/peter/.cargo/bin/cargo
  - test
  - --locked
  - --lib
  - deep_acyclic_closure_generates_flat_compilable_source_on_a_small_stack
  - --
  - --test-threads=1
  head: 4e299c590644a2769b28b7015a1611111125e02f
  started: '2026-10-19T00:16:32.497648+00:00'
  finished: '2026-10-19T00:16:51.995777+00:00'
  exit: 0
  log: /home/peter/dev/worktrees/kani-lead-codex/ir834/author/fix1-unit-equality.log
- label: unit-schedule
  command:
  - /home/peter/.cargo/bin/cargo
  - test
  - --locked
  - --lib
  - ac7_hundred_thousand_call_nodes_schedule_on_a_512_kib_stack
  - --
  - --test-threads=1
  head: 4e299c590644a2769b28b7015a1611111125e02f
  started: '2026-10-19T00:16:51.997145+00:00'
  finished: '2026-10-19T00:17:06.990778+00:00'
  exit: 0
  log: /home/peter/dev/worktrees/kani-lead-codex/ir834/author/fix1-unit-schedule.log
- label: unit-publication
  command:
  - /home/peter/.cargo/bin/cargo
  - test
  - --locked
  - --lib
  - 'publication::publish::tests::'
  - --
  - --test-threads=1
  head: 4e299c590644a2769b28b7015a1611111125e02f
  started: '2026-10-19T00:17:06.992349+00:00'
  finished: '2026-10-19T00:17:07.143855+00:00'
  exit: 0
  log: /home/peter/dev/worktrees/kani-lead-codex/ir834/author/fix1-unit-publication.log
- label: unit-snapshot
  command:
  - /home/peter/.cargo/bin/cargo
  - test
  - --locked
  - --lib
  - tc_027_changed_staged_bytes_and_unexpected_files_refuse_identity
  - --
  - --test-threads=1
  head: 4e299c590644a2769b28b7015a1611111125e02f
  started: '2026-10-19T00:17:07.145342+00:00'
  finished: '2026-10-19T00:17:07.237290+00:00'
  exit: 0
  log: /home/peter/dev/worktrees/kani-lead-codex/ir834/author/fix1-unit-snapshot.log
candidate_limits: No open review finding remains. Root final aggregate/full actual Kani and latest artifact custody
  still required before merge; old author35/35 applies only to original review head.
historical_evidence_limits: Initial src-only static preparation missed three references in two library-included
  test files. Actual lint failed at 4618e7f, the mechanical imports were corrected, and all recorded final-head
  checks then passed. The old static report was not compile proof.
artifact_id: SR-12970
method: code-review
current_review_verdict: PASS after fixed FND-001; final gates pending
clean: true
scope:
- id: FR-005-AC-1
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: Failed generation leaves no partial published bundle or edited developer region.
- id: FR-005-AC-7
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: When publication fails before the destination is replaced, or fails after replacement began and the prior
    destination is restored, the publisher reports destination state `unchanged`, the destination holds its prior
    bytes or remains absent, an adjacent developer-owned file is byte-identical, and no staging or backup sibling
    remains.
- id: FR-005-AC-8
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: When removal of the replaced bundle's backup fails after the staged bundle has replaced the destination,
    the publisher reports destination state `published`, the destination holds every artifact of the new bundle,
    and the backup sibling remains as visible residue.
- id: FR-015-AC-1
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Pre, post, invariant and frame obligations of one contract produce separate harnesses with distinct identities.
- id: FR-015-AC-4
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: No harness assumption excludes an undefined, refused or incomplete runtime outcome.
- id: FR-017-AC-2
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: An absent launcher is refused with a typed reason naming its path before anything runs.
- id: FR-017-AC-4
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A run is verified only when the process exited successfully, the report states success, and every cover
    property is reported satisfied; a successful run with an unsatisfied cover is cover-unsatisfied with its satisfied
    and total counts; a report of success from an unsuccessfully exited process, a report with no cover property
    and a report with no successful check are each inconclusive with their own reason; console banners never decide
    the verdict.
- id: FR-017-AC-5
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A failed non-unwinding check with a concrete playback is falsified carrying that playback verbatim and
    never the playback of a satisfied cover; a failure with no playback is inconclusive for that reason; a failed
    unwinding assertion is inconclusive as an exhausted bound rather than falsified, and a succeeded unwinding check
    in a results listing is not a failure.
- id: FR-017-AC-6
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The execution evidence of a single run carries the obligation kind, the harness path, the launcher path,
    the complete argument vector, the unwind bound, the solver, the exit code and the outcome.
- id: FR-017-AC-7
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A crate whose library source does not contain the harness source byte for byte is refused, and no backend
    runs.
- id: FR-017-AC-21
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: N harnesses with equal option vectors (the harness selection removed) and equal identity `ProofCeilings`
    (`wall_clock` T and `memory_bytes` M) start exactly one launcher process whose argument vector holds one `--harness
    <module::harness> --exact` pair per member in request order, `--harness-timeout` T, then the shared options;
    N harnesses in G such groups start G processes; so N > 1 compatible harnesses start fewer than N processes;
    one harness starts one process with the single-run argument vector. Requests whose wall-clock or memory ceilings
    differ start separate processes. The group's proc
- id: FR-017-AC-22
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Over a real or captured batch report, members are matched by `module::harness` path (two members with
    the bare symbol `check` stay distinct), and each member''s outcome, checks and SUCCESS count come from its own
    entry: a falsified member beside a verified member leaves the verified member verified; a Success member in
    a non-zero exit with no Failure entry is inconclusive, and with one Failure entry is verified; a Failure entry
    with no checks and exit status `timeout` is inconclusive timed-out naming T while the others keep their results;
    a Failure entry with no checks and no timeout is inconcl'
- id: FR-017-AC-23
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A group's report that lacks a requested harness, holds one twice or holds an unrequested one refuses
    the group with a typed cause and classifies no member; a member whose source is not in the crate refuses the
    whole batch with `HarnessNotInCrate` and launches nothing; a falsified member's playback is the first counterexample
    block headed for its own path and never another member's block, a member that fails two property checks (two
    counterexample blocks under its path, as real Kani prints) takes the first, as the same harness alone does,
    and neither it nor its neighbours lose their evidence, a
- id: FR-017-AC-28
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Before either child starts, a package or harness whose path or bytes differ from the retained routed
    output, a selected oracle symbol or argument order that differs from the retained scalar identity, or an operand
    count/value outside the retained ordered ranges is refused with a typed cause and no receipt. For an accepted
    request, the exact supplied `ScalarLimits` are passed unchanged to `KaniScalarNativeRequest`, used to initialize
    the native meter, and returned in the receipt; no hidden or substituted limit value is permitted. CG attests
    these supplied, used and returned values are equal, bu
- id: FR-018-AC-28
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: With a caller-selected type-resolution work limit of at least 100,000 and `cg.equality.source_bytes`
    raised to the measured flat source size, an admitted acyclic type chain of 100,000 entries reconstructs and
    generates an oracle with the same result as an equivalent shallow type on a 512 KiB stack; with the same raised
    source-byte limit, the default type-resolution work limit refuses it at entry 65,537, and a selected work limit
    one below its entry count refuses it at the first denied entry. No depth setting or fixed depth refusal participates
    in these runs. PLANNED (IR-511).
- id: FR-018-AC-30
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The 100,000-entry acyclic closure renders as flat source without a type-depth-sized constructor expression;
    under the 1,048,576-byte default it returns `SourceTooLarge` naming that limit when the source exceeds it, and
    under a caller limit equal to its measured source bytes it emits a complete, compilable oracle within the publication
    artifact and bundle byte limits. One byte less refuses before that byte is appended and publishes no partial
    artifact. PLANNED (IR-511).
- id: FR-021-AC-7
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: A tree-shaped 100,000-node V2 `expression`/`call` graph whose nested children are separate admitted nodes
    reached by `reference` produces a flat SSA instruction table on a 512 KiB stack when caller work and source-byte
    limits fit it; no CG analysis or emission depth cap participates. The schedule crate compiles. Running a concrete
    oracle against a QSL evaluator is outside CG's boundary (IR-845); no Contract Runtime `MAX_CALL_DEPTH` behavior
    is required.
- id: FR-021-AC-15
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Each scheduled call point has one constant-size location record with function origin, stable occurrence
    ordinal, parent ordinal, child ordinal and instruction slot. A charged query reconstructs the same path as an
    independent walk of the request V2 reference graph; no record embeds the entire root-to-point path. A 100,000-link
    chain uses O(100,000) map storage and generation work.
- id: FR-021-AC-25
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'Across one body, the counter charges each node entry, child edge, emitted instruction and location record
    once in source traversal order. A setting equal to the required units emits a complete schedule; one less refuses
    that function as `GenerationWorkExhausted { limit, consumed: limit + 1 }` before the denied action, with no
    partial root or location record and no change to an unrelated function. A failed Contract IR lowering record
    keeps its lowering refusal and incurs no generation charge. PLANNED (IR-511).'
- id: FR-021-AC-27
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: An admitted nested body consists of separate V2 expression nodes connected only by `reference` leaves
    under QSpec FR-322, with each expression node reached from one parent in this CG subset; callee argument 0 binds
    an exact V2 function node, parameter references bind the declared parameter slots, and remaining arguments preserve
    source order. Missing, cyclic, shared-expression, unbound-parameter, ill-typed or wrong-operation edges refuse
    with a typed cause before schedule emission. PLANNED (IR-511).
- id: FR-021-AC-28
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Under the default 1,048,576-byte source limit, a 100,000-instruction body that needs more bytes returns
    `SourceTooLarge` naming the default; raising the caller limit to its measured flat source bytes emits a complete
    compilable schedule crate, with deterministic ordinal-range modules if one artifact would exceed its publication
    limit. One byte below the measured count refuses before append with no partial output. PLANNED (IR-511).
- id: FR-021-AC-29
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: A 100,000-call linear body produces 100,000 constant-size parent-linked location records; requesting
    its deepest path under sufficient query limits returns exactly 100,000 child ordinals, and a one-unit-below
    query limit refuses without a partial path. Generation does not materialize all paths. PLANNED (IR-511).
- id: FR-016-AC-8
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A witness schema binds the obligation's persisted argument bindings, in the order the obligation persists
    them, position for position to the harness's symbolic arguments, and each decoded value is named by the binding
    at its position; a binding that is not an argument is refused with a typed schema refusal that reports no failure
    and is none of the five replay results.
- id: FR-016-AC-9
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A decoded falsification whose witness makes the native function evaluate the clause to false settles
    as `reproduced-with-evaluated-witness` with category `violation`; the verdict depends on the witness value,
    so a witness at which the function holds does not settle so.
- id: FR-016-AC-10
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A decoded falsification at which the native function evaluates the clause to true settles `inconclusive`
    with the proved violation and the replayed success both named.
- id: FR-041-AC-1
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: A nonempty reviewed declaration is checked against the current compiled-module census and the merge-base
    declaration. An absent compiled module, a removed claim, and an empty list each name the claim or condition
    and fail; adding a compiled claim does not erase an older one.
- id: NFR-006-AC-13
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: For a Kani-touching pull request, the author runs the full gate on its clean, stable candidate before
    opening it, and the merger runs it again on the clean, stable current candidate before merge; both actual processes
    exit successfully with all expected tests run. A copied body line alone cannot satisfy either run.
- id: NFR-006-AC-15
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: A Kani-touching pull request whose required lane did not run or failed remains unmerged and does not
    claim real-Kani verification; a written explanation does not waive the gate.
- id: FR-005
  path: spec/core/functional/FR-005-cli-conformance.md
  role: context_only
  excerpt: 'When invoked through the library or CLI, the generator shall write a complete artifact bundle

    atomically and shall retain differential and cross-backend semantic-parity results.'
- id: FR-015
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: 'When a caller selects Kani generation for complete-V1 contracts whose scalar

    oracles FR-014 generated, the code generator shall emit one bounded Kani

    obligation per precondition, postcondition, invariant and frame condition,

    each bounded by its model-domain bounds. It is the Kani

    backend''s one generator

    ([ADR-001](../../decisions/ADR-001-overlapping-generators-and-input-models.md)),

    and every item it lowers is a claim over an admitted `quire.checked-package/v2`

    package.


    For production requests after the FR-330 consumer transition,

    [FR-039](../../core/functional/FR-039-contract-package-consump'
- id: FR-017
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: context_only
  excerpt: 'When a caller runs an FR-015, FR-022/FR-014 or FR-035 harness, the code generator shall invoke the

    installed Kani backend and retain the backend''s own reported outcome as typed

    execution evidence. A caller may hand the generator several harnesses to run together

    (FR-017-AC-21 to FR-017-AC-25); the generator then forms groups, runs each

    group in one launcher process, and retains one evidence record per harness. Unless a

    statement says "batch" or "group", it describes a run of one harness.


    [FR-040](./FR-040-cancellation-producer.md) adds the one launcher-observed cancellation outcome:

    the backe'
- id: FR-041
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: context_only
  excerpt: 'When the repository claims proof coverage for a generated-harness module, its existing

    [`make kani-gate`](../non-functional/NFR-006-real-kani-lane-gating.md) SHALL accept that claim only

    after a real Kani run discharges a proposition in the module and an executed mutation inside the

    module falsifies a proof check. The gate applies the proof-stage acceptance rule of QSL ADR-011

    §2.3 to CG''s claimed modules. It makes no claim about a module absent from its declared list or

    about a semantic stage the test input did not traverse.'
- id: NFR-006
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: context_only
  excerpt: "- The repository shall define the **Kani-touching set** as the changed paths matching\n  `src/kani/**`,\
    \ `src/oracle/**`, `src/routed/**`, `src/replay/**`, `src/core/**`,\n  `src/publication/**`, `tests/it/kani_*.rs`,\
    \ `tests/it/skeleton_spine.rs`,\n  `tests/it/bounded_kani_corpus.rs`, `tests/it/scratch_crate.rs`,\n  `tests/exact_scalar_support/package.rs`,\
    \ `tests/checked_package_support/base.rs`,\n  `tests/checked_package_support/rekey.rs`,\n  `tests/state_frame_support/**`,\
    \ `schemas/kani-*.schema.json`,\n  `schemas/generated-rust-kani-*.schema.json`, `Cargo.toml` and\n  `Cargo.lock`.\n\
    - The `make kani-"
- id: FR-018
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: 'When a caller supplies an admitted `quire.checked-package/v2` package and a set

    of equality expression nodes, the code generator shall emit a deterministic Rust

    oracle crate whose functions decide each node by reconstructing the item''s

    `TypeEnvironment` and calling the Contract Runtime

    `TypeEnvironment::check_equality` and `CheckedEquality::evaluate` with a

    caller-supplied `Meter`. It never compares two values structurally, and never

    copies a resource charge or a planned pair count.


    This is the composite/structural equality slice of complete-V1 oracle

    generation. [FR-014](./FR-014-exact-scala'
- id: FR-021
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: context_only
  excerpt: 'When a caller supplies an admitted `quire.checked-package/v2` package, function

    bindings and requested `call` nodes, the generator shall classify the admitted

    expression-node graph, build a deterministic, target-neutral instruction

    schedule for each admitted body, and record one generated or typed-refused

    disposition per item. The schedule preserves V2 operation identity, operand

    order, value/result types, function binding, and source occurrence identity;

    it states no charge amount and executes no operation itself. Its body analysis

    and schedule emission use charged iterative walks and no fixe'
- id: FR-016
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: context_only
  excerpt: 'When a FR-015 harness yields a counterexample, the code generator shall decode

    the witness into typed complete-V1 values, validate each value against its

    declared domain, and replay it through native runtime execution before any

    failure is reported.'
- id: StR-001
  path: spec/core/stakeholder/StR-001-traceable-generation.md
  role: context_only
  excerpt: 'Assurance engineers require that the generator shall derive executable, property-test, proof, and

    coverage artifacts reproducibly from one validated contract without hiding unsupported semantics,

    vacuity, rejection, or assumptions.'
- id: AD-004-L-2
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "- L-2. The import graph is acyclic and follows the dependency direction above, `#[cfg(test)]`\n  modules\
    \ included, and no file imports an item through the crate root. Test: a layout test that\n  reads `use crate::`\
    \ lines and flags a bare `crate::<Item>` path in code (inline types, calls;\n  not string literals or comments,\
    \ so intra-doc links are rewritten by step 2g-0 but not checked). It lands at the end of step 2, after the edge-removal\
    \ steps, so it can\n  pass when it lands."
- id: src/core/mod.rs::// Target-owned scratch allocation
  path: src/core/mod.rs
  role: examined
  excerpt: '// Target-owned scratch allocation shared by subsystem unit tests.

    #[cfg(test)]

    pub(crate) mod test_support;'
- id: src/core/test_support.rs::pub(crate) fn discover_scratch
  path: src/core/test_support.rs
  role: examined
  excerpt: "pub(crate) fn discover_scratch(name: &str) -> PathBuf {\n    static SWEEP: std::sync::Once = std::sync::Once::new();\n\
    \    static CALLS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);\n    let executable\
    \ = env::current_exe().unwrap();\n    let target = executable\n        .ancestors()\n        .nth(3)\n     \
    \   .expect(\"the test executable sits under a target directory\");\n    let base = target.join(\"tmp\").join(\"\
    quire-kani-discover\");\n    fs::create_dir_all(&base).unwrap();\n    SWEEP.call_once(|| {\n        for entry\
    \ in fs::read_dir(&base).unwrap().flatten() {"
- id: src/kani/generate/render.rs::let directory = crate::core::test_support::discover_scratch("render-determinism");
  path: src/kani/generate/render.rs
  role: examined
  excerpt: "        let directory = crate::core::test_support::discover_scratch(\"render-determinism\");\n     \
    \   let path = directory.join(\"output.rs\");\n        let status = std::process::Command::new(std::env::current_exe().unwrap())\n\
    \            .args([\n                \"--exact\",\n                \"kani::generate::render::tests::rendering_is_identical_in_another_process\"\
    ,\n                \"--test-threads=1\",\n            ])\n            .env(OUTPUT_ENV, &path)\n            .status()\n\
    \            .unwrap();\n        assert!(status.success());"
- id: 'src/kani/run/execute.rs::        core::test_support::discover_scratch,'
  path: src/kani/run/execute.rs
  role: examined
  excerpt: "        core::test_support::discover_scratch,\n        kani::{\n            identity::StateFrameProperty,\n\
    \            test_support::{\n                report, state_frame_harness, write_launcher, COVER_NO, COVER_OK,\
    \ PASSED,\n            },\n        },\n    };\n\n    /// A precondition harness asserts nothing; its only property\
    \ is its cover. The same report\n    /// that is a vacuous proof for any other harness therefore decides by\
    \ the cover alone for a\n    /// precondition harness, whose satisfied cover is its one successful check."
- id: src/kani/run/launch.rs::use crate::core::test_support::discover_scratch;
  path: src/kani/run/launch.rs
  role: examined
  excerpt: "    use crate::core::test_support::discover_scratch;\n\n    #[trace(\"FR-028-AC-2\")]\n    #[test]\n\
    \    fn zero_wall_ceiling_cannot_accept_an_already_exited_launcher() {\n        let deadline = Instant::now();\n\
    \        let mut child = Command::new(\"sh\")\n            .args([\"-c\", \"exit 0\"])\n            .process_group(0)\n\
    \            .spawn()\n            .unwrap();\n        let pid = Pid::from_raw(i32::try_from(child.id()).unwrap()).unwrap();"
- id: src/kani/run/memory.rs::let root = crate::core::test_support::discover_scratch("memory-signed-group");
  path: src/kani/run/memory.rs
  role: examined
  excerpt: "        let root = crate::core::test_support::discover_scratch(\"memory-signed-group\");\n        let\
    \ directory = root.join(\"42\");\n        fs::create_dir_all(directory.join(\"task/42\")).unwrap();\n      \
    \  fs::write(directory.join(\"task/42/children\"), []).unwrap();\n        let mut fields = [\"0\"; 22];\n  \
    \      fields[0] = \"S\";\n        fields[1] = \"1\";\n        fields[2] = \"42\";\n        fields[19] = \"\
    200\";\n        fields[20] = \"1048576\";\n        fields[21] = \"16\";\n        let stat = |fields: &[&str]|\
    \ format!(\"42 (owned) {}\", fields.join(\" \"));"
- id: src/kani/run/namespace.rs::use crate::core::test_support::discover_scratch;
  path: src/kani/run/namespace.rs
  role: examined
  excerpt: "    use crate::core::test_support::discover_scratch;\n    use crate::kani::{\n        identity::ProofCeilings,\n\
    \        run::{launch::run_bounded_launcher, owners::LaunchOwners},\n    };\n    use ix_trace_rs::trace;\n \
    \   use std::os::unix::process::CommandExt;\n    use std::process::Stdio;\n\n    fn ready(handle: &OwnedFd)\
    \ -> bool {\n        let mut fds = [PollFd::new(handle, PollFlags::IN)];\n        poll("
- id: src/kani/run/report_file.rs::use crate::core::test_support::discover_scratch;
  path: src/kani/run/report_file.rs
  role: examined
  excerpt: "    use crate::core::test_support::discover_scratch;\n    use ix_trace_rs::trace;\n\n    /// The exported\
    \ report is read bounded: a file over the limit is refused, and a report that\n    /// was never written is\
    \ `None`.\n    #[trace(\"TC-027\")]\n    #[test]\n    fn tc_027_the_report_is_read_bounded_and_refused_not_truncated()\
    \ {\n        let directory = discover_scratch(\"report-bound\");\n        let path = directory.join(\"report.json\"\
    );\n        assert_eq!(read_report(&path), Ok(None));\n        fs::write(&path, vec![b' '; REPORT_LIMIT]).unwrap();"
- id: src/kani/run/scalar.rs::fn tc_027_changed_staged
  path: src/kani/run/scalar.rs
  role: examined
  excerpt: "    fn tc_027_changed_staged_bytes_and_unexpected_files_refuse_identity() {\n        let root = crate::core::test_support::discover_scratch(\"\
    scalar-snapshot-control\");\n        fs::write(root.join(\"source.rs\"), b\"original\").unwrap();\n        let\
    \ expected = vec![(\"source.rs\".to_owned(), b\"original\".to_vec())];\n        assert!(ensure_snapshot_matches(&root,\
    \ &expected).is_ok());\n        fs::write(root.join(\"source.rs\"), b\"changed\").unwrap();\n        assert!(matches!(\n\
    \            ensure_snapshot_matches(&root, &expected),\n            Err(KaniScalarNativeError::SnapshotChanged)\n\
    \        ));\n        fs"
- id: src/kani/run/tool.rs::use crate::core::test_support::discover_scratch;
  path: src/kani/run/tool.rs
  role: examined
  excerpt: "    use crate::core::test_support::discover_scratch;\n\n    /// `discover_from` is a pure function of\
    \ its arguments, so every `HOME`/`CARGO_HOME`/`PATH`\n    /// combination is exercised directly here with no\
    \ process environment mutation.\n    #[trace(\"FR-017-AC-2\", \"TC-027\")]\n    #[test]\n    fn tc_027_the_launcher_resolves_through_cargo_home_then_path()\
    \ {\n        let directory = discover_scratch(\"launcher\");\n        let cargo_home = directory.join(\"cargo-home\"\
    );\n        let on_path = directory.join(\"on-path\");\n        fs::create_dir_all(cargo_home.join(\"bin\")).unwrap();\n\
    \        fs::create_dir_all"
- id: src/kani/test_support.rs:://! Test helpers shared
  path: src/kani/test_support.rs
  role: examined
  excerpt: "//! Test helpers shared by the tests of more than one `kani/` file.\n\nuse std::fs;\n\nuse crate::{\n\
    \    core::{\n        artifact::Artifact,\n        identity::{HarnessSymbol, ModuleSymbol},\n    },\n    kani::{\n\
    \        abi::KaniSolver,\n        identity::{StateFrameHarness, StateFrameIdentity, StateFrameProperty, StateFrameScope},"
- id: src/oracle/equality/mod.rs::let scratch = crate::core::test_support::discover_scratch
  path: src/oracle/equality/mod.rs
  role: examined
  excerpt: "        let scratch = crate::core::test_support::discover_scratch(\"oracle-equality-deep\");\n     \
    \   std::fs::create_dir_all(scratch.join(\"src\")).expect(\"scratch source directory\");\n        std::fs::create_dir_all(scratch.join(\"\
    tests\")).expect(\"scratch test directory\");\n        std::fs::write(\n            scratch.join(\"Cargo.toml\"\
    ),\n            oracle_crate_manifest(COMPOSITE_EQUALITY_CRATE_NAME),\n        )\n        .expect(\"scratch\
    \ manifest\");\n        std::fs::copy(\n            concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/Cargo.lock\"),\n\
    \            scratch.join(\"Cargo.lock\"),\n        )"
- id: src/oracle/function/schedule.rs::crate::core::test_support::discover_scratch
  path: src/oracle/function/schedule.rs
  role: examined
  excerpt: "                    crate::core::test_support::discover_scratch(\"oracle-function-schedule\");\n   \
    \             let source_directory = directory.join(\"src\");\n                std::fs::create_dir_all(&source_directory).expect(\"\
    create source directory\");\n                std::fs::write(source_directory.join(\"lib.rs\"), source)\n   \
    \                 .expect(\"write schedule crate root\");\n                for module in modules {\n       \
    \             let path = directory.join(&module.path);\n                    std::fs::write(path, module.contents).expect(\"\
    write schedule module\");\n                }\n           "
- id: src/publication/publish.rs::fn temporary(
  path: src/publication/publish.rs
  role: examined
  excerpt: "    fn temporary(name: &str) -> PathBuf {\n        crate::core::test_support::discover_scratch(&format!(\"\
    publication-{name}\"))\n    }\n\n    fn generated(path: &str, contents: &str) -> Artifact {\n        Artifact::new(path,\
    \ contents)\n    }\n\n    fn bundle(version: &str) -> ArtifactBundle {\n        ArtifactBundle::new(vec![\n\
    \            generated(\"src/generated.rs\", version),\n            generated(\"src/generated/nested.rs\", &format!(\"\
    nested {version}\")),"
- id: tests/it/kani_batching.rs::struct Lane
  path: tests/it/kani_batching.rs
  role: examined
  excerpt: "struct Lane {\n    _scratch: TemporaryDirectory,\n    crate_directory: PathBuf,\n    installation: KaniInstallation,\n\
    \    counts: PathBuf,\n    cancel: quire_exact::Cancel,\n}\n\nimpl Lane {\n    fn new(label: &str, harnesses:\
    \ &[StateFrameHarness]) -> Self {\n        let real = KaniInstallation::discover().expect(\"cargo-kani is installed\"\
    );\n        let scratch = TemporaryDirectory::new(&format!(\"quire-kani-batching-{label}\"));"
- id: tests/it/kani_generation_claims.rs::fn base_claims_allow
  path: tests/it/kani_generation_claims.rs
  role: examined
  excerpt: "fn base_claims_allow_proven_absence_but_refuse_failed_object_read() {\n    let scratch = crate::scratch_crate::TemporaryDirectory::new(\"\
    quire-kani-claims-base\");\n    let directory = &scratch.path;\n    let git = |args: &[&str]| {\n        let\
    \ output = Command::new(\"git\")\n            .arg(\"-C\")\n            .arg(directory)\n            .args(args)\n\
    \            .output()\n            .expect(\"scratch Git must launch\");\n        assert!(output.status.success(),\
    \ \"{}\", output_text(&output));\n        String::from_utf8(output.stdout).expect(\"UTF-8 Git output\")"
- id: tests/it/kani_obligations.rs::pub(crate) fn write_crate
  path: tests/it/kani_obligations.rs
  role: examined
  excerpt: "pub(crate) fn write_crate(harness: &KaniObligationHarness, subject: &str) -> TemporaryDirectory {\n\
    \    let scratch = TemporaryDirectory::new(\"quire-kani-obligations-crate\");\n    let directory = &scratch.path;\n\
    \    fs::write(\n        directory.join(\"src/lib.rs\"),\n        format!(\n            \"//! Generated obligation\
    \ check crate.\\n\\n{}\\n{subject}\",\n            harness.rust.contents\n        ),\n    )\n    .unwrap();\n\
    \    write_manifest("
- id: tests/it/kani_obligations_symbolic_domains.rs::fn write_collection_scratch
  path: tests/it/kani_obligations_symbolic_domains.rs
  role: examined
  excerpt: "fn write_collection_scratch(harness: &KaniCollectionObligationHarness) -> TemporaryDirectory {\n   \
    \ let scratch = TemporaryDirectory::new(\"quire-kani-symbolic-domains-collection\");\n    let directory = &scratch.path;\n\
    \    fs::write(directory.join(\"src/lib.rs\"), &harness.rust.contents)\n        .expect(\"write the generated\
    \ artifact byte for byte\");\n    write_manifest(\n        directory,\n        &format!(\n            \"[package]\\\
    nname = \\\"generated-kani-collection\\\"\\nversion = \\\"0.0.0\\\"\\nedition = \\\"2021\\\"\\npublish = false\\\
    n\\n[dependencies]\\n{}\\n\\n[workspace]\\n\",\n            runtime_dependency(&[\"exa"
- id: tests/it/kani_render_private.rs::use crate::core::test_support::discover_scratch
  path: tests/it/kani_render_private.rs
  role: examined
  excerpt: "use crate::core::test_support::discover_scratch;\nuse crate::kani::{\n    classify::KaniRunOutcome,\n\
    \    generate::unsatisfiable_harness_for_test,\n    run::{\n        execute::{execute_kani_obligation, KaniExecutionRequest},\n\
    \        tool::KaniInstallation,\n    },\n};\nuse ix_trace_rs::trace;\n\n#[trace(\"FR-015-AC-117\", \"TC-025\"\
    )]"
- id: tests/it/kani_scalar_native_private.rs::let control = crate::core::test_support::discover_scratch
  path: tests/it/kani_scalar_native_private.rs
  role: examined
  excerpt: "        let control = crate::core::test_support::discover_scratch(if cancelled {\n            \"scalar-cancel\"\
    \n        } else {\n            \"scalar-timeout\"\n        });\n        fs::create_dir_all(&control).unwrap();\n\
    \        let started = control.join(\"started\");\n        let launcher = control.join(\"standin\");\n     \
    \   fs::write(\n            &launcher,\n            format!(\n                \"#!/bin/sh\\nprintf started >\
    \ '{}'\\nexec /bin/sleep 30\\n\","
- id: tests/it/kani_witness_join.rs::fn write_crate(
  path: tests/it/kani_witness_join.rs
  role: examined
  excerpt: "fn write_crate(harness: &KaniObligationHarness, subject: &str) -> TemporaryDirectory {\n    let scratch\
    \ = TemporaryDirectory::new(\"quire-kani-witness-join-crate\");\n    let directory = &scratch.path;\n    fs::write(\n\
    \        directory.join(\"src/lib.rs\"),\n        format!(\n            \"//! Generated obligation check crate.\\\
    n\\n{}\\n{subject}\",\n            harness.rust.contents\n        ),\n    )\n    .unwrap();\n    write_manifest("
- id: tests/it/layout.rs::fn l_2_every_import
  path: tests/it/layout.rs
  role: examined
  excerpt: "fn l_2_every_import_follows_the_dependency_direction() {\n    let files = source_files();\n    let modules:\
    \ BTreeSet<Vec<String>> = files.keys().map(|file| module_of(file)).collect();\n    let mut violations = Vec::new();\n\
    \    for edge in edges(&files) {\n        let from = module_of(&edge.file);\n        let (Some(directory), Some(target))\
    \ = (from.first(), edge.path.first()) else {\n            continue;\n        };\n        let reach = may_import(directory);\n\
    \        if directory != target && !reach.contains(&target.as_str()) {\n            violations.push(format!("
- id: tests/it/skeleton_spine.rs::fn prove(
  path: tests/it/skeleton_spine.rs
  role: examined
  excerpt: "fn prove(harness: &KaniObligationHarness, subject: &str) -> KaniRunOutcome {\n    let cancel = quire_exact::Cancel::new();\n\
    \    let installation = KaniInstallation::discover().expect(\"cargo-kani is installed\");\n    let scratch =\
    \ write_crate(harness, subject);\n    let directory = &scratch.path;\n    let evidence = execute_kani_obligation(&KaniExecutionRequest\
    \ {\n        cancel: &cancel,\n        installation: &installation,\n        harness: harness.into(),\n    \
    \    crate_directory: directory,\n        target_directory: &PathBuf::from(env!(\"CARGO_TARGET_TMPDIR\")).join(\"\
    kani-spine\"),\n        home_direct"
- id: tests/it/scratch_crate.rs::pub(crate) struct TemporaryDirectory
  path: tests/it/scratch_crate.rs
  role: context_only
  excerpt: "pub(crate) struct TemporaryDirectory {\n    pub(crate) path: PathBuf,\n    _lock: fs::File,\n}\n\nimpl\
    \ TemporaryDirectory {\n    pub(crate) fn new(prefix: &str) -> Self {\n        Self::in_directory(Path::new(env!(\"\
    CARGO_TARGET_TMPDIR\")), prefix)\n    }\n\n    fn in_directory(base: &Path, prefix: &str) -> Self {\n      \
    \  fs::create_dir_all(base).unwrap();"
- id: src/lib.rs::library-path-modules
  path: src/lib.rs
  role: context_only
  excerpt: '// Private integration control needs internal generation and execution in the library test binary.

    #[cfg(test)]

    #[path = "../tests/it/kani_render_private.rs"]

    mod kani_render_private;


    #[cfg(test)]

    #[path = "../tests/it/kani_scalar_native_private.rs"]

    mod kani_scalar_native_private;


    #[cfg(all(test, target_os = "linux"))]

    #[path = "../tests/it/kani_terminal_private.rs"]

    mod kani_terminal_private;


    // The leaf directory: artifact, diagnostic, identity, profile and source-map.

    // `core` here is this crate''s own layout module (the AD names the directory `core/`). In this

    // file it shadows the ex'
- id: AD-004::target-tree-core-test-support
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "    test_support.rs           `#[cfg(test)]` only: target-owned scratch directories shared across\n\
    \                              subsystem unit tests; imports only std"
- id: AD-004::module-map-core-test-support
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '| Shared unit-test scratch allocator (`discover_scratch`) | core | `core/test_support.rs` | `#[cfg(test)]`
    only; one target/dead-PID allocator shared across subsystems |'
- id: AD-004::D-7-Kani-helper-list
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "  `pub(crate)`: `state_frame_harness`, `report`, `PASSED`, `COVER_OK` and `COVER_NO`.\n  It imports\
    \ `identity` and `abi` and no generator. The subsystem-independent"
- id: AD-004::D-7-core-allocator-placement
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "  `discover_scratch` allocator lives in `core/test_support.rs`, declared `#[cfg(test)]` in\n  `core/mod.rs`;\
    \ it imports only `std` and is called directly by tests across subsystems."
- id: AD-004::item-map-Kani-five
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '| Tests: `state_frame_harness`, `report`, `PASSED`, `COVER_OK`, `COVER_NO` | 5 | `kani/test_support.rs`
    | pub(crate) | shared by the tests of several Kani files (D-7) |'
- id: AD-004::item-map-core-one
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '| Tests: `discover_scratch` | 1 | `core/test_support.rs` | pub(crate), cfg(test) | target/dead-PID scratch
    allocator shared across subsystems (D-7) |'
- id: library-path-context::tests/checked_package_support/rekey.rs
  path: tests/checked_package_support/rekey.rs
  role: context_only
  excerpt: "//! Test-fixture key repair driven by Contract IR's public candidate API and reader refusals.\n//!\n\
    //! A package builder retains its readable source IDs. This module repairs a copy of its wire,\n//! so building\
    \ another fixture cannot change IDs observed by a parallel test. The reader remains\n//! the final authority\
    \ for admission.\n\nuse std::collections::{BTreeMap, BTreeSet};\n\nuse quire_canonical::{sha256, Limits};\n\
    use quire_contract_model::{\n    derive_application_node_id, derive_structural_candidates, ApplicationIdentityContext,\n\
    \    ApplicationIdentityError, ApplicationNodeDraft, CheckedNodeId, "
- id: library-path-context::tests/common/proof_ceilings.rs
  path: tests/common/proof_ceilings.rs
  role: context_only
  excerpt: "//! Resource ceilings for test fixtures; never production defaults.\n\nuse super::ProofCeilings;\nuse\
    \ std::{num::NonZeroU64, time::Duration};\n\n/// The explicit successful-run fixture budget.\npub fn proof_ceilings()\
    \ -> ProofCeilings {\n    proof_ceilings_with_wall_clock(Duration::from_secs(600))\n}\n\n/// The same fixture\
    \ memory budget with the test's explicit wall allowance.\npub fn proof_ceilings_with_wall_clock(wall_clock:\
    \ Duration) -> ProofCeilings {"
- id: library-path-context::tests/exact_scalar_support/package.rs
  path: tests/exact_scalar_support/package.rs
  role: context_only
  excerpt: '//! Admitted CheckedPackage V2 fixtures for exact scalar generation.

    //!

    //! The base (`tests/checked_package_support/base.rs`) supplies an enum declaration,

    //! one of its members, a dimension and a declared unit under their nominal

    //! keys. Scalar types, values and expressions are appended under readable

    //! zero-padded keys, and the package identity is re-derived from the

    //! finished graph.

    //!

    //! Bounds are `bounded_domain` nodes keyed by the digest of their content and

    //! `foreign` (see `Bound::key`/`Bound::foreign`), and listed in the

    //! dependencies of each expression that uses the'
- id: library-path-context::tests/it/kani_scalar_native_digest.rs
  path: tests/it/kani_scalar_native_digest.rs
  role: context_only
  excerpt: "//! Independent JCS/SHA-256 oracle for the scalar proof-content string-only shape.\npub(crate) fn independent_digest(entries:\
    \ &[(String, Vec<u8>)]) -> [u8; 32] {\n    use sha2::{Digest, Sha256};\n    let artifacts = entries\n      \
    \  .iter()\n        .map(|(path, bytes)| {\n            format!(\n                \"{{\\\"contents\\\":{},\\\
    \"path\\\":{}}}\",\n                serde_json::to_string(std::str::from_utf8(bytes).unwrap()).unwrap(),\n \
    \               serde_json::to_string(path).unwrap()\n            )\n        })"
- id: library-path-context::tests/it/kani_scalar_native_fixture.rs
  path: tests/it/kani_scalar_native_fixture.rs
  role: context_only
  excerpt: "//! Shared admitted scalar routing fixture for the integration and private library controls.\nuse super::{package,\
    \ proof_ceilings_with_wall_clock};\nuse package::Bound;\nuse quire_contract_codegen::{\n    generate_routed,\
    \ routed_contract_lowering_profile, Artifact, BackendDescriptor, Candidate,\n    ContractDriverResultSelection,\
    \ ContractRequestPosition, GenerationContexts,\n    KaniGenerationContext, KaniScalarObligationHarness, KindOutput,\
    \ ProviderOrigin,\n    RoutedGenerationItem,\n};\nuse quire_contract_model::{\n    CheckedPackageReadLimits,\
    \ ContractDependencies, ContractPackageReadResult,\n    Cont"
- id: library-path-context::tests/it/kani_terminal_private.rs
  path: tests/it/kani_terminal_private.rs
  role: context_only
  excerpt: "//! Private orchestration: reuse bounded report stand-ins without reversing kani layer imports.\nuse\
    \ crate::kani::{\n    classify::{KaniInconclusiveReason, KaniRunOutcome},\n    run::execute::batch_tests::no_verdict_diagnostics::failure_cause,\n\
    \    terminal::run_terminal_value,\n};\nuse ix_trace_rs::trace;\nuse qsl_replay::{Certification, IncompleteCause,\
    \ ProofBasis, TerminalValue};\n\n#[trace(\n    \"FR-017-AC-37\",\n    \"FR-029-AC-32\","
- id: library-path-context::tests/state_frame_support/model.rs
  path: tests/state_frame_support/model.rs
  role: context_only
  excerpt: '//! The one source of the state-frame fixture''s model facts: the object''s integer fields with the

    //! range each declares, and the fields its operation''s frame grants. The hand-built checked

    //! package and the QSL twin''s domain package are both built from it.


    /// Each field of the operation''s state and its inclusive range.

    pub const FIELDS: [(&str, (i64, i64)); 2] = [("balance", (0, 1000)), ("audit", (0, 1000))];


    /// The fields the operation''s frame modifies.

    pub const GRANTED: [&str; 1] = ["balance"];'
- id: library-path-context::tests/state_frame_support/native_twin.rs
  path: tests/state_frame_support/native_twin.rs
  role: context_only
  excerpt: '//! The QSL twin of the state-frame fixture: the `test/bank` domain package, the native unit

    //! whose `post` clause and operation frame produce the checked package used for harnesses, and

    //! the invocation documents of one concrete run.

    //!

    //! The twin''s fields, their ranges and the fields its frame grants come from `model`, the same

    //! source the Rust fixture''s checked package is built from. The rest is tied to the Rust side only

    //! by names: the model `Bank`, its object `Account` and its operation `deposit`. The request and

    //! envelope that put a run before QSL are built by the crate u'
dispositions:
- fnd: FND-001
  outcome: fixed
  after_excerpt: crate::core::test_support::discover_scratch("oracle-equality-deep");
  reason: Single unchanged allocator/SWEEP/CALLS relocated to cfg(test) core::test_support; all callers including
    library-included tests import core directly; unchanged actual L-2 and acyclicity controls pass on exact final
    fix head.
  checked_head: 4e299c590644a2769b28b7015a1611111125e02f
  additional_after_excerpts:
  - path: src/oracle/function/schedule.rs
    excerpt: crate::core::test_support::discover_scratch("oracle-function-schedule");
  - path: src/publication/publish.rs
    excerpt: crate::core::test_support::discover_scratch(&format!("publication-{name}"))
  - path: src/core/mod.rs
    excerpt: '// Target-owned scratch allocation shared by subsystem unit tests.

      #[cfg(test)]

      pub(crate) mod test_support;'
```
