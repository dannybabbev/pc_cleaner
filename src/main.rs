mod scanner;
use std::path::Path;

fn main() {
    println!("Welcome to Mac Cleaner!");

    let start_path = Path::new("/Users/daniel/");
    let matchers = vec![String::from("node_modules"), String::from("target")];

    let res = scanner::perform_scan(start_path, &matchers).unwrap();
    println!("{}", res);
}
