use std::collections::HashSet;
use std::fs;

const FILE_PATH: &str = "./day06/input.txt";
const WINDOW_SIZE: usize = 14;

fn main() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let received_stream = contents.chars().collect::<Vec<char>>();
    let mut discarded = 0;

    for rx_chars in received_stream.windows(WINDOW_SIZE) {
        if rx_chars.iter().copied().collect::<HashSet<char>>().len() == WINDOW_SIZE {
            break;
        } else {
            discarded += 1
        }
    }

    println!("answer is {}", discarded + WINDOW_SIZE)
}