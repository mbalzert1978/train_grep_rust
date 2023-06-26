use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn read_lines(path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|line| line.map(|l| l.to_lowercase()))
        .collect::<io::Result<Vec<String>>>()?;
    Ok(lines)
}

fn find_lines<'a>(lines: &'a [String], value: &'a str) -> Vec<&'a str> {
    lines
        .iter()
        .filter(|line| line.contains(value))
        .map(|line| line.as_str())
        .collect()
}

fn show_lines(lines: &[&str]) {
    if lines.is_empty() {
        println!("No match found.");
    } else {
        for line in lines {
            println!("{}", line);
        }
    }
}

fn extract_path_and_value() -> (String, String) {
    let mut args = env::args().skip(1);
    let path = args.next().unwrap_or_else(|| {
        eprintln!("Path argument missing.");
        std::process::exit(0);
    });
    let value = args.next().unwrap_or_else(|| {
        eprintln!("Value argument missing.");
        std::process::exit(0);
    });
    (path, value.to_lowercase())
}

fn main() {
    let (path, value) = extract_path_and_value();
    let path = Path::new(&path);
    if let Ok(lines) = read_lines(&path) {
        let lines = find_lines(&lines, &value);
        show_lines(&lines);
    } else {
        println!("Failed to read lines from file.");
        std::process::exit(0);
    }
}
