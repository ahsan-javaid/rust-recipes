use std::fs;

fn main() {
    // Replace "/path/to/directory" with the directory path you want to list
    let directory_path = "/path/to/directory";

    match fs::read_dir(directory_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    // Get the file/directory name as a String
                    if let Some(file_name) = entry.file_name().to_str() {
                        println!("{}", file_name);
                    } else {
                        eprintln!("Error converting file name to string.");
                    }
                } else {
                    eprintln!("Error reading directory entry.");
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
