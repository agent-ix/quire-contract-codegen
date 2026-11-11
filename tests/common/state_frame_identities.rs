//! Hand-written QSL obligation identities for the state-frame lane (FR-015-AC-137). Each is a
//! literal the tests compare against, never derived from the code under test.

use qsl_replay::{ModelIdentity, NodeKey, ObligationIdentity};
use quire_contract_codegen::StateFrameRoleIdentities;
use quire_exact::PathSegment;

fn key(package: &str, path: &str, ordinal: u64) -> NodeKey {
    NodeKey::new(package, vec![PathSegment::Text(path.to_owned())], ordinal).expect("a node key")
}

/// The `contract` role's identity: no model, ordinal 3.
pub fn contract() -> ObligationIdentity {
    ObligationIdentity::new(key("ids", "contract", 7), None, 3)
}

/// The `contract` identity with only its ordinal changed.
pub fn contract_ordinal_only() -> ObligationIdentity {
    ObligationIdentity::new(key("ids", "contract", 7), None, 4)
}

/// The `contract` identity with only its node changed.
pub fn contract_node_only() -> ObligationIdentity {
    ObligationIdentity::new(key("ids", "contract", 8), None, 3)
}

/// The `contract` identity with only a model added.
pub fn contract_model_only() -> ObligationIdentity {
    ObligationIdentity::new(
        key("ids", "contract", 7),
        Some(ModelIdentity::new("model-a")),
        3,
    )
}

/// The `frame` role's identity: differs from [`contract`] in node, model and ordinal.
pub fn frame() -> ObligationIdentity {
    ObligationIdentity::new(
        key("ids", "frame", 9),
        Some(ModelIdentity::new("model-b")),
        5,
    )
}

/// An identity whose node path holds a `Node` segment that itself nests a key with a `Node`
/// segment.
pub fn nested() -> ObligationIdentity {
    let inner = NodeKey::new(
        "ids",
        vec![
            PathSegment::Text("leaf".to_owned()),
            PathSegment::Text(String::new()),
        ],
        1,
    )
    .expect("inner key");
    let middle = NodeKey::new(
        "ids",
        vec![
            PathSegment::Text("middle".to_owned()),
            PathSegment::Node(inner),
        ],
        2,
    )
    .expect("middle key");
    let outer = NodeKey::new(
        "ids",
        vec![
            PathSegment::Node(middle),
            PathSegment::Text("tail".to_owned()),
        ],
        3,
    )
    .expect("outer key");
    ObligationIdentity::new(outer, Some(ModelIdentity::new("model-n")), 11)
}

/// The pair the generation tests use.
pub fn roles() -> StateFrameRoleIdentities {
    StateFrameRoleIdentities::new(contract(), frame()).expect("distinct identities")
}

static ROLES: std::sync::LazyLock<StateFrameRoleIdentities> = std::sync::LazyLock::new(roles);

/// The identity of `role` in [`roles`], for a `StateFrame` item that borrows it.
pub fn for_role(role: quire_contract_codegen::StateFrameRole) -> &'static ObligationIdentity {
    match role {
        quire_contract_codegen::StateFrameRole::Contract => ROLES.contract(),
        quire_contract_codegen::StateFrameRole::Frame => ROLES.frame(),
    }
}
