use d03::*;
use utils::Coord;

const INPUT_FILENAME: &str = "input.txt";
const GEAR_CHAR: char = '*';
const GEAR_NUM_NUMBERS: usize = 2;

fn main() {
    let input_str = match utils::file_to_vec(INPUT_FILENAME) {
        Err(_) => {
            println!("Couldn't turn file into vec!");
            return;
        }
        Ok(v) => v,
    };

    let len_y = input_str.len();
    let len_x = input_str[0].len();

    // Create empty matrix to store number locations
    let mut number_matrix: Vec<Vec<Option<Number>>> = Vec::new();
    for i in 0..len_y {
        number_matrix.push(Vec::new());
        number_matrix[i].resize(len_x, None);
    }

    // Store info about all of the numbers
    for (y, line) in input_str.iter().enumerate() {
        let mut curr_x = 0;
        loop {
            match line[curr_x..].find(|c: char| c.is_digit(RADIX)) {
                Some(mut x) => {
                    x += curr_x;
                    let end_idx = match line[x..].find(|c: char| !c.is_digit(RADIX)) {
                        Some(val) => val + x,
                        None => line.len(),
                    };
                    let value = line[x..end_idx].parse::<u32>().unwrap();

                    let num = Number {
                        start_coord: Coord { x, y },
                        len: end_idx - x,
                        value,
                    };

                    for i in x..end_idx {
                        number_matrix[y][i] = Some(num);
                    }

                    curr_x = end_idx;
                }
                None => break,
            }
        }
    }

    // Sort through the schematic to find gear and store gear ratios
    let mut gear_ratios: Vec<u32> = Vec::new();
    for (y, line) in input_str.iter().enumerate() {
        let mut curr_x = 0;
        loop {
            match line[curr_x..].find(GEAR_CHAR) {
                Some(mut x) => {
                    x += curr_x;

                    match find_gear_ratio(Coord { x, y }, &number_matrix) {
                        Some(ratio) => gear_ratios.push(ratio),
                        None => (),
                    }

                    curr_x = x + 1;
                }
                None => break,
            }
        }
    }

    let result = gear_ratios.into_iter().reduce(|acc, r| acc + r).unwrap();

    println!("{:?}", result);
}

fn find_gear_ratio(gear_coord: Coord, number_matrix: &Vec<Vec<Option<Number>>>) -> Option<u32> {
    let len_y = number_matrix.len();
    let len_x = number_matrix[0].len();

    // Calc the x slice to search
    let mut search_len_x = 2;
    let start_x = if gear_coord.x != 0 {
        search_len_x += 1;
        gear_coord.x - 1
    } else {
        0
    };
    let end_x = if start_x + search_len_x > len_x {
        //search_len_x -= 1; UNUSED
        len_x
    } else {
        start_x + search_len_x
    };

    // Calc the y slice to search
    let mut search_len_y = 2;
    let start_y = if gear_coord.y != 0 {
        search_len_y += 1;
        gear_coord.y - 1
    } else {
        0
    };
    let end_y = if start_y + search_len_y > len_y {
        //search_len_y -= 1; UNUSED
        len_y
    } else {
        start_y + search_len_y
    };

    // Search for numbers and count unique numbers
    let mut adjacent: Vec<Option<Number>> = Vec::new();
    for line in &number_matrix[start_y..end_y] {
        adjacent.extend_from_slice(&line[start_x..end_x])
    }
    let mut numbers: Vec<Number> = adjacent.into_iter().filter_map(|n| n).collect();
    numbers.dedup();

    if numbers.len() == GEAR_NUM_NUMBERS {
        Some(numbers.iter().fold(1, |acc, n| acc * n.value))
    } else {
        None
    }
}
