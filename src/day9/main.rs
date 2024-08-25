use advent_of_code::open_file;

fn main() {
    let content = open_file::open("./src/day9/input.txt");

    println!("{}", part_one(&content));
    println!("{}", part_two(&content));
}

fn part_one(input: &str) -> i32 {
    let mut ans: i32 = 0;

    let lines = input.lines();
    for line in lines {
        let mut nums: Vec<i32> = vec![];
        for num_string in line.split_whitespace() {
            let num: i32 = num_string.parse().unwrap();
            nums.push(num);
        }
        ans += find_next_val(nums);
    }
    ans
}

fn part_two(input: &str) -> i32 {
    let mut ans: i32 = 0;

    let lines = input.lines();
    for line in lines {
        let mut nums: Vec<i32> = vec![];
        for num_string in line.split_whitespace() {
            let num: i32 = num_string.parse().unwrap();
            nums.push(num);
        }
        ans += find_next_val_part_2(nums);
    }
    ans
}

fn find_next_val(nums: Vec<i32>) -> i32 {
    let mut diffs: Vec<i32> = vec![];
    let mut is_same: bool = true;
    let mut last_diff: i32 = 0;
    for i in 0..nums.len() - 1 {
        let diff = nums[i + 1] - nums[i];
        diffs.push(diff);
        if i != 0 && diff != last_diff {
            is_same = false;
        }
        last_diff = diff;
    }
    if !is_same {
        return nums.last().unwrap() + find_next_val(diffs);
    }
    nums.last().unwrap() + last_diff
}

fn find_next_val_part_2(nums: Vec<i32>) -> i32 {
    let mut diffs: Vec<i32> = vec![];
    let mut is_same: bool = true;
    let mut last_diff: i32 = 0;
    for i in 0..nums.len() - 1 {
        let diff = nums[i + 1] - nums[i];
        diffs.push(diff);
        if i != 0 && diff != last_diff {
            is_same = false;
        }
        last_diff = diff;
    }
    if !is_same {
        return nums.first().unwrap() - find_next_val_part_2(diffs);
    }
    nums.first().unwrap() - diffs.first().unwrap()
}

#[test]
fn test_part_one() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    assert_eq!(part_one(input), 114);
}

#[test]
fn test_part_two() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    assert_eq!(part_two(input), 2);
}

#[test]
fn test_find_next_val_a() {
    let array: Vec<i32> = vec![0, 3, 6, 9, 12, 15];
    assert_eq!(find_next_val(array), 18);
}

#[test]
fn test_find_next_val_b() {
    let array: Vec<i32> = vec![1, 3, 6, 10, 15, 21];
    assert_eq!(find_next_val(array), 28);
}

#[test]
fn test_find_next_val_part_two_a() {
    let array: Vec<i32> = vec![0, 3, 6, 9, 12, 15];
    assert_eq!(find_next_val_part_2(array), -3);
}

#[test]
fn test_find_next_val_part_two_b() {
    let array: Vec<i32> = vec![1, 3, 6, 10, 15, 21];
    assert_eq!(find_next_val_part_2(array), 0);
}

#[test]
fn test_find_next_val_part_two_c() {
    let array: Vec<i32> = vec![10, 13, 16, 21, 30, 45];
    assert_eq!(find_next_val_part_2(array), 5);
}
