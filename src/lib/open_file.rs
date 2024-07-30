use std::{fs::File, io::Read};

pub fn open(path: &str) -> String {
    let mut file = File::open(path).expect("Unable to find the file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Cannot read file");
    return content;
}
