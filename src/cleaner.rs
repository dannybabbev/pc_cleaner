use std::io;
use std::path::Path;

use crate::scanner::ScanResult;

/// Move selected directories to trash
pub fn run_cleaner(scan_result: &ScanResult) -> io::Result<u64> {
    let mut deleted_count = 0;

    for path in scan_result.paths() {
        move_dir_to_trash(path)?;
        deleted_count += 1;
    }

    Ok(deleted_count)
}

fn move_dir_to_trash(path: &Path) -> io::Result<()> {
    println!("Moving to trash {:?}...", path);
    trash::delete(path).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    Ok(())
}
