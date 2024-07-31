use advent_of_code::open_file;

fn main() {
    let content = open_file::open("./src/day3/input.txt");
    // println!("{}", content);
    println!("{}", part_one(&content));
}

fn part_one(content: &String) -> u32 {
    let mut ans: u32 = 0;
    let array = convert_string_to_2d_array(&content);
    // println!("{}", array.len());
    for y_index in 0..array.len() {
        let mut number: String = String::new();
        let mut adjacent_to_symbol: bool = false;
        for x_index in 0..array[y_index].len() {
            // println!("{}", &array[y_index][x_index]);
            if is_number(&array[y_index][x_index]) {
                number.push(array[y_index][x_index]);
                if adjacent_to_symbol == false && is_adjacent_to_symbol(&array, x_index, y_index) {
                    adjacent_to_symbol = true;
                }
            } else {
                if number.len() > 0 && adjacent_to_symbol == true {
                    println!("{}", number);
                    ans += number.parse::<u32>().unwrap();
                }
                number.clear();
                adjacent_to_symbol = false;
            }
        }
        if number.len() > 0 && adjacent_to_symbol == true {
            ans += number.parse::<u32>().unwrap();
        }
    }
    return ans;
}

fn is_number(input: &char) -> bool {
    match input {
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
            return true;
        }
        _ => {
            return false;
        }
    }
}

fn is_symbol(input: &char) -> bool {
    if is_number(input) == false && *input != '.' {
        return true;
    }
    return false;
}

fn is_adjacent_to_symbol(array: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let mut y_min = y;
    if y_min != 0 {
        y_min -= 1;
    }
    let mut x_min = x;
    if x_min != 0 {
        x_min -= 1;
    }
    for y_index in y_min..=y + 1 {
        if y_index >= array.len() {
            continue;
        }
        for x_index in x_min..=x + 1 {
            if x_index >= array[y_index].len() {
                continue;
            }
            if x_index == x && y_index == y {
                continue;
            }
            if is_symbol(&array[y_index][x_index]) {
                return true;
            }
        }
    }
    return false;
}

fn convert_string_to_2d_array(input: &String) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = vec![vec![]];
    let mut line: Vec<char> = vec![];
    for c in input.chars() {
        if c != '\n' {
            line.push(c);
        } else {
            if line.len() > 0 {
                output.push(line.clone());
            }
            line.clear();
        }
    }
    if line.len() > 0 {
        output.push(line.clone());
    }
    return output;
}

#[test]
fn test_part_1() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        .to_string();
    assert_eq!(part_one(&input), 4361);
}

#[test]
fn test_part_1_2() {
    let input = "467..114..
...*......
..35...633
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        .to_string();
    assert_eq!(part_one(&input), 4361);
}
