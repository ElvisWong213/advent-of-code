use std::usize;

use advent_of_code::open_file;

fn main() {
    let content = open_file::open("./src/day10/input.txt");

    println!("{}", part_one(&content));
    println!("{}", part_two(&content));
}

fn part_one(input: &str) -> usize {
    let mut row_size: usize = 0;
    let mut array: Vec<char> = vec![];
    let mut start_index: usize = 0;
    let mut nodes : Vec<Node> = vec![];
    set_size(input, &mut row_size, &mut array);
    load_to_nodes(&array, &mut nodes, &mut start_index, row_size);
    let order: Vec<usize> = bfs(&mut nodes, start_index);
    order.len()/2
}

fn part_two(input: &str) -> usize {
    0
}

fn set_size(content: &str, row_size: &mut usize, array: &mut Vec<char>) {
    let mut finish_count_row: bool = false;
    for c in content.chars() {
        if c == '\n' {
            finish_count_row = true;
            continue;
        }
        array.push(c);
        if !finish_count_row {
            *row_size += 1;
        }
    }
}

fn bfs(nodes: &mut [Node], index: usize) -> Vec<usize> {
    let mut queue: Vec<usize> = vec![];
    let mut output: Vec<usize> = vec![];
    queue.push(index);
    while !queue.is_empty() {
        match queue.pop() {
            Some(node_index) => {
                let node = &mut nodes[node_index];
                if node.visited {
                    continue;
                }
                output.push(node_index);
                node.visited = true;
                for neighbour_index in node.neighbours.clone() {
                    let neighbour_node = &nodes[neighbour_index];
                    if !neighbour_node.visited && !neighbour_node.is_ground {
                        queue.push(neighbour_index);
                    }
                }
            },
            None => {
                return output
            }
        }
    }
    output
}

enum Direction {
   Top,
   Bottom,
   Left,
   Right
}

impl Direction {
   fn connects(self) -> [char; 3] {
       match self {
          Direction::Top => {
              ['|', '7', 'F']
          },
          Direction::Bottom => {
              ['|', 'L', 'J']
          },
          Direction::Left => {
              ['-', 'L', 'F']
          },
          Direction::Right => {
              ['-', '7', 'J']
          }
       }
   }
}

fn is_connect(out_c: &char, direction: Direction) -> bool {
    if direction.connects().contains(out_c) {
        return true
    }
    false
}

fn load_to_nodes(array: &[char], nodes: &mut Vec<Node>, start_index: &mut usize, row_size: usize) {
    for (index, c) in array.iter().enumerate() {
        let mut node: Node = Node::new();
        match c {
            '|' => {
                if let Some(i) = top(index, row_size, array.len()) {
                    let out_c = array[i];
                    if is_connect(&out_c, Direction::Top) {
                        node.neighbours.push(i);
                    }
                }
                if let Some(i) = bottom(index, row_size, array.len()) {
                    let out_c = array[i];
                    if is_connect(&out_c, Direction::Bottom) {
                        node.neighbours.push(i);
                    }
                }
            },
            '-' => {
                if let Some(i) = left(index,  array.len()) {
                    let out_c = array[i];
                    if is_connect(&out_c, Direction::Left) {
                        node.neighbours.push(i);
                    }
                }
                if let Some(i) = right(index,  array.len()) {
                    let out_c = array[i];
                    if is_connect(&out_c, Direction::Right) {
                        node.neighbours.push(i);
                    }
                }
            },
            'L' => {
                if let Some(i) = top(index, row_size, array.len()) {
                    let out_c = array[i];
                    if is_connect(&out_c, Direction::Top) {
                        node.neighbours.push(i);
                    }
                }
                if let Some(i) = right(index,  array.len()) {
                    let out_c = array[i];
                    if is_connect(&out_c, Direction::Right) {
                        node.neighbours.push(i);
                    }
                }
            },
            'J' => {
                if let Some(i) = top(index, row_size, array.len()) {
                    let out_c = array[i];
                    if is_connect(&out_c, Direction::Top) {
                        node.neighbours.push(i);
                    }
                }
                if let Some(i) = left(index,  array.len()) {
                    let out_c = array[i];
                    if is_connect(&out_c, Direction::Left) {
                        node.neighbours.push(i);
                    }
                }
            },
            '7' => {
                if let Some(i) = left(index,  array.len()) {
                    let out_c = array[i];
                    if is_connect(&out_c, Direction::Left) {
                        node.neighbours.push(i);
                    }
                }
                if let Some(i) = bottom(index, row_size, array.len()) {
                    let out_c = array[i];
                    if is_connect(&out_c, Direction::Bottom) {
                        node.neighbours.push(i);
                    }
                }
            },
            'F' => {
                if let Some(i) = right(index,  array.len()) {
                    let out_c = array[i];
                    if is_connect(&out_c, Direction::Right) {
                        node.neighbours.push(i);
                    }
                }
                if let Some(i) = bottom(index, row_size, array.len()) {
                    let out_c = array[i];
                    if is_connect(&out_c, Direction::Bottom) {
                        node.neighbours.push(i);
                    }
                }
            },
            '.' => {
                node.is_ground = true;
            },
            'S' => {
                if let Some(i) = top(index, row_size, array.len()) {
                    let out_c = array[i];
                    if is_connect(&out_c, Direction::Top) {
                        node.neighbours.push(i);
                    }
                }
                if let Some(i) = bottom(index, row_size, array.len()) {
                    let out_c = array[i];
                    if is_connect(&out_c, Direction::Bottom) {
                        node.neighbours.push(i);
                    }
                }
                if let Some(i) = left(index, array.len()) {
                    let out_c = array[i];
                    if is_connect(&out_c, Direction::Left) {
                        node.neighbours.push(i);
                    }
                }
                if let Some(i) = right(index,  array.len()) {
                    let out_c = array[i];
                    if is_connect(&out_c, Direction::Right) {
                        node.neighbours.push(i);
                    }
                }
                *start_index = index;
            },
            _ => {
                panic!("invalid char");
            }
        }
        nodes.push(node);
    }
}

fn top(index: usize, row_size: usize, size: usize) -> Option<usize> {
    if index < row_size {
        return None
    }
    if index > size - 1 {
        return None
    }
    Some(index - row_size)
}

fn bottom(index: usize, row_size: usize, size: usize) -> Option<usize> {
    if index + row_size > size - 1 {
        return None
    }
    if index > size - 1 {
        return None
    }
    Some(index + row_size)
}

fn left(index: usize, size: usize) -> Option<usize> {
    if index > size - 1 {
        return None
    }
    if index == 0 {
        return None
    }
    Some(index - 1)
}

fn right(index: usize, size: usize) -> Option<usize> {
    if index >= size - 1 {
        return None
    }
    Some(index + 1)
}

struct Node {
    neighbours: Vec<usize>,
    visited: bool,
    is_ground: bool
}

impl Node {
    fn new() -> Self {
        Self { neighbours: vec![], visited: false, is_ground: false }
    }
}

#[test]
fn test_part_one() {
    let input = ".....
.S-7.
.|.|.
.L-J.
.....";
    assert_eq!(part_one(input), 4);
}
