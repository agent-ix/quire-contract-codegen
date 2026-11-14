//! Private domain-conformance and completeness controls for the graph sampler (FR-015-AC-106,
//! TC-025, IR-805 slice C). Kani lane: `cargo test --lib kani_graph_domain_private -- --ignored
//! --test-threads=1`, serially inside the flock and the memory scope.
//!
//! The oracle is a test harness, not a production generator. Over a candidate carrier that holds
//! every graph of the healthy domain and of the three mutated domains (`N + 1` objects, `U + 1`
//! edges per row), it asserts `G(g) implies D(g)` (conformance) and `D(g) implies G(g)`
//! (completeness):
//!
//! - `D` is written here from the authored declaration (population maximum, edge bounds,
//!   uniqueness), which the fixture first checks against the package's own `select_population`
//!   and `relationship_end` projections. It reads nothing from a generated harness.
//! - `G` is the emitted sampler itself: its draw names, ranges and assumptions are read back from
//!   the generated source and evaluated on the carrier graph, with the sampler's storage shape
//!   cross-checked against the draws. A graph the sampler has no storage for is rejected, never
//!   truncated.
//! - The candidate is drawn independently of `G` and `D`; only the carrier's box and its dormant padding are constrained.
//!
//! A pure-bound mutant is the production renderer run with one replaced bound
//! (`render_with_bounds`); the package, `D`, the carrier and the clause rules stay unchanged.

use crate::{
    kani::{
        classify::KaniRunOutcome, generate::graph::render_with_bounds,
        identity::KaniGraphObligationHarness, output::playback::parse_assertion_playback,
        test_support::proof_ceilings::proof_ceilings_with_wall_clock,
    },
    kani_graph_private::{authored, harness, run_falsified, run_outcome, Authored},
    KaniObligationRequest,
};
use ix_trace_rs::trace;
use quire_contract_model::{
    selected_population::{
        select_population, CheckedSelectedPopulationOutcome as Population,
        CheckedSelectedPopulationSource,
    },
    CheckedNodeId, CheckedTerm,
};
use std::{collections::HashMap, time::Duration};

/// The authored population maximum of the fixture's `bound Nodes on M::CollectionNodes [4]`.
const POPULATION: i64 = 4;
/// The `{0,3}` ends of the fixture's `kids` (unique) and `litter` (non-unique) relationships.
const LOWER: i64 = 0;
const UPPER: i64 = 3;
/// Carrier extents: `N + 1` objects and `U + 1` edges per row.
const ROWS: usize = 5;
const COLS: usize = 4;

const CONFORMANCE: &str =
    "domain conformance: the sampler admits only graphs of the authored domain";
const COMPLETENESS: &str =
    "domain completeness: the sampler admits every graph of the authored domain";

/// What the fixture authors for the selected end.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Declared {
    population: i64,
    lower: i64,
    upper: i64,
    unique: bool,
}

#[derive(Clone, Copy, Debug)]
enum End {
    Unique,
    NonUnique,
}

impl End {
    const ALL: [Self; 2] = [Self::Unique, Self::NonUnique];

    const fn role(self) -> &'static str {
        match self {
            Self::Unique => "kids",
            Self::NonUnique => "litter",
        }
    }

    const fn declared(self) -> Declared {
        Declared {
            population: POPULATION,
            lower: LOWER,
            upper: UPPER,
            unique: matches!(self, Self::Unique),
        }
    }

    fn authored(self) -> Authored {
        authored(
            &format!("not reaches(self, self, {})", self.role()),
            POPULATION as u64,
        )
    }
}

/// One active-or-dormant graph of the verification carrier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Graph {
    count: i64,
    start: i64,
    target: i64,
    degrees: [i64; ROWS],
    edges: [[i64; COLS]; ROWS],
}

impl Graph {
    /// `count` objects, the operands at object zero, and the given active rows (degree zero and
    /// no targets elsewhere).
    fn with_rows(count: i64, rows: &[&[i64]]) -> Self {
        let mut graph = Self {
            count,
            start: 0,
            target: 0,
            degrees: [0; ROWS],
            edges: [[0; COLS]; ROWS],
        };
        for (row, targets) in rows.iter().enumerate() {
            graph.degrees[row] = targets.len() as i64;
            graph.edges[row][..targets.len()].copy_from_slice(targets);
        }
        graph
    }

    /// The graph with every dormant slot zeroed: a row at or past `count` and a slot at or past
    /// its row's degree. A zero population carries the sentinel operands `0`.
    fn normalized(mut self) -> Self {
        for row in 0..ROWS {
            if (row as i64) >= self.count {
                self.degrees[row] = 0;
            }
            for column in 0..COLS {
                if (row as i64) >= self.count || (column as i64) >= self.degrees[row] {
                    self.edges[row][column] = 0;
                }
            }
        }
        if self.count == 0 {
            self.start = 0;
            self.target = 0;
        }
        self
    }

    fn variable(&self, name: &str) -> i64 {
        match name {
            "object_count" => self.count,
            "start" => self.start,
            "target" => self.target,
            _ => {
                let (kind, indices) = name.split_once('_').expect("a drawn slot name");
                let mut indices = indices
                    .split('_')
                    .map(|index| index.parse::<usize>().unwrap());
                match kind {
                    "degree" => self.degrees[indices.next().unwrap()],
                    "edge" => self.edges[indices.next().unwrap()][indices.next().unwrap()],
                    other => panic!("unknown draw {other}"),
                }
            }
        }
    }
}

/// `D(g)`: membership in the authored domain, over active rows and active slots only.
fn native_domain(declared: Declared, graph: &Graph) -> bool {
    if graph.count < 0 || graph.count > declared.population {
        return false;
    }
    // An invariant has one parameter, so both operands are one live object.
    if graph.count > 0
        && !(graph.start == graph.target && graph.start >= 0 && graph.start < graph.count)
    {
        return false;
    }
    (0..graph.count as usize).all(|row| {
        let degree = graph.degrees[row];
        if degree < declared.lower || degree > declared.upper {
            return false;
        }
        let targets = &graph.edges[row][..degree as usize];
        targets
            .iter()
            .all(|target| (0..graph.count).contains(target))
            && (!declared.unique
                || targets
                    .iter()
                    .enumerate()
                    .all(|(at, target)| !targets[..at].contains(target)))
    })
}

/// The same `D` as Rust source for the Kani carrier.
fn domain_source(declared: Declared) -> String {
    let Declared {
        population,
        lower,
        upper,
        unique,
    } = declared;
    format!(
        "fn domain(count: i64, start: i64, target: i64, degrees: &[i64; {ROWS}], edges: &[[i64; {COLS}]; {ROWS}]) -> bool {{\n\
         if count < 0 || count > {population} {{ return false; }}\n\
         if count > 0 && !(start == target && start >= 0 && start < count) {{ return false; }}\n\
         for row in 0..{ROWS} {{\n\
         if (row as i64) < count {{\n\
         let degree = degrees[row];\n\
         if degree < {lower} || degree > {upper} {{ return false; }}\n\
         for column in 0..{COLS} {{\n\
         if (column as i64) < degree {{\n\
         let target = edges[row][column];\n\
         if target < 0 || target >= count {{ return false; }}\n\
         if {unique} {{ for earlier in 0..column {{ if edges[row][earlier] == target {{ return false; }} }} }}\n\
         }}\n\
         }}\n\
         }}\n\
         }}\n\
         true\n}}\n"
    )
}

/// What the emitted sampler draws and assumes, read back from its generated source.
struct Sampler {
    rows: usize,
    cols: usize,
    draws: Vec<String>,
    conditions: Vec<String>,
}

fn between<'a>(text: &'a str, open: &str, close: &str) -> &'a str {
    let start = text.find(open).expect("the signature names its shape") + open.len();
    let end = text[start..].find(close).expect("the shape closes") + start;
    &text[start..end]
}

/// Observes the sampler: its storage shape from the helper signature, its draws and every
/// assumption (range, guard, uniqueness) from the proof body. The draws must be exactly the
/// slots the storage shape has, so a shape the draws do not fill is a failed observation.
fn observe(source: &str) -> Sampler {
    let signature = source
        .lines()
        .find(|line| line.starts_with("fn graph_condition_"))
        .expect("the generated condition helper");
    let rows: usize = between(signature, "degrees: &[i64; ", "]").parse().unwrap();
    let cols: usize = between(signature, "edges: &[[i64; ", "]").parse().unwrap();
    let proof = source
        .split("#[kani::proof]")
        .nth(1)
        .expect("the generated proof");
    let mut draws = Vec::new();
    let mut conditions = Vec::new();
    for line in proof.lines().map(str::trim).filter(|line| !line.is_empty()) {
        if let Some(name) = line
            .strip_prefix("let ")
            .and_then(|rest| rest.strip_suffix(": i64 = kani::any();"))
        {
            draws.push(name.to_owned());
        } else if let Some(condition) = line
            .strip_prefix("kani::assume(")
            .and_then(|rest| rest.strip_suffix(");"))
        {
            conditions.push(condition.to_owned());
        } else if line.starts_with("fn prove_") {
            continue;
        } else if line.starts_with("let degrees = ") {
            // The first statement after the constraints: nothing later restricts a draw.
            break;
        } else {
            panic!("unrecognised statement in the sampler's constraint section: {line}");
        }
    }
    let mut expected: Vec<String> = ["object_count", "start", "target"]
        .map(str::to_owned)
        .into();
    for row in 0..rows {
        expected.push(format!("degree_{row}"));
        expected.extend((0..cols).map(|column| format!("edge_{row}_{column}")));
    }
    assert_eq!(draws, expected, "the draws fill exactly the storage shape");
    Sampler {
        rows,
        cols,
        draws,
        conditions,
    }
}

#[derive(Clone, Copy)]
enum Value {
    Int(i64),
    Bool(bool),
}

/// Evaluates one assumption of the sampler: `||`, `&&`, comparisons, draw names, integer
/// literals with an `_i64` suffix, parentheses.
struct Evaluator<'a> {
    tokens: Vec<String>,
    at: usize,
    values: &'a HashMap<String, i64>,
}

impl Evaluator<'_> {
    fn tokenize(text: &str) -> Vec<String> {
        let chars: Vec<char> = text.chars().collect();
        let mut tokens = Vec::new();
        let mut at = 0;
        while at < chars.len() {
            let current = chars[at];
            let pair: String = chars[at..chars.len().min(at + 2)].iter().collect();
            if current.is_whitespace() {
                at += 1;
            } else if ["&&", "||", "==", "!=", ">=", "<="].contains(&pair.as_str()) {
                tokens.push(pair);
                at += 2;
            } else if "()<>".contains(current) {
                tokens.push(current.to_string());
                at += 1;
            } else {
                let end = chars[at..]
                    .iter()
                    .position(|ch| !(ch.is_alphanumeric() || *ch == '_'))
                    .map_or(chars.len(), |offset| at + offset);
                assert!(end > at, "unexpected `{current}` in `{text}`");
                tokens.push(chars[at..end].iter().collect());
                at = end;
            }
        }
        tokens
    }

    fn evaluate(text: &str, values: &HashMap<String, i64>) -> bool {
        let mut evaluator = Evaluator {
            tokens: Self::tokenize(text),
            at: 0,
            values,
        };
        let result = evaluator.or();
        assert_eq!(evaluator.at, evaluator.tokens.len(), "`{text}` fully read");
        evaluator.boolean(result)
    }

    fn peek(&self) -> Option<&str> {
        self.tokens.get(self.at).map(String::as_str)
    }

    fn boolean(&self, value: Value) -> bool {
        match value {
            Value::Bool(value) => value,
            Value::Int(_) => panic!("an integer where a Boolean is required"),
        }
    }

    fn integer(&self, value: Value) -> i64 {
        match value {
            Value::Int(value) => value,
            Value::Bool(_) => panic!("a Boolean where an integer is required"),
        }
    }

    fn or(&mut self) -> Value {
        let mut value = self.and();
        while self.peek() == Some("||") {
            self.at += 1;
            let right = self.and();
            value = Value::Bool(self.boolean(value) || self.boolean(right));
        }
        value
    }

    fn and(&mut self) -> Value {
        let mut value = self.comparison();
        while self.peek() == Some("&&") {
            self.at += 1;
            let right = self.comparison();
            value = Value::Bool(self.boolean(value) && self.boolean(right));
        }
        value
    }

    fn comparison(&mut self) -> Value {
        let left = self.atom();
        let Some(operator) = self
            .peek()
            .filter(|token| ["==", "!=", ">=", "<=", "<", ">"].contains(token))
            .map(str::to_owned)
        else {
            return left;
        };
        self.at += 1;
        let (left, right) = (self.integer(left), {
            let right = self.atom();
            self.integer(right)
        });
        Value::Bool(match operator.as_str() {
            "==" => left == right,
            "!=" => left != right,
            ">=" => left >= right,
            "<=" => left <= right,
            "<" => left < right,
            _ => left > right,
        })
    }

    fn atom(&mut self) -> Value {
        let token = self.tokens[self.at].clone();
        self.at += 1;
        if token == "(" {
            let value = self.or();
            assert_eq!(self.peek(), Some(")"));
            self.at += 1;
            return value;
        }
        if let Some(value) = self.values.get(&token) {
            return Value::Int(*value);
        }
        Value::Int(
            token
                .strip_suffix("_i64")
                .unwrap_or(&token)
                .parse()
                .unwrap_or_else(|_| panic!("unknown name `{token}`")),
        )
    }
}

impl Sampler {
    /// `G(g)`: the sampler stores and admits this graph. Every draw reads the carrier slot of the
    /// same name, so a graph with more objects, a longer row or a larger target than the sampler's
    /// ranges allow fails an assumption instead of being indexed or truncated.
    fn admits(&self, graph: &Graph) -> bool {
        let values: HashMap<String, i64> = self
            .draws
            .iter()
            .map(|name| (name.clone(), graph.variable(name)))
            .collect();
        self.conditions
            .iter()
            .all(|condition| Evaluator::evaluate(condition, &values))
    }
}

/// The carrier slot a sampler draw reads.
fn carrier_slot(draw: &str) -> String {
    match draw {
        "object_count" => "c_count".into(),
        "start" => "c_start".into(),
        "target" => "c_target".into(),
        _ => {
            let parts: Vec<usize> = draw
                .split('_')
                .skip(1)
                .map(|index| index.parse().unwrap())
                .collect();
            let fits = match parts.as_slice() {
                [row] => *row < ROWS,
                [row, column] => *row < ROWS && *column < COLS,
                _ => false,
            };
            assert!(
                fits,
                "the carrier has no slot for {draw}: control unqualified"
            );
            if draw.starts_with("degree_") {
                format!("c_deg[{}]", parts[0])
            } else {
                format!("c_edge[{}][{}]", parts[0], parts[1])
            }
        }
    }
}

/// What a run asserts or covers.
enum Run<'a> {
    /// Both implications and the cover `D and G and count = N`.
    Oracle,
    /// Only the one final cover over the carrier, no assertion and no membership assumption.
    Probe(&'a str),
}

/// The oracle harness source for `sampler`, defined under the sampler harness's own symbols.
fn oracle_source(
    vehicle: &KaniGraphObligationHarness,
    sampler: &Sampler,
    declared: Declared,
    run: &Run<'_>,
) -> String {
    let mut body = format!(
        "let mut c_deg = [0i64; {ROWS}];\nlet mut c_edge = [[0i64; {COLS}]; {ROWS}];\n\
         let c_count: i64 = kani::any();\nlet c_start: i64 = kani::any();\nlet c_target: i64 = kani::any();\n\
         for row in 0..{ROWS} {{ c_deg[row] = kani::any(); }}\n\
         for row in 0..{ROWS} {{ for column in 0..{COLS} {{ c_edge[row][column] = kani::any(); }} }}\n\
         kani::assume(c_count >= 0 && c_count <= {ROWS});\n\
         kani::assume(c_start >= 0 && c_start < {ROWS} && c_target >= 0 && c_target < {ROWS});\n\
         kani::assume(c_count > 0 || (c_start == 0 && c_target == 0));\n\
         for row in 0..{ROWS} {{\n\
         kani::assume(c_deg[row] >= 0 && c_deg[row] <= {COLS});\n\
         kani::assume((row as i64) < c_count || c_deg[row] == 0);\n\
         for column in 0..{COLS} {{\n\
         kani::assume(c_edge[row][column] >= 0 && c_edge[row][column] < {ROWS});\n\
         kani::assume(((row as i64) < c_count && (column as i64) < c_deg[row]) || c_edge[row][column] == 0);\n\
         }}\n\
         }}\n"
    );
    for draw in &sampler.draws {
        body.push_str(&format!("let {draw}: i64 = {};\n", carrier_slot(draw)));
    }
    let admitted = sampler
        .conditions
        .iter()
        .map(|condition| format!("({condition})"))
        .collect::<Vec<_>>()
        .join(" && ");
    body.push_str(&format!(
        "let g = {admitted};\nlet d = domain(c_count, c_start, c_target, &c_deg, &c_edge);\n"
    ));
    match run {
        Run::Oracle => body.push_str(&format!(
            "assert!(!g || d, \"{CONFORMANCE}\");\nassert!(!d || g, \"{COMPLETENESS}\");\n\
             kani::cover!(d && g && c_count == {}, \"a graph of the full authored population is in both domains\");\n",
            declared.population
        )),
        Run::Probe(cover) => body.push_str(&format!(
            "kani::cover!({cover}, \"the boundary witness is reachable in the carrier\");\n"
        )),
    }
    format!(
        "{}#[cfg(kani)]\nmod {} {{\n{}#[kani::proof]\nfn {}() {{\n{body}}}\n}}\n",
        "",
        vehicle.identity.module_symbol.as_str(),
        domain_source(declared),
        vehicle.identity.harness_symbol.as_str(),
    )
}

/// The sampler's harness identity with the oracle source sealed in place of its proof.
fn oracle(
    vehicle: &KaniGraphObligationHarness,
    declared: Declared,
    run: &Run<'_>,
) -> KaniGraphObligationHarness {
    let sampler = observe(&vehicle.rust.contents);
    let mut rust = vehicle.rust.clone();
    rust.contents = oracle_source(vehicle, &sampler, declared, run);
    // The oracle replaces the sampler's proof inside the sampler's own harness identity; the seal
    // is the same private fault-injection seam the semantic mutants use.
    KaniGraphObligationHarness::generated(vehicle.identity.clone(), rust, vehicle.record.clone())
}

/// A pure-bound mutant: the unchanged claim rendered with one replaced bound.
#[derive(Clone, Copy, Debug)]
enum Mutant {
    PopulationAbove,
    PopulationBelow,
    EdgeUpperAbove,
}

impl Mutant {
    const ALL: [Self; 3] = [
        Self::PopulationAbove,
        Self::PopulationBelow,
        Self::EdgeUpperAbove,
    ];

    const fn bounds(self) -> (u64, u64) {
        match self {
            Self::PopulationAbove => (POPULATION as u64 + 1, UPPER as u64),
            Self::PopulationBelow => (POPULATION as u64 - 1, UPPER as u64),
            Self::EdgeUpperAbove => (POPULATION as u64, UPPER as u64 + 1),
        }
    }

    /// The assertion the mutant violates.
    const fn violated(self) -> &'static str {
        match self {
            Self::PopulationBelow => COMPLETENESS,
            Self::PopulationAbove | Self::EdgeUpperAbove => CONFORMANCE,
        }
    }

    /// Whether the mismatch graph is admitted by `G` (conformance) rather than by `D`.
    const fn sampler_only(self) -> bool {
        !matches!(self, Self::PopulationBelow)
    }

    /// The spec's concrete boundary witness for this mutant over `end`.
    fn witness(self, end: End) -> Graph {
        match self {
            Self::PopulationAbove => Graph::with_rows(POPULATION + 1, &[]),
            Self::PopulationBelow => Graph::with_rows(POPULATION, &[]),
            Self::EdgeUpperAbove => Graph::with_rows(
                POPULATION,
                &[match end {
                    End::Unique => &[0, 1, 2, 3],
                    End::NonUnique => &[0, 0, 1, 2],
                }],
            ),
        }
    }

    /// The carrier condition that is exactly `witness` with the required mismatch.
    fn cover(self, end: End) -> String {
        let witness = self.witness(end);
        let rows: Vec<String> = (0..ROWS)
            .flat_map(|row| {
                let degree = format!("c_deg[{row}] == {}", witness.degrees[row]);
                let targets = (0..COLS)
                    .map(|column| {
                        format!("c_edge[{row}][{column}] == {}", witness.edges[row][column])
                    })
                    .collect::<Vec<_>>();
                std::iter::once(degree).chain(targets)
            })
            .collect();
        let mismatch = if self.sampler_only() {
            "g && !d"
        } else {
            "d && !g"
        };
        format!(
            "{mismatch} && c_count == {} && c_start == 0 && c_target == 0 && {}",
            witness.count,
            rows.join(" && ")
        )
    }
}

fn request() -> KaniObligationRequest<'static> {
    KaniObligationRequest {
        ceilings: proof_ceilings_with_wall_clock(Duration::from_secs(1800)),
        items: &[],
        subject_path: "crate::graph_subject",
        unwind: 8,
    }
}

fn mutant_harness(authored: &Authored, mutant: Mutant) -> KaniGraphObligationHarness {
    let (population, upper) = mutant.bounds();
    render_with_bounds(
        &request(),
        &authored.package,
        &authored.clause,
        &authored.occurrence,
        &[0],
        population,
        upper,
    )
    .expect("the production renderer admits the mutated bound")
}

/// The projections the expectation is checked against: the package's own population selection and
/// relationship end of the claim's reachability application.
fn projected(authored: &Authored) -> Declared {
    let application = authored
        .package
        .resolved_graph()
        .nodes
        .iter()
        .map(|node| node.node_id.clone())
        .find(|id| {
            matches!(
                authored.package.term(id),
                Some(CheckedTerm::Application { operation, .. })
                    if operation.identity() == Some("quire.op.model.reaches")
            )
        })
        .expect("the claim's reachability application");
    let selector = authored
        .package
        .reaches_population(&application)
        .expect("the application names a population")
        .expect("a selector");
    let Population::Finite(population) =
        select_population(CheckedSelectedPopulationSource::Admitted {
            package: &authored.package,
            selector,
        })
    else {
        panic!("a finite population");
    };
    let end = authored
        .package
        .relationship_end(&application)
        .expect("the end");
    Declared {
        population: population.maximum.as_str().parse().unwrap(),
        lower: end.lower as i64,
        upper: end.upper.expect("a bounded end") as i64,
        unique: end.unique,
    }
}

/// The identity-level facts a witness reports about the original claim and the generated domain.
#[derive(Debug, Eq, PartialEq)]
struct DomainWitness {
    clause: CheckedNodeId,
    edge: CheckedNodeId,
    count: i64,
    operands: (i64, i64),
    rows: Vec<(i64, Vec<i64>)>,
    original: Declared,
    generated: (u64, u64),
    capacity: (usize, usize),
    in_domain: bool,
    in_sampler: bool,
}

fn witness_of(
    original: &KaniGraphObligationHarness,
    mutant: &KaniGraphObligationHarness,
    declared: Declared,
    graph: &Graph,
) -> DomainWitness {
    let sampler = observe(&mutant.rust.contents);
    DomainWitness {
        clause: original.identity.clause_id.clone(),
        edge: original.identity.edge.declaration.clone(),
        count: graph.count,
        operands: (graph.start, graph.target),
        rows: (0..graph.count.min(ROWS as i64) as usize)
            .map(|row| {
                let degree = graph.degrees[row];
                (
                    degree,
                    graph.edges[row][..degree.clamp(0, COLS as i64) as usize].to_vec(),
                )
            })
            .collect(),
        original: declared,
        generated: (
            mutant.identity.maximum_population,
            mutant.identity.edge.upper,
        ),
        capacity: (sampler.rows, sampler.cols),
        in_domain: native_domain(declared, graph),
        in_sampler: sampler.admits(graph),
    }
}

/// Decodes a Falsified oracle run: the assertion Kani named, and the carrier graph it drew.
fn decode(
    vehicle: &KaniGraphObligationHarness,
    counterexample: &str,
) -> (Graph, Vec<&'static str>) {
    let playback = parse_assertion_playback(
        counterexample,
        vehicle.identity.harness_symbol.as_str(),
        vehicle.identity.module_symbol.as_str(),
        usize::MAX,
        usize::MAX,
    )
    .expect("the actual backend playback parses");
    let named: Vec<&'static str> = [CONFORMANCE, COMPLETENESS]
        .into_iter()
        .filter(|message| playback.matches_generated_assertion_message(message))
        .collect();
    let values: Vec<i64> = playback
        .entries
        .iter()
        .map(|(_, bytes)| i64::from_le_bytes(bytes[..8].try_into().expect("an i64 draw")))
        .collect();
    assert_eq!(
        values.len(),
        3 + ROWS + ROWS * COLS,
        "one value per carrier draw"
    );
    let mut graph = Graph {
        count: values[0],
        start: values[1],
        target: values[2],
        degrees: [0; ROWS],
        edges: [[0; COLS]; ROWS],
    };
    graph.degrees.copy_from_slice(&values[3..3 + ROWS]);
    for (row, slots) in values[3 + ROWS..].chunks(COLS).enumerate() {
        graph.edges[row].copy_from_slice(slots);
    }
    (graph, named)
}

fn verified(outcome: KaniRunOutcome, label: &str) {
    assert!(
        matches!(outcome, KaniRunOutcome::Verified),
        "{label}: expected Verified, got {outcome:?}"
    );
}

/// The healthy oracle verifies both implications with its cover satisfied.
fn healthy_oracle_verifies(end: End) {
    let authored = end.authored();
    let healthy = harness(&authored, &[0]);
    let label = format!("domain-healthy-{}", end.role());
    verified(
        run_outcome(&oracle(&healthy, end.declared(), &Run::Oracle), &label),
        &label,
    );
}

/// The healthy sampler's boundary probes: a full-population graph in both domains whose first row
/// has the end's lower, then its upper, degree.
fn healthy_probes_are_reachable(end: End) {
    let authored = end.authored();
    let healthy = harness(&authored, &[0]);
    for (name, degree) in [("lower", LOWER), ("upper", UPPER)] {
        let cover = format!("d && g && c_count == {POPULATION} && c_deg[0] == {degree}");
        let label = format!("domain-probe-{name}-{}", end.role());
        verified(
            run_outcome(
                &oracle(&healthy, end.declared(), &Run::Probe(&cover)),
                &label,
            ),
            &label,
        );
    }
}

/// A mutant is Falsified at the assertion it violates with a decoded witness whose independent
/// `D` and `G` results are the required mismatch, and its boundary probe is reachable.
fn mutant_is_detected(end: End, mutant: Mutant) {
    let authored = end.authored();
    let healthy = harness(&authored, &[0]);
    let sampler = mutant_harness(&authored, mutant);
    assert_eq!(sampler.identity.package, healthy.identity.package);
    assert_eq!(sampler.identity.clause_id, healthy.identity.clause_id);
    let label = format!("domain-{mutant:?}-{}", end.role());
    let oracle_run = oracle(&sampler, end.declared(), &Run::Oracle);
    let counterexample = run_falsified(&oracle_run, &label);
    let (graph, named) = decode(&oracle_run, &counterexample);
    assert_eq!(named, [mutant.violated()], "{label}: the failed assertion");
    let witness = witness_of(&healthy, &sampler, end.declared(), &graph);
    assert_eq!(
        (witness.in_sampler, witness.in_domain),
        (mutant.sampler_only(), !mutant.sampler_only()),
        "{label}: {witness:?}"
    );
    let (population, upper) = mutant.bounds();
    assert_eq!(witness.generated, (population, upper));
    assert_eq!(witness.original, end.declared());
    let probe = oracle(&sampler, end.declared(), &Run::Probe(&mutant.cover(end)));
    let probe_label = format!("{label}-probe");
    verified(run_outcome(&probe, &probe_label), &probe_label);
}

#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "real Kani qualification: run serially inside the flock and the 24G scope"]
fn real_kani_domain_oracle_verifies_a_unique_end() {
    let end = End::Unique;
    healthy_oracle_verifies(end);
}

#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "real Kani qualification: run serially inside the flock and the 24G scope"]
fn real_kani_domain_oracle_verifies_a_non_unique_end() {
    let end = End::NonUnique;
    healthy_oracle_verifies(end);
}

#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "real Kani qualification: run serially inside the flock and the 24G scope"]
fn real_kani_domain_boundary_probes_reach_a_unique_end() {
    let end = End::Unique;
    healthy_probes_are_reachable(end);
}

#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "real Kani qualification: run serially inside the flock and the 24G scope"]
fn real_kani_domain_boundary_probes_reach_a_non_unique_end() {
    let end = End::NonUnique;
    healthy_probes_are_reachable(end);
}

#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "real Kani qualification: run serially inside the flock and the 24G scope"]
fn real_kani_population_plus_one_violates_conformance_on_a_unique_end() {
    let end = End::Unique;
    mutant_is_detected(end, Mutant::PopulationAbove);
}

#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "real Kani qualification: run serially inside the flock and the 24G scope"]
fn real_kani_population_plus_one_violates_conformance_on_a_non_unique_end() {
    let end = End::NonUnique;
    mutant_is_detected(end, Mutant::PopulationAbove);
}

#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "real Kani qualification: run serially inside the flock and the 24G scope"]
fn real_kani_population_minus_one_violates_completeness_on_a_unique_end() {
    let end = End::Unique;
    mutant_is_detected(end, Mutant::PopulationBelow);
}

#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "real Kani qualification: run serially inside the flock and the 24G scope"]
fn real_kani_population_minus_one_violates_completeness_on_a_non_unique_end() {
    let end = End::NonUnique;
    mutant_is_detected(end, Mutant::PopulationBelow);
}

#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "real Kani qualification: run serially inside the flock and the 24G scope"]
fn real_kani_edge_upper_plus_one_violates_conformance_on_a_unique_end() {
    let end = End::Unique;
    mutant_is_detected(end, Mutant::EdgeUpperAbove);
}

#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "real Kani qualification: run serially inside the flock and the 24G scope"]
fn real_kani_edge_upper_plus_one_violates_conformance_on_a_non_unique_end() {
    let end = End::NonUnique;
    mutant_is_detected(end, Mutant::EdgeUpperAbove);
}

/// The expectation `D` is written from is the authored declaration, and the package's own
/// projections agree with it before any mutation.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
fn the_expected_domain_matches_the_package_projections() {
    for end in End::ALL {
        assert_eq!(projected(&end.authored()), end.declared(), "{end:?}");
    }
}

/// The observer reads the emitted sampler, and the native `D` and `G` give each boundary witness
/// the membership the spec requires: a mutant-only graph for `N + 1` and `U + 1`, an omitted
/// original-domain graph for `N - 1`, and healthy graphs in both domains.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
fn native_membership_agrees_with_the_boundary_witnesses_of_every_mutant() {
    for end in End::ALL {
        let authored = end.authored();
        let healthy = observe(&harness(&authored, &[0]).rust.contents);
        assert_eq!(
            (healthy.rows, healthy.cols),
            (POPULATION as usize, UPPER as usize)
        );
        for degree in [LOWER, UPPER] {
            let row: Vec<i64> = (0..degree).collect();
            let graph = Graph::with_rows(POPULATION, &[&row]);
            assert!(native_domain(end.declared(), &graph) && healthy.admits(&graph));
        }
        for mutant in Mutant::ALL {
            let sampler = observe(&mutant_harness(&authored, mutant).rust.contents);
            let (population, upper) = mutant.bounds();
            assert_eq!(
                (sampler.rows, sampler.cols),
                (population as usize, upper as usize),
                "{mutant:?}: storage follows the changed bound"
            );
            let graph = mutant.witness(end);
            assert_eq!(
                (
                    sampler.admits(&graph),
                    native_domain(end.declared(), &graph)
                ),
                (mutant.sampler_only(), !mutant.sampler_only()),
                "{mutant:?} {end:?}"
            );
            // The unmutated sampler agrees with the domain on the same graph.
            assert_eq!(
                healthy.admits(&graph),
                native_domain(end.declared(), &graph),
                "{mutant:?} {end:?}: healthy"
            );
        }
    }
}

/// The inclusive range the sampler draws `draw` from, read from its range assumption.
fn drawn_range(sampler: &Sampler, draw: &str) -> (i64, i64) {
    let prefix = format!("{draw} >= ");
    let condition = sampler
        .conditions
        .iter()
        .find(|condition| condition.starts_with(&prefix))
        .unwrap_or_else(|| panic!("no range assumption for {draw}"));
    let number = |text: &str| text.trim_end_matches("_i64").parse::<i64>().unwrap();
    let (lower, upper) = condition[prefix.len()..]
        .split_once(&format!(" && {draw} <= "))
        .expect("an inclusive range");
    (number(lower), number(upper))
}

/// The carrier's box holds every active value of the compared domains: every range the healthy
/// and mutated samplers draw from, and the authored domain, lies inside the carrier's count,
/// operand, degree and target extents, so no compared graph is cut off by the box.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
fn the_carrier_box_contains_every_sampler_range_and_the_authored_domain() {
    for end in End::ALL {
        let declared = end.declared();
        assert!(declared.population <= ROWS as i64 && declared.upper <= COLS as i64);
        let authored = end.authored();
        let mut samplers = vec![observe(&harness(&authored, &[0]).rust.contents)];
        samplers.extend(
            Mutant::ALL.map(|mutant| observe(&mutant_harness(&authored, mutant).rust.contents)),
        );
        for sampler in &samplers {
            for draw in &sampler.draws {
                let (lower, upper) = drawn_range(sampler, draw);
                let (box_lower, box_upper) = match draw.as_str() {
                    "object_count" => (0, ROWS as i64),
                    "start" | "target" => (0, ROWS as i64 - 1),
                    name if name.starts_with("degree_") => (0, COLS as i64),
                    _ => (0, ROWS as i64 - 1),
                };
                assert!(
                    box_lower <= lower && upper <= box_upper,
                    "{draw} range {lower}..={upper} leaves the carrier box"
                );
            }
        }
    }
}

/// `D` enforces a nonzero lower bound (the fixture's ends have lower bound zero, so the sampler's
/// lower-bound assumption is not exercised here, only the native predicate).
#[trace("TC-025", "FR-015-AC-106")]
#[test]
fn the_native_domain_predicate_enforces_a_nonzero_lower_bound() {
    let declared = Declared {
        lower: 1,
        ..End::NonUnique.declared()
    };
    assert!(!native_domain(declared, &Graph::with_rows(2, &[&[1], &[]])));
    assert!(native_domain(declared, &Graph::with_rows(2, &[&[1], &[0]])));
}

/// Graphs both domains judge by their own rule: a repeated target on a unique end is outside
/// both, a repeated target on a non-unique end is inside both, and a dangling target is outside
/// both.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
fn uniqueness_and_dangling_targets_are_part_of_both_membership_predicates() {
    let repeated = Graph::with_rows(POPULATION, &[&[0, 0]]);
    let unique_sampler = observe(&harness(&End::Unique.authored(), &[0]).rust.contents);
    let loose_sampler = observe(&harness(&End::NonUnique.authored(), &[0]).rust.contents);
    assert!(!native_domain(End::Unique.declared(), &repeated) && !unique_sampler.admits(&repeated));
    assert!(native_domain(End::NonUnique.declared(), &repeated) && loose_sampler.admits(&repeated));
    let dangling = Graph::with_rows(2, &[&[2]]);
    assert!(!native_domain(End::Unique.declared(), &dangling) && !unique_sampler.admits(&dangling));
}

/// Zeroing dormant padding preserves the sampler's admission: any dormant fill the sampler could
/// draw admits exactly as the zeroed graph does, for every profile and mutant sampler.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
fn zeroed_dormant_padding_preserves_the_samplers_admission() {
    let mut state = 0x9e37_79b9_7f4a_7c15_u64;
    let mut next = move |modulus: i64| {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((state >> 33) as i64) % modulus
    };
    for end in End::ALL {
        let authored = end.authored();
        let mut samplers = vec![observe(&harness(&authored, &[0]).rust.contents)];
        samplers.extend(
            Mutant::ALL.map(|mutant| observe(&mutant_harness(&authored, mutant).rust.contents)),
        );
        for sampler in &samplers {
            for _ in 0..400 {
                let mut graph = Graph::with_rows(next(ROWS as i64 + 1), &[]);
                graph.start = next(ROWS as i64);
                graph.target = graph.start;
                for row in 0..ROWS {
                    graph.degrees[row] = next(COLS as i64 + 1);
                    for column in 0..COLS {
                        graph.edges[row][column] = next(ROWS as i64);
                    }
                }
                let zeroed = graph.normalized();
                let mut filled = zeroed;
                for row in 0..ROWS {
                    for column in 0..COLS {
                        let dormant =
                            (row as i64) >= zeroed.count || (column as i64) >= zeroed.degrees[row];
                        if dormant {
                            filled.edges[row][column] = next(sampler.rows.max(1) as i64);
                        }
                    }
                    if (row as i64) >= zeroed.count {
                        filled.degrees[row] = next(sampler.cols as i64 + 1);
                    }
                }
                if zeroed.count == 0 {
                    filled.start = next(sampler.rows.max(1) as i64);
                    filled.target = filled.start;
                }
                assert_eq!(sampler.admits(&filled), sampler.admits(&zeroed));
            }
        }
    }
}

/// Each oracle run has exactly one cover, last in the harness, and no `kani::assume` mentions a
/// sampler draw or a membership predicate: only carrier slots (box and padding) are constrained.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
fn every_oracle_run_has_one_final_cover_and_assumes_only_carrier_slots() {
    let authored = End::Unique.authored();
    let sampler = mutant_harness(&authored, Mutant::EdgeUpperAbove);
    for run in [Run::Oracle, Run::Probe("g && !d")] {
        let source = oracle(&sampler, End::Unique.declared(), &run).rust.contents;
        assert_eq!(source.matches("kani::cover!").count(), 1);
        let proof = source.split("#[kani::proof]").nth(1).unwrap();
        let after_cover = proof
            .lines()
            .skip_while(|line| !line.trim().starts_with("kani::cover!"))
            .skip(1);
        assert!(after_cover.map(str::trim).all(|line| line == "}"));
        for line in proof.lines().filter(|line| line.contains("kani::assume")) {
            assert!(
                line.contains("c_") && !line.contains(" g ") && !line.contains("domain("),
                "{line}"
            );
            let names: Vec<&str> = line
                .split(|ch: char| !(ch.is_alphanumeric() || ch == '_'))
                .collect();
            for draw in &observe(&sampler.rust.contents).draws {
                assert!(!names.contains(&draw.as_str()), "{line} mentions {draw}");
            }
        }
        assert_eq!(
            source.matches("assert!(").count(),
            usize::from(matches!(run, Run::Oracle)) * 2
        );
    }
}

/// The mutant renders only the replaced bound: package, clause, edge declaration and uniqueness
/// stay those of the original claim.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
fn a_bound_mutant_changes_only_the_population_maximum_or_the_edge_upper_bound() {
    let authored = End::Unique.authored();
    let healthy = harness(&authored, &[0]);
    for mutant in Mutant::ALL {
        let sampler = mutant_harness(&authored, mutant);
        let (population, upper) = mutant.bounds();
        assert_eq!(sampler.identity.maximum_population, population);
        assert_eq!(sampler.identity.edge.upper, upper);
        let mut restored = sampler.identity.clone();
        restored.maximum_population = healthy.identity.maximum_population;
        restored.edge.upper = healthy.identity.edge.upper;
        restored.harness_symbol = healthy.identity.harness_symbol.clone();
        restored.module_symbol = healthy.identity.module_symbol.clone();
        restored.options = healthy.identity.options.clone();
        restored.witness_options = healthy.identity.witness_options.clone();
        assert_eq!(restored, healthy.identity, "{mutant:?}");
    }
}
