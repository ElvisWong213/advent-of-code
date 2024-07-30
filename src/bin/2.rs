use advent_of_code::open_file;

fn main() {
    let content = open_file::open("./src/bin/2.txt");
    println!("{:}", content);
}

struct Game {
    id: u16,
    red: u16,
    green: u16,
    blue: u16
}

impl Game {
}
