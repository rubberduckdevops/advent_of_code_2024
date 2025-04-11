use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let mut left_side: Vec<String> = Vec::new();
    let mut right_side: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.map_while(Result::ok) {
            let mut split_result = line.split_ascii_whitespace();
            left_side.push(split_result.next().unwrap().to_string());
            right_side.push(split_result.next().unwrap().to_string());
        }

        left_side.sort();
        right_side.sort();
        println!("{:?}", left_side);
        println!("{:?}", right_side);
        find_total_distance(&left_side, &right_side);
        find_similarity_score(&left_side, &right_side);
    }
}

fn find_total_distance(left_side: &Vec<String>, right_side: &Vec<String>){
    let mut idx = 0;
    let mut distances: Vec<i32> = Vec::new();
    while idx < left_side.len() {
        let number_1 = &left_side[idx].parse::<i32>().unwrap();
        let number_2 = &right_side[idx].parse::<i32>().unwrap();
        println!("Number one {}", number_1);
        println!("Number two {}", number_2);
        let distance: i32 = number_1 - number_2;
        println!("{}", distance.abs());
        distances.push(distance.abs());
        idx += 1;
    }
    let mut total_distance: i32 = 0;
    for distance in distances {
        total_distance += distance;
    }
    println!("Total Distance: {}", total_distance);
}

fn find_similarity_score(left_list: &Vec<String>, right_list: &Vec<String>) {
    let mut total_sim_score = 0;
    for string in left_list {
        let count = &right_list.iter().filter(|&s| s == string).count();
        if count > &0 {
            total_sim_score += count * &string.parse::<usize>().unwrap();
        }
    }
    println!("total score: {}", total_sim_score);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}