use structopt::StructOpt;
use std::env;
use std::fs;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    let args = Cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path),
    };

    let content = fs::read_to_string("operating-social-system.txt")
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", content);
}
