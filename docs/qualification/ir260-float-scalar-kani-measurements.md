# IR-260 IEEE float scalar Kani measurements

Target `x86_64-unknown-linux-gnu`, `--solver cadical`, unwind 2 (the comparison path has no loop; 2
is above its largest loop bound of 0 plus 1). Every option is the harness's own vector, recorded in
`identity.options` and in the run evidence's `arguments`: `-Z function-contracts --harness
<module>::<proof> --exact --unwind 2 --solver cadical --output-format regular`, then the runner's
`-Z unstable-options --export-json <report>`. No tool versions are recorded in this evidence
(QSL-406, EA-49). Source: `tests/it/float_scalar_harness.rs`, one `#[ignore]` test per verdict, each
run in its own `systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0` under one `flock
/tmp/cargo-slot.lock`, one job at a time, a 30 minute hard cap. Wall is the whole test process (crate
build plus proof); memory.peak is the scope cgroup's, which includes the test process and `cargo
kani`. Every job ended well inside the 10 minute probe cap.

## Corpus

A V1-draft QSL source (`quire.value.complete/v1`), functions over `Float64[nearest-even]` and
`Float32[nearest-even]` parameters whose body is `quire::value::ieee::numericEqual(a, b)`,
`totalOrder(a, b)` or `bitIdentical(a, b)`, compiled by QSL, admitted by Contract IR, lowered through
`generate_routed` by CG. QSL main at the time of the run (`f32450ae9`) admits float parameters, the operators `=` and `!=`
(lowered to `quire.op.ieee.numeric_equal`), the three relation intrinsics, `+ - * /` over one
width, and `convert` to `Rational` (its `qsl-replay` tests `compile_ieee_profile` and the fixtures
under `qsl-replay/tests/fixtures/ieee_float`). A float ordering operator lowers to three
`numeric_equal` nodes and one `total_order`, so it is not one row node. A float literal enters as a
bit-constructor value node and is not a drawn parameter: this row refuses such an operand with
`float.operand_not_parameter`. The `ir260-float-source` branch of QSL (a commutativity case through
`convert<Rational[...]>(a + b)`) is not on QSL main, and its operation is float addition, which this
row refuses. The QSL-948 source admission the ticket comments name has therefore landed for the
comparison case run here; no synthetic IR is used anywhere in this evidence.

## Verdicts

The harness draws each operand as one `i64` bit pattern (every `i64` for `float64`, `0..=0xFFFF_FFFF`
for `float32`), builds `quire_exact::NativeFloat::binary64(bits as u64)` or `binary32(bits as u32)`,
calls `compare_native_float` under a meter that never denies and compares the Boolean with an
expected side that reads the bit patterns directly (an exponent and fraction classification for
`numeric_equal`, bit equality for `bit_identical`, the host's `total_cmp` for `total_order`).

| Case | Verdict | Wall (s) | memory.peak (MB) |
| --- | --- | --- | --- |
| numeric_equal, float64 | Verified | 3.5 | 348 |
| total_order, float64 | Verified | 3.3 | 335 |
| bit_identical, float64 | Verified | 7.1 | 753 |
| numeric_equal, float32 | Verified | 5.3 | 352 |
| total_order, float32 | Verified | 3.3 | 333 |
| bit_identical, float32 | Verified | 3.0 | 326 |
| numeric_equal float64 asked as bit_identical | Falsified (counterexample) | 18.5 | 3131 |
| bit_identical float64 asked as numeric_equal | Falsified (counterexample) | 27.1 | 4891 |
| total_order float32 asked as bit_identical | Falsified (counterexample) | 21.5 | 3766 |
| numeric_equal float32, expected side with a reflexive NaN | Falsified (counterexample) | 28.0 | 5106 |
| numeric_equal float64 asked as bit_identical, decoded and replayed through the owner route | Falsified (counterexample) | 18.8 | 3132 |

Each mutant asserts that the replaced text occurs in the routed harness and that the replacement
changed it. The replayed counterexample is decoded to exact `u64` bit patterns and evaluated by QSL's
`replay` over `WitnessValue::Float64`; the owner's answer is numeric equality and differs from the
mutant's bit identity on that witness.

## Not tractable and refused, no harness

Every arithmetic identity of the row (`quire.op.ieee.float32|float64` `add`, `sub`, `mul`, `div`,
`sqrt`, `fma`) is a typed refusal naming its capability (`float.<op>.not_kani_tractable`) with no
harness. The measured basis is quire-exact FR-405's evidence: an arithmetic operation is a supported
row only when its three harnesses (`nearest-even`, `exact`, and the four directed spellings) are
all Verified. Only the `nearest-even` harnesses of `add` and `sub` at both widths and of `mul` at
`binary32` are Verified; every `exact` and directed harness, and every harness of `div`, `sqrt` and
`fma`, reached no verdict within the 600 s limit (18 of the 31 above the 16 GiB ceiling). The five
rows are in `ir1530-general-route-scalar-table.json` (`add`/`sub`, `mul`, `div`, `sqrt`, `fma`, both
widths, `max_operand_bits` 32).

Kani symbolic `f32` division finding. Kani's model of the host `f32` division is not bit-exact: at
the operands `0x800000A9 / 0x9C873333` it returns a value other than the hardware's and the
correctly rounded `0x1BA00000`. A host `/` is therefore not usable as the expected side of a `div`
harness, and the bracketing of the quotient in exact integers that quire-exact's `div` harnesses use
is solver-bound (no `div` verdict at either width). This is recorded against IR-260 and IR-1526: the
`div` row in the tractability table, and the refusal detail of `float.div.not_kani_tractable`, carry
it.
