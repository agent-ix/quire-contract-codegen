//! Pure ordered emission of validated harness descriptions (FR-015).

use super::{
    outcome::UnsupportedObligation,
    spec::{
        Argument, CollectionElement, CollectionSpec, FrameAssertion, FrameSpec, HarnessShape,
        HarnessSpec, Init, LetBinding, LetType, ProofKind, Statement,
    },
};
use crate::{
    core::artifact::{Artifact, MAX_GENERATED_SOURCE_BYTES},
    kani::{
        abi::{i64_literal, KaniPrimitiveType},
        identity::CollectionKindIdentity,
    },
};

fn line(source: &mut String, text: &str) {
    for part in text.split('\n') {
        source.push_str(part);
        source.push('\n');
    }
}
fn binding(source: &mut String, binding: &LetBinding) {
    let mutable = if binding.mutable { "mut " } else { "" };
    let ty = match binding.ty {
        None => "",
        Some(LetType::I128) => ": i128",
    };
    let init = match &binding.init {
        Init::Expression(expression) => expression.as_str().to_owned(),
        Init::NativeAdmission { checked, stage } => format!(
            "match {} {{ Ok(value) => value, Err(_) => panic!(\"{}\") }}",
            checked.as_str(),
            stage.message(),
        ),
        Init::ScalarMeter(limits) => {
            let fields = [
                ("integer_bits", limits.integer_bits),
                ("decimal_digits", limits.decimal_digits),
                ("scale_expansion", limits.scale_expansion),
                ("text_input_bytes", limits.text_input_bytes),
                ("text_scalars", limits.text_scalars),
                ("normalized_scalars", limits.normalized_scalars),
                ("unit_edges", limits.unit_edges),
                ("value_occurrences", limits.value_occurrences),
                ("work_units", limits.work_units),
                ("result_units", limits.result_units),
            ];
            let mut text = "rt::Meter::new(rt::ScalarLimits {\n".to_owned();
            for (name, value) in fields {
                let literal = if value == u64::MAX {
                    "u64::MAX".to_owned()
                } else {
                    value.to_string()
                };
                text.push_str(&format!("{name}: {literal},\n"));
            }
            text.push_str("})");
            text
        }
    };
    line(
        source,
        &format!("let {mutable}{}{ty} = {init};", binding.name),
    );
}
fn literal(ty: KaniPrimitiveType, value: i128) -> Result<String, UnsupportedObligation> {
    match ty {
        KaniPrimitiveType::I64 => i64::try_from(value).map(i64_literal).map_err(|error| {
            UnsupportedObligation::InvalidGeneratedSyntax {
                error: error.to_string(),
            }
        }),
        KaniPrimitiveType::Boolean => Ok((value != 0).to_string()),
    }
}
pub(super) fn render(spec: &HarnessSpec) -> Result<Artifact, UnsupportedObligation> {
    match spec.shape() {
        HarnessShape::Statements {
            label,
            oracle_source,
            proof,
            body,
        } => render_statements(spec, label, oracle_source, *proof, body),
        HarnessShape::Collection {
            collection,
            precondition,
        } => render_collection(spec, collection, *precondition),
        HarnessShape::Frame(frame) => render_frame(spec, frame),
    }
}

fn render_frame(spec: &HarnessSpec, frame: &FrameSpec) -> Result<Artifact, UnsupportedObligation> {
    let path = spec.path();
    let fields = frame
        .state_fields
        .iter()
        .map(|field| format!("{field}: kani::any()"))
        .collect::<Vec<_>>()
        .join(", ");
    let mut source = format!(
        "{}#[cfg(kani)]\nmod {} {{\n    use super::*;\n\n    #[kani::proof]\n    fn {}() {{\n        let pre: {} = {} {{ {fields} }};\n",
        frame.prefix, path.module, path.harness, frame.state_path, frame.state_path,
    );
    for domain in &frame.domains {
        source.push_str(&format!(
            "        kani::assume(pre.{} >= {} && pre.{} <= {});\n",
            domain.field,
            i64_literal(domain.minimum),
            domain.field,
            i64_literal(domain.maximum),
        ));
    }
    source.push_str(&format!(
        "        let mut post = pre.clone();\n        {}(&mut post);\n",
        frame.subject_path
    ));
    match &frame.assertion {
        FrameAssertion::Postcondition {
            field,
            operator,
            left_is_pre,
            message,
        } => {
            let (left, right) = if *left_is_pre {
                ("pre", "post")
            } else {
                ("post", "pre")
            };
            source.push_str(&format!(
                "        assert!(\n            {left}.{field} {operator} {right}.{field},\n            \"{{}}\",\n            {message:?}\n        );\n"
            ));
        }
        FrameAssertion::Effects(checked) => {
            for (field, message) in checked {
                source.push_str(&format!(
                    "        assert!(post.{field} == pre.{field}, \"{{}}\", {message:?});\n"
                ));
            }
        }
    }
    source.push_str(&format!(
        "        kani::cover!({}, {:?});\n    }}\n}}\n",
        spec.cover().condition.as_str(),
        spec.cover().message
    ));
    if source.len() > MAX_GENERATED_SOURCE_BYTES {
        return Err(UnsupportedObligation::ResourceLimitExceeded {
            bytes: source.len(),
        });
    }
    syn::parse_file(&source).map_err(|error| UnsupportedObligation::InvalidGeneratedSyntax {
        error: error.to_string(),
    })?;
    Ok(Artifact::new(
        format!("src/generated/{}.rs", path.module),
        source,
    ))
}

fn render_statements(
    spec: &HarnessSpec,
    label: &str,
    oracle_source: &str,
    proof: ProofKind,
    body: &[Statement],
) -> Result<Artifact, UnsupportedObligation> {
    let path = spec.path();
    let mut source = format!("// SPDX-License-Identifier: MIT OR Apache-2.0\n// @generated by quire-contract-codegen. Do not edit.\n// Obligation: {}\n\n{}\n#[cfg(kani)]\nmod {} {{\nuse super::*;\n\n", label, oracle_source, path.module);
    match proof {
        ProofKind::Proof => source.push_str("#[kani::proof]\n"),
    }
    source.push_str(&format!("fn {}() {{\n", path.harness));
    for statement in body {
        match statement {
            Statement::Input {
                name,
                ty,
                domain: (minimum, maximum),
                then,
            } => {
                line(
                    &mut source,
                    &format!("        let {name}: {} = kani::any();", ty.source_name()),
                );
                line(
                    &mut source,
                    &format!(
                        "kani::assume({name} >= {} && {name} <= {});",
                        literal(*ty, *minimum)?,
                        literal(*ty, *maximum)?
                    ),
                );
                for item in then {
                    binding(&mut source, item);
                }
            }
            Statement::Assume { condition } => line(
                &mut source,
                &format!("kani::assume({});", condition.as_str()),
            ),
            Statement::Let(item) => binding(&mut source, item),
            Statement::Call {
                result,
                callee,
                args,
            } => {
                let args = args
                    .iter()
                    .map(|arg| match arg {
                        Argument::Value(name) => name.to_owned(),
                        Argument::BooleanOutcomeClosure(name) => {
                            format!("|| rt::Outcome::Completed({name})")
                        }
                        Argument::Borrow(name) => format!("&{name}"),
                        Argument::BorrowMut(name) => format!("&mut {name}"),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                line(&mut source, &format!("let {result} = {callee}({args});"));
            }
            Statement::SubjectImplicationCall => line(
                &mut source,
                "let subject = quire_contract_runtime::operators::implies_short_circuit(left, || right);",
            ),
            Statement::Assert { condition, message } => line(
                &mut source,
                &format!("assert!({}, \"{message}\");", condition.as_str()),
            ),
            Statement::ConditionalProbe { condition, body } => {
                line(&mut source, &format!("if {} {{", condition.as_str()));
                for item in body {
                    match item {
                        Statement::Let(binding) => self::binding(&mut source, binding),
                        Statement::Assert { condition, message } => line(
                            &mut source,
                            &format!("assert!({}, \"{message}\");", condition.as_str()),
                        ),
                        _ => {
                            return Err(UnsupportedObligation::InvalidGeneratedSyntax {
                                error: "invalid conditional probe statement".to_owned(),
                            });
                        }
                    }
                }
                line(&mut source, "}");
            }
        }
    }
    line(
        &mut source,
        &format!(
            "kani::cover!({}, \"{}\");",
            spec.cover().condition.as_str(),
            spec.cover().message
        ),
    );
    source.push_str("}\n}\n");
    if source.len() > MAX_GENERATED_SOURCE_BYTES {
        return Err(UnsupportedObligation::ResourceLimitExceeded {
            bytes: source.len(),
        });
    }
    syn::parse_file(&source).map_err(|error| UnsupportedObligation::InvalidGeneratedSyntax {
        error: error.to_string(),
    })?;
    Ok(Artifact::new(
        format!("src/generated/{}.rs", path.module),
        source,
    ))
}

fn render_collection(
    spec: &HarnessSpec,
    collection: &CollectionSpec,
    precondition: bool,
) -> Result<Artifact, UnsupportedObligation> {
    let path = spec.path();
    let quantifier = collection.quantifier.name();
    let collection_call = collection.quantifier.iterator_method();
    let item_name = &collection.item_name;
    let length_name = &collection.length_name;
    let minimum_length = collection.minimum_length;
    let maximum_length = collection.maximum_length;
    let array_len = collection.array_len;
    let uniqueness_assumption = match collection.kind {
        CollectionKindIdentity::Sequence => String::new(),
        CollectionKindIdentity::OrderedSet => format!(
            "                for prior in 0..index {{\n\
                     kani::assume(item != {item_name}[prior as usize]);\n\
                     }}\n"
        ),
    };
    let (element_type, element_assumption) = match collection.element {
        CollectionElement::Boolean => ("bool", String::new()),
        CollectionElement::Integer { minimum, maximum } => (
            "i64",
            format!("                kani::assume(item >= {minimum} && item <= {maximum});\n"),
        ),
    };
    let predicate = collection.predicate_expression();
    let (oracle_source, claim_source, evaluation, assertion) = if precondition {
        (
            format!(
                "             #[allow(dead_code)] enum ClauseEvaluation {{ Completed(bool), Undefined, Refused, Incomplete }}\n\
                 fn oracle_clause({item_name}: &[{element_type}; {array_len}], {length_name}: i64) -> ClauseEvaluation {{\n\
                     let claim = (0..{length_name}).{collection_call}(|index| {{\n\
                         let item = {item_name}[index as usize];\n\
                         {predicate}\n\
                     }});\n\
                     ClauseEvaluation::Completed(claim)\n\
                 }}\n"
            ),
            String::new(),
            format!("                 let outcome = oracle_clause(&{item_name}, {length_name});\n"),
            crate::kani::identity::COLLECTION_PRECONDITION_TOTALITY_ASSERTION.to_owned(),
        )
    } else {
        (
            String::new(),
            format!(
                "                 let claim = (0..{length_name}).{collection_call}(|index| {{\n\
                     let item = {item_name}[index as usize];\n\
                     {predicate}\n\
                 }});\n"
            ),
            String::new(),
            format!("assert!(claim, \"the symbolic collection satisfies the selected QSL {quantifier} body\");"),
        )
    };
    let source = format!(
        "// SPDX-License-Identifier: MIT OR Apache-2.0\n\
         // Claim: {quantifier} over admitted model field `{}`\n\
         #[cfg(kani)]\n\
         mod {} {{\n\
{oracle_source}\
             #[kani::proof]\n\
             fn {}() {{\n\
                 let {length_name}: i64 = kani::any();\n\
                 kani::assume({length_name} >= {minimum_length} && {length_name} <= {maximum_length});\n\
                 let {item_name}: [{element_type}; {array_len}] = kani::any();\n\
                 for index in 0..{length_name} {{\n\
                     let item = {item_name}[index as usize];\n\
{element_assumption}\
{uniqueness_assumption}\
                 }}\n\
{claim_source}\
{evaluation}\
                 {assertion}\n\
                 kani::cover!({}, \"{}\");\n\
             }}\n\
         }}\n",
        collection.field_name,
        path.module,
        path.harness,
        spec.cover().condition.as_str(),
        spec.cover().message,
    );
    if source.len() > MAX_GENERATED_SOURCE_BYTES {
        return Err(UnsupportedObligation::ResourceLimitExceeded {
            bytes: source.len(),
        });
    }
    syn::parse_file(&source).map_err(|error| UnsupportedObligation::InvalidGeneratedSyntax {
        error: error.to_string(),
    })?;
    Ok(Artifact::new(
        format!("src/generated/{}.rs", path.module),
        source,
    ))
}

#[cfg(test)]
pub(super) mod tests {
    use super::super::spec::{Cover, ExprText};
    use super::*;
    use crate::{
        core::identity::{HarnessPath, HarnessSymbol, ModuleSymbol},
        kani::abi::KaniSolver,
    };
    use ix_trace_rs::trace;

    fn expression(text: &str) -> ExprText {
        ExprText::new(text).unwrap()
    }
    fn input(name: &str) -> Statement {
        Statement::Input {
            name: name.into(),
            ty: KaniPrimitiveType::I64,
            domain: (-1, 1),
            then: vec![],
        }
    }
    fn call() -> Statement {
        Statement::Call {
            result: "answer".into(),
            callee: "probe".into(),
            args: vec![Argument::Borrow("x".into())],
        }
    }
    fn make_spec(oracle: String, body: Vec<Statement>) -> HarnessSpec {
        HarnessSpec::new(
            "independent control".into(),
            HarnessPath {
                module: ModuleSymbol::try_from("control").unwrap(),
                harness: HarnessSymbol::try_from("check").unwrap(),
            },
            oracle,
            ProofKind::Proof,
            body,
            vec![Cover {
                condition: expression("answer"),
                message: "a positive result is reachable".into(),
            }],
            2,
            KaniSolver::Cadical,
        )
        .unwrap()
    }
    fn oracle() -> String {
        "fn probe(x: &i64) -> bool { *x >= 0 }".into()
    }
    fn fixture() -> HarnessSpec {
        make_spec(
            oracle(),
            vec![
                input("x"),
                Statement::Assume {
                    condition: expression("x != -1"),
                },
                call(),
                Statement::Assert {
                    condition: expression("answer"),
                    message: "nonnegative input".into(),
                },
            ],
        )
    }

    #[trace(
        "FR-015-AC-113",
        "FR-015-AC-114",
        "FR-015-AC-116",
        "TC-025",
        "TC-001",
        "NFR-002-AC-1",
        "NFR-002-AC-2"
    )]
    #[test]
    fn renderer_emits_only_fixed_structure_and_supplied_data_in_order() {
        let spec = fixture();
        let artifact = render(&spec).unwrap();
        assert_eq!(artifact.path, "src/generated/control.rs");
        let header = artifact
            .contents
            .lines()
            .take_while(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(header.contains("// @generated by quire-contract-codegen"));
        assert!(header.contains("// SPDX-License-Identifier: MIT OR Apache-2.0"));
        assert!(header.contains("independent control"));
        assert!(!header
            .replacen("// @generated by quire-contract-codegen", "", 1)
            .contains("// @generated by quire-contract-codegen"));
        assert!(!header
            .replacen("// SPDX-License-Identifier: MIT OR Apache-2.0", "", 1)
            .contains("// SPDX-License-Identifier: MIT OR Apache-2.0"));
        assert!(!header
            .replace("independent control", "")
            .contains("independent control"));
        assert_eq!(artifact.contents, "// SPDX-License-Identifier: MIT OR Apache-2.0\n// @generated by quire-contract-codegen. Do not edit.\n// Obligation: independent control\n\nfn probe(x: &i64) -> bool { *x >= 0 }\n#[cfg(kani)]\nmod control {\nuse super::*;\n\n#[kani::proof]\nfn check() {\n        let x: i64 = kani::any();\nkani::assume(x >= -1_i64 && x <= 1_i64);\nkani::assume(x != -1);\nlet answer = probe(&x);\nassert!(answer, \"nonnegative input\");\nkani::cover!(answer, \"a positive result is reachable\");\n}\n}\n");
        assert_eq!(artifact, render(&spec).unwrap());
        for body in [
            vec![input("x"), input("y"), call()],
            vec![input("y"), input("x"), call()],
            vec![
                Statement::Assume {
                    condition: expression("true"),
                },
                input("x"),
                call(),
            ],
            vec![
                input("x"),
                call(),
                Statement::Let(LetBinding {
                    name: "copy".into(),
                    ty: None,
                    mutable: false,
                    init: Init::Expression(expression("answer")),
                }),
                Statement::Assert {
                    condition: expression("copy"),
                    message: "copied result".into(),
                },
            ],
        ] {
            let artifact = render(&make_spec(oracle(), body)).unwrap();
            let parsed = syn::parse_file(&artifact.contents).unwrap();
            let syn::Item::Mod(module) = parsed.items.last().unwrap() else {
                panic!("module wrapper");
            };
            let syn::Item::Fn(proof) = module.content.as_ref().unwrap().1.last().unwrap() else {
                panic!("proof function");
            };
            let syn::Stmt::Macro(last) = proof.block.stmts.last().unwrap() else {
                panic!("final cover");
            };
            assert_eq!(last.mac.path.segments.last().unwrap().ident, "cover");
            assert_eq!(artifact.contents.matches("kani::cover!(").count(), 1);
        }
        let first = render(&make_spec(oracle(), vec![input("x"), input("y"), call()])).unwrap();
        let swapped = render(&make_spec(oracle(), vec![input("y"), input("x"), call()])).unwrap();
        assert_ne!(first.contents, swapped.contents);
        assert!(first.contents.find("let x:").unwrap() < first.contents.find("let y:").unwrap());
        assert!(
            swapped.contents.find("let y:").unwrap() < swapped.contents.find("let x:").unwrap()
        );
    }

    #[trace("FR-015-AC-114", "TC-025")]
    #[test]
    fn rendering_is_identical_in_another_process() {
        const OUTPUT_ENV: &str = "IR858_RENDER_PROCESS_OUTPUT";
        if let Some(path) = std::env::var_os(OUTPUT_ENV) {
            std::fs::write(path, render(&fixture()).unwrap().contents).unwrap();
            return;
        }
        let directory = crate::core::test_support::discover_scratch("render-determinism");
        let path = directory.join("output.rs");
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "kani::generate::render::tests::rendering_is_identical_in_another_process",
                "--test-threads=1",
            ])
            .env(OUTPUT_ENV, &path)
            .status()
            .unwrap();
        assert!(status.success());
        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            render(&fixture()).unwrap().contents
        );
        std::fs::remove_dir_all(directory).unwrap();
    }

    #[trace("FR-015-AC-115", "TC-025")]
    #[test]
    fn renderer_refuses_size_before_invalid_syntax_with_exact_byte_count() {
        let valid = render(&make_spec(oracle(), vec![input("x"), call()])).unwrap();
        let overhead = valid.contents.len() - oracle().len();
        for oracle in [
            " ".repeat(MAX_GENERATED_SOURCE_BYTES),
            "?".repeat(MAX_GENERATED_SOURCE_BYTES),
            "é".repeat(MAX_GENERATED_SOURCE_BYTES),
        ] {
            let bytes = oracle.len() + overhead;
            assert_eq!(
                render(&make_spec(oracle, vec![input("x"), call()])).unwrap_err(),
                UnsupportedObligation::ResourceLimitExceeded { bytes }
            );
        }
        assert!(matches!(
            render(&make_spec("?".into(), vec![input("x"), call()])),
            Err(UnsupportedObligation::InvalidGeneratedSyntax { .. })
        ));
        let at_limit = format!(
            "{}{}",
            oracle(),
            " ".repeat(MAX_GENERATED_SOURCE_BYTES - valid.contents.len())
        );
        assert_eq!(
            render(&make_spec(at_limit, vec![input("x"), call()]))
                .unwrap()
                .contents
                .len(),
            MAX_GENERATED_SOURCE_BYTES
        );
    }

    pub(crate) fn unsatisfiable_harness() -> crate::kani::identity::KaniScalarObligationHarness {
        use crate::kani::identity::{
            KaniScalarObligationHarness, ScalarObligationArgument, ScalarObligationIdentity,
        };
        // The safe addition is reached before the contradiction. Its overflow check gives
        // the classifier a successful property; the final cover is then unsatisfied.
        let spec = make_spec(
            oracle(),
            vec![
                input("x"),
                Statement::Let(LetBinding {
                    name: "_shifted".into(),
                    ty: None,
                    mutable: false,
                    init: Init::Expression(expression("x + 1_i64")),
                }),
                Statement::Assume {
                    condition: expression("x < 0 && x >= 0"),
                },
                call(),
            ],
        );
        let rust = render(&spec).unwrap();
        let identity = ScalarObligationIdentity {
                integer_representation: crate::kani::identity::ScalarIntegerRepresentation::General,
                native_integer_selection: None,
            ceilings: crate::kani::test_support::proof_ceilings::proof_ceilings(),
            node_id: serde_json::from_value(serde_json::json!({"domain":"quire.checked-semantic-node/v1", "digest":"1".repeat(64)})).unwrap(),
            operation_identity: "independent.control".into(), oracle_symbol: "probe".into(),
            module_symbol: spec.path().module.clone(), harness_symbol: spec.path().harness.clone(),
            arguments: vec![ScalarObligationArgument { identifier: "x".into(), minimum: -1, maximum: 1 }],
            inferred_operands: vec![], solver: spec.solver(), unwind: spec.unwind(), options: spec.options().to_vec(),
        };
        let record =
            super::super::record::record(spec.path().module.as_str(), &identity, &rust).unwrap();
        KaniScalarObligationHarness::generated(identity, rust, record)
    }
}
