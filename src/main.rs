use std::env;
use std::fs;
use std::path::Path;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: renamer <file_path> <new_name>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let new_name = &args[2];

    if let Err(err) = rename_file(file_path, new_name) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    println!("File renamed successfully.");
}

fn rename_file(file_path: &str, new_name: &str) -> io::Result<()> {
    let path = Path::new(file_path);

    if !path.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Specified path is not a file.",
        ));
    }

    let parent_dir = path.parent().unwrap(); // Get the parent directory
    let new_path = parent_dir.join(new_name); // Create the new path

    fs::rename(path, new_path)
}