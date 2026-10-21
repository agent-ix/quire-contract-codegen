// SPDX-License-Identifier: AGPL-3.0-or-later
//! Authentic record and tuple sources compiled only through the public replay root.
use qsl_replay::{compile_admitted, AdmittedCompilation, DependencyInput, ReplayLimits, SourceIdentity, StageLimits, SuppliedLibrary};

pub const SOURCE: &str = r#"language "ix:native" edition "1-draft";
profile v = "quire.value.complete/v1";
tuple Pair(Boolean, Boolean);
tuple OtherPair(Boolean, Boolean);
record Holder { flag: Boolean; pair: Pair; }
record OtherHolder { flag: Boolean; pair: Pair; }
enum Marker { UNUSED, ALSO_UNUSED }
function pair_same using v(a: Pair,b: Pair): Boolean pure { a = b }
function pair_different using v(a: Pair,b: Pair): Boolean pure { a != b }
function holder_same using v(a: Holder,b: Holder): Boolean pure { a = b }
function holder_different using v(a: Holder,b: Holder): Boolean pure { a != b }
function other_pair_same using v(a: OtherPair,b: OtherPair): Boolean pure { a = b }
function other_holder_same using v(a: OtherHolder,b: OtherHolder): Boolean pure { a = b }
"#;

pub fn compile(source: &str, dependencies: &DependencyInput) -> AdmittedCompilation {
    compile_admitted(SourceIdentity::new("test", "admitted-composites.native", "git", "fixture"),
        "admitted-composites.native", source.as_bytes(), [], dependencies,
        StageLimits::default(), ReplayLimits::default()).expect("authentic composite source compiles")
}

pub fn imported(transitive: bool) -> (AdmittedCompilation, Vec<(String, AdmittedCompilation)>) {
    let header = "language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\n";
    let library = |identity: &str, name: &str, body: &str| SuppliedLibrary {
        identity: identity.to_owned(), source: SourceIdentity::new("test", name, "git", "fixture"),
        path: format!("{name}.native"), bytes: format!("{header}{body}").into_bytes(),
    };
    let compile_library = |item: &SuppliedLibrary, dependencies: &DependencyInput| {
        compile_admitted(item.source.clone(), &item.path, &item.bytes, [], dependencies,
            StageLimits::default(), ReplayLimits::default()).expect("real library compiles")
    };
    let leaf = library("test/composites", "composites", "tuple ImportedPair(Boolean, Boolean);\nrecord ImportedHolder { flag: Boolean; pair: ImportedPair; }\n");
    let original = compile_library(&leaf, &DependencyInput::default());
    let mut retained = vec![(leaf.identity.clone(), original)];
    let (supplied, import) = if transitive {
        let mid = library("test/composite-mid", "composite-mid", "import \"test/composites\" as c;\nfunction h using v(): Boolean pure { true }\n");
        let compiled = compile_library(&mid, &DependencyInput::new([leaf.clone()]).expect("original library"));
        retained.push((mid.identity.clone(), compiled));
        (vec![leaf, mid], "import \"test/composite-mid\" as m;\n")
    } else { (vec![leaf], "import \"test/composites\" as c;\n") };
    let source = SOURCE.replacen("tuple Pair", &format!("{import}tuple Pair"), 1);
    (compile(&source, &DependencyInput::new(supplied).expect("original dependencies")), retained)
}
