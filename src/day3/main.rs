use advent_of_code::open_file;

fn main() {
    let content = open_file::open("./src/day3/input.txt");

    println!("{:}", part_one(&content));
    println!("{:}", part_two(&content));
}

fn part_one(content: &str) -> u32 {
    let start: &str = "mul(";
    let mut buffer: String = String::new();
    let mut number_buffer: String = String::new();
    let mut numbers: [u32; 2] = [0, 0];
    let mut total: u32 = 0;

    for c in content.chars() {
        if buffer.len() == start.len() {
            if c == ',' {
                numbers[0] = number_buffer.parse().unwrap();
                number_buffer.clear();
                continue;
            }
            if c == ')' {
                numbers[1] = number_buffer.parse().unwrap();
                total += numbers[0] * numbers[1];
                numbers = [0, 0];
                number_buffer.clear();
                buffer.clear();
                continue;
            }
            if c >= '0' && c <= '9' {
                number_buffer.push(c);
            } else {
                numbers = [0, 0];
                number_buffer.clear();
                buffer.clear();
            }
            continue;
        }
        if c == start.chars().nth(buffer.len()).unwrap() {
            buffer.push(c);
        } else {
            numbers = [0, 0];
            number_buffer.clear();
            buffer.clear();
        }
    }
    total
}

fn part_two(content: &str) -> u32 {
    let start: &str = "mul(";
    let mut buffer: String = String::new();
    let mut number_buffer: String = String::new();
    let mut numbers: [u32; 2] = [0, 0];
    let mut total: u32 = 0;
    let mut do_math: bool = true;
    let mut do_buffer: String = String::new();

    for c in content.chars() {
        check_do(&mut do_buffer, &c, &mut do_math);
        if !do_math {
            continue;
        }
        if buffer.len() == start.len() {
            if c == ',' {
                numbers[0] = number_buffer.parse().unwrap();
                number_buffer.clear();
                continue;
            }
            if c == ')' {
                numbers[1] = number_buffer.parse().unwrap();
                total += numbers[0] * numbers[1];
                numbers = [0, 0];
                number_buffer.clear();
                buffer.clear();
                continue;
            }
            if c >= '0' && c <= '9' {
                number_buffer.push(c);
            } else {
                numbers = [0, 0];
                number_buffer.clear();
                buffer.clear();
            }
            continue;
        }
        if c == start.chars().nth(buffer.len()).unwrap() {
            buffer.push(c);
        } else {
            numbers = [0, 0];
            number_buffer.clear();
            buffer.clear();
        }
    }
    total
}

fn check_do(do_buffer: &mut String, input: &char, do_math: &mut bool) {
    let do_char: [char; 4] = ['d', 'o', '(', ')'];
    let dont_char: [char; 7] = ['d', 'o', 'n', '\'', 't', '(', ')'];
    let buffer_size = do_buffer.len();
    let mut is_do: bool = true;

    if buffer_size == do_char.len() {
        let mut pass: bool = true;
        for (index, c) in do_buffer.chars().enumerate() {
            if c != do_char[index] {
                pass = false;
                break;
            }
        }
        if pass {
            *do_math = true;
            do_buffer.clear();
            return;
        }
        
    }

    if buffer_size == dont_char.len() {
        let mut pass: bool = true;
        for (index, c) in do_buffer.chars().enumerate() {
            if c != dont_char[index] {
                pass = false;
                break;
            }
        }
        if pass {
            *do_math = false;
        }
        do_buffer.clear();
        return;
    }

    if buffer_size < do_char.len() {
        if *input == do_char[buffer_size] {
            do_buffer.push(*input);
            return;
        } else {
            is_do = false;
        }
    }
    if buffer_size < dont_char.len() {
        if *input == dont_char[buffer_size] {
            do_buffer.push(*input);
        } else if !is_do {
            do_buffer.clear();
        }
    }
}

#[test]
fn test_part_one() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(161, part_one(input));
}

#[test]
fn test_part_two() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(48, part_two(input));
}
