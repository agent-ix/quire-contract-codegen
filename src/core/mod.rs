//! The leaf of the crate: the shared primitives every other directory builds on (AD-004).
//!
//! `core` imports nothing else in this crate. The generated file and the validated bundle, the
//! generation diagnostic vocabulary, source-map records, the identity newtypes of a generated
//! harness, the shared naming helpers and the version profile live here.

// The generated file and the validated bundle every generator builds.
// Implements: FR-005
pub(crate) mod artifact;
// The one caller of quire-canonical's RFC 8785 encoder and digest.
// Implements: FR-016-AC-21
pub(crate) mod canonical;
// Typed access to one admitted FR-330 package.
// Implements: FR-039
pub(crate) mod contract_package;
// The generation diagnostic vocabulary, below every generator.
pub(crate) mod diagnostic;
// Identity newtypes of a generated harness.
pub(crate) mod identity;
// The one typed access layer for admitted V2 clause conditions.
pub(crate) mod ir;
// Bounded readable name components, unique names and symbol derivation.
// Implements: FR-022
pub(crate) mod naming;
// Selected native integer capacity and interval decisions.
pub(crate) mod native_integer;
// The version profile: the emitted oracle crate manifest, written once.
pub(crate) mod profile;
// Header identity for each generated Rust artifact.
pub(crate) mod provenance;
// Source-map records tracing generated source to a clause.
pub(crate) mod source_map;

// Target-owned scratch allocation shared by subsystem unit tests.
#[cfg(test)]
pub(crate) mod test_support;
