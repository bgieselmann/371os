use std::fs::read_to_string;
use std::env;

fn main() {
    let args:Vec<String> = env::args().collect();

    let file_path = &args[1];

    let lines = read_lines(file_path);

    let mut lc = 0;
    let mut wc = 0;
    let mut bc = 0;

    for line in lines {
        lc = lc + 1;
        wc = wc + word_count(line.clone());
        bc = bc + byte_count(line.clone());
    }

    bc = bc + lc;

    println!("      {}     {}    {} {}", lc, wc, bc, file_path);
}


fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn byte_count(line: String) -> usize {
    line.len()
    // note this doesn't include the newline character
}

fn word_count(line: String) -> u32 {
    let chars = line.chars();
    let mut count = 0;
    let mut last_char_is_whitespace = true;

    for char in chars {
        if char.is_whitespace() == false {
            if last_char_is_whitespace == true {
                count = count + 1;
            }
            last_char_is_whitespace = false;
        }
        else {
            last_char_is_whitespace = true;
        }
    }
    count
}

fn english_word_count(line: String) -> u32 {
    let chars = line.chars();
    let mut count = 0;
    let mut last_char_in_alphabet = false;

    for char in chars {
        if char.is_alphabetic() {
            if last_char_in_alphabet == false {
		count = count + 1;
            }
            last_char_in_alphabet = true;
        }
        else {
            last_char_in_alphabet = false;
        }
    }

    count
}
