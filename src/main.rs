mod scanner;
use std::path::Path;


fn main() {
    println!("Welcome to Mac Cleaner!");

    let start_path = Path::new("/Users/daniel/dev");
    let res = scanner::perform_scan(start_path, "node_modules").unwrap();
    println!("{}", res);
}
