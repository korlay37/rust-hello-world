use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
    // #[arg(short = 'o', long = "output")]
    // output: std::path::PathBuf
}

fn main() {
    let args = Cli::parse();
    // let content =  std::fs::read_to_string(&args.path).expect("could not read file");
    let file = File::open(&args.path).expect("could not open file");
    let reader = BufReader::new(file);

    

    for line in reader.lines() {
        let line = line.expect("could not read line");
        if line.contains(&args.pattern){
            println!("{}", line);
        }
    }
}
