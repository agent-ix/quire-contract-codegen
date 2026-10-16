---
id: SR-11901
title: "Failure-domain review of IR-809 NoVerdict diagnostics"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-017-kani-execution-evidence.md (NoVerdict diagnostics, AC-32 to AC-36), spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md (PR #403)"
review_set: subset
---

## Summary

Ticket: IR-809. Examined the phase rule, the redaction step, the tail rule and the batch sharing
rule for unstated failure modes, identity confusion and edge cases. The phase markers were
confirmed against real Kani 0.68.0/CBMC 6.11.0 output (`Kani Rust Verifier `, `Checking harness `,
`Solving with CaDiCaL 3.0.0`, the compile markers). A cargo metadata failure (missing path
dependency) prints the banner and `error: Failed to get cargo metadata` and no compile marker, so
it falls to `unknown`: the stated under-claim holds there. `backend`/`solver` are reachable only by
stand-ins, which the spec states honestly and AC-33 exercises. Six findings.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | Over-claim in a batch: the rule is "first match over the whole group stream" and the group shares one record, so in a group A,B,C where A reached `Solving with ` and the driver died after `Checking harness B`, every member (including C, never started) carries phase `solver`. That contradicts the stated invariant that the phase names "the last stage the output shows was started" and "never over-claims". State that a group phase is the furthest stage any member reached, or derive it from the stream after the last `Checking harness ` line. | spec/kani/functional/FR-017-kani-execution-evidence.md:366-385, :422-425 | wrong-requirement |
| FND-002 | medium | Tail rule ambiguity on a trailing newline: for a stream ending in one >8192-byte line plus `\n` (the normal shape), a literal reading takes the empty suffix after the final `\n` as "the longest suffix ... that starts at the start of a line", giving an empty excerpt with every byte omitted, while another reading treats the long line as "the last line" and keeps its last 8192 bytes. The final error line is lost under the first. Define "line" (terminator included) and that the empty suffix is not a candidate; add the trailing-newline case to AC-34. | spec/kani/functional/FR-017-kani-execution-evidence.md:405-407, :469 | wrong-requirement |
| FND-003 | medium | Replacement order between `<HOME>` and the crate/target directories is unspecified ("the longer replaced first" covers only crate vs target). A crate under home (`/home/peter/dev/...`, the real lane) becomes `<HOME>/dev/...` if home is replaced first and `<CRATE_DIR>` otherwise, so AC-35's expectation depends on an unstated order. Degenerate home values are also unhandled: `HOME=/` (containers, services) would replace every `/`, and plain prefix replacement turns `/x/crate2/...` into `<CRATE_DIR>2/...`. Specify longest-first across all three, a path-component boundary, and skipping a home of `/` or empty. | spec/kani/functional/FR-017-kani-execution-evidence.md:397-399, :470 | missing-requirement |
| FND-004 | medium | "At the start of a word" is undefined for the `<PATH>` and `<DURATION>` rules. After `<CRATE_DIR>` substitution, `<CRATE_DIR>/src/lib.rs` has `/src/lib.rs` preceded by `>`: if word start means "after a non-word character", it becomes `<CRATE_DIR><PATH>`, contradicting AC-35's "relative remainder of a path under a known directory kept"; the same reading turns `https://host/x` into `https:<PATH>` and keeps `(` and `)` inside the path run as in Cargo's `Compiling kprobe v0.1.0 (/tmp/...)`. Define the boundary (start of line or after whitespace / an opening bracket) and say whether a trailing `)` or `:` belongs to the path. | spec/kani/functional/FR-017-kani-execution-evidence.md:399-402, :470 | wrong-requirement |
| FND-005 | low | The duration rule anchors only the end of the word, so a number inside an identifier is eaten: `Checking harness proofs::check_ttl_30s...` becomes `check_ttl_<DURATION>...`, `u32s` becomes `u<DURATION>`. Classification is unaffected, but the excerpt loses the harness name it exists to show. Require the number to start the word (or follow `: `/whitespace), and say that the exponent may be signed (Kani prints `1.385e-05s`). | spec/kani/functional/FR-017-kani-execution-evidence.md:400-401 | wrong-requirement |
| FND-006 | low | The `backend` phase is said to name a driver that "ended after CBMC started", but Kani prints `Checking harness ` before its per-harness goto linking/instrumentation and before CBMC's own `CBMC 6.11.0 (cbmc-6.11.0)` banner, so a death in that window is `backend` with CBMC never started. Reword to "after Kani began checking the harness". | spec/kani/functional/FR-017-kani-execution-evidence.md:372-373, :390-391 | wrong-requirement |

## Scope examined

Phase rules 1-6 and the under-claim bullet; the CBMC-failure measurement bullet; redaction steps
1-4; `omittedBytes`; the determinism bullet; the 8 MiB interaction bullet; the batch-sharing
bullet; AC-32 to AC-36; TC-027 and TC-043 step 11. Clean: rule 1 (report exported -> `unknown`)
matches the only NoVerdict-with-report path in src/kani/classify.rs:214; rule 5 (`launcher`) is
reachable by the silent stand-in and by a real usage error; the over-limit/unread path cannot
produce a record because it produces no evidence; resource/deadline stops are never NoVerdict, so
they carry no record.

## Verdict

Changes requested on FND-001 to FND-004; FND-005 and FND-006 are wording fixes.
