use d02::*;
use file_reader;

const INPUT_FILENAME: &str = "input.txt";

fn main() {
    let input_str = match file_reader::file_to_vec(INPUT_FILENAME) {
        Err(_) => {
            println!("Couldn't turn file into vec!");
            return;
        }
        Ok(v) => v,
    };

    let games = input_str.into_iter().map(|s| Game::from_string(&s));
    let result = games
        .map(|g| get_minimum_power(&g))
        .reduce(|acc, p| acc + p)
        .unwrap();

    println!("{:?}", result);
}

fn get_minimum_power(g: &Game) -> u32 {
    let mut max_red: u32 = 0;
    let mut max_green: u32 = 0;
    let mut max_blue: u32 = 0;

    for pull in &g.pulls {
        if pull.red > max_red {
            max_red = pull.red;
        }
        if pull.green > max_green {
            max_green = pull.green;
        }
        if pull.blue > max_blue {
            max_blue = pull.blue;
        }
    }

    max_red * max_green * max_blue
}
