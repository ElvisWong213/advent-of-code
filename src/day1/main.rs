use std::collections::HashMap;

use advent_of_code::open_file;

fn main() {
    let content = open_file::open("./src/day1/input.txt");

    println!("{:}", part_one(&content));
    println!("{:}", part_two(&content));
}

fn part_one(content: &str) -> u32 {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    for (index, number) in content.split_whitespace().enumerate() {
        if index % 2 == 0 {
            left.push(number.parse().unwrap());
        } else {
            right.push(number.parse().unwrap());
        }
    }
    right.sort();
    left.sort();

    if right.len() != left.len() {
        println!("Array size are different");
        return 0;
    }

    let mut ans: u32 = 0;
    for index in 0..right.len() {
        let left_num = left[index];
        let right_num = right[index];
        if left_num > right_num {
            ans += left_num - right_num;
        } else {
            ans += right_num - left_num;
        }
    }

    ans
}

fn part_two(content: &str) -> u32 {
    let mut left: Vec<u32> = vec![];
    let mut right: HashMap<u32, u32> = HashMap::new();

    for (index, number) in content.split_whitespace().enumerate() {
        if index % 2 == 0 {
            left.push(number.parse().unwrap());
        } else {
            let right_num: u32 = number.parse().unwrap();
            match right.get(&right_num) {
                Some(count) => {
                    right.insert(right_num, count + 1);
                }
                None => {
                    right.insert(right_num, 1);
                }
            }
        }
    }

    let mut ans: u32 = 0;
    for left_num in left {
        if let Some(count) = right.get(&left_num) {
            ans += left_num * count;
        }
    }

    ans
}
