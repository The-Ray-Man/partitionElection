use std::{
    collections::BTreeSet,
    fs::{File, OpenOptions},
    io::Write,
};

use crate::structures::Ranking;

/// Reads the content of a file and returns it as a vector of strings.
/// If the file does not exist, an error message is returned.
pub fn read_from_file(path: &str) -> Result<Vec<String>, String> {
    use std::io::{BufRead, BufReader};
    let file = File::open(path);
    if let Ok(file) = file {
        let reader = BufReader::new(file);

        let mut result = Vec::new();
        for line in reader.lines() {
            match line {
                Ok(line) => result.push(line),
                Err(e) => return Err(e.to_string()),
            }
        }
        Ok(result)
    } else {
        Err(format!("File '{}' not found", path).to_string())
    }
}

/// Writes a ranking to a file.
/// If the file does not exist, it is created and overwritten.
/// Can panic.
pub fn write_rankings_to_file(rankings: BTreeSet<Ranking>, filename: &str) {
    let path = format!("logs/rankings/{}", filename);
    // Check if the file exists
    if !std::path::Path::new(&path).exists() {
        println!("File does not exist. Creating and writing to it.");

        // Create the file and write content
        let mut file: File = OpenOptions::new()
            .create(true) // Create the file if it doesn't exist
            .truncate(true)
            .write(true) // Open for writing
            .open(&path)
            .unwrap_or_else(|err| {
                eprintln!("Error: {:#?}", err);
                eprintln!("Failed to create a file {} with write access.", path);
                std::process::exit(1)
            });

        for ranking in &rankings {
            file.write_all(ranking.to_string().as_bytes())
                .unwrap_or_else(|err| {
                    eprintln!("Error: {:#?}", err);
                    eprintln!("Failed to write to file {}", path);
                    std::process::exit(1)
                });
            file.write_all(b"\n").unwrap_or_else(|err| {
                eprintln!("Error: {:#?}", err);
                eprintln!("Failed to write to file {}", path);
                std::process::exit(1)
            });
        }
    } else {
        println!("File already exists.");
    }
}
