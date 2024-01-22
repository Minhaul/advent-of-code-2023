use d08::*;

const INPUT_FILENAME: &str = "input.txt";

fn main() {
    let input_str = match utils::file_to_vec(INPUT_FILENAME) {
        Err(_) => {
            println!("Couldn't turn file into vec!");
            return;
        }
        Ok(v) => v,
    };

    let mut network_strings = input_str;
    let direction_string = network_strings.remove(0);
    network_strings.remove(0);

    let mut starting_nodes: Vec<Location> = Vec::new();
    for node in &network_strings {
        let first_location = &node[0..node.find(" ").unwrap()];
        assert_eq!(first_location.len(), LOCATION_LEN);
        if first_location.as_bytes()[LOCATION_LEN - 1] == 'A' as u8 {
            starting_nodes.push(String::from(first_location));
        }
    }
    let map = Map::from_strings(&direction_string, network_strings);

    let mut traverses: Vec<MapTraverse> = Vec::new();
    for node in starting_nodes.iter() {
        traverses.push(MapTraverse::from_map(&map, node));
    }

    let mut loop_lens: Vec<u64> = Vec::new();
    for traverse in traverses.iter_mut() {
        let mut loop_len = 0;
        let mut curr_loop_len = 0;
        loop {
            let location = traverse.traverse();
            curr_loop_len += 1;

            if location.as_bytes()[LOCATION_LEN - 1] == 'Z' as u8 {
                if loop_len == 0 {
                    // Go again to make sure this isn't a half loop
                    loop_len = curr_loop_len;
                    curr_loop_len = 0;
                } else {
                    if loop_len != curr_loop_len {
                        panic!("BAD LOOP LENS");
                    }

                    loop_lens.push(loop_len);
                    break;
                }
            }
        }
    }

    let result = lcm(&loop_lens);

    println!("{:?}", result);
}

fn lcm(nums: &Vec<u64>) -> u64 {
    struct Lcm {
        original: u64,
        curr_prod: u64,
        curr_mult: u64,
    }

    let mut sorted = nums.clone();
    sorted.sort();

    let mut working_zone: Vec<Lcm> = Vec::new();
    for num in sorted {
        working_zone.push(Lcm {
            original: num,
            curr_prod: num,
            curr_mult: 1,
        });
    }

    loop {
        // Check for equality
        let mut value = None;
        let mut all_equal = true;
        for lcm in &working_zone {
            if value.is_none() {
                value = Some(lcm.curr_prod);
            } else {
                if value.unwrap() != lcm.curr_prod {
                    all_equal = false;
                    break;
                }
            }
        }

        if all_equal {
            break;
        }

        // Not all equal, keep working
        working_zone.sort_by_key(|l| l.curr_prod);
        let first = &mut working_zone[0];
        first.curr_mult += 1;
        first.curr_prod = first.original * first.curr_mult;
    }

    working_zone[0].curr_prod
}
