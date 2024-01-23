use d12::*;

const INPUT_FILENAME: &str = "input.txt";

fn main() {
    let input_str = match utils::file_to_vec(INPUT_FILENAME) {
        Ok(s) => s,
        Err(e) => panic!("Couldn't turn file into vec: {e:?}"),
    };

    let mut num_arrangements: u64 = 0;
    for line in input_str.iter() {
        let mut condition = match line.parse::<SpringCondition>() {
            Ok(c) => c,
            Err(e) => panic!("{e:?}"),
        };

        condition.unfold(5);

        num_arrangements += condition.num_arrangements();
    }

    let result = num_arrangements;
    println!("{result}");
}
