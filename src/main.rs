use std::fs::File;
use std::io::{Cursor, Read, Seek};
use std::path::PathBuf;

use anyhow::{anyhow, Error};
use structopt::StructOpt;

trait ReadSeek: Read + Seek {}
impl ReadSeek for Cursor<Vec<u8>> {}
impl ReadSeek for File {}

#[derive(Debug, StructOpt)]
#[structopt(about = "Extract embedded files from documents")]
struct Opt {
    #[structopt(short, long)]
    target_dir: Option<PathBuf>,

    #[structopt(help = "Path to an office file", parse(from_os_str))]
    file: PathBuf,
}

fn main() -> Result<(), Error> {
    env_logger::init();

    let opt = Opt::from_args();

    let target_dir = opt
        .target_dir
        .unwrap_or_else(|| std::env::current_dir().expect("Could not get the current directory"));

    if !target_dir.exists() {
        return Err(anyhow!("Target directory {:?} does not exist", target_dir));
    }

    match unembedx::extract_embedded_files(&opt.file, &target_dir)? {
        0 => println!("No embedded files found"),
        1 => println!("\nCreated one file"),
        count => println!("\nCreated {} files", count),
    }

    Ok(())
}
