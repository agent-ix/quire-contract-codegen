//! Exact same-helper role commands and owned bootstrap mappings for FR-034.
//!
//! A command recipe does not establish a spawned role or its arm. The separate retained owners
//! record the actual Child before any fallible pin/handshake operation. No executable PATH search,
//! raw descriptor adoption, backend environment override or pre-exec callback is used here.

use std::{
    ffi::OsStr,
    io,
    path::Path,
    process::{Command, Stdio},
};

use super::control::RoleEndpoint;

const LAUNCHER_ARGUMENT: &str = "--quire-kani-launcher";
const OUTER_ARGUMENT: &str = "--quire-kani-outer";

/// Private roles of the one explicitly configured, artifact-matched package helper.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum HelperRole {
    Launcher,
    Outer,
    Inner,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum RoleSelectionError {
    UnknownRole,
    ExcessArguments,
}

impl std::fmt::Display for RoleSelectionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "helper role selection refused: {self:?}")
    }
}

impl std::error::Error for RoleSelectionError {}

impl HelperRole {
    fn argument(self) -> Option<&'static str> {
        match self {
            Self::Launcher => Some(LAUNCHER_ARGUMENT),
            Self::Outer => Some(OUTER_ARGUMENT),
            // The actual original guardian invocation is the inner role with no arguments.
            Self::Inner => None,
        }
    }

    /// Parse only the role selector, excluding argv0. No unknown argument authorizes a role.
    pub(super) fn select<I, A>(mut arguments: I) -> Result<Self, RoleSelectionError>
    where
        I: Iterator<Item = A>,
        A: AsRef<OsStr>,
    {
        let role = match arguments.next() {
            None => Self::Inner,
            Some(argument) if argument.as_ref() == OsStr::new(LAUNCHER_ARGUMENT) => Self::Launcher,
            Some(argument) if argument.as_ref() == OsStr::new(OUTER_ARGUMENT) => Self::Outer,
            Some(_) => return Err(RoleSelectionError::UnknownRole),
        };
        if arguments.next().is_some() {
            return Err(RoleSelectionError::ExcessArguments);
        }
        Ok(role)
    }

    /// Child-only stdin mapping transfers ownership, not an arbitrary inherited fd number.
    /// L's dedicated spawner sets process_group(0); O establishes its own session after arm.
    pub(super) fn command(self, helper: &Path, bootstrap: RoleEndpoint) -> io::Result<Command> {
        if !helper.is_absolute() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "matched helper path must be absolute",
            ));
        }
        let mut command = Command::new(helper);
        if let Some(argument) = self.argument() {
            command.arg(argument);
        }
        command
            // No trusted startup role activates arbitrary original-backend loader inputs.
            // C's exact original environment is retained separately in authenticated bounded
            // BackendCommand metadata, restored only by the installed backend exec boundary.
            .env_clear()
            .stdin(Stdio::from(bootstrap.into_child_mapping()))
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());
        Ok(command)
    }
}
