use core::fmt;
use std::io;
use std::fs::{self, read_dir};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use fmt::Display;
use chrono::{DateTime, Local};
use humansize::{format_size, DECIMAL};


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
        write!(f, "{:?} (last modified: {}; last visited: {}) {}", self.path, m.format(format), a.format(format), format_size(self.byte_size, DECIMAL))
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

/// TODO: Match target/ as well
/// walk directories until a match is found
fn visit_dirs_recursive(dir: &Path, path_matcher: &str, matching_paths: &mut Vec<MatchingPath>) -> io::Result<()> {
    // Skip symlinks entirely
    let metadata = fs::symlink_metadata(dir)?;
    if metadata.file_type().is_symlink() {
        return Ok(());
    }

    if dir.is_dir() {
        if dir.ends_with(path_matcher) {
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

        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs_recursive(&path, path_matcher, matching_paths)?;
            }
        }
    }
    Ok(())
}

pub fn visit_dirs(dir: &Path, path_matcher: &str) -> io::Result<Vec<MatchingPath>> {
    let mut matching_paths: Vec<MatchingPath> = Vec::new();

    visit_dirs_recursive(dir, path_matcher, &mut matching_paths)?;

    Ok(matching_paths)
}
