#![allow(unused)]

use std::io::{Result, Error};
use std::path::{Path, PathBuf};
use std::{fs, env};

// Get the cache path.
pub fn get_cache_path() -> Result<PathBuf> {
    let mut cache_path = env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or(Error::new(std::io::ErrorKind::NotFound, "Environment variable 'HOME' not found"))?;
    cache_path.push(".cache");
    Ok(cache_path) 
}

// List files in directory.
pub fn list_directory(path: &Path) -> Result<Vec<PathBuf>> {
    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>>>()?;
    entries.sort();
    Ok(entries)  
}

// Get file size in bytes.
pub fn get_file_size(path: &Path) -> Result<u64> {
    let metadata = fs::metadata(path)?;

    Ok(metadata.len())
}

// Remove file.
pub fn remove_file(_path: &Path) -> Result<()> {

    Ok(())
}
