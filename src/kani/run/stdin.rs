//! Explicit original backend stdin, captured before any guardian control descriptors (FR-034).

use std::{
    io,
    os::fd::{BorrowedFd, OwnedFd},
};

/// The original backend stdin at the ordinary inherited-exec boundary.
///
/// Acquire this before creating control pairs, pipes or other descriptors. A library hosted in
/// a process with absent FD0 supplies [`Self::Closed`] explicitly; it must not manufacture a
/// borrow of that absent descriptor. The caller retains this value for the execution request.
#[derive(Debug)]
pub enum OriginalStdin {
    /// Restore this actual open file description as backend stdin.
    ///
    /// The descriptor's CLOEXEC flag concerns its transport, not whether the backend stdin is
    /// open. Explicitly supplying this variant therefore restores it even when that flag is set.
    Open(OwnedFd),
    /// Leave backend FD0 closed at exec entry.
    ///
    /// The backend's own runtime may subsequently sanitize closed standard descriptors.
    Closed,
}

impl OriginalStdin {
    /// Capture ordinary inherited-exec semantics from a known-valid original source descriptor.
    ///
    /// The source must remain valid, with stable descriptor flags, throughout this call. Reading
    /// flags and cloning are separate operations, not an atomic snapshot of ambient stdio. A
    /// source marked CLOEXEC becomes [`Self::Closed`]; a transport clone's CLOEXEC does not.
    /// Callers that already own an explicitly open stdin can directly supply [`Self::Open`].
    pub fn capture(source: BorrowedFd<'_>) -> io::Result<Self> {
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
}
