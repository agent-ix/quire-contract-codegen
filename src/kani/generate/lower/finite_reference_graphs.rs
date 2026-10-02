//! Identity-preserving finite-reference graph admission for generated backends.

use quire_contract_ir::kani::{
    lower_reaches, DispatchIndex, GraphLowering, GraphRequest, KaniOutcome, KaniProfile,
    ValidatedFiniteInput,
};

/// Produces the reviewed positive-length reachability plan before generated artifacts are rendered.
///
/// Identity, snapshot and reference validation remain owned by Contract IR; malformed or exhausted
/// graph requests retain typed non-Boolean outcomes and produce no partial artifact.
pub fn prepare_finite_graph_reaches(
    profile: &KaniProfile,
    dispatch: &DispatchIndex,
    input: &ValidatedFiniteInput,
    request: GraphRequest,
) -> Result<GraphLowering, KaniOutcome> {
    lower_reaches(profile, dispatch, input, request)
}
