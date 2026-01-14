use std::fs::read_to_string;

fn main() {

    let mut lines = read_lines("main.rs");

    for line in lines {
        println!("{}", char_count(line));
    }
}


fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn char_count(line: String) -> u32 {
    let mut chars = line.chars();
    let mut count = 0;

    for char in chars {
        count = count + 1;
    };

    count
}
