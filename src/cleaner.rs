use std::fs;
use std::io;
use std::path::Path;

use crate::scanner::ScanResult;

/// Delete all directories from the scan result
pub fn delete_directories(scan_result: &ScanResult) -> io::Result<u64> {
    let mut deleted_count = 0;

    for path in scan_result.paths() {
        delete_directory(path)?;
        deleted_count += 1;
    }

    Ok(deleted_count)
}

fn delete_directory(path: &Path) -> io::Result<()> {
    println!("Deleting {:?}...", path);
    fs::remove_dir_all(path)?;
    Ok(())
}
