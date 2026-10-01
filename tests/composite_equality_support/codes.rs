//! The composite/structural corpus's fixture codes and readable node keys.
//!
//! Dependency-free on purpose: `package.rs` re-exports all of it, and the
//! generated-crate agreement cases (`agreement_cases.rs`), which run in a
//! scratch crate that depends only on the Contract Runtime, load this file as
//! their `package` module so `agreement.rs` resolves the same keys there.

#![allow(dead_code)] // Each consumer uses a different subset.

/// A readable node key: the code, zero-padded to a 64-digit digest.
pub fn key(code: u32) -> String {
    format!("{code:0>64}")
}

pub const T_BOOLEAN: u32 = 1;
pub const T_INTEGER: u32 = 2;
pub const T_TEXT: u32 = 3;
pub const T_FLOAT64: u32 = 4;
pub const BD_TEXT: u32 = 6;
pub const T_INTEGER_BOUNDED: u32 = 7;
pub const BD_INTEGER: u32 = 8;
pub const T_DECIMAL_SMALL: u32 = 9;
pub const BD_DECIMAL: u32 = 10;
/// `Rational[-10, 10; 1, 1]`: a `convert<T>` source admitted into
/// `T_RATIONAL_WIDE` (`admits_equality_conversion`'s `Rational -> Rational`
/// row).
pub const T_RATIONAL_NARROW: u32 = 12;
pub const BD_RATIONAL_NARROW: u32 = 13;
/// `Rational[-100, 100; 1, 5]`: wide enough to admit `T_RATIONAL_NARROW`
/// (`Rational -> Rational`) and `T_DECIMAL_SMALL` (`Decimal -> Rational`).
pub const T_RATIONAL_WIDE: u32 = 14;
pub const BD_RATIONAL_WIDE: u32 = 15;
/// `Rational[-50, 50; 1, 1]`: denominator pinned to `1`, so
/// `admits_equality_conversion`'s `Rational -> Integer/Int/Decimal` row
/// admits `T_INTEGER`.
pub const T_RATIONAL_INT: u32 = 16;
pub const BD_RATIONAL_INT: u32 = 17;
/// `Decimal[-1000, 1000; 0, 2]`: wide enough to admit `T_DECIMAL_SMALL`
/// (`Decimal -> Decimal`).
pub const T_DECIMAL_WIDE: u32 = 18;
pub const BD_DECIMAL_WIDE: u32 = 19;

pub const R_POINT: u32 = 20;
pub const R_FLOAT: u32 = 21;
pub const CB_SMALL: u32 = 22;
pub const SEQ_R_FLOAT: u32 = 23;
pub const TUP_PAIR: u32 = 24;
pub const OPT_INT: u32 = 25;
pub const R_DUP: u32 = 27;
pub const REF_TYPE: u32 = 28;
pub const R_SELF: u32 = 29;
pub const OPT_SELF: u32 = 30;
pub const R_PAIR_OF_POINTS: u32 = 31;
pub const SEQ_INT: u32 = 32;
/// A record whose one field is a `REF_TYPE` reference: an equality operand reaching a
/// `reference` composite (FR-018-AC-7). `quire.op.reference.eq` needs a `Reference<X>` whose `X` is
/// a model object type of a selected document, which this corpus has none of, so the reference
/// is reached through a record and compared by `quire.op.structural.eq`.
pub const R_WITH_REF: u32 = 34;
/// Not registered in `corpus_package()`: used only as a `key()`/`code_id()`
/// input to build a standalone `TypeEnvironment` for FR-018-AC-6's negative
/// control, exactly as `R_FLOAT` is reused for its positive one.
pub const R_NOT_FLOAT: u32 = 33;

pub const M_BARE: u32 = 40;
pub const F_BARE: u32 = 41;
pub const S_BARE: u32 = 42;
pub const T_BARE: u32 = 43;

pub const E_RECORD: u32 = 100;
pub const E_NESTED_IEEE: u32 = 101;
pub const E_TUPLE: u32 = 102;
pub const E_OPTION: u32 = 103;
pub const E_TEXT: u32 = 104;
pub const E_ENUM: u32 = 105;
pub const E_DUP: u32 = 106;
pub const E_BAD_CONVERT: u32 = 107;
pub const E_REFERENCE: u32 = 108;
pub const E_CALL: u32 = 109;
pub const E_SELF: u32 = 110;
pub const E_CONV: u32 = 111;
pub const E_COLLECTION: u32 = 112;
pub const E_PAIR_OF_POINTS: u32 = 113;
pub const E_CONV_CHARGE: u32 = 114;
/// FR-018 Behavior's "disagrees with its descriptor's ... operand types"
/// (codegen#82): body operands both reference `T_INTEGER`; every request in
/// this module's tests over this node uses a different descriptor type, so
/// `check_operand_types` refuses it before generation.
pub const E_OPERAND_MISMATCH: u32 = 115;
/// `admits_equality_conversion`'s `Rational -> Rational` row (codegen#83).
pub const E_CONV_RAT_RAT: u32 = 116;
/// `admits_equality_conversion`'s `Rational -> Integer/Int/Decimal` row,
/// exercised against `Integer` (codegen#83).
pub const E_CONV_RAT_INT: u32 = 117;
/// `admits_equality_conversion`'s `Decimal -> Rational` row (codegen#83).
pub const E_CONV_DEC_RAT: u32 = 118;
/// `admits_equality_conversion`'s `Decimal -> Decimal` row (codegen#83).
pub const E_CONV_DEC_DEC: u32 = 119;
/// `admits_equality_conversion`'s `Decimal -> Integer/Int` row, exercised
/// against `Integer` (codegen#83).
pub const E_CONV_DEC_INT: u32 = 120;
/// An equality directly over `REF_TYPE` operands: refused by Contract IR at admission, so it is
/// only in [`direct_reference_package`](super::direct_reference_package), never in the corpus.
pub const E_REFERENCE_DIRECT: u32 = 121;
/// An equality whose left operand is a conversion of a conversion, only in
/// [`nested_conversion_package`](super::nested_conversion_package).
pub const E_NESTED_CONV: u32 = 122;
/// An equality whose left operand is a `rational.div` application node, only in
/// [`application_operand_package`](super::application_operand_package).
pub const E_APPLICATION_OPERAND: u32 = 123;
