//! Observe the actual emitted sampler. Unrecognized admission or storage syntax refuses.
use super::CandidateGraph;
use quote::ToTokens;
use std::collections::{BTreeMap, BTreeSet};
use syn::{Expr, Item, Pat, Stmt};

#[derive(serde::Serialize)]
pub(super) struct SamplerObservation {
    pub(super) population_capacity: usize,
    pub(super) row_capacities: Vec<usize>,
    pub(super) scalar_bindings: BTreeMap<String, String>,
    pub(super) assumptions: Vec<String>,
    #[serde(skip)]
    predicates: Vec<Expr>,
    pub(super) input_names: Vec<String>,
    pub(super) scalar_ranges: BTreeMap<String, (i64, i64)>,
}
fn tokens(value: &impl ToTokens) -> String {
    value.to_token_stream().to_string()
}
fn name(expr: &Expr) -> String {
    let Expr::Path(path) = expr else {
        panic!("sampler binding is a scalar name")
    };
    path.path.get_ident().unwrap().to_string()
}
fn local_name(local: &syn::Local) -> String {
    match &local.pat {
        Pat::Ident(pat) => pat.ident.to_string(),
        Pat::Type(pat) => {
            let Pat::Ident(pat) = pat.pat.as_ref() else {
                panic!("scalar binding")
            };
            pat.ident.to_string()
        }
        _ => panic!("unrecognized sampler binding"),
    }
}
fn array(expr: &Expr) -> Vec<Expr> {
    let Expr::Array(array) = expr else {
        panic!("actual fixed array required")
    };
    array.elems.iter().cloned().collect()
}
fn array_type(ty: &syn::Type) -> (usize, &syn::Type) {
    let syn::Type::Array(array) = ty else {
        panic!("fixed array type")
    };
    let Expr::Lit(value) = &array.len else {
        panic!("literal capacity")
    };
    let syn::Lit::Int(value) = &value.lit else {
        panic!("integer capacity")
    };
    (value.base10_parse().unwrap(), array.elem.as_ref())
}
fn parameter_type(arg: &syn::FnArg) -> &syn::Type {
    let syn::FnArg::Typed(arg) = arg else {
        panic!("function argument")
    };
    let syn::Type::Reference(arg) = arg.ty.as_ref() else {
        panic!("borrowed storage")
    };
    arg.elem.as_ref()
}

impl SamplerObservation {
    pub(super) fn parse(source: &str, proof_name: &str) -> Self {
        let parsed = syn::parse_file(source).expect("actual generated Rust parses");
        let proof = parsed
            .items
            .iter()
            .find_map(|item| {
                let Item::Mod(module) = item else { return None };
                module.content.as_ref()?.1.iter().find_map(|item| {
                    let Item::Fn(function) = item else {
                        return None;
                    };
                    (function.sig.ident == proof_name).then_some(function)
                })
            })
            .expect("selected actual proof");
        let mut inputs = Vec::new();
        let mut predicates = Vec::new();
        let mut scalar_ranges = BTreeMap::new();
        let mut pending_range = None;
        let mut arrays = BTreeMap::new();
        let mut call = None;
        for statement in &proof.block.stmts {
            match statement {
                Stmt::Local(local) => {
                    let init = &local
                        .init
                        .as_ref()
                        .expect("initialized sampler binding")
                        .expr;
                    match init.as_ref() {
                        Expr::Call(expr) if tokens(&expr.func) == "kani :: any" => {
                            let Pat::Type(typed) = &local.pat else {
                                panic!("typed scalar draw")
                            };
                            assert_eq!(tokens(&typed.ty), "i64");
                            assert!(pending_range.is_none(), "each draw needs its actual range");
                            pending_range = Some(local_name(local));
                            inputs.push(local_name(local));
                        }
                        Expr::Call(expr) => {
                            call = Some(expr);
                            break;
                        }
                        Expr::Array(_) => {
                            arrays.insert(local_name(local), array(init));
                        }
                        _ => panic!("unobserved sampler initialization: {}", tokens(statement)),
                    }
                }
                Stmt::Expr(Expr::Call(call), Some(_)) if tokens(&call.func) == "kani :: assume" => {
                    assert_eq!(call.args.len(), 1);
                    let predicate = call.args[0].clone();
                    if let Some(symbol) = pending_range.take() {
                        let Expr::Binary(both) = &predicate else {
                            panic!("input range conjunction")
                        };
                        assert!(matches!(both.op, syn::BinOp::And(_)));
                        let (Expr::Binary(lower), Expr::Binary(upper)) =
                            (both.left.as_ref(), both.right.as_ref())
                        else {
                            panic!("inclusive input range")
                        };
                        assert!(
                            matches!(lower.op, syn::BinOp::Ge(_))
                                && matches!(upper.op, syn::BinOp::Le(_))
                        );
                        assert_eq!(name(&lower.left), symbol);
                        assert_eq!(name(&upper.left), symbol);
                        scalar_ranges.insert(
                            symbol,
                            (
                                evaluate(&lower.right, &BTreeMap::new()),
                                evaluate(&upper.right, &BTreeMap::new()),
                            ),
                        );
                    }
                    predicates.push(predicate);
                }
                _ => panic!("unobserved sampler statement: {}", tokens(statement)),
            }
        }
        let call = call.expect("sampler's actual evaluator call");
        assert!(pending_range.is_none());
        assert_eq!(call.args.len(), 5);
        let helper = name(&call.func);
        let degrees_name = match &call.args[1] {
            Expr::Reference(r) => name(&r.expr),
            _ => panic!("degrees"),
        };
        let edges_name = match &call.args[2] {
            Expr::Reference(r) => name(&r.expr),
            _ => panic!("edges"),
        };
        let degrees = arrays.remove(&degrees_name).unwrap();
        let rows = arrays.remove(&edges_name).unwrap();
        assert!(arrays.is_empty(), "no unobserved sampler arrays");
        assert_eq!(degrees.len(), rows.len());
        assert!(
            rows.len() <= 5,
            "union carrier must hold production capacity"
        );
        let row_values: Vec<_> = rows.iter().map(array).collect();
        let row_capacities: Vec<_> = row_values.iter().map(Vec::len).collect();
        assert!(row_capacities.iter().all(|n| *n <= 4));
        let mut bindings = BTreeMap::from([
            (name(&call.args[0]), "g.count".into()),
            (
                name(&call.args[3]),
                "if g.count == 0 { 0 } else { g.start }".into(),
            ),
            (
                name(&call.args[4]),
                "if g.count == 0 { 0 } else { g.target }".into(),
            ),
        ]);
        for (row, degree) in degrees.iter().enumerate() {
            bindings.insert(
                name(degree),
                format!("if g.count > {row} {{ g.degrees[{row}] }} else {{ 0 }}"),
            );
            for (slot, edge) in row_values[row].iter().enumerate() {
                bindings.insert(name(edge), format!("if g.count > {row} && g.degrees[{row}] > {slot} {{ g.edges[{row}][{slot}] }} else {{ 0 }}"));
            }
        }
        assert_eq!(inputs.iter().collect::<BTreeSet<_>>().len(), inputs.len());
        assert_eq!(
            inputs.iter().cloned().collect::<BTreeSet<_>>(),
            bindings.keys().cloned().collect()
        );
        // The evaluator consumes exactly these arrays and count. Recognize its active-slot
        // interpretation, rather than assuming padding can be projected away for any helper.
        let function = parsed
            .items
            .iter()
            .find_map(|item| match item {
                Item::Fn(f) if f.sig.ident == helper => Some(f),
                _ => None,
            })
            .unwrap();
        let (degree_capacity, _) = array_type(parameter_type(&function.sig.inputs[1]));
        let (row_capacity, edge_type) = array_type(parameter_type(&function.sig.inputs[2]));
        let (slot_capacity, _) = array_type(edge_type);
        assert_eq!(degree_capacity, degrees.len());
        assert_eq!(row_capacity, rows.len());
        assert!(row_capacities.iter().all(|n| *n == slot_capacity));
        let n = rows.len();
        let expected: syn::Block = syn::parse_str(&format!("{{ let mut paths = [[false; {n}]; {n}]; for row in 0..{n} {{ for edge in 0..{slot_capacity} {{ if row < count as usize && edge < degrees[row] as usize {{ paths[row][edges[row][edge] as usize] = true; }} }} }} }}")).unwrap();
        assert_eq!(tokens(&function.block.stmts[0]), tokens(&expected.stmts[0]));
        assert_eq!(
            tokens(&function.block.stmts[1]),
            tokens(&expected.stmts[1]),
            "unrecognized active graph interpretation"
        );
        // Every original sampler assumption is interpreted, including scalar range assumptions.
        let assumptions = predicates.iter().map(tokens).collect();
        let result = Self {
            population_capacity: n,
            row_capacities,
            scalar_bindings: bindings,
            assumptions,
            predicates,
            input_names: inputs,
            scalar_ranges,
        };
        // Dormant normalization is valid only when the actual range/guard predicates admit
        // zero padding. The full healthy D=>G check additionally tests all active graphs.
        assert!(result.admits(&CandidateGraph {
            count: 0,
            start: 0,
            target: 0,
            degrees: [4; 5],
            edges: [[4; 4]; 5]
        }));
        result
    }

    fn values(&self, g: &CandidateGraph) -> Option<BTreeMap<String, i64>> {
        if g.count < 0 || g.count > self.population_capacity as i64 {
            return None;
        }
        for row in 0..usize::try_from(g.count).ok()? {
            if g.degrees[row] < 0 || g.degrees[row] > self.row_capacities[row] as i64 {
                return None;
            }
        }
        // This uses the same observed position map as the emitted predicate, with no bound
        // or multiplicity inference. Values are assigned only after capacity rejection.
        let mut values = BTreeMap::new();
        for (symbol, binding) in &self.scalar_bindings {
            let value = if binding == "g.count" {
                g.count
            } else if binding.contains("g.start") {
                if g.count == 0 {
                    0
                } else {
                    g.start
                }
            } else if binding.contains("g.target") {
                if g.count == 0 {
                    0
                } else {
                    g.target
                }
            } else {
                let expression: Expr = syn::parse_str(binding).unwrap();
                eval_candidate(&expression, g)
            };
            values.insert(symbol.clone(), value);
        }
        Some(values)
    }
    pub(super) fn admits(&self, g: &CandidateGraph) -> bool {
        self.values(g)
            .is_some_and(|values| self.predicates.iter().all(|p| evaluate(p, &values) != 0))
    }
    pub(super) fn source(&self) -> String {
        let mut source = format!("fn generated_admits(g: &CandidateGraph) -> bool {{\nif g.count < 0 || g.count > {} {{ return false; }}\n", self.population_capacity);
        for (row, slots) in self.row_capacities.iter().enumerate() {
            source.push_str(&format!("if g.count > {row} && (g.degrees[{row}] < 0 || g.degrees[{row}] > {slots}) {{ return false; }}\n"));
        }
        for (symbol, binding) in &self.scalar_bindings {
            source.push_str(&format!("let {symbol}: i64 = {binding};\n"));
        }
        for predicate in &self.assumptions {
            source.push_str(&format!("if !({predicate}) {{ return false; }}\n"));
        }
        source.push_str("true\n}\n");
        source
    }
}

fn evaluate(expr: &Expr, values: &BTreeMap<String, i64>) -> i64 {
    match expr {
        Expr::Lit(lit) => match &lit.lit {
            syn::Lit::Int(n) => n.base10_parse().unwrap(),
            syn::Lit::Bool(b) => i64::from(b.value),
            _ => panic!("unsupported sampler literal"),
        },
        Expr::Path(_) => *values
            .get(&name(expr))
            .expect("sampler variable is observed"),
        Expr::Paren(expr) => evaluate(&expr.expr, values),
        Expr::Binary(binary) => {
            let left = evaluate(&binary.left, values);
            match binary.op {
                syn::BinOp::And(_) if left == 0 => return 0,
                syn::BinOp::Or(_) if left != 0 => return 1,
                _ => {}
            }
            let right = evaluate(&binary.right, values);
            i64::from(match binary.op {
                syn::BinOp::And(_) => left != 0 && right != 0,
                syn::BinOp::Or(_) => left != 0 || right != 0,
                syn::BinOp::Eq(_) => left == right,
                syn::BinOp::Ne(_) => left != right,
                syn::BinOp::Lt(_) => left < right,
                syn::BinOp::Le(_) => left <= right,
                syn::BinOp::Gt(_) => left > right,
                syn::BinOp::Ge(_) => left >= right,
                _ => panic!("unsupported sampler binary operator"),
            })
        }
        _ => panic!("unsupported sampler assumption: {}", tokens(expr)),
    }
}
fn eval_candidate(expr: &Expr, g: &CandidateGraph) -> i64 {
    match expr {
        Expr::If(branch) => {
            let condition = eval_candidate(&branch.cond, g);
            let block = if condition != 0 {
                &branch.then_branch
            } else {
                let (_, other) = branch.else_branch.as_ref().unwrap();
                let Expr::Block(block) = other.as_ref() else {
                    panic!("normalization else block")
                };
                &block.block
            };
            let Stmt::Expr(expr, None) = block.stmts.last().unwrap() else {
                panic!("normalization value")
            };
            eval_candidate(expr, g)
        }
        Expr::Binary(binary) => {
            let l = eval_candidate(&binary.left, g);
            let r = eval_candidate(&binary.right, g);
            i64::from(match binary.op {
                syn::BinOp::And(_) => l != 0 && r != 0,
                syn::BinOp::Gt(_) => l > r,
                _ => panic!("normalization operator"),
            })
        }
        Expr::Field(field) => match &field.member {
            syn::Member::Named(member) if member == "count" => g.count,
            _ => panic!("normalization scalar"),
        },
        Expr::Index(index) => {
            let slot = usize::try_from(eval_candidate(&index.index, g)).unwrap();
            match index.expr.as_ref() {
                Expr::Field(field) if tokens(&field.member) == "degrees" => g.degrees[slot],
                Expr::Index(row) => {
                    let row = usize::try_from(eval_candidate(&row.index, g)).unwrap();
                    g.edges[row][slot]
                }
                _ => panic!("normalization array"),
            }
        }
        Expr::Lit(_) => evaluate(expr, &BTreeMap::new()),
        _ => panic!("normalization syntax"),
    }
}
