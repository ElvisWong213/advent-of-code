use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("./src/bin/1.txt").expect("Unable to find the file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Cannot read file");
    // print!("{:}", content);

    println!("{:}", part_one(&content));
    println!("{:}", part_two(&content));
}

fn part_one(content: &String) -> u32 {
    let mut ans: u32 = 0;
    let mut cache: String = String::new();
    for c in content.chars() {
        if c == '\n' {
            if cache.len() == 2 {
                match cache.parse::<u32>() {
                    Ok(num) => {
                        if num >= 10 && num < 100 {
                            ans += num;
                        }
                    }
                    Err(_) => {
                        continue;
                    }
                };
            }
            if cache.len() == 1 {
                match cache.parse::<u32>() {
                    Ok(num) => {
                        ans += num * 10 + num;
                    }
                    Err(_) => {
                        continue;
                    }
                };
            }
            cache.clear();
        }
        match c.to_digit(10) {
            Some(_) => {
                if cache.len() >= 2 {
                    cache.pop();
                }
                cache.push(c);
            }
            None => {}
        };
    }
    ans
}

fn part_two(content: &String) -> u32 {
    let mut ans: u32 = 0;
    let mut number_cache: String = String::new();
    let mut word_cache: String = String::new();
    for c in content.chars() {
        if c == '\n' {
            if number_cache.len() == 2 {
                match number_cache.parse::<u32>() {
                    Ok(num) => {
                        if num >= 10 && num < 100 {
                            ans += num;
                        }
                    }
                    Err(_) => {
                        continue;
                    }
                };
            }
            if number_cache.len() == 1 {
                match number_cache.parse::<u32>() {
                    Ok(num) => {
                        ans += num * 10 + num;
                    }
                    Err(_) => {
                        continue;
                    }
                };
            }
            number_cache.clear();
            word_cache.clear();
            continue;
        }
        match c.to_digit(10) {
            Some(_) => {
                if number_cache.len() >= 2 {
                    number_cache.pop();
                }
                number_cache.push(c);
            }
            None => {
                word_cache.push(c);
            }
        };
        // println!("{:}", word_cache);
        match check_string_to_digit(&word_cache) {
            ConvertResult::None => {
                // if word_cache.len() > 0 {
                //     word_cache = word_cache.pop().unwrap().to_string();
                // }
                word_cache.clear();
            }
            ConvertResult::Some => {
                // println!("{:}", word_cache);
            }
            ConvertResult::Match(c) => {
                println!("Match: {:}", c);
                if number_cache.len() >= 2 {
                    number_cache.pop();
                }
                number_cache.push(c);
                // if word_cache.len() > 0 {
                //     word_cache = word_cache.pop().unwrap().to_string();
                // }
                word_cache.clear();
            }
            ConvertResult::Duplicate => {
                word_cache.pop();
            }
        }
    }
    ans
}

enum ConvertResult {
    None,
    Some,
    Match(char),
    Duplicate
}

fn check_string_to_digit(input: &String) -> ConvertResult {
    let string_digits: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    if input.len() > 5 {
        return ConvertResult::None;
    }
    let mut some_match = false;
    for (i, sd) in string_digits.into_iter().enumerate() {
        let mut sd_array = sd.chars();
        let mut is_match = true;
        if input.len() != sd.len() {
            is_match = false;
        } else {
            // println!("  {:}, {:}", input, sd);
        }
        let mut last_char: char = ' ';
        for c in input.chars() {
            if last_char == c {
                return ConvertResult::Duplicate;
            }
            last_char = c;
            let sd_char = match sd_array.nth(0) {
                Some(sd_c) => sd_c,
                None => {
                    is_match = false;
                    break;
                }
            };
            // println!("{:}, sd_char: {:}, c: {:}", sd_char != c, sd_char, c);
            if sd_char != c {
                is_match = false;
                break;
            } else {
                some_match = true;
            }
        }
        if is_match == true {
            return ConvertResult::Match(char::from_digit((i + 1) as u32, 10).unwrap());
        }
    }
    if some_match == true {
        return ConvertResult::Some;
    }
    return ConvertResult::None;
}

#[test]
fn test_part_2_a() {
    let intput = "ggrbl5cthnzlsbjssixpt\n".to_string();
    let result = part_two(&intput);
    assert_eq!(result, 56);
}

#[test]
fn test_part_2_b() {
    let intput = "four98six83five\n".to_string();
    let result = part_two(&intput);
    assert_eq!(result, 45);
}

#[test]
fn test_part_2_c() {
    let intput = "sixthree8\n".to_string();
    let result = part_two(&intput);
    assert_eq!(result, 68);
}

#[test]
fn test_part_2_d() {
    let intput = "nineeight\n".to_string();
    let result = part_two(&intput);
    assert_eq!(result, 98);
}
