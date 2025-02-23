#![allow(unused)]

use std::io::{Result, Error};
use std::path::{Path, PathBuf};
use std::{fs, env};

// List cache files.
pub fn list_cache_files() -> Result<Vec<PathBuf>> {
    let mut cache_path = env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or(Error::new(std::io::ErrorKind::NotFound, "HOME not found"))?;
    cache_path.push(".cache");

    let mut entries = fs::read_dir(cache_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>>>()?;
    entries.sort();

    Ok(entries)  
}

// Get file size.
pub fn get_file_size(_path: &Path) -> Result<u64> {

    Ok(42)
}

// Go to directory.
pub fn go_to(_path: &Path) -> Result<()> {

    Ok(())
}

// Remove file.
pub fn remove_file(_path: &Path) -> Result<()> {

    Ok(())
}
