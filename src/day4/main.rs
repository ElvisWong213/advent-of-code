use core::panic;

use advent_of_code::open_file;

fn main() {
    let content = open_file::open("./src/day4/input.txt");

    println!("{:}", part_one(&content));
    println!("{:}", part_two(&content));
}

fn part_one(content: &str) -> u32 {
    let mut list_2d: Vec<Vec<char>> = vec![];

    for line in content.lines() {
        let mut list: Vec<char> = vec![];
        for c in line.chars() {
            list.push(c);
        }
        list_2d.push(list);
    }

    let mut count: u32 = 0;
    let target = "XMAS";
    for y in 0..list_2d.len() {
        for x in 0..list_2d[0].len() {
            let point: Point = Point::new(x as i32, y as i32);
            count += find_horizontal(&list_2d, &point, target) as u32;
            count += find_vertical(&list_2d, &point, target) as u32;
            count += find_diagonal(&list_2d, &point, target) as u32;
        }
    }
    count
}

fn part_two(content: &str) -> u32 {
    let mut list_2d: Vec<Vec<char>> = vec![];

    for line in content.lines() {
        let mut list: Vec<char> = vec![];
        for c in line.chars() {
            list.push(c);
        }
        list_2d.push(list);
    }

    let mut count: u32 = 0;
    let target = "MAS";
    for y in 0..list_2d.len() {
        for x in 0..list_2d[0].len() {
            let point: Point = Point::new(x as i32, y as i32);
            if find_x(&list_2d, &point, target) == 2 {
                count += 1;
            }
        }
    }
    count
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        if x < 0 || y < 0 {
            panic!("Invalid input");
        }
        Self { x , y }
    }
}

fn find_horizontal(list_2d: &[Vec<char>], start: &Point, target: &str) -> u8 {
    let mut count: u8 = 0;
    let mut buffer: String = String::new();
    let max_y: usize = list_2d.len();
    if max_y == 0 {
        panic!("list is empty");
    }
    let max_x: usize = list_2d[0].len();
    if max_x == 0 {
        panic!("list is empty");
    }

    // forward
    for x_index in start.x..start.x + 4 {
        if x_index as usize >= max_x {
            break;
        }
        buffer.push(list_2d[start.y as usize][x_index as usize]);
    }

    if is_xmas(&buffer, target) {
        count += 1;
    }
    buffer.clear();

    // backward
    for x_index in (start.x - 3..=start.x).rev() {
        if x_index < 0 {
            break;
        }
        buffer.push(list_2d[start.y as usize][x_index as usize]);
    }

    if is_xmas(&buffer, target) {
        count += 1;
    }

    count
}

fn find_vertical(list_2d: &[Vec<char>], start: &Point, target: &str) -> u8 {
    let mut count: u8 = 0;
    let mut buffer: String = String::new();
    let max_y: usize = list_2d.len();
    if max_y == 0 {
        panic!("list is empty");
    }
    if list_2d[0].is_empty() {
        panic!("list is empty");
    }

    // down
    for y_index in start.y..start.y + 4 {
        if y_index as usize >= max_y {
            break;
        }
        buffer.push(list_2d[y_index as usize][start.x as usize]);
    }

    if is_xmas(&buffer, target) {
        count += 1;
    }
    buffer.clear();

    // up
    for y_index in (start.y - 3..=start.y).rev() {
        if y_index < 0 {
            break;
        }
        buffer.push(list_2d[y_index as usize][start.x as usize]);
    }

    if is_xmas(&buffer, target) {
        count += 1;
    }

    count
}

fn find_diagonal(list_2d: &[Vec<char>], start: &Point, target: &str) -> u8 {
    let mut count: u8 = 0;
    let mut buffer: String = String::new();
    let max_y: usize = list_2d.len();
    if max_y == 0 {
        panic!("list is empty");
    }
    let max_x: usize = list_2d[0].len();
    if max_x == 0 {
        panic!("list is empty");
    }

    // right down
    for step in 0..4 {
        let y_index = start.y + step;
        let x_index = start.x + step;
        if y_index as usize >= max_y || x_index as usize >= max_x {
            break;
        }
        buffer.push(list_2d[y_index as usize][x_index as usize]);
    }

    if is_xmas(&buffer, target) {
        count += 1;
    }
    buffer.clear();

    // right up
    for step in 0..4 {
        let y_index = start.y - step;
        let x_index = start.x + step;
        if y_index as usize >= max_y || x_index as usize >= max_x {
            break;
        }
        buffer.push(list_2d[y_index as usize][x_index as usize]);
    }

    if is_xmas(&buffer, target) {
        count += 1;
    }
    buffer.clear();

    // left down
    for step in 0..4 {
        let y_index = start.y + step;
        let x_index = start.x - step;
        if y_index as usize >= max_y || x_index as usize >= max_x {
            break;
        }
        buffer.push(list_2d[y_index as usize][x_index as usize]);
    }

    if is_xmas(&buffer, target) {
        count += 1;
    }
    buffer.clear();

    // left up
    for step in 0..4 {
        let y_index = start.y - step;
        let x_index = start.x - step;
        if y_index as usize >= max_y || x_index as usize >= max_x {
            break;
        }
        buffer.push(list_2d[y_index as usize][x_index as usize]);
    }

    if is_xmas(&buffer, target) {
        count += 1;
    }

    count
}

fn find_x(list_2d: &[Vec<char>], center: &Point, target: &str) -> u8 {
    let mut count: u8 = 0;
    let mut buffer: String = String::new();
    let max_y: usize = list_2d.len();
    if max_y == 0 {
        panic!("list is empty");
    }
    let max_x: usize = list_2d[0].len();
    if max_x == 0 {
        panic!("list is empty");
    }

    for step in -1..=1 {
        let y_index = center.y + step;
        let x_index = center.x + step;
        if !(0..max_y as i32).contains(&y_index) || !(0..max_x as i32).contains(&x_index) {
            continue;
        }
        buffer.push(list_2d[y_index as usize][x_index as usize]);
    }

    if is_xmas(&buffer, target) {
        count += 1;
    } else {
        buffer = buffer.chars().rev().collect();

        if is_xmas(&buffer, target) {
            count += 1;
        }
    }
    buffer.clear();

    for step in -1..=1 {
        let y_index = center.y - step;
        let x_index = center.x + step;
        if !(0..max_y as i32).contains(&y_index) || !(0..max_x as i32).contains(&x_index) {
            continue;
        }
        buffer.push(list_2d[y_index as usize][x_index as usize]);
    }

    if is_xmas(&buffer, target) {
        count += 1;
    } else {
        buffer = buffer.chars().rev().collect();

        if is_xmas(&buffer, target) {
            count += 1;
        }
    }
    count
}

fn is_xmas(input: &str, target: &str) -> bool {
    input == target
}

#[test]
fn test_part_one() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    assert_eq!(18, part_one(input));
}

#[test]
fn test_part_two() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    assert_eq!(9, part_two(input));
}
