use std::{cmp, collections::{HashMap, HashSet, LinkedList}, fs};

use itertools::Itertools;
use colored::*;

fn main() {
    // part1();
    part2();
}

#[derive(Debug)]
struct Point(i64, i64, i64);

impl From<(i64, i64, i64)> for Point {
    fn from(value: (i64, i64, i64)) -> Self {
        Self(value.0, value.1, value.2)
    }
}

impl ToString for Point {
    fn to_string(&self) -> String {
        format!("Point({}, {}, {})", self.0, self.1, self.2)
    }
}

fn calc_dist(point_a: &Point, point_b: &Point) -> i64 {
    let dx = point_a.0 - point_b.0;
    let dy = point_a.1 - point_b.1;
    let dz = point_a.2 - point_b.2;
    dx*dx + dy*dy + dz*dz
}


struct DisjointSets {
    sets: Vec<HashSet<usize>>
}

impl DisjointSets {
    fn new(num_points: usize) -> Self {
        let mut sets: Vec<HashSet<usize>> = Vec::with_capacity(num_points);
        for i in 0..num_points {
            let mut cur_set = HashSet::new();
            cur_set.insert(i);
            sets.push(cur_set);
        }

        Self { sets }
    }

    fn get_distinct_sets(&self) -> &Vec<HashSet<usize>> {
        &self.sets
    }
    fn merge_set(&mut self, point_index_1: usize, point_index_2: usize) -> bool {
        let set_1_index = self.sets.iter().enumerate().filter_map(|(i, set)| {
            if set.contains(&point_index_1) {
                Some(i)
            }
            else {
                None
            }
        }).next().unwrap();

        let set_2_index = self.sets.iter().enumerate().filter_map(|(i, set)| {
            if set.contains(&point_index_2) {
                Some(i)
            }
            else {
                None
            }
        }).next().unwrap();
    
        if set_1_index == set_2_index {
            println!("{} merging due to loop: {:?}", "Skipping".red(), self.sets[set_1_index]);
            return false;
        }

        // Merge the sets
        let mut set_1 = self.sets.swap_remove(cmp::max(set_1_index, set_2_index));
        let set_2 = self.sets.swap_remove(cmp::min(set_1_index, set_2_index));
        
        print!("{} {:?} and {:?}", "Merging".green(), set_1, set_2);
        set_1.extend(set_2.into_iter());
        println!("--> {:?}", set_1);
        

        self.sets.push(set_1);
        true
    }
}

fn part1() {
    println!("Hello, world!");

    // let file_path = "input_simple.txt";
    let file_path = "input_full.txt";
    let points = fs::read_to_string(file_path).unwrap().lines().map(|line| {
        let tup = line.split(",").map(|word| word.parse::<i64>().unwrap()).collect_tuple::<(i64, i64, i64)>().unwrap();
        Point(tup.0, tup.1, tup.2)
    }).collect::<Vec<Point>>();

    // println!("points: {:?}", points);

    let mut distances = Vec::new();
    for i in 0..(points.len() - 1) {
        for j in (i+1)..points.len() {
            let distance = calc_dist(&points[i], &points[j]);
            distances.push((distance, i, j));
        }
    }

    // Sort by distances ascending order
    distances.sort_by(|a, b| a.0.cmp(&b.0));

    let mut disjoint_sets = DisjointSets::new(points.len());
    let mut edge_counter = 0;
    let num_connections = 1000;

    println!("");
    for (distance, index_a, index_b) in distances {
        println!("edge: {}, Points ({}) and ({})", edge_counter.to_string().yellow(), index_a.to_string().yellow(), index_b.to_string().yellow());
        
        if disjoint_sets.merge_set(index_a, index_b) {
            edge_counter += 1;
            if edge_counter >= num_connections { break; }
        }
        else {
            edge_counter += 1;
        }

        println!("")

    }

    let circuits = disjoint_sets.get_distinct_sets();
    
    println!("----- Final sets");
    for c in circuits {
        println!("  - {:?} --> {}", c, c.len());
    }

    let mut lengths = circuits.iter().map(|set| set.len()).collect::<Vec<usize>>();
    lengths.sort_by(|a, b| b.cmp(a));
    assert!(lengths.len() >= 3);
    println!("lengths: {:?}", lengths[0] * lengths[1] * lengths[2]);
}

fn part2() {
    println!("Hello, world!");

    // let file_path = "input_simple.txt";
    let file_path = "input_full.txt";
    let points = fs::read_to_string(file_path).unwrap().lines().map(|line| {
        let tup = line.split(",").map(|word| word.parse::<i64>().unwrap()).collect_tuple::<(i64, i64, i64)>().unwrap();
        Point(tup.0, tup.1, tup.2)
    }).collect::<Vec<Point>>();

    // println!("points: {:?}", points);

    let mut distances = Vec::new();
    for i in 0..(points.len() - 1) {
        for j in (i+1)..points.len() {
            let distance = calc_dist(&points[i], &points[j]);
            distances.push((distance, i, j));
        }
    }

    // Sort by distances ascending order
    distances.sort_by(|a, b| a.0.cmp(&b.0));

    let mut disjoint_sets = DisjointSets::new(points.len());

    println!("");
    let mut final_nodes = None;
    for (distance, index_a, index_b) in distances {
        println!("Points {} and {}", index_a.to_string().yellow(), index_b.to_string().yellow());
        disjoint_sets.merge_set(index_a, index_b);
        if disjoint_sets.get_distinct_sets().len() == 1 {
            final_nodes = Some((index_a, index_b));
            break
        }
        println!("")
    }

    let (final_index_1, final_index_2) = final_nodes.expect("Could not form a single loop");

    let circuits = disjoint_sets.get_distinct_sets();
    
    println!("----- Final sets");
    for c in circuits {
        println!("  - {:?} --> {}", c, c.len());
    }

    println!("");
    println!("Final points: {:?} and {:?}", points[final_index_1], points[final_index_2]);
    println!("Final value: {}", points[final_index_1].0 * points[final_index_2].0);
}
