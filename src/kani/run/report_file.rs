//! The report file Kani exports for one run: its launch-unique path, the removal of a stale file
//! under that name and the bounded read (FR-017, L-7).

use std::{
    fs, io,
    io::Read,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
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
const REPORT_LIMIT: usize = 16 * 1024 * 1024;

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
pub(super) fn read_report(path: &Path) -> Result<Option<Vec<u8>>, KaniReportRefusal> {
    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(KaniReportRefusal::Unreadable {
                detail: error.to_string(),
            })
        }
    };
    let mut bytes = Vec::new();
    file.take(
        u64::try_from(REPORT_LIMIT)
            .unwrap_or(u64::MAX)
            .saturating_add(1),
    )
    .read_to_end(&mut bytes)
    .map_err(|error| KaniReportRefusal::Unreadable {
        detail: error.to_string(),
    })?;
    if bytes.len() > REPORT_LIMIT {
        return Err(KaniReportRefusal::TooLarge {
            limit: REPORT_LIMIT,
        });
    }
    Ok(Some(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kani::test_support::discover_scratch;

    /// The exported report is read bounded: a file over the limit is refused, and a report that
    /// was never written is `None`.
    ///
    /// Trace: FR-017-AC-19, TC-027
    #[test]
    fn tc_027_the_report_is_read_bounded_and_refused_not_truncated() {
        let directory = discover_scratch("report-bound");
        let path = directory.join("report.json");
        assert_eq!(read_report(&path), Ok(None));
        fs::write(&path, vec![b' '; REPORT_LIMIT]).unwrap();
        assert_eq!(
            read_report(&path).map(|r| r.map(|b| b.len())),
            Ok(Some(REPORT_LIMIT))
        );
        fs::write(&path, vec![b' '; REPORT_LIMIT + 1]).unwrap();
        assert_eq!(
            read_report(&path),
            Err(KaniReportRefusal::TooLarge {
                limit: REPORT_LIMIT
            })
        );
        remove_stale_report(&path).unwrap();
        assert_eq!(read_report(&path), Ok(None));
        let _ = fs::remove_dir_all(directory);
    }
}
