use std::collections::HashMap;

use advent_of_code::open_file;

fn main() {
    let content = open_file::open("./src/day3/input.txt");
    // println!("{}", content);
    let mut blueprint: BluePrint = BluePrint::new();
    blueprint.load_data(&content);
    println!("{}", part_one(&blueprint));
    println!("{}", part_two(&blueprint));
}

fn part_one(blueprint: &BluePrint) -> u32 {
    let mut ans: u32 = 0;

    for num in &blueprint.number_array {
        for coordinate in &num.1 {
             if blueprint.is_adjacent_to_symbol(&coordinate) == true {
                 ans += num.0 as u32;
                 break;
             }
        }
    }
    return ans;
}

fn part_two(blueprint: &BluePrint) -> u32 {
    let mut ans: u32 = 0;

    for coordinate in &blueprint.gear_array {
        ans += blueprint.multiply_two_adjacent_numbers(coordinate);
    }
    
    return ans;
}

fn is_number(input: &char) -> bool {
    match input {
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
            return true;
        }
        _ => {
            return false;
        }
    }
}

fn is_symbol(input: &char) -> bool {
    if is_number(input) == false && *input != '.' {
        return true;
    }
    return false;
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn next_row(&mut self) {
        self.x = 0;
        self.y += 1;
    }
}

struct BluePrint {
    number_map: HashMap<Coordinate, u16>,
    symbol_map: HashMap<Coordinate, char>,
    number_array: Vec<(u16, Vec<Coordinate>)>,
    gear_array: Vec<Coordinate>
}

impl BluePrint {
    pub fn new() -> Self {
        Self { number_map: HashMap::new(), symbol_map: HashMap::new(), number_array: vec![], gear_array: vec![] }
    }

    pub fn load_data(&mut self, input: &String) {
        let mut coordinate = Coordinate::new(0, 0);
        let mut number_cache: String = String::new();
        let mut number_coordinates: Vec<Coordinate> = vec![];
        for c in input.chars() {
            self.load_number(&coordinate, &c, &mut number_cache, &mut number_coordinates);
            self.load_symbol(&coordinate, &c);
            coordinate.x += 1;
            if c == '\n' {
                coordinate.next_row();
            }
        }
    }

    fn load_number(&mut self, coordinate: &Coordinate, c: &char, number_cache: &mut String, number_coordinates: &mut Vec<Coordinate>) {
        if is_number(c) {
            number_cache.push(*c);
            number_coordinates.push(*coordinate);
            return;
        } 
        if number_cache.len() > 0 {
            let number: u16 = number_cache.parse().unwrap();
            for coordinate in &mut *number_coordinates {
                self.number_map.insert(*coordinate, number);
            }
            self.number_array.push((number, number_coordinates.to_vec()));
            number_cache.clear();
            number_coordinates.clear();
        }
    }

    fn load_symbol(&mut self, coordinate: &Coordinate, c: &char) {
        if is_symbol(c) {
            self.symbol_map.insert(*coordinate, *c);
        }
        if *c == '*' {
            self.gear_array.push(*coordinate);
        }
    }

    pub fn is_adjacent_to_symbol(&self, coordinate: &Coordinate) -> bool {
        let mut y_min = coordinate.y;
        if y_min != 0 {
            y_min -= 1;
        }
        let mut x_min = coordinate.x;
        if x_min != 0 {
            x_min -= 1;
        }
        for y_index in y_min..=coordinate.y + 1 {
            for x_index in x_min..=coordinate.x + 1 {
                if x_index == coordinate.x && y_index == coordinate.y {
                    continue;
                }
                let target_coordinate = Coordinate::new(x_index, y_index);
                match self.symbol_map.get(&target_coordinate) {
                    Some(_) => {
                        return true;
                    }
                    None => {}
                    
                }
            }
        }
        return false;
    }

    pub fn multiply_two_adjacent_numbers(&self, coordinate: &Coordinate) -> u32 {
        let mut count: u8 = 0;
        let mut output: u32 = 1;
        let mut y_min = coordinate.y;
        if y_min != 0 {
            y_min -= 1;
        }
        let mut x_min = coordinate.x;
        if x_min != 0 {
            x_min -= 1;
        }
        for y_index in y_min..=coordinate.y + 1 {
            let mut last_val: Option<u16> = None;
            for x_index in x_min..=coordinate.x + 1 {
                if x_index == coordinate.x && y_index == coordinate.y {
                    continue;
                }
                let target_coordinate = Coordinate::new(x_index, y_index);
                match self.number_map.get(&target_coordinate) {
                    Some(val) => {
                        if self.is_equal_last_val(&last_val, val) == true {
                            continue;
                        }
                        count += 1;
                        if count > 2 {
                            return 0;
                        }
                        output *= *val as u32;
                        last_val = Some(*val);
                    }
                    None => {
                        last_val = None;
                    }
                    
                }
            }
        }
        if count == 2 {
            return output;
        }
        return 0;
    }

    fn is_equal_last_val(&self, last_val: &Option<u16>, current_val: &u16) -> bool {
        match last_val {
            Some(last) => {
                if last == current_val {
                    return true;
                }
            }
            None => {}
        }
        return false;
    }
}

#[test]
fn test_part_1() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        .to_string();
    let mut bp = BluePrint::new();
    bp.load_data(&input);
    assert_eq!(part_one(&bp), 4361);
}

#[test]
fn test_part_1_2() {
    let input = "467..114..
...*......
..35...633
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        .to_string();
    let mut bp = BluePrint::new();
    bp.load_data(&input);
    assert_eq!(part_one(&bp), 4361);
}

#[test]
fn test_part_2() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        .to_string();
    let mut bp = BluePrint::new();
    bp.load_data(&input);
    assert_eq!(part_two(&bp), 467835);
}
