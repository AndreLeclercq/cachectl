#![allow(unused)]

use std::io::{Result, Error};
use std::path::{Path, PathBuf};
use std::{fs, env};

/// Finds the Linux system's cache location (XDG or ~/.cache) to let users manage their temporary files.
///
/// # Returns
/// - `Ok(PathBuf)` - Cache path found according to XDG spec
/// - `Err` - Critical for program operation, means no writable cache exists
///
/// # Examples
/// ```
/// let cache_path = get_cache_path()?;
/// println!("Cache directory: {}", cache_path.display());
/// ```
pub fn get_cache_path() -> Result<PathBuf> {
    if let Some(cache_dir) = env::var_os("XDG_CACHE_HOME").map(PathBuf::from) {
        return Ok(cache_dir)
    }
    let mut cache_path = env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or(Error::new(std::io::ErrorKind::NotFound, "Neither 'XDG_CACHE_HOME or 'HOME' found"))?;
    cache_path.push(".cache");
    Ok(cache_path) 
}

/// Lists directory contents to help users browse and select cache files to manage.
///
/// # Returns
/// - `Ok(Vec<PathBuf>)` - Sorted list of directory entries
/// - `Err` - Directory inaccessible or unreadable
///
/// # Examples
/// ```
/// let entries = list_directory(&cache_path)?;
/// for entry in entries {
///     println!("{}", entry.display());
/// }
/// ```
pub fn list_directory(path: &Path) -> Result<Vec<PathBuf>> {
    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>>>()?;
    entries.sort();
    Ok(entries)  
}

/// Measures file sizes to support cleanup decisions and display disk usage in UI.
///
/// # Returns
/// - `Ok(u64)` - File size in bytes
/// - `Err` - File inaccessible or unreadable
///
/// # Examples
/// ```
/// let size = get_file_size(&file_path)?;
/// println!("Size: {} bytes", size);
/// ```
pub fn get_file_size(path: &Path) -> Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

/// Removes cache files and directories to free up disk space and maintain system health.
///
/// # Returns
/// - `Ok(())` - Successfully deleted
/// - `Err` - Failed to delete path
///
/// # Examples
/// ```
/// delete(&cache_path)?;
/// ```
pub fn delete(path: &Path) -> Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}
