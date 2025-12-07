
use std::cmp;
use std::collections::HashSet;
use std::collections::VecDeque;
// use std::env;
use std::fs;

// use itertools::Itertools;

fn is_empty_col(math: &Vec<Vec<char>>, col: usize) -> bool {
    let rows = math.len();
    
    for row in 0..rows {
        if math[row][col] != ' ' {
            return false;
        }
    }
    true
}
fn main() {
    println!("Hello, world!");

    // let file_path = "input_simple.txt";
    let file_path = "input_full.txt";
    
    let lines: Vec<String> = fs::read_to_string(file_path).expect("Did not find file").lines().map(|s| String::from(s)).collect();
    
    
    let mut math = Vec::new();
    let mut add_operator = None;
    for (i, line) in lines.iter().enumerate() {
        if i < lines.len() - 1 {
            let chars: Vec<char> = line.chars().collect();
            math.push(chars);
        }
        else {
            let op: Vec<bool> = line.split_whitespace().map(|s| {
                match s {
                    "*" => false,
                    "+" => true,
                    op => unreachable!("Unknown operator: {}", op),
                }
            }).collect();
            add_operator = Some(op);
        }
    }
    
    let add_operator = add_operator.unwrap();
    
    // println!("{:?}", math);
    // println!("{:?}", add_operator);

    for i in 1..math.len() {
        assert_eq!(math[0].len(), math[i].len());
    }

    let rows = math.len();
    let cols = math[0].len();
    let mut operator_index = 0;
    
    let mut numbers_to_operate = Vec::new();
    let mut final_results = Vec::new();

    for col in 0..(cols+1) {
        if col == cols || is_empty_col(&math, col) {
            let operator = add_operator[operator_index];
            println!("numbers: {:?}, operator: {}", numbers_to_operate, if operator {"add"} else {"mul"});
            let result = numbers_to_operate.into_iter().reduce(|acc, ele| {
                if operator {
                    acc + ele
                }
                else {
                    acc * ele
                }
            }).unwrap();
            final_results.push(result);
            
            numbers_to_operate = Vec::new();
            operator_index += 1;
            continue;   
        }

        let number: String = (0..rows).map(|i| math[i][col]).collect::<String>();
        let number = number.trim().parse::<i64>().unwrap();
        numbers_to_operate.push(number);
    }

    println!("final results: {:?}", final_results);
    let final_reduced_result = final_results.into_iter().reduce(|acc, ele| acc + ele).unwrap();
    println!("final reduced results: {:?}", final_reduced_result);


}
