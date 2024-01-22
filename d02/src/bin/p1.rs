use d02::*;

const INPUT_FILENAME: &str = "input.txt";

const RED_MAX: u32 = 12;
const GREEN_MAX: u32 = 13;
const BLUE_MAX: u32 = 14;

fn main() {
    let input_str = match utils::file_to_vec(INPUT_FILENAME) {
        Err(_) => {
            println!("Couldn't turn file into vec!");
            return;
        }
        Ok(v) => v,
    };

    let games = input_str.into_iter().map(|s| Game::from_string(&s));
    let result = games
        .filter(|g| is_game_valid(&g))
        .fold(0, |acc, g| acc + g.id);

    println!("{:?}", result);
}

fn is_game_valid(g: &Game) -> bool {
    for pull in &g.pulls {
        if pull.red > RED_MAX || pull.green > GREEN_MAX || pull.blue > BLUE_MAX {
            return false;
        }
    }

    true
}
