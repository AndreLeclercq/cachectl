use std::path::Path;

mod fs;

fn main() {
    println!("Hello, world!");
    /*
    match fs::list_cache_files() {
        Ok(files) => {
            println!("{:?}", files)
        },
        Err(e) => eprintln!("Error: {}", e),
    }
    */
    let test_path = Path::new("/home/andre/.cache/nvim");
    match fs::get_file_size(test_path) {
        Ok(size) => {
            println!("{:?} bytes", size)
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}
