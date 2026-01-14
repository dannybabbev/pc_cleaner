mod scanner;
use std::path::Path;


fn main() {
    println!("Hello, world!");

    let start_path = Path::new("/Users/daniel/dev");
    let matched_paths = scanner::visit_dirs(start_path, "node_modules").unwrap();
}
