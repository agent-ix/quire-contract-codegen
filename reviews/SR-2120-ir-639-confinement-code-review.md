---
id: "SR-2120"
title: "IR-639 code-review review of report threat boundary and backend IPC confinement"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@b0cb2737eaea22aa1f58970931d92d4f55e3868e; spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-049-caller-death-ownership.md (unopened PR; diff against base cd9fdaa46fec2ae8f7cdc8c9bc4e1c32bce81770)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-2120: IR-639 code-review review of report threat boundary and backend IPC confinement

## Summary

Ticket: IR-639. PR: quire-contract-codegen#unopened at b0cb2737eaea22aa1f58970931d92d4f55e3868e. Reviewer model claude-opus-5-5, run dbb8a12e-b532-45a6-a5bd-451efbb27322.
Scope units examined: FR-034 (new section lines 522-574 and Dependencies lines 734-741),
FR-034-AC-35, FR-034-AC-36, FR-034-AC-37, TC-049 (Coverage rows, steps 22-24, Expected Results
rows). Context only: FR-034-AC-4, FR-034 recipe lines 61, 90, 108-109, 157, 280-284. 2 finding(s).

## Method

Read the full base..head diff (+110/-0 lines; git diff --numstat shows 66 and 44 insertions and zero deletions, so FR-034-AC-1..34 and TC-049 steps 1..21 are byte-unchanged at head). Grounded the cited CG facts in source at the frozen head: src/kani/run/execute.rs start (BoundedLaunchError::Unavailable -> KaniExecutionRefusal::MemoryMechanismUnavailable, lines 348-371), KaniExecutionRefusal::code (lines 138-151, None for that variant), src/kani/terminal.rs run_terminal_value(&KaniRunOutcome, ..) (line 82), src/kani/run/launch.rs run-stage error classification (lines 224-229) and stdio setup (lines 245-247), src/kani/run/namespace.rs bwrap recipe (lines 129-142). No test, build, Kani or gate was run; all new criteria and TC-049 steps 22-24 are PLANNED/UNRUN and no tag for FR-034-AC-35..37 exists in src/ or tests/. quire validate on the two changed files exited 0 with ambient warnings recorded in the review manifest. Code-review lane: checked every source fact the new text cites and every refusal/route claim against the actual Rust types and classifiers. No Rust file changed, so the rust-review checklist was applied only to the spec's claims about Rust APIs; no Rust or gap-analysis approval is implied.

## Verdict

**FAIL**: two medium findings on the cited refusal route; the cited type and function facts (variant name, code() = None, run_terminal_value input type) are otherwise accurate.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The 'existing typed pre-Dispatch unavailable route with its actual cause' only exists for some causes. src/kani/run/launch.rs:224-229 maps a run-stage io::Error to BoundedLaunchError::Unavailable only when its kind is Unsupported or NotFound; every other kind becomes BoundedLaunchError::Io, which start (execute.rs:365) maps to KaniExecutionRefusal::Tool(KaniToolError::Io { tool: Launcher }). A denied network/mount namespace (EPERM, PermissionDenied) or a failed stdio fstat (EIO, EOVERFLOW) carried 'with its actual cause' through that classifier yields a Tool refusal that names the launcher, contradicting AC-35 and AC-36. State that the admission site classifies these causes as Unavailable whatever their errno kind, or name the stage whose errors map to Unavailable wholesale (as MemoryObserver::prepare and NamespaceOwner::prepare do at launch.rs:207 and 215). | spec/kani/functional/FR-034-caller-death-ownership.md:566-568 |
| FND-002 | medium | AC-36 routes a caller-supplied socket stdin through KaniExecutionRefusal::MemoryMechanismUnavailable. The variant's documented meaning (execute.rs:69, 'No memory-enforcement mechanism is available') and its Display (execute.rs:158-161, 'backend tree memory enforcement is unavailable: {cause}') then misreport a caller-fixable input-type rejection as missing host memory enforcement. The variant has no field that distinguishes the cases, so a caller can tell socket stdin from host capability absence only by parsing the io::Error text. Allocate a distinct typed refusal (a variant, not a code string), or require the variant's doc and Display to be amended so its meaning covers every cause routed through it. | spec/kani/functional/FR-034-caller-death-ownership.md:673 |

## Dispositions

Round 1, reviewed at 622ecfb2cd8ea9fb9489e4839686f2f258f2bfee (base 5d3eaa2bbedcfbd59d8bd3d8df681b70e74cad60); session dbb8a12e-b532-45a6-a5bd-451efbb27322, run 4ade9999-807a-4f26-9625-9b01e9be076f, model claude-opus-5-5.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 622ecfb2cd8ea9fb9489e4839686f2f258f2bfee: Admission failures are now classified as BoundedLaunchError::Unavailable at the admission site regardless of errno/io::ErrorKind with the original cause retained; the text names the run-stage Unsupported/NotFound classifier as insufficient. |
| FND-002 | fixed | 622ecfb2cd8ea9fb9489e4839686f2f258f2bfee: The same top-level variant now carries a mandatory typed KaniStartupAdmissionCause (MemoryEnforcement, BackendStdioSocket, BackendStdioInspectionFailed, CapabilityUnavailable) beside the unchanged io::Error cause, with no Option/default or string discriminant, and planned CODE broadens the variant doc and Display. A caller can now tell socket stdin from host capability absence by typed field. |
