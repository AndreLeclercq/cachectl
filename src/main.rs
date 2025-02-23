mod fs;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let cache_path = fs::get_cache_path()?; 
    let entries = fs::list_directory(&cache_path)?;
    for entry in entries {
        let size = fs::get_file_size(&entry)?;
        println!("{:?} - {} bytes", entry, size);
    }
    Ok(())
}
