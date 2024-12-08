use core::panic;
use std::char;

use advent_of_code::open_file;

fn main() {
    let content = open_file::open("./src/day7/input.txt");

    println!("{:}", part_one(&content));
    println!("{:}", part_two(&content));
}

fn part_one(content: &str) -> u64 {
    let mut value_list: Vec<Value> = vec![];
    for line in content.lines() {
        let mut value: Value = Value::new();
        let split_line: Vec<&str> = line.split(':').collect();
        if split_line.len() != 2 {
            panic!("invalid inut");
        }
        value.test = split_line[0].parse().unwrap();
        for n in split_line[1].split_whitespace() {
            value.numbers.push(n.parse().unwrap());
        }
        value_list.push(value);
    }

    let mut total: u64 = 0;
    for case in value_list {
        let remain = case.test;
        if is_equal_remain(remain, &case.numbers, case.numbers.len() - 1) {
            total += case.test;
        }
    }
    total
}

fn part_two(content: &str) -> u64 {
    let mut value_list: Vec<Value> = vec![];
    for line in content.lines() {
        let mut value: Value = Value::new();
        let split_line: Vec<&str> = line.split(':').collect();
        if split_line.len() != 2 {
            panic!("invalid inut");
        }
        value.test = split_line[0].parse().unwrap();
        for n in split_line[1].split_whitespace() {
            value.numbers.push(n.parse().unwrap());
        }
        value_list.push(value);
    }

    let mut total: u64 = 0;
    for case in value_list {
        let remain = case.test;
        if is_equal_remain_p2(remain, &case.numbers, case.numbers.len() - 1) {
            total += case.test;
        }
    }
    total
}

fn is_equal_remain(remain: u64, numbers: &[u64], index: usize) -> bool {
    let number = numbers[index];
    if number > remain {
        return false;
    }
    if index == 0 {
        return (remain % number == 0 && remain / number == 0) || remain == number;
    }
    if remain % number == 0 && is_equal_remain(remain / number, numbers, index - 1) {
        true
    } else {
        is_equal_remain(remain - number, numbers, index - 1)
    }
}

fn is_equal_remain_p2(remain: u64, numbers: &[u64], index: usize) -> bool {
    let number = numbers[index];
    if number > remain {
        return false;
    }
    if index == 0 {
        return (remain % number == 0 && remain / number == 0) || remain == number;
    }
    if (remain % number == 0 && is_equal_remain_p2(remain / number, numbers, index - 1)) || is_equal_remain_p2(remain - number, numbers, index - 1) {
        return true;
    } 
    let mut remain_char: Vec<char> = remain.to_string().chars().collect();
    let number_char: Vec<char> = number.to_string().chars().collect();
    let mut same: bool = true;
    if remain_char.len() < number_char.len() {
        return false;
    }
    for index in (0..number_char.len()).rev() {
        if number_char[index] != remain_char.pop().unwrap() {
            same = false;
            break;
        }
    }
    if same {
        let mut new_remain: String = String::new();
        for c in &remain_char {
            new_remain.push(*c);
        }
        if remain_char.is_empty() {
            new_remain = "1".to_string();
        }
        is_equal_remain_p2(new_remain.parse().unwrap(), numbers, index - 1)
    } else {
        false
    }
}

struct Value {
    test: u64,
    numbers: Vec<u64>
}

impl Value {
    fn new() -> Self {
        Self { test: 0, numbers: vec![] }
    }
}

#[test]
fn test_part_one() {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
    assert_eq!(3749, part_one(input));
}

#[test]
fn test_part_one_a() {
    assert_eq!(48033216, part_one("48033216: 1 11 2 4 11 6 8 266 19"));
}

#[test]
fn test_part_two() {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
    assert_eq!(11387, part_two(input));
}

#[test]
fn test_part_two_a() {
    let input = "618: 3 100 456 57 5";
    assert_eq!(0, part_two(input));
}
