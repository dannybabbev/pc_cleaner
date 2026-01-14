mod defaults;
mod scanner;
use dirs;
mod prompts;
use std::path::PathBuf;

use crate::prompts::y_or_exit;

fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(&path[2..]);
        }
    } else if path == "~" {
        if let Some(home) = dirs::home_dir() {
            return home;
        }
    }
    PathBuf::from(path)
}

fn main() {
    println!("Welcome to PC Cleaner!");

    let home_dir = dirs::home_dir().unwrap();

    // TODO: Prompt for custom matchers
    let matchers = defaults::DEFAULT_MATCHERS;

    let scan_path = prompts::prompt_input(
        format!("Scan directory ({:?})", home_dir.display()),
        home_dir,
        |s| Ok::<PathBuf, std::convert::Infallible>(expand_tilde(&s)),
    )
    .unwrap();

    println!("==========================================");
    println!("Scanning: {:?}", scan_path);
    println!("Dirs to match: ({:?}): ", matchers);

    y_or_exit("Proceed scan? (Y/n)", true).unwrap();

    let res = scanner::perform_scan(&scan_path, &matchers).unwrap();
    println!("{}", res);
}
