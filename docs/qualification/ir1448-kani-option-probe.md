# IR-1448 installed-Kani probe of the option encoding

This file is the durable record behind FR-055. It states what was run, what came back and what
was not exercised. It measures the encoding only; it is not CG evidence and carries no proof
credit for any FR-055 criterion.

## What was probed

A hand-written scratch crate (one file, no CG, no quire-exact) following the IR-1022 recipe
(`ir1022-kani-clause-measurements.md`): an `Option<Int[0, 1000]>` as a `Copy` struct of a
presence `bool` and an `i64` payload, the payload assumed in `0..=1000` when present and `0`
when absent (canonical absent); a `Copy` payload-free `Stop { Undefined, OutOfDomain }`;
`option_present` returning the presence and `option_value` returning `Err(Undefined)` when
absent; no heap-carrying enum arm and no dropped outcome; the cover read into a plain `bool` and
last. The claim is `result = (present(x) implies value(x) <= 1000)` against a hand-written
subject. The composite probe is `Option<R>` with `R` an `i64` in `0..=1000` and a `bool`, all
leaves canonical when absent, claim `present(y) implies value(y).n <= 1000`.

## How it was run

Kani 0.68.0 (standalone `kani probe.rs`), `--harness <name> --exact --unwind 3 --solver cadical
-Z concrete-playback --concrete-playback print`, `timeout 600`, inside `flock
/tmp/cargo-slot.lock` and `systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0`, one run
at a time. Memory is the scope's cgroup `memory.peak` read after the run; it is a lower bound
and not a RSS measurement, so only the order of magnitude is meaningful.

## Results

| Harness | Subject or claim | Verdict | Program steps | Kani verification time | cgroup peak |
| --- | --- | --- | --- | --- | --- |
| healthy | `!present or value <= 1000` | VERIFICATION SUCCESSFUL, cover satisfied | 930 | 0.028 s | about 33 MB |
| mut_present | `!present or value < 1000` | FAILED: "produced result agrees with the oracle" | 930 | 0.035 s | about 33 MB |
| mut_absent | `present and value <= 1000` | FAILED, same check | 930 | well under 1 s | about 33 MB |
| undefined_unguarded | claim `value(x) <= 1000`, no guard | FAILED, same check (oracle stops `Undefined`) | 930 | well under 1 s | about 33 MB |
| comp_ok | composite, `!present or n <= 1000` | SUCCESSFUL, cover satisfied | 367 | 0.023 s | about 32 MB |
| comp_mutant | composite, `n < 1000` | FAILED: "composite agrees" | 367 | well under 1 s | about 34 MB |

Counterexample playback (the first vector of each failure; the second is the satisfied cover's
witness, not a counterexample): mut_present presence 1, payload 1000; mut_absent presence 0,
payload 0; undefined_unguarded presence 0, payload 0; comp_mutant presence 1, `n` 1000.

Every form verified or was falsified in well under the 10 minute probe limit, so no option form
here needs a tractability refusal. The refusals of FR-055 are by shape (payload family, nesting,
null state, unrecognised operation), not by cost.

## What this does not establish

- Not CG generation: the harness, oracle and subject are hand-written, the quire-exact option
  kernel and its meter were not used, and `option.value` of an absent option is modelled as
  `Undefined` by assumption (the kernel outcome is the authority, FR-055).
- Not CG's runner: no two-phase run, no identity record.
- Not scaled: the probe has one 0..=1000 payload, not a clause with arithmetic over an option, a
  large record payload or several options; those costs are unmeasured.
- The cgroup peak is a lower bound; the 16 GiB cap was not exercised.
