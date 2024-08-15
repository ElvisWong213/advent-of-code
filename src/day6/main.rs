use advent_of_code::open_file;

fn main() {
    let content: String = open_file::open("./src/day6/input.txt");

    println!("{}", part_one(&content));
    println!("{}", part_two(&content));
}

fn part_one(input: &str) -> u32 {
    let rounds = parse_input_to_rounds(input);
    let mut ans: u32 = 1;

    for round in rounds {
        let max_time = round.time;
        let record_distance = round.distance;
        let mut ways: u16 = 0;
        for hold_time in 0..=round.time {
            let remaining_time = max_time - hold_time;
            let distance = hold_time * remaining_time;
            if distance > record_distance {
                ways += 1;
            }
        }
        ans *= ways as u32;
    }
    ans
}

fn part_two(input: &str) -> u64 {
    let round: Round = parse_input_to_round(input);
    let mut min: u64 = 0;
    let mut max: u64 = 0;
    for time in 0..=round.time {
        let remaining_time = round.time - time;
        let distance = time * remaining_time;
        if distance > round.distance {
            min = time;
            break;
        }
    }
    for time in (0..=round.time).rev() {
        let remaining_time = round.time - time;
        let distance = time * remaining_time;
        if distance > round.distance {
            max = time;
            break;
        }
    }
    max - min + 1
}

fn parse_input_to_round(input: &str) -> Round {
    let mut lines = input.lines();
    let time_array: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
    let distance_array: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
    let size = time_array.len();

    let mut time: String = String::new();
    let mut distance: String = String::new();
    for i in 1..size {
        time.push_str(time_array[i]);
        distance.push_str(distance_array[i]);
    }
    Round::new(time.parse().unwrap(), distance.parse().unwrap())
}

fn parse_input_to_rounds(input: &str) -> Vec<Round> {
    let mut lines = input.lines();
    let time_array: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
    let distance_array: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
    let size = time_array.len();

    let mut rounds: Vec<Round> = vec![];
    for i in 1..size {
        let time = time_array[i].parse::<u64>().unwrap();
        let distance = distance_array[i].parse::<u64>().unwrap();
        let new_round: Round = Round::new(time, distance);
        rounds.push(new_round);
    }
    rounds
}

struct Round {
    time: u64,
    distance: u64
}

impl Round {
    fn new(time: u64, distance: u64) -> Self {
        Self { time , distance }
    }
}

#[test]
fn test_part_one() {
    let input = "Time:      7  15   30
Distance:  9  40  200";
    assert_eq!(part_one(input), 288);
}

#[test]
fn test_part_two() {
    let input = "Time:      7  15   30
Distance:  9  40  200";
    assert_eq!(part_two(input), 71503);
}
