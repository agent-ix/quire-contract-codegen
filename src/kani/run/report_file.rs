//! The report file Kani exports for one run: its launch-unique path, the removal of a stale file
//! under that name and the bounded read (FR-017, L-7).

use std::{
    fs, io,
    io::Read,
    os::unix::fs::{MetadataExt, OpenOptionsExt},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};

use crate::kani::output::report::KaniReportRefusal;

/// Stem of the report file Kani exports into the request's target directory. Each launch adds
/// this process's id and a process-wide counter, so the name is unique to one launch among every
/// run sharing the directory, in this process and in others running at the same time.
const REPORT_FILE_STEM: &str = "quire-kani-report";

static REPORT_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Most bytes of the exported report this module reads. A report larger than this is refused,
/// never truncated: a truncated report is not JSON, and reading part of one would invent a
/// verdict.
const REPORT_LIMIT: u64 = super::REPORT_CONTENT_BYTES;

pub(super) fn fresh_report_path(target_directory: &Path) -> PathBuf {
    let sequence = REPORT_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    target_directory.join(format!(
        "{REPORT_FILE_STEM}-{}-{sequence}.json",
        std::process::id()
    ))
}

/// Removes a file left under this launch's own report name (a crashed earlier process that had
/// the same id and sequence), so a run that exports none can never be read as that one. Another
/// run's file has another name and is never touched.
pub(super) fn remove_stale_report(path: &Path) -> Result<(), KaniReportRefusal> {
    match fs::remove_file(path) {
        Err(error) if error.kind() != io::ErrorKind::NotFound => {
            Err(KaniReportRefusal::Unreadable {
                detail: error.to_string(),
            })
        }
        _ => Ok(()),
    }
}

/// Reads the exported report, bounded by [`REPORT_LIMIT`]. `None` is a report that was never
/// written; one that cannot be read, or is too large, is refused.
pub(super) fn read_report(
    path: &Path,
    deadline: Option<Instant>,
) -> Result<Option<Vec<u8>>, KaniReportRefusal> {
    let unreadable = |error: io::Error| KaniReportRefusal::Unreadable {
        detail: error.to_string(),
    };
    let limit = usize::try_from(REPORT_LIMIT)
        .map_err(|_| unreadable(io::Error::other("report cap exceeds platform range")))?;
    let check_deadline = || {
        if deadline.is_some_and(|deadline| Instant::now() >= deadline) {
            Err(unreadable(io::Error::new(
                io::ErrorKind::TimedOut,
                "original identity deadline elapsed during report read",
            )))
        } else {
            Ok(())
        }
    };
    check_deadline()?;
    // Do not block opening a malicious FIFO or follow a replaced symlink outside this run's file.
    let file = match fs::OpenOptions::new()
        .read(true)
        .custom_flags(
            i32::try_from((rustix::fs::OFlags::NONBLOCK | rustix::fs::OFlags::NOFOLLOW).bits())
                .map_err(|_| {
                    unreadable(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "report open flags exceed platform range",
                    ))
                })?,
        )
        .open(path)
    {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(KaniReportRefusal::Unreadable {
                detail: error.to_string(),
            })
        }
    };
    let identity = file.metadata().map_err(unreadable)?;
    if !identity.is_file() {
        return Err(unreadable(io::Error::new(
            io::ErrorKind::InvalidData,
            "run report is not a regular file",
        )));
    }
    let mut bytes = Vec::new();
    let detection = REPORT_LIMIT
        .checked_add(1)
        .ok_or_else(|| unreadable(io::Error::other("report detection bound overflow")))?;
    let mut bounded = file.take(detection);
    let mut chunk = [0; 65_536];
    loop {
        check_deadline()?;
        let count = bounded.read(&mut chunk).map_err(unreadable)?;
        if count == 0 {
            break;
        }
        bytes.extend_from_slice(&chunk[..count]);
    }
    if bytes.len() > limit {
        return Err(KaniReportRefusal::TooLarge { limit });
    }
    let retained = bounded.get_ref().metadata().map_err(unreadable)?;
    let current = fs::symlink_metadata(path).map_err(unreadable)?;
    if !current.is_file()
        || retained.dev() != current.dev()
        || retained.ino() != current.ino()
        || identity.len() != retained.len()
        || identity.mtime() != retained.mtime()
        || identity.mtime_nsec() != retained.mtime_nsec()
        || identity.ctime() != retained.ctime()
        || identity.ctime_nsec() != retained.ctime_nsec()
    {
        return Err(unreadable(io::Error::new(
            io::ErrorKind::InvalidData,
            "run report identity changed during read",
        )));
    }
    check_deadline()?;
    Ok(Some(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kani::test_support::discover_scratch;

    /// Trace: FR-034-AC-10, FR-034-AC-12, FR-034-AC-20
    #[test]
    fn completed_report_retention_refuses_symlink_and_expired_original_deadline() {
        let directory = discover_scratch("guardian-report-retention");
        let other = directory.join("other-run.json");
        let assigned = directory.join("assigned-run.json");
        fs::write(&other, b"other run report").unwrap();
        std::os::unix::fs::symlink(&other, &assigned).unwrap();
        assert!(matches!(
            read_report(&assigned, None),
            Err(KaniReportRefusal::Unreadable { .. })
        ));
        assert_eq!(fs::read(&other).unwrap(), b"other run report");
        fs::remove_file(&assigned).unwrap();
        fs::write(&assigned, b"actual completed report").unwrap();
        assert!(matches!(
            read_report(&assigned, Some(Instant::now())),
            Err(KaniReportRefusal::Unreadable { .. })
        ));
        assert_eq!(
            read_report(&assigned, None).unwrap().unwrap(),
            b"actual completed report"
        );
        fs::remove_dir_all(directory).unwrap();
    }

    /// The exported report is read bounded: a file over the limit is refused, and a report that
    /// was never written is `None`.
    ///
    /// Trace: TC-027
    #[test]
    fn tc_027_the_report_is_read_bounded_and_refused_not_truncated() {
        let limit = usize::try_from(REPORT_LIMIT).unwrap();
        let directory = discover_scratch("report-bound");
        let path = directory.join("report.json");
        assert_eq!(read_report(&path, None), Ok(None));
        fs::write(&path, vec![b' '; limit]).unwrap();
        assert_eq!(
            read_report(&path, None).map(|r| r.map(|b| b.len())),
            Ok(Some(limit))
        );
        fs::write(&path, vec![b' '; limit + 1]).unwrap();
        assert_eq!(
            read_report(&path, None),
            Err(KaniReportRefusal::TooLarge { limit })
        );
        remove_stale_report(&path).unwrap();
        assert_eq!(read_report(&path, None), Ok(None));
        let _ = fs::remove_dir_all(directory);
    }
}
