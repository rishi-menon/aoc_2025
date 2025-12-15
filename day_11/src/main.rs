use std::{collections::HashMap, fs};

use itertools::Itertools;

fn main() {
    println!("Hello, world!");
    part1();
}
#[derive(Debug)]
struct Device {
    name: String,
    outs: Vec<String>
}

fn parse_input(file_name: &str) -> Vec<Device> {
    fs::read_to_string(file_name).unwrap().lines().filter(|&line| !line.starts_with("#")).map(|line| {
        let (device_name, rest) = line.split(":").collect_tuple::<(&str, &str)>().unwrap();
        let device_outs = rest.split_whitespace().map(|s| String::from(s)).collect::<Vec<String>>();
        Device{name: String::from(device_name), outs: device_outs}
    })
    // .inspect(|s| println!("{}", s))
    .collect::<Vec<Device>>()
}

//////////////////////////////////////////////////////////////
///                    Part 1
//////////////////////////////////////////////////////////////

fn calculate_paths(devices: &HashMap<String, Device>) -> i32 {
    let start_device = devices.get("you").unwrap();
    calculate_paths_helper(devices, start_device)
}

fn calculate_paths_helper(devices: &HashMap<String, Device>, start_device: &Device) -> i32 {
    let mut paths_counter = 0;
    for device_out_name in start_device.outs.iter() {
        if device_out_name == "out" {
            paths_counter += 1;
        }
        else {
            let device_out = devices.get(device_out_name);
            if device_out.is_none() {
                println!("Device {} does not exist in map", device_out_name);
            }
            
            paths_counter += calculate_paths_helper(devices, device_out.unwrap());
        }
        
    };
    paths_counter
}

fn part1() {
    // let file_name = "input.txt";
    // let file_name = "input_simple.txt";
    let file_name = "input_full.txt";
    let devices = parse_input(file_name);
    let devices = devices.into_iter().map(|item| (item.name.clone(), item)).collect::<HashMap<String, Device>>();
    
    let paths = calculate_paths(&devices);
    println!("paths: {:?}", paths);
}


//////////////////////////////////////////////////////////////
///                    Part 2
//////////////////////////////////////////////////////////////
fn part2() {
}
