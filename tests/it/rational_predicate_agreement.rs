//! Native body agreement, distinct from public replay and generated-proof acceptance.
use crate::rational_predicate_support::{admit, run_native, source_parameters, HEALTHY, MUTANT};
use ix_trace_rs::trace;
use quire_contract_codegen::{generate_rational_predicate_oracle, FunctionPredicateItem};

#[trace("TC-060", "FR-046-AC-2", "FR-046-AC-3", "FR-046-AC-4")]
#[test]
fn generated_native_bodies_preserve_exact_owner_results_and_all_charge_prefixes() {
    let mut oracles = Vec::new();
    for source in [HEALTHY, MUTANT] {
        let (package, function) = admit(source);
        oracles.push(
            generate_rational_predicate_oracle(FunctionPredicateItem {
                package: &package,
                function: &function,
            })
            .unwrap(),
        );
    }
    let indices = [HEALTHY, MUTANT]
        .into_iter()
        .zip(&oracles)
        .map(|(source, oracle)| {
            let original = source_parameters(source, "rational_commutative");
            for (prepared, (name, node)) in oracle.parameters().iter().zip(&original) {
                assert_eq!(&prepared.name, name);
                assert_eq!(prepared.node.digest.as_ref(), node);
            }
            (
                original.iter().position(|(name, _)| name == "a").unwrap(),
                original.iter().position(|(name, _)| name == "b").unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let harness = NATIVE_REFERENCE
        .replace("HEALTHY_A", &indices[0].0.to_string())
        .replace("HEALTHY_B", &indices[0].1.to_string())
        .replace("MUTANT_A", &indices[1].0.to_string())
        .replace("MUTANT_B", &indices[1].1.to_string());
    let stdout = run_native(
        &[
            ("healthy", &oracles[0].source),
            ("mutant", &oracles[1].source),
        ],
        &harness,
    );
    assert!(stdout.contains("canonical=111 tuples=12321 healthy_false=0 mutant_false=12210"));
    assert!(stdout.contains("all named denial occurrences and all ten budget boundaries agree"));
}

const NATIVE_REFERENCE: &str = r#"
use quire_exact::{AdmittedCharge,ChargePoint,InjectedDenial,Integer,IntegerInterval,LimitKind,Meter,Outcome,Rational,RationalArithmetic,RationalDomain,ScalarLimits,Value};
use std::num::NonZeroU64;

fn limits()->ScalarLimits {ScalarLimits {integer_bits:u64::MAX,decimal_digits:u64::MAX,scale_expansion:u64::MAX,text_input_bytes:u64::MAX,text_scalars:u64::MAX,normalized_scalars:u64::MAX,unit_edges:u64::MAX,value_occurrences:u64::MAX,work_units:u64::MAX,result_units:u64::MAX}}
fn bounded(mut l:ScalarLimits,kind:LimitKind,n:u64)->ScalarLimits {
 match kind {LimitKind::IntegerBits=>l.integer_bits=n,LimitKind::DecimalDigits=>l.decimal_digits=n,
 LimitKind::ScaleExpansion=>l.scale_expansion=n,LimitKind::TextInputBytes=>l.text_input_bytes=n,
 LimitKind::TextScalars=>l.text_scalars=n,LimitKind::NormalizedScalars=>l.normalized_scalars=n,
 LimitKind::UnitEdges=>l.unit_edges=n,LimitKind::ValueOccurrences=>l.value_occurrences=n,
 LimitKind::WorkUnits=>l.work_units=n,LimitKind::ResultUnits=>l.result_units=n};l
}
#[derive(Debug,PartialEq,Eq)]struct Snapshot {outcome:Outcome<bool>,charges:Vec<AdmittedCharge>,counters:Vec<u64>,admissions:u64}
fn observe(f:impl FnOnce(&mut Meter)->Outcome<bool>,limits:ScalarLimits,deny:Option<InjectedDenial>)->Snapshot {
 let mut meter=Meter::new(limits);if let Some(denial)=deny {meter=meter.with_injected_denial(denial);}
 let outcome=f(&mut meter);assert!(!meter.charge_log_truncated());
 Snapshot{outcome,charges:meter.admitted_charges().to_vec(),counters:LimitKind::ALL.iter().map(|k|meter.consumed(*k)).collect(),admissions:meter.admission_count()}
}
fn reference(a:&Rational,b:&Rational,mutant:bool,meter:&mut Meter)->Outcome<bool> {
 // This independent source-authored tree uses owner operations, never emitted helper selection.
 // The original source's inferred add domain is [-162,162;1,81], not its input domain.
 let domain=RationalDomain::new(IntegerInterval::new(Integer::from(-162_i64),Integer::from(162_i64)).unwrap(),IntegerInterval::new(Integer::from(1_i64),Integer::from(81_i64)).unwrap()).unwrap();
 macro_rules! complete {($value:expr)=>{match $value {Outcome::Completed(v)=>v,Outcome::Undefined(v)=>return Outcome::Undefined(v),Outcome::Refused(v)=>return Outcome::Refused(v),Outcome::Incomplete(v)=>return Outcome::Incomplete(v)}};}
 let left=complete!(quire_exact::evaluate_rational_arithmetic(RationalArithmetic::Add(a,b),Some(&domain),meter));
 let right=if mutant {complete!(quire_exact::evaluate_rational_arithmetic(RationalArithmetic::Add(a,a),Some(&domain),meter))}
 else {complete!(quire_exact::evaluate_rational_arithmetic(RationalArithmetic::Add(b,a),Some(&domain),meter))};
 quire_exact::planned_equality(&Value::Rational(left),&Value::Rational(right),meter)
}
fn exercise(a:&Rational,b:&Rational,mutant:bool)->bool {
 let expected=|m:&mut Meter|reference(a,b,mutant,m);
 let actual=|m:&mut Meter| {let mut args=vec![a.clone(),b.clone()];
 if mutant {args[MUTANT_A]=a.clone();args[MUTANT_B]=b.clone();super::mutant::predicate(&args,m).unwrap()}
 else {args[HEALTHY_A]=a.clone();args[HEALTHY_B]=b.clone();super::healthy::predicate(&args,m).unwrap()}};
 let full=observe(expected,limits(),None);assert_eq!(observe(actual,limits(),None),full);
 let mut occurrences:Vec<(ChargePoint,u64)>=Vec::new();
 for charge in &full.charges {
 let count=if let Some((_,count))=occurrences.iter_mut().find(|(point,_)|*point==charge.point){*count+=1;*count}
 else {occurrences.push((charge.point,1));1};
 let deny=Some(InjectedDenial{point:charge.point,occurrence:NonZeroU64::new(count).unwrap()});
 let stopped=observe(expected,limits(),deny);assert!(matches!(stopped.outcome,Outcome::Incomplete(_)));
 assert_eq!(observe(actual,limits(),deny),stopped);
 }
 for (index,kind) in LimitKind::ALL.iter().enumerate() {
 for bound in [0,full.counters[index],full.counters[index].saturating_sub(1)] {
 let limited=bounded(limits(),*kind,bound);
 assert_eq!(observe(actual,limited.clone(),None),observe(expected,limited,None));
 }}
 matches!(full.outcome,Outcome::Completed(false))
}
#[test] fn whole_original_domain_and_denial_prefixes() {
 let mut values=Vec::new();for n in -9_i64..=9 {for d in 1_i64..=9 {
 let value=Rational::new(Integer::from(n),Integer::from(d)).unwrap();
 if value.numerator()==&Integer::from(n) && value.denominator()==&Integer::from(d){values.push(value);}
 }}assert_eq!(values.len(),111);
 let mut healthy_false=0;let mut mutant_false=0;
 for a in &values {for b in &values {healthy_false+=usize::from(exercise(a,b,false));mutant_false+=usize::from(exercise(a,b,true));}}
 assert_eq!(healthy_false,0);assert_eq!(mutant_false,12210);
 println!("canonical=111 tuples=12321 healthy_false={healthy_false} mutant_false={mutant_false}");
 println!("all named denial occurrences and all ten budget boundaries agree");
}
"#;

#[trace("TC-060", "FR-046-AC-3", "FR-046-AC-4")]
#[test]
fn admitted_nested_addition_preserves_arbitrary_precision_and_stops() {
    let source = include_str!("rational_predicate_support/fixtures/large_intermediate.native");
    let (package, function) = admit(source);
    let oracle = generate_rational_predicate_oracle(FunctionPredicateItem {
        package: &package,
        function: &function,
    })
    .unwrap();
    let original = source_parameters(source, "rational_large_intermediate");
    for (prepared, (name, node)) in oracle.parameters().iter().zip(&original) {
        assert_eq!(&prepared.name, name);
        assert_eq!(prepared.node.digest.as_ref(), node);
    }
    let a = original.iter().position(|(name, _)| name == "a").unwrap();
    let b = original.iter().position(|(name, _)| name == "b").unwrap();
    let shared = NATIVE_REFERENCE.split("fn reference(").next().unwrap();
    let specific = LARGE_REFERENCE
        .replace("ARG_A", &a.to_string())
        .replace("ARG_B", &b.to_string());
    let stdout = run_native(&[("huge", &oracle.source)], &format!("{shared}{specific}"));
    assert!(stdout
        .contains("beyond-i128 intermediate, nested tree, all denial prefixes and budgets agree"));
}

const LARGE_REFERENCE: &str = r#"
fn reference(a:&Rational,b:&Rational,meter:&mut Meter)->Outcome<bool> {
 macro_rules! complete {($value:expr)=>{match $value {Outcome::Completed(v)=>v,Outcome::Undefined(v)=>return Outcome::Undefined(v),Outcome::Refused(v)=>return Outcome::Refused(v),Outcome::Incomplete(v)=>return Outcome::Incomplete(v)}};}
 let ab=complete!(quire_exact::evaluate_rational_arithmetic(RationalArithmetic::Add(a,b),None,meter));
 let ab_again=complete!(quire_exact::evaluate_rational_arithmetic(RationalArithmetic::Add(a,b),None,meter));
 let left=complete!(quire_exact::evaluate_rational_arithmetic(RationalArithmetic::Add(&ab,&ab_again),None,meter));
 let ba=complete!(quire_exact::evaluate_rational_arithmetic(RationalArithmetic::Add(b,a),None,meter));
 let ba_again=complete!(quire_exact::evaluate_rational_arithmetic(RationalArithmetic::Add(b,a),None,meter));
 let right=complete!(quire_exact::evaluate_rational_arithmetic(RationalArithmetic::Add(&ba,&ba_again),None,meter));
 quire_exact::planned_equality(&Value::Rational(left),&Value::Rational(right),meter)
}
#[test]fn owner_big_continuation_and_every_stop_prefix() {
 let maximum=i64::MAX;
 let a=Rational::new(Integer::from(maximum),Integer::from(maximum-2)).unwrap();
 let b=Rational::new(Integer::from(maximum),Integer::from(maximum-4)).unwrap();
 assert!(a.add(&b).add(&a.add(&b)).numerator().magnitude_bits()>127);
 for (a,b) in [(a.clone(),b.clone()),(a.neg(),b),(a.clone(),a)] {
 let expected=|m:&mut Meter|reference(&a,&b,m);
 let actual=|m:&mut Meter|{let mut args=vec![a.clone(),b.clone()];args[ARG_A]=a.clone();args[ARG_B]=b.clone();super::huge::predicate(&args,m).unwrap()};
 let full=observe(expected,limits(),None);assert_eq!(full.outcome,Outcome::Completed(true));assert_eq!(observe(actual,limits(),None),full);
 let mut occurrences:Vec<(ChargePoint,u64)>=Vec::new();
 for charge in &full.charges {
 let count=if let Some((_,count))=occurrences.iter_mut().find(|(point,_)|*point==charge.point){*count+=1;*count}else{occurrences.push((charge.point,1));1};
 let denial=Some(InjectedDenial{point:charge.point,occurrence:NonZeroU64::new(count).unwrap()});
 let stopped=observe(expected,limits(),denial);assert!(matches!(stopped.outcome,Outcome::Incomplete(_)));assert_eq!(observe(actual,limits(),denial),stopped);
 }
 for (index,kind) in LimitKind::ALL.iter().enumerate(){for bound in [0,full.counters[index],full.counters[index].saturating_sub(1)]{
 let budget=bounded(limits(),*kind,bound);assert_eq!(observe(actual,budget.clone(),None),observe(expected,budget,None));}}
 }
 println!("beyond-i128 intermediate, nested tree, all denial prefixes and budgets agree");
}
"#;
