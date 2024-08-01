use core::panic;
use std::{collections::HashSet, str::SplitWhitespace};

use advent_of_code::open_file;

fn main() {
    let content = open_file::open("./src/day4/input.txt");

    // let mut games: Vec<Game> = vec![];
    println!("{}", part_one(&content));
    println!("{}", part_two(&content));
}

fn part_one(input: &String) -> u16 {
    let mut ans: u16 = 0;
    let lines: Vec<&str> = input.lines().collect();
    for line in lines {
        let mut game = Game::new();
        game.load_data(line);
        let total_match = total_match_number(&game.my_number, &game.winning_number);
        if total_match == 0 {
            continue;
        }
        ans += 2_u16.pow(total_match as u32 - 1);
    }
    return ans;
}

fn part_two(input: &String) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut games: Vec<Game> = vec![];
    let mut ans: u32 = 0;
    for line in lines {
        let mut game = Game::new();
        game.load_data(line);
        games.push(game);
    }
    for game_index in 0..games.len() {
        let game = &games[game_index];
        let total_match = total_match_number(&game.my_number, &game.winning_number);
        ans += game.amount;

        for _ in 0..game.amount {
            for match_index in 0..total_match as usize {
                let index = game_index + match_index + 1;
                if index >= games.len() {
                    break;
                }
                games[index].amount += 1;
            }
        }
    }
    return ans;
}

fn total_match_number(set_a: &HashSet<u8>, set_b: &HashSet<u8>) -> u8 {
    let intersrtion = set_a.intersection(set_b);
    let mut count: u8 = 0;
    for _ in intersrtion {
        count += 1;
    }
    return count;
}

struct Game {
    id: u16,
    winning_number: HashSet<u8>,
    my_number: HashSet<u8>,
    amount: u32,
}

impl Game {
    fn new() -> Self {
        Self {
            id: 0,
            winning_number: HashSet::new(),
            my_number: HashSet::new(),
            amount: 0,
        }
    }

    fn load_data(&mut self, input: &str) {
        let games: Vec<&str> = input.split(|c| c == ':' || c == '|').collect();
        if games.len() != 3 {
            panic!("Invalid input data");
        }
        self.id = self.split_whitespace_to_id(&mut games[0].split_whitespace());
        if self.id == 0 {
            panic!("Unable to read card id");
        }
        self.winning_number = self.split_whitespace_to_set(&mut games[1].split_whitespace());
        self.my_number = self.split_whitespace_to_set(&mut games[2].split_whitespace());
        self.amount = 1;
    }

    fn split_whitespace_to_id(&self, input: &mut SplitWhitespace) -> u16 {
        loop {
            match input.next() {
                Some(val) => match val.parse::<u16>() {
                    Ok(num) => return num,
                    Err(_) => {}
                },
                None => return 0,
            }
        }
    }

    fn split_whitespace_to_set(&self, input: &mut SplitWhitespace) -> HashSet<u8> {
        let mut output: HashSet<u8> = HashSet::new();
        loop {
            match input.next() {
                Some(num) => {
                    output.insert(num.parse::<u8>().unwrap());
                }
                None => {
                    break;
                }
            }
        }
        return output;
    }
}

#[test]
fn test_part_one() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        .to_string();
    assert_eq!(part_one(&input), 13);
}

#[test]
fn test_part_two() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        .to_string();
    assert_eq!(part_two(&input), 30);
}
