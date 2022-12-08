use std::fs;

const FILE_PATH: &str = "./day05/input.txt";

fn main() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let cargos: Vec<&str> = contents
        .split('\n')
        .take_while(|split| split.trim() != "")
        .collect();
    let current_height_of_cargos = cargos.len();

    let count_of_crate_places: usize = cargos[current_height_of_cargos - 1]
        .trim()
        .split(' ')
        .last()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let mut cargos_stack: Vec<Vec<char>> =
        vec![vec!['_'; count_of_crate_places]; current_height_of_cargos * count_of_crate_places];

    for (i, line) in cargos[0..current_height_of_cargos - 1]
        .iter()
        .rev()
        .enumerate()
    {
        line.chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .enumerate()
            .for_each(|(y, x)| {
                if x.contains(&'[') {
                    cargos_stack[i][y] = x[1];
                } else {
                    cargos_stack[i][y] = '_';
                }
            });
    }

    let mut cargos_stack = transpose(cargos_stack);

    cargos_stack.iter().for_each(|it| {
        println!("{:#?}", it);
    });

    let actions: Vec<(u32, u32, u32)> = contents
        .split('\n')
        .skip_while(|split| split.trim() != "")
        .skip(1)
        .map(|action| {
            let parts: Vec<&str> = action.split(' ').collect();
            return (
                parts[1].parse::<u32>().unwrap(),
                parts[3].parse::<u32>().unwrap() - 1,
                parts[5].parse::<u32>().unwrap() - 1,
            );
        })
        .collect();

    for action in actions {
        for i in 0..action.0 {
            get_last_element(cargos_stack[action.1])
        }
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn get_last_element(&stack: Vec<char>) -> char {
    for i in stack.enumerate().rev() {
        if i == '_' {
            continue;
        } else {
            i
        }
    }
}
