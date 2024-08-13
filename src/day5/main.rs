use core::panic;
use std::{collections::HashSet, hash::Hash};

use advent_of_code::open_file::open;

fn main() {
    let content = open("./src/day5/input.txt");

    println!("{}", part_one(&content));
    println!("{}", part_two(&content));
}

fn part_one(input: &str) -> u64 {
    let mut ans: u64 = u64::MAX;
    let lines = input.lines();
    let mut map_type: &str = "";
    let mut seed_array: Vec<&str> = vec![];
    let mut maps_array: Vec<Vec<Map>> = Vec::with_capacity(7);
    for _ in 0..7 {
        maps_array.push(vec![]);
    }
    for line in lines {
        let data: Vec<&str> = line.split(':').collect();
        if data.len() == 2 {
            map_type = data[0].split_whitespace().collect::<Vec<&str>>()[0];
            if map_type == "seeds" {
                seed_array = data[1].split_whitespace().collect();
            }
            continue;
        }
        load_data(&mut maps_array, data[0], map_type);
    }
    for seed in seed_array {
        let mut val: u64 = seed.parse().unwrap();
        for maps in &maps_array {
            for map in maps {
                if map.is_in_range(val) {
                    val = val - map.source_start + map.destination_start;
                    break;
                }
            }
        }
        if val < ans {
            ans = val;
        }
    }
    ans
}

fn part_two(input: &str) -> u64 {
    let lines = input.lines();
    let mut map_type: &str = "";
    let mut seed_array: Vec<&str> = vec![];
    let mut maps_array: Vec<Vec<Map>> = Vec::with_capacity(7);
    for _ in 0..7 {
        maps_array.push(vec![]);
    }
    for line in lines {
        let data: Vec<&str> = line.split(':').collect();
        if data.len() == 2 {
            map_type = data[0].split_whitespace().collect::<Vec<&str>>()[0];
            if map_type == "seeds" {
                seed_array = data[1].split_whitespace().collect();
            }
            continue;
        }
        load_data(&mut maps_array, data[0], map_type);
    }
    let old_maps_array = maps_array.clone();
    for (index, maps) in old_maps_array.into_iter().enumerate() {
        if maps.is_empty() {
            continue;
        }
        let mut start: u64 = 0;
        let mut end: u64;
        for map in maps {
            if map.source_start > 0 {
                end = map.source_start - 1;
            } else {
                end = map.source_start;
            }
            if start != end {
                if start > 0 {
                    start += 1;
                }
                let mut new_map = Map::new();
                let length = end - start + 1;
                new_map.load_data(start, start, length);
                maps_array[index].push(new_map);
                maps_array[index].sort_by(|a, b| a.source_start.cmp(&b.source_start));
            }
            start = map.get_source_end();
        }
        // let mut new_map = Map::new();
        // new_map.load_data(start, start, u64::MAX);
        // maps_array[index].push(new_map);
        // maps_array[index].sort_by(|a, b| a.source_start.cmp(&b.source_start));
    }
    let mut seed_list: Vec<SeedData> = vec![];
    // load all seed to list
    for seed_index in 0..seed_array.len() / 2 {
        let start: u64 = seed_array[seed_index * 2].parse().unwrap();
        let length: u64 = seed_array[seed_index * 2 + 1].parse().unwrap();
        let seed_data: SeedData = SeedData::new(start, length);
        seed_list.push(seed_data);
    }
    let mut new_seed_set: HashSet<SeedData> = HashSet::new();
    for maps in maps_array {
        for seed in &seed_list {
            find_destination(seed, &mut new_seed_set, &maps);
        }
        seed_list.clear();
        seed_list = new_seed_set.clone().into_iter().collect();
        new_seed_set.clear();
    }
    seed_list.sort_by(|a, b| a.start.cmp(&b.start));
    match seed_list.first() {
        Some(arr) => arr.start,
        None => 0,
    }
}

fn find_destination(seed: &SeedData, new_seed_set: &mut HashSet<SeedData>, maps: &Vec<Map>) {
    for map in maps {
        if !map.is_in_range(seed.start) {
            continue;
        }
        if let Some(other_seed_data) = seed.get_new_seed_data(map) {
            find_destination(&other_seed_data, new_seed_set, maps);
        }
        let new_seed: SeedData = seed.update_seed_data(map);
        new_seed_set.insert(new_seed);
        return;
    }
    new_seed_set.insert(*seed);
}

fn load_data(array: &mut [Vec<Map>], input: &str, map_type: &str) {
    let numbers: Vec<&str> = input.split_whitespace().collect();
    if numbers.len() != 3 {
        return;
    }
    let mut map: Map = Map::new();
    map.load_data(
        numbers[0].parse().unwrap(),
        numbers[1].parse().unwrap(),
        numbers[2].parse().unwrap(),
    );
    let index: usize = match map_type {
        "seed-to-soil" => 0,
        "soil-to-fertilizer" => 1,
        "fertilizer-to-water" => 2,
        "water-to-light" => 3,
        "light-to-temperature" => 4,
        "temperature-to-humidity" => 5,
        "humidity-to-location" => 6,
        _ => panic!("Map type is empty"),
    };
    array[index].push(map);
    array[index].sort_by(|a, b| a.source_start.cmp(&b.source_start));
}

#[derive(Clone, Copy)]
struct SeedData {
    start: u64,
    length: u64,
}

impl Hash for SeedData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.start.hash(state);
    }
}

impl PartialEq for SeedData {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
    }
}

impl Ord for SeedData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for SeedData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for SeedData {}

impl SeedData {
    fn new(start: u64, length: u64) -> Self {
        Self { start, length }
    }

    fn get_end(&self) -> u64 {
        self.start + self.length - 1
    }

    fn get_new_seed_data(&self, map: &Map) -> Option<SeedData> {
        if !map.is_in_range(self.start) {
            return None;
        }
        let seed_end: u64 = self.get_end();
        let map_source_end: u64 = map.get_source_end();
        if seed_end <= map_source_end {
            return None;
        }
        let new_start: u64 = map_source_end + 1;
        let new_length: u64 = seed_end - new_start + 1;
        let new_seed_data: SeedData = SeedData::new(new_start, new_length);
        Some(new_seed_data)
    }

    fn update_seed_data(&self, map: &Map) -> SeedData {
        if !map.is_in_range(self.start) {
            return *self;
        }
        let seed_end: u64 = self.get_end();
        let map_source_end: u64 = map.get_source_end();

        let diff: i64 = map.destination_start as i64 - map.source_start as i64;
        let new_start: u64 = (self.start as i64 + diff) as u64;
        let mut new_length: u64 = self.length;
        if seed_end > map_source_end {
            new_length = map_source_end - self.start + 1;
        }
        SeedData::new(new_start, new_length)
    }
}

#[derive(Clone)]
struct Map {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl Map {
    fn new() -> Self {
        Self {
            destination_start: 0,
            source_start: 0,
            length: 0,
        }
    }

    fn load_data(&mut self, destination_start: u64, source_start: u64, length: u64) {
        self.destination_start = destination_start;
        self.source_start = source_start;
        self.length = length;
    }

    fn get_source_end(&self) -> u64 {
        self.source_start + self.length - 1
    }

    fn is_in_range(&self, value: u64) -> bool {
        let source_end = self.source_start + self.length - 1;
        if value >= self.source_start && value <= source_end {
            return true;
        }
        false
    }
}

#[test]
fn test_part_one() {
    let input: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    assert_eq!(part_one(input), 35);
}

#[test]
fn test_part_two() {
    let input: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    assert_eq!(part_two(input), 46);
}

#[test]
fn test_part_two_a() {
    let input: &str = "seeds: 79 14 55 13

seed-to-soil map:

soil-to-fertilizer map:

fertilizer-to-water map:

water-to-light map:

light-to-temperature map:

temperature-to-humidity map:

humidity-to-location map:
";
    assert_eq!(part_two(input), 55);
}

#[test]
fn test_part_two_b() {
    let input = "seeds: 0 5

seed-to-soil map:
5 0 5

soil-to-fertilizer map:
10 5 5

fertilizer-to-water map:
15 10 5

water-to-light map:
20 15 5

light-to-temperature map:
25 20 5

temperature-to-humidity map:
30 25 5

humidity-to-location map:
35 30 5";
    assert_eq!(part_two(input), 35);
}
