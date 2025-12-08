use std::fs;

fn main() {
    // part1();
    part2();
}
fn part2() {
    // let file_name = "input_simple.txt";
    let file_name = "input_full.txt";

    let lines: Vec<String> = fs::read_to_string(file_name).unwrap().lines().map(|s| String::from(s)).collect();
    // println!("{:?}", lines);
    let (first, rest) = lines.split_first().unwrap();

    let laser = first.chars().map(|c| {
        match c {
            'S' => 1i64,
            '.' => 0i64,
            other => unreachable!("Unknown character: {}", other),
        }
    }).collect::<Vec<i64>>();

    let grid = rest.iter().map(|s| {
        let res = s.chars().map(|c| {
            match c {
            '^' => true,
            '.' => false,
            other => unreachable!("Unknown character: {}", other),
        }
        }).collect::<Vec<bool>>();
        res
    }).collect::<Vec<Vec<bool>>>();

    let mut cur_laser = laser;
    for (layer_i, splitter) in grid.iter().enumerate() {
        let mut new_laser = vec![0i64; cur_laser.len()];
        for i in 0..splitter.len() {
            if cur_laser[i] > 0 && splitter[i] {
                if (i as i64) - 1 >= 0 {
                    new_laser[i-1] += cur_laser[i];
                }
                if i+1 < new_laser.len() {
                    new_laser[i+1] += cur_laser[i];
                }
            }
            else if cur_laser[i] > 0 {
                new_laser[i] += cur_laser[i];
            }
        }

        // println!("");
        // println!("--- Layer {}", layer_i);
        // for &i in cur_laser.iter() {
        //     print!("{}", i);
        // }
        // println!("");
        // for &i in splitter.iter() {
        //     if i {
        //         print!("^");
        //     }
        //     else {
        //         print!(".");
        //     }
        // }
        // println!("");

        // for &i in new_laser.iter() {
        //     print!("{}", i);
        // }
        // println!("");


        cur_laser = new_laser;
    }

    // println!("");
    // println!("{:?}", cur_laser);
    let total_timelines = cur_laser.into_iter().reduce(|acc, ele| acc + ele).unwrap();
    println!("total_timelines: {}", total_timelines);

}
fn part1() {
    // let file_name = "input_simple.txt";
    let file_name = "input_full.txt";

    let lines: Vec<String> = fs::read_to_string(file_name).unwrap().lines().map(|s| String::from(s)).collect();
    // println!("{:?}", lines);
    let (first, rest) = lines.split_first().unwrap();

    let laser = first.chars().map(|c| {
        match c {
            'S' => true,
            '.' => false,
            other => unreachable!("Unknown character: {}", other),
        }
    }).collect::<Vec<bool>>();

    let grid = rest.iter().map(|s| {
        let res = s.chars().map(|c| {
            match c {
            '^' => true,
            '.' => false,
            other => unreachable!("Unknown character: {}", other),
        }
        }).collect::<Vec<bool>>();
        res
    }).collect::<Vec<Vec<bool>>>();


    let mut num_splits = 0;
    let mut cur_laser = laser;
    for (layer_i, splitter) in grid.iter().enumerate() {
        let mut new_laser = vec![false; cur_laser.len()];

        let mut layer_splits = 0;
        for i in 0..splitter.len() {
            if cur_laser[i] && splitter[i] {
                layer_splits += 1;

                if (i as i64) - 1 >= 0 {
                    new_laser[i-1] = true;
                }
                if i+1 < new_laser.len() {
                    new_laser[i+1] = true;
                }
            }
            else if cur_laser[i] {
                new_laser[i] = cur_laser[i];
            }
        }

        // Debug stuff
        println!("");
        println!("--- Layer {}", layer_i);
        for &i in cur_laser.iter() {
            if i {
                print!("|");
            }
            else {
                print!(".");
            }
        }
        println!("");
        for &i in splitter.iter() {
            if i {
                print!("^");
            }
            else {
                print!(".");
            }
        }
        println!("");

        for &i in new_laser.iter() {
            if i {
                print!("|");
            }
            else {
                print!(".");
            }
        }
        println!("");


        num_splits += layer_splits;
        cur_laser = new_laser;

    }

    println!("");
    println!("Num splits: {}", num_splits);

}
