//! IR-1526 AC-10 measurement probes: the `#[ignore]`d installed-Kani probes and their host tests.
//! This module's path holds none of the `Makefile` Kani-lane filters, so `make kani` never
//! selects the probes; run one by name:
//!
//! - general route: `IR1526_PROBE_OP=<mul|add> IR1526_PROBE_BITS=<15|23|31> cargo test --test it
//!   ir1526_probe_wide_integer_multiply_under_installed_kani -- --ignored --nocapture`
//! - native route: `IR1526_NATIVE_OP=<add|mul|chain> IR1526_NATIVE_BITS=<10|15|23|31> cargo test
//!   --test it ir1526_probe_native_route_under_installed_kani -- --ignored --nocapture`

use std::{
    fs,
    process::Command,
    time::{Duration, Instant},
};

use quire_contract_codegen::{
    KaniObligationOutcome, KaniObligationRequest, KaniRunOutcome, KaniV2ClauseObligationHarness,
    ObligationItem, ProofCeilings,
};

use super::kani_obligations_state_frame::native_twin::Twin;
use super::kani_obligations_v2_clause_controls::{
    run_installed, subject_module, ScratchCrate, SUBJECT,
};
use crate::scratch_crate::TemporaryDirectory;

/// The operand widths the AC-10 probe can measure, in unsigned bits as FR-056 Terms count them:
/// width `n` is `Int[0, 2^n - 1]`. `IR1526_PROBE_BITS` picks one per run.
const IR1526_PROBE_WIDTHS: [u32; 3] = [15, 23, 31];

/// The wall-clock ceiling of the probe run.
const IR1526_PROBE_CEILINGS: ProofCeilings = ProofCeilings {
    memory_bytes: std::num::NonZeroU64::new(24 * 1024 * 1024 * 1024).unwrap(),
    wall_clock: Duration::from_secs(600),
};

fn probe_bits() -> u32 {
    let bits = std::env::var("IR1526_PROBE_BITS")
        .ok()
        .map_or(IR1526_PROBE_WIDTHS[0], |value| {
            value.parse().expect("IR1526_PROBE_BITS is a number")
        });
    assert!(
        IR1526_PROBE_WIDTHS.contains(&bits),
        "IR1526_PROBE_BITS must be one of {IR1526_PROBE_WIDTHS:?}"
    );
    bits
}

/// The operation the probe clause is built around; `IR1526_PROBE_OP` picks one per run.
#[derive(Clone, Copy)]
enum ProbeOp {
    Mul,
    Add,
}

impl ProbeOp {
    fn from_env() -> Self {
        match std::env::var("IR1526_PROBE_OP").as_deref() {
            Err(_) | Ok("mul") => Self::Mul,
            Ok("add") => Self::Add,
            Ok(other) => panic!("IR1526_PROBE_OP must be mul or add, got {other}"),
        }
    }

    fn operation_id(self) -> &'static str {
        match self {
            Self::Mul => "quire.op.integer.mul",
            Self::Add => "quire.op.integer.add",
        }
    }

    /// The clause; the add control holds no multiply.
    pub(crate) fn condition(self) -> &'static str {
        match self {
            Self::Mul => "other * offset + 3 <= 100",
            Self::Add => "other + offset <= 100",
        }
    }
}

/// The probe clause with both operands `Int[0, 2^bits - 1]`, negotiated on the general route
/// with an empty tractability table.
fn probe_harness(op: ProbeOp, bits: u32) -> KaniV2ClauseObligationHarness {
    use quire_contract_codegen::{
        negotiate_kani_obligations_with_options, GenerationOptions, IntegerRoute,
    };
    let scratch = TemporaryDirectory::new("ir1526-probe-table");
    let table = scratch.path.join("table.json");
    fs::write(&table, b"[]").unwrap();
    let upper = (1_i64 << bits) - 1;
    let (package, clause) =
        Twin::with_two_parameter_precondition((0, upper), (0, upper), op.condition())
            .emitted_package("TwoParameters");
    let outcome = negotiate_kani_obligations_with_options(
        &KaniObligationRequest {
            ceilings: IR1526_PROBE_CEILINGS,
            items: &[ObligationItem::V2Clause {
                package: &package,
                clause: &clause,
                claim: &crate::common::v2_claim::claim_for(&package, &clause),
                dependencies: None,
            }],
            subject_path: SUBJECT,
            unwind: 3,
        },
        &GenerationOptions {
            tractability_table: Some(table),
        },
        IntegerRoute::General,
    )
    .expect("the request negotiates");
    assert!(matches!(
        &outcome.records()[0].disposition,
        quire_contract_codegen::ObligationDisposition::Supported { .. }
    ));
    let KaniObligationOutcome::Emitted {
        mut v2_clause_harnesses,
        ..
    } = outcome
    else {
        panic!("the request is valid");
    };
    assert_eq!(v2_clause_harnesses.len(), 1, "one contract harness");
    v2_clause_harnesses.remove(0)
}

#[test]
fn the_wide_probe_clauses_negotiate_supported_with_one_harness_at_every_width() {
    for op in [ProbeOp::Mul, ProbeOp::Add] {
        for bits in IR1526_PROBE_WIDTHS {
            let harness = probe_harness(op, bits);
            let source = &harness.rust.contents;
            assert!(
                source.contains(&format!("<= {}_i64", (1_i64 << bits) - 1)),
                "the {bits}-bit bound is rendered"
            );
            assert_eq!(
                source.contains("evaluate_integer_arithmetic(rt::IntegerArithmetic::Multiply"),
                matches!(op, ProbeOp::Mul),
                "only the multiply clause holds a multiply"
            );
        }
    }
}

/// Measures, never asserts a verdict: the AC-10 wide integer probe. `IR1526_PROBE_OP` is `mul`
/// (default, `other * offset + 3 <= 100`) or `add` (the control, `other + offset <= 100`, which
/// holds no multiply). Measured on terra (Kani 0.68.0, CBMC 6.11.0, generator 0.1.0, kernel 1,
/// 24 GiB lane), op `mul`: operands of 15, 23 and 31 unsigned bits each ended `TimedOut` at the
/// 600 s ceiling (wall 600.0 s, 600.1 s, 600.1 s). Control, op `add` at 15 bits (no multiply):
/// `Inconclusive(FailedWithoutCounterexample)` after 542.5 s, so an add-only general-route clause
/// does not verify either. The runner reported no peak memory.
#[test]
#[ignore = "measurement probe, run by name (no Kani lane): IR1526_PROBE_BITS=<15|23|31> IR1526_PROBE_OP=<mul|add>, one run each, serially"]
fn ir1526_probe_wide_integer_multiply_under_installed_kani() {
    use quire_contract_codegen::{GENERATOR_VERSION, KERNEL_VERSION};
    let op = ProbeOp::from_env();
    let bits = probe_bits();
    let harness = probe_harness(op, bits);
    // A precondition harness reaches no subject, so the crate is the harness alone.
    let crate_scratch = ScratchCrate::new("ir1526-probe", &harness.rust.contents, None);
    let started = Instant::now();
    let run = run_installed(&crate_scratch, &harness);
    let wall = started.elapsed();
    let verdict = match &run.evidence.outcome {
        KaniRunOutcome::Verified => "Verified".to_owned(),
        KaniRunOutcome::Falsified { .. } => "Falsified".to_owned(),
        KaniRunOutcome::Inconclusive { reason } => format!("{reason:?}"),
        other => format!("{other:?}"),
    };
    let kani_version = Command::new("cargo")
        .args(["kani", "--version"])
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .unwrap_or_default();
    println!(
        "IR1526-PROBE op={} bits={bits} verdict={verdict} wall={:.1}s kani={kani_version} generator={GENERATOR_VERSION} kernel={KERNEL_VERSION}",
        op.operation_id(),
        wall.as_secs_f64()
    );
}

/// The native-route probe widths: unsigned operand bits `n` of `Int[0, 2^n - 1]`. The native
/// carrier is signed, so FR-056 counts it as `n` bits. `IR1526_NATIVE_BITS` picks one.
pub(crate) const IR1526_NATIVE_WIDTHS: [u32; 4] = [10, 15, 23, 31];

/// The operation of a native-route probe clause; `IR1526_NATIVE_OP` picks one per run. The native
/// route renders no precondition with arithmetic and no two-parameter clause, so each clause is a
/// Boolean-result postcondition `result = (<condition>)` over the one operand `amount`, the form
/// the native route renders arithmetic in; the subject returns the same expression.
#[derive(Clone, Copy)]
pub(crate) enum NativeOp {
    Add,
    Mul,
    Chain,
}

impl NativeOp {
    pub(crate) const ALL: [Self; 3] = [Self::Add, Self::Mul, Self::Chain];

    fn from_env() -> Self {
        match std::env::var("IR1526_NATIVE_OP").as_deref() {
            Err(_) | Ok("add") => Self::Add,
            Ok("mul") => Self::Mul,
            Ok("chain") => Self::Chain,
            Ok(other) => panic!("IR1526_NATIVE_OP must be add, mul or chain, got {other}"),
        }
    }

    pub(crate) fn corpus_name(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Mul => "multiply",
            Self::Chain => "chain",
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Mul => "mul",
            Self::Chain => "chain",
        }
    }

    /// The clause: an add, a multiply with no add, and a multiply then an add.
    pub(crate) fn condition(self) -> &'static str {
        match self {
            Self::Add => "amount + 3 <= 100",
            Self::Mul => "amount * amount <= 100",
            Self::Chain => "amount * amount + 3 <= 100",
        }
    }
}

fn native_bits() -> u32 {
    let bits = std::env::var("IR1526_NATIVE_BITS")
        .ok()
        .map_or(IR1526_NATIVE_WIDTHS[0], |value| {
            value.parse().expect("IR1526_NATIVE_BITS is a number")
        });
    assert!(
        IR1526_NATIVE_WIDTHS.contains(&bits),
        "IR1526_NATIVE_BITS must be one of {IR1526_NATIVE_WIDTHS:?}"
    );
    bits
}

pub(crate) fn native_probe_clause(
    op: NativeOp,
    bits: u32,
) -> (
    quire_contract_model::CheckedPackageV2,
    quire_contract_model::CheckedNodeId,
) {
    Twin::with_bounded_amount_result_postcondition((1_i64 << bits) - 1, op.condition())
        .emitted_package("BalanceNeverDrops")
}

/// The clause with its one operand `Int[0, 2^bits - 1]`, negotiated on the native route with an
/// empty tractability table.
fn native_probe_harness(op: NativeOp, bits: u32) -> KaniV2ClauseObligationHarness {
    use quire_contract_codegen::{
        negotiate_kani_obligations_with_options, GenerationOptions, IntegerRoute,
    };
    let scratch = TemporaryDirectory::new("ir1526-native-probe-table");
    let table = scratch.path.join("table.json");
    fs::write(&table, b"[]").unwrap();
    let (package, clause) = native_probe_clause(op, bits);
    let outcome = negotiate_kani_obligations_with_options(
        &KaniObligationRequest {
            ceilings: IR1526_PROBE_CEILINGS,
            items: &[ObligationItem::V2Clause {
                package: &package,
                clause: &clause,
                claim: &crate::common::v2_claim::claim_for(&package, &clause),
                dependencies: None,
            }],
            subject_path: SUBJECT,
            unwind: 3,
        },
        &GenerationOptions {
            tractability_table: Some(table),
        },
        IntegerRoute::Native,
    )
    .expect("the request negotiates");
    assert!(
        matches!(
            &outcome.records()[0].disposition,
            quire_contract_codegen::ObligationDisposition::Supported { .. }
        ),
        "{} at {bits} bits: {:?}",
        op.name(),
        outcome.records()[0].disposition
    );
    let KaniObligationOutcome::Emitted {
        mut v2_clause_harnesses,
        ..
    } = outcome
    else {
        panic!("the request is valid");
    };
    assert_eq!(v2_clause_harnesses.len(), 1, "one contract harness");
    v2_clause_harnesses.remove(0)
}

#[test]
fn the_native_probe_clauses_negotiate_supported_with_one_harness_at_every_width() {
    for op in NativeOp::ALL {
        for bits in IR1526_NATIVE_WIDTHS {
            let harness = native_probe_harness(op, bits);
            let source = &harness.rust.contents;
            assert!(
                source.contains(&format!("{}_i64", (1_i64 << bits) - 1)),
                "{} at {bits} bits renders its bound",
                op.name()
            );
        }
    }
}

/// Measures, never asserts a verdict: the AC-10 native-route probe. The harness is the generated
/// postcondition harness over the subject `check(amount) = <condition>`. `IR1526_NATIVE_OP` is `add`
/// (`other + 3 <= 100`), `mul` (`other * other <= 100`) or `chain` (`other * other + 3 <= 100`);
/// `IR1526_NATIVE_BITS` is the unsigned operand width of `Int[0, 2^n - 1]`.
#[test]
#[ignore = "measurement probe, run by name (no Kani lane): IR1526_NATIVE_BITS=<10|15|23|31> IR1526_NATIVE_OP=<add|mul|chain>, one run each, serially"]
fn ir1526_probe_native_route_under_installed_kani() {
    use quire_contract_codegen::{GENERATOR_VERSION, KERNEL_VERSION};
    let op = NativeOp::from_env();
    let bits = native_bits();
    let harness = native_probe_harness(op, bits);
    let lib = format!(
        "{}{}",
        harness.rust.contents,
        subject_module(op.condition())
    );
    let crate_scratch = ScratchCrate::new("ir1526-native-probe", &lib, None);
    let started = Instant::now();
    let run = run_installed(&crate_scratch, &harness);
    let wall = started.elapsed();
    let verdict = match &run.evidence.outcome {
        KaniRunOutcome::Verified => "Verified".to_owned(),
        KaniRunOutcome::Falsified { .. } => "Falsified".to_owned(),
        KaniRunOutcome::Inconclusive { reason } => format!("{reason:?}"),
        other => format!("{other:?}"),
    };
    let kani_version = Command::new("cargo")
        .args(["kani", "--version"])
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .unwrap_or_default();
    println!(
        "IR1526-PROBE op={} bits={bits} route=native clause=`{}` verdict={verdict} wall={:.1}s kani={kani_version} generator={GENERATOR_VERSION} kernel={KERNEL_VERSION}",
        harness_operation(op),
        op.condition(),
        wall.as_secs_f64()
    );
}

/// The catalog operation ids of the clause, as the corpus rows name them.
fn harness_operation(op: NativeOp) -> &'static str {
    match op {
        NativeOp::Add => "quire.op.integer.add",
        NativeOp::Mul => "quire.op.integer.mul",
        NativeOp::Chain => "quire.op.integer.mul+quire.op.integer.add",
    }
}
