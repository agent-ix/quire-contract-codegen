//! C's prepared bounded captures and actual original reader-thread ownership.
//!
//! Output pipes and full capture reservations exist before L. Once reader creation starts,
//! failures retain any already-created JoinHandle in this same owner. Whole-chain settlement
//! must precede joining; neither a stop flag nor dropping this object proves writer EOF.

use std::{
    fs::File,
    io,
    os::fd::OwnedFd,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use rustix::pipe::{pipe_with, PipeFlags};

use super::{
    caller_bootstrap::CallerRoleSettlement,
    launch::{finish_capture, CaptureFailure, CaptureFlags, Captured, PreparedCapture},
};

/// Real reader-thread results after owned process writers and creator have settled.
pub(super) struct SettledCaptures {
    pub(super) stdout: Captured,
    pub(super) stderr: Captured,
}

pub(super) struct CallerStreams {
    stdout_prepared: Option<PreparedCapture>,
    stderr_prepared: Option<PreparedCapture>,
    stdout_reader: Option<File>,
    stderr_reader: Option<File>,
    pub(super) stdout: Option<JoinHandle<Captured>>,
    pub(super) stderr: Option<JoinHandle<Captured>>,
    pub(super) flags: CaptureFlags,
    pub(super) combined_text: Option<PreparedCaptureText>,
    reservation: u64,
    started: bool,
}

impl CallerStreams {
    pub(super) fn prepare(limit: usize) -> io::Result<(Self, OwnedFd, OwnedFd)> {
        let stdout = PreparedCapture::prepare(limit)?;
        let stderr = PreparedCapture::prepare(limit)?;
        let combined_limit = limit
            .checked_mul(2)
            .and_then(|bytes| bytes.checked_add(1))
            .ok_or_else(|| io::Error::other("combined capture reservation overflow"))?;
        let combined_text = PreparedCaptureText::prepare(combined_limit)?;
        let reservation = stdout
            .reserved_bytes()?
            .checked_add(stderr.reserved_bytes()?)
            .and_then(|bytes| bytes.checked_add(combined_text.reserved_bytes().ok()?))
            .and_then(|bytes| bytes.checked_add(u64::try_from(std::mem::size_of::<Self>()).ok()?))
            .and_then(|bytes| {
                bytes.checked_add(u64::try_from(2 * std::mem::size_of::<AtomicBool>()).ok()?)
            })
            .ok_or_else(|| io::Error::other("caller capture reservation overflow"))?;
        let (stdout_reader, stdout_writer) = pipe_with(PipeFlags::CLOEXEC)?;
        let (stderr_reader, stderr_writer) = pipe_with(PipeFlags::CLOEXEC)?;
        Ok((
            Self {
                stdout_prepared: Some(stdout),
                stderr_prepared: Some(stderr),
                stdout_reader: Some(File::from(stdout_reader)),
                stderr_reader: Some(File::from(stderr_reader)),
                stdout: None,
                stderr: None,
                flags: CaptureFlags {
                    stop: Arc::new(AtomicBool::new(false)),
                    failed: Arc::new(AtomicBool::new(false)),
                },
                combined_text: Some(combined_text),
                reservation,
                started: false,
            },
            stdout_writer,
            stderr_writer,
        ))
    }

    pub(super) fn reserved_bytes(&self) -> u64 {
        self.reservation
    }

    /// Call only after storing the actual L spawner. This records each successful thread before
    /// attempting the next one; a later failure cannot discard the first retained JoinHandle.
    pub(super) fn start(&mut self) -> io::Result<()> {
        if self.started {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "capture creation already attempted",
            ));
        }
        self.started = true;
        let stdout = self
            .stdout_prepared
            .take()
            .ok_or_else(|| io::Error::other("prepared stdout capture unavailable"))?;
        let reader = self
            .stdout_reader
            .take()
            .ok_or_else(|| io::Error::other("owned stdout pipe unavailable"))?;
        self.stdout = Some(stdout.spawn(reader, &self.flags)?);
        let stderr = self
            .stderr_prepared
            .take()
            .ok_or_else(|| io::Error::other("prepared stderr capture unavailable"))?;
        let reader = self
            .stderr_reader
            .take()
            .ok_or_else(|| io::Error::other("owned stderr pipe unavailable"))?;
        self.stderr = Some(stderr.spawn(reader, &self.flags)?);
        Ok(())
    }
    /// The genuine C role token precedes the stop flag; all child/mapping writer descriptions
    /// are then gone. A requested stop is not a reader join. Retain both handles on expiry and
    /// consume them only after positive is_finished observations within the same cutoff.
    pub(super) fn settle(&mut self, roles: &CallerRoleSettlement) -> io::Result<SettledCaptures> {
        self.flags.stop.store(true, Ordering::Release);
        let cutoff = roles.cutoff();
        while self
            .stdout
            .as_ref()
            .is_some_and(|reader| !reader.is_finished())
            || self
                .stderr
                .as_ref()
                .is_some_and(|reader| !reader.is_finished())
        {
            let remaining = cutoff
                .checked_duration_since(Instant::now())
                .filter(|remaining| !remaining.is_zero())
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::TimedOut, "owned capture join unconfirmed")
                })?;
            thread::park_timeout(remaining.min(Duration::from_millis(20)));
        }
        if Instant::now() >= cutoff {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "owned capture settlement cutoff elapsed",
            ));
        }
        let absent = || {
            Err(CaptureFailure::Unread {
                detail: "the capture reader was not created".to_owned(),
            })
        };
        let stdout = self
            .stdout
            .take()
            .map(finish_capture)
            .unwrap_or_else(absent);
        let stderr = self
            .stderr
            .take()
            .map(finish_capture)
            .unwrap_or_else(absent);
        if Instant::now() >= cutoff {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "owned capture joins exceeded settlement cutoff",
            ));
        }
        Ok(SettledCaptures { stdout, stderr })
    }
}

/// Full byte-capture UTF8 expansion storage. At most three UTF8 bytes replace each invalid
/// source byte; this exact finite capacity is reserved and charged before any role is launched.
pub(super) struct PreparedCaptureText {
    text: String,
    source_limit: usize,
}

impl PreparedCaptureText {
    fn prepare(source_limit: usize) -> io::Result<Self> {
        let capacity = source_limit
            .checked_mul(3)
            .ok_or_else(|| io::Error::other("capture text reservation overflow"))?;
        let mut text = String::new();
        text.try_reserve_exact(capacity).map_err(io::Error::other)?;
        Ok(Self { text, source_limit })
    }

    fn reserved_bytes(&self) -> io::Result<u64> {
        let bytes = self
            .text
            .capacity()
            .checked_add(std::mem::size_of::<Self>())
            .ok_or_else(|| io::Error::other("capture text reservation overflow"))?;
        u64::try_from(bytes).map_err(io::Error::other)
    }

    /// Same standard UTF8 validation/replacement semantics, written into the already owned
    /// String. No intermediate lossy Cow/String may allocate another full output after launch.
    pub(super) fn decode(mut self, bytes: &[u8]) -> io::Result<String> {
        self.append_lossy(bytes)?;
        Ok(self.text)
    }

    /// Combine the two actual bounded captures into their pre-L reserved storage. No lossy
    /// temporary String, joined byte Vec or post-launch growth is required.
    pub(super) fn decode_joined(
        mut self,
        stdout: &[u8],
        stderr: &[u8],
        limit: usize,
    ) -> io::Result<String> {
        let combined = limit
            .checked_mul(2)
            .and_then(|bytes| bytes.checked_add(1))
            .ok_or_else(|| io::Error::other("combined capture bound overflow"))?;
        if stdout.len() > limit || stderr.len() > limit || combined > self.source_limit {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "capture exceeds prepared joined bound",
            ));
        }
        self.append_lossy(stdout)?;
        self.text.push('\n');
        self.append_lossy(stderr)?;
        Ok(self.text)
    }

    fn append_lossy(&mut self, mut bytes: &[u8]) -> io::Result<()> {
        if bytes.len() > self.source_limit {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "capture exceeds prepared text bound",
            ));
        }
        while !bytes.is_empty() {
            match std::str::from_utf8(bytes) {
                Ok(valid) => {
                    self.text.push_str(valid);
                    break;
                }
                Err(error) => {
                    let valid = bytes
                        .get(..error.valid_up_to())
                        .ok_or_else(|| io::Error::other("capture UTF8 prefix is unavailable"))?;
                    self.text.push_str(
                        std::str::from_utf8(valid)
                            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?,
                    );
                    self.text.push('\u{fffd}');
                    let consumed = match error.error_len() {
                        Some(invalid) => error
                            .valid_up_to()
                            .checked_add(invalid)
                            .ok_or_else(|| io::Error::other("capture UTF8 position overflow"))?,
                        None => bytes.len(),
                    };
                    bytes = bytes
                        .get(consumed..)
                        .ok_or_else(|| io::Error::other("capture UTF8 remainder is unavailable"))?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Trace: FR-034-AC-18, FR-034-AC-34
    #[test]
    fn joined_capture_uses_its_original_reservation_and_preserves_stream_limits() {
        let text = PreparedCaptureText::prepare(9).unwrap();
        let capacity = text.text.capacity();
        let joined = text.decode_joined(b"a\xff", b"b\xe2\x82", 4).unwrap();
        assert_eq!(joined, "a\u{fffd}\nb\u{fffd}");
        assert_eq!(joined.capacity(), capacity);
        assert!(PreparedCaptureText::prepare(9)
            .unwrap()
            .decode_joined(b"12345", b"", 4)
            .is_err());
        assert!(PreparedCaptureText::prepare(8)
            .unwrap()
            .decode_joined(b"", b"", 4)
            .is_err());
    }

    /// Trace: FR-034-AC-18, FR-034-AC-34.
    #[test]
    fn reserved_capture_text_preserves_standard_lossy_output_without_accepting_over_limit() {
        for bytes in [
            b"valid".as_slice(),
            b"a\xffb\xe2\x82",
            b"\xf0\x80\x80\x80",
            b"\xed\xa0\x80",
        ] {
            let text = PreparedCaptureText::prepare(bytes.len()).unwrap();
            assert_eq!(text.decode(bytes).unwrap(), String::from_utf8_lossy(bytes));
        }
        let text = PreparedCaptureText::prepare(1).unwrap();
        assert_eq!(
            text.decode(b"ab").unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
    }
}
