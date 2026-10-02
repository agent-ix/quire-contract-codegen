//! The precondition family: rendering the Kani harness of a V1 precondition clause (FR-015).

use crate::kani::generate::clause::{call, symbolic_arguments, Abi, LoweredClause, SlotContext};

pub(super) fn render_precondition(lowered: &LoweredClause<'_>, abi: &Abi) -> Option<String> {
    let holds = call(&lowered.oracle, SlotContext::Precondition, abi)?;
    Some(format!(
        "#[cfg(kani)]\n\
mod {module} {{\n\
    use super::*;\n\
\n\
    #[kani::proof]\n\
    fn {harness}() {{\n\
{arguments}        let precondition_holds = {holds};\n\
        kani::cover!(precondition_holds, \"precondition is satisfiable within the IR bounds\");\n\
    }}\n\
}}\n",
        module = lowered.symbols.module,
        harness = lowered.symbols.harness,
        arguments = symbolic_arguments(&abi.arguments),
    ))
}
