// SPDX-License-Identifier: AGPL-3.0-or-later
//! Execute reconstructed union operands with their original admitted member bindings.
mod generated {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/names.rs"));
}

#[ix_trace_rs::trace("FR-018-AC-2", "FR-018-AC-8", "FR-018-AC-17", "TC-029")]
#[test]
fn reconstructed_union_members_execute_equality() {
    let compiled = qsl_replay::compile_admitted(
        qsl_replay::CompileUnit {
            source: qsl_replay::SourceIdentity::new("test", "union.native"),
            package_name: "union-identifier".to_owned(),
            path: "union.native",
            bytes: include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/original.native")),
        },
        [],
        &qsl_replay::DependencyInput::default(),
        qsl_replay::StageLimits::default(),
        qsl_replay::ReplayLimits::default(),
    )
    .expect("same original union context");
    let prepared = generated::environment_shape(Some(compiled.context()))
        .expect("rebuild union members using validated Identifier values");
    let declaration = compiled
        .context()
        .composites()
        .find(|item| item.union_members().is_some())
        .expect("original union");
    let values = declaration
        .union_members()
        .unwrap()
        .iter()
        .map(|member| {
            let payload = if member.payload().is_empty() {
                vec![]
            } else {
                vec![quire_exact::Value::Boolean(true)]
            };
            quire_exact::UnionValue::from_admitted(member.member().clone(), payload)
        })
        .collect::<Vec<_>>();
    for (left, right, expected) in [
        (&values[0], &values[0], true),
        (&values[1], &values[1], true),
        (&values[0], &values[1], false),
    ] {
        let mut meter = quire_exact::Meter::new(quire_exact::ScalarLimits {
            integer_bits: u64::MAX,
            decimal_digits: u64::MAX,
            scale_expansion: u64::MAX,
            text_input_bytes: u64::MAX,
            text_scalars: u64::MAX,
            normalized_scalars: u64::MAX,
            unit_edges: u64::MAX,
            value_occurrences: u64::MAX,
            work_units: u64::MAX,
            result_units: u64::MAX,
        });
        assert_eq!(
            generated::oracle_shape(&prepared, left, right, &mut meter),
            quire_exact::Outcome::Completed(expected)
        );
    }
}
