use advent_of_code::open_file;

fn main() {
    let content = open_file::open("./src/day2/input.txt");
    // println!("{:}", content);

    // let mut games: Vec<Game> = vec![];
    part_one(&content);
    part_two(&content);
}

fn part_one(content: &str) {
    let lines: Vec<&str> = content.lines().collect();
    let target_game: Game = Game::new_target(12, 13, 14);
    let mut id_sum: u16 = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let game = Game::new(line);
        // games.push(game);
        if game.is_contain(&target_game) {
            id_sum += game.id;
        }
    }
    println!("{:}", id_sum);
}

fn part_two(content: &str) {
    let lines: Vec<&str> = content.lines().collect();
    let mut sum: u32 = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let game = Game::new(line);
        sum += game.power();
    }
    println!("{:}", sum);
}

struct Game {
    id: u16,
    red: u16,
    green: u16,
    blue: u16,
}

impl Game {
    pub fn new_target(red: u16, green: u16, blue: u16) -> Self {
        Self {
            id: 0,
            red,
            green,
            blue,
        }
    }

    pub fn new(input: &str) -> Self {
        let parts: Vec<&str> = input.split(": ").collect();
        // println!("{:}", parts[0]);
        let game_id: u16 = parts[0].to_string().replace("Game ", "").parse().unwrap();
        let draws: Vec<&str> = parts[1].split("; ").collect();
        let mut red: u16 = 0;
        let mut green: u16 = 0;
        let mut blue: u16 = 0;
        for draw in draws {
            let results: Vec<&str> = draw.split(", ").collect();
            for result in results {
                // println!("{:}", result);
                let data: Vec<&str> = result.split(' ').collect();
                let value = data[0].parse().unwrap();
                let key = data[1].replace('\n', "");
                match key.as_str() {
                    "red" => {
                        if value > red {
                            red = value;
                        }
                    }
                    "green" => {
                        if value > green {
                            green = value;
                        }
                    }
                    "blue" => {
                        if value > blue {
                            blue = value;
                        }
                    }
                    _ => {}
                }
            }
        }
        Self {
            id: game_id,
            red,
            green,
            blue,
        }
    }

    pub fn is_contain(&self, target_game: &Game) -> bool {
        if self.red <= target_game.red
            && self.green <= target_game.green
            && self.blue <= target_game.blue
        {
            return true;
        }
        false
    }

    pub fn power(&self) -> u32 {
        self.red as u32 * self.green as u32 * self.blue as u32
    }
}

#[test]
fn test_game_id() {
    let input = "Game 1: 2 blue, 4 green; 7 blue, 1 red, 14 green; 5 blue, 13 green, 1 red; 1 red, 7 blue, 11 green";
    let game = Game::new(input);
    assert_eq!(game.id, 1);
}

#[test]
fn test_game_red() {
    let input = "Game 1: 2 blue, 4 green; 7 blue, 1 red, 14 green; 5 blue, 13 green, 1 red; 1 red, 7 blue, 11 green";
    let game = Game::new(input);
    assert_eq!(game.red, 1);
}

#[test]
fn test_game_green() {
    let input = "Game 1: 2 blue, 4 green; 7 blue, 1 red, 14 green; 5 blue, 13 green, 1 red; 1 red, 7 blue, 11 green";
    let game = Game::new(input);
    assert_eq!(game.green, 14);
}

#[test]
fn test_game_blue() {
    let input = "Game 1: 2 blue, 4 green; 7 blue, 1 red, 14 green; 5 blue, 13 green, 1 red; 1 red, 7 blue, 11 green";
    let game = Game::new(input);
    assert_eq!(game.blue, 7);
}

#[test]
fn test() {
    let input= "Game 95: 1 red, 1 blue, 3 green; 2 green, 6 blue; 1 green, 13 blue, 1 red; 3 green, 15 blue\n";
    let game = Game::new(input);
    let target_game: Game = Game::new_target(12, 13, 14);
    assert!(!game.is_contain(&target_game));
}
