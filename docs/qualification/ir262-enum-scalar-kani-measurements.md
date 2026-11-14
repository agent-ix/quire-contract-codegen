# IR-262 enum scalar Kani measurements

Kani 0.68.0, CBMC 6.11.0, `--solver cadical`, unwind 1, rustc stable, quire-exact `b81e5950`
(native enum carrier, FR-407). Every option is the harness's own vector, recorded in its
`identity.options` (`-Z function-contracts --harness <module>::<proof> --exact --unwind 1
--solver cadical --output-format regular`, then the runner's `-Z unstable-options --export-json`).
Source: `tests/it/enum_scalar_harness.rs`, one `#[ignore]` test per verdict, each run in its own
`systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0` under one `flock
/tmp/cargo-slot.lock`. Wall is the whole test process (crate build plus proof); memory.peak is the
scope cgroup's, which includes the test process and `cargo kani`.

Corpus: `ordered enum Level { LOW, MID, HIGH }` and `enum Color { RED, GREEN, BLUE }`
(the owner admits an unordered enum's members in sorted order), functions `a < b` and the rest,
compiled by QSL, admitted by Contract IR, lowered by CG with the owner's admitted context.

| Case | Verdict | Wall (s) | memory.peak (MB) |
| --- | --- | --- | --- |
| lt, le, gt, ge, eq, ne over Level | Verified | 3.1 to 3.4 | 307 to 321 |
| eq, ne over Color | Verified | 3.2 | 309 |
| lt rendered as le, le as lt, gt as ge, ge as gt | Falsified (counterexample) | 14.8 to 15.0 | 2057 |
| eq rendered as ne, ne as eq | Falsified (counterexample) | 12.3 | 1563 |
| reversed ordering (lt, le, gt, ge) | Falsified (counterexample) | 14.8 to 15.1 | 2057 |
| lt with an exhausted work budget (Incomplete) | Falsified | 14.8 | 1496 |

Tractability: the enum row is tractable (seconds, about 0.3 GB healthy). The measured table of
IR-1526 refuses not-tractable shapes keyed by an integer route, so no enum row is added to it.
