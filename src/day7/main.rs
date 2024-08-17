use std::collections::HashMap;

use advent_of_code::open_file;

const PART_ONE_TABLE: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
const PART_TWO_TABLE: [char; 13] = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

fn main() {
    let content: String = open_file::open("./src/day7/input.txt");

    println!("{}", part_one(&content));
    println!("{}", part_two(&content));
}

fn part_one(input: &str) -> u64 {
    let mut games: Vec<Game> = vec![];
    let lines = input.lines();
    for line in lines {
        let texts: Vec<&str> = line.split_whitespace().collect();
        let mut new_game: Game = Game::new();
        new_game.hand = texts[0].to_string();
        new_game.bid = texts[1].parse().unwrap();
        new_game.find_score(texts[0], false);
        push_game_to_array(&mut games, new_game, &PART_ONE_TABLE);
    }

    let mut ans: u64 = 0;
    for (index, game) in games.iter().enumerate() {
        ans += game.bid as u64 * (index + 1) as u64; 
    }
    ans
}

fn part_two(input: &str) -> u64 {
    let mut games: Vec<Game> = vec![];
    let lines = input.lines();
    for line in lines {
        let texts: Vec<&str> = line.split_whitespace().collect();
        let mut new_game: Game = Game::new();
        new_game.hand = texts[0].to_string();
        new_game.bid = texts[1].parse().unwrap();
        new_game.find_score(texts[0], true);
        push_game_to_array(&mut games, new_game, &PART_TWO_TABLE);
    }

    let mut ans: u64 = 0;
    for (index, game) in games.iter().enumerate() {
        ans += game.bid as u64 * (index + 1) as u64; 
    }
    ans
}

fn push_game_to_array(games: &mut Vec<Game>, new_game: Game, table: &[char]) {
    for i in 0..games.len() {
        if !new_game.is_greater_than(&games[i], table) {
            games.insert(i, new_game);
            return;
        }
    }
    games.push(new_game);
}

struct Game {
    hand: String,
    score: u8,
    bid: u16
}

impl Game {
    fn new() -> Self {
        Self { hand: String::new(), score: 0, bid: 0 }
    }

    fn find_score(&mut self, hand: &str, is_part_two: bool) {
        let mut map: HashMap<char, u8> = HashMap::new();
        for c in hand.chars() {
            match map.get(&c) {
                Some(amount) => {
                    map.insert(c, amount + 1);
                }
                None => {
                    map.insert(c, 1);
                }
            }
        }
        let mut j_amount: u8 = 0;
        if is_part_two {
            if let Some(amount) = map.remove(&'J') {
                j_amount = amount;
            }
            
        }
        let mut val: Vec<u8> = map.into_values().collect();
        if val.is_empty() {
            self.score = 7;
            return;
        }
        val.sort_by(|a, b| b.cmp(a));
        val[0] += j_amount;
        self.score = match val[0] {
            5 => 7,
            4 => 6,
            3 => {
                match val[1] {
                    2 => 5,
                    _ => 4
                }
            }
            2 => {
                match val[1] {
                    2 => 3,
                    _ => 2
                }
            }
            1 => 1,
            _ => 0
        };
    }

    fn is_greater_than(&self, other_game: &Game, table: &[char]) -> bool {
        if self.score > other_game.score {
            return true;
        }
        if self.score < other_game.score {
            return false;
        }
        let mut a_chars = self.hand.chars();
        let mut b_chars = other_game.hand.chars();
        for _ in 0..5 {
            let a = Self::card_to_number(&a_chars.next().unwrap(), table);
            let b = Self::card_to_number(&b_chars.next().unwrap(), table);
            if a > b {
                return true;
            }
            if b > a {
                return false;
            }
        }
        false
    }

    fn card_to_number(c: &char, table: &[char]) -> u8 {
        for (index, table_c) in table.iter().enumerate() {
            if table_c == c {
                return index as u8 + 1;
            }
        }
        0
    }
}

#[test]
fn test_part_one() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_eq!(part_one(input), 6440);
}

#[test]
fn test_part_two() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_eq!(part_two(input), 5905);
}
