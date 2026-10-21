//! Authentic public-facade enum fixture, shared by generator and generated native tests.

use qsl_replay::{
    compile_admitted, AdmittedCompilation, AdmittedContext, DependencyInput, ReplayLimits,
    SourceIdentity, StageLimits,
};
use quire_exact::{EnumMember, EnumShape, Value, ValueType};
use quire_semantic_value::enumeration::{EnumDeclaration, EnumMemberIndex, EnumValue};

pub const SOURCE: &str = "language \"ix:native\" edition \"1-draft\";\n\
profile v = \"quire.value.complete/v1\";\n\
ordered enum Ordered { READY, DONE, UNUSED }\n\
enum Unordered { READY, DONE, UNUSED }\n\
ordered enum Other { READY, DONE, UNUSED }\n\
function ordered_same using v(a: Ordered,b: Ordered): Boolean pure { a = b }\n\
function ordered_different using v(a: Ordered,b: Ordered): Boolean pure { a != b }\n\
function unordered_same using v(a: Unordered,b: Unordered): Boolean pure { a = b }\n\
function other_same using v(a: Other,b: Other): Boolean pure { a = b }\n";

pub fn compile(source: &str) -> AdmittedCompilation {
    compile_admitted(
        SourceIdentity::new("test", "admitted-enums.native", "git", "fixture"),
        "admitted-enums.native",
        source.as_bytes(),
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("authentic enum source compiles through the public root facade")
}

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
pub fn imported(transitive: bool) -> (AdmittedCompilation, Vec<(String, AdmittedCompilation)>) {
    const HEADER: &str = "language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\n";
    let library = |identity: &str, name: &str, body: &str| qsl_replay::SuppliedLibrary {
        identity: identity.to_owned(),
        source: SourceIdentity::new("test", name, "git", "fixture"),
        path: format!("{name}.native"), bytes: format!("{HEADER}{body}").into_bytes(),
    };
    let compile_library = |item: &qsl_replay::SuppliedLibrary, dependencies: &DependencyInput| {
        compile_admitted(item.source.clone(), &item.path, &item.bytes, [], dependencies,
            StageLimits::default(), ReplayLimits::default()).expect("real library compiles")
    };
    let shades = library("test/shades", "shades", "enum Shade { DARK, LIGHT }\n");
    let shade_compiled = compile_library(&shades, &DependencyInput::default());
    let mut retained = vec![(shades.identity.clone(), shade_compiled)];
    let (supplied, import) = if transitive {
        let mid = library("test/mid", "mid", "import \"test/shades\" as s;\nfunction h using v(): Boolean pure { true }\n");
        let mid_compiled = compile_library(&mid, &DependencyInput::new([shades.clone()]).expect("shade input"));
        retained.push((mid.identity.clone(), mid_compiled));
        (vec![shades, mid], "import \"test/mid\" as m;\n")
    } else {
        (vec![shades], "import \"test/shades\" as s;\n")
    };
    let text = SOURCE.replacen("ordered enum Ordered", &format!("{import}ordered enum Ordered"), 1);
    let compiled = compile_admitted(
        SourceIdentity::new("test", "admitted-enums.native", "git", "fixture"),
        "admitted-enums.native", text.as_bytes(), [],
        &DependencyInput::new(supplied).expect("real dependency provision"),
        StageLimits::default(), ReplayLimits::default(),
    ).expect("the real root compiles with its original libraries");
    (compiled, retained)
}

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
