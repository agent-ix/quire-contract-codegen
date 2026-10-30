// SPDX-License-Identifier: AGPL-3.0-or-later

//! Typed refinement solver evidence and the FR-045 certificate adapter handoff.

use std::{
    collections::BTreeSet,
    ffi::{OsStr, OsString},
    num::NonZeroU64,
};

use super::{captured_alethe_proof, SolverExecution, SolverStatus};
use crate::kani::run::process::{ProcessOutcome, ProcessRun};

/// A query's position in the recompiled refinement's declaration-order steps array.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RefinementRole {
    /// FR-315's Initial query.
    Base,
    /// FR-315's Step query at the declaration-order array position.
    Step {
        /// Zero-based position in the recompiled declaration-order steps array.
        row: usize,
    },
}

/// The invocation selected from authoritative FR-315 query output.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RefinementExpected<Item, Subjects> {
    /// The refinement item.
    pub item: Item,
    /// Concrete and abstract subjects in their selected order.
    pub subjects: Subjects,
    /// Initial or declaration-order indexed step query.
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
    item: Item,
    /// Concrete and abstract subjects in their selected order.
    subjects: Subjects,
    /// Initial or declaration-order indexed step query.
    role: RefinementRole,
    /// The distinct CG invocation identity.
    attempt: NonZeroU64,
    /// Exact canonical bytes sent on stdin; absence is an incomplete record.
    query_bytes: Option<Vec<u8>>,
    /// Alethe bytes extracted from this invocation's completed stdout.
    proof_bytes: Option<Vec<u8>>,
    /// Interpreted solver response or typed non-success.
    status: SolverStatus,
    /// The selected executable, limits, completion and captured streams.
    process: ProcessRun,
}

impl<Item, Subjects> RefinementRun<Item, Subjects> {
    /// Bind a bounded solver invocation to its selected subjects.
    ///
    /// Only a completed `Unsat` from the selected cvc5 proof-dump mode may carry a proof.
    /// Malformed proof framing changes the refinement record's status to `MalformedResponse`.
    pub fn from_execution(
        subjects: Subjects,
        execution: SolverExecution<(), Item, (), RefinementRole>,
    ) -> Self {
        let super::SolverExecution { query, process } = execution;
        let super::QueryRun {
            association,
            mut status,
        } = query;
        let super::QueryAssociation { selection, attempt } = association;
        let proof_bytes = if status == SolverStatus::Unsat
            && process.started
            && matches!(
                process.outcome,
                ProcessOutcome::Completed { success: true, .. }
            )
            && cvc5_proof_dump_selected(&process.arguments)
        {
            match process.stdout.as_deref().map(captured_alethe_proof) {
                Some(Ok(Some(proof))) => Some(proof.to_vec()),
                Some(Err(error)) => {
                    status = error;
                    None
                }
                Some(Ok(None)) | None => None,
            }
        } else {
            None
        };
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

    /// Return the selected refinement item.
    #[must_use]
    pub fn item(&self) -> &Item {
        &self.item
    }

    /// Return the selected concrete and abstract subjects.
    #[must_use]
    pub fn subjects(&self) -> &Subjects {
        &self.subjects
    }

    /// Return the query's typed refinement role and declaration-order row.
    #[must_use]
    pub fn role(&self) -> RefinementRole {
        self.role
    }

    /// Return the distinct identity assigned to this solver invocation.
    #[must_use]
    pub fn attempt(&self) -> NonZeroU64 {
        self.attempt
    }

    /// Return the query bytes submitted on this invocation's stdin.
    #[must_use]
    pub fn query_bytes(&self) -> Option<&[u8]> {
        self.query_bytes.as_deref()
    }

    /// Return the proof bytes extracted from this invocation's captured stdout.
    #[must_use]
    pub fn proof_bytes(&self) -> Option<&[u8]> {
        self.proof_bytes.as_deref()
    }

    /// Return the interpreted solver response or typed non-success.
    #[must_use]
    pub fn status(&self) -> SolverStatus {
        self.status
    }

    /// Return the bounded process result that supplied this record.
    #[must_use]
    pub fn process(&self) -> &ProcessRun {
        &self.process
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
    /// CG cannot authenticate candidate proof bytes against the solver's captured output.
    ProofProvenanceUnavailable,
}

/// Validate run records and serialize the proof-bearing QSL adapter handoff.
///
/// Expected entries must come from the recompiled refinement and FR-315 output, indexed by
/// declaration-order step position. Proof bytes must match the same completed invocation's stdout.
pub fn refinement_handoff<Item: Eq, Subjects: Eq>(
    step_count: usize,
    expected: &[RefinementExpected<Item, Subjects>],
    records: &[RefinementRun<Item, Subjects>],
) -> Result<Vec<u8>, RefinementHandoffError> {
    let ordered = validate_records(step_count, expected, records)?;
    serialize_verified_fields(&ordered)
}

fn validate_records<'a, Item: Eq, Subjects: Eq>(
    step_count: usize,
    expected: &'a [RefinementExpected<Item, Subjects>],
    records: &'a [RefinementRun<Item, Subjects>],
) -> Result<Vec<&'a RefinementRun<Item, Subjects>>, RefinementHandoffError> {
    let count = step_count
        .checked_add(1)
        .ok_or(RefinementHandoffError::LengthOverflow)?;
    if expected.len() != count || records.len() != count {
        return Err(RefinementHandoffError::Shape);
    }
    u64::try_from(count).map_err(|_| RefinementHandoffError::LengthOverflow)?;
    let mut ordered = vec![None; count];
    let mut selected = vec![None; count];
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
    let mut validated = Vec::with_capacity(count);
    for (expected, record) in selected.into_iter().zip(ordered) {
        let (Some(expected), Some(record)) = (expected, record) else {
            return Err(RefinementHandoffError::Shape);
        };
        let Some(query) = record.query_bytes.as_deref() else {
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
        let Some(proof) = record.proof_bytes.as_deref() else {
            return Err(RefinementHandoffError::MissingField);
        };
        if record
            .process
            .stdout
            .as_deref()
            .and_then(|stdout| captured_alethe_proof(stdout).ok().flatten())
            != Some(proof)
            || !cvc5_proof_dump_selected(&record.process.arguments)
        {
            return Err(RefinementHandoffError::ProofProvenanceUnavailable);
        }
        for field in [query, proof] {
            u64::try_from(field.len()).map_err(|_| RefinementHandoffError::LengthOverflow)?;
        }
        validated.push(record);
    }
    Ok(validated)
}

fn cvc5_proof_dump_selected(arguments: &[OsString]) -> bool {
    arguments
        .iter()
        .any(|argument| argument.as_os_str() == OsStr::new("--dump-proofs"))
        && arguments
            .iter()
            .any(|argument| argument.as_os_str() == OsStr::new("--proof-format-mode=alethe"))
}

fn serialize_verified_fields<Item, Subjects>(
    records: &[&RefinementRun<Item, Subjects>],
) -> Result<Vec<u8>, RefinementHandoffError> {
    let count = u64::try_from(records.len()).map_err(|_| RefinementHandoffError::LengthOverflow)?;
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&count.to_le_bytes());
    for record in records {
        match record.role {
            RefinementRole::Base => bytes.push(0),
            RefinementRole::Step { row } => {
                bytes.push(1);
                let row = u64::try_from(row).map_err(|_| RefinementHandoffError::LengthOverflow)?;
                bytes.extend_from_slice(&row.to_le_bytes());
            }
        }
        for field in [
            record.query_bytes.as_deref().expect("validated query"),
            record.proof_bytes.as_deref().expect("validated proof"),
        ] {
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
#[path = "refinement_source_tests.rs"]
mod source_tests;

#[cfg(test)]
mod tests {
    use std::{
        ffi::OsString,
        num::{NonZeroU64, NonZeroUsize},
        time::Duration,
    };

    use ix_trace_rs::trace;
    use qsl_replay::{ProofFormat, RefinementSmtCertificate, RefutedQuery};
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

    fn run_with_proof(role: RefinementRole, proof: Option<&[u8]>) -> (Expected, Run) {
        let query = format!("canonical query for {role:?}\n").into_bytes();
        let proof_text = proof
            .map(|bytes| std::str::from_utf8(bytes).expect("ASCII test proof"))
            .unwrap_or_default();
        let wrapper = if proof.is_some() {
            format!("({proof_text})")
        } else {
            String::new()
        };
        let script = format!("cat >&2; printf 'unsat\\n{wrapper}'");
        let execution = run_solver_query(
            QuerySelection {
                product: (),
                item: "refinement-a",
                instance: (),
                depth: NonZeroU64::MIN,
                kind: role,
                query_bytes: query.clone(),
            },
            "/bin/sh".into(),
            vec![
                OsString::from("-c"),
                OsString::from(script),
                OsString::from("--dump-proofs"),
                OsString::from("--proof-format-mode=alethe"),
            ],
            ProcessLimits {
                input_bytes: NonZeroUsize::new(1024).expect("limit"),
                wall_clock: Duration::from_secs(5),
                memory_bytes: NonZeroU64::new(128 * 1024 * 1024).expect("limit"),
                output_bytes: NonZeroUsize::new(1024).expect("limit"),
            },
            Cancel::new(),
        )
        .expect("direct bounded solver invocation");
        let mut response = b"unsat\n".to_vec();
        response.extend_from_slice(wrapper.as_bytes());
        assert_eq!(
            execution.process().stdout.as_deref(),
            Some(response.as_slice())
        );
        assert_eq!(
            execution.process().stderr.as_deref(),
            Some(query.as_slice())
        );
        let attempt = execution.query().association.attempt();
        let record = Run::from_execution(["concrete", "abstract"], execution);
        assert_eq!(record.proof_bytes(), proof);
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

    fn run(role: RefinementRole, proof: &[u8]) -> (Expected, Run) {
        run_with_proof(role, Some(proof))
    }

    fn collection_with_step_proof(step_proof: &[u8]) -> (Vec<Expected>, Vec<Run>) {
        let mut expected = Vec::new();
        let mut records = Vec::new();
        for role in std::iter::once(RefinementRole::Base)
            .chain((0..7).map(|row| RefinementRole::Step { row }))
        {
            let proof = match role {
                RefinementRole::Base => b"(base)".as_slice(),
                RefinementRole::Step { row: 3 } => step_proof,
                RefinementRole::Step { .. } => b"(step)".as_slice(),
            };
            let (selection, record) = run(role, proof);
            expected.push(selection);
            records.push(record);
        }
        (expected, records)
    }

    fn collection() -> (Vec<Expected>, Vec<Run>) {
        collection_with_step_proof(b"(step)")
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

    fn format_fields(expected: &[Expected], records: &[Run]) -> Vec<u8> {
        refinement_handoff(expected.len() - 1, expected, records).expect("verified handoff")
    }

    #[trace("TC-059", "FR-045-AC-4")]
    #[test]
    fn canonical_roles_and_exact_fields_are_order_independent() {
        let (expected, mut records) = collection();
        assert!(refinement_handoff(7, &expected, &records).is_ok());
        let first = format_fields(&expected, &records);
        records.reverse();
        assert_eq!(format_fields(&expected, &records), first);
        assert_eq!(format_fields(&expected, &records), first);
        let mut cursor = 0;
        assert_eq!(number(&first, &mut cursor), 8);
        let mut parts = Vec::new();
        for (index, selection) in expected.iter().enumerate() {
            assert_eq!(first[cursor], u8::from(index != 0));
            cursor += 1;
            if index != 0 {
                assert_eq!(number(&first, &mut cursor), (index - 1) as u64);
            }
            let query = field(&first, &mut cursor).to_vec();
            assert_eq!(query, selection.query_bytes);
            let proof = if index == 0 { b"(base)" } else { b"(step)" };
            let emitted = field(&first, &mut cursor).to_vec();
            assert_eq!(emitted, proof);
            parts.push(RefutedQuery {
                query,
                proof: emitted,
            });
        }
        assert_eq!(cursor, first.len());
        let certificate = RefinementSmtCertificate {
            format: ProofFormat::Alethe,
            base: parts.remove(0),
            steps: parts,
        };
        assert_eq!(certificate.base.query, expected[0].query_bytes);
        assert_eq!(certificate.steps.len(), 7);
        let (different_expected, different_records) = collection_with_step_proof(b"(stEp)");
        let changed = format_fields(&different_expected, &different_records);
        assert_eq!(changed.len(), first.len());
        assert_eq!(
            first
                .iter()
                .zip(&changed)
                .filter(|(left, right)| left != right)
                .count(),
            1
        );
        records[4].proof_bytes = Some(Vec::new());
        assert_eq!(
            refinement_handoff(7, &expected, &records),
            Err(RefinementHandoffError::ProofProvenanceUnavailable)
        );
        records[4].proof_bytes = Some(b"(stEp)".to_vec());
        assert_eq!(
            refinement_handoff(7, &expected, &records),
            Err(RefinementHandoffError::ProofProvenanceUnavailable)
        );
        let (zero_expected, zero_records) = {
            let (entry, record) = run(RefinementRole::Base, b"(base)");
            (vec![entry], vec![record])
        };
        assert!(refinement_handoff(0, &zero_expected, &zero_records).is_ok());
        let mut cursor = 0;
        assert_eq!(
            number(&format_fields(&zero_expected, &zero_records), &mut cursor),
            1
        );
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

    #[trace("TC-059", "FR-045-AC-7")]
    #[test]
    fn caller_supplied_proof_cannot_be_handed_off_as_emitted_proof() {
        let (expected, mut records) = collection();
        records[4].proof_bytes = Some(b"(fake)".to_vec());
        assert_eq!(
            refinement_handoff(7, &expected, &records),
            Err(RefinementHandoffError::ProofProvenanceUnavailable)
        );
        assert_eq!(
            refinement_handoff(usize::MAX - 1, &expected, &records),
            Err(RefinementHandoffError::Shape)
        );
    }

    #[trace("TC-059", "FR-045-AC-7")]
    #[test]
    fn completed_unsat_without_emitted_proof_has_missing_field() {
        let (expected, record) = run_with_proof(RefinementRole::Base, None);
        assert_eq!(record.status(), SolverStatus::Unsat);
        assert_eq!(record.proof_bytes(), None);
        assert_eq!(
            refinement_handoff(0, &[expected], &[record]),
            Err(RefinementHandoffError::MissingField)
        );
        let (expected, record) = run(RefinementRole::Base, b"");
        assert_eq!(record.proof_bytes(), Some(b"".as_slice()));
        assert!(refinement_handoff(0, &[expected], &[record]).is_ok());
    }
}
