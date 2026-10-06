---
id: TC-049
title: "Verify original-caller death cannot release an unowned backend"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: verifies
---
# TC-049: Verify original-caller death cannot release an unowned backend

## Description

Planned production lifecycle scenarios for
[FR-034](../functional/FR-034-caller-death-ownership.md). These scenarios claim no executable
coverage. Run them only through the actual first-party Cargo guardian and bounded executor;
the earlier tiny research topology is not the implementation under test. The preceding ceiling
slice and its Linux namespace prerequisites must be present before positive execution scenarios
can pass. [TC-039](./TC-039-bounded-proof-ceilings.md) remains the owner of the broader ceiling
and refinement scenarios.

## Test Procedure

1. Build the package's real `quire-kani-guardian` in the owning worktree target directory. Run a
   separate original-caller fixture through the production bounded executor. Record its owned
   guardian/monitor/INIT identities and retain pidfds for observable termination. Use private
   observable handshake barriers to stop at pre-initialization, gated INIT and immediately after
   Dispatch; these barriers observe production transitions without bypassing their behavior.
   Kill only that fixture caller by SIGKILL at each barrier, and repeat abrupt loss with abort
   and a forced process-kill model of OOM (no host-wide memory pressure). Before initialization
   completes, verify no monitor can be created without a live lease. While gated, verify no
   backend marker. After Dispatch, verify INIT termination and owned descendants' termination.
2. Bind another actual child to the private endpoint and require peer rejection. Supply wrong
   monitor/INIT start identity, incorrect parent, namespace identity or namespace PID, incomplete
   startup data and a positively dead INIT. Verify no Dispatch or marker. Assert observer
   readiness before the valid Dispatch control, then retain an unmutated valid-run control.
3. While gated, inject lease EOF, partial/malformed/overlimit startup information, monitor exit
   and setup failure. Observe the retained writer and unreaped monitor ownership through pinned
   group cancellation. Confirm group/claimed INIT termination before gate close or reaping.
   Do not infer cleanup from monitor exit alone. Repeat with concurrent independent runs and an
   unrelated fixture child, verifying they remain live until their own cleanup.
4. After Dispatch, create owned double-fork/setsid descendants which reparent before observation,
   a positively acknowledged late fork during cancellation, and a nested PID namespace when the
   already required platform facilities permit it. Kill the original caller and verify namespace
   INIT teardown cancels them all, including descendants absent from the observer's previous
   sample. Record unsupported platform refusal rather than a passing skipped ownership claim.
5. Exercise normal completion with adopted background work, nonzero/signal exit, backend exec
   failure, setup refusal, explicit cancellation, timeout, memory excess, capture overflow/read
   failure and memory-observation failure. Close backend streams and fail the guardian separately
   while the caller lives. Assert typed refusal where ownership or cleanup cannot be established,
   no hung capture, and no acceptance of a valid success report beside a cleanup failure.
   Verify helper/monitor termination and per-run endpoint removal. For caller-death cases the
   surviving test controller observes cleanup; no dead-caller evidence is manufactured.
6. Exercise explicit installed-helper discovery and package installation. Point discovery at a
   missing or unusable executable, and require typed refusal without backend marker. No copied
   fixture executable or production cfg(test) shortcut participates. Confirm setup documentation
   names the actual helper and Linux/bubblewrap prerequisites without changing host policy.
7. Send malformed, unknown-field, oversized and excessive pending control records, close the
   lease, and race an unrelated concurrent exec. Require bounded refusal/cancellation, never
   Dispatch from EOF. Verify backend descendants and unrelated execs hold no caller-lease or
   retained-gate writer. Inspect safe descriptor mapping alongside runtime inheritance checks.
8. Execute a backend that echoes raw non-UTF8 argument bytes, stdin, overridden and inherited
   environment, cwd, and separate stdout/stderr markers. Check exact recipe preservation and
   capture separation. Feed ordinary completed/refused/falsified reports, actual wall/memory
   stops and ambiguous live-worker RSS through the existing classification path. Keep original
   identity ceilings. When bounded native refinement is implemented, repeat its ceiling-stop
   control through that same ownership path and assert FR-028 AC-24's class/evidence.
9. Distinguish original-deadline expiry before startup, expiry during handshake, and the helper
   setup cap expiring with a still-live original deadline. Require respectively no Dispatch,
   timed out, and typed setup refusal. Verify the executor never resets the original deadline,
   finite control/capture shutdown, and refusal on unconfirmed teardown; no invented physical
   disappearance guarantee is measured by a time threshold.
10. Run independent mutants removing lease cancellation, removing retained gate ownership, and
    closing/reaping before pinned cancellation. Each must fail the named premature-marker or
    surviving-owned-process assertion, rather than compilation/setup. Record the observation
    before emergency teardown of only the fixture's pinned namespace/group. Restore production
    code and require the same focused controls to pass. Executable tests carry criterion trace
    tags for the behavior they actually assert.

## Expected Results

| Authority | Required observation | Regression caught |
|---|---|---|
| FR-034-AC-1/2/3/7 | No pre-Dispatch marker; original-caller loss triggers pinned cancellation before gate close/reap, or claimed INIT termination after Dispatch | Lease cancellation omitted; retained writer removed; gate closes before kill |
| FR-034-AC-4/5/6 | Actual owned peer/monitor/INIT chain and observer readiness precede Dispatch | Arbitrary PPid accepted; reused identity or unrelated peer accepted |
| FR-034-AC-8/9 | Kernel namespace teardown cancels escaped, late-born and nested descendants, preserving other runs | Kill only sampled descendants; cross-run cancellation |
| FR-034-AC-10/11/12 | Every conclusion settles owned teardown; live-caller helper failure refuses; caller-death controller sees endpoint/helper cleanup | Success report accepted before INIT kill; leaked helper/socket |
| FR-034-AC-13/14 | Real package helper and accurate setup instructions; missing or unusable executable refuses | Copied binary, alternate launcher or false prerequisite documentation |
| FR-034-AC-15/16 | Invalid bounded controls refuse; caller lease is exclusive and descriptors are safely mapped | EOF authorization; lease inherited by backend or unrelated exec |
| FR-034-AC-17/18/19; FR-028-AC-2/3/21/24; FR-017-AC-14/24/25 | Exact recipe, separate bounded captures and existing resource/report outcomes after cleanup | Corrupt argv/stdio; diagnostics become report; weaken ceilings; ambiguous RSS becomes zero |
| FR-034-AC-20/21/22 | Original deadline persists; setup-cap expiry is distinct from identity timeout; shutdown observation is bounded and unconfirmed cleanup refuses | Setup resets deadline; setup failure falsely recorded as identity timeout; capture hang |
| FR-034-AC-23/24 | Real built helper and positive lifecycle observations; each ownership mutant fails before separate emergency cleanup | Test-only production bypass; sleep-vacuous pass; cleanup masks mutant survival |

No scenario prose, source inspection alone or killed research probe establishes executable
coverage. Native refinement remains planned until its real typed entry is delivered. Simultaneous
host failure or forced killing of both caller and guardian is outside the original-caller lifecycle
claim; live-caller guardian failure must still refuse and exercise owned cleanup.
