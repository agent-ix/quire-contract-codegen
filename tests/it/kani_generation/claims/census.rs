//! Fail-closed attribution and reachability for current compiled claims.
use quire_contract_codegen::{
    KaniCheckClass, KaniCheckResult, KaniCheckStatus, KaniExecutionEvidence, KaniRunOutcome,
    KaniScalarObligationHarness,
};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum Refusal {
    EmptyClaims,
    MissingModule(String),
    Unreached(String),
    MissingRoot(String),
    DependentExpectation(String),
    InvalidControl(String),
    HelperEvidence,
    MissingControl(String),
    InvalidBuild(String),
}
#[derive(Clone, Debug)]
pub(super) struct Claim {
    pub module: String,
    pub harness: String,
    pub start: u32,
    pub end: u32,
    failure_lines: std::ops::RangeInclusive<u32>,
}
impl Claim {
    pub fn generated(harness: &KaniScalarObligationHarness) -> Result<Self, Refusal> {
        let module = harness.identity.module_symbol.to_string();
        let parsed = syn::parse_file(&harness.rust.contents).map_err(invalid)?;
        let item = parsed
            .items
            .iter()
            .find_map(|item| match item {
                syn::Item::Mod(m) if m.ident == module => Some(m),
                _ => None,
            })
            .ok_or_else(|| Refusal::MissingModule(module.clone()))?;
        let Some((_, items)) = &item.content else {
            return Err(Refusal::MissingModule(module));
        };
        let symbol = harness.identity.harness_symbol.to_string();
        for name in [
            &symbol,
            "subject_boolean_implication",
            "expected_boolean_implication",
        ] {
            if !items
                .iter()
                .any(|item| matches!(item, syn::Item::Fn(f) if f.sig.ident == name))
            {
                return Err(Refusal::MissingRoot(format!("{module}::{name}")));
            }
        }
        use syn::{parse::Parser, spanned::Spanned};
        let mut assertions = Vec::new();
        for item in items {
            let syn::Item::Fn(function) = item else {
                continue;
            };
            if function.sig.ident != symbol {
                continue;
            }
            for statement in &function.block.stmts {
                let syn::Stmt::Macro(statement) = statement else {
                    continue;
                };
                if !statement.mac.path.is_ident("assert") {
                    continue;
                }
                let arguments =
                    syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated
                        .parse2(statement.mac.tokens.clone())
                        .map_err(invalid)?;
                let Some(syn::Expr::Binary(binary)) = arguments.first() else {
                    continue;
                };
                if matches!(binary.op, syn::BinOp::Eq(_))
                    && matches!(&*binary.left, syn::Expr::Path(path) if path.path.is_ident("subject"))
                    && matches!(&*binary.right, syn::Expr::Path(path) if path.path.is_ident("expected"))
                {
                    assertions.push(
                        u32::try_from(statement.span().start().line).map_err(invalid)?
                            ..=u32::try_from(statement.span().end().line).map_err(invalid)?,
                    );
                }
            }
        }
        let [failure_lines] = assertions.as_slice() else {
            return Err(invalid("one independent subject assertion required"));
        };
        Ok(Self {
            failure_lines: failure_lines.clone(),
            harness: format!("{module}::{symbol}"),
            module,
            start: u32::try_from(item.span().start().line).map_err(invalid)?,
            end: u32::try_from(item.span().end().line).map_err(invalid)?,
        })
    }
    fn contains(&self, check: &KaniCheckResult) -> bool {
        check.location.file == "src/lib.rs"
            && check
                .location
                .line
                .is_some_and(|line| self.start <= line && line <= self.end)
    }
}
fn invalid(error: impl std::fmt::Display) -> Refusal {
    Refusal::InvalidBuild(error.to_string())
}

pub(super) struct Compiled {
    modules: BTreeSet<String>,
    harnesses: BTreeSet<String>,
    names: BTreeMap<String, String>,
    edges: BTreeMap<String, BTreeSet<String>>,
}
fn files(directory: &Path, result: &mut Vec<PathBuf>) -> Result<(), Refusal> {
    for entry in fs::read_dir(directory).map_err(invalid)? {
        let entry = entry.map_err(invalid)?;
        let kind = entry.file_type().map_err(invalid)?;
        if kind.is_dir() {
            files(&entry.path(), result)?;
        } else if kind.is_file() {
            result.push(entry.path());
        }
    }
    Ok(())
}
fn instrument() -> Result<PathBuf, Refusal> {
    let output = Command::new("cargo-kani")
        .arg("--version")
        .output()
        .map_err(invalid)?;
    if !output.status.success() {
        return Err(invalid("Kani version unavailable"));
    }
    let version = String::from_utf8_lossy(&output.stdout);
    let version = version
        .split_whitespace()
        .find(|part| part.chars().next().is_some_and(|c| c.is_ascii_digit()))
        .ok_or_else(|| invalid("Kani version absent"))?;
    let home = std::env::var_os("HOME").ok_or_else(|| invalid("home unavailable"))?;
    Ok(PathBuf::from(home)
        .join(".kani")
        .join(format!("kani-{version}"))
        .join("bin/goto-instrument"))
}
impl Compiled {
    pub fn read(target: &Path, claim: &Claim) -> Result<Self, Refusal> {
        let mut artifacts = Vec::new();
        files(target, &mut artifacts)?;
        let mut matches = Vec::new();
        for path in artifacts
            .iter()
            .filter(|p| p.to_string_lossy().ends_with(".kani-metadata.json"))
        {
            let value: Value =
                serde_json::from_slice(&fs::read(path).map_err(invalid)?).map_err(invalid)?;
            if let Some(harnesses) = value["proof_harnesses"].as_array() {
                for harness in harnesses {
                    if harness["pretty_name"].as_str() == Some(&claim.harness) {
                        matches.push((value.clone(), harness.clone()));
                    }
                }
            }
        }
        let [(metadata, harness)] = matches.as_slice() else {
            return Err(Refusal::MissingModule(claim.module.clone()));
        };
        if harness["original_file"].as_str() != Some("src/lib.rs") {
            return Err(invalid("claim source differs"));
        }
        let original = harness["goto_file"]
            .as_str()
            .ok_or_else(|| invalid("missing GOTO file"))?;
        let binary = PathBuf::from(original.replace(".symtab.out", ".out"));
        if !binary.starts_with(target) || !binary.is_file() {
            return Err(invalid("fresh compiled GOTO missing"));
        }
        let tool = instrument()?;
        let symbols = Command::new(&tool)
            .args(["--show-symbol-table", "--json-ui"])
            .arg(&binary)
            .output()
            .map_err(invalid)?;
        let graph = Command::new(&tool)
            .arg("--call-graph")
            .arg(&binary)
            .output()
            .map_err(invalid)?;
        if !symbols.status.success() || !graph.status.success() {
            return Err(invalid("compiled inspection failed"));
        }
        let rows: Vec<Value> = serde_json::from_slice(&symbols.stdout).map_err(invalid)?;
        let table = rows
            .iter()
            .find_map(|row| row["symbolTable"].as_object())
            .ok_or_else(|| invalid("symbol table absent"))?;
        let names = table
            .iter()
            .filter(|(_, v)| v["type"]["id"] == "code")
            .filter_map(|(id, v)| Some((id.clone(), v["prettyName"].as_str()?.to_owned())))
            .collect();
        let graph = String::from_utf8(graph.stdout).map_err(invalid)?;
        let mut edges: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for row in graph.lines() {
            if let Some((caller, callee)) = row.split_once(" -> ") {
                edges
                    .entry(caller.to_owned())
                    .or_default()
                    .insert(callee.to_owned());
            }
        }
        let modules = metadata["proof_harnesses"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|h| {
                h["pretty_name"]
                    .as_str()?
                    .rsplit_once("::")
                    .map(|(module, _)| module.to_owned())
            })
            .collect();
        let harnesses = metadata["proof_harnesses"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|h| h["pretty_name"].as_str().map(str::to_owned))
            .collect();
        let result = Self {
            modules,
            harnesses,
            names,
            edges,
        };
        let proof = result.symbol(&claim.harness)?;
        let reachable = result.reachable(&proof);
        for root in [
            "subject_boolean_implication",
            "expected_boolean_implication",
        ] {
            let name = format!("{}::{root}", claim.module);
            if !reachable.contains(&result.symbol(&name)?) {
                return Err(Refusal::MissingRoot(name));
            }
        }
        fs::write(
            target.join("compiled-census.json"),
            serde_json::to_vec_pretty(metadata).map_err(invalid)?,
        )
        .map_err(invalid)?;
        fs::write(target.join("compiled-call-graph.txt"), graph).map_err(invalid)?;
        fs::write(target.join("compiled-symbols.json"), symbols.stdout).map_err(invalid)?;
        Ok(result)
    }
    fn symbol(&self, name: &str) -> Result<String, Refusal> {
        let symbols: Vec<_> = self
            .names
            .iter()
            .filter(|(_, pretty)| *pretty == name)
            .map(|(id, _)| id.clone())
            .collect();
        match symbols.as_slice() {
            [symbol] => Ok(symbol.clone()),
            _ => Err(Refusal::MissingRoot(name.to_owned())),
        }
    }
    fn reachable(&self, root: &str) -> BTreeSet<String> {
        let mut reached = BTreeSet::new();
        let mut pending = vec![root.to_owned()];
        while let Some(next) = pending.pop() {
            if reached.insert(next.clone()) {
                if let Some(children) = self.edges.get(&next) {
                    pending.extend(children.iter().cloned());
                }
            }
        }
        reached
    }
    pub fn shared_helpers(&self, claim: &Claim) -> Result<BTreeSet<String>, Refusal> {
        let subject = self.symbol(&format!("{}::subject_boolean_implication", claim.module))?;
        let expectation =
            self.symbol(&format!("{}::expected_boolean_implication", claim.module))?;
        let subject_reach = self.reachable(&subject);
        let expected_reach = self.reachable(&expectation);
        for id in &expected_reach {
            let name = self
                .names
                .get(id)
                .ok_or_else(|| Refusal::MissingRoot(id.clone()))?;
            if id == &subject || name.contains("quire_exact") || name.contains("oracle") {
                return Err(Refusal::DependentExpectation(name.clone()));
            }
        }
        if !subject_reach.iter().any(|id| {
            self.names.get(id).is_some_and(|name| {
                name.contains("quire_contract_runtime::operators::implies_short_circuit")
            })
        }) {
            return Err(Refusal::MissingRoot("runtime implication".into()));
        }
        Ok(subject_reach
            .intersection(&expected_reach)
            .cloned()
            .collect())
    }
    pub fn qualify(&self, claims: &[Claim], checks: &[KaniCheckResult]) -> Result<(), Refusal> {
        if claims.is_empty() {
            return Err(Refusal::EmptyClaims);
        }
        for claim in claims {
            if !self.modules.contains(&claim.module) {
                return Err(Refusal::MissingModule(claim.module.clone()));
            }
            if !self.harnesses.contains(&claim.harness) {
                return Err(Refusal::MissingRoot(claim.harness.clone()));
            }
            if !checks.iter().any(|check| {
                claim.contains(check)
                    && check.class != KaniCheckClass::Cover
                    && check.status == KaniCheckStatus::Success
            }) {
                return Err(Refusal::Unreached(claim.module.clone()));
            }
        }
        Ok(())
    }
    pub fn require_failure(
        &self,
        claim: &Claim,
        evidence: &KaniExecutionEvidence,
    ) -> Result<(), Refusal> {
        if evidence.harness_path != claim.harness
            || !matches!(evidence.outcome, KaniRunOutcome::Falsified { .. })
            || !evidence.checks.iter().any(|check| {
                claim.contains(check)
                    && check.class != KaniCheckClass::Cover
                    && check.status == KaniCheckStatus::Failure
                    && check.class == KaniCheckClass::from("assertion".to_owned())
                    && check
                        .location
                        .line
                        .is_some_and(|line| claim.failure_lines.contains(&line))
            })
        {
            return Err(Refusal::InvalidControl(claim.module.clone()));
        }
        Ok(())
    }
}

pub(super) struct Control<'a> {
    pub name: &'a str,
    pub before: &'a str,
    pub after: &'a str,
    pub positive: &'a KaniExecutionEvidence,
    pub mutant: &'a KaniExecutionEvidence,
}

/// Require one distinct, executed source control for every module and observed helper.
pub(super) fn controls(
    claims: &[Claim],
    observed_helpers: &BTreeSet<String>,
    reviewed_helpers: &BTreeSet<String>,
    records: &[Control<'_>],
) -> Result<(), Refusal> {
    if observed_helpers != reviewed_helpers {
        return Err(Refusal::HelperEvidence);
    }
    let required: BTreeSet<_> = claims
        .iter()
        .map(|claim| claim.module.clone())
        .chain(observed_helpers.iter().cloned())
        .collect();
    let mut seen = BTreeSet::new();
    let mut mutations = BTreeSet::new();
    for record in records {
        if !required.contains(record.name)
            || !seen.insert(record.name.to_owned())
            || record.before == record.after
            || !mutations.insert((record.before, record.after))
            || record.positive.outcome != KaniRunOutcome::Verified
            || !matches!(record.mutant.outcome, KaniRunOutcome::Falsified { .. })
            || record.positive.arguments.is_empty()
            || record.mutant.arguments.is_empty()
            || !record.mutant.checks.iter().any(|check| {
                check.status == KaniCheckStatus::Failure
                    && check.class == KaniCheckClass::from("assertion".to_owned())
            })
        {
            return Err(Refusal::InvalidControl(record.name.to_owned()));
        }
    }
    if let Some(missing) = required.difference(&seen).next() {
        return Err(Refusal::MissingControl(missing.clone()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;
    use quire_contract_codegen::KaniCheckLocation;

    #[trace("TC-056", "FR-041-AC-2")]
    #[test]
    fn two_modules_in_one_file_need_separate_located_successes() {
        let first = Claim {
            module: "first".into(),
            harness: "first::proof".into(),
            start: 10,
            end: 20,
            failure_lines: 10..=20,
        };
        let second = Claim {
            module: "second".into(),
            harness: "second::proof".into(),
            start: 30,
            end: 40,
            failure_lines: 30..=40,
        };
        let compiled = Compiled {
            modules: BTreeSet::from([first.module.clone(), second.module.clone()]),
            harnesses: BTreeSet::from([first.harness.clone(), second.harness.clone()]),
            names: BTreeMap::new(),
            edges: BTreeMap::new(),
        };
        let success = |line| KaniCheckResult {
            id: 1,
            class: "assertion".to_owned().into(),
            location: KaniCheckLocation {
                file: "src/lib.rs".into(),
                line: Some(line),
            },
            status: KaniCheckStatus::Success,
        };
        let first_check = success(15);
        let second_check = success(35);
        let claims = [first, second.clone()];
        compiled
            .qualify(&claims, &[first_check.clone(), second_check.clone()])
            .unwrap();
        for status in [
            KaniCheckStatus::Unreachable,
            KaniCheckStatus::Satisfied,
            KaniCheckStatus::Failure,
            KaniCheckStatus::Unknown,
        ] {
            let mut invalid = second_check.clone();
            invalid.status = status;
            assert_eq!(
                compiled.qualify(&claims, &[first_check.clone(), invalid]),
                Err(Refusal::Unreached(second.module.clone()))
            );
        }
        let mut unknown_location = second_check;
        unknown_location.location.line = None;
        assert_eq!(
            compiled.qualify(&claims, &[first_check.clone(), unknown_location]),
            Err(Refusal::Unreached(second.module.clone()))
        );
        let mut outside = success(35);
        outside.location.file = "other/lib.rs".into();
        assert_eq!(
            compiled.qualify(&claims, &[first_check.clone(), outside]),
            Err(Refusal::Unreached(second.module.clone()))
        );
        let mut cover = success(35);
        cover.class = KaniCheckClass::Cover;
        assert_eq!(
            compiled.qualify(&claims, &[first_check, cover]),
            Err(Refusal::Unreached(second.module))
        );
    }
    #[trace("TC-056", "FR-041-AC-3", "FR-041-AC-4")]
    #[test]
    fn transitive_intersection_requires_exact_evidence_and_each_helper_control() {
        let claim = Claim {
            module: "m".into(),
            harness: "m::proof".into(),
            start: 1,
            end: 50,
            failure_lines: 1..=50,
        };
        let mut compiled = Compiled {
            modules: BTreeSet::from(["m".into()]),
            harnesses: BTreeSet::from(["m::proof".into()]),
            names: BTreeMap::from([
                ("subject".into(), "m::subject_boolean_implication".into()),
                ("expected".into(), "m::expected_boolean_implication".into()),
                (
                    "runtime".into(),
                    "quire_contract_runtime::operators::implies_short_circuit".into(),
                ),
                ("bridge".into(), "indirect".into()),
                ("shared".into(), "shared".into()),
            ]),
            edges: BTreeMap::from([
                ("subject".into(), BTreeSet::from(["runtime".into()])),
                ("runtime".into(), BTreeSet::from(["shared".into()])),
                ("expected".into(), BTreeSet::from(["bridge".into()])),
                ("bridge".into(), BTreeSet::from(["shared".into()])),
            ]),
        };
        let observed = compiled.shared_helpers(&claim).unwrap();
        assert_eq!(observed, BTreeSet::from(["shared".to_owned()]));
        let no_helpers = BTreeSet::new();
        assert_eq!(
            controls(std::slice::from_ref(&claim), &observed, &no_helpers, &[]),
            Err(Refusal::HelperEvidence)
        );
        assert_eq!(
            controls(std::slice::from_ref(&claim), &no_helpers, &observed, &[]),
            Err(Refusal::HelperEvidence)
        );
        assert_eq!(
            controls(&[], &observed, &observed, &[]),
            Err(Refusal::MissingControl("shared".into()))
        );
        compiled
            .edges
            .insert("bridge".into(), BTreeSet::from(["subject".into()]));
        assert_eq!(
            compiled.shared_helpers(&claim),
            Err(Refusal::DependentExpectation(
                "m::subject_boolean_implication".into()
            ))
        );
        compiled
            .edges
            .insert("bridge".into(), BTreeSet::from(["exact".into()]));
        compiled
            .names
            .insert("exact".into(), "quire_exact::operation".into());
        assert_eq!(
            compiled.shared_helpers(&claim),
            Err(Refusal::DependentExpectation(
                "quire_exact::operation".into()
            ))
        );
        compiled.edges.remove("bridge");
        assert!(
            compiled.shared_helpers(&claim).unwrap().is_empty(),
            "explicit empty computed set"
        );
    }
}
