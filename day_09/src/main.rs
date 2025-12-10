use std::{cmp, hash, collections::HashSet, fs, hash::Hash, convert};

use colored::Colorize;
use itertools::{Itertools, MinMaxResult};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point(i64, i64);
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Edge(Point, Point);
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Shape(Vec<Edge>, Vec<Point>);

impl From<Vec<Edge>> for Shape {
    fn from(edges: Vec<Edge>) -> Self { 
        let points = edges.iter().map(|edge| edge.0).collect::<Vec<Point>>();
        Self(edges, points)
    }
}

impl Shape {
    // SAT
    // fn contains(&self, point: Point) -> bool {
    //     self.0.iter().map(|edge| {
    //         let point_a = edge.0;
    //         let point_b = edge.1;
    //         let normal = (-(point_b.1 - point_a.1), point_b.0 - point_a.0);
            
    //         let minmax_enum = self.1.iter().map(|v| v.0 * normal.0 + v.1 * normal.1).minmax();
    //         let (min, max) = match minmax_enum {
    //             MinMaxResult::MinMax(a, b) => (a, b),
    //             _ => panic!("We don't have a min and max after projecting"),
    //         };

    //         let proj = point.0 * normal.0 + point.1 * normal.1;
    //         min <= proj && proj <= max
    //     }).all(convert::identity)
    // }
    fn contains(&self, point: Point) -> bool {
        false
    }
}

// fn print_board(points: &Vec<Point>) {
//     // Print grid
//     let cols = points.iter().map(|x| x.0).max().unwrap() + 1;
//     let rows = points.iter().map(|x| x.1).max().unwrap() + 1;

//     for i in 0..rows+1 {
//         if i == 0 {
//             print!("  ");
//             for j in 0..cols+1 {
//                 print!("{}", (j % 10).to_string().green());
//             }
//             println!("");
//         }
        
//         for j in 0..cols+1 {
//             if j == 0 {
//                 print!("{} ", (i % 10).to_string().green());
//             }

//             let cur_point = Point(i, j);
//             if points.contains(&cur_point) {
//                 print!("{}", "#".red());
//             } else {
//                 print!("{}", ".");
//             }
//         }
//         println!("");
        
//     }
// }

fn print_shape(shape: &Shape, points: &Vec<Point>) {
    // Print grid
    let cols = points.iter().map(|x| x.0).max().unwrap() + 1;
    let rows = points.iter().map(|x| x.1).max().unwrap() + 1;

    for row in 0..rows+1 {
        if row == 0 {
            print!("  ");
            for col in 0..cols+1 {
                print!("{}", (col % 10).to_string().green());
            }
            println!("");
        }
        
        for col in 0..cols+1 {
            if col == 0 {
                print!("{} ", (row % 10).to_string().green());
            }

            let cur_point = Point(col, row);
            if points.contains(&cur_point) {
                print!("{}", "#".red());
            }
            else if shape.contains(cur_point) {
                print!("{}", "*".yellow());
            } else {
                print!("{}", ".");
            }
        }
        println!("");
        
    }
}

fn print_board_with_square(points: &Vec<Point>, point_a: Point, point_b: Point) {
    // Print grid
    let cols = points.iter().map(|x| x.0).max().unwrap() + 1;
    let rows = points.iter().map(|x| x.1).max().unwrap() + 1;

    let slice_cols = (cmp::min(point_a.0, point_b.0), cmp::max(point_a.0, point_b.0));
    let slice_rows = (cmp::min(point_a.1, point_b.1), cmp::max(point_a.1, point_b.1));

    for row in 0..rows+1 {
        if row == 0 {
            print!("  ");
            for col in 0..cols+1 {
                print!("{}", (col % 10).to_string().green());
            }
            println!("");
        }
        
        for col in 0..cols+1 {
            if col == 0 {
                print!("{} ", (row % 10).to_string().green());
            }

            let cur_point = Point(col, row);
            if points.contains(&cur_point) {
                print!("{}", "#".red());
            }
            else if slice_cols.0 <= cur_point.0 && cur_point.0 <= slice_cols.1 && slice_rows.0 <= cur_point.1 && cur_point.1 <= slice_rows.1 {
                print!("{}", "#".yellow());
            } else {
                print!("{}", ".");
            }
        }
        println!("");
        
    }
    println!("Area: {}", calculate_area(point_a, point_b));
    println!("");
}

fn main() {
    // part1_naive();
    // part1_actual();
    part2_naive();
}

fn calculate_area(point_a: Point, point_b: Point) -> i64 {
    let dx = (point_a.0 - point_b.0).abs();
    let dy = (point_a.1 - point_b.1).abs();
    (dx + 1) * (dy + 1)
}

fn part1_naive() {
    println!("----- Naive solution -----");

    // String parsing
    // let file_path = "input_simple.txt";
    let file_path = "input_full.txt";
    let points = fs::read_to_string(file_path).expect("File does not exist").lines().map(|line| {
        let nums: (i64, i64) = line.split(",").map(|word| word.parse::<i64>().expect("Could not parse number")).collect_tuple().expect("Expected tuple");
        Point(nums.0, nums.1)
    }).collect::<Vec<Point>>();

    let mut max_area = 0i64;
    for i in 0..points.len()-1 {
        for j in (i+1)..points.len() {
            let area = calculate_area(points[i], points[j]);

            if max_area < area {
                max_area = area;
                // print_board_with_square(&points, points[i], points[j]);
            }
        }
    }

    println!("Max area: {}", max_area);
}


fn part1_actual() {
    println!("----- Actual solution -----");

    // String parsing
    // let file_path = "input_simple.txt";
    let file_path = "input_full.txt";
    let mut points = fs::read_to_string(file_path).expect("File does not exist").lines().map(|line| {
        let nums: (i64, i64) = line.split(",").map(|word| word.parse::<i64>().expect("Could not parse number")).collect_tuple().expect("Expected tuple");
        Point(nums.0, nums.1)
    }).collect::<HashSet<Point>>();

    let point_a = *points.iter().max_by_key(|&point| point.0).unwrap();
    let point_b = *points.iter().max_by_key(|&point| point.1).unwrap();
}

fn part2_naive() {
    println!("----- Part 2 niave solution -----");

    // String parsing
    let file_path = "input_simple.txt";
    // let file_path = "input_full.txt";
    let points = fs::read_to_string(file_path).expect("File does not exist").lines().map(|line| {
        let nums: (i64, i64) = line.split(",").map(|word| word.parse::<i64>().expect("Could not parse number")).collect_tuple().expect("Expected tuple");
        Point(nums.0, nums.1)
    }).collect::<Vec<Point>>();

    let shape: Shape = (0..points.len()).map(|i| Edge(points[i], points[(i+1) % points.len()])).collect::<Vec<Edge>>().into();

    println!("{:?}", points);
    print_shape(&shape, &points);
    // print_board(&points);

    // let mut max_area = 0i64;
    // for i in 0..points.len()-1 {
    //     for j in (i+1)..points.len() {
    //         let area = calculate_area(points[i], points[j]);

    //         if max_area < area {
    //             max_area = area;
    //             print_board_with_square(&points, points[i], points[j]);
    //         }
    //     }
    // }

}