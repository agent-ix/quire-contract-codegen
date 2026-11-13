---
id: SR-17940486709564847047961
title: "IR-1530 PR #825 code-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #825 at 212a7dfd, git diff origin/main...HEAD (main 9677d931); IR-1530; docs/qualification/ir1530-general-route-scalar-table.json, docs/qualification/ir1530-general-route-scalar-not-tractable.md, tests/it/kani_tractability.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-056
    type: reviews
---

## Summary

Ticket: IR-1530 (follow-up to #821). This is a code-review with the rust-review lane folded in. The shipped general-route scalar table is split into
two rows: `quire.op.integer.add` from 2 bits (new probe) and `quire.op.integer.sub` from 4 bits
(unchanged). The tests now assert the add row at 2 bits, refusal at 2, 3 and 4 bits, a 1-bit add
running, and sub running at 3 bits and refused at 4. The embedded table still equals the docs file,
now with 2 rows.

Width reasoning (question b): the add row "from 2 bits" is not an extrapolation over 3 and 4 bits.
All three widths have a measured timeout: 2 bits from this probe (`Int[2, 2]`), 3 bits from the
quire-driver lane run (`Int[4, 4]`) and 4 bits from #816 (`Int[0, 9]`). Wider than 4 rests on
FR-056's "from B bits" monotonic rule, as before. The 2-bit probe has singleton operands, which
supports the doc's claim that the cost is the `rt::Integer` carrier, not the width. Keeping sub at its one
measured width (4 bits) follows the measurement-only policy. Narrower subtractions are left to the
backstop, and the doc says so.

IR-1526 consumers (question d): no behaviour change. Generation tables (`tests/fixtures/tractability/table.json`, `GenerationOptions`) are untouched, and only `shipped_general_route_scalar()` changes. quire-driver does not call it or the decided wrappers yet (grep of `quire-driver/src`), so no production path changes.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | The add row records `peak_bytes: null` and the doc says "peak memory not captured", but the probe's cgroup peak was recorded. The user journal on titan shows `run-p3781358-i28999794.scope` started 2026-11-13 07:11:47 with `cargo test --test it ir1530_probe_general_route_two_bit_add -- --ignored --nocapture`, then `Consumed 10min 8.708s CPU time over 10min 339ms wall clock time, 13.6G memory peak`. The driver's 3-bit run (`run-p3742405-i28935435.scope`, `successful_native_receipt_is_authenticated_against_retained_limits`) shows `13.8G memory peak`. No re-probe is needed: record the scope peak (systemd's `G` is GiB, about 1.46e10 bytes) in the row and the doc, and say it is the scope peak, not a runner-reported peak. | docs/qualification/ir1530-general-route-scalar-table.json:11, docs/qualification/ir1530-general-route-scalar-not-tractable.md:22, docs/qualification/ir1530-general-route-scalar-not-tractable.md:35 |
| FND-002 | medium | The 2-bit probe is not retained. `ir1530_probe_general_route_two_bit_add` (the test the journal shows was run) is not in the PR or on main, and the doc names neither it, its fixture (`x + 1` with `left` `Int[2, 2]`, `right` `Int[1, 1]`) nor a log. The row that now refuses 2- and 3-bit adds cannot be reproduced from the repo. #819 set the precedent by committing its probe as `ir1526_probe_wide_integer_multiply_under_installed_kani`. Commit the probe as an ignored Kani-lane test and name it in the doc. | docs/qualification/ir1530-general-route-scalar-not-tractable.md:31-36 |

## Verdict

The data change is sound and the tests match it; the embedded table equals the docs file. Not
mergeable yet: FND-001 is a recoverable data correction (no re-probe), and FND-002 is about
reproducibility of the evidence. Both are small and fit in this PR.
