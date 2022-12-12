use std::fs;

const FILE_PATH: &str = "./day05/input.txt";

fn main() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let cargos: Vec<&str> = contents
        .lines()
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
            .collect::<Vec<_>>()
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
    let cargos_stack_2 = cargos_stack.clone();
    let actions: Vec<(usize, usize, usize)> = contents
        .lines()
        .skip_while(|split| split.trim() != "")
        .skip(1)
        .map(|action| {
            let parts: Vec<&str> = action.split(' ').collect();
            (
                parts[1].parse::<usize>().unwrap(),
                parts[3].parse::<usize>().unwrap() - 1,
                parts[5].parse::<usize>().unwrap() - 1,
            )
        })
        .collect();


    // part 1
    for action in &actions {
        for _i in 0..action.0 {
            let (top_crate_pos, top_crate) = get_top_crate(&cargos_stack[action.1]);
            let destination_position = get_destination_crate_location(&cargos_stack[action.2]);

            cargos_stack.get_mut(action.2 as usize).unwrap()[destination_position] = top_crate;
            cargos_stack.get_mut(action.1 as usize).unwrap()[top_crate_pos] = '_';
        }
    }

    for stack in cargos_stack {
        print!("{}", get_top_crate(&stack).1);
    }
    println!();

    cargos_stack = cargos_stack_2;

    for action in &actions {
        let top_crate_pos = get_top_crate(&cargos_stack[action.1]).0;
        let destination_position = get_destination_crate_location(&cargos_stack[action.2]);

        for _i in 0..action.0 {
            cargos_stack.get_mut(action.2 as usize).unwrap()[destination_position + action.0 - _i - 1] =
                cargos_stack[action.1][top_crate_pos - _i];
            cargos_stack.get_mut(action.1 as usize).unwrap()[top_crate_pos - _i] = '_';
        }
    }

    for stack in cargos_stack {
        print!("{}", get_top_crate(&stack).1);
    }
    println!();
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

fn get_top_crate(stack: &Vec<char>) -> (usize, char) {
    for (i, value) in stack.iter().enumerate().rev() {
        if value != &'_' {
            return (i, *value);
        }
    }
    (0, '_')
}

fn get_destination_crate_location(stack: &Vec<char>) -> usize {
    for (i, value) in stack.iter().enumerate() {
        if value == &'_' {
            return i;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::get_top_crate;

    #[test]
    fn test_get_last_element() {
        let my_vec = vec!['_', '_', '_', 'a', 'b', 'c'];
        println!("{}", get_top_crate(&my_vec).1);
    }
}

