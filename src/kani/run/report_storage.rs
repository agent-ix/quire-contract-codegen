//! O-owned anonymous report collection and sealed-descriptor consumer validation.
//!
//! No report pathname exists. A nonblocking pipe is drained in finite event-loop work; quiet is
//! never EOF. Only actual EOF permits immutable memfd handoff. The collector reserves its entire
//! possible backing before exposure, including the detection byte, irrespective of sparse size.

use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Seek, SeekFrom},
    os::{
        fd::{AsFd, AsRawFd, OwnedFd},
        unix::fs::OpenOptionsExt,
    },
    time::Instant,
};

use rustix::{
    fs::{fcntl_add_seals, fcntl_get_seals, fstat, memfd_create, MemfdFlags, SealFlags},
    pipe::{fcntl_getpipe_size, pipe_with, PipeFlags},
};
use serde::{Deserialize, Serialize};

use super::{outer_setup::PreparedOuter, pipe_policy};

pub(super) const REPORT_SLOT: i32 = 5;
const READ_BYTES: usize = 65_536;
const READS_PER_TICK: usize = 4;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PipeIdentity {
    device: u64,
    inode: u64,
}

impl PipeIdentity {
    fn original(pipe: impl AsFd) -> io::Result<Self> {
        let metadata = fstat(pipe)?;
        if rustix::fs::FileType::from_raw_mode(metadata.st_mode) != rustix::fs::FileType::Fifo {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "report endpoint is not a pipe",
            ));
        }
        Ok(Self {
            device: metadata.st_dev,
            inode: metadata.st_ino,
        })
    }

    pub(super) fn verify(&self, pipe: impl AsFd) -> io::Result<()> {
        if Self::original(pipe)? != *self {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "report pipe identity changed",
            ));
        }
        Ok(())
    }
}

/// Entire finite backing reservation, checked before exposure and added to every RSS tick.
#[derive(Clone, Copy, Debug)]
pub(super) struct BackingReserve {
    pub(super) pipe_bytes: u64,
    pub(super) memfd_bytes: u64,
}

impl BackingReserve {
    pub(super) fn charge(self, owned_rss: u64, caller_buffers: u64) -> io::Result<u64> {
        owned_rss
            .checked_add(caller_buffers)
            .and_then(|total| total.checked_add(self.pipe_bytes))
            .and_then(|total| total.checked_add(self.memfd_bytes))
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "whole-run backing charge overflow",
                )
            })
    }
}

#[derive(Debug)]
pub(super) enum ReportError {
    Io(io::Error),
    Deadline,
    CapacityChanged,
    ReportCapExceeded,
    WriterNotTransferred,
    MissingEof,
    MissingSeals,
    DescriptorIdentityChanged,
    ContentSizeMismatch,
    WriterAccess,
    WriterSlot,
    WriterClose(nix::errno::Errno),
}

impl From<rustix::io::Errno> for ReportError {
    fn from(error: rustix::io::Errno) -> Self {
        Self::Io(error.into())
    }
}

impl From<io::Error> for ReportError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl std::fmt::Display for ReportError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "unnamed report I/O: {error}"),
            other => write!(formatter, "unnamed report refused: {other:?}"),
        }
    }
}

impl std::error::Error for ReportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::WriterClose(error) => Some(error),
            _ => None,
        }
    }
}

/// Entry-only pipe acquisition primitive, after the role protocol has authenticated O's original
/// pipe identity and the fixed child mapping. This function does not authenticate that protocol.
/// It requires actual single-thread entry and never adopts or retries closure of an inherited fd.
pub(super) fn acquire_inner_writer(expected: &PipeIdentity) -> Result<File, ReportError> {
    super::outer_setup::require_single_thread().map_err(io::Error::other)?;
    let flags = i32::try_from((rustix::fs::OFlags::NONBLOCK | rustix::fs::OFlags::CLOEXEC).bits())
        .map_err(|_| io::Error::other("writer open flags exceed platform range"))?;
    // Opening an occupied slot creates new ownership. No FromRawFd or borrowed raw adoption is
    // involved; the expected identity must originate from O before the original child mapping.
    let writer = OpenOptions::new()
        .write(true)
        .custom_flags(flags)
        .open(format!("/proc/self/fd/{REPORT_SLOT}"))?;
    if writer.as_raw_fd() == REPORT_SLOT {
        return Err(ReportError::WriterSlot);
    }
    expected.verify(&writer)?;
    let access = rustix::fs::fcntl_getfl(&writer)?;
    if access & rustix::fs::OFlags::ACCMODE != rustix::fs::OFlags::WRONLY
        || !access.contains(rustix::fs::OFlags::NONBLOCK)
        || !rustix::io::fcntl_getfd(&writer)?.contains(rustix::io::FdFlags::CLOEXEC)
    {
        return Err(ReportError::WriterAccess);
    }
    // This is the only raw-slot operation: REPORT_SLOT is the authenticated, occupied mapping
    // in single-thread entry, with no intervening close/rebind. EINTR/error never retries the
    // integer; the caller must use its already-owned cancellation path instead.
    nix::unistd::close(REPORT_SLOT).map_err(ReportError::WriterClose)?;
    Ok(writer)
}

/// O alone owns its reader/memfd; the spawn writer is transferred exactly once to child mapping.
pub(super) struct ReportCollector {
    reader: OwnedFd,
    writer: Option<OwnedFd>,
    retained: File,
    identity: PipeIdentity,
    capacity: usize,
    reserve: BackingReserve,
    collected: usize,
    limit: usize,
    detection: usize,
    eof: bool,
}

impl ReportCollector {
    /// Called only by prepared single-thread O, before ANY child or writer exposure.
    pub(super) fn prepare(outer: &PreparedOuter<'_>) -> Result<Self, ReportError> {
        outer.require_creator_live().map_err(io::Error::other)?;
        let limit = usize::try_from(super::REPORT_CONTENT_BYTES)
            .map_err(|_| io::Error::other("report cap exceeds platform range"))?;
        let detection = limit.checked_add(1).ok_or(ReportError::ReportCapExceeded)?;
        let (reader, writer) = pipe_with(PipeFlags::CLOEXEC | PipeFlags::NONBLOCK)?;
        let identity = PipeIdentity::original(&writer)?;
        identity.verify(&reader)?;
        let capacity = fcntl_getpipe_size(&reader)?;
        if capacity == 0 || fcntl_getpipe_size(&writer)? != capacity {
            return Err(ReportError::CapacityChanged);
        }
        let page = u64::try_from(rustix::param::page_size())
            .map_err(|_| io::Error::other("report page size exceeds platform range"))?;
        if page == 0 || !page.is_power_of_two() {
            return Err(io::Error::other("report page size is unavailable").into());
        }
        let reserve = BackingReserve {
            pipe_bytes: round_pages(
                u64::try_from(capacity)
                    .map_err(|_| io::Error::other("pipe capacity exceeds platform range"))?,
                page,
            )?,
            memfd_bytes: round_pages(
                u64::try_from(detection)
                    .map_err(|_| io::Error::other("report cap exceeds platform range"))?,
                page,
            )?,
        };
        let descriptor = memfd_create(
            "quire-kani-report",
            MemfdFlags::CLOEXEC | MemfdFlags::ALLOW_SEALING,
        )?;
        if !fcntl_get_seals(&descriptor)?.is_empty() {
            return Err(ReportError::MissingSeals);
        }
        let retained = File::from(descriptor);
        if retained.metadata()?.len() != 0 {
            return Err(ReportError::ContentSizeMismatch);
        }
        // Only this actual O role is confined. C and L remain unchanged. All later contained
        // writer-bearing forks/execs inherit this operation-wide capacity restriction.
        pipe_policy::install()?;
        if fcntl_getpipe_size(&reader)? != capacity {
            return Err(ReportError::CapacityChanged);
        }
        Ok(Self {
            reader,
            writer: Some(writer),
            retained,
            identity,
            capacity,
            reserve,
            collected: 0,
            limit,
            detection,
            eof: false,
        })
    }

    pub(super) fn identity(&self) -> PipeIdentity {
        self.identity
    }
    pub(super) fn reserve(&self) -> BackingReserve {
        self.reserve
    }

    /// The owner must check its whole-run charge before this child-only mapping transfer.
    pub(super) fn take_spawn_writer(&mut self) -> Result<OwnedFd, ReportError> {
        self.writer.take().ok_or(ReportError::WriterNotTransferred)
    }

    /// Finite concurrent event-loop work; the next control/deadline/accounting tick remains live.
    pub(super) fn drain_tick(&mut self, deadline: Option<Instant>) -> Result<(), ReportError> {
        let mut bytes = [0; READ_BYTES];
        for _ in 0..READS_PER_TICK {
            check_deadline(deadline)?;
            if fcntl_getpipe_size(&self.reader)? != self.capacity {
                return Err(ReportError::CapacityChanged);
            }
            if self.eof {
                return Ok(());
            }
            let remaining = self
                .detection
                .checked_sub(self.collected)
                .ok_or(ReportError::ReportCapExceeded)?;
            if remaining == 0 {
                return Err(ReportError::ReportCapExceeded);
            }
            let length = remaining.min(bytes.len());
            match rustix::io::read(&self.reader, &mut bytes[..length]) {
                Ok(0) => {
                    self.eof = true;
                    return Ok(());
                }
                Ok(count) => {
                    // Never retain beyond the one detection byte, even between observer ticks.
                    self.collected = self
                        .collected
                        .checked_add(count)
                        .ok_or(ReportError::ReportCapExceeded)?;
                    let mut written = 0;
                    while written < count {
                        check_deadline(deadline)?;
                        match rustix::io::write(&self.retained, &bytes[written..count]) {
                            Ok(0) => {
                                return Err(io::Error::new(
                                    io::ErrorKind::WriteZero,
                                    "report collector made no progress",
                                )
                                .into())
                            }
                            Ok(count) => written += count,
                            Err(rustix::io::Errno::INTR) => {}
                            Err(error) => return Err(error.into()),
                        }
                    }
                    if self.collected > self.limit {
                        return Err(ReportError::ReportCapExceeded);
                    }
                }
                Err(rustix::io::Errno::AGAIN) => return Ok(()),
                Err(rustix::io::Errno::INTR) => {}
                Err(error) => return Err(error.into()),
            }
        }
        Ok(())
    }

    pub(super) fn eof(&self) -> bool {
        self.eof
    }

    /// Actual EOF is necessary; outer orchestration separately confirms I/M/all-writer settlement.
    pub(super) fn seal(mut self, deadline: Option<Instant>) -> Result<SealedReport, ReportError> {
        check_deadline(deadline)?;
        if self.writer.is_some() {
            return Err(ReportError::WriterNotTransferred);
        }
        if !self.eof {
            return Err(ReportError::MissingEof);
        }
        if self.retained.metadata()?.len()
            != u64::try_from(self.collected)
                .map_err(|_| io::Error::other("report size exceeds platform range"))?
        {
            return Err(ReportError::ContentSizeMismatch);
        }
        self.retained.seek(SeekFrom::Start(0))?;
        fcntl_add_seals(&self.retained, required_seals())?;
        if !fcntl_get_seals(&self.retained)?.contains(required_seals()) {
            return Err(ReportError::MissingSeals);
        }
        check_deadline(deadline)?;
        Ok(SealedReport {
            descriptor: self.retained,
            bytes: self.collected,
        })
    }
}

pub(super) struct SealedReport {
    pub(super) descriptor: File,
    pub(super) bytes: usize,
}

fn required_seals() -> SealFlags {
    SealFlags::WRITE | SealFlags::GROW | SealFlags::SHRINK | SealFlags::SEAL
}

/// C validates the actual safely received CLOEXEC descriptor before allocating or reading bytes.
pub(super) fn read_received(
    descriptor: OwnedFd,
    expected_bytes: usize,
    deadline: Option<Instant>,
) -> Result<Option<Vec<u8>>, ReportError> {
    check_deadline(deadline)?;
    let limit = usize::try_from(super::REPORT_CONTENT_BYTES)
        .map_err(|_| io::Error::other("report cap exceeds platform range"))?;
    if expected_bytes > limit {
        return Err(ReportError::ReportCapExceeded);
    }
    if !rustix::io::fcntl_getfd(&descriptor)?.contains(rustix::io::FdFlags::CLOEXEC)
        || !fcntl_get_seals(&descriptor)?.contains(required_seals())
    {
        return Err(ReportError::MissingSeals);
    }
    let mut file = File::from(descriptor);
    let identity = fstat(&file)?;
    if rustix::fs::FileType::from_raw_mode(identity.st_mode) != rustix::fs::FileType::RegularFile
        || file.metadata()?.len()
            != u64::try_from(expected_bytes)
                .map_err(|_| io::Error::other("report size exceeds platform range"))?
    {
        return Err(ReportError::ContentSizeMismatch);
    }
    file.seek(SeekFrom::Start(0))?;
    let mut collected = Vec::with_capacity(expected_bytes);
    let mut bytes = [0; READ_BYTES];
    loop {
        check_deadline(deadline)?;
        let count = file.read(&mut bytes)?;
        if count == 0 {
            break;
        }
        let next = collected
            .len()
            .checked_add(count)
            .ok_or(ReportError::ReportCapExceeded)?;
        if next > expected_bytes {
            return Err(ReportError::ContentSizeMismatch);
        }
        collected.extend_from_slice(&bytes[..count]);
    }
    let final_identity = fstat(&file)?;
    if identity.st_dev != final_identity.st_dev || identity.st_ino != final_identity.st_ino {
        return Err(ReportError::DescriptorIdentityChanged);
    }
    if collected.len() != expected_bytes {
        return Err(ReportError::ContentSizeMismatch);
    }
    check_deadline(deadline)?;
    Ok((!collected.is_empty()).then_some(collected))
}

fn check_deadline(deadline: Option<Instant>) -> Result<(), ReportError> {
    if deadline.is_some_and(|deadline| Instant::now() >= deadline) {
        Err(ReportError::Deadline)
    } else {
        Ok(())
    }
}

fn round_pages(bytes: u64, page: u64) -> io::Result<u64> {
    bytes
        .checked_add(
            page.checked_sub(1)
                .ok_or_else(|| io::Error::other("zero page size"))?,
        )
        .map(|rounded| rounded / page)
        .and_then(|pages| pages.checked_mul(page))
        .ok_or_else(|| io::Error::other("report backing reservation overflow"))
}
