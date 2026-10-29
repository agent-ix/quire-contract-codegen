// SPDX-License-Identifier: AGPL-3.0-or-later

//! Typed refinement solver evidence and the FR-045 certificate adapter handoff.

use std::{collections::BTreeSet, num::NonZeroU64};

use super::{SolverExecution, SolverStatus};
use crate::kani::run::process::{ProcessOutcome, ProcessRun};

/// A query's position in the recompiled refinement's canonical steps array.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RefinementRole {
    /// FR-315's Initial query.
    Base,
    /// FR-315's Step query at the canonical array position.
    Step { row: usize },
}

/// The invocation selected from authoritative FR-315 query output.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RefinementExpected<Item, Subjects> {
    /// The refinement item.
    pub item: Item,
    /// Concrete and abstract subjects in their selected order.
    pub subjects: Subjects,
    /// Initial or canonically indexed step query.
    pub role: RefinementRole,
    /// The distinct CG invocation identity.
    pub attempt: NonZeroU64,
    /// Exact canonical bytes submitted to the solver.
    pub query_bytes: Vec<u8>,
}

/// One genuine solver invocation, with proof output held separately from its query.
#[derive(Debug)]
pub struct RefinementRun<Item, Subjects> {
    /// The selected refinement and query identity.
    pub item: Item,
    /// Concrete and abstract subjects in their selected order.
    pub subjects: Subjects,
    /// Initial or canonically indexed step query.
    pub role: RefinementRole,
    /// The distinct CG invocation identity.
    pub attempt: NonZeroU64,
    /// Exact canonical bytes sent on stdin; absence is an incomplete record.
    pub query_bytes: Option<Vec<u8>>,
    /// Exact emitted Alethe bytes, independently captured; absence is not an empty proof.
    pub proof_bytes: Option<Vec<u8>>,
    /// Interpreted solver response or typed non-success.
    pub status: SolverStatus,
    /// The selected executable, limits, completion and captured streams.
    pub process: ProcessRun,
}

impl<Item, Subjects> RefinementRun<Item, Subjects> {
    /// Bind a completed bounded solver invocation to the selected subjects and proof capture.
    ///
    /// The caller extracts proof bytes from the solver's documented output channel. CG does
    /// not infer Alethe bytes from a query or rewrite the supplied proof.
    pub fn from_execution(
        subjects: Subjects,
        execution: SolverExecution<(), Item, (), RefinementRole>,
        proof_bytes: Option<Vec<u8>>,
    ) -> Self {
        let super::SolverExecution { query, process } = execution;
        let super::QueryRun {
            association,
            status,
        } = query;
        let super::QueryAssociation { selection, attempt } = association;
        Self {
            item: selection.item,
            subjects,
            role: selection.kind,
            attempt,
            query_bytes: Some(selection.query_bytes),
            proof_bytes,
            status,
            process,
        }
    }
}

/// Why a collection cannot be offered to QSL's refinement certificate adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RefinementHandoffError {
    /// The expected or recorded Base/Step set has a missing, duplicate or out-of-range role.
    Shape,
    /// A record differs from its selected item, subjects, attempt, role or query bytes.
    Association,
    /// A required query or proof field is absent.
    MissingField,
    /// A solver response is incomplete or is not a successful Unsat.
    NonProof(SolverStatus),
    /// A count, row or byte length cannot fit the specified unsigned 64-bit encoding.
    LengthOverflow,
}

/// Validate records and serialize only the fields QSL FR-314's adapter needs.
///
/// Expected entries must come from the recompiled refinement and FR-315 output, indexed by
/// canonical step position. Input record order does not affect the serialized sequence.
pub fn refinement_handoff<Item: Eq, Subjects: Eq>(
    step_count: usize,
    expected: &[RefinementExpected<Item, Subjects>],
    records: &[RefinementRun<Item, Subjects>],
) -> Result<Vec<u8>, RefinementHandoffError> {
    let count = step_count
        .checked_add(1)
        .ok_or(RefinementHandoffError::LengthOverflow)?;
    let count_wire = u64::try_from(count).map_err(|_| RefinementHandoffError::LengthOverflow)?;
    let mut ordered = vec![None; count];
    let mut selected = vec![None; count];
    if expected.len() != count || records.len() != count {
        return Err(RefinementHandoffError::Shape);
    }
    for entry in expected {
        let slot = role_slot(entry.role, step_count)?;
        if selected[slot].replace(entry).is_some() {
            return Err(RefinementHandoffError::Shape);
        }
    }
    let mut attempts = BTreeSet::new();
    if expected.iter().any(|entry| !attempts.insert(entry.attempt)) {
        return Err(RefinementHandoffError::Association);
    }
    for record in records {
        let slot = role_slot(record.role, step_count)?;
        if ordered[slot].replace(record).is_some() {
            return Err(RefinementHandoffError::Shape);
        }
    }
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&count_wire.to_le_bytes());
    for (expected, record) in selected.into_iter().zip(ordered) {
        let (Some(expected), Some(record)) = (expected, record) else {
            return Err(RefinementHandoffError::Shape);
        };
        let Some(query) = record.query_bytes.as_deref() else {
            return Err(RefinementHandoffError::MissingField);
        };
        let Some(proof) = record.proof_bytes.as_deref() else {
            return Err(RefinementHandoffError::MissingField);
        };
        if record.item != expected.item
            || record.subjects != expected.subjects
            || record.role != expected.role
            || record.attempt != expected.attempt
            || query != expected.query_bytes
        {
            return Err(RefinementHandoffError::Association);
        }
        if record.status != SolverStatus::Unsat {
            return Err(RefinementHandoffError::NonProof(record.status));
        }
        if !record.process.started
            || !matches!(
                record.process.outcome,
                ProcessOutcome::Completed { success: true, .. }
            )
        {
            return Err(RefinementHandoffError::NonProof(
                SolverStatus::ProcessFailed,
            ));
        }
        match record.role {
            RefinementRole::Base => bytes.push(0),
            RefinementRole::Step { row } => {
                bytes.push(1);
                let row = u64::try_from(row).map_err(|_| RefinementHandoffError::LengthOverflow)?;
                bytes.extend_from_slice(&row.to_le_bytes());
            }
        }
        for field in [query, proof] {
            let length =
                u64::try_from(field.len()).map_err(|_| RefinementHandoffError::LengthOverflow)?;
            bytes.extend_from_slice(&length.to_le_bytes());
            bytes.extend_from_slice(field);
        }
    }
    Ok(bytes)
}

fn role_slot(role: RefinementRole, step_count: usize) -> Result<usize, RefinementHandoffError> {
    match role {
        RefinementRole::Base => Ok(0),
        RefinementRole::Step { row } if row < step_count => row
            .checked_add(1)
            .ok_or(RefinementHandoffError::LengthOverflow),
        RefinementRole::Step { .. } => Err(RefinementHandoffError::Shape),
    }
}

#[cfg(test)]
mod tests {
    use std::{
        num::{NonZeroU64, NonZeroUsize},
        time::Duration,
    };

    use ix_trace_rs::trace;
    use quire_exact::Cancel;

    use super::{
        refinement_handoff, RefinementExpected, RefinementHandoffError, RefinementRole,
        RefinementRun,
    };
    use crate::{
        kani::run::process::ProcessLimits,
        routed::smt::{run_solver_query, QuerySelection, SolverStatus},
    };

    type Run = RefinementRun<&'static str, [&'static str; 2]>;
    type Expected = RefinementExpected<&'static str, [&'static str; 2]>;

    fn run(role: RefinementRole, proof: &[u8]) -> (Expected, Run) {
        let mut query = b"unsat\n".to_vec();
        query.extend_from_slice(proof);
        let execution = run_solver_query(
            QuerySelection {
                product: (),
                item: "refinement-a",
                instance: (),
                depth: NonZeroU64::MIN,
                kind: role,
                query_bytes: query.clone(),
            },
            "/usr/bin/cat".into(),
            vec![],
            ProcessLimits {
                input_bytes: NonZeroUsize::new(1024).expect("limit"),
                wall_clock: Duration::from_secs(5),
                memory_bytes: NonZeroU64::new(128 * 1024 * 1024).expect("limit"),
                output_bytes: NonZeroUsize::new(1024).expect("limit"),
            },
            Cancel::new(),
        )
        .expect("direct bounded solver invocation");
        assert_eq!(
            execution.process().stdout.as_deref(),
            Some(query.as_slice())
        );
        let attempt = execution.query().association.attempt();
        let emitted = execution
            .process()
            .stdout
            .as_deref()
            .expect("captured stdout")
            .strip_prefix(b"unsat\n")
            .expect("test solver response");
        assert_eq!(emitted, proof);
        let proof_bytes = emitted.to_vec();
        let record = Run::from_execution(["concrete", "abstract"], execution, Some(proof_bytes));
        (
            Expected {
                item: "refinement-a",
                subjects: ["concrete", "abstract"],
                role,
                attempt,
                query_bytes: query,
            },
            record,
        )
    }

    fn collection() -> (Vec<Expected>, Vec<Run>) {
        let mut expected = Vec::new();
        let mut records = Vec::new();
        for role in std::iter::once(RefinementRole::Base)
            .chain((0..7).map(|row| RefinementRole::Step { row }))
        {
            let proof = match role {
                RefinementRole::Base => b"(base)".as_slice(),
                RefinementRole::Step { .. } => b"(step)".as_slice(),
            };
            let (selection, record) = run(role, proof);
            expected.push(selection);
            records.push(record);
        }
        (expected, records)
    }

    fn number(bytes: &[u8], cursor: &mut usize) -> u64 {
        let value = u64::from_le_bytes(bytes[*cursor..*cursor + 8].try_into().expect("number"));
        *cursor += 8;
        value
    }

    fn field<'a>(bytes: &'a [u8], cursor: &mut usize) -> &'a [u8] {
        let length = number(bytes, cursor);
        let end = *cursor + usize::try_from(length).expect("host length");
        let value = &bytes[*cursor..end];
        *cursor = end;
        value
    }

    #[trace("TC-059", "FR-045-AC-4", "FR-045-AC-5")]
    #[test]
    fn canonical_roles_and_exact_fields_are_order_independent() {
        let (expected, mut records) = collection();
        let first = refinement_handoff(7, &expected, &records).expect("valid handoff");
        records.reverse();
        assert_eq!(
            refinement_handoff(7, &expected, &records),
            Ok(first.clone())
        );
        assert_eq!(
            refinement_handoff(7, &expected, &records),
            Ok(first.clone())
        );
        let mut cursor = 0;
        assert_eq!(number(&first, &mut cursor), 8);
        for (index, selection) in expected.iter().enumerate() {
            assert_eq!(first[cursor], u8::from(index != 0));
            cursor += 1;
            if index != 0 {
                assert_eq!(number(&first, &mut cursor), (index - 1) as u64);
            }
            assert_eq!(field(&first, &mut cursor), selection.query_bytes.as_slice());
            let proof = if index == 0 { b"(base)" } else { b"(step)" };
            assert_eq!(field(&first, &mut cursor), proof);
        }
        assert_eq!(cursor, first.len());
        records[4].proof_bytes = Some(Vec::new());
        let empty = refinement_handoff(7, &expected, &records).expect("empty proof is present");
        assert_ne!(empty, first);
        records[4].proof_bytes = Some(b"(stEp)".to_vec());
        let changed = refinement_handoff(7, &expected, &records).expect("changed proof");
        assert_eq!(changed.len(), first.len());
        assert_eq!(
            first
                .iter()
                .zip(&changed)
                .filter(|(left, right)| left != right)
                .count(),
            1
        );
        let (zero_expected, zero_records) = {
            let (entry, record) = run(RefinementRole::Base, b"(base)");
            (vec![entry], vec![record])
        };
        assert!(refinement_handoff(0, &zero_expected, &zero_records).is_ok());
    }

    #[trace("TC-059", "FR-045-AC-4", "FR-045-AC-6", "FR-045-AC-7")]
    #[test]
    fn shape_association_and_missing_fields_refuse_handoff() {
        let (expected, mut records) = collection();
        let base = records.remove(0);
        assert_eq!(
            refinement_handoff(7, &expected, &records),
            Err(RefinementHandoffError::Shape)
        );
        records.insert(0, base);
        let displaced = records.remove(4);
        records.push(run(RefinementRole::Base, b"(base)").1);
        assert_eq!(
            refinement_handoff(7, &expected, &records),
            Err(RefinementHandoffError::Shape)
        );
        records.pop();
        records.insert(4, displaced);
        let row = records.remove(4);
        assert_eq!(
            refinement_handoff(7, &expected, &records),
            Err(RefinementHandoffError::Shape)
        );
        records.insert(4, row);
        records[4].role = RefinementRole::Step { row: 7 };
        assert_eq!(
            refinement_handoff(7, &expected, &records),
            Err(RefinementHandoffError::Shape)
        );
        records[4].role = RefinementRole::Step { row: 3 };
        records[4].query_bytes = None;
        assert_eq!(
            refinement_handoff(7, &expected, &records),
            Err(RefinementHandoffError::MissingField)
        );
        records[4].query_bytes = Some(expected[4].query_bytes.clone());
        records[4].proof_bytes = None;
        assert_eq!(
            refinement_handoff(7, &expected, &records),
            Err(RefinementHandoffError::MissingField)
        );
        records[4].proof_bytes = Some(b"(step)".to_vec());
        records[4].item = "other";
        assert_eq!(
            refinement_handoff(7, &expected, &records),
            Err(RefinementHandoffError::Association)
        );
        records[4].item = "refinement-a";
        records[4].query_bytes = Some(b"changed".to_vec());
        assert_eq!(
            refinement_handoff(7, &expected, &records),
            Err(RefinementHandoffError::Association)
        );
        records[4].query_bytes = Some(expected[4].query_bytes.clone());
        records[4].status = SolverStatus::Unknown;
        assert_eq!(
            refinement_handoff(7, &expected, &records),
            Err(RefinementHandoffError::NonProof(SolverStatus::Unknown))
        );
        records[4].status = SolverStatus::Unsat;
        records[4].subjects = ["other", "abstract"];
        assert_eq!(
            refinement_handoff(7, &expected, &records),
            Err(RefinementHandoffError::Association)
        );
        records[4].subjects = ["concrete", "abstract"];
        records[4].attempt = records[5].attempt;
        assert_eq!(
            refinement_handoff(7, &expected, &records),
            Err(RefinementHandoffError::Association)
        );
        records[4].attempt = expected[4].attempt;
        records.swap(4, 5);
        records[4].role = RefinementRole::Step { row: 3 };
        records[5].role = RefinementRole::Step { row: 4 };
        assert_eq!(
            refinement_handoff(7, &expected, &records),
            Err(RefinementHandoffError::Association)
        );
        records.swap(4, 5);
        records[4].role = RefinementRole::Step { row: 3 };
        records[5].role = RefinementRole::Step { row: 4 };
        records[4].role = RefinementRole::Base;
        assert_eq!(
            refinement_handoff(7, &expected, &records),
            Err(RefinementHandoffError::Shape)
        );
        records[4].role = RefinementRole::Step { row: 3 };
        records.remove(5);
        records.push(run(RefinementRole::Step { row: 3 }, b"(step)").1);
        assert_eq!(
            refinement_handoff(7, &expected, &records),
            Err(RefinementHandoffError::Shape)
        );
    }
}
