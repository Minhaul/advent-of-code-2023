use d07::*;
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

    let mut hand_bids: Vec<(Hand, u32)> = Vec::new();
    for line in input_str {
        let split: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(split.len(), 2);
        hand_bids.push((Hand::from_str_p1(&split[0]), split[1].parse().unwrap()));
    }

    hand_bids.sort_by_key(|hb| hb.0.clone());

    let mut total_winnings: Vec<u32> = Vec::new();
    for (idx, entry) in hand_bids.into_iter().enumerate() {
        let rank = (idx + 1) as u32;
        total_winnings.push(entry.1 * rank);
    }

    let result = total_winnings.into_iter().reduce(|acc, w| acc + w).unwrap();
    println!("{:?}", result);
}
