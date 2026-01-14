use chrono::{DateTime, Local};
use core::fmt;
use fmt::Display;
use humansize::{DECIMAL, format_size};
use std::fs::{self, read_dir};
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub struct MatchingPath {
    path: PathBuf,
    last_modified: SystemTime,
    last_accessed: SystemTime,
    byte_size: u64,
}

impl Display for MatchingPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let format = "%Y-%m-%d";
        let m: DateTime<Local> = self.last_modified.into();
        let a: DateTime<Local> = self.last_accessed.into();
        write!(
            f,
            "{:?} (last modified: {}; last visited: {}) {}",
            self.path,
            m.format(format),
            a.format(format),
            format_size(self.byte_size, DECIMAL)
        )
    }
}

pub struct ScanResult {
    pub matched_paths: Vec<MatchingPath>,
    pub sum_byte_size: u64,
}

impl Display for ScanResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} Paths; {} Total",
            self.matched_paths.len(),
            format_size(self.sum_byte_size, DECIMAL)
        )
    }
}

impl ScanResult {
    pub fn formatted_sum_byte_size(&self) -> String {
        format_size(self.sum_byte_size, DECIMAL)
    }

    /// Get the paths from the scan result
    pub fn paths(&self) -> impl Iterator<Item = &Path> {
        self.matched_paths.iter().map(|p| p.path.as_path())
    }

    /// Filter paths by last accessed time, keeping only those accessed before the cutoff
    pub fn filter_by_last_accessed(&self, cutoff: SystemTime) -> ScanResult {
        let filtered: Vec<_> = self
            .matched_paths
            .iter()
            .filter(|p| p.last_accessed < cutoff)
            .collect();

        let total_size: u64 = filtered.iter().map(|p| p.byte_size).sum();

        // Print filtered paths
        for p in &filtered {
            println!("{}", p);
        }

        ScanResult {
            matched_paths: filtered
                .into_iter()
                .map(|p| MatchingPath {
                    path: p.path.clone(),
                    last_modified: p.last_modified,
                    last_accessed: p.last_accessed,
                    byte_size: p.byte_size,
                })
                .collect(),
            sum_byte_size: total_size,
        }
    }
}

/// Source: https://github.com/webdesus/fs_extra/blob/1754296075e7cc4a25feaa876a3f4b9daccc0b98/src/dir.rs#L762C1-L817C1
/// Returns the size of the file or directory in bytes.(!important: folders size not count)
///
/// If used on a directory, this function will recursively iterate over every file and every
/// directory inside the directory. This can be very time consuming if used on large directories.
///
/// Does not follow symlinks.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just
/// these cases:
///
/// * This `path` directory does not exist.
/// * Invalid `path`.
/// * The current process does not have the permission to access `path`.
///
/// # Examples
/// ```rust,ignore
/// extern crate fs_extra;
/// use fs_extra::dir::get_size;
///
/// let folder_size = get_size("dir")?;
/// println!("{}", folder_size); // print directory size in bytes
/// ```
pub fn get_size<P>(path: P) -> io::Result<u64>
where
    P: AsRef<Path>,
{
    // Using `fs::symlink_metadata` since we don't want to follow symlinks,
    // as we're calculating the exact size of the requested path itself.
    let path_metadata = path.as_ref().symlink_metadata()?;

    let mut size_in_bytes = 0;

    if path_metadata.is_dir() {
        for entry in read_dir(&path)? {
            let entry = entry?;
            // `DirEntry::metadata` does not follow symlinks (unlike `fs::metadata`), so in the
            // case of symlinks, this is the size of the symlink itself, not its target.
            let entry_metadata = entry.metadata()?;

            if entry_metadata.is_dir() {
                // The size of the directory entry itself will be counted inside the `get_size()` call,
                // so we intentionally don't also add `entry_metadata.len()` to the total here.
                size_in_bytes += get_size(entry.path())?;
            } else {
                size_in_bytes += entry_metadata.len();
            }
        }
    } else {
        size_in_bytes = path_metadata.len();
    }

    Ok(size_in_bytes)
}

/// walk directories until a match is found, return from the branch
fn visit_dirs_recursive(
    dir: &Path,
    path_matcher: &[&str],
    matching_paths: &mut Vec<MatchingPath>,
) -> io::Result<()> {
    // Skip symlinks entirely
    let metadata = fs::symlink_metadata(dir)?;
    if metadata.file_type().is_symlink() {
        return Ok(());
    }

    if dir.is_dir() {
        if path_matcher.iter().any(|i| dir.ends_with(i)) {
            let meta = fs::metadata(dir)?;
            let modified = meta.modified()?;
            let accessed = meta.accessed()?;
            let size = get_size(dir)?;

            let p = MatchingPath {
                path: dir.to_path_buf(),
                last_modified: modified,
                last_accessed: accessed,
                byte_size: size,
            };

            println!("{}", p);

            matching_paths.push(p);

            return Ok(());
        }

        let entries = match fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                println!("Skipping {:?}; Reason (Perimssion Denied)", dir);
                return Ok(());
            }
            Err(e) => return Err(e),
        };

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs_recursive(&path, path_matcher, matching_paths)?;
            }
        }
    }
    Ok(())
}

/// Visit all dirs under `dir` and match with `path_matcher`
fn visit_dirs(dir: &Path, path_matcher: &[&str]) -> io::Result<Vec<MatchingPath>> {
    let mut matching_paths: Vec<MatchingPath> = Vec::new();

    visit_dirs_recursive(dir, path_matcher, &mut matching_paths)?;

    Ok(matching_paths)
}

/// Call this function to scan your system
pub fn perform_scan(dir: &Path, path_matcher: &[&str]) -> io::Result<ScanResult> {
    let paths = visit_dirs(dir, path_matcher)?;

    let total_size: u64 = paths.iter().map(|i| i.byte_size).sum();

    let res = ScanResult {
        matched_paths: paths,
        sum_byte_size: total_size,
    };

    Ok(res)
}
