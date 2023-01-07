use std::fs::File;
use std::io::{Cursor, Read, Seek};
use std::path::Path;

use anyhow::{Context, Error};
use zip::read::{ZipArchive, ZipFile};

mod cfb;
mod utils;

const MAX_IN_MEMORY_SIZE: usize = 50 * 1024 * 1024;

trait ReadSeek: Read + Seek {}
impl ReadSeek for Cursor<Vec<u8>> {}
impl ReadSeek for File {}

pub fn extract_embedded_files<T: AsRef<Path>, S: AsRef<Path>>(
    source: T,
    target_dir: S,
) -> Result<usize, Error> {
    let source = source.as_ref();

    let mut archive = File::open(source)
        .map_err(|e| e.into())
        .and_then(ZipArchive::new)
        .with_context(|| format!("Could not open {:?}", source))?;

    println!("Looking for embedded files in {:?}\n", source);

    let mut files_count = 0;
    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .with_context(|| "Error when reading zip archive".to_string())?;


        if !file.is_file() || !file.name().contains("/embeddings/") {
            continue;
        }

        let result = get_seekable_stream(&mut file)
            .and_then(|stream| cfb::extract_from_stream(stream, &target_dir));

        match result {
            Ok(count) => files_count += count,
            Err(e) => eprintln!("Failure during extraction from {}: {:?}\n", file.name(), e),
        }
    }

    Ok(files_count)
}

fn get_seekable_stream(file: &mut ZipFile) -> Result<Box<dyn ReadSeek>, Error> {
    if file.size() <= MAX_IN_MEMORY_SIZE as u64 {
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(Box::new(Cursor::new(buf)))
    } else {
        let mut tmp =
            tempfile::tempfile().with_context(|| "Failed to create temporary file".to_string())?;

        std::io::copy(file, &mut tmp)
            .with_context(|| "Failed to write temporary file".to_string())?;

        Ok(Box::new(tmp))
    }
}
