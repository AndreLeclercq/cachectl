mod fs;

fn main() {
    println!("Hello, world!");
    match fs::list_cache_files() {
        Ok(files) => {
            println!("{:?}", files)
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}
