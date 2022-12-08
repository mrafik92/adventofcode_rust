use std::fs;

const FILE_PATH: &str = "./day4/input.txt";

fn main() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let complete_overlapping_intersections_count = contents
        .trim()
        .split('\n')
        .filter(|area| check_if_complete_overlapping(area))
        .count();

    let partial_overlapping_intersections_count = contents
        .trim()
        .split('\n')
        .filter(|area| check_if_partial_overlapping(area))
        .count();

    println!(
        "# of overlapping_intersections_count {}",
        complete_overlapping_intersections_count
    );
    println!(
        "# of partial_overlapping_intersections_count {}",
        partial_overlapping_intersections_count
    );
}

fn extract_range(range: &String) -> Vec<i32> {
    range
        .split('-')
        .map(|range| range.parse::<i32>().unwrap())
        .collect()
}

fn get_ranges(p: &str) -> (Vec<i32>, Vec<i32>) {
    let parts: Vec<String> = p.split(',').map(|s| s.to_string()).collect();

    let first_range = extract_range(&parts[0]);
    let second_range = extract_range(&parts[1]);

    (first_range, second_range)
}

fn check_if_complete_overlapping(p: &str) -> bool {
    let (first_range, second_range) = get_ranges(p);

    ((first_range[0] <= second_range[0]) & (first_range[1] >= second_range[1]))
        || ((first_range[0] >= second_range[0]) & (first_range[1] <= second_range[1]))
}

fn check_if_partial_overlapping(p: &str) -> bool {
    let (first_range, second_range) = get_ranges(p);

    (((first_range[0] >= second_range[0]) & (first_range[0] <= second_range[1]))
        || ((first_range[1] >= second_range[0]) & (first_range[1] <= second_range[1])))
        || (((second_range[0] >= first_range[0]) & (second_range[0] <= first_range[1]))
            || ((second_range[1] >= first_range[0]) & (second_range[1] <= first_range[1])))
}
