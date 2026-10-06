//! The Kani backend this crate runs and where it is located (FR-017).

use std::{env, ffi::OsString, fmt, fs, io, path::PathBuf};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use serde::Serialize;

/// A backend component this module locates or reads.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KaniTool {
    /// The `cargo-kani` launcher.
    Launcher,
    /// The generated crate's `src/lib.rs`, checked for the harness's generated source.
    Library,
}

/// Why the installed backend could not be located or started.
#[derive(Debug)]
pub enum KaniToolError {
    /// The component is absent at the path it was looked for.
    Missing {
        /// Component.
        tool: KaniTool,
        /// Path looked at.
        path: PathBuf,
    },
    /// The component exists but could not be read or started.
    Io {
        /// Component.
        tool: KaniTool,
        /// Path read or started.
        path: PathBuf,
        /// Underlying error.
        error: io::Error,
    },
}

impl fmt::Display for KaniToolError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing { tool, path } => {
                write!(formatter, "{tool:?} is absent at {}", path.display())
            }
            Self::Io { tool, path, error } => {
                write!(formatter, "{tool:?} at {}: {error}", path.display())
            }
        }
    }
}

impl std::error::Error for KaniToolError {}

/// Where the Kani launcher lives.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KaniInstallation {
    /// The `cargo-kani` executable that is invoked directly.
    pub launcher: PathBuf,
}

impl KaniInstallation {
    /// Inspects the launcher's current file kind and Unix execute bits before dispatch.
    /// This snapshot does not pin the file or eliminate replacement/permission races at exec.
    pub(super) fn require_executable(&self) -> Result<(), KaniToolError> {
        let fault = |error| KaniToolError::Io {
            tool: KaniTool::Launcher,
            path: self.launcher.clone(),
            error,
        };
        let metadata = fs::metadata(&self.launcher).map_err(&fault)?;
        #[cfg(unix)]
        let executable = metadata.permissions().mode() & 0o111 != 0;
        #[cfg(not(unix))]
        let executable = true;
        if !metadata.is_file() || !executable {
            return Err(fault(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "launcher must be a regular executable file",
            )));
        }
        Ok(())
    }

    /// Locates the launcher the way Cargo resolves a subcommand — `$CARGO_HOME/bin`
    /// (default `$HOME/.cargo/bin`) first, then `PATH`.
    pub fn discover() -> Result<Self, KaniToolError> {
        Self::discover_from(
            env::var_os("HOME"),
            env::var_os("CARGO_HOME"),
            env::var_os("PATH"),
        )
    }

    /// `discover`'s resolution logic, taking each environment variable as an explicit argument
    /// instead of reading the process environment. `discover` is the only caller in this crate;
    /// tests call this directly so every `HOME`/`CARGO_HOME`/`PATH` combination is
    /// exercised as a pure function of its arguments, never by mutating process-wide state with
    /// `env::set_var`/`env::remove_var`, which races with any other thread reading `environ` —
    /// including a concurrently spawned child process snapshotting the environment at fork/exec.
    fn discover_from(
        home: Option<OsString>,
        cargo_home: Option<OsString>,
        path: Option<OsString>,
    ) -> Result<Self, KaniToolError> {
        let home = home.map(PathBuf::from);
        let launcher_name = format!("cargo-kani{}", env::consts::EXE_SUFFIX);
        let cargo_home = cargo_home
            .map(PathBuf::from)
            .or_else(|| home.as_ref().map(|home| home.join(".cargo")));
        let launcher = cargo_home
            .map(|directory| directory.join("bin").join(&launcher_name))
            .into_iter()
            .chain(
                path.map(|value| env::split_paths(&value).collect::<Vec<_>>())
                    .unwrap_or_default()
                    .into_iter()
                    .map(|directory| directory.join(&launcher_name)),
            )
            .find(|candidate| candidate.is_file())
            .ok_or_else(|| KaniToolError::Missing {
                tool: KaniTool::Launcher,
                path: PathBuf::from(&launcher_name),
            })?;
        Ok(Self { launcher })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::kani::test_support::discover_scratch;

    /// `discover_from` is a pure function of its arguments, so every `HOME`/`CARGO_HOME`/`PATH`
    /// combination is exercised directly here with no process environment mutation.
    ///
    /// Trace: FR-017-AC-2, TC-027
    #[test]
    fn tc_027_the_launcher_resolves_through_cargo_home_then_path() {
        let directory = discover_scratch("launcher");
        let cargo_home = directory.join("cargo-home");
        let on_path = directory.join("on-path");
        fs::create_dir_all(cargo_home.join("bin")).unwrap();
        fs::create_dir_all(&on_path).unwrap();
        let launcher_name = format!("cargo-kani{}", env::consts::EXE_SUFFIX);

        // Nothing to find: refused as a missing launcher.
        let error = KaniInstallation::discover_from(
            None,
            Some(cargo_home.clone().into_os_string()),
            Some(on_path.clone().into_os_string()),
        )
        .unwrap_err();
        assert!(
            matches!(
                error,
                KaniToolError::Missing {
                    tool: KaniTool::Launcher,
                    ..
                }
            ),
            "got {error}"
        );

        // Only on PATH: found there, and HOME need not be set.
        fs::write(on_path.join(&launcher_name), b"").unwrap();
        let installation = KaniInstallation::discover_from(
            None,
            Some(cargo_home.clone().into_os_string()),
            Some(on_path.clone().into_os_string()),
        )
        .unwrap();
        assert_eq!(installation.launcher, on_path.join(&launcher_name));

        // `$CARGO_HOME/bin` wins over PATH.
        fs::write(cargo_home.join("bin").join(&launcher_name), b"").unwrap();
        let installation = KaniInstallation::discover_from(
            None,
            Some(cargo_home.clone().into_os_string()),
            Some(on_path.into_os_string()),
        )
        .unwrap();
        assert_eq!(
            installation.launcher,
            cargo_home.join("bin").join(&launcher_name)
        );

        let _ = fs::remove_dir_all(directory);
    }
}
