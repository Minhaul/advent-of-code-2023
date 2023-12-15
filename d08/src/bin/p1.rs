use d08::*;
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

    let mut network_strings = input_str;
    let direction_string = network_strings.remove(0);
    network_strings.remove(0);
    let map = Map::from_strings(&direction_string, network_strings);

    let start = Location::from("AAA");
    let mut traverse = MapTraverse::from_map(&map, &start);

    let mut result = 0;
    loop {
        let location = traverse.traverse();
        result += 1;

        if location == "ZZZ" {
            break;
        }
    }

    println!("{:?}", result);
}
