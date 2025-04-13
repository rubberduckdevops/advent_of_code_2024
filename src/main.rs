use std::fs::File;
use std::io::{self, Read, BufRead};
use std::path::Path;
use regex::Regex;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> io::Result<()> {
    let mut program_memory: Vec<Vec<String>> = Vec::new();
    let mut part_2_value: u64 = 0;

    let mut file = File::open("./input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    part_2_value = find_all_instructions(&contents);

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.flatten() {
            program_memory.push(find_mul_patterns(&line));
        }
    }


    let mut total_value: u32 = 0;
    for chunk in program_memory {
        for m in chunk.iter() {
            if let Some(value) = execute_mem_function(m) {
                total_value += value;
            }
        }
    }

    println!("Part 1: {}", total_value);
    println!("Part 2: {}", part_2_value);
    Ok(())
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

#[derive(Debug)]
enum Instruction {
    Multiply { position: usize, x: u64, y: u64 },
    Do(usize),
    Dont(usize),
}

fn find_all_instructions(text: &str) -> u64 {
    let mut enabled = true;

    // Create regex patterns
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let enable_pattern = Regex::new(r"do\(\)").unwrap();
    let disable_pattern = Regex::new(r"don\'t\(\)").unwrap();

    // Collect all the instructions
    let mut instructions = Vec::new();

    // Find all mul patterns
    for cap in pattern.captures_iter(text) {
        let position = cap.get(0).unwrap().start();
        let x = cap[1].parse::<u64>().unwrap();
        let y = cap[2].parse::<u64>().unwrap();
        instructions.push(Instruction::Multiply { position, x, y });
    }

    // Find all do() patterns
    for mat in enable_pattern.find_iter(text) {
        instructions.push(Instruction::Do(mat.start()));
    }

    // Find all don't() patterns
    for mat in disable_pattern.find_iter(text) {
        instructions.push(Instruction::Dont(mat.start()));
    }

    // Sort by position!
    instructions.sort_by_key(|instr| match instr {
        Instruction::Multiply { position, .. } => *position,
        Instruction::Do(pos) => *pos,
        Instruction::Dont(pos) => *pos,
    });

    let mut sum = 0;
    for instr in instructions {
        match instr {
            Instruction::Do(_) => enabled = true,
            Instruction::Dont(_) => enabled = false,
            Instruction::Multiply { x, y, .. } => {
                if enabled {
                    let result = x * y;
                    sum += result;
                }
            }
        }
    }

    sum
}

fn find_mul_patterns(text: &str) -> Vec<String> {
    // Create a regex pattern that matches mul(x,y) where x and y are 1-3 digits
    let pattern = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    // Collect all matches into a vector
    pattern.find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect()
}