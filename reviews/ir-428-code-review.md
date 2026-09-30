---
id: "SR-611"
title: "IR-428 code review (incl. rust-review lane): bump quire-contract-ir to main"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@140868a8033f5c768fccca02f5c7a9f02a7001dd; Cargo.toml, Cargo.lock, deny.toml"
relationships: []
---

# SR-611: IR-428 code review

## Summary

Ticket: IR-428 (created by the reviewer; the PR carried no ticket). PR:
agent-ix/quire-contract-codegen#195, head 140868a, diffed against origin/main 8e02db4.

The PR moves the `quire-contract-ir` pin from 37c34de8 to a7e019a6. It moves the
`quire-verification-contracts` dev-dependency from 61f4a44d to 4c49706d, which is the rev
IR a7e019a6's `quire-contract-model` resolves. In deny.toml it drops the license exceptions
for `quire-observation`, `quire-protocol` and `agent-ix-baseline-producer`, and adds one for
`qsl-attrs`. No Rust source changed.

## Method

I read the whole diff. I checked every `[[package]]` added to or removed from Cargo.lock. I ran
`cargo tree` for the serde_json features and for the removed crates on the head and on
origin/main. I read both QSL copies' manifests and the qsl-attrs manifest. I ran `make deny` on
both revs and `cargo test --locked --workspace --all-targets` on the head.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | `[sources] allow-git` still lists quire-observation, quire-protocol, tl-mltl and tl-syntax, which the graph no longer reaches. `make deny` warns `unmatched-source` for all four, and the block's own comment says it lists only repos "the resolved graph actually reaches". | deny.toml:64-79 |

### FND-001 detail

The PR drops the license exceptions for the removed crates but not their git sources. As a
result, the comment at deny.toml:64-67 is now false, and re-adding any of those repos would
pass the `sources` check with no review. The license check would still catch it, because those
crates are AGPL and their exceptions are gone. Fix: delete deny.toml lines 73, 74, 78 and 79.
The four remaining `license-not-encountered` / `advisory-not-detected` warnings are also on
origin/main, so they are not part of this finding.

## Verdict

Approve with one low finding. Every check the owner asked for passes:

- The Cargo.lock changes are exactly what the bump needs. Nine packages were removed:
  agent-ix-baseline-producer, engineering-assurance, quire-mltl, quire-observation,
  quire-protocol, quire-verification-contracts@da8d7272, tl-mltl and tl-syntax. The
  `quire-contract-ir` and `quire-contract-model` sources were rotated. The `quire-contract-ir`
  dependency list lost the removed crates plus serde_stacker, sha2 and stacker, which are still
  in the graph through other crates. No crate was added and no registry version changed.
- `cargo tree -e features -i serde_json --target all` on the head shows only the features
  default, float_roundtrip, raw_value, std and unbounded_depth. `arbitrary_precision` is absent.
  On origin/main the same command shows `arbitrary_precision` coming from
  agent-ix-baseline-producer through quire-observation. That shows the check is not vacuous.
- `cargo tree --workspace -e normal,dev,build` on the head contains none of quire-observation,
  quire-protocol, quire-mltl, tl-syntax, tl-mltl, agent-ix-baseline-producer or
  engineering-assurance.
- The qsl-attrs exception is correct and minimal. Its source is
  github.com/agent-ix/quire-spec-language, which makes it first-party. Its manifest declares
  `license = "AGPL-3.0-or-later"` at both QSL revs, and the exception is scoped to that one crate
  and that one license. qsl-attrs was already in origin/main's graph, and origin/main's
  `make deny` rejected it.
- `make deny` exits 0 on the head. On origin/main it exits 2: 4 license rejections
  (engineering-assurance, qsl-attrs at both revs, quire-mltl) and 2 source-not-allowed. The PR
  body says main failed only on the qsl-attrs rejection, which understates it. That is an error
  in the PR text, not the code.
- Tests: 205 passed, 8 failed, 5 ignored. The 8 failures have the names the coder listed
  (bound_coverage x1, bounded_kani_corpus x3, kani_generation x3, oracle_generation tc_006).
  Every one fails on host tooling: "no such command: `kani`", "cargo-kani version query failed",
  "qualified tool missing" or "qualified llvm-tools must already be installed".
- The remaining second QSL copy (qsl-replay at 1a368fa4, IR at 9395be42) is fine to leave for a
  follow-up. I grepped every Cargo.toml in both QSL checkouts and in both filament-core-data
  checkouts (033e2284, 1572ba4b) for `arbitrary_precision` and found no match. They enable only
  raw_value and unbounded_depth.
