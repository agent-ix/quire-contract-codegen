//! Actual inner INIT's descriptor protection after its final namespace/credential transitions.
//!
//! I keeps the required trusted channels, but arbitrary same-UID descendants must not reopen
//! them through procfs, ptrace or pidfd_getfd. Safe kernel operations remove I's ptrace bounding
//! authority, clear effective/permitted/inheritable and ambient capabilities, and confirm NNP
//! plus non-dumpability. Backend-specific syscall policy remains a separate exec boundary.

use std::io;

use rustix::{
    process::{dumpable_behavior, set_dumpable_behavior, DumpableBehavior},
    thread::{
        capabilities, capability_is_in_ambient_set, capability_is_in_bounding_set,
        clear_ambient_capability_set, no_new_privs, remove_capability_from_bounding_set,
        set_capabilities, set_no_new_privs, CapabilitySet, CapabilitySets,
    },
};

fn invalid() -> io::Error {
    io::Error::new(
        io::ErrorKind::Unsupported,
        "trusted inner owner protection could not be positively established",
    )
}

/// Called at genuine authenticated I entry, before Ready/Dispatch or any backend child. Failure
/// refuses admission; no backend is authorized with partially installed owner protection.
pub(super) fn protect_inner() -> io::Result<()> {
    super::outer_setup::require_single_thread().map_err(io::Error::other)?;
    if rustix::process::getpid().as_raw_nonzero().get() != 1
        || rustix::process::getuid().as_raw() != 0
        || rustix::process::getgid().as_raw() != 0
    {
        return Err(invalid());
    }
    set_no_new_privs(true)?;
    // Already absent is an actual kernel observation, not an ignored EPERM. If authority is
    // present but cannot safely be removed, this is unavailable admission with no Dispatch.
    if capability_is_in_bounding_set(CapabilitySet::SYS_PTRACE)? {
        remove_capability_from_bounding_set(CapabilitySet::SYS_PTRACE)?;
    }
    clear_ambient_capability_set()?;
    set_capabilities(
        None,
        CapabilitySets {
            effective: CapabilitySet::empty(),
            permitted: CapabilitySet::empty(),
            inheritable: CapabilitySet::empty(),
        },
    )?;
    // This follows all credential/capability transitions. I never changes credentials or execs
    // again; the separate same-PID installer child is the only process that execs the backend.
    set_dumpable_behavior(DumpableBehavior::NotDumpable)?;
    require_protected()
}

/// Recheck before exposing a backend or accepting its lifecycle events. A profile blocking an
/// attack does not replace these actual owner facts; syscall failure is never treated as zero.
pub(super) fn require_protected() -> io::Result<()> {
    let actual = capabilities(None)?;
    if !actual.effective.is_empty()
        || !actual.permitted.is_empty()
        || !actual.inheritable.is_empty()
        || capability_is_in_bounding_set(CapabilitySet::SYS_PTRACE)?
        || capability_is_in_ambient_set(CapabilitySet::SYS_PTRACE)?
        || !no_new_privs()?
        || dumpable_behavior()? != DumpableBehavior::NotDumpable
    {
        return Err(invalid());
    }
    Ok(())
}
