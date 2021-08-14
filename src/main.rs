/// Cat clone made by Rust

use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;
use std::iter::Iterator;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// number nonempty output lines
    #[structopt(short = "b", long)]
    number_nonblank: bool,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() -> io::Result<()>{
    let opt = Opt::from_args();

    let mut lines = vec![];
    for path in opt.files {
        let f = File::open(path)?;
        let f = BufReader::new(f);
        for line in f.lines() {
            lines.push(line.unwrap());
        }
    }

    let mut i = 1;
    let mut output = vec![];
    for line in lines {
        if opt.number_nonblank {
            if line.trim().is_empty() {
                output.push(line);
                continue;
            }
            output.push(format!("{0: >6} {1}", i, line));
            i += 1;
        } else {
            // no option
            output.push(line);
        }
    }
    println!("{}", output.join("\n"));

    Ok(())
}
