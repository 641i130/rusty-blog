use walkdir::WalkDir;
use std::path::Path;
fn main() {
    for file in WalkDir::new("./md").into_iter().filter_map(|file| file.ok()) {
        if (file.metadata().unwrap().is_file() && file.path().extension().unwrap() == "md") {
            // ONLY GET MD files!!!
            println!("{}", file.path().display());
        }
    }
}