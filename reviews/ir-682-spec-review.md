---
id: SR-2980
title: "IR-682 spec-review/base review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #320 head (commit subject: Specify build-specific native workspace conformance analysis); spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-049-caller-death-ownership.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-2980: IR-682 spec-review/base review

## Summary

Ticket: IR-682. PR: quire-contract-codegen#320, spec-only, two files. The amendment adds the
FR-034 section "Supported-build proof of named native workspace", criteria FR-034-AC-41 through
FR-034-AC-50 (all PLANNED/UNRUN Analysis) and the matching TC-049 procedure.

The repository facts it cites are correct. `make build` runs `cargo build --locked --release`
(Makefile:89-90). The release profile sets `lto = "thin"` and `codegen-units = 1`
(Cargo.toml:45-47). `rust-toolchain.toml` selects channel 1.98.1, and Cargo.toml:5 sets
`rust-version = "1.98.1"`. Cargo.toml declares no `[features]` section, so there are no default
features. CI runs `cargo test` and `cargo clippy` on the default dev profile with toolchain 1.98.1
and no target or RUSTFLAGS overrides (.github/workflows/ci.yml:14,28,31,34). The amendment adds no
numeric cap, transient exclusion or guessed reserve, and no SHA, local path or conflict marker.

The method is not yet sound enough to bound the chain. Three high findings block merge. First, the
initial configuration names no executable, and it relies on profile and toolchain files that Cargo
ignores for consumers. Second, the method has no per-thread stack rule, although the caller
operations run on several threads. Third, nothing carries the proven stack bound into the runtime
`caller_run_buffers` value, and the "no runtime refusal" rule contradicts the existing
missing-named-input refusal.

## Method

Read the full diff, the existing FR-034 accounting text (lines 1130-1188 at head), the
fixture-build lines (526-540), the Dependencies section and TC-049's slice and Expected Results
tables. Verified each build fact against Makefile, Cargo.toml, rust-toolchain.toml and
.github/workflows/ci.yml. Checked consistency with the pending IR-639 code branch
`code/ir-639-outer-namespace-owner`: src/kani/run/caller_bootstrap.rs (named-buffer reservations),
launch.rs (capture threads, `CAPTURE_STACK_BYTES`) and spawner.rs (`SPAWNER_STACK_BYTES`). Also
checked Linear IR-655 (fixture slice staging). Every new paragraph and every criterion from
FR-034-AC-41 to FR-034-AC-50 was examined.

## Verdict

**NOT MERGE-READY.** The build facts are accurate, the criteria are honestly PLANNED/UNRUN, and no
numeric cap or exclusion is added. Three high findings must be fixed before merge: the initial
configuration names no executable, the method ignores concurrent thread stacks, and the runtime
stack bound is not carried and contradicts the existing refusal rule. Five medium findings are
under-specified method steps.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | The initial evidence configuration names no executable and uses build settings that do not reach a real consumer. `make build` builds only the library (no `[[bin]]`, no src/bin; Makefile:89-90, Cargo.toml:12-14), so it produces no final linked executable for FR-034-AC-42 to disassemble. Cargo also ignores `[profile.release]` and rust-toolchain.toml of a dependency: a consumer's executable uses the consumer's own workspace profile and toolchain. The x86_64 proof therefore describes no delivered consumer build, and line 1212 ("a consumer overriding this package's profiles") misstates Cargo, because consumers never inherit those profiles. | spec/kani/functional/FR-034-caller-death-ownership.md:1202-1223; FR-034-AC-41; FR-034-AC-42; Makefile:89-90; Cargo.toml:12-14,45-47; rust-toolchain.toml:2 | wrong-requirement |
| FND-002 | high | The method has no rule for concurrent thread stacks. The named caller control and capture operations run on several threads in the pending IR-639 code: capture threads with `CAPTURE_STACK_BYTES` (launch.rs:141,664) and a spawner thread with `SPAWNER_STACK_BYTES` (spawner.rs:25,154). Lines 1228-1240 and FR-034-AC-45 take one maximum over a single call graph from the operation roots. Two outcomes follow. Thread bodies reached through std's thread-start indirect call are either summed as if nested, or left unresolved, which makes every proof UNPROVEN. Or the concurrently live per-thread maxima are not summed, which undercounts. Each thread entry needs its own root, and the bound needs the sum of the per-thread maxima of concurrently live threads. | spec/kani/functional/FR-034-caller-death-ownership.md:1228-1244; FR-034-AC-43; FR-034-AC-45 | missing-requirement |
| FND-003 | high | The proven stack bound has no runtime carrier, and the PR contradicts the existing refusal rule. Lines 1249-1250 require runtime C to include "the established maximum native-stack workspace" in `caller_run_buffers` before exposure. No mechanism carries an analysis result into the executable, and no rule says what C charges in an UNPROVEN configuration. Existing lines 1166-1168 say a missing named input "shall refuse ... never omit ... a caller cap", while new lines 1198-1199 forbid any runtime refusal. On aarch64, or on any unproven build, C must both refuse and not refuse. Embedding the value also changes the executable, which triggers re-analysis under FR-034-AC-49; that fixed point is not addressed. | spec/kani/functional/FR-034-caller-death-ownership.md:1196-1199,1249-1251,1166-1168; FR-034-AC-50; FR-034-AC-48 | wrong-requirement |
| FND-004 | medium | Root identification after LTO is undefined. With thin LTO and `codegen-units = 1`, the named Rust operations are usually inlined into consumer functions, so they have no symbol in the final executable. The text does not say how roots are located (symbols, debuginfo, a non-inlined boundary, or attributing whole host frames), so two analysts would bound different code. | spec/kani/functional/FR-034-caller-death-ownership.md:1228-1230; FR-034-AC-43 | missing-requirement |
| FND-005 | medium | Dynamically loaded runtime frames are not tied to the configuration. On x86_64-unknown-linux-gnu, glibc and the libgcc_s unwinder are shared libraries loaded at run time, so they are not in the final linked executable. The configuration identity (line 1202-1205, FR-034-AC-41) records the build environment but not the run-time glibc, libgcc_s or loader version. A proof against the build host's libc can then be claimed for a host with deeper libc or unwinder frames. "on luna" (line 1214) names a host rather than a configuration property, and what counts as an "explicit finite bound source" for an external frame is not defined. | spec/kani/functional/FR-034-caller-death-ownership.md:1202-1205,1214,1240-1242; FR-034-AC-47; FR-034-AC-41 | missing-requirement |
| FND-006 | medium | The boundary between the new charged native workspace and the existing exclusion is not drawn. Existing lines 1179-1182 exclude "caller thread/native runtime, TLS, guard/alternate-stack and allocator transient allocations beyond the named per-run controls/captures". The new text charges libc, runtime and allocator frames "when they contribute to that workspace" (1240-1242), and also says the exclusions are unchanged (1253-1254, FR-034-AC-47). Neither text says which stack bytes are "native runtime" (excluded) and which are named workspace (charged). | spec/kani/functional/FR-034-caller-death-ownership.md:1240-1242,1253-1255,1179-1182; FR-034-AC-47 | wrong-requirement |
| FND-007 | medium | "Actual allocation capacity" is ambiguous. It could mean the Rust container capacity (for example `Vec::capacity`) or the allocator's real usable chunk size including size-class rounding. The existing exclusion of "allocator transient allocations" makes the second reading debatable, so analysts would produce different sums. | spec/kani/functional/FR-034-caller-death-ownership.md:1246-1253; FR-034-AC-48 | wrong-requirement |
| FND-008 | medium | The "conformance claim" has no defined surface or producer, and the delivery gate is not stated. The text never says where a native-accounting conformance claim is recorded (rustdoc, release note, CI artifact) or who checks it, so FR-034-AC-50's "conformance claim fails" has no observable pass or fail. The text also does not say whether the x86_64 proof is required before the single IR-639 lifecycle CODE PR merges, or which IR-655 slice it belongs to. | spec/kani/functional/FR-034-caller-death-ownership.md:1192-1199,1222-1223; FR-034-AC-50; FR-034-AC-49 | missing-requirement |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-009 | low | The heap rule (FR-034:1268-1270) says allocator usable chunk size, rounding and overhead "remain within the existing incidental allocator exclusions". But the existing exclusion (FR-034:1179-1185) covers "allocator transient allocations beyond the named per-run controls/captures", and it forbids reclassifying an allocated named quantity as incidental. Chunk overhead of a named allocation lasts as long as that allocation and is not beyond it. The new sentence therefore extends the exclusion while claiming it is unchanged, and a reviewer reading lines 1179-1185 could reject a sum that leaves out that overhead. | spec/kani/functional/FR-034-caller-death-ownership.md:1268-1270,1179-1185; FR-034-AC-48 | wrong-requirement |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed in commit "Clarify supported-build native accounting proof and charge ownership" | FR-034:1200-1217 make the final consumer executable the analysis input, state that `make build` is library-only and that dependency profiles and toolchain files do not select consumer settings. |
| FND-002 | fixed in commit "Clarify supported-build native accounting proof and charge ownership" | FR-034:1244-1252 and AC-45 require thread-entry roots including indirect starts, per-thread maxima summed over concurrently live instances, and UNPROVEN for unknown thread counts or lifetimes. |
| FND-003 | fixed in commit "Clarify supported-build native accounting proof and charge ownership" | FR-034:1274-1285 define an independently declared private charge inside `caller_run_buffers`, proof that the bound fits the declaration, no write-back, and a missing actual charge keeping the existing refusal (lines 1166-1168) as distinct from a missing proof. |
| FND-004 | fixed in commit "Clarify supported-build native accounting proof and charge ownership" | FR-034:1237-1242 map roots through symbols and debug/inlining information, cover the whole host frame when inlined, and make an unresolved mapping UNPROVEN. |
| FND-005 | fixed in commit "Clarify supported-build native accounting proof and charge ownership" | FR-034:1208,1232-1233,1256-1260 record loaded runtime libraries, disassemble them, and require bound sources that match the loaded ABI and configuration. "luna" now names only where the evidence input is produced. |
| FND-006 | fixed in commit "Clarify supported-build native accounting proof and charge ownership" | FR-034:1255-1263 charge every frame reached while carrying named state and keep unrelated incidental runtime, TLS and guard exclusions, with no relabelling. |
| FND-007 | fixed in commit "Clarify supported-build native accounting proof and charge ownership" | FR-034:1265-1270 define capacity as the returned Rust capacity times element size plus metadata, and exclude allocator chunk size. See new FND-009 for the residual wording tension. |
| FND-008 | fixed in commit "Clarify supported-build native accounting proof and charge ownership" | FR-034:1223-1230 name the CG CODE author as receipt producer and the independent CODE reviewer as checker, gate the IR-639 merge with a conformance claim, and require a per-criterion disposition. |
