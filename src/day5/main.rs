use core::panic;
use std::u64;

use advent_of_code::open_file::open;

fn main() {
    let content = open("./src/day5/input.txt");

    println!("{}", part_one(&content));
}

fn part_one(input: &String) -> u64 {
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
        load_data(&mut maps_array, data[0], &map_type);
    }
    for seed in seed_array {
        let mut val: u64 = seed.parse().unwrap();
        for maps in &maps_array {
            // println!("{}", maps.len());
            for map in maps {
                if val >= map.source_start && val <= map.source_end {
                    val = val - map.source_start + map.destination_start;
                    break;
                }
            }
        }
        if val < ans {
            ans = val;
        }
    }
    return ans;
}

fn load_data(array: &mut Vec<Vec<Map>>, input: &str, map_type: &str) {
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

struct Map {
    destination_start: u64,
    destination_end: u64,
    source_start: u64,
    source_end: u64,
}

impl Map {
    fn new() -> Self {
        Self {
            destination_start: 0,
            destination_end: 0,
            source_start: 0,
            source_end: 0,
        }
    }

    fn load_data(&mut self, destination_start: u64, source_start: u64, range: u64) {
        self.destination_start = destination_start;
        self.source_start = source_start;
        self.destination_end = destination_start + range - 1;
        self.source_end = source_start + range - 1;
    }
}

#[test]
fn test_part_one() {
    let input: String = "seeds: 79 14 55 13

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
56 93 4"
        .to_string();
    assert_eq!(part_one(&input), 35);
}
