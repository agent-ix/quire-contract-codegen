//! The one source of the state-frame fixture's model facts: the object's integer fields with the
//! range each declares, and the fields its operation's frame grants. The hand-built checked
//! package and the QSL twin's domain package are both built from it.

/// Each field of the operation's state and its inclusive range.
pub const FIELDS: [(&str, (i64, i64)); 2] = [("balance", (0, 1000)), ("audit", (0, 1000))];

/// The fields the operation's frame modifies.
pub const GRANTED: [&str; 1] = ["balance"];
