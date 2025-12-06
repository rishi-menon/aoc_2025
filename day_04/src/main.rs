
use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");
    
    let file_path = "input_simple.txt";
    for line in fs::read_to_string(file_path).expect("Should have been able to read the file").lines() {
        println!("{line}");
    }
}
