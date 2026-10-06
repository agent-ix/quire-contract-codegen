//! Prepared same-PID execution of the retained backend recipe.
//!
//! The trusted single-thread backend entry owns authentication, actual policy installation and
//! positive Dispatch. It calls this boundary only after those checks, keeps its authenticated
//! bootstrap fd0 open and stable until exec, and never retries an exec failure without
//! settling the run. This module installs no policy and creates no child or control transport.

#![cfg(target_os = "linux")]

use command_fds::{CommandFdExt, FdMapping};
use std::{
    fs::File,
    io,
    os::unix::process::CommandExt,
    process::{Command, Stdio},
};

use super::{namespace::BackendCommand, report_storage::REPORT_SLOT, stdin::OriginalStdin};

/// Owns the exact command and its retained stdin/report descriptors until exec or failure.
/// Descriptor mappings belong to the command's safe dependency hook, not to a new process.
pub(super) struct PreparedBackendExec {
    command: Command,
}

/// Prepare the admitted recipe without changing its program, argv, environment or directory.
///
/// The caller must retain the authenticated bootstrap at fd0 through preparation and exec.
/// `Closed` describes original backend stdin, not the currently open bootstrap descriptor.
/// Captured stdout/stderr remain inherited; only the owned report writer gains the report slot.
pub(super) fn prepare(
    recipe: BackendCommand,
    stdin: OriginalStdin,
    report_writer: File,
) -> io::Result<PreparedBackendExec> {
    if REPORT_SLOT < 5 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "backend report slot overlaps standard or control descriptors",
        ));
    }
    let mut command = recipe.into_command();
    match stdin {
        OriginalStdin::Open(descriptor) => {
            command.stdin(Stdio::from(descriptor));
        }
        OriginalStdin::Closed => {
            let bootstrap = std::io::stdin();
            let flags = rustix::io::fcntl_getfd(&bootstrap)?;
            rustix::io::fcntl_setfd(&bootstrap, flags | rustix::io::FdFlags::CLOEXEC)?;
            if !rustix::io::fcntl_getfd(&bootstrap)?.contains(rustix::io::FdFlags::CLOEXEC) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "backend bootstrap stdin remains visible at exec",
                ));
            }
            // Inherited fd0 closes at exec because the actual bootstrap is CLOEXEC. A null
            // input or a freshly opened descriptor would change original closed-stdin semantics.
            command.stdin(Stdio::inherit());
        }
    }
    command.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    command
        .fd_mappings(vec![FdMapping {
            parent_fd: report_writer.into(),
            child_fd: REPORT_SLOT,
        }])
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;
    Ok(PreparedBackendExec { command })
}

impl PreparedBackendExec {
    /// Replace this already authenticated, policy-restricted backend process with its recipe.
    /// Success never returns; failure preserves the exec error for the caller's settlement.
    pub(super) fn exec(mut self) -> io::Error {
        self.command.exec()
    }
}
