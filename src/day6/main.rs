use std::{collections::{HashMap, HashSet}, hash::Hash};

use advent_of_code::open_file;

fn main() {
    let content = open_file::open("./src/day6/input.txt");

    println!("{:}", part_one(&content));
    println!("{:}", part_two(&content));
}

fn part_one(content: &str) -> u32 {
    let mut map: HashMap<Position, Element> = HashMap::new();
    let mut player_position: Position = Position::new(0, 0);
    let mut player_position_found: bool = false;

    for (y_index, line) in content.lines().enumerate() {
        for (x_index, c) in line.chars().enumerate() {
            let key: Position = Position::new(x_index as i32, y_index as i32);
            match c {
                '.' => {
                    map.insert(key, Element::Air);
                }
                '#' => {
                    map.insert(key, Element::Obstacle);
                }
                '^' => {
                    if player_position_found {
                        panic!("more than one player");
                    }
                    player_position = key.clone();
                    player_position_found = true;
                    map.insert(key, Element::Player);
                }
                _ => {
                    panic!("invalid map");
                }
            }
        }
    }
    match get_visit(&map, &player_position) {
        Some(visit) => {
            visit.len() as u32
        }
        None => {
            0
        }
    }
}

fn part_two(content: &str) -> u32 {
    let mut map: HashMap<Position, Element> = HashMap::new();
    let mut player_position: Position = Position::new(0, 0);
    let mut player_position_found: bool = false;

    for (y_index, line) in content.lines().enumerate() {
        for (x_index, c) in line.chars().enumerate() {
            let key: Position = Position::new(x_index as i32, y_index as i32);
            match c {
                '.' => {
                    map.insert(key, Element::Air);
                }
                '#' => {
                    map.insert(key, Element::Obstacle);
                }
                '^' => {
                    if player_position_found {
                        panic!("more than one player");
                    }
                    player_position = key.clone();
                    player_position_found = true;
                    map.insert(key, Element::Player);
                }
                _ => {
                    panic!("invalid map");
                }
            }
        }
    }
    match get_visit(&map, &player_position) {
        Some(visit) => {
            count_in_loop(&mut map, &visit, &player_position)
        }
        None => {
            0
        }
    }
}

fn get_visit(map: &HashMap<Position, Element>, player_position: &Position) -> Option<HashSet<Position>> {
    let mut current_position: Position = player_position.clone();

    let mut finish: bool = false;
    let mut direction: Direction = Direction::Up;
    let mut visit: HashSet<Position> = HashSet::new();
    let mut visit_obstacle: HashMap<Position, HashSet<Direction>> = HashMap::new();
    while !finish {
        visit.insert(current_position.clone());
        let mut count: u8 = 0;
        loop {
            if count >= 4 {
                return None
            }
            let mut new_position = current_position.clone();
            match direction {
                Direction::Up => {
                    new_position.y -= 1;
                } 
                Direction::Down => {
                    new_position.y += 1;
                }
                Direction::Left => {
                    new_position.x -= 1;
                }
                Direction::Right => {
                    new_position.x += 1;
                }
            }
            if !is_obstacle(map, &new_position, &mut finish) {
                current_position = new_position;
                break;
            } else {
                match visit_obstacle.get(&new_position) {
                    Some(direction_set) => {
                        let mut new_set: HashSet<Direction> = HashSet::new();
                        for e in direction_set {
                            new_set.insert(*e);
                        }
                        if !new_set.insert(direction) {
                            return None;
                        }
                        visit_obstacle.insert(new_position, new_set);
                    }
                    None => {
                        let mut new_set: HashSet<Direction> = HashSet::new();
                        new_set.insert(direction);
                        visit_obstacle.insert(new_position, new_set);
                    }
                }
            }
            if finish {
                break;
            }
            direction = direction.next();
            count += 1;
        }
    }
    Some(visit)
}

fn is_obstacle(map: &HashMap<Position, Element>, new_position: &Position, finish: &mut bool) -> bool {
    match map.get(new_position) {
        Some(element) => {
            if *element == Element::Obstacle {
                return true;
            }
        }
        None => {
            *finish = true;
        }
    }
    false
}

fn count_in_loop(map: &mut HashMap<Position, Element>, visit: &HashSet<Position>, player_position: &Position) -> u32 {
    let mut count: u32 = 0;
    for new_obstacle in visit.iter() {
        if new_obstacle == player_position {
            continue;
        }
        let last_element = map.insert(new_obstacle.clone(), Element::Obstacle);

        match get_visit(map, player_position) {
            Some(_) => {
            }
            None => {
                count += 1;
            }
        }

        if let Some(element) = last_element {
            map.insert(new_obstacle.clone(), element);
        }
    }
    count
}

#[derive(Eq, PartialEq)]
enum Element {
    Air,
    Obstacle,
    Player,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::Up => Self::Right,
            Direction::Down => Self::Left,
            Direction::Left => Self::Up,
            Direction::Right => Self::Down,
        }
    }
}

#[test]
fn test_part_one() {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
    assert_eq!(41, part_one(input));
}

#[test]
fn test_part_two() {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
    assert_eq!(6, part_two(input));
}
