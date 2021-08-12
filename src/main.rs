/// Cat clone made by Rust

use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() -> io::Result<()>{
    let opt = Opt::from_args();
    for path in &opt.files {
        let f = File::open(path)?;
        let f = BufReader::new(f);
        for line in f.lines() {
            println!("{}", line.unwrap());
        }
    }
    Ok(())
}
