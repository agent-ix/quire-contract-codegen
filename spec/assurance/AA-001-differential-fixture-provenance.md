---
id: AA-001
title: "Sireum differential fixture provenance and open evidence claim"
type: AssuranceArgument
status: proposed
owner: quire-contract-codegen maintainers
profile: ix://agent-ix/quire-contract-codegen/AP-001
top_claim:
  id: permitted-third-party-agreement
  statement: attributed permitted third-party observations agree with the declared CG Boolean pilot semantics
  subject: the proposed Sireum Boolean pilot described in this record
  status: open
reasoning:
  - id: provenance-before-comparison
    statement: a comparison is attributable only when producer, inputs, observations and retention permission are established
    supports: permitted-third-party-agreement
    sufficiency_criteria:
      - actual acquired tool and exact invocation are recorded
      - every retained input and output has permitted provenance and immutable content identity
      - each pilot case has an attributable raw observation and a declared semantic comparison
      - adverse controls remain non-success and every discrepancy has a disposition
  - id: distinguish-local-evidence
    statement: existing independent local evaluation does not establish execution of an external tool
    supports: permitted-third-party-agreement
    sufficiency_criteria:
      - local evaluator and generated compile/runtime observations retain their own evidence class
      - zero acquired external results supplies zero third-party comparison evidence
assumptions: []
participants:
  - id: provenance-author
    role: specification and research author
    authority: record source observations and limitations
    independence: authored this record; supplies no independent acceptance verdict
  - id: independent-reviewer
    role: Luna IR specification reviewer requested
    authority: assess the scoped contract and evidence sufficiency for maintainers
    independence: separate from the provenance author; assignment and verdict pending
challenges:
  - id: output-permission
    target: permitted-third-party-agreement
    statement: permission for the particular retained output or derived fixture has not been established
    status: open
    owner: quire-contract-codegen maintainers
  - id: no-external-run
    target: permitted-third-party-agreement
    statement: no tool archive, executable run or authoritative third-party output has been acquired for this pilot
    status: open
    owner: quire-contract-codegen maintainers
  - id: unimplemented-comparison
    target: permitted-third-party-agreement
    statement: the Rust acquisition and comparison procedure and adverse controls have not been implemented or exercised
    status: open
    owner: quire-contract-codegen maintainers
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AP-001
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-005
    type: references
  - target: ix://agent-ix/quire-contract-codegen/NFR-002
    type: references
---
# AA-001: Sireum differential fixture provenance and open evidence claim

## Claim

The third-party agreement claim is **open**. This record supplies a proposed acquisition and
retention contract, public-source research and an explicit unavailable lane. There are zero
retained Sireum/INSPECTA outputs and zero observed third-party comparisons.

The [IR-103 recorded owner decision](https://linear.app/agent-ix/issue/IR-103#comment-c6ed05ff)
permits selecting either tool, a suitable released version and sensible reproducible parameters.
It also requires that unclear retention permission retain neither bytes nor a derived fixture.
The decision keeps this dependency open; local independent evaluation remains local evidence.

## Reasoning

The source observations identify a candidate. The proposed procedure and adverse controls define
what an attributable future comparison would need; the current record leaves those obligations open.

### Source research and candidate selection

Public release/license/CLI sources were inspected on **2026-10-23 UTC**. This is the research
date, not an acquisition or execution date.

| Item | Observed source and disposition |
|------|---------------------------------|
| Candidate | Sireum Kekinian / Logika, release [4.20261021.b1efad5d](https://github.com/sireum/kekinian/releases/tag/4.20261021.b1efad5d), published 2026-10-21; GitHub's latest non-prerelease at inspection. Suitability for the pilot remains untested. |
| Exact source | [Release source](https://github.com/sireum/kekinian/tree/b1efad5d98aaa00fca51f4764f4bd700270a1198); its Logika submodule resolves to [this source revision](https://github.com/sireum/logika/tree/b8f09a19eeac3efc15e03fbd1368ca03a3be62c1). |
| Candidate distribution | The release's `sireum-cli-linux-amd64.tar.xz`. No distribution or executable was acquired. |
| Software license | Release [license](https://github.com/sireum/kekinian/blob/4.20261021.b1efad5d/license.txt) and matching Logika [license](https://github.com/sireum/logika/blob/b8f09a19eeac3efc15e03fbd1368ca03a3be62c1/LICENSE.md) identify BSD-2-Clause terms. Any redistributed software must retain the applicable notices and conditions. Bundled components require their own provenance assessment. |
| Output retention | No permission determination for a particular output/derived fixture has been made. The software license reference alone is not recorded as permission for every possible output or input. No output bytes or derived external fixture retained. |
| INSPECTA | Not selected for this pilot; no acquisition, invocation or comparison recorded. The owner decision allows a later separately recorded choice. |
| Current lane | Unavailable: no acquired tool, licensed retained result or executed comparison. No content digest is invented for absent bytes. |

### Proposed local acquisition procedure

The future procedure first records the selected distribution source, actual acquisition date,
resolved tool and solver identities, applicable notices, and permission for the exact inputs and
retained outputs. It preserves immutable identities binding those exact bytes to the comparison
evidence. A changed release or solver configuration is a new recorded comparison subject.

Initial inputs are first-party Boolean pilot cases, not copied upstream examples. Their Slang
renderings are external-tool input data; first-party acquisition, rendering, parsing, comparison
and audit logic remains Rust. No tool run, input rendering or output acquisition occurs in this
change. Each future rendered input needs its exact retained bytes and identity before execution.

The selected release's [CLI definition](https://github.com/sireum/logika/blob/b8f09a19eeac3efc15e03fbd1368ca03a3be62c1/shared/src/main/scala/org/sireum/logika/cli.scala)
declares `logika verifier`, parseable messages, sequential SMT queries and a timeout in seconds.
The proposed argument-vector template is:

```text
<recorded-sireum-executable> logika verifier --parseable-messages --smt2-seq --timeout 2 inputs/<case-id>.sc
```

This template has not been run or validated against an installed binary. The future receipt
records its fully expanded argument vector, working directory, relevant environment, exact
resolved solver configuration, stdout, stderr and exit status. Missing tools or unavailable
solver configurations leave the lane unavailable. Default solver choices must be recorded;
the release label alone does not establish the actual executable or solver used.

Acquisition is an explicit local operation. Quire, Quoin, normal build/test invocations and hosted
workflows do not download, generate or refresh these outputs implicitly. This record adds no
hosted workflow, installer, executable wrapper or language exception.

### Pilot semantics and normalization

The proposed domain is pure Boolean constants and `p`/`q` valuations. It includes negation,
conjunction, disjunction and implication; every included binary expression uses all four ordered
Boolean input pairs. The exact rendered cases and order remain an input artifact to be authored
and reviewed before execution.

For each ground input, CG's observed Boolean result is compared with an external assertion of
the same expression and expected value. A Logika proof status is evidence about that assertion,
not itself the Boolean return value. The comparison needs an attributable assertion/case identity
and a complete terminal observation. It preserves failure, timeout and unknown outcomes rather
than treating a zero process exit alone as verification.

The negative pilot control changes one expected result, for example conjunction at `(false,
true)` from false to true. A parser that classifies both assertions as agreement fails that
control. Integer width/overflow, arithmetic, division, floating point, metering, temporal/state
semantics and effectful short-circuit behavior are outside this Boolean pilot and receive no
comparison credit. Pure Boolean truth-table agreement makes no claim about those distinctions.

Raw observations remain unchanged. The proposed semantic view joins by case/assertion identity
and records terminal outcome; timestamps, paths, diagnostic order and solver statistics are not
used to infer semantic agreement. Any display normalization is separately named and preserves
the raw evidence. No actual normalizer or message grammar is claimed implemented here.

### Evidence and adverse controls before a future claim

Every retained case needs tool/source identity, corpus/source identity, actual acquisition and
run conditions, exact invocation, raw input/output content identities, per-file license and
permission, normalization definition, comparison result and any discrepancy disposition.
Existing shared assurance/evidence owners remain authoritative; this record defines no new
serialization format or duplicate evidence framework.

| Required control | Evidence interpretation |
|------------------|-------------------------|
| Complete permitted case and attributable terminal observation | Eligible for comparison; eligibility alone does not establish agreement. |
| Missing input, tool or output | Visible unavailable case, no agreement credit. |
| Unsupported tool version or semantic domain | Visible unsupported case, no agreement credit. |
| Truncated, ambiguous or malformed messages | Non-success with retained permitted raw cause; no guessed terminal result. |
| Source identity mismatch | Refused comparison subject; expected and observed identities remain distinguishable. |
| Raw input/output content drift | Refused evidence binding; no refresh of the expected identity during checking. |
| Unknown retention permission | Unavailable lane, with neither third-party bytes nor a derived fixture retained. |
| Discrepancy lacking a regression fixture or documented semantic difference | Unresolved discrepancy; no successful corpus result. |
| Incorrect expected Boolean result | Detects a parser/comparator that falsely reports agreement. |

These are planned Rust acceptance controls after specification review, not executed tests. Every
future discrepancy joins to exactly one disposition in
[TC-007](../core/matrix/TC-007-cross-backend-parity.md). Missing, unavailable and unsupported cases
remain visible in the inventory rather than disappearing from the denominator.

## Sufficiency Decision

**OPEN; third-party lane unavailable.** Public metadata and source-license research identify a
candidate and its remaining evidence obligations. They establish no tool execution, output
retention permission or differential agreement.

The existing [265-expression independent evaluator](../../tests/it/oracle_generation.rs) and
generated compile/runtime controls are local implementation evidence. This research change did
not rerun those controls and does not relabel them as Sireum/INSPECTA evidence. Independent Luna
IR specification review is requested; no reviewer acceptance or completion of IR-103 is claimed.

## Challenges

The open frontmatter challenges preserve the missing output-permission determination, external
run and Rust comparison controls. Resolving them needs actual permitted evidence and the owning
review decision. Until then, the top claim remains open and the ticket's fixture dependency
remains explicit.
