use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut report_data: Vec<Vec<i32>> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.flatten() {
            report_data.push(extract_report_data(&line));
        }
    }

    let mut total_safe_reports = 0;
    for report in &report_data {
        if is_safe_report(report) || can_be_made_safe_with_removal(report) {
            total_safe_reports += 1;
        }
    }

    println!("Total Safe Reports: {}", total_safe_reports);
}

// Check if a report is safe without any modifications
fn is_safe_report(report: &[i32]) -> bool {
    if report.len() < 2 {
        return true; // Single element or empty reports are trivially safe
    }

    // Determine direction (increasing or decreasing)
    let increasing = report[0] < report[1];

    for i in 1..report.len() {
        // Check condition 1: consistent direction
        let current_increasing = report[i-1] < report[i];
        if current_increasing != increasing {
            return false;
        }

        // Check condition 2: difference between 1 and 3 inclusive
        let diff = (report[i] - report[i-1]).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }

    true
}

// Check if removing a single level can make the report safe
fn can_be_made_safe_with_removal(report: &[i32]) -> bool {
    if report.len() <= 2 {
        return false; // Can't remove from reports of 2 or fewer elements
    }

    // Try removing each level and check if the resulting report is safe
    for i in 0..report.len() {
        let mut modified_report = report.to_vec();
        modified_report.remove(i);

        if is_safe_report(&modified_report) {
            return true;
        }
    }

    false
}

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