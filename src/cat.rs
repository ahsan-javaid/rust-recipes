use std::{env, io};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()> {

    let filenames: Vec<String> = env::args().skip(1).collect();

    if filenames.is_empty() {
        println!("Usage cat <file> <file>...");
        return Ok(())
    }

    for file in filenames {
        match File::open(&file) {
            Ok(file) => {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    println!("{}", line?);
                }
            }
            Err(e) => {
                eprintln!("Error reading {}: {}", file, e);
            }
        }
    }

    Ok(())
}
