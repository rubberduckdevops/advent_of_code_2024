use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn extract_report_data(string: &String) -> Vec<i32> {
    string
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut report_data: Vec<Vec<i32>> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.flatten() {
            report_data.push(extract_report_data(&line));
        }
    }

    println!("Hello, world!");
}
