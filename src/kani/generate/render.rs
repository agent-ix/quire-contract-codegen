//! Pure ordered emission of validated harness descriptions (FR-015).

use crate::{core::artifact::{Artifact, MAX_GENERATED_SOURCE_BYTES}, kani::abi::{i64_literal, KaniPrimitiveType}};
use super::{outcome::UnsupportedObligation, spec::{Argument, HarnessSpec, Init, LetBinding, LetType, ProofKind, Statement}};

fn line(source: &mut String, text: &str) {
    for part in text.split('\n') { source.push_str(part); source.push('\n'); }
}
fn binding(source: &mut String, binding: &LetBinding) {
    let mutable = if binding.mutable { "mut " } else { "" };
    let ty = match binding.ty { None => "", Some(LetType::I128) => ": i128" };
    let init = match &binding.init {
        Init::Expression(expression) => expression.as_str(),
        Init::UnlimitedMeter => "rt::Meter::new(rt::ScalarLimits {\ninteger_bits: u64::MAX,\ndecimal_digits: u64::MAX,\nscale_expansion: u64::MAX,\ntext_input_bytes: u64::MAX,\ntext_scalars: u64::MAX,\nnormalized_scalars: u64::MAX,\nunit_edges: u64::MAX,\nvalue_occurrences: u64::MAX,\nwork_units: u64::MAX,\nresult_units: u64::MAX,\n})",
    };
    line(source, &format!("let {mutable}{}{ty} = {init};", binding.name));
}
fn literal(ty: KaniPrimitiveType, value: i128) -> Result<String, UnsupportedObligation> {
    match ty {
        KaniPrimitiveType::I64 => i64::try_from(value).map(i64_literal).map_err(|error| UnsupportedObligation::InvalidGeneratedSyntax { error: error.to_string() }),
        KaniPrimitiveType::Boolean => Ok((value != 0).to_string()),
    }
}
pub(super) fn render(spec: &HarnessSpec) -> Result<Artifact, UnsupportedObligation> {
    let path = spec.path();
    let mut source = format!("// SPDX-License-Identifier: MIT OR Apache-2.0\n// Obligation: {}\n\n{}\n#[cfg(kani)]\nmod {} {{\nuse super::*;\n\n", spec.label(), spec.oracle_source(), path.module);
    match spec.proof() { ProofKind::Proof => source.push_str("#[kani::proof]\n") }
    source.push_str(&format!("fn {}() {{\n", path.harness));
    for statement in spec.body() {
        match statement {
            Statement::Input { name, ty, domain: (minimum, maximum), then } => {
                line(&mut source, &format!("        let {name}: {} = kani::any();", ty.source_name()));
                line(&mut source, &format!("kani::assume({name} >= {} && {name} <= {});", literal(*ty, *minimum)?, literal(*ty, *maximum)?));
                for item in then { binding(&mut source, item); }
            }
            Statement::Assume { condition } => line(&mut source, &format!("kani::assume({});", condition.as_str())),
            Statement::Let(item) => binding(&mut source, item),
            Statement::Call { result, callee, args } => {
                let args = args.iter().map(|arg| match arg { Argument::Borrow(name) => format!("&{name}"), Argument::BorrowMut(name) => format!("&mut {name}") }).collect::<Vec<_>>().join(", ");
                line(&mut source, &format!("let {result} = {callee}({args});"));
            }
            Statement::Assert { condition, message } => line(&mut source, &format!("assert!({}, \"{message}\");", condition.as_str())),
        }
    }
    line(&mut source, &format!("kani::cover!({}, \"{}\");", spec.cover().condition.as_str(), spec.cover().message));
    source.push_str("}\n}\n");
    if source.len() > MAX_GENERATED_SOURCE_BYTES { return Err(UnsupportedObligation::ResourceLimitExceeded { bytes: source.len() }); }
    syn::parse_file(&source).map_err(|error| UnsupportedObligation::InvalidGeneratedSyntax { error: error.to_string() })?;
    Ok(Artifact::new(format!("src/generated/{}.rs", path.module), source))
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::spec::{Cover, ExprText};
    use crate::{core::identity::{HarnessPath, HarnessSymbol, ModuleSymbol}, kani::abi::KaniSolver};
    use ix_trace_rs::trace;

    fn expression(text: &str) -> ExprText { ExprText::new(text).unwrap() }
    fn input(name: &str) -> Statement { Statement::Input { name: name.into(), ty: KaniPrimitiveType::I64, domain: (-1, 1), then: vec![] } }
    fn call() -> Statement { Statement::Call { result: "answer".into(), callee: "probe".into(), args: vec![Argument::Borrow("x".into())] } }
    fn make_spec(oracle: String, body: Vec<Statement>) -> HarnessSpec {
        HarnessSpec::new("independent control".into(), HarnessPath { module: ModuleSymbol::try_from("control").unwrap(), harness: HarnessSymbol::try_from("check").unwrap() }, oracle, ProofKind::Proof, body, vec![Cover { condition: expression("answer"), message: "a positive result is reachable".into() }], 2, KaniSolver::Cadical).unwrap()
    }
    fn oracle() -> String { "fn probe(x: &i64) -> bool { *x >= 0 }".into() }
    fn fixture() -> HarnessSpec { make_spec(oracle(), vec![input("x"), Statement::Assume { condition: expression("x != -1") }, call(), Statement::Assert { condition: expression("answer"), message: "nonnegative input".into() }]) }

    #[trace("FR-015-AC-113", "FR-015-AC-114", "FR-015-AC-116", "TC-025")]
    #[test]
    fn renderer_emits_only_fixed_structure_and_supplied_data_in_order() {
        let spec = fixture();
        let artifact = render(&spec).unwrap();
        assert_eq!(artifact.path, "src/generated/control.rs");
        assert_eq!(artifact.contents, "// SPDX-License-Identifier: MIT OR Apache-2.0\n// Obligation: independent control\n\nfn probe(x: &i64) -> bool { *x >= 0 }\n#[cfg(kani)]\nmod control {\nuse super::*;\n\n#[kani::proof]\nfn check() {\n        let x: i64 = kani::any();\nkani::assume(x >= -1_i64 && x <= 1_i64);\nkani::assume(x != -1);\nlet answer = probe(&x);\nassert!(answer, \"nonnegative input\");\nkani::cover!(answer, \"a positive result is reachable\");\n}\n}\n");
        assert_eq!(artifact, render(&spec).unwrap());
        for body in [vec![input("x"), input("y"), call()], vec![input("y"), input("x"), call()], vec![Statement::Assume { condition: expression("true") }, input("x"), call()], vec![input("x"), call(), Statement::Let(LetBinding { name: "copy".into(), ty: None, mutable: false, init: Init::Expression(expression("answer")) }), Statement::Assert { condition: expression("copy"), message: "copied result".into() }]] {
            let artifact = render(&make_spec(oracle(), body)).unwrap();
            let parsed = syn::parse_file(&artifact.contents).unwrap();
            let syn::Item::Mod(module) = parsed.items.last().unwrap() else { panic!("module wrapper"); };
            let syn::Item::Fn(proof) = module.content.as_ref().unwrap().1.last().unwrap() else { panic!("proof function"); };
            let syn::Stmt::Macro(last) = proof.block.stmts.last().unwrap() else { panic!("final cover"); };
            assert_eq!(last.mac.path.segments.last().unwrap().ident, "cover");
            assert_eq!(artifact.contents.matches("kani::cover!(").count(), 1);
        }
        let first = render(&make_spec(oracle(), vec![input("x"), input("y"), call()])).unwrap();
        let swapped = render(&make_spec(oracle(), vec![input("y"), input("x"), call()])).unwrap();
        assert_ne!(first.contents, swapped.contents);
        assert!(first.contents.find("let x:").unwrap() < first.contents.find("let y:").unwrap());
        assert!(swapped.contents.find("let y:").unwrap() < swapped.contents.find("let x:").unwrap());
    }

    #[trace("FR-015-AC-114", "TC-025")]
    #[test]
    fn rendering_is_identical_in_another_process() {
        const OUTPUT_ENV: &str = "IR858_RENDER_PROCESS_OUTPUT";
        if let Some(path) = std::env::var_os(OUTPUT_ENV) {
            std::fs::write(path, render(&fixture()).unwrap().contents).unwrap();
            return;
        }
        let directory = crate::kani::test_support::discover_scratch("render-determinism");
        let path = directory.join("output.rs");
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "kani::generate::render::tests::rendering_is_identical_in_another_process", "--test-threads=1"])
            .env(OUTPUT_ENV, &path).status().unwrap();
        assert!(status.success());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), render(&fixture()).unwrap().contents);
        std::fs::remove_dir_all(directory).unwrap();
    }

    #[trace("FR-015-AC-115", "TC-025")]
    #[test]
    fn renderer_refuses_size_before_invalid_syntax_with_exact_byte_count() {
        let valid = render(&make_spec(oracle(), vec![input("x"), call()])).unwrap();
        let overhead = valid.contents.len() - oracle().len();
        for oracle in [" ".repeat(MAX_GENERATED_SOURCE_BYTES), "?".repeat(MAX_GENERATED_SOURCE_BYTES), "é".repeat(MAX_GENERATED_SOURCE_BYTES)] {
            let bytes = oracle.len() + overhead;
            assert_eq!(render(&make_spec(oracle, vec![input("x"), call()])).unwrap_err(), UnsupportedObligation::ResourceLimitExceeded { bytes });
        }
        assert!(matches!(render(&make_spec("?".into(), vec![input("x"), call()])), Err(UnsupportedObligation::InvalidGeneratedSyntax { .. })));
        let at_limit = format!("{}{}", oracle(), " ".repeat(MAX_GENERATED_SOURCE_BYTES - valid.contents.len()));
        assert_eq!(render(&make_spec(at_limit, vec![input("x"), call()])).unwrap().contents.len(), MAX_GENERATED_SOURCE_BYTES);
    }

    #[trace("FR-015-AC-117", "TC-025")]
    #[test]
    #[ignore = "installed Kani backend: run this private control explicitly and serially"]
    fn unsatisfiable_assume_is_cover_unsatisfied_under_real_kani() {
        use crate::kani::{identity::{KaniScalarObligationHarness, ScalarObligationIdentity, ScalarObligationArgument}, run::{tool::KaniInstallation, execute::{execute_kani_obligation, KaniExecutionRequest}}, classify::KaniRunOutcome};
        // The safe addition is reached before the contradiction. Its overflow check gives
        // the classifier a successful property; the final cover is then unsatisfied.
        let spec = make_spec(oracle(), vec![input("x"), Statement::Let(LetBinding { name: "_shifted".into(), ty: None, mutable: false, init: Init::Expression(expression("x + 1_i64")) }), Statement::Assume { condition: expression("x < 0 && x >= 0") }, call()]);
        let rust = render(&spec).unwrap();
        let identity = ScalarObligationIdentity {
            ceilings: crate::kani::test_support::proof_ceilings::proof_ceilings(),
            node_id: serde_json::from_value(serde_json::json!({"domain":"quire.checked-semantic-node/v1", "digest":"1".repeat(64)})).unwrap(),
            operation_identity: "independent.control".into(), oracle_symbol: "probe".into(),
            module_symbol: spec.path().module.clone(), harness_symbol: spec.path().harness.clone(),
            arguments: vec![ScalarObligationArgument { identifier: "x".into(), minimum: -1, maximum: 1 }],
            inferred_operands: vec![], solver: spec.solver(), unwind: spec.unwind(), options: spec.options().to_vec(),
        };
        let record = super::super::record::record(spec.path().module.as_str(), &identity, &rust).unwrap();
        let harness = KaniScalarObligationHarness::generated(identity, rust, record);
        let directory = crate::kani::test_support::discover_scratch("render-unsat");
        std::fs::create_dir_all(directory.join("src")).unwrap();
        std::fs::write(directory.join("Cargo.toml"), "[package]\nname = \"render-unsat-control\"\nversion = \"0.0.0\"\nedition = \"2021\"\n[workspace]\n").unwrap();
        std::fs::write(directory.join("src/lib.rs"), &harness.rust.contents).unwrap();
        std::fs::write(directory.join("build.rs"), "fn main() { println!(\"cargo:rustc-check-cfg=cfg(kani)\"); }\n").unwrap();
        let installation = KaniInstallation::discover().expect("installed cargo-kani");
        let evidence = execute_kani_obligation(&KaniExecutionRequest { cancel: &quire_exact::Cancel::new(), installation: &installation, harness: (&harness).into(), crate_directory: &directory, target_directory: &directory.join("target"), home_directory: None }).unwrap();
        assert!(evidence.success_checks > 0, "a reachable pre-assumption property must succeed: {evidence:?}");
        assert_eq!(evidence.outcome, KaniRunOutcome::CoverUnsatisfied { satisfied: 0, total: 1 });
        std::fs::remove_dir_all(directory).unwrap();
    }
}
