---
id: REV-017
title: "Complete bound coverage and native result identity design"
type: SpecReview
analysis: gap-analysis
scope: "issue #5 next bounded slice atop published PR #27"
review_set: subset
---

# Complete bound coverage and native result identity design

## Summary

**PROPOSAL; coordinator approval required before implementation.** Base is published
`cd345e1dc0199db9abeac9955fd1bfcc121cddc9`; keep its exact IR
`93674480c572c237fe87c5d509b17206664bdd62` and runtime
`e360dad8a3e0e54f9b8457ff7f3748be0f2acdb3` pins in this first slice.
No producer, schema, proof obligation, or passing coverage row is claimed by this packet.

Campaign checkpoint: the coordinator subsequently approved phase A exactly as complete
unqualified observations. The implementation follows this proposal without promoting a
runtime dependency or implementing native transport/qualification. Phase B remains a design
gate. REV-018 records the resulting local candidate and qualifications separately from this
preimplementation history.

Issue #5 requires per-requirement LLVM coverage/rejection facts. PR #23's original
specification-only body is historical: its published recovery `a953a203` implements
probes/LLVM primitives and explicitly leaves native binding and aggregate analysis open.
REV-014/015 own that disposition. PR #27 now supplies immutable complete bound generation
and full package identity in maps. SR-007 still leaves native campaign result publication
open; generated test outcomes are not the existing generation-conformance producer stream.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-24001 | high | Matching native-result digests establishes consistency, not execution authentication; keep observation explicitly unqualified until shared owner verification. | FR-004-AC-4, FR-004-AC-9 |
| FND-24002 | high | Runtime CampaignReport lacks a validated native-process transport; Display parsing or fake-verdict reconstruction would create a private accounting boundary. | FR-004-AC-6 |
| FND-24003 | high | Complete generated population is available but no aggregate joins it to independently derived typed implication identities and exact supplied bytes. | FR-004-AC-7 |
| FND-24004 | medium | Generated campaign terminal outcomes, including exhaustion and prior report accounting, are not native producer results yet. | FR-004-AC-6, SR-007 |

## Three separate boundaries

1. Codegen derives complete expected clauses/probes from public typed IR and generated
   artifacts, then computes domain coverage observations. It does not execute LLVM.
2. A separately invoked native producer records what it actually built, ran, merged, and
   exported. A result manifest is domain data, not an attestation or authentication token.
3. Quoin owns retention, digest verification, attestations, and receipts. Quire owns static
   coverage facts. The shared consuming obligation owns qualification; no local aggregate
   assurance verdict, human-sufficiency decision, evidence store, or receipt schema appears.

Hash equality establishes consistency, not that an executable ran. In particular, a
caller-supplied `verified: true`, successful subprocess exit alone, matching filenames, or
valid LLVM metadata cannot create run-qualified coverage. A sealed file alone does not
establish producer trust either; shared verification must bind the authorized producer,
candidate, record, and expected run. Replayed internally consistent old data fails the
independently supplied current run/candidate expectation, not a guessed wall-clock age.

## Proposed smallest library API: phase A

```rust
pub fn analyze_bound_coverage(
    package: &quire_contract_ir::BoundPackage,
    generated: &BoundOracleGeneration,
    inputs: BoundCoverageInputs<'_>,
) -> BoundCoverageAnalysis;

// Borrowed caller bytes, not paths that the analyzer reads or tools it executes.
pub struct BoundCoverageInputs<'a> {
    pub source_root: &'a str,
    pub artifacts: &'a [ArtifactBytes<'a>],
    pub llvm_export: Option<&'a [u8]>,
}
```

`ArtifactBytes` supplies an exact bundle-relative path and bytes, not a trusted digest.
Require the complete generated bundle inventory (including its generation bodies), compare
every supplied byte sequence to its immutable generated artifact, and recompute SHA-256.
There is no alternate public constructor for a trusted generated population. The operation
does not regenerate outputs or seal their bodies. `GeneratedBoundOracles.bound_digest`,
complete clause identities, declarations, expressions, and informational population must
match the separately supplied `BoundPackage`; type accessors, never private wire, own this.

`BoundCoverageAnalysis` is immutable, versioned domain output with a read-only API and
deterministic serialization. It contains the bound digest, ordered full ClauseRefs,
informational references, available exact-byte digests, diagnostic locations, and one
per-clause observation or explicit unavailable state. A clause observation contains its
evaluation count, ordered implication observations, and existing four-way classification.
Per-requirement buckets retain every clause and census; they never replace the population
with a majority, percentage, or maximum-count classification.

Result axes are separate:

- analysis: `complete`, `incomplete`, `invalid_input`, `unsupported`, `no_executable`;
- clause: existing `exercised`, `partially_exercised`, `vacuous`, `unexecuted`, or no
  classification with a diagnostic;
- provenance: **`unqualified` only in phase A**; no successful analysis result is named
  `passed`, `attested`, `run_qualified`, or `coverage_satisfied`.

For invalid package/artifact/census binding, emit no measured classifications. After that
global boundary succeeds, missing probe observations may retain other independently measured
clause facts but the analysis is incomplete, never a successful subset. Resource refusal
retains identities acquired before refusal and does not allocate a fake population of zeros.
Valid empty/informational-only input returns `no_executable`, with exact informational refs,
no measured clause, no publishable generation artifact, and no positive obligation result.

This deliberately omits runtime campaign import and native-result deserialization from the
first executable API. It advances complete observation without pretending those gates exist.

## Independent population and map semantics

- Traverse each bound typed expression iteratively, assigning each implication a deterministic
  consequent-emission ordinal local to its full ClauseRef: traverse the left operand, record
  the owning implication, then traverse the right operand. This is the current generator's
  order, not ordinary node pre-order. Count nested/sibling implications beneath short-circuit
  and total Boolean operators from IR, never from the supplied map or coverage files.
- The immutable generator's map must contain exactly one envelope, one evaluation probe,
  and exactly that many consequent probes in the generator's established emission order.
  Compare the supplied map bytes to that expected map; a caller cannot swap semantic regions
  while keeping a plausible count. Check package/requirement/u64 revision/clause on every row,
  path ownership, role constraints, ranges, token containment, and unique probe identities.
- Keep the existing map schema and source naming unchanged for phase A; no independent
  hand-authored source-map producer is accepted. A future cross-version producer requires a
  separate compatibility contract, not relaxation to count-only matching.
- Require one eligible normalized LLVM file per expected generated source. External
  dependency files remain permitted as the current parser defines, but an extra file under
  `src/generated/` that is not in the expected bundle is foreign and refuses binding.
- Missing segments/probes are unavailable; partial spans, gaps, non-count spans, final
  unterminated spans, duplicate files/probes, or positive consequent with zero evaluation
  retain the existing refusal semantics. No file-summary or zero-substitution fallback.
- Preserve 1 MiB source, 4096-artifact, 16 MiB per-artifact, 128 MiB bundle, 16 MiB LLVM,
  4096-file and 250000-segment bounds. Check batch sizes before allocation/partial results;
  cap analysis output at 16 MiB before serialization and refuse oversize explicitly.

## Proposed native domain result: phase B owner gate

A future `codegen.native-coverage-result/v1` is a narrowly scoped producer result, not a
generic evidence envelope. Its separate strict schema must be reviewed before code exists.
The analyzer would consume it alongside an independently supplied expected run, not use
the result to declare its own expected population. Unknown versions, duplicate JSON keys,
duplicate identities, oversized inventories, and incomplete process stages fail closed.

Required bindings are one connected manifest:

| Fact | Independent binding/check |
|---|---|
| Candidate, record, run ID | Actual shared context and expected launch identity; not copied from the imported result |
| Bound package | Exact public IR digest and complete ClauseRef population |
| Generated outputs | Complete paths, lengths, SHA-256 for Rust/maps/generation bodies; expected immutable bundle equality |
| Driver and build configuration | Exact Cargo.toml, Cargo.lock, driver/support sources, configuration and explicitly controlled environment digests; no hidden build scripts or undeclared sources in the first profile |
| Tools | Resolved executable bytes plus observed version/commit for cargo, rustc, cargo-llvm-cov, llvm-cov and llvm-profdata; qualified target and test optimization settings |
| Build/run/merge/export | Ordered real argv, cwd, controlled environment, exit or signal, and exact stdin/stdout/stderr identities; stdout text is never parsed for a passing test verdict |
| Instrumented binary | Exact Cargo-emitted selected test artifact and bytes, joined to the actually invoked path, not merely a file found in target/ |
| Profiles and export | All fresh run-specific raw profiles, exact merge input set, merged profile, export input binary set and final exact JSON digest; no old shared target/profile directory |
| Campaign facts | Runtime-owned complete validated report transport plus generated campaign terminal outcome and configuration; no codegen-private counter constructor |
| Analysis output | Exact versioned domain schema digest and output bytes retained by the shared chain, separately from source-generation attestations |

First native profile should be one isolated package, one selected native test binary,
Rust 1.94.1, x86_64-unknown-linux-gnu, test opt-level 0, cargo-llvm-cov 0.9.0 /
LLVM JSON 3.0.1. More binaries, custom runners, unsupported wrappers, source remapping,
and ambient profile reuse are unsupported until an explicit profile qualifies them.
Missing tools or killed/incomplete stages are unavailable/inconclusive with retained facts;
they never become skipped-success. Post-run source/config mutation is a failed identity
check; unchanged digests are still not tamper-proof against an actively malicious executor.

Installed cargo-llvm-cov documents `--no-report` and `report --json`; the current fixture
uses the real all-in-one `cargo +stable llvm-cov --offline --json --output-path ...`.
The detailed producer sequence must be banked and measured before promising binary/stage
provenance. In particular, do not invent Cargo message-stream flags or infer the child
binary path from a filename glob. Analysis remains a non-executing consumer in every phase.

## Runtime transport decision required

At the pinned runtime, `CampaignReport` and `CampaignCounts` are private-field types with
no serde or validated count constructor. Their Display strings are diagnostics, not a
wire contract. Replaying invented Verdict objects to reconstruct counts is prohibited.
Runtime identity contains requirement and revision, not package; a codegen-side binding
must add the enclosing full RequirementRef and compare revision to canonical u64 decimal.

Recommended owner request: a runtime-owned versioned snapshot and bounded validating decoder,
preserving accepted/rejected/failed/discarded and `failed <= accepted`, with an explicit
saturation/overflow policy. A verified transport object may then join codegen by exact
package-qualified requirement and campaign identity. Zero invocations contradict positive
oracle coverage only when that campaign is the declared sole execution source. Requirement
counts must not be duplicated once per clause or summed across unrelated runs.

SR-007's generated outcome retains attempted counts, retry/shrink behavior, floor/ceiling
policy, and `Exhausted { reason, summary, policy }`; process exit alone cannot replace it.
The producer must obtain that real generated outcome through a specified transport too.
No pre/post pairing, Boolean-to-integer strategy binding, or arbitrary clause campaign is
inferred from BoundPackage. Until these owner interfaces exist, campaign import is unavailable
and even a completely exercised native LLVM observation remains unqualified.

## Banked controls and dispatch order

1. Approve phase A API/state semantics. Bank synthetic complete-package controls, with the
   unchanged genuine generated bundle as positive counterfactual for each mutation.
2. Implement complete typed census and artifact/map join, then observations and strict domain
   output schema. Native TC-006 must generate through `generate_bound_oracles`, not collect
   manually chosen single-clause requests. Keep runtime/IR pins and all source guards intact.
3. Native controls: vacuous, no-implication positive, sibling partial, nested partial,
   unexecuted and all-exercised in one complete package; two packages with identical local
   requirement/clause names; actual missing-tool and interrupted-run outcomes when the
   producer interface exists. Every phase-A native result still says unqualified.
4. Independent tampering controls: drop one/all clauses, informational ref, evaluation or
   consequent; duplicate/substitute maps, probes, package/revision, source bytes, paths, and
   export files; provide a fully internally consistent different generated package. Assert
   exact diagnostics/no measured classifications for global binding failures.
5. Native-result gate later: mutate each identity edge separately, including valid old run
   with matching bytes but wrong expected run/candidate; foreign binary/profile; partial
   merge inputs; failed/aborted execution; missing or inconsistent campaign transport. Each
   has a healthy native counterfactual and discriminates its intended validator.
6. After runtime and producer interfaces are independently accepted, add the shared consumer
   adapter with an explicitly declared new proof input. Current ROW_RESULTS has only
   pass/fail/malformed/unavailable/not-computed/vacuous; do not coerce unsupported/inconclusive
   into a passing generation stream or silently extend the shared vocabulary. Native-run
   qualification and default adverse/unavailable denial require their own accepted mapping.

FR-004-AC-1..8 and TC-006 stay planned for full issue closure. No native proof obligation is
declared for absent code. The existing source-generation ProofAttestationV1 `passed` means
generation succeeded and never certifies this coverage operation. No Quoin lock promotion,
frontend, CLI, Kani, or automatic owner exception is part of this dispatch.

## Approval request

Approve phase A as complete **unqualified observation** with the API above, a new domain-only
analysis schema, no runtime dependency promotion, and no run-qualified constructor. Separately
assign the runtime snapshot/terminal-outcome and authorized native producer/shared verification
contracts. A request to combine these into one implementation needs those owner contracts first;
the current public types cannot honestly satisfy that combined claim.
