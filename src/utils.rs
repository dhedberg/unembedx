use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;

use anyhow::{anyhow, Error};
use lazy_static::lazy_static;

pub fn get_next_filename(base: &str) -> String {
    lazy_static! {
        static ref SEQUENCES: Mutex<HashMap<String, usize>> = Mutex::new(HashMap::new());
    }

    let mut sequences = SEQUENCES.lock().unwrap();

    let sequence = sequences
        .entry(base.to_string())
        .and_modify(|v| *v += 1)
        .or_insert(0);

    format!("{}_{}", base, sequence)
}

#[cfg(feature = "filetypes")]
pub fn guess_extension_from_contents(path: &Path) -> Result<Option<String>, Error> {
    let cookie = magic::Cookie::open(magic::cookie::Flags::MIME_TYPE)?;

    let database = Default::default();
    let cookie = cookie.load(&database)
        .map_err(|e| anyhow!("Failed to load magic database: {:?}", e))?;

    let guessed_extension = cookie
        .file(path)
        .map(|mime_type| new_mime_guess::get_mime_extensions_str(&mime_type))?
        .and_then(|extension_list| extension_list.first())
        .map(|s| s.to_string());

    Ok(guessed_extension)
}

#[cfg(not(feature = "filetypes"))]
pub fn guess_extension_from_contents(_: &Path) -> Result<Option<String>, Error> {
    Ok(None)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_filename() {
        assert_eq!("test_0", get_next_filename("test"));
        assert_eq!("test_1", get_next_filename("test"));
        assert_eq!("abc_0", get_next_filename("abc"));
        assert_eq!("test_2", get_next_filename("test"));
        assert_eq!("abc_1", get_next_filename("abc"));
    }

    #[test]
    #[cfg(feature = "filetypes")]
    fn test_guess_extensions_from_contents() {
        let blank_pdf = std::env::current_dir().unwrap().join("testdata/blank.pdf");
        let guessed_extension = guess_extension_from_contents(&blank_pdf).unwrap();

        assert_eq!(Some("pdf".to_string()), guessed_extension);
    }

    #[test]
    #[cfg(not(feature = "filetypes"))]
    fn test_guess_extensions_from_contents_dummy() {
        let blank_pdf = std::env::current_dir().unwrap().join("testdata/blank.pdf");
        let guessed_extension = guess_extension_from_contents(&blank_pdf).unwrap();

        assert_eq!(None, guessed_extension);
    }
}
