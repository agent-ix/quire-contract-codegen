//! The identity of one Kani obligation (ADR-013 O-09, IR-15): a digest over the obligation's
//! clause, its kind and its `arguments`, and over nothing else.
//!
//! `arguments` are the harness's symbolic bindings, ascending by identifier. Each names the
//! finite domain the proof assumed of it, and the
//! harness emits one `kani::any()` per binding in that order, so position *i* of the bindings is
//! position *i* of Kani's playback. The clause's source span, the generated symbols, the solver
//! and the unwind bound are not part of the preimage: two harnesses that prove the same clause
//! of the same kind over the same bindings are one obligation, however they were rendered.
//!
//! Both obligation families, the clause-oracle harnesses of [`crate::kani_obligations`] and the
//! state-frame harnesses of [`crate::state_frame`], mint their identity here, once.

use std::fmt;

use qsl_replay::ByteDigest;
use serde::{Serialize, Serializer};
use serde_json::Value;

use crate::{
    kani::{KaniBindingRole, KaniIntegerBounds, KaniPrimitiveType},
    kani_obligations::{KaniObligationIdentity, ObligationBinding, ObligationKind},
};

/// Domain separation of the preimage.
const PREIMAGE_DOMAIN: &str = "quire.codegen.kani-obligation-identity/v1";

/// The digest of one obligation's identity.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ObligationDigest([u8; 32]);

impl ObligationDigest {
    /// The digest bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Display for ObligationDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().try_for_each(|byte| write!(f, "{byte:02x}"))
    }
}

impl fmt::Debug for ObligationDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ObligationDigest({self})")
    }
}

impl Serialize for ObligationDigest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

/// Why no identity could be minted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IdentityRefusal {
    /// The bindings are not strictly ascending by identifier, so the harness argument order is
    /// not the order the identity commits to.
    ArgumentsNotAscending {
        /// The first identifier that does not follow its predecessor.
        identifier: String,
    },
    /// The preimage did not serialize.
    Preimage,
}

impl fmt::Display for IdentityRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArgumentsNotAscending { identifier } => write!(
                f,
                "binding `{identifier}` does not follow its predecessor in ascending order"
            ),
            Self::Preimage => f.write_str("the identity preimage did not serialize"),
        }
    }
}

impl std::error::Error for IdentityRefusal {}

/// A binding as the identity commits to it: everything that decides what the position ranges
/// over, and nothing that only describes how it was rendered.
///
/// The binding's domain content is its identifier and its declared range. The domain-key form
/// (the declaring node and a path into its type) is pending QSL's key shape under QSL-345, and
/// the preimage gains it then, which re-mints every identity.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct BindingPreimage<'a> {
    identifier: &'a str,
    role: KaniBindingRole,
    primitive_type: KaniPrimitiveType,
    integer_bounds: &'a Option<KaniIntegerBounds>,
}

#[derive(Serialize)]
struct Preimage<'a, C: Serialize> {
    domain: &'static str,
    clause: &'a C,
    kind: ObligationKind,
    arguments: Vec<BindingPreimage<'a>>,
}

/// The identity digest of the obligation of `kind` over `clause` and `arguments`.
///
/// # Errors
///
/// [`IdentityRefusal::ArgumentsNotAscending`] when `arguments` are not strictly ascending by
/// identifier.
pub fn obligation_digest<C: Serialize>(
    clause: &C,
    kind: ObligationKind,
    arguments: &[ObligationBinding],
) -> Result<ObligationDigest, IdentityRefusal> {
    if let Some(pair) = arguments
        .windows(2)
        .find(|pair| pair[0].identifier >= pair[1].identifier)
    {
        return Err(IdentityRefusal::ArgumentsNotAscending {
            identifier: pair[1].identifier.clone(),
        });
    }
    let preimage = Preimage {
        domain: PREIMAGE_DOMAIN,
        clause,
        kind,
        arguments: arguments
            .iter()
            .map(|binding| BindingPreimage {
                identifier: &binding.identifier,
                role: binding.role,
                primitive_type: binding.primitive_type,
                integer_bounds: &binding.integer_bounds,
            })
            .collect(),
    };
    // `serde_json::Value` orders object members by name, so the bytes do not depend on how the
    // preimage structs happen to list their fields.
    let value = serde_json::to_value(&preimage).map_err(|_| IdentityRefusal::Preimage)?;
    let bytes = canonical(&value).into_bytes();
    Ok(ObligationDigest(ByteDigest::of(&bytes).as_bytes()))
}

fn canonical(value: &Value) -> String {
    match value {
        Value::Object(members) => {
            let members = members
                .iter()
                .map(|(name, member)| {
                    format!("{}:{}", Value::String(name.clone()), canonical(member))
                })
                .collect::<Vec<_>>();
            format!("{{{}}}", members.join(","))
        }
        Value::Array(items) => {
            let items = items.iter().map(canonical).collect::<Vec<_>>();
            format!("[{}]", items.join(","))
        }
        scalar => scalar.to_string(),
    }
}

impl KaniObligationIdentity {
    /// This obligation's identity digest: over its clause, kind and arguments. The clause's
    /// source span is not in the preimage.
    ///
    /// # Errors
    ///
    /// [`IdentityRefusal`] when the arguments are not ascending by identifier.
    pub fn digest(&self) -> Result<ObligationDigest, IdentityRefusal> {
        obligation_digest(&self.clause, self.kind, &self.arguments)
    }
}
