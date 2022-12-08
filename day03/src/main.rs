use std::fs;

const FILE_PATH: &str = "./day03/input.txt";

fn main() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let sum: u32 = contents
        .trim()
        .lines()
        .map(|split| split.trim())
        .map(|split| (split, split.len() / 2))
        .map(|(split, size)| get_common_char(&split[..size], &split[size..]))
        .map(get_item_priority)
        .sum();

    let sum_of_three: u32 = contents
        .trim()
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunks_of_3| get_common_char_from_array(chunks_of_3))
        .map(get_item_priority)
        .sum();

    println!("Sum is {}", sum);
    println!("Sum of 3 is {}", sum_of_three);
}

fn get_common_char(p0: &str, p1: &str) -> char {
    assert_eq!(p1.len(), p0.len());

    for p_0 in p0.chars() {
        for p_1 in p1.chars() {
            if p_1 == p_0 {
                return p_0;
            }
        }
    }
    '0'
}

fn get_common_char_from_array(p: &[&str]) -> char {
    for p_0 in p[0].chars() {
        for p_1 in p[1].chars() {
            for p_2 in p[2].chars() {
                if p_1 == p_0 && p_1 == p_2 {
                    return p_0;
                }
            }
        }
    }
    '0'
}

fn get_item_priority(item: char) -> u32 {
    if item.is_lowercase() {
        item as u32 - 'a' as u32 + 1
    } else {
        item as u32 - 'A' as u32 + 27
    }
}
