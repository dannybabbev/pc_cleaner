mod cleaner;
mod defaults;
mod scanner;
use chrono::{Duration, Local};
use dirs;
mod prompts;
use std::path::PathBuf;

use crate::prompts::{new_section, prompt_selection, y_or_exit};

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

    new_section();
    println!("Scanning: {:?}", scan_path);
    println!("Dirs to match: ({:?}): ", matchers);

    y_or_exit("Proceed scan? (Y/n)", true).unwrap();

    let res = scanner::perform_scan(&scan_path, &matchers).unwrap();
    println!("{}", res);

    new_section();

    // Time filter options: no filter, 1 month, 3 months, 6 months
    let time_options: &[(Option<i64>, &str)] = &[
        (None, "No filter - all results"),
        (Some(1), "1 month ago"),
        (Some(3), "3 months ago"),
        (Some(6), "6 months ago"),
    ];

    let months_back = prompt_selection("Filter by last visited", time_options).unwrap();

    let filtered_res = if let Some(months) = months_back {
        let now = Local::now();
        let cutoff = now - Duration::days(months * 30);
        let cutoff_system_time = cutoff
            .naive_local()
            .and_local_timezone(Local)
            .unwrap()
            .into();

        new_section();
        println!(
            "Filtering paths not accessed since {}...",
            cutoff.format("%Y-%m-%d")
        );

        let filtered = res.filter_by_last_accessed(cutoff_system_time);
        println!("{}", filtered);
        filtered
    } else {
        new_section();
        println!("No filter applied - showing all results");
        res
    };

    new_section();
    y_or_exit("Delete directories? (y/N)", false).unwrap();

    let deleted = cleaner::delete_directories(&filtered_res).unwrap();
    println!("Cleanup finished! Deleted {} directories.", deleted);
    println!("Saved {}", filtered_res.formatted_sum_byte_size());
}
