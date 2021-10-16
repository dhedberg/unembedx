use std::io::{Read, Seek};
use std::os::unix::prelude::OsStrExt;
use std::path::Path;

use anyhow::{Context, Error};

use crate::utils;

pub fn extract_from_stream<T: Read + Seek, S: AsRef<Path>>(
    stream: T,
    target_dir: S,
) -> Result<usize, Error> {
    let mut cfb = cfb::CompoundFile::open(stream)?;

    let mut extracted_count = 0;
    let entries: Vec<cfb::Entry> = cfb.read_storage("")?.collect();
    for entry in entries {
        if !entry.is_stream() {
            continue;
        }

        let is_stream_of_interest = entry.path().as_os_str().as_bytes().iter().all(|&b| b > 10);

        if is_stream_of_interest {
            write_stream_to_file(&mut cfb, &target_dir, entry)?;
            extracted_count += 1;
        }
    }

    Ok(extracted_count)
}

fn write_stream_to_file<T: Read + Seek, S: AsRef<Path>>(
    cfb: &mut cfb::CompoundFile<T>,
    target_dir: S,
    entry: cfb::Entry,
) -> Result<(), Error> {
    let target_dir = target_dir.as_ref();

    let mut source_stream = cfb
        .open_stream(entry.path())
        .with_context(|| "Failed to open cfb stream".to_string())?;

    let mut tmp_file = tempfile::NamedTempFile::new_in(target_dir)
        .with_context(|| format!("Failed to create target file in {:?}", target_dir))?;

    std::io::copy(&mut source_stream, &mut tmp_file)
        .with_context(|| "Failed to write cfb stream to temporary file".to_string())?;

    let mut filename = utils::get_next_filename("embedded_file");
    if let Ok(Some(extension)) = utils::guess_extension_from_contents(tmp_file.path()) {
        filename.push('.');
        filename.push_str(&extension);
    }

    let final_path = target_dir.join(filename);
    tmp_file
        .persist_noclobber(&final_path)
        .with_context(|| format!("Failed to rename temporary file to {:?}", final_path))?;

    println!("Wrote file {:?}", final_path);

    Ok(())
}
