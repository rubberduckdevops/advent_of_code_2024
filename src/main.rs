use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;
use std::collections::HashMap;



fn main() {
    let mut report_data: Vec<Vec<i32>> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.map_while(Result::ok) {
            report_data.push(extract_report_data(&line));
            println!("{}", line);
        }
    }
    println!("{:?}", report_data);
    let mut total_safe_reports: i32 = 0;
    for data in report_data {
        if determine_safe(data) {
            println!("Report Safe");
            total_safe_reports += 1;
        }
    }
    println!("Total Safe: {}", total_safe_reports);
}

fn determine_safe(report: Vec<i32>) -> bool{
    /**
        The levels are either all increasing or all decreasing.
        Any two adjacent levels differ by at least one and at most three.
    **/
    let condition_1: bool = determine_direction_safe(&report);
    let condition_2: bool = determine_level_differ_safe(&report);


    if condition_1 & condition_2 {
        println!("Safe!");
        true
    } else {
        println!("UNSAFE");
        false
    }

}

fn determine_level_differ_safe(report: &Vec<i32>) -> bool {
    for i in 1..report.len() {
        let diff = report[i] - report[i-1];
        if diff.abs() > 3 {
            println!("Unsafe Greater than 3");
            return false
        } else if diff.abs() < 1 {
            println!("Unsafe Less than 1");
            return false
        } else {
            continue;
        }
    }
    true
}

fn determine_direction_safe(report: &Vec<i32>) -> bool{
    let mut is_increasing = false;
    let mut is_decreasing = false;
    for i in 1..report.len() {
        if report[i] <= report[i-1] {
            is_increasing = true;
        } else {
            is_decreasing = true;
        }
    }

    if is_increasing & is_decreasing {
        println!("Direction: Unsafe");
        return false
    } else if is_increasing {
        println!("Direction: Safe(Up)");
        return true
    }else if is_decreasing {
        println!("Direction: Safe(Down)");
        return true
    } else {
        println!("UNSAFE! BROKEN");
        return false
    }


}

fn extract_report_data(string: &String) -> Vec<i32>{
    let split_line = string.split_ascii_whitespace();
    let mut report: Vec<i32> = Vec::new();
    for line in split_line {
        report.push(
            line.parse::<i32>().unwrap()
        );

    }
    report
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}