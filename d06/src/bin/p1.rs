const INPUT_FILENAME: &str = "input.txt";

fn main() {
    let input_str = match utils::file_to_vec(INPUT_FILENAME) {
        Err(_) => {
            println!("Couldn't turn file into vec!");
            return;
        }
        Ok(v) => v,
    };

    let times: Vec<u32> = input_str[0]
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

    let records: Vec<u32> = input_str[1]
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

    assert_eq!(times.len(), records.len());

    let mut win_margins: Vec<u32> = Vec::new();
    for race in 0..times.len() {
        let mut ways_to_win = 0;
        for time in 1..times[race] {
            let speed = time;
            let distance = speed * (times[race] - time);
            if distance > records[race] {
                ways_to_win += 1;
            } else if ways_to_win > 0 {
                break;
            }
        }

        win_margins.push(ways_to_win);
    }

    let result = win_margins.into_iter().reduce(|acc, m| acc * m).unwrap();
    println!("{:?}", result);
}
