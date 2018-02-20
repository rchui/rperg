// help.rs
use std::process;

pub fn help_check(args: &Vec<String>) {
    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        println!("\n{}\n",
        "
RPERG - Rust Parallel grep by Ryan Chui (2018)

  rperg is a custom multithreaded rust implmentation of grep to search extremely large files, datasets, and
  directories original conceptualized at the National Center for Supercomputing Applications and implemented
  at 23andMe.

  Usage:

    rperg [-A <#>|-f <file>|-r|-v|-V|-w] <search term>

  Modes:

    -A    After Context         rperg will grab a number of lines after the line containing the <search term>.
                                If this option is set too high, repeated sequences can appear in the output.
                                This does not work with search inversion.
    -f    Single File Search    Signals rperg to only search the <file> for the <search term>. If -f is not
                                used, rperg will search the entire directory where rperg is called from.
    -i    Include Hidden        Include hidden files in the search. Default search behavior is to ignore hidden
                                files.
    -r    Recursive Search      Recursively searches through the directory and all sub-directories for the given
                                <search term>. Will not do anything if the [-f <file>] flag is given.
    -v    Search Inversion      Search for every line that does not include the <search term>.
    -V    Enable Verbose        The file path toto the file containing <search term> will be printed along with
                                the search result.
    -w    File Parallelism      rperg will perform single-threades searches of multiple files. Default search
                                behavior is to search files one at a time with multiple threads. This is optimal
                                the files are small, similar size, or there are many of them.");
    process::exit(0);
    }
}