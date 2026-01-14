mod defaults;
mod scanner;
use dirs;
mod prompts;
use std::{path::PathBuf, str::FromStr};

use crate::prompts::y_or_exit;

fn main() {
    println!("Welcome to PC Cleaner!");

    let home_dir = dirs::home_dir().unwrap();
    let matchers = defaults::DEFAULT_MATCHERS;

    let scan_path = prompts::prompt_input(
        format!("Scan directory ({:?})", home_dir.display()),
        home_dir,
        |s| PathBuf::from_str(&s),
    )
    .unwrap();

    println!("Scanning: {:?}", scan_path);
    println!("Dirs to match: ({:?}): ", matchers);

    y_or_exit("Proceed scan?", true).unwrap();

    let res = scanner::perform_scan(&scan_path, &matchers).unwrap();
    println!("{}", res);
}
