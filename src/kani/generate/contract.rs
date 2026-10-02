//! The contract family: rendering the Kani contract harness of a V1 postcondition or invariant
//! clause (FR-015).

use crate::{
    kani::abi::i64_literal,
    kani::generate::clause::{call, symbolic_arguments, Abi, LoweredClause, SlotContext},
    kani::identity::ObligationKind,
};

/// The non-vacuity cover every contract harness ends with. It is reachable only on a path where
/// the IR argument bounds and every `requires` hold together, so an unsatisfied cover means the
/// `ensures` was never checked.
const CONTRACT_COVER: &str = "contract requires and IR bounds are jointly satisfiable";

pub(super) fn render_contract(
    subject_path: &str,
    lowered: &LoweredClause<'_>,
    abi: &Abi,
) -> Option<String> {
    let mut requires = lowered
        .assumed
        .iter()
        .map(|oracle| call(oracle, SlotContext::Precondition, abi))
        .collect::<Option<Vec<_>>>()?;
    let obligation = match lowered.kind {
        ObligationKind::Postcondition => call(&lowered.oracle, SlotContext::Postcondition, abi)?,
        ObligationKind::Invariant => {
            requires.push(call(&lowered.oracle, SlotContext::InvariantBefore, abi)?);
            call(&lowered.oracle, SlotContext::InvariantAfter, abi)?
        }
        ObligationKind::Precondition | ObligationKind::Frame => return None,
    };
    let result_bounds = abi
        .results
        .iter()
        .filter_map(|binding| {
            let bounds = binding.integer_bounds.as_ref()?;
            let access = abi.access(&binding.identifier)?;
            Some(format!(
                "{access} >= {} && {access} <= {}",
                i64_literal(bounds.minimum),
                i64_literal(bounds.maximum)
            ))
        })
        .collect::<Vec<_>>();
    let ensures = if result_bounds.is_empty() {
        obligation
    } else {
        format!("({}) && {obligation}", result_bounds.join(" && "))
    };
    let result_type = match abi.results.as_slice() {
        [] => "()".to_owned(),
        [binding] => binding.primitive_type.source_name().to_owned(),
        bindings => format!(
            "({})",
            bindings
                .iter()
                .map(|binding| binding.primitive_type.source_name())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    };
    let post_state = if abi.results.is_empty() {
        "_post_state"
    } else {
        "post_state"
    };
    let declarations = abi
        .arguments
        .iter()
        .map(|binding| {
            format!(
                "{}: {}",
                binding.identifier,
                binding.primitive_type.source_name()
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    let names = abi
        .arguments
        .iter()
        .map(|binding| binding.identifier.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    let requires = requires
        .iter()
        .map(|predicate| format!("    #[kani::requires({predicate})]\n"))
        .collect::<String>();
    Some(format!(
        "#[cfg(kani)]\n\
mod {module} {{\n\
    use super::*;\n\
\n\
{requires}    #[kani::ensures(|{post_state}: &{result_type}| {ensures})]\n\
    fn {contract}({declarations}) -> {result_type} {{\n\
        {subject_path}({names})\n\
    }}\n\
\n\
    #[kani::proof_for_contract({contract})]\n\
    fn {harness}() {{\n\
{arguments}        let _ = {contract}({names});\n\
        kani::cover!(true, \"{CONTRACT_COVER}\");\n\
    }}\n\
}}\n",
        module = lowered.symbols.module,
        contract = lowered.symbols.contract,
        harness = lowered.symbols.harness,
        arguments = symbolic_arguments(&abi.arguments),
    ))
}
