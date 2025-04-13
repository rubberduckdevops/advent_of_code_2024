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
        let mut report_outcome = determine_safe(data);
        while !report_outcome.condition_2 || !report_outcome.condition_1 {
            println!("{}", report_outcome.report_data.len());
            if report_outcome.report_data.len() < 6 {
                println!("Index less than 6 break loop");
                break;
            }
            report_outcome = determine_safe(report_outcome.report_data)
        }
        if report_outcome.condition_2 && report_outcome.condition_1 {
            println!("Report completed!");
            total_safe_reports += 1
        }

    }
    println!("Total Safe: {}", total_safe_reports);
}

struct ReactorReport {
    report_data: Vec<i32>,
    condition_1: bool,
    condition_2: bool,
    direction: Direction,
}

#[derive(PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
    Unknown,
}

impl ReactorReport {
    fn new(data: Vec<i32>, cond1: bool, cond2: bool, dir: Direction) -> Self {
        ReactorReport {
            report_data: data,
            condition_1: cond1,
            condition_2: cond2,
            direction: dir,
        }
    }
}


fn determine_safe(new_data: Vec<i32>) -> ReactorReport{
    /**
        The levels are either all increasing or all decreasing.
        Any two adjacent levels differ by at least one and at most three.
    **/

    let mut report = ReactorReport::new(
        new_data.clone(),
        true,
        true,
        Direction::Unknown,
    );
    println!("Starting evaluation");
    for i in 1..report.report_data.len() {
        let data_tuple: (i32, i32) = (report.report_data[i-1], report.report_data[i]);
        // Test Condition 1
        let pair_direction = determine_direction(data_tuple);
        println!("{:#?}", data_tuple);
        if report.direction == Direction::Unknown {
            report.direction = pair_direction;
        } else {
            if pair_direction != report.direction {
                println!("Direction does not match...");
                report.condition_1 = false;
                println!("{}", report.condition_1);
                // Handle failure here
            } else {
                report.condition_1 = true;
            }

        }

        // Test Condition 2
        if level_differ(data_tuple) {
            report.condition_2 = true;
        } else {
            report.condition_2 = false;
        }

        // Check Both Conditions are True
        if report.condition_1 && report.condition_2 {
            println!("Happy days conditions match for tuple");
            continue;
        } else {
            report.report_data.remove(i);
            println!("{:?}", report.report_data);
            return report;
        }
    }
    report
}

fn determine_direction(pair:(i32,i32)) -> Direction{
    if pair.0 < pair.1 {
        Direction::Decreasing
    } else {
        Direction::Increasing
    }
}

fn determine_safe_dampener(report: Vec<i32>){
    let mut report_safety = false;
    let mut failure_count = 0;
    for i in 1..report.len() {
        if level_differ((report[i], report[i-1])) {
            continue;
        } else {
            println!("maybe we should try again");
            failure_count += 1;
        }
    }

}


fn level_differ(pair: (i32,i32)) -> bool{
    let diff:i32 = pair.0 - pair.1;
    if diff.abs() > 3 {
        return false
    } else if diff.abs() == 0 {
        return false
    } else {
        return true
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