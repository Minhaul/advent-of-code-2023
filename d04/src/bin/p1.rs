use d04::*;
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

    let mut cards: Vec<Card> = Vec::new();
    for line in input_str {
        cards.push(Card::from_string(&line));
    }

    let scores: Vec<u32> = cards.into_iter().map(|c| score(c.num_winners())).collect();
    let result = scores.into_iter().reduce(|acc, s| acc + s).unwrap();

    println!("{:?}", result);
}

fn score(num_winners: u32) -> u32 {
    if num_winners > 0 {
        let base: u32 = 2;
        base.pow(num_winners - 1)
    } else {
        0
    }
}
