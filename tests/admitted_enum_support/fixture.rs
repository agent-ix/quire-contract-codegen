//! Authentic public-facade enum fixture, shared by generator and generated native tests.

use qsl_replay::{
    compile_admitted, AdmittedCompilation, CompileUnit, DependencyInput, ReplayLimits,
    SourceIdentity, StageLimits,
};

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
        CompileUnit {
            source: SourceIdentity::new("test", "admitted-enums.native", "git", "fixture"),
            package_name: "admitted-enums".to_owned(),
            path: "admitted-enums.native",
            bytes: source.as_bytes(),
        },
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("authentic enum source compiles through the public root facade")
}

pub fn imported(transitive: bool) -> (AdmittedCompilation, Vec<(String, AdmittedCompilation)>) {
    const HEADER: &str =
        "language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\n";
    let library = |identity: &str, name: &str, body: &str| qsl_replay::SuppliedLibrary {
        identity: identity.to_owned(),
        source: SourceIdentity::new("test", name, "git", "fixture"),
        path: format!("{name}.native"),
        bytes: format!("{HEADER}{body}").into_bytes(),
    };
    let compile_library = |item: &qsl_replay::SuppliedLibrary, dependencies: &DependencyInput| {
        compile_admitted(
            CompileUnit {
                source: item.source.clone(),
                package_name: item.identity.clone(),
                path: &item.path,
                bytes: &item.bytes,
            },
            [],
            dependencies,
            StageLimits::default(),
            ReplayLimits::default(),
        )
        .expect("real library compiles")
    };
    let shades = library("test/shades", "shades", "enum Shade { DARK, LIGHT }\n");
    let shade_compiled = compile_library(&shades, &DependencyInput::default());
    let mut retained = vec![(shades.identity.clone(), shade_compiled)];
    let (supplied, import) = if transitive {
        let mid = library(
            "test/mid",
            "mid",
            "import \"test/shades\" as s;\nfunction h using v(): Boolean pure { true }\n",
        );
        let mid_compiled = compile_library(
            &mid,
            &DependencyInput::new([shades.clone()]).expect("shade input"),
        );
        retained.push((mid.identity.clone(), mid_compiled));
        (vec![shades, mid], "import \"test/mid\" as m;\n")
    } else {
        (vec![shades], "import \"test/shades\" as s;\n")
    };
    let text = SOURCE.replacen(
        "ordered enum Ordered",
        &format!("{import}ordered enum Ordered"),
        1,
    );
    let compiled = compile_admitted(
        CompileUnit {
            source: SourceIdentity::new("test", "admitted-enums.native", "git", "fixture"),
            package_name: "admitted-enums".to_owned(),
            path: "admitted-enums.native",
            bytes: text.as_bytes(),
        },
        [],
        &DependencyInput::new(supplied).expect("real dependency provision"),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("the real root compiles with its original libraries");
    (compiled, retained)
}
