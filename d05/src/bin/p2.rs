const INPUT_FILENAME: &str = "input.txt";

#[derive(Debug)]
struct SeedEntry {
    range_start: u64,
    range_len: u64,
}

#[derive(Debug)]
struct MapEntry {
    dest_range_start: u64,
    source_range_start: u64,
    range_len: u64,
}

fn main() {
    let input_str = match utils::file_to_vec(INPUT_FILENAME) {
        Err(_) => {
            println!("Couldn't turn file into vec!");
            return;
        }
        Ok(v) => v,
    };

    let seeds_str = &input_str[0];
    let seeds: Vec<SeedEntry> = build_seed_entries(&seeds_str[seeds_str.find(":").unwrap() + 2..]);

    let mut maps: Vec<Vec<MapEntry>> = find_maps(&input_str);
    maps.reverse();

    let mut result = 0;
    'main: for i in 0.. {
        let mut curr_dest_value = i;
        for map in &maps {
            for entry in map {
                if entry.dest_range_start <= curr_dest_value
                    && curr_dest_value < entry.dest_range_start + entry.range_len
                {
                    curr_dest_value =
                        entry.source_range_start + curr_dest_value - entry.dest_range_start;
                    break;
                }
            }
        }

        // Is this a valid seed?
        let seed = curr_dest_value;
        for seed_entry in &seeds {
            if seed_entry.range_start < seed && seed < seed_entry.range_start + seed_entry.range_len
            {
                result = i;
                break 'main;
            }
        }
    }

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

fn build_seed_entries(seed_entries: &str) -> Vec<SeedEntry> {
    let values: Vec<u64> = seed_entries
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    assert_eq!(values.len() % 2, 0);

    let mut entries: Vec<SeedEntry> = Vec::new();
    for i in (0..values.len()).step_by(2) {
        entries.push(SeedEntry {
            range_start: values[i],
            range_len: values[i + 1],
        });
    }

    entries
}
