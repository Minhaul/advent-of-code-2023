use file_reader;

const INPUT_FILENAME: &str = "input.txt";
const RADIX: u32 = 10;

fn main() {
    let input_str = match file_reader::file_to_vec(INPUT_FILENAME) {
        Err(_) => {
            println!("Couldn't turn file into vec!");
            return;
        },
        Ok(v) => v,
    };

    let cal_values = input_str.into_iter().map(|s| get_calibration_value(&s));
    let result = cal_values.reduce(|acc, x| acc + x).unwrap();

    println!("{:?}", result);
}

fn get_calibration_value(s: &String) -> u32 {
    let first_num: u32 = s.chars().find_map(|x| x.to_digit(RADIX)).unwrap();
    let last_num: u32 = s.chars().rev().find_map(|x| x.to_digit(RADIX)).unwrap();

    first_num * RADIX + last_num
}
