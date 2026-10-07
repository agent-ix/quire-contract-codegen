//! Internal original backend stdin, captured before any per-run descriptor can reuse FD0.

use std::{
    io,
    os::fd::{BorrowedFd, OwnedFd},
};

/// The original backend stdin at the ordinary inherited-exec boundary.
///
/// C acquires this before controls, validation files or pipes. Initial absence is checked without
/// borrowing FD0 or allocating a descriptor. Once original presence is established, every failed
/// inspection/duplication refuses rather than converting an originally open input to Closed.
#[derive(Debug)]
pub(super) enum OriginalStdin {
    /// Restore this actual open file description as backend stdin.
    ///
    /// The clone's CLOEXEC flag concerns transport. Original exec visibility was already
    /// established during capture; the backend receives this actual open description.
    Open(OwnedFd),
    /// Leave backend FD0 closed at exec entry.
    ///
    /// The backend's own runtime may subsequently sanitize closed standard descriptors.
    Closed,
}

#[derive(Debug)]
pub(super) enum StdinInventoryError {
    Socket,
    HostDirectory,
    Inspection(io::Error),
}

impl std::fmt::Display for StdinInventoryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Socket => formatter.write_str("original backend stdin has socket type"),
            Self::HostDirectory => {
                formatter.write_str("original backend stdin fd0 has host-directory type")
            }
            Self::Inspection(error) => {
                write!(formatter, "captured backend stdin inspection: {error}")
            }
        }
    }
}

impl std::error::Error for StdinInventoryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Socket | Self::HostDirectory => None,
            Self::Inspection(error) => Some(error),
        }
    }
}

impl OriginalStdin {
    /// Give one run an owned reference to the originally captured open description. Reuse of
    /// ambient fd0 after initial capture cannot change this source; Closed performs no syscall.
    #[cfg(target_os = "linux")]
    pub(super) fn clone_for_run(&self) -> io::Result<Self> {
        match self {
            Self::Open(descriptor) => rustix::io::fcntl_dupfd_cloexec(descriptor, 3)
                .map(Self::Open)
                .map_err(Into::into),
            Self::Closed => Ok(Self::Closed),
        }
    }

    /// Inspect the actual retained Open pin at eligible launch, never the now-reusable fd0.
    /// Closed retains its ordinary-exec meaning without issuing fstat on an absent descriptor.
    pub(super) fn inspect_backend(&self) -> Result<(), StdinInventoryError> {
        let Self::Open(descriptor) = self else {
            return Ok(());
        };
        let actual = rustix::fs::fstat(descriptor)
            .map_err(|error| StdinInventoryError::Inspection(error.into()))?;
        match rustix::fs::FileType::from_raw_mode(actual.st_mode) {
            rustix::fs::FileType::Socket => return Err(StdinInventoryError::Socket),
            // Preserving this description would expose original host-directory lookup authority
            // across the private root. Refuse the root capability; never rewrite input to admit it.
            rustix::fs::FileType::Directory => return Err(StdinInventoryError::HostDirectory),
            _ => {}
        }
        Ok(())
    }
    /// The caller must keep standard descriptors stable during this initial snapshot, as required
    /// by std's Stdin borrowing contract. This function never changes original descriptor flags.
    #[cfg(target_os = "linux")]
    pub(super) fn capture_original() -> io::Result<Self> {
        // statfs/lstat are path-only kernel operations: they create no fd that could reuse an
        // initially absent slot. Missing procfs is not evidence that stdin was originally closed.
        if rustix::fs::statfs("/proc/self/fd")?.f_type != rustix::fs::PROC_SUPER_MAGIC
            || !std::fs::symlink_metadata("/proc/self/fd")?.is_dir()
        {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "original stdin presence requires real procfs",
            ));
        }
        // A real self link must identify this caller in the proc view. These path-only checks
        // precede descriptor creation; a foreign proc view is a missing capture capability.
        let self_link = std::fs::read_link("/proc/self")?;
        if self_link != std::path::PathBuf::from(std::process::id().to_string()) {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "procfs self link does not identify the caller",
            ));
        }
        match std::fs::symlink_metadata("/proc/self/fd/0") {
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Self::Closed),
            Err(error) => return Err(error),
            Ok(metadata) if !metadata.file_type().is_symlink() => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "original stdin entry is not a procfs descriptor link",
                ));
            }
            Ok(_) => {}
        }
        // Absence after the positive original entry is instability, never Closed. Following
        // the proc link uses stat, not opening a new description or reserving a descriptor slot.
        let expected = std::fs::metadata("/proc/self/fd/0")?;
        use std::os::{fd::AsFd, unix::fs::MetadataExt};
        let stdin = std::io::stdin();
        let before = rustix::fs::fstat(&stdin)?;
        let expected_type = rustix::fs::FileType::from_raw_mode(expected.mode());
        if before.st_dev != expected.dev()
            || before.st_ino != expected.ino()
            || rustix::fs::FileType::from_raw_mode(before.st_mode) != expected_type
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "original stdin identity changed during capture",
            ));
        }
        let original_flags = rustix::io::fcntl_getfd(&stdin)? & rustix::io::FdFlags::CLOEXEC;
        let original_access = rustix::fs::fcntl_getfl(&stdin)? & rustix::fs::OFlags::ACCMODE;
        let captured = Self::capture(stdin.as_fd())?;
        let after = rustix::fs::fstat(&stdin)?;
        // Shared open-description flags can change in another process without violating C's
        // descriptor-table precondition. Only access mode and original exec visibility are the
        // SPEC's instability comparison; O_NONBLOCK/O_APPEND are neither frozen nor restored.
        if rustix::io::fcntl_getfd(&stdin)? & rustix::io::FdFlags::CLOEXEC != original_flags
            || rustix::fs::fcntl_getfl(&stdin)? & rustix::fs::OFlags::ACCMODE != original_access
            || before.st_dev != after.st_dev
            || before.st_ino != after.st_ino
            || rustix::fs::FileType::from_raw_mode(after.st_mode) != expected_type
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "original stdin identity changed after capture",
            ));
        }
        if matches!(&captured, Self::Closed)
            != original_flags.contains(rustix::io::FdFlags::CLOEXEC)
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "original stdin exec visibility changed during capture",
            ));
        }
        if let Self::Open(descriptor) = &captured {
            let cloned = rustix::fs::fstat(descriptor)?;
            if before.st_dev != cloned.st_dev
                || before.st_ino != cloned.st_ino
                || rustix::fs::FileType::from_raw_mode(cloned.st_mode) != expected_type
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "captured stdin description identity changed",
                ));
            }
        }
        Ok(captured)
    }

    #[cfg(not(target_os = "linux"))]
    pub(super) fn capture_original() -> io::Result<Self> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "bounded original stdin capture requires Linux procfs",
        ))
    }

    /// Capture ordinary inherited-exec semantics from a known-valid original source descriptor.
    ///
    /// The source must remain valid, with stable descriptor flags, throughout this call. Reading
    /// flags and cloning are separate operations, not an atomic snapshot of ambient stdio. A
    /// source marked CLOEXEC becomes [`Self::Closed`]; a transport clone's CLOEXEC does not.
    fn capture(source: BorrowedFd<'_>) -> io::Result<Self> {
        if rustix::io::fcntl_getfd(source)?.contains(rustix::io::FdFlags::CLOEXEC) {
            Ok(Self::Closed)
        } else {
            source.try_clone_to_owned().map(Self::Open)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::fd::AsFd;

    /// Trace: FR-034-AC-16, FR-034-AC-17
    #[test]
    fn original_cloexec_source_is_closed_at_the_inherited_exec_boundary() {
        let (source, _writer) = rustix::pipe::pipe_with(rustix::pipe::PipeFlags::CLOEXEC).unwrap();
        assert!(matches!(
            OriginalStdin::capture(source.as_fd()).unwrap(),
            OriginalStdin::Closed
        ));
        assert!(rustix::io::fcntl_getfd(&source)
            .unwrap()
            .contains(rustix::io::FdFlags::CLOEXEC));
    }

    /// Trace: FR-034-AC-16, FR-034-AC-17
    #[test]
    fn transport_cloexec_does_not_reclassify_an_open_original_source() {
        let (source, writer) = rustix::pipe::pipe_with(rustix::pipe::PipeFlags::CLOEXEC).unwrap();
        rustix::io::fcntl_setfd(&source, rustix::io::FdFlags::empty()).unwrap();
        let OriginalStdin::Open(clone) = OriginalStdin::capture(source.as_fd()).unwrap() else {
            panic!("an open original source must remain Open");
        };
        assert!(rustix::io::fcntl_getfd(&clone)
            .unwrap()
            .contains(rustix::io::FdFlags::CLOEXEC));
        rustix::io::write(&writer, b"actual-original-stdin").unwrap();
        let mut bytes = [0; 21];
        let count = rustix::io::read(&clone, &mut bytes).unwrap();
        assert_eq!(&bytes[..count], b"actual-original-stdin");
        assert_eq!(
            rustix::io::fcntl_getfd(&source).unwrap(),
            rustix::io::FdFlags::empty()
        );
    }

    /// Trace: FR-034-AC-36.
    #[test]
    fn open_actual_socket_stdin_is_refused_by_owned_inventory() {
        let (source, _peer) = std::os::unix::net::UnixStream::pair().unwrap();
        rustix::io::fcntl_setfd(&source, rustix::io::FdFlags::empty()).unwrap();
        let captured = OriginalStdin::capture(source.as_fd()).unwrap();
        assert!(matches!(
            captured.inspect_backend(),
            Err(StdinInventoryError::Socket)
        ));
    }

    /// Trace: FR-034-AC-36.
    #[test]
    fn original_cloexec_socket_is_closed_and_has_no_backend_socket_inventory() {
        let (source, _peer) = std::os::unix::net::UnixStream::pair().unwrap();
        rustix::io::fcntl_setfd(&source, rustix::io::FdFlags::CLOEXEC).unwrap();
        let captured = OriginalStdin::capture(source.as_fd()).unwrap();
        assert!(matches!(captured, OriginalStdin::Closed));
        assert!(captured.inspect_backend().is_ok());
    }

    /// Trace: FR-034-AC-36.
    #[test]
    fn captured_directory_stdin_cannot_cross_the_private_root() {
        let source = std::fs::File::open("/").unwrap();
        rustix::io::fcntl_setfd(&source, rustix::io::FdFlags::empty()).unwrap();
        let captured = OriginalStdin::capture(source.as_fd()).unwrap();
        assert!(matches!(
            captured.inspect_backend(),
            Err(StdinInventoryError::HostDirectory)
        ));
        assert_eq!(
            rustix::io::fcntl_getfd(&source).unwrap(),
            rustix::io::FdFlags::empty()
        );
    }

    /// Trace: FR-034-AC-36.
    #[test]
    #[cfg(target_os = "linux")]
    fn captured_regular_input_remains_admitted_without_rewriting_shared_flags() {
        let source =
            rustix::fs::memfd_create("original-regular-input", rustix::fs::MemfdFlags::empty())
                .unwrap();
        let original = rustix::fs::fcntl_getfl(&source).unwrap();
        let captured = OriginalStdin::capture(source.as_fd()).unwrap();
        assert!(matches!(captured, OriginalStdin::Open(_)));
        assert!(captured.inspect_backend().is_ok());
        assert_eq!(rustix::fs::fcntl_getfl(&source).unwrap(), original);
        rustix::io::write(&source, b"unchanged-original-file").unwrap();
        let OriginalStdin::Open(pin) = captured else {
            panic!("regular input must remain Open");
        };
        rustix::fs::seek(&source, rustix::fs::SeekFrom::Start(0)).unwrap();
        let mut bytes = [0; 23];
        let count = rustix::io::read(&pin, &mut bytes).unwrap();
        assert_eq!(&bytes[..count], b"unchanged-original-file");
    }
}
