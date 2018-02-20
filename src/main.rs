// main.rs

use std::env;
mod help;
mod utils;

#[derive(Debug)]
struct Settings {
    recursive: bool,
    invert: bool,
    verbose: bool,
    is_file: bool,
    file_wise: bool,
    check_hidden: bool,
    extra: bool,
    num_extra: u32,
    file: String,
    term: String
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            recursive: false,
            invert: false,
            verbose: false,
            is_file: false,
            file_wise: false,
            check_hidden: false,
            extra: false,
            num_extra: 0,
            file: String::from(""),
            term: String::from("")
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let settings = Settings::default();
    let mut file_paths: Vec<String> = Vec::new();
    help::help_check(&args);
    let cwd = utils::get_cwd();
    println!("{:?}", cwd);
    println!("{:?}", args);
    println!("{:?}", file_paths);
    println!("{:#?}", settings);
}
