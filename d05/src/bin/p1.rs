use file_reader;

const INPUT_FILENAME: &str = "input.txt";

#[derive(Debug)]
struct MapEntry {
    dest_range_start: u64,
    source_range_start: u64,
    range_len: u64,
}

fn main() {
    let input_str = match file_reader::file_to_vec(INPUT_FILENAME) {
        Err(_) => {
            println!("Couldn't turn file into vec!");
            return;
        }
        Ok(v) => v,
    };

    let seeds_str = &input_str[0];
    let seeds: Vec<u64> = seeds_str[seeds_str.find(":").unwrap() + 2..]
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let maps: Vec<Vec<MapEntry>> = find_maps(&input_str);

    let mut locations: Vec<u64> = Vec::new();
    for seed in seeds {
        let mut curr_source_value = seed;
        for map in &maps {
            for entry in map {
                if entry.source_range_start <= curr_source_value
                    && curr_source_value < entry.source_range_start + entry.range_len
                {
                    curr_source_value =
                        entry.dest_range_start + curr_source_value - entry.source_range_start;
                    break;
                }
            }
        }

        locations.push(curr_source_value);
    }

    locations.sort();
    let result = locations[0];

    println!("{:?}", result);
}

fn find_maps(s: &Vec<String>) -> Vec<Vec<MapEntry>> {
    let mut maps: Vec<Vec<MapEntry>> = Vec::new();

    let mut curr_map_start_idx: usize = 0;

    // Find the start of the first map
    for (i, line) in s.into_iter().enumerate() {
        if line.contains("map") {
            curr_map_start_idx = i + 1;
            break;
        }
    }

    // Now actually find all the maps
    for (i, line) in s.into_iter().enumerate() {
        if i < curr_map_start_idx {
            continue;
        }

        if line.contains("map") {
            let mut map: Vec<MapEntry> = Vec::new();
            for line in &s[curr_map_start_idx..i - 1] {
                map.push(build_map_entry(line));
            }

            maps.push(map);
            curr_map_start_idx = i + 1;
        }
    }

    // Add the last map
    let mut map: Vec<MapEntry> = Vec::new();
    for line in &s[curr_map_start_idx..] {
        map.push(build_map_entry(line));
    }

    maps.push(map);

    maps
}

fn build_map_entry(map_entry: &String) -> MapEntry {
    let entry: Vec<u64> = map_entry
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    assert_eq!(entry.len(), 3);

    MapEntry {
        dest_range_start: entry[0],
        source_range_start: entry[1],
        range_len: entry[2],
    }
}
