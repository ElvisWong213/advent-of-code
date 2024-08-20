use std::collections::HashMap;

use advent_of_code::open_file;

fn main() {
    let content = open_file::open("./src/day8/input.txt");

    println!("{}", part_one(&content));
    println!("{}", part_two(&content));
}

fn part_one(input: &str) -> u16 {
    let mut node_set: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut instruction: String = String::new();
    let lines = input.lines();
    let mut current_val: &str = "";

    for (line_index, line) in lines.enumerate() {
        if line_index == 0 {
            instruction = line.to_string();
            continue;
        }
        let nodes = line.split(" = ");
        let mut key: &str = "";
        for (node_index, node) in nodes.enumerate() {
            if node_index == 0 {
                key = node;
                if current_val.is_empty() {
                    current_val = node;
                }
                continue;
            }
            let values: Vec<&str> = node.split(", ").collect();
            let value = (values[0].trim_matches('('), values[1].trim_matches(')'));
            node_set.insert(key, value);
        }
    }

    let mut step: u16 = 0;
    while current_val != "ZZZ" {
        for c in instruction.chars() {
            if c == 'L' {
                current_val = node_set.get(current_val).unwrap().0;
            }
            if c == 'R' {
                current_val = node_set.get(current_val).unwrap().1;
            }
            step += 1;
        }
    }
    step
}

fn part_two(input: &str) -> u64 {
    let mut node_set: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut instruction: String = String::new();
    let lines = input.lines();
    let mut current_vals: Vec<&str> = vec![];

    for (line_index, line) in lines.enumerate() {
        if line_index == 0 {
            instruction = line.to_string();
            continue;
        }
        let nodes = line.split(" = ");
        let mut key: &str = "";
        for (node_index, node) in nodes.enumerate() {
            if node_index == 0 {
                key = node;
                if node.ends_with('A') {
                    current_vals.push(node);
                }
                continue;
            }
            let values: Vec<&str> = node.split(", ").collect();
            let value = (values[0].trim_matches('('), values[1].trim_matches(')'));
            node_set.insert(key, value);
        }
    }

    let mut ans: u64 = 1;
    let mut steps: Vec<u32> = vec![];
    for val in current_vals {
        let mut step: u32 = 0;
        let mut current_val = val;
        while !current_val.ends_with('Z') {
            for c in instruction.chars() {
                if c == 'L' {
                    current_val = node_set.get(current_val).unwrap().0;
                }
                if c == 'R' {
                    current_val = node_set.get(current_val).unwrap().1;
                }
                step += 1;
            }
        }
        steps.push(step);
    }
    for i in steps {
        ans = find_lcm(ans, i as u64, 2);
    }
    ans
}

fn find_lcm(a: u64, b: u64, start: u64) -> u64 {
    if start > a || start > b {
        return a * b;
    }
    let new_a = a / start;
    let re_a = a % start;
    let new_b = b / start;
    let re_b = b % start;
    if re_a != 0 || re_b != 0 {
        return find_lcm(a, b, start + 1);
    }
    start * find_lcm(new_a, new_b, start)
}

#[test]
fn test_part_one_a() {
    let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    assert_eq!(part_one(input), 2);
}

#[test]
fn test_part_one_b() {
    let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    assert_eq!(part_one(input), 6);
}

#[test]
fn test_part_two_a() {
    let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    assert_eq!(part_two(input), 6);
}

#[test]
fn test_find_lcm_a() {
    assert_eq!(find_lcm(6, 12, 2), 12);
}

#[test]
fn test_find_lcm_b() {
    assert_eq!(find_lcm(11, 12, 2), 132);
}
