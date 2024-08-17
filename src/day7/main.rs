use std::collections::HashMap;

use advent_of_code::open_file;

fn main() {
    let content: String = open_file::open("./src/day7/input.txt");

    println!("{}", part_one(&content));
}

fn part_one(input: &str) -> u64 {
    let mut games: Vec<Game> = vec![];
    let lines = input.lines();
    for line in lines {
        let texts: Vec<&str> = line.split_whitespace().collect();
        let mut new_game: Game = Game::new();
        new_game.hand = texts[0].to_string();
        new_game.bid = texts[1].parse().unwrap();
        new_game.find_score(texts[0]);
        push_game_to_array(&mut games, new_game);
    }

    let mut ans: u64 = 0;
    for (index, game) in games.iter().enumerate() {
        ans += game.bid as u64 * (index + 1) as u64; 
    }
    ans
}

fn push_game_to_array(games: &mut Vec<Game>, new_game: Game) {
    for i in 0..games.len() {
        if !new_game.is_greater_than(&games[i]) {
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

    fn find_score(&mut self, hand: &str) {
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
        let mut val: Vec<u8> = map.into_values().collect();
        val.sort_by(|a, b| b.cmp(a));
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

    fn is_greater_than(&self, other_game: &Game) -> bool {
        if self.score > other_game.score {
            return true;
        }
        if self.score < other_game.score {
            return false;
        }
        let mut a_chars = self.hand.chars();
        let mut b_chars = other_game.hand.chars();
        for _ in 0..5 {
            let a = Self::card_to_number(&a_chars.next().unwrap());
            let b = Self::card_to_number(&b_chars.next().unwrap());
            if a > b {
                return true;
            }
            if b > a {
                return false;
            }
        }
        false
    }

    fn card_to_number(c: &char) -> u8 {
        match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => {
                c.to_digit(10).unwrap() as u8
            }
        }
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
