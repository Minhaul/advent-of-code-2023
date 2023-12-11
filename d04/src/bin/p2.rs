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

    let mut copies: Vec<u32> = vec![1; cards.len()];
    for (idx, card) in cards.into_iter().enumerate() {
        let curr_copies = copies[idx];
        let winners = card.num_winners();
        // oh god this is ugly
        for i in idx + 1..idx + 1 + winners as usize {
            copies[i] += curr_copies;
        }
    }

    let result = copies.into_iter().reduce(|acc, c| acc + c).unwrap();

    println!("{:?}", result);
}
