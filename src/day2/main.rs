use advent_of_code::open_file;

fn main() {
    let content = open_file::open("./src/day2/input.txt");

    println!("{:}", part_one(&content));
    println!("{:}", part_two(&content));
}

fn part_one(content: &str) -> u32 {
    let mut safe_count: u32 = 0;
    for line in content.lines() {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        let mut safe: bool = true;
        let mut state: State = State::NotSet;
        for index in 0..numbers.len() - 1 {
            let a: i32 = numbers[index].parse().unwrap();
            let b: i32 = numbers[index + 1].parse().unwrap();
            let diff = a - b;
            if diff > 3 || diff < -3 || diff == 0 {
                safe = false;
                break;
            }
            if index == 0 {
                state = State::new_state(diff);
            } else if state != State::new_state(diff) {
                    safe = false;
                    break;
            }
        }
        if safe {
            safe_count += 1;
        }
    }
    safe_count
}

fn part_two(content: &str) -> u32 {
    let mut safe_count: u32 = 0;
    for line in content.lines() {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        let mut safe: bool = true;
        let mut state: State = State::NotSet;
        for index in 0..numbers.len() - 1 {
            let a: i32 = numbers[index].parse().unwrap();
            let b: i32 = numbers[index + 1].parse().unwrap();
            let diff = a - b;
            if diff > 3 || diff < -3 || diff == 0 {
                safe = false;
                break;
            }
            if index == 0 {
                state = State::new_state(diff);
            } else if state != State::new_state(diff) {
                    safe = false;
                    break;
            }
        }
        if safe {
            safe_count += 1;
        } else {
            for index in 0..numbers.len() {
                if is_safe(&numbers, index) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }
    safe_count
}

fn is_safe(numbers: &[&str], ignore_index: usize) -> bool {
    let mut safe: bool = true;
    let mut state: State = State::NotSet;
    for index in 0..numbers.len() - 1 {
        let a_index = index;
        let mut b_index = index + 1;
        if a_index == ignore_index {
            continue;
        }
        if b_index == ignore_index {
            b_index += 1;
        }
        if b_index >= numbers.len() {
            return safe;
        }
        let a: i32 = numbers[a_index].parse().unwrap();
        let b: i32 = numbers[b_index].parse().unwrap();
        let diff = a - b;
        if diff > 3 || diff < -3 || diff == 0 {
            safe = false;
            break;
        }
        if state == State::NotSet {
            state = State::new_state(diff);
        } else if state != State::new_state(diff) {
            safe = false;
            break;
        }
    }
    safe
}

#[derive(PartialEq)]
enum State {
    Increase,
    Decrease,
    NotSet,
}

impl State {
    fn new_state(diff: i32) -> Self {
        if diff > 0 {
            return State::Increase;
        } 
        if diff < 0 {
            return State::Decrease;
        }
        State::NotSet
    }
}

#[test]
fn test_part_one() {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
    assert_eq!(2, part_one(input));
}

#[test]
fn test_part_two() {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
    assert_eq!(4, part_two(input));
}

#[test]
fn test_part_two_a() {
    assert_eq!(0, part_two("1 1 1 1 1"));
    assert_eq!(0, part_two("1 1 2 2 3"));
    assert_eq!(0, part_two("1 2 8 9 5"));
    assert_eq!(0, part_two("1 9 1 9 5"));
}

#[test]
fn test_part_two_b() {
    assert_eq!(1, part_two("1 9 3 4 5"));
    assert_eq!(1, part_two("1 2 3 4 9"));
    assert_eq!(1, part_two("1 5 6 7 8"));
    assert_eq!(1, part_two("9 2 3 4 5"));
    assert_eq!(1, part_two("9 8 7 6 5"));
    assert_eq!(1, part_two("9 8 7 6 9"));
    assert_eq!(1, part_two("1 2 7 4 5"));
}
