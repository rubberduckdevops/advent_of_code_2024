use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut program_memory: Vec<Vec<String>> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.flatten() {
            program_memory.push(find_mul_patterns(&line));
        }
    }
    let mut total_value: u32 = 0;
    for chunk in program_memory {

        for (i, m) in chunk.iter().enumerate() {
            println!("{}: {}", i, m);
            let value = execute_mem_function(m);
            total_value += execute_mem_function(m).unwrap();
        }
    }
    println!("{}", total_value)

}

fn execute_mem_function(text: &str) -> Option<u32> {
    let content = text.strip_prefix("mul(")?.strip_suffix(")")?;
    let parts: Vec<&str> = content.split(',').collect();
    if parts.len() != 2 {
        return None;
    }
    let first = parts[0].parse::<u32>().ok()?;
    let second = parts[1].parse::<u32>().ok()?;
    let evaluation = first * second;
    Some(evaluation)

}

fn find_mul_patterns(text: &str) -> Vec<String> {
    // Create a regex pattern that matches mul(x,y) where x and y are 1-3 digits
    // Use capture groups to then further parse down to allow for multiply
    // let pattern = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // Collect all matches into a vector
    pattern.find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect()
}
