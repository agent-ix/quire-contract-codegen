---
id: "SR-2123"
title: "IR-639 spec-review/integrity review of report threat boundary and backend IPC confinement"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen@b0cb2737eaea22aa1f58970931d92d4f55e3868e; spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-049-caller-death-ownership.md (unopened PR; diff against base cd9fdaa46fec2ae8f7cdc8c9bc4e1c32bce81770)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-2123: IR-639 spec-review/integrity review of report threat boundary and backend IPC confinement

## Summary

Ticket: IR-639. PR: quire-contract-codegen#unopened at b0cb2737eaea22aa1f58970931d92d4f55e3868e. Reviewer model claude-opus-5-5, run dbb8a12e-b532-45a6-a5bd-451efbb27322.
Scope units examined: FR-034 (new section lines 522-574 and Dependencies lines 734-741),
FR-034-AC-35, FR-034-AC-36, FR-034-AC-37, TC-049 (Coverage rows, steps 22-24, Expected Results
rows). Context only: FR-034-AC-4, FR-034 recipe lines 61, 90, 108-109, 157, 280-284. 1 finding(s).

## Method

Read the full base..head diff (+110/-0 lines; git diff --numstat shows 66 and 44 insertions and zero deletions, so FR-034-AC-1..34 and TC-049 steps 1..21 are byte-unchanged at head). Grounded the cited CG facts in source at the frozen head: src/kani/run/execute.rs start (BoundedLaunchError::Unavailable -> KaniExecutionRefusal::MemoryMechanismUnavailable, lines 348-371), KaniExecutionRefusal::code (lines 138-151, None for that variant), src/kani/terminal.rs run_terminal_value(&KaniRunOutcome, ..) (line 82), src/kani/run/launch.rs run-stage error classification (lines 224-229) and stdio setup (lines 245-247), src/kani/run/namespace.rs bwrap recipe (lines 129-142). No test, build, Kani or gate was run; all new criteria and TC-049 steps 22-24 are PLANNED/UNRUN and no tag for FR-034-AC-35..37 exists in src/ or tests/. quire validate on the two changed files exited 0 with ambient warnings recorded in the review manifest. Integrity lens: consistency of the new section with the unchanged normative recipe (L namespaces line 61, setup capability list line 90, exact M argv lines 108-109), the EARS list, the AC table, and TC-049 Coverage/Procedure/Expected Results.

## Verdict

**FAIL**: one high finding. The new mandatory profile is not allocated to any role and contradicts the still-normative exact recipe.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The new IPC profile is mandatory, but no role is assigned to establish it, and it contradicts recipe text that is still normative. Lines 108-109 still require M to run bwrap with exactly '--unshare-user --unshare-pid --as-pid-1 --new-session --bind / / --dev-bind /dev /dev --proc /proc ...': no network namespace, and the whole host root bound. Line 61 has L create only user, PID and mount namespaces, and line 90 lists only NEWUSER/NEWPID/NEWNS as setup capabilities. A host-root bind exposes every host pathname socket (for example /run/user/<uid>/bus and /tmp/.X11-unix) unless some layer curates the view, and no layer is named. Line 540 concedes that the recipe does not establish the profile. But the change neither amends the argv and capability lists nor says which role (L, O or M) creates the network namespace and curates the mount view. It also does not say which capability check produces AC-35's refusal. Allocate both to a named role and update the argv and capability text in the same change. | spec/kani/functional/FR-034-caller-death-ownership.md:535-540 |

## New findings (disposition pass 1)

Reviewed at 622ecfb2cd8ea9fb9489e4839686f2f258f2bfee (base 5d3eaa2bbedcfbd59d8bd3d8df681b70e74cad60).

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | The fix requires that 'Installation/exec failure shall be known as typed refusal before positive Dispatch', and in the same paragraph keeps 'arbitrary backend creation/execution needs positive Dispatch'. Exec of the arbitrary backend recipe can only happen after positive Dispatch, so its failure (for example ENOENT or EACCES on the recipe program) cannot be known before Dispatch. The text is only consistent if 'exec' means exec into the filtered boundary/helper entry, which then waits for Dispatch before executing the recipe. TC-049 step 22 ('Force installation/exec failure and require typed refusal before positive Dispatch') inherits the same ambiguity. Say which exec is meant, and state how a post-Dispatch recipe exec failure is classified. | spec/kani/functional/FR-034-caller-death-ownership.md:582-586 |

## Dispositions

Round 1, reviewed at 622ecfb2cd8ea9fb9489e4839686f2f258f2bfee (base 5d3eaa2bbedcfbd59d8bd3d8df681b70e74cad60); session dbb8a12e-b532-45a6-a5bd-451efbb27322, run 4ade9999-807a-4f26-9625-9b01e9be076f, model claude-opus-5-5.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 622ecfb2cd8ea9fb9489e4839686f2f258f2bfee: L now creates NEWNET and the confined private root before O; O validates both before M; the setup capability list (line 90) adds NEWNET; the unchanged exact inner argv's --bind / / is defined to bind O's confined root, never the host root. Continuous pathname exclusion is assigned to the backend-only seccomp policy. |

Round 2, reviewed at 004c864626720341f3e06994f3d17494bbd6aa44 (prior 622ecfb2cd8ea9fb9489e4839686f2f258f2bfee, base 5d3eaa2bbedcfbd59d8bd3d8df681b70e74cad60); session dbb8a12e-b532-45a6-a5bd-451efbb27322, run f90d3cf1-e644-4114-8208-665e32c34e50, model claude-opus-5-5.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-002 | fixed | 004c864626720341f3e06994f3d17494bbd6aa44: A two-row boundary table now separates the two failures. Policy/filter/privilege installation failure happens before positive Dispatch and is reported over the authenticated startup channel as typed unavailable admission. Exec failure of the actual recipe happens after Dispatch and is handled by the existing bounded backend-failure rules (AC-10 already lists backend exec failure), with no synthetic evidence, no new kind and no retroactive pre-Dispatch reclassification. TC-049 step 22 now tests the two cases separately. |

Round 3, reviewed at a210dc10310e77499d14cbc9f172524a80194a85 (prior 004c864626720341f3e06994f3d17494bbd6aa44, base 5d3eaa2bbedcfbd59d8bd3d8df681b70e74cad60); session dbb8a12e-b532-45a6-a5bd-451efbb27322, run aeb596e2-83a8-4867-a9ef-6170edca8002, model claude-opus-5-5.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-003 | fixed | a210dc10310e77499d14cbc9f172524a80194a85: Step 22's post-Dispatch exec failure now uses an authored regular executable script whose shebang names a deliberately absent absolute interpreter. KaniInstallation::require_executable (src/kani/run/tool.rs:66-84) checks only regular-file kind and execute bits, so it passes. KaniInstallation's only field is the public launcher (tool.rs:58-61), so an ordinary caller can name the script. The launcher is the recipe program (execute.rs:822 BackendCommand::new). execve then fails with ENOENT for the absent interpreter after positive Dispatch. That avoids the execvp ENOEXEC shell fallback, and closing the writing handle avoids ETXTBSY. Missing and non-executable launchers stay pre-Dispatch Tool refusals. No race, sleep, copied ELF or public hook is used. The fixture is still PLANNED/UNRUN. |

## New findings (disposition pass 2)

Reviewed at 004c864626720341f3e06994f3d17494bbd6aa44 (prior 622ecfb2cd8ea9fb9489e4839686f2f258f2bfee).

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | TC-049 step 22's post-Dispatch recipe exec failure case names 'missing/non-executable recipe program' as its trigger. Those inputs never reach Dispatch: start (src/kani/run/execute.rs:355-358) calls KaniInstallation::require_executable (src/kani/run/tool.rs:66-84) before launch, which refuses a missing path or a file with no execute bit as KaniToolError::Io before any backend spawn. The named case therefore yields the existing pre-Dispatch Tool refusal, or else needs a timing race that TC-049 forbids. Name a trigger that passes the existing check and fails at the actual exec, such as an executable regular file with an invalid format or a missing ELF interpreter, or state that the existing pre-check stays and how the post-Dispatch path is reached. | spec/kani/matrix/TC-049-caller-death-ownership.md:436-438 |

## New findings (disposition pass 5)

Scoped round 5 on the stdio-capture fix and the new settlement/kernel-fault/API delta only; reviewed at 0696d1bc567f1d5717b9a54cdda828664211f9d5 (prior 412f056e814882eb94b9e9e65977d6993326f3aa, base 5d3eaa2bbedcfbd59d8bd3d8df681b70e74cad60); session dbb8a12e-b532-45a6-a5bd-451efbb27322, run 7533286d-299c-4811-af95-031c491e2c1e, model claude-opus-5-5.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | high | The new settlement rule clamps every settlement, Drop and join observation to the remaining original deadline T, forbids any added grace or extra timeout, and returns CleanupUnconfirmed with no outcome when confirmation is not reached in that window. When the run is stopped because T itself expired (the wall-clock ceiling), the remaining window is zero. SIGKILL delivery, namespace teardown and reaping all take non-zero time, so confirmation essentially never fits. Every genuine timeout would therefore return Err(Guardian { kind: CleanupUnconfirmed }) instead of the Inconclusive timed-out classification. That contradicts the unchanged FR-028-AC-2 and FR-034-AC-20 ('actual identity-deadline expiry during setup or execution uses the same classification'), and AC-10's confirmed teardown on timeout. Today launch.rs:128-131 returns within timeout plus a small constant (poll interval plus STOP_DRAIN_LIMIT), which the new rule now forbids. The 'existing lease-close/cleanup caps' of 250 ms and five seconds named on line 805 appear in neither merged source nor the prior spec. Allocate an explicit settlement reserve inside T (stop the backend at T minus the reserve) or a bounded post-expiry settlement allowance, and state how timeout classification and CleanupUnconfirmed interact. | spec/kani/functional/FR-034-caller-death-ownership.md:803-808 |
