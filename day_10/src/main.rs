use std::collections::{BTreeMap, BinaryHeap, HashMap};
use std::{collections::HashSet, fs, ops::Index};
use std::ops::BitXor;
use std::cmp::Ordering;

fn main() {
    // part1();
    part2();
}

////////////////////////////////////////////////////////////////
// Part 1
////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Machine {
    target: u64,
    buttons: Vec<u64>,
    joltages: Vec<i32>,
}

fn parse_file(file_path: &str) -> Vec<Machine> {
    let machines = fs::read_to_string(file_path).unwrap().lines().map(|line| {
        let words = line.split_whitespace().map(|word| word).collect::<Vec<&str>>();
        
        let indicator = words[0];
        let joltage = words[words.len() - 1];
        // Parse buttons
        let buttons = (1..words.len()-1).map(|i| {
            let word = words[i];
            assert_eq!(*word.chars().nth(0).iter().next().unwrap(), '(');
            assert_eq!(*word.chars().nth(word.len() - 1).iter().next().unwrap(), ')');
            let button_indices = word[1..word.len()-1].split(',').map(|c| c.parse::<u8>().unwrap()).collect::<Vec<u8>>();
            
            let mut button_value = 0u64;
            for &index in button_indices.iter() {
                assert!(index < 63u8, "index value {} is too large", index);
                button_value |= (1u64 << index);
            }
            button_value
        }).collect::<Vec<u64>>();

        // Parse target  
        assert_eq!(*indicator.chars().nth(0).iter().next().unwrap(), '[');
        assert_eq!(*indicator.chars().nth(indicator.len() - 1).iter().next().unwrap(), ']');
        let mut indicator_value = 0u64;
        for (i, indicator) in indicator[1..indicator.len()-1].chars().enumerate() {
            assert!(i < 63);
            if indicator == '#' {
                indicator_value |= (1u64 << i)
            }
        }
        // Parse joltages
        assert_eq!(*joltage.chars().nth(0).iter().next().unwrap(), '{');
        assert_eq!(*joltage.chars().nth(joltage.len() - 1).iter().next().unwrap(), '}');
        
        let joltage_values = joltage[1..joltage.len()-1].split(',').map(|w| w.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        // Print stuff
        // println!("button");
        // for i in buttons.iter() {
        //     println!("{:?}", i);
        // }
        // println!("target: {}", indicator_value);
        // println!("joltages: {:?}", joltage_values);
        Machine {
            target: indicator_value,
            buttons: buttons,
            joltages: joltage_values,
        }
    }).collect::<Vec<Machine>>();
    machines
}

fn calc_min_button_presses(machine: &Machine) -> i32 {
    let mut prev_states = HashSet::new();
    prev_states.insert(0);

    for num_presses in (1..10000) {
        
        let mut cur_states = HashSet::new();
        for &prev_state in prev_states.iter() {
            for &button in machine.buttons.iter() {
                let cur_state = prev_state ^ button;
                if cur_state == machine.target {
                    return num_presses
                }
                cur_states.insert(cur_state);
            }
        }
        prev_states = cur_states;
    }
    panic!("We should never reach here. If we reach here then increase the for loop range");
}

fn part1() {
    println!("Hello, world!");
    let file_path = "input_full.txt";
    // let file_path = "input_simple.txt";
    let machines = parse_file(file_path);

    let mut result = 0;
    for machine in machines.iter() {
        let value = calc_min_button_presses(machine);
        result += value;

        println!("{:?} --> {}", machine, value);
    }

    println!("final result: {}", result);

}

////////////////////////////////////////////////////////////////
// Part 2
////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Machine2 {
    buttons: Vec<MachineState>,
    joltage: MachineState,
    num_joltage: i32,
}

const COUNTERS: usize = 11;
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct MachineState {
    counter: [i32; COUNTERS],
}

impl MachineState {
    fn is_invalid(&self) -> bool {
        return self.counter.iter().any(|&x| x < 0);
    }

    fn combine_state(state: &MachineState, button: &MachineState) -> MachineState {
        let mut new_counter = [0; COUNTERS];
        for i in 0..COUNTERS {
            new_counter[i] = state.counter[i] - button.counter[i];
        }
        MachineState { counter: new_counter }
    }
    fn get_cost(&self) -> i32 {
        let mut cost = 0;
        for i in self.counter {
            cost += i;
        }
        cost
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct HeapItem {
    num_steps: i32,
    state: MachineState,
}

impl HeapItem {
    fn new(num_steps: i32, state: MachineState) -> Self {
        HeapItem {
            num_steps,
            state,
        }
    }
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some( self.cmp(other) )
    }
}
impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        // let cost_a = self.state.get_cost();
        // let cost_b = other.state.get_cost();
        // cost_b.cmp(&cost_a)
        
        // For breadth first search
        other.num_steps.cmp(&self.num_steps)
    }
}

fn parse_file_2(file_path: &str) -> Vec<Machine2> {
    let machines = fs::read_to_string(file_path).unwrap()
    .lines()
    .filter(|&x| !x.starts_with("#"))
    .map(|line| {
        let words = line.split_whitespace().map(|word| word).collect::<Vec<&str>>();
        
        // let indicator = words[0];
        
        // Joltages
        let (joltage, num_joltage) = {
            let joltage = words[words.len() - 1];
            // Parse joltages
            assert_eq!(*joltage.chars().nth(0).iter().next().unwrap(), '{');
            assert_eq!(*joltage.chars().nth(joltage.len() - 1).iter().next().unwrap(), '}');        
            let joltage_values = joltage[1..joltage.len()-1].split(',').map(|w| w.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            let num_joltage = joltage_values.len() as i32;
            assert!(num_joltage <= COUNTERS as i32);

            let mut joltages_array = [0; COUNTERS];
            for i in 0..num_joltage as usize {
                joltages_array[i] = joltage_values[i];
            }
            
            (MachineState { counter: joltages_array }, num_joltage)
        };
        
        // Parse buttons
        let buttons = {
            (1..words.len()-1).map(|i| {
                let word = words[i];
                assert_eq!(*word.chars().nth(0).iter().next().unwrap(), '(');
                assert_eq!(*word.chars().nth(word.len() - 1).iter().next().unwrap(), ')');
                let button_indices = word[1..word.len()-1].split(',').map(|c| c.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                
                let mut button_state = [0; COUNTERS];
                for index in button_indices {
                    assert!(index < COUNTERS as i32);
                    button_state[index as usize] = 1;
                }
                MachineState { counter: button_state }
            }).collect::<Vec<MachineState>>()
        };

        // Print stuff
        // println!("button");
        // for i in buttons.iter() {
        //     println!("{:?}", i);
        // }
        // println!("target: {}", indicator_value);
        // println!("joltages: {:?}", joltage_values);
        Machine2 {
            buttons: buttons,
            joltage: joltage,
            num_joltage: num_joltage,
        }
    }).collect::<Vec<Machine2>>();
    machines
}

fn calc_joltage(machine: &Machine2) -> i32 {
    map = HashMap::new();
    calc_joltage_helper(machine, &mut map, &machine.joltage, 0);
}
fn calc_joltage_helper(machine: &Machine2, map: &mut HashMap<MachineState, i32>, target: &MachineState, step: i32) -> Option<i32> {

    


}
// fn calc_joltage(machine: &Machine2) -> i32 {
//     let mut priority_queue = BinaryHeap::new();
//     priority_queue.push(HeapItem::new(0, machine.joltage.clone()) );

//     let mut all_set: HashSet<HeapItem> = HashSet::new();
    
//     let mut debug_counter = 0;


//     while !priority_queue.is_empty() {
//         let item = priority_queue.pop().unwrap();
//         assert!(!item.state.is_invalid());
//         // println!("Processing {:?} (cost: {})", item.state, item.state.get_cost());
//         println!("Processing {:?} (step: {})", item.state, item.num_steps);

//         if item.state.counter.iter().all(|&x| x == 0) {
//             return item.num_steps;
//         }
        
        
//         if all_set.contains(&item) {
//             let seen_item = all_set.get(&item).unwrap();
//             assert!(seen_item.num_steps <= item.num_steps, "already seen: {}, current: {}", seen_item.num_steps, item.num_steps);
//             continue;
//         }
//         else {
//             all_set.insert(item.clone());
//         }

//         for button in machine.buttons.iter() {
//             let new_state = MachineState::combine_state(&item.state, button);
            
//             if !new_state.is_invalid() {
//                 // println!("  -- Pushing {:?} (cost: {})", new_state, new_state.get_cost());
//                 // println!("  -- Pushing {:?} (step: {})", new_state, item.num_steps + 1);
//                 priority_queue.push(
//                     HeapItem { num_steps: item.num_steps + 1, state: new_state }
//                 );                
//             }
//         }

//         debug_counter += 1;

//         if debug_counter >= 20 {
//             // break;
//         }
//     }
//     panic!("This should never happen")
// }
fn part2() {
    println!("Hello, world!");
    let file_path = "input.txt";
    // let file_path = "input_full.txt";
    // let file_path = "input_simple.txt";
    let machines = parse_file_2(file_path);

    let mut result = 0;
    for machine in machines.iter() {
        let value = calc_joltage(machine);
        result += value;

        println!("{:?} --> {}", machine, value);
    }

    println!("Final result: {}", result);

}