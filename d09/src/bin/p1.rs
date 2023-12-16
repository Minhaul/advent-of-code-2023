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

    let mut next_vals: Vec<i64> = Vec::new();
    for line in input_str {
        let num_seq: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        next_vals.push(next_val(&num_seq));
    }

    let result = next_vals.into_iter().reduce(|acc, v| acc + v).unwrap();

    println!("{:?}", result);
}

fn next_val(seq: &Vec<i64>) -> i64 {
    if all_zeros(seq) {
        return 0;
    }

    let mut diffs_seq: Vec<i64> = Vec::new();
    for idx in 1..seq.len() {
        diffs_seq.push(seq[idx] - seq[idx - 1]);
    }

    seq.last().unwrap() + next_val(&diffs_seq)
}

fn all_zeros(seq: &Vec<i64>) -> bool {
    for val in seq {
        if *val != 0 {
            return false;
        }
    }

    return true;
}
