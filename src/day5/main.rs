use std::collections::HashMap;

use advent_of_code::open_file;

fn main() {
    let content = open_file::open("./src/day5/input.txt");

    println!("{:}", part_one(&content));
    println!("{:}", part_two(&content));
}

fn part_one(content: &str) -> u32 {
    let mut rules: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut page_numbers: Vec<Vec<u8>> = vec![];
    let mut is_rule: bool = true;

    for line in content.lines() {
        if line.is_empty() {
            is_rule = false;
            continue;
        }
        if is_rule {
            set_rules(&mut rules, line);
        } else {
            set_page_numbers(&mut page_numbers, line);
        }
    }
    
    let mut ans: u32 = 0;
    for numbers in page_numbers {
        let size = numbers.len();
        if is_correct(&numbers, &rules) {
            ans += numbers[size / 2] as u32;
        }
    }
    ans
}

fn part_two(content: &str) -> u32 {
    let mut rules: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut page_numbers: Vec<Vec<u8>> = vec![];
    let mut is_rule: bool = true;

    for line in content.lines() {
        if line.is_empty() {
            is_rule = false;
            continue;
        }
        if is_rule {
            set_rules(&mut rules, line);
        } else {
            set_page_numbers(&mut page_numbers, line);
        }
    }
    
    let mut ans: u32 = 0;
    for numbers in &mut page_numbers {
        let size = numbers.len();
        if !is_correct(numbers, &rules) {
            ans += correct_numbers(numbers, &rules)[size / 2] as u32;
        }
    }
    ans
}

fn set_rules(rules: &mut HashMap<u8, Vec<u8>>, input: &str) {
    let data: Vec<&str> = input.split('|').collect();
    if data.len() != 2 {
        panic!("rules format not correct");
    }
    let key: u8 = data[0].parse().unwrap();
    let val: u8 = data[1].parse().unwrap();
    match rules.get(&key) {
        Some(list) => {
            let mut new_list = list.clone();
            new_list.push(val);
            rules.insert(key, new_list);
        }
        None => {
            let list: Vec<u8> = Vec::from([val]);
            rules.insert(key, list);
        }
    }
}

fn set_page_numbers(page_numbers: &mut Vec<Vec<u8>>, input: &str) {
    let mut numbers: Vec<u8> = vec![];
    for number in input.split(',') {
        numbers.push(number.parse::<u8>().unwrap());
    }
    page_numbers.push(numbers);
}

fn is_correct(numbers: &[u8], rules: &HashMap<u8, Vec<u8>>) -> bool {
    let mut visit: Vec<u8> = vec![];
    for number in numbers {
        if let Some(list) = rules.get(number) {
            for num in list {
                if visit.contains(num) {
                    return false;
                }
            }
        }
        visit.push(*number);
    }
    true
}

fn correct_numbers(numbers: &[u8], rules: &HashMap<u8, Vec<u8>>) -> Vec<u8> {
    let mut visit: Vec<u8> = vec![];
    for number in numbers {
        if let Some(list) = rules.get(number) {
            let mut inserted: bool = false;
            for (index, visit_num) in visit.iter().enumerate() {
                if list.contains(visit_num) {
                    visit.insert(index, *number);
                    inserted = true;
                    break;
                }
            }
            if !inserted {
                visit.push(*number);
            }
        } 
    }
    visit
}

#[test]
fn test_part_one() {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
    assert_eq!(143, part_one(input));
}

#[test]
fn test_part_two() {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
    assert_eq!(123, part_two(input));
}
