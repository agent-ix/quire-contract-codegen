// SPDX-License-Identifier: AGPL-3.0-or-later
//! Each legacy agreement vector retains its own genuine source/context and original scalar domains.
use qsl_replay::{AdmittedCompilation, DependencyInput, ReplayLimits, SourceIdentity, StageLimits};

pub const CASES: &[&str] = &[
    "record",
    "tuple",
    "option",
    "collection",
    "recursive",
    "nested",
    "text",
    "enum",
    "conv",
    "conv_charge",
    "conv_rat_rat",
    "conv_rat_int",
    "conv_dec_rat",
    "conv_dec_dec",
    "conv_dec_int",
];

pub fn source(case: &str) -> String {
    let (declarations, left, right, target) = match case {
        "record" => ("record Point { x: Integer; y: Integer; }\nrecord OtherPoint { x: Integer; y: Integer; }\n", "Point", "Point", None),
        "tuple" => ("tuple Pair(Int[-100, 100], Text[0, 16; nfc]);\n", "Pair", "Pair", None),
        "option" => ("", "Option<Int[-100, 100]>", "Option<Int[-100, 100]>", None),
        "collection" => ("", "Sequence<Int[-100, 100]>[0, 8]", "Sequence<Int[-100, 100]>[0, 8]", None),
        "recursive" => ("record SelfRecord { next: SelfRecord?; }\n", "SelfRecord", "SelfRecord", None),
        "nested" => ("record Point { x: Integer; y: Integer; }\nrecord PairOfPoints { a: Point; b: Point; }\n", "PairOfPoints", "PairOfPoints", None),
        "text" => ("", "Text[0, 16; nfc]", "Text[0, 16; nfc]", None),
        "enum" => ("ordered enum Phase { OPEN, SHUT }\n", "Phase", "Phase", None),
        "conv" => ("", "Int[-100, 100]", "Integer", Some("Integer")),
        "conv_charge" => ("", "Int[-100, 100]", "Decimal[-100, 100; 0, 0; nearest-even]", Some("Decimal[-100, 100; 0, 0; nearest-even]")),
        "conv_rat_rat" => ("", "Rational[-10, 10; 1, 1]", "Rational[-100, 100; 1, 5]", Some("Rational[-100, 100; 1, 5]")),
        "conv_rat_int" => ("", "Rational[-50, 50; 1, 1]", "Integer", Some("Integer")),
        "conv_dec_rat" => ("", "Decimal[-100, 100; 0, 0; nearest-even]", "Rational[-100, 100; 1, 5]", Some("Rational[-100, 100; 1, 5]")),
        "conv_dec_dec" => ("", "Decimal[-100, 100; 0, 0; nearest-even]", "Decimal[-1000, 1000; 0, 2; nearest-even]", Some("Decimal[-1000, 1000; 0, 2; nearest-even]")),
        "conv_dec_int" => ("", "Decimal[-100, 100; 0, 0; nearest-even]", "Integer", Some("Integer")),
        other => panic!("unknown agreement source case {other}"),
    };
    let left_expression =
        target.map_or_else(|| "a".to_owned(), |target| format!("convert<{target}>(a)"));
    format!("language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\n{declarations}function f using v(a: {left},b: {right}): Boolean pure {{ {left_expression} = b }}\n")
}

pub fn compile(case: &str) -> AdmittedCompilation {
    assert!(
        CASES.contains(&case),
        "the retained source case is declared"
    );
    compile_source(&source(case))
}

pub fn compile_source(source: &str) -> AdmittedCompilation {
    qsl_replay::compile_admitted(
        qsl_replay::CompileUnit {
            source: SourceIdentity::new("test", "composite-agreement.native", "git", "fixture"),
            package_name: "composite-agreement".to_owned(),
            path: "composite-agreement.native",
            bytes: source.as_bytes(),
        },
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("the original agreement source must admit before any key is used")
}
