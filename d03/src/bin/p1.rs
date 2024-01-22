use d03::*;
use utils::Coord;

const INPUT_FILENAME: &str = "input.txt";

fn main() {
    let input_str = match utils::file_to_vec(INPUT_FILENAME) {
        Err(_) => {
            println!("Couldn't turn file into vec!");
            return;
        }
        Ok(v) => v,
    };

    // Store info about all of the numbers
    let mut numbers: Vec<Number> = Vec::new();
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

                    numbers.push(Number {
                        start_coord: Coord { x, y },
                        len: end_idx - x,
                        value,
                    });

                    curr_x = end_idx;
                }
                None => break,
            }
        }
    }

    // Sort through the numbers and only store the part numbers
    let result = numbers
        .iter()
        .filter(|n| is_part_number(&n, &input_str))
        .fold(0, |acc, n| acc + n.value);

    println!("{:?}", result);
}

fn is_part_number(number: &Number, schematic: &Vec<String>) -> bool {
    let len_y = schematic.len();
    let len_x = schematic[0].len();

    // Calc the x slice to search
    let mut search_len_x = number.len + 1;
    let start_x = if number.start_coord.x != 0 {
        search_len_x += 1;
        number.start_coord.x - 1
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
    let start_y = if number.start_coord.y != 0 {
        search_len_y += 1;
        number.start_coord.y - 1
    } else {
        0
    };
    let end_y = if start_y + search_len_y > len_y {
        //search_len_y -= 1; UNUSED
        len_y
    } else {
        start_y + search_len_y
    };

    // Search for symbols to see if it's a part number
    for line in &schematic[start_y..end_y] {
        if line[start_x..end_x].contains(|c: char| !c.is_digit(RADIX) && c != '.') {
            return true;
        }
    }

    return false;
}
