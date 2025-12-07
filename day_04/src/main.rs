
use std::collections::VecDeque;
use std::env;
use std::fs;

use colored::Colorize;


fn num_adjacent(grid: &Vec<Vec<bool>>, i: i32, j: i32) -> i32 {
    let size_x = grid.len() as i32;
    let size_y = grid[0].len() as i32;

    let mut count = 0;
    for dx in -1..2 {
        for dy in -1..2 {
            let x = i + dx;
            let y = j + dy;

            if dx == 0 && dy == 0 { continue; }
            if x < 0 || x >= size_x || y < 0 || y >= size_y { continue; }
            
            if grid[x as usize][y as usize] {
               count += 1; 
            }
        }
    }
    count
}

fn count_rolls(grid: &Vec<Vec<bool>>) -> i32 {
    let size_x = grid.len();
    let size_y = grid[0].len();
    println!("Grid: {}x{}", size_x,size_y);

    // debug board
    println!("");
    for i in 0..size_x {
        for j in 0..size_y {
            let is_accessible = num_adjacent(grid, i as i32, j as i32) < 4;
            if !grid[i][j] {
                print!("{}", ".")
            }
            else if is_accessible {
                print!("{}", "@".green());
            }
            else {
                print!("{}", "@".red());
            }
        }
        println!("");
    }
    // println!("");
    // for i in 0..size_x {
    //     for j in 0..size_y {
    //         let num_accessible = num_adjacent(grid, i as i32, j as i32);
    //         print!("{num_accessible}")
    //     }
    //     println!("");
    // }

    let mut num_accessible = 0;
    for i in 0..size_x {
        for j in 0..size_y {
            if grid[i][j] {
                num_accessible += if num_adjacent(grid, i as i32, j as i32) < 4 { 1 } else { 0 }
            }
        }
    }
    num_accessible
}

fn count_remove_rolls(grid: &Vec<Vec<bool>>) -> i32 {
    let size_x = grid.len() as i32;
    let size_y = grid[0].len() as i32;
    println!("Grid: {}x{}", size_x,size_y);

    // debug board
    println!("");
    for i in 0..size_x {
        for j in 0..size_y {
            let is_accessible = num_adjacent(grid, i as i32, j as i32) < 4;
            if !grid[i as usize][j as usize] {
                print!("{}", ".")
            }
            else if is_accessible {
                print!("{}", "@".green());
            }
            else {
                print!("{}", "@".red());
            }
        }
        println!("");
    }


    let mut grid = grid.clone();
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();

    for i in 0..size_x {
        for j in 0..size_y {
            queue.push_back((i, j));
        }
    }

    let mut num_removal = 0;

    while !queue.is_empty() {
        let (i, j) = queue.pop_front().expect("queue cannot be empty");
        
        if i < 0 || i >= size_x || j < 0 || j >= size_y {
            continue;
        }
        if !grid[i as usize][j as usize] {
            continue;
        }

        if num_adjacent(&grid, i, j) < 4 {
            grid[i as usize][j as usize] = false;
            num_removal += 1;

            for dx in -1..2 {
                for dy in -1..2 {
                    let x = i + dx;
                    let y = j + dy;
                    if dx == 0 && dy == 0 { continue; }
                    
                    println!("Pushed ({x}, {y}) to queue");
                    queue.push_back((x, y));
                }
            }
        }

    }
    num_removal
}
fn main() {
    println!("Hello, world!");
    
    // let file_path = "input_full.txt";
    let file_path = "input_full.txt";
    let mut grid: Vec<Vec<bool>> = Vec::new();
    for line in fs::read_to_string(file_path).expect("Should have been able to read the file").lines() {
        let row: Vec<bool> = line.chars().map(|c| c == '@').collect();
        grid.push(row);
    }

    let count = count_remove_rolls(&grid);
    println!("");
    println!("Number of removed rolls: {}", count);


}
