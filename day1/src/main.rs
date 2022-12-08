use std::fs;

const FILE_PATH: &str = "./day1/input.txt";

fn main() {
    println!("In file {}", FILE_PATH);

    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");

    let mut _current_index = 0;
    let mut additions: Vec<u32> = vec![0; 1];
    let mut _biggest_calories = 0;

    for line in lines {
        match line.trim() {
            "" => {
                _current_index += 1;
                additions.push(0);
            }
            _ => {
                let calories: u32 = match line.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("cannot parse {line}");
                        continue;
                    }
                };
                additions[_current_index] += calories;
            }
        }
    }

    additions.sort();
    let biggest = additions.last().unwrap();
    let mut biggest_three = 0;

    for i in (additions.len() - 3)..(additions.len()) {
        biggest_three += additions[i];
    }
    println!("biggest one is {biggest}");
    println!("biggest three are {biggest_three}");
}