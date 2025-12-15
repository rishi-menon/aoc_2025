use std::{collections::HashMap, fs};

use itertools::Itertools;

fn main() {
    println!("Hello, world!");
    // part1();
    part2();
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
fn calculate_paths_2(devices: &HashMap<String, Device>) -> i64 {
    let start_device = devices.get("svr").unwrap();
    let mut map = HashMap::new();
    calculate_paths_helper_2(devices, start_device, &mut map, false, false)
}

fn calculate_paths_helper_2(devices: &HashMap<String, Device>, start_device: &Device, map: &mut HashMap<(String, bool, bool),  i64>, has_dac: bool, has_fft: bool) -> i64 {
    // println!("--- device {}", start_device.name);
    
    let has_dac = has_dac || (start_device.name == "dac");
    let has_fft = has_fft || (start_device.name == "fft");
    
    if let Some(value) = map.get(&(start_device.name.clone(), has_dac, has_fft)) {
        return *value;
    }

    let mut paths_counter = 0i64;
    for device_out_name in start_device.outs.iter() {
        if device_out_name == "out" {
            println!("Reached out: {}, {}", has_dac, has_fft);
            if has_dac && has_fft {
                paths_counter += 1;
            }
        }
        else {
            let device_out = devices.get(device_out_name);
            if device_out.is_none() {
                println!("Device {} does not exist in map", device_out_name);
            }
            paths_counter += calculate_paths_helper_2(devices, device_out.unwrap(), map, has_dac, has_fft);
        }
    };

    map.insert((start_device.name.clone(), has_dac, has_fft), paths_counter);
    println!("--- device {}, count: {}", start_device.name, paths_counter);
    
    paths_counter
}
fn part2() {
    // let file_name = "input.txt";
    // let file_name = "input_simple.txt";
    let file_name = "input_full.txt";
    let devices = parse_input(file_name);
    let devices = devices.into_iter().map(|item| (item.name.clone(), item)).collect::<HashMap<String, Device>>();
    
    let paths = calculate_paths_2(&devices);
    println!("paths: {:?}", paths);
}

