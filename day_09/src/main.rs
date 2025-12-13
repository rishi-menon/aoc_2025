use std::{cmp, fs, hash::Hash};

use colored::Colorize;
use itertools::{Itertools};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Edge(Point, Point);

enum IntersectEnum {
    None,
    Grazing(i64), // winding num
    Full,
}

impl Edge {
    fn intersect(&self, other_edge: &Edge) -> IntersectEnum {
        // returns True when the intersection was not grazing
        // returns winding number

        let (a, b) = (self.0, self.1);
        let (c, d) = (other_edge.0, other_edge.1);

        let (d1x, d1y) = (b.x-a.x, b.y-a.y);
        let (d2x, d2y) = (d.x-c.x, d.y-c.y);

        let denominator = -d1x * d2y + d1y*d2x;
        // println!("---");
        // println!("A: ({}, {}), B: ({}, {}), C: ({}, {}), D: ({}, {})", a.x, a.y, b.x, b.y, c.x, c.y, d.x, d.y);
        // println!("d1: ({}, {}), d2: ({}, {})", d1x, d1y, d2x, d2y);
        // println!("denominator: {}", denominator);
        if denominator == 0 {
            // Parallel lines could technically intersect
            let (t1, t2) = if d1x == 0 && d2x == 0 {
                if a.x != c.x {
                    return IntersectEnum::None;
                }

                let t1 = (c.y - a.y) as f32 / d1y as f32;
                let t2 = (d.y - a.y) as f32 / d1y as f32;
                (t1, t2)
            }
            else if d1y == 0 && d2y == 0 {
                if a.y != c.y {
                    return IntersectEnum::None;
                }
                let t1 = (c.x - a.x) as f32 / d1x as f32;
                let t2 = (d.x - a.x) as f32 / d1x as f32;
                (t1, t2)
            }
            else {
                panic!("Invalid assertion");
            };

            // println!("t1: {}, t2: {}", t1, t2);
            if (0.0 < t1 && t1 < 1.0) || (0.0 < t2 && t2 < 1.0) {
                return IntersectEnum::Full;
            }
            if (0.0 <= t1 && t1 <= 1.0) || (0.0 <= t2 && t2 <= 1.0) {
                // 0 since the rays are parallel
                return IntersectEnum::Grazing(0);
            }
            else {
                return IntersectEnum::None;
            }
        }

        let (e, f) = (c.x - a.x, c.y - a.y);

        let t1 = (-d2y * e + d2x * f) as f32 / (denominator as f32);
        let t2 = (-d1y * e + d1x * f) as f32 / (denominator as f32);
        // println!("t1: {}, t2: {}", t1, t2);

        if 0.0 < t1 && t1 < 1.0 && 0.0 < t2 && t2 < 1.0 {
            return IntersectEnum::Full;
        }
        else if 0.0 <= t1 && t1 <= 1.0 && 0.0 <= t2 && t2 <= 1.0 {
            // rays are not parallel
            let winding_num = if d1x == 0 && d2y == 0 {
                d2y.signum()
            }
            else if d1y == 0 && d2x == 0 {
                d2x.signum()
            }
            else {
                panic!("Invalid case");
            };
            return IntersectEnum::Grazing(winding_num);
        }
        else {
            return IntersectEnum::None; 
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Shape {
    edges: Vec<Edge>,
    points: Vec<Point>,
}


impl From<Vec<Point>> for Shape {
    fn from(points: Vec<Point>) -> Self { 
        // let points = edges.iter().map(|edge| edge.0).collect::<Vec<Point>>();
        let edges = (0..points.len()).map(|i| Edge(points[i], points[(i+1) % points.len()])).collect::<Vec<Edge>>();
        Self{edges, points}
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

    fn valid_row(&self, cur_edge: Edge) -> bool {
        let mut winding_num = 0;
        let mut has_winding_num = false;
        for edge in self.edges.iter() {
            match cur_edge.intersect(edge) {
                IntersectEnum::None => { continue; }
                IntersectEnum::Grazing(w) => { winding_num += w; has_winding_num = true; },
                IntersectEnum::Full => {return false; }
            }
        }
        // return winding_num != 0;
        println!("Winding num: {winding_num}, has_winding_num: {has_winding_num}");
        if has_winding_num {
            return winding_num != 0;
        }
        else {
            return true;
        }
    }

    fn valid_square(&self, point_a: Point, point_b: Point) -> bool {
        let (min_y, max_y) = (cmp::min(point_a.y, point_b.y), cmp::max(point_a.y, point_b.y));
        let (min_x, max_x) = (cmp::min(point_a.x, point_b.x), cmp::max(point_a.x, point_b.x));
        for row in min_y..max_y+1 {
            let cur_edge = Edge(
                Point{x:point_a.x, y:row},
                Point{x:point_b.x, y:row},
            );
            if !self.valid_row(cur_edge) {
                return false;
            }
        }
        true
    }
    fn largest_square(&self) -> i64 {
        let num_points = self.points.len();
        
        let mut max_area = 0i64;
        for i in 0..num_points-1 {
            for j in i+1..num_points {
                println!("----- i: {}, j: {}", i, j);
                print_rect(self, self.points[i], self.points[j]);
                if self.valid_square(self.points[i], self.points[j]) {
                    println!("{}", "Valid grid".green());
                    
                    let cur_area = calculate_area(self.points[i], self.points[j]);
                    
                    if max_area < cur_area || true {
                        println!("cur_area: {cur_area}");
                        max_area = cmp::max(cur_area, max_area);
                    }
                }
                else {
                    println!("{}", "Invalid grid".green());
                }
            }
        }       
        max_area
    }
}

fn print_edges(edge_1: &Edge, edge_2: &Edge) {
    // Print grid
    let cols = cmp::max(cmp::max(edge_1.0.x, edge_1.1.x), cmp::max(edge_2.0.x, edge_2.1.x)) + 2;
    let rows = cmp::max(cmp::max(edge_1.0.y, edge_1.1.y), cmp::max(edge_2.0.y, edge_2.1.y)) + 2;

    println!("rows: {}, cols: {}", rows, cols);
    let mut grid = vec![vec![0; cols as usize]; rows as usize];
    
    let (a, b) = (edge_1.0, edge_1.1);
    if a.x == b.x {
        let (min, max) = (cmp::min(a.y, b.y), cmp::max(a.y, b.y));
        for row in min..max+1 {
            grid[row as usize][a.x as usize] = 2;
        }
    }
    else if a.y == b.y {
        let (min, max) = (cmp::min(a.x, b.x), cmp::max(a.x, b.x));
        for col in min..max+1 {
            grid[a.y as usize][col as usize] = 2;
        }
    }
    else {
        assert!(false, "Impossible case");
    }
    
    let (a, b) = (edge_2.0, edge_2.1);
    if a.x == b.x {
        let (min, max) = (cmp::min(a.y, b.y), cmp::max(a.y, b.y));
        for row in min..max+1 {
            grid[row as usize][a.x as usize] = 2;
        }
    }
    else if a.y == b.y {
        let (min, max) = (cmp::min(a.x, b.x), cmp::max(a.x, b.x));
        for col in min..max+1 {
            grid[a.y as usize][col as usize] = 2;
        }
    }
    else {
        assert!(false, "Impossible case");
    }

    grid[edge_1.0.y as usize][edge_1.0.x as usize] = 1;
    grid[edge_1.1.y as usize][edge_1.1.x as usize] = 1;
    grid[edge_2.0.y as usize][edge_2.0.x as usize] = 1;
    grid[edge_2.1.y as usize][edge_2.1.x as usize] = 1;

    for row in 0..rows {
        if row == 0 {
            print!("  ");
            for col in 0..cols {
                print!("{}", (col % 10).to_string().green());
            }
            println!("");
        }
        
        for col in 0..cols {
            if col == 0 {
                print!("{} ", (row % 10).to_string().green());
            }
            
            match grid[row as usize][col as usize] {
                0 => print!("."),
                1 => print!("{}", "#".red()),
                2 => print!("{}", "*".yellow()),
                t => panic!("Unexpected grid value: {t}"),
            };
        }
        println!("");
        
    }


}   
fn print_rect(shape: &Shape, point_a: Point, point_b: Point) {
    // Print grid
    let cols = shape.points.iter().map(|point| point.x).max().unwrap() + 1;
    let rows = shape.points.iter().map(|point| point.y).max().unwrap() + 1;

    // println!("rows: {}, cols: {}", rows, cols);
    let mut grid = vec![vec![0; cols as usize]; rows as usize];
    
    let (min_x, max_x) = (cmp::min(point_a.x, point_b.x), cmp::max(point_a.x, point_b.x));
    let (min_y, max_y) = (cmp::min(point_a.y, point_b.y), cmp::max(point_a.y, point_b.y));
    
    for row in min_y..max_y+1 {
        for col in min_x..max_x+1 {
            grid[row as usize][col as usize] = 2;
        }
    }
    for point in shape.points.iter() {
        if grid[point.y as usize][point.x as usize] == 0 {
            grid[point.y as usize][point.x as usize] = 1;
        }
    }

    for row in 0..rows {
        if row == 0 {
            print!("  ");
            for col in 0..cols {
                print!("{}", (col % 10).to_string().green());
            }
            println!("");
        }
        
        for col in 0..cols {
            if col == 0 {
                print!("{} ", (row % 10).to_string().green());
            }
            
            match grid[row as usize][col as usize] {
                0 => print!("."),
                1 => print!("{}", "#".red()),
                2 => print!("{}", "*".yellow()),
                t => panic!("Unexpected grid value: {t}"),
            };
        }
        println!("");
        
    }


}   
// fn print_shape(shape: &Shape) {
//     // Print grid
//     let cols = shape.points.iter().map(|point| point.x).max().unwrap() + 1;
//     let rows = shape.points.iter().map(|point| point.y).max().unwrap() + 1;

//     for row in 0..rows+1 {
//         if row == 0 {
//             print!("  ");
//             for col in 0..cols+1 {
//                 print!("{}", (col % 10).to_string().green());
//             }
//             println!("");
//         }
        
//         for col in 0..cols+1 {
//             if col == 0 {
//                 print!("{} ", (row % 10).to_string().green());
//             }

//             let cur_point = Point{x:col, y:row};
//             if shape.points.contains(&cur_point) {
//                 print!("{}", "#".red());
//             }
//             else if shape.contains(cur_point) {
//                 print!("{}", "*".yellow());
//             } else {
//                 print!("{}", ".");
//             }
//         }
//         println!("");
        
//     }
// }

fn main() {
    // part1_naive();
    part2_naive();
}

fn calculate_area(point_a: Point, point_b: Point) -> i64 {
    let dx = (point_a.x - point_b.x).abs();
    let dy = (point_a.y - point_b.y).abs();
    (dx + 1) * (dy + 1)
}

fn part1_naive() {
    println!("----- Naive solution -----");

    // String parsing
    // let file_path = "input_simple.txt";
    let file_path = "input_full.txt";
    let points = fs::read_to_string(file_path).expect("File does not exist").lines().map(|line| {
        let nums: (i64, i64) = line.split(",").map(|word| word.parse::<i64>().expect("Could not parse number")).collect_tuple().expect("Expected tuple");
        Point{x: nums.0, y:nums.1}
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


fn part2_naive() {
    println!("----- Part 2 niave solution -----");

    // String parsing
    let file_path = "input_simple.txt";
    // let file_path = "input_full.txt";
    let points = fs::read_to_string(file_path).expect("File does not exist").lines().map(|line| {
        let nums: (i64, i64) = line.split(",").map(|word| word.parse::<i64>().expect("Could not parse number")).collect_tuple().expect("Expected tuple");
        Point{x: nums.0, y:nums.1}
    }).collect::<Vec<Point>>();

    let shape: Shape = points.into();
    let max_area = shape.largest_square();
    println!("Largest area: {}", max_area);
    // println!("{:?}", shape.points);
    // // print_shape(&shape);

    // let edge_1 = Edge (
    //     Point { x: 1, y: 0 },
    //     Point { x: 1, y: 5 },
    // );
    // let edge_2 = Edge (
    //     Point { x: 10, y: 0 },
    //     Point { x: 1, y: 0 },
    // );
    // print_edges(&edge_1, &edge_2);
    
    // println!("Intersect: {}", edge_1.intersect(&edge_2));

}