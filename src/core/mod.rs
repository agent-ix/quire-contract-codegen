//! The leaf of the crate: the shared primitives every other directory builds on (AD-004).
//!
//! `core` imports nothing else in this crate. The generated file and the validated bundle, the
//! generation diagnostic vocabulary, source-map records, the identity newtypes of a generated
//! harness and the version profile live here.

// The generated file and the validated bundle every generator builds.
// Implements: FR-005
pub(crate) mod artifact;
// The generation diagnostic vocabulary, below every generator.
pub(crate) mod diagnostic;
// Identity newtypes of a generated harness.
pub(crate) mod identity;
// The version profile: the emitted oracle crate manifest, written once.
pub(crate) mod profile;
// Source-map records tracing generated source to a clause.
pub(crate) mod source_map;
