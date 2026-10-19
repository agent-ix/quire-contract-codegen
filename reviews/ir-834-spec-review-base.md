---
id: SR-12972
title: IR-834 base spec review of AD-004 test allocator placement correction
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-codegen; spec/assurance/AD-004-cg-crate-layout.md placement edits and source context;
  IR-834 PR447
review_set: base
relationships:
- target: ix://agent-ix/quire-contract-codegen/AD-004
  type: references
---

## Summary

Ticket: IR-834. Base spec review of the actual AD-004 placement/documentation changes in PR447 fix round 1, checked at clean 4e299c590644a2769b28b7015a1611111125e02f. The changed target tree, module-to-subsystem map, D-7 description and item table agree with one unchanged std-only allocator relocated under cfg(test) core/test_support.rs. No new static document finding was confirmed. This is the completed external reviewer artifact for current source; private posting and latest custody follow validation.

## Verdict

**PASS (static changed-document base review only).** Existing dependency direction, cfg(test) rule and L-2 are unchanged; the documented relocation restores those boundaries. Actual final-head L-2 direction and graph-acyclic checks pass among six layout controls. SR-12970 records its own append-only fixed disposition. Root final aggregate/full actual Kani and latest artifact custody remain required; this document verdict is not whole-repository assurance PASS or permission to merge.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Review set: base. No applicable AssuranceProfile or required review_selection exists under the repository spec tree. Full method applicability is recorded below. Parent expressly authorized correcting these existing placement views; no new requirement, tracking ledger, relocation machinery, digest/pin or policy exception was requested or introduced. No inherited debt is duplicated as a NEW finding.

Changed prose and clean parts actually examined:

| Existing AD-004 view | Assessment |
| --- | --- |
| Target directory tree | core/test_support.rs is marked cfg(test), shared across subsystem unit tests and std-only. Actual core/mod.rs declaration and helper imports agree. |
| Module-to-subsystem map | Existing discover_scratch is placed in core at core/test_support.rs. Exactly one definition remains; no compatibility alias or second implementation. |
| D-7 helper description | Kani-specific state_frame_harness/report/PASSED/COVER_OK/COVER_NO remain in kani/test_support.rs; cross-subsystem discover_scratch is documented separately in core. Existing core direction permits all callers. |
| Original item-count table | Original six listed helpers are represented as five Kani helpers plus one core helper. Total and item identities are preserved. This is source-placement documentation, not a new runtime API. |
| Existing dependency/L-2 context | Oracle/publication still depend only on core; cfg(test) remains subject to the same direction. No rule, parser, edge walker or allowlist is weakened. tests/it/layout.rs only gains the new real path in its module-file inventory. |

Base checklist: no new/edited US, FR, NFR, StR, acceptance-criterion, constraint or option ID, no new artifact ID in AD-004, and no duplicate ID created. Its frontmatter and relationships are byte-unchanged. New source-path references resolve to the actual helper/declaration. Existing D-7 references still name the same section and helper role. No new external link, revision, version pin, hash or artifact inventory mechanism is added. New source-path descriptions have one responsibility and explicit visibility; no new error, state-transition, input-boundary or option-permutation semantics require invented criteria. The repository-wide six coverage rules remain the whole-gap method's territory: prior 853-criterion status debt is not discharged by this documentation correction.

Read-only source consistency inspection independently confirms allocator doc/body plus local SWEEP/CALLS state are byte-identical to the prior Kani helper, with one definition in core. All caller changes are direct path/import updates, including the two execute.rs test import trees; cancellation and production bodies remain untouched. The three original reported callers now name core; all supporting source excerpts below are static context; actual execution is established separately by the inspected final-head author logs. Source/helper behavior and full PR correctness are the separate code/Rust and gap/disposition lanes.

## Method Applicability

spec-review/base is applicable because the fix changes architecture documentation under spec/. EARS does not apply without requirement-statement edits; integrity has no artifact/criterion/relationship restructuring to assess; object review has no domain object change; dependency analysis has no new relationship edge/order. Failure-domain, evidence, risk-complexity and scope-boundary have no new semantics, verification choices, dependencies or boundary policy. Criterion-strength and security have no changed criterion/security contract. A separate broad architecture-evaluation is not commissioned: the particular AD-004 edge restoration is already inspected directly by code/Rust and this document consistency method. Detailed applicability reasons are in the machine record. Only SR-12972 is allocated; no padding artifacts are created.

## Validation and Limits

Quire structural validation and final lightweight assurance audit ran; exact commands/stdout/stderr/exits are retained beside this file and in round1/. No formatter, Cargo, rustc, make, test, lint, compile, Kani, CBMC, load or other heavy command was run by this reviewer. Actual author layout6 (including L-2 direction and acyclicity), scratch7, affected Kani/private100/4ignored, equality1 actual Cargo, schedule1 actual rustc, publication10 and snapshot1 pass on4e299c5; fmt/lint/lib+itcompile/docs exit0. Original static preparation missed three library-included old helper references; actual initial lint exposed them and final mechanical caller corrections precede all passing checks. Both outside-src callers are included in the final source context. The prior full actual author Kani 35/35 belongs only to 8b842a5020a3246effef19612f3e2eaf9af4d54e and is not evidence for the static fix head. Root final aggregate/full actual Kani remain pending after review/disposition/latest custody. Source and the original Findings/Verdict text of SR-12970/SR-12971 remain untouched.

## Examined Scope

Complete scope follows, including every changed placement unit and clean context. Every excerpt is a nonempty verbatim actual source slice, <=600 characters. No engine AC tag changed. Two explicitly identified local AD-004 L-2 verification bindings record actual direction/acyclic support; they do not assert a new engine target or full semantic discharge.

```yaml
repo: agent-ix/quire-contract-codegen
ticket: IR-834
pr: quire-contract-codegen#447
artifact_id: SR-12972
method: spec-review/base
analysis: base
review_set: base
model: gpt-6.1-sol
run: ce89d6a6-ed82-4ef8-b019-5e78667251c3
checked_head: 4e299c590644a2769b28b7015a1611111125e02f
prior_reviewed_head: 8b842a5020a3246effef19612f3e2eaf9af4d54e
phase: completed base document review; root final gates and latest custody pending
clean: true
document_delta_verdict: PASS
candidate_mergeability: No open independent-review finding; latest custody and root final aggregate/full actual
  Kani still required.
scope:
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
- id: AD-004::dependency-direction
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: context_only
  excerpt: 'publication --> core

    routed      --> replay, kani, strategy, oracle, core

    replay      --> kani, core

    kani        --> oracle, core

    strategy    --> oracle, core

    evidence    --> oracle, core

    oracle      --> core

    core        --> (nothing in this crate)

    lib         --> everything (re-exports only)'
- id: AD-004::cfg-test-direction
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: context_only
  excerpt: "- A `#[cfg(test)]` module obeys the same direction as the file it sits in. A test that needs a\n  generator\
    \ as a fixture lives in `tests/it/`, not in the runner's file. The back-edge in the pair"
- id: AD-004-L-2
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: context_only
  excerpt: "- L-2. The import graph is acyclic and follows the dependency direction above, `#[cfg(test)]`\n  modules\
    \ included, and no file imports an item through the crate root. Test: a layout test that\n  reads `use crate::`\
    \ lines and flags a bare `crate::<Item>` path in code (inline types, calls;\n  not string literals or comments,\
    \ so intra-doc links are rewritten by step 2g-0 but not checked). It lands at the end of step 2, after the edge-removal\
    \ steps, so it can"
- id: AD-004::boundary-behavior-exclusion
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: context_only
  excerpt: 'Out of scope: behaviour. Every requirement keeps its id and its criteria. No requirement id is

    minted here; the labels `L-1` and onward are local to this document, as the labels in the other

    ADs are. Two places record the design intent of a behaviour change that its own requirement ticket

    carries, and say so where they do: step 4c (a V2 input for the contract families) and step 5 (the

    terminal map). They add no behaviour claim of their own. Also out of scope: IR''s and RT''s own'
- id: core::test_support-declaration
  path: src/core/mod.rs
  role: context_only
  excerpt: '// Target-owned scratch allocation shared by subsystem unit tests.

    #[cfg(test)]

    pub(crate) mod test_support;'
- id: core::test_support-std-imports
  path: src/core/test_support.rs
  role: context_only
  excerpt: "use std::{\n    env, fs,\n    path::{Path, PathBuf},\n};"
- id: core::test_support-allocator-state
  path: src/core/test_support.rs
  role: context_only
  excerpt: "pub(crate) fn discover_scratch(name: &str) -> PathBuf {\n    static SWEEP: std::sync::Once = std::sync::Once::new();\n\
    \    static CALLS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);\n    let executable\
    \ = env::current_exe().unwrap();\n    let target = executable\n        .ancestors()\n        .nth(3)\n     \
    \   .expect(\"the test executable sits under a target directory\");\n    let base = target.join(\"tmp\").join(\"\
    quire-kani-discover\");\n    fs::create_dir_all(&base).unwrap();"
- id: core::test_support-dead-PID-sweep
  path: src/core/test_support.rs
  role: context_only
  excerpt: "    SWEEP.call_once(|| {\n        for entry in fs::read_dir(&base).unwrap().flatten() {\n          \
    \  let owner = entry\n                .file_name()\n                .to_str()\n                .and_then(|file_name|\
    \ file_name.split('-').next_back())\n                .and_then(|pid| pid.parse::<u32>().ok());\n           \
    \ if owner.is_some_and(|pid| !Path::new(\"/proc\").join(pid.to_string()).exists()) {\n                let _\
    \ = fs::remove_dir_all(entry.path());\n            }\n        }\n    });"
- id: core::test_support-call-identity
  path: src/core/test_support.rs
  role: context_only
  excerpt: "    let call = CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);\n    let path = base.join(format!(\"\
    {name}-{call}-{}\", std::process::id()));\n    let _ = fs::remove_dir_all(&path);\n    fs::create_dir_all(&path).unwrap();\n\
    \    path\n}"
- id: layout::module-inventory-core-test-support
  path: tests/it/layout.rs
  role: context_only
  excerpt: '    "core/test_support.rs",'
- id: layout::may-import
  path: tests/it/layout.rs
  role: context_only
  excerpt: "fn may_import(directory: &str) -> &'static [&'static str] {\n    match directory {\n        \"core\"\
    \ => &[],\n        \"oracle\" | \"publication\" => &[\"core\"],\n        \"strategy\" | \"evidence\" | \"kani\"\
    \ => &[\"oracle\", \"core\"],\n        \"replay\" => &[\"kani\", \"core\"],\n        \"routed\" => &[\"replay\"\
    , \"kani\", \"strategy\", \"oracle\", \"core\"],\n        other => panic!(\"`{other}` is not a directory of\
    \ the layout\"),\n    }\n}"
- id: layout::L-2-existing-check
  path: tests/it/layout.rs
  role: context_only
  excerpt: "fn l_2_every_import_follows_the_dependency_direction() {\n    let files = source_files();\n    let modules:\
    \ BTreeSet<Vec<String>> = files.keys().map(|file| module_of(file)).collect();\n    let mut violations = Vec::new();\n\
    \    for edge in edges(&files) {\n        let from = module_of(&edge.file);\n        let (Some(directory), Some(target))\
    \ = (from.first(), edge.path.first()) else {\n            continue;\n        };\n        let reach = may_import(directory);\n\
    \        if directory != target && !reach.contains(&target.as_str()) {\n            violations.push(format!("
- id: source-context::src/oracle/equality/mod.rs
  path: src/oracle/equality/mod.rs
  role: context_only
  excerpt: '        let scratch = crate::core::test_support::discover_scratch("oracle-equality-deep");'
- id: source-context::src/oracle/function/schedule.rs
  path: src/oracle/function/schedule.rs
  role: context_only
  excerpt: '                    crate::core::test_support::discover_scratch("oracle-function-schedule");'
- id: source-context::src/publication/publish.rs
  path: src/publication/publish.rs
  role: context_only
  excerpt: '        crate::core::test_support::discover_scratch(&format!("publication-{name}"))'
- id: source-context::src/kani/run/execute.rs
  path: src/kani/run/execute.rs
  role: context_only
  excerpt: '        core::test_support::discover_scratch,'
- id: source-context::src/kani/test_support.rs
  path: src/kani/test_support.rs
  role: context_only
  excerpt: //! Test helpers shared by the tests of more than one `kani/` file.
- id: source-context::tests/it/kani_render_private.rs
  path: tests/it/kani_render_private.rs
  role: context_only
  excerpt: use crate::core::test_support::discover_scratch;
- id: source-context::tests/it/kani_scalar_native_private.rs
  path: tests/it/kani_scalar_native_private.rs
  role: context_only
  excerpt: "        let control = crate::core::test_support::discover_scratch(if cancelled {\n            \"scalar-cancel\"\
    \n        } else {\n            \"scalar-timeout\""
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
findings: []
bindings:
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
method_applicability:
- method: spec-review/base
  applicable: true
  reason: Actual architecture documentation under spec/ changes; base checklist assesses changed placement prose,
    IDs/links and consistency with existing L-2.
  artifact: SR-12972
  review_set: base
- method: spec-integrity-analysis
  applicable: false
  reason: No requirement/artifact/section structure, ID, criterion, frontmatter, count total or relationship is
    added or changed. One existing source placement is reconciled in existing views, preserving original six items
    as five plus one.
- method: spec-ears-analysis
  applicable: false
  reason: No FR/NFR/StR requirement statement is authored or edited.
- method: spec-object-review
  applicable: false
  reason: No domain object/entity model or cross-reference identity changes; this is an existing test helper path.
- method: spec-dependency-analysis
  applicable: false
  reason: No declared relationships edge or feature/enablement ordering changes. The documented dependency direction
    is unchanged.
- method: spec-failure-domain-analysis
  applicable: false
  reason: No new operating/error/identity/topology semantics; the allocator body and local state relocate unchanged.
- method: spec-evidence-analysis
  applicable: false
  reason: No obligation or verification-method catalog selection changes.
- method: spec-risk-complexity-analysis
  applicable: false
  reason: No new technical requirement, volatility or delivery dependency introduced by placement correction.
- method: spec-scope-boundary-analysis
  applicable: false
  reason: Subsystem/test-only boundaries remain identical; helper relocation restores existing permitted core direction.
- method: spec-architecture-evaluation
  applicable: false
  reason: Specific implemented dependency fix is already independently inspected by code/Rust and this changed-document
    consistency pass. No broad repository architecture evaluation requested; duplicating the same defect would add
    no independent scope.
- method: spec-criterion-strength-analysis
  applicable: false
  reason: No acceptance criterion authored or edited; no calibration service needed for path-description changes.
- method: spec-security-analysis
  applicable: false
  reason: No new security contract/production behavior or threat boundary.
original_SR12970_FND001_disposition: fixed by separate SR-12970 round1 on exact4e299c5 with actual L2/acyclic checks
execution_evidence: Actual author confined checks pass on4e299c5; reviewer ran only lightweight read-only/Quire
  tooling. Historical author35/35 belongs only to8b842a5.
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
```
