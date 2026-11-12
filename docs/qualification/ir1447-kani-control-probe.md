# IR-1447 probe: tractability of `let`, `if` and `case` under installed Kani

This is the durable record behind FR-015-AC-152 to FR-015-AC-173. It is a spec-time probe of hand-written
scratch code, not a delivery and not CG evidence: no CG generator, harness or runner took part.

## What was probed

A scratch crate outside the repository (one `lib.rs`, no dependency on CG, quire-exact or the contract
runtime), following the IR-1022 recipe of `ir1022-kani-clause-measurements.md`: native `i64` integers, no
`Outcome`/`Value` heap arm, an independent `i128` oracle evaluated by substitution. Each clause is a
postcondition with a Boolean result: the subject returns the Boolean, the harness asserts it equals the
independent expectation at the message `the produced V2 Boolean result agrees with the independent checked
condition` (`src/kani/generate/clause/v2.rs`). A Cargo feature `mutant` flips the mutant site.

| Harness | Clause | Healthy subject | Mutant subject | Distinguishing input |
| --- | --- | --- | --- | --- |
| `prove_let` | `let y = amount + 1 in y <= 1000`, `amount` in 0..=999 | `amount + 1 <= 1000` | binding computed as `amount + 2` | 999 |
| `prove_if` | `(if amount < 500 then amount + 1 else amount - 1) >= 500`, `amount` in 0..=1000 | test `amount < 500` | test `amount <= 500` | 500 |
| `prove_case` | `(case kind of A -> amount + 1, B -> amount * 2, C -> 0) >= 1000`, `kind` an `i64` member index in 0..3, `amount` in 0..=1000 | arm `B` is `amount * 2` | arm `B` is `amount * 3` | member 1, `amount` 400 |
| `prove_nest` | `let y = (if amount < 500 then amount + 1 else amount - 1) in y >= 500` | test `amount < 500` | test `amount <= 500` | 500 |
| `prove_coll` | invariant `forall element: (if element < 500 then element else 500) <= 500`, cardinality 0 to 4, element in 0..=1000 | no subject; oracle else branch `500` | generated-oracle else branch `501` | an element of at least 500 |

The collection claim has no subject (the direct collection harness of FR-015-AC-99), so its mutant is in the
oracle. An earlier collection mutant `element <= 500` verified: at 500 it selects `element` or `500`, the same
value (an equivalent mutant), and is not a control.

## How it was run

Kani 0.68.0 (CBMC 6.11.0), one job at a time under the shared cargo slot lock, inside
`systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0`, `timeout 600` per job:

    cargo kani -Z function-contracts [--features mutant] --harness h::<name> --exact --solver cadical \
        --output-format terse

No `--unwind` flag was given: each harness carries `#[kani::unwind(n)]`, which governs it. `prove_let`,
`prove_if`, `prove_case` and `prove_nest` have no loop and use 3; `prove_coll` iterates four elements and uses 6.
Wall is `/usr/bin/time` of the `cargo kani` process after the lock was taken, with the scratch crate already
built. Memory is `time`'s maximum resident set size of that process tree, not a cgroup peak.

## Results

| Harness | Healthy | Mutant | Failing check (mutant) | Wall healthy / mutant | Kani verification time healthy / mutant | Max RSS |
| --- | --- | --- | --- | --- | --- | --- |
| `prove_let` | SUCCESSFUL | FAILED | the produced V2 Boolean result agrees with the independent checked condition | 0.16 s / 0.15 s | 0.024 s / 0.021 s | 136 MB |
| `prove_if` | SUCCESSFUL | FAILED | the produced V2 Boolean result agrees with the independent checked condition | 0.14 s / 0.17 s | 0.024 s / 0.029 s | 135 MB |
| `prove_case` | SUCCESSFUL | FAILED | the produced V2 Boolean result agrees with the independent checked condition | 0.18 s / 0.18 s | 0.040 s / 0.041 s | 137 MB |
| `prove_nest` | SUCCESSFUL | FAILED | the produced V2 Boolean result agrees with the independent checked condition | 0.17 s / 0.17 s | 0.030 s / 0.028 s | 135 MB |
| `prove_coll` | SUCCESSFUL | FAILED | the admitted collection body holds for every drawn collection | 0.21 s / 0.21 s | 0.054 s / 0.056 s | 143 MB |

Every form verified or falsified in well under one second, against the 10 minute allowance. The earlier
integer-range `match` probe (a `match` over the ranges 0..=99, 100..=499 and the rest) also verified and
falsified in 0.02 s, but it is not a Contract IR form and is not in the table.

## What this does and does not show

- Native `let`, `if`, a `case` over an arity-0 union drawn as a member index, a nested `let` over an `if`,
  and a collection `if` are tractable at this shape. The solver cost is the cost of the arithmetic they
  select; the control structure added none that was visible.
- The covers are weak. The `let`, `if`, `nest` and `case` covers constrain only to the reach of the distinguishing
  input (`kani::cover!(x == 500)` and the like) or to the equality itself, and the collection cover is full
  cardinality; "cover satisfied" shows reachability, not non-vacuity of every branch. The delivering ticket's
  harnesses carry FR-015-AC-7's completion cover.
- The probe has none of the IR-1022 cost drivers: no `Outcome<_>`/`Value` heap arm, no meter, no kernel
  evaluation of the operators. The real harness runs the quire-exact kernel (43.5 s healthy and 11.6 GB falsified
  for the clause of `ir1509-kani-clause-controls.md`), so the probe predicts only that control adds little to that
  baseline, not the real wall or memory. The union is a plain `i64` index here; the spec requires that
  carrier (FR-015-AC-154), because a kernel `Value::Union` arm is the fat-enum cost driver of
  `ir1022-kani-clause-measurements.md` and was not probed. The real measurement belongs to the delivering ticket.
- A `case` scrutinee in Contract IR is a `union` (QSpec FR-440 and FR-322 `union_arms`, enforced at admission
  by quire-contract-ir FR-038), one arm per member in declaration order. No integer-range `case` reaches CG,
  and a QSL `enum` is the separate `enum` family and is never a `case` scrutinee. The spec therefore lowers
  only arity-0 unions and refuses a union with a payload member (FR-015-AC-162).
- The union is probed as a plain index; FR-025-AC-19 (PLANNED, IR-1447) adds the missing subject-ABI row for it.
- Not probed: payload-carrying union members, non-native branch values, nesting beyond one `let` over one `if`,
  and the laziness and refusal fixtures of FR-015-AC-168 and FR-015-AC-169, which need the kernel.
