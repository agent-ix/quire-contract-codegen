// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX
//! Owned settings construction after original creator/descriptor authentication.
//!
//! UTF-8 helper text remains borrowed until this consuming boundary. Its decoded byte length
//! is not an allocation cap or native-stack proof; actual role storage/RSS remains accounted.

use std::path::PathBuf;

use super::{
    recipe_decode::MaterializationError, role_protocol::RunSettings, settings_decode::SettingsBytes,
};

/// Preserve the original serde PathBuf string semantics without an intermediate owned copy.
/// Caller must authenticate the whole source frame and check its original clock/owned pins.
pub(super) fn materialize(
    settings: SettingsBytes<'_>,
) -> Result<RunSettings, MaterializationError> {
    let bytes = settings.helper.decoded_bytes()?;
    let mut helper = String::new();
    helper
        .try_reserve_exact(bytes)
        .map_err(MaterializationError::Allocation)?;
    settings.helper.append_to(&mut helper, bytes)?;
    Ok(RunSettings {
        helper: PathBuf::from(helper),
        identity: settings.identity,
        authority: settings.authority,
        deadline: settings.deadline,
        started: settings.started,
        settlement_reserve: settings.settlement_reserve,
        work_deadline: settings.work_deadline,
        setup_deadline: settings.setup_deadline,
        caller_uid: settings.caller_uid,
        caller_gid: settings.caller_gid,
        memory_bytes: settings.memory_bytes,
        caller_run_buffers: settings.caller_run_buffers,
    })
}

/// Actual fixed output-layout terms beyond the borrowed settings schema; heap capacities and
/// native initialization/return-place/copy highwater require independent supported-build proof.
pub(super) fn decode_bytes() -> Result<u64, super::guardian_decode::DecodeError> {
    use super::guardian_decode::{DecodeCause, DecodeError, DecodeSite};
    use std::mem::{size_of, size_of_val};
    let terms = [
        size_of::<String>(),
        size_of::<PathBuf>(),
        size_of::<RunSettings>(),
        size_of::<Result<RunSettings, MaterializationError>>(),
        size_of::<MaterializationError>(),
        size_of::<usize>(),
    ];
    let storage = || DecodeError::new(DecodeSite::Storage, DecodeCause::StorageBound);
    let bytes = u64::try_from(size_of_val(&terms)).map_err(|_| storage())?;
    terms.into_iter().try_fold(bytes, |bytes, term| {
        bytes
            .checked_add(u64::try_from(term).map_err(|_| storage())?)
            .ok_or_else(storage)
    })
}
