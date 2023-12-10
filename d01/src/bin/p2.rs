use file_reader;

const INPUT_FILENAME: &str = "input.txt";
const RADIX: u32 = 10;
const DIGITS: [(u32, &str); 9] =
[
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
];

#[derive(Debug)]
struct IdxVal {
    index: usize,
    value: u32,
}

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
    let mut first_num = IdxVal {
        index: usize::MAX,
        value: 0,
    };
    let mut last_num = IdxVal {
        index: 0,
        value: 0,
    };

    for digit in DIGITS {
        let digit_str: String = digit.0.to_string();
        let mut number_matches = s.match_indices(&digit_str);
        let mut name_matches = s.match_indices(digit.1);

        let mut first: Option<usize> = None;
        let mut last: Option<usize> = None;
        match (number_matches.next(), name_matches.next()) {
            (Some((m1, _)), Some((m2, _))) => {
                first = Some(m1.min(m2));
                last = Some(m1.max(m2));
            },
            (Some((m1, _)), None) => {
                first = Some(m1);
                last = Some(m1);
            },
            (None, Some((m2, _))) => {
                first = Some(m2);
                last = Some(m2);
            },
            (None, None) => (),
        }

        let mut later: Option<usize> = None;
        match (number_matches.last(), name_matches.last()) {
            (Some((m1, _)), Some((m2, _))) => {
                later = Some(m1.max(m2));
            },
            (Some((m1, _)), None) => {
                later = Some(m1);
            },
            (None, Some((m2, _))) => {
                later = Some(m2);
            },
            (None, None) => (),
        }
        if last == None || (later != None && later.unwrap() > last.unwrap()) {
            last = later;
        }

        if first != None && first.unwrap() < first_num.index {
            first_num.index = first.unwrap();
            first_num.value = digit.0;
        }

        if last != None && last.unwrap() >= last_num.index {
            last_num.index = last.unwrap();
            last_num.value = digit.0;
        }
    }

    first_num.value * RADIX + last_num.value
}
