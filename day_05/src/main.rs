

use std::cmp;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;

use itertools::Itertools;

// use colored::Colorize;


fn check_valid(valid_indices: &Vec<(i64, i64)>, query_index: i64) -> bool {
    for slice in valid_indices {
        if slice.0 <= query_index && query_index <= slice.1 {
            return true;
        }
    }
    return false
}

// fn main() {
//     println!("Hello, world!");
//     let file_path = "input_full.txt";
//     // let file_path = "input_simple.txt";

//     let mut valid_indices: Vec<(i64, i64)> = Vec::new();
//     let mut query_indices: Vec<i64> = Vec::new();
//     for line in fs::read_to_string(file_path).expect("Did not find file").lines() {
//         if line.contains("-") {
//             let indices: Vec<i64> = line.split("-").map(|s| s.parse::<i64>().expect("Cannot parse i64")).collect();
//             valid_indices.push((indices[0], indices[1]));
//         }
//         else if !line.is_empty() {
//             let num: i64 = line.parse().unwrap();
//             query_indices.push(num);
//             // println!("nums: {}", num);
//         }
//     }

//     let mut num_fresh_ids = 0;
//     for q in query_indices {
//         if check_valid(&valid_indices, q) {
//             num_fresh_ids += 1;
//         }
//     }

//     println!("Num fresh ids: {num_fresh_ids}")

// }

fn naive_solution(valid_indices: &Vec<(i64, i64)>) -> i64{
    let mut set = HashSet::new();
    for slice in valid_indices {
        for i in slice.0 .. (slice.1 + 1) {
            set.insert(i);
        }
    }
    set.len() as i64
}

fn complicated_solution(valid_indices: &Vec<(i64, i64)>) -> i64 {
    let mut counter = 0i64;
    let mut prev_start = valid_indices[0].0;
    let mut prev_end = valid_indices[0].1;

    for slice in valid_indices {
        let (new_start, new_end) = *slice;

        if new_start > prev_end {
            println!("Distinct slice: ({}, {}) --> {}", prev_start, prev_end, (prev_end - prev_start) + 1);
            counter += (prev_end - prev_start) + 1;

            prev_start = new_start;
            prev_end = new_end;
            continue;
        }
        
        println!("Merging slice: ({}, {}) and ({}, {})", prev_start, prev_end, new_start, new_end);
        prev_start = cmp::min(prev_start, new_start);
        prev_end = cmp::max(prev_end, new_end);
    }
    println!("Distinct slice: ({}, {}) --> {}", prev_start, prev_end, (prev_end - prev_start) + 1);
    counter += (prev_end - prev_start) + 1;
    counter

}

fn main() {
    println!("Hello, world!");
    let file_path = "input_full.txt";
    // let file_path = "input_simple.txt";

    let mut valid_indices: Vec<(i64, i64)> = Vec::new();
    for line in fs::read_to_string(file_path).expect("Did not find file").lines() {
        if line.contains("-") {
            let indices: (i64, i64) = line.split("-").map(|s| s.parse::<i64>().expect("Cannot parse i64")).collect_tuple().unwrap();
            // println!("{:?}", indices);
            valid_indices.push((indices.0, indices.1));
        }
    }

    valid_indices.sort();
    println!("{:?}", valid_indices);    
    let complciated_valid = complicated_solution(&valid_indices);

    println!("");
    println!("Complicated valid: {}", complciated_valid);


    // let mut num_fresh_ids = 0;
    // for q in query_indices {
    //     if check_valid(&valid_indices, q) {
    //         num_fresh_ids += 1;
    //     }
    // }

    // println!("Num fresh ids: {num_fresh_ids}")

}
