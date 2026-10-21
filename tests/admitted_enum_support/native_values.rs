//! Native agreement values from genuine owner declarations and retained source selection.
use qsl_replay::{AdmittedCompilation, AdmittedContext};
use quire_exact::{EnumMember, EnumShape, Value, ValueType};
use quire_semantic_value::enumeration::{EnumDeclaration, EnumMemberIndex, EnumValue};
use super::fixture::imported;

pub struct EnumData {
    pub shape: ValueType,
    pub index: EnumMemberIndex,
    pub values: Vec<Value>,
    pub members: Vec<EnumValue>,
    pub names: Vec<String>,
}

pub fn enum_data(context: &AdmittedContext, name: &str) -> EnumData {
    let admitted = context
        .enums()
        .find(|item| {
            item.qualified_name()
                .iter()
                .map(|part| part.as_str())
                .eq([name])
        })
        .expect("the source declared this enum");
    let declaration = EnumDeclaration::new(
        admitted.key(),
        admitted.is_ordered(),
        admitted
            .cases()
            .iter()
            .map(|case| case.case().as_str().to_owned())
            .collect(),
    )
    .expect("genuine declaration cases reconstruct");
    let wire = qsl_replay::WireNodeId::from_digest(*admitted.key().as_bytes());
    let mut index = EnumMemberIndex::default();
    let mut members = Vec::new();
    for case in admitted.cases() {
        let selected = context
            .enum_case(wire, case.case())
            .expect("every declared case resolves");
        let value = declaration
            .member(selected.case().as_str(), selected.key())
            .expect("the owner case belongs to its declaration");
        index.record(value.clone());
        members.push(value);
    }
    let shape = ValueType::Enum(EnumShape::new(
        admitted.is_ordered(),
        members.iter().map(EnumValue::variant),
    ));
    let values = members
        .iter()
        .map(|member| {
            Value::Enum(EnumMember::new(
                member.variant(),
                u32::try_from(member.position()).expect("finite fixture rank fits u32"),
            ))
        })
        .collect();
    let names = members
        .iter()
        .map(|member| member.case().to_owned())
        .collect();
    EnumData {
        shape,
        index,
        values,
        members,
        names,
    }
}

/// Real direct or transitive imports; returned libraries are the exact independently compiled inputs.
/// The host runner records which genuine source/dependency fixture produced these exact artifacts.
pub fn native_compilation() -> (AdmittedCompilation, Vec<(String, AdmittedCompilation)>) {
    let mode = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/context-mode"))
        .expect("the generator retained its actual source/dependency mode");
    match mode.as_str() {
        "direct" => imported(false),
        "transitive" => imported(true),
        other => panic!("unknown native fixture mode {other}"),
    }
}
