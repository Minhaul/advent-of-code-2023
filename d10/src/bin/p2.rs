use d10::*;
use file_reader;
use utils::Coord;

const INPUT_FILENAME: &str = "input.txt";

type IntermedMap = Vec<Vec<Option<NorthSouth>>>;

#[derive(Debug, Clone, Copy)]
struct NorthSouth {
    north: bool,
    south: bool,
}

fn main() {
    let input_str = match file_reader::file_to_vec(INPUT_FILENAME) {
        Err(_) => {
            println!("Couldn't turn file into vec!");
            return;
        }
        Ok(v) => v,
    };

    // Intermediate map that denotes which locations contain a piece of the loop
    // and if so, does that piece have a connection to the north and/or south
    let mut intermed_map: IntermedMap = vec![vec![None; input_str[0].len()]; input_str.len()];

    let start_coord = find_start_coord(&input_str);

    let (mut direc, mut coord) = find_connecting_pipe(&input_str, start_coord);
    let north = direc == Direction::North;
    let south = direc == Direction::South;
    mark_intermed_map(&mut intermed_map, start_coord, NorthSouth { north, south });
    loop {
        let pipe = get_pipe_at_coord(&input_str, coord);
        if pipe == START {
            let mut curr_ns = intermed_map[coord.y][coord.x].unwrap();
            match direc {
                Direction::North => curr_ns.south = true,
                Direction::South => curr_ns.north = true,
                _ => (),
            }
            mark_intermed_map(&mut intermed_map, coord, curr_ns);
            break;
        }

        let mut north = false;
        let mut south = false;
        match pipe {
            NE | NW => north = true,
            SE | SW => south = true,
            NS => {
                north = true;
                south = true;
            }
            _ => (),
        }
        mark_intermed_map(&mut intermed_map, coord, NorthSouth { north, south });

        (direc, coord) = find_next_pipe(&input_str, direc, coord);
    }

    // Go through intermediate map to count spaces inside the loop
    let mut inside_count = 0;
    for row in intermed_map.into_iter() {
        let mut inside = false;
        let mut north = false;
        let mut south = false;
        for space in row {
            match space {
                Some(ns) => {
                    if ns.north {
                        north = !north;
                    }
                    if ns.south {
                        south = !south;
                    }

                    if north && south {
                        inside = !inside;
                        north = false;
                        south = false;
                    }
                }
                None => inside_count += if inside { 1 } else { 0 },
            }
        }
    }

    let result = inside_count;
    println!("{:?}", result);
}

#[inline]
fn mark_intermed_map(map: &mut IntermedMap, coord: Coord, north_south: NorthSouth) {
    map[coord.y][coord.x] = Some(north_south);
}
