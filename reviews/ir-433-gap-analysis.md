---
id: "SR-619"
title: "IR-433 gap analysis: one copy of first-party crates in CG, FR-030/TC-041"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@a57aa9108f0d49227ec8d818605acaf7bb895b53; Makefile, Cargo.toml, scripts/check_one_copy.awk, CLAUDE.md, README.md, spec/functional/complete-v1/FR-030-ir-outcome-terminal-map.md, spec/test/complete-v1/TC-041-ir-outcome-terminal-map.md, spec/test-matrix.md, spec/index.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-030
    type: reviews
---

# SR-619: IR-433 gap analysis

## Summary

Ticket: IR-433. PR: agent-ix/quire-contract-codegen#196, head a57aa91. Plan completion: not
assessed (planless). The owner plan was read as context only.

## Method

I checked that the new spec units are honestly Planned and have no evidence claims. I checked
that no production source changed without an owning requirement, and that the developer docs
describe the new gates. I grepped `src/` and `tests/` for any existing map from `KaniOutcome` to
`TerminalValue`. I ran `quire validate` with the Makefile's globs.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | CLAUDE.md's command list still describes `make deny` as "all configured cargo-deny lanes", although it now also runs the one-copy gate. It does not list `make use-local` / `make use-remote`, or say that `LOCKED` drops `--locked` while a patch is active. | CLAUDE.md:19-25, Makefile:14, Makefile:97 |

## Verdict

Approve. There is one low documentation finding.

What is right:

- FR-030 and TC-041 are `Planned`, and the matrix rows are `🚧 Planned`. No test carries an
  FR-030 tag, and `src/`/`tests/` contain no `provider_result`, `KaniProviderResult` or
  `TerminalValue` map. So there is no coverage inflation, and "No code implements this map at
  this revision" is true. The only related code is `classify_transcript`'s call to
  `KaniOutcome::proved_from_checks` (src/kani_execution.rs:615), which is FR-029/FR-017
  territory.
- No `src/` or `tests/` file changed. The skeleton-spine test compiles and passes against qsl-replay at
  966e7d2 with no edit.
- FR-030 and TC-041 have no id collisions: neither id exists on origin/main.
- `quire validate --scope . 'spec/**/*.md' 'plan/**/*.md' 'reviews/**/*.md'` exits 0. `make spec`
  itself exits 2 on this host, because the Makefile's `override QUIRE` points at
  `~/.npm-global/bin/quire`, which is not installed here. That is a host problem, not a PR
  defect.
- The TC-041 matrix placement defect is recorded in SR-620, not here.
