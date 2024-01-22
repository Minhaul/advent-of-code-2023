use d10::*;

const INPUT_FILENAME: &str = "input.txt";

fn main() {
    let input_str = match utils::file_to_vec(INPUT_FILENAME) {
        Err(_) => {
            println!("Couldn't turn file into vec!");
            return;
        }
        Ok(v) => v,
    };

    let start_coord = find_start_coord(&input_str);

    // Find one of the starting directions to go down
    let (mut direc, mut coord) = find_connecting_pipe(&input_str, start_coord);
    let mut pipe_len = 1;
    loop {
        (direc, coord) = find_next_pipe(&input_str, direc, coord);
        pipe_len += 1;

        if get_pipe_at_coord(&input_str, coord) == START {
            break;
        }
    }

    let result = pipe_len / 2;
    println!("{:?}", result);
}
