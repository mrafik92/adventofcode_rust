use crate::Hand::{PAPER, ROCK, SCISSORS};
use crate::HandResult::{Draw, Lose, Win};
use std::fs;

const FILE_PATH: &str = "./day02/input.txt";

fn main() {
    println!("In file {}", FILE_PATH);

    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let splits = contents.trim().split('\n');

    let mut score: u32 = 0;

    for split in splits {
        let trimmed = split.trim();
        if trimmed.len() == 3 {
            let mut game_score = 0;
            game_score += get_play_score(trimmed.chars().nth(2).unwrap());
            // game_score += get_round_score(trimmed.chars().nth(2).unwrap(), trimmed.chars().nth(0).unwrap());
            game_score += get_score_from_hand(get_my_hand_from_result(
                convert_enemy_hand(trimmed.chars().nth(0).unwrap()),
                convert_game_result(trimmed.chars().nth(2).unwrap()),
            ));
            score += game_score;
        }
    }

    println!("score is {}", score);
}

fn convert_enemy_hand(enemy: char) -> Hand {
    match enemy {
        'A' => ROCK,
        'B' => PAPER,
        'C' => SCISSORS,
        _ => ROCK,
    }
}

fn convert_game_result(enemy: char) -> HandResult {
    match enemy {
        'X' => Lose,
        'Y' => Draw,
        'Z' => Win,
        _ => Lose,
    }
}

fn get_play_score(play: char) -> u32 {
    match play {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => {
            println!("received invalid input {}", play);
            0
        }
    }
}

#[derive(PartialEq)]
enum Hand {
    ROCK,
    PAPER,
    SCISSORS,
}

pub enum HandResult {
    Win,
    Lose,
    Draw,
}

fn beats(hand: Hand, opponent: Hand) -> HandResult {
    if hand == opponent {
        return Draw;
    }

    return match hand {
        ROCK => {
            if opponent == SCISSORS {
                return Win;
            }
            Lose
        }
        PAPER => {
            if opponent == ROCK {
                return Win;
            }
            Lose
        }
        SCISSORS => {
            if opponent == PAPER {
                return Win;
            }
            Lose
        }
    };
}

fn get_round_score(you: char, him: char) -> u32 {
    let your_hand = match you {
        'X' => ROCK,
        'Y' => PAPER,
        'Z' => SCISSORS,
        _ => {
            println!("invalid you {}", you);
            ROCK
        }
    };

    let his_hand = match him {
        'A' => ROCK,
        'B' => PAPER,
        'C' => SCISSORS,
        _ => {
            println!("invalid him {}", him);
            ROCK
        }
    };

    let res = beats(your_hand, his_hand);

    match res {
        Win => 6,
        Lose => 0,
        Draw => 3,
    }
}

fn get_round_score_from_result(play: char) -> u32 {
    match play {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => 0,
    }
}

fn get_my_hand_from_result(enemy: Hand, result: HandResult) -> Hand {
    match result {
        Draw => enemy,
        Win => match enemy {
            ROCK => PAPER,
            PAPER => SCISSORS,
            SCISSORS => ROCK,
        },
        Lose => match enemy {
            ROCK => SCISSORS,
            PAPER => ROCK,
            SCISSORS => PAPER,
        },
    }
}

fn get_score_from_hand(play: Hand) -> u32 {
    match play {
        ROCK => 1,
        PAPER => 2,
        SCISSORS => 3,
    }
}
