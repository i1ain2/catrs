/// Cat clone made by Rust

use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::{Context, Result};

#[derive(StructOpt, Debug)]
struct Opt {
    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() -> Result<()>{
    let opt = Opt::from_args();
    // println!("{:#?}", opt);
    for path in &opt.files {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("could not read file `{}`", path.display()))?;
        println!("{}", content);
    }
    Ok(())
}
