use std::collections::HashMap;

use advent_of_code::open_file;

fn main() {
    let content = open_file::open("./src/day1/input.txt");
    // print!("{:}", content);

    println!("{:}", part_one(&content));
    println!("{:}", part_two(&content));
}

fn part_one(content: &str) -> u32 {
    let mut ans: u32 = 0;
    let mut number_cache: NumCache = NumCache::new();
    for c in content.chars() {
        if c == '\n' {
            ans += number_cache.get_val() as u32;
            number_cache.clear();
            continue;
        }
        if let Some(num) = c.to_digit(10) {
            number_cache.push(num as u8);
        }
    }
    ans
}

fn part_two(content: &str) -> u32 {
    let mut ans: u32 = 0;
    let mut number_cache: NumCache = NumCache::new();
    let mut text_cache: Vec<char> = vec![];
    for c in content.chars() {
        if c == '\n' {
            ans += number_cache.get_val() as u32;
            number_cache.clear();
            text_cache.clear();
            continue;
        }
        match c.to_digit(10) {
            Some(num) => {
                number_cache.push(num as u8);
                text_cache.clear();
            }
            None => {
                text_cache.push(c);
            }
        };
        if let Some(num) = check_string_to_digit(&mut text_cache) {
            number_cache.push(num);
        }
    }
    ans
}

fn check_string_to_digit(text_cache: &mut Vec<char>) -> Option<u8> {
    let string_digits: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut map: HashMap<&str, u8> = HashMap::new();
    for (i, w) in string_digits.into_iter().enumerate() {
        map.insert(w, i as u8 + 1);
    }

    let size = text_cache.len();
    let mut pointer: usize = 0;

    while pointer < size {
        let mut word: String = String::new();
        for i in pointer..size {
            word.push(text_cache[i]);
        }
        // println!("word: {:}", word);
        if let Some(val) = map.get(word.as_str()) {
            match text_cache.pop() {
                Some(last_char) => {
                    text_cache.clear();
                    text_cache.push(last_char);
                }
                None => {
                    text_cache.clear();
                }
            };
            // println!("val: {:}", val);
            return Some(*val);
        }
        pointer += 1;
    }
    None
}

struct NumCache {
    value: Vec<u8>,
    size: usize,
}

impl NumCache {
    pub fn new() -> Self {
        Self {
            value: vec![],
            size: 0,
        }
    }

    pub fn push(&mut self, new_val: u8) {
        if new_val == 0 || new_val > 9 {
            return;
        }
        if self.size >= 2 {
            self.value.pop();
            self.size -= 1;
        }
        self.value.push(new_val);
        self.size += 1;
    }

    pub fn get_val(&self) -> u8 {
        if self.size == 1 {
            return self.value[0] * 10 + self.value[0];
        }
        if self.size == 2 {
            return self.value[0] * 10 + self.value[1];
        }
        0
    }

    pub fn clear(&mut self) {
        self.value.clear();
        self.size = 0;
    }
}

#[test]
fn test_part_2_a() {
    let intput = "ggrbl5cthnzlsbjssixpt\n";
    println!("{:}", intput);
    let result = part_two(intput);
    assert_eq!(result, 56);
}

#[test]
fn test_part_2_b() {
    let intput = "four98six83five\n";
    let result = part_two(intput);
    assert_eq!(result, 45);
}

#[test]
fn test_part_2_c() {
    let intput = "sixthree8\n";
    let result = part_two(intput);
    assert_eq!(result, 68);
}

#[test]
fn test_part_2_d() {
    let intput = "nineeight\n";
    let result = part_two(intput);
    assert_eq!(result, 98);
}
