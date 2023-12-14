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

    let times: Vec<&str> = input_str[0]
        .split_whitespace()
        .filter(|s| s.parse::<u32>().is_ok())
        .collect();

    let mut time_string: String = String::from("");
    for i in 0..times.len() {
        time_string.push_str(times[i]);
    }

    let records: Vec<&str> = input_str[1]
        .split_whitespace()
        .filter(|s| s.parse::<u32>().is_ok())
        .collect();

    let mut record_string: String = String::from("");
    for i in 0..records.len() {
        record_string.push_str(records[i]);
    }

    let time: u64 = time_string.parse().unwrap();
    let record: u64 = record_string.parse().unwrap();

    let mut ways_to_win = 0;
    for hold_time in 1..time {
        let speed = hold_time;
        let distance = speed * (time - hold_time);
        if distance > record {
            ways_to_win += 1;
        } else if ways_to_win > 0 {
            break;
        }
    }

    let result = ways_to_win;
    println!("{:?}", result);
}
