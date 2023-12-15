use std::collections::HashMap;

pub type Location = String;
pub const LOCATION_LEN: usize = 3;

#[derive(Debug)]
pub struct MapTraverse<'a> {
    map: &'a Map,
    state: TraverseState<'a>,
}

#[derive(Debug)]
struct TraverseState<'a> {
    direction_idx: usize,
    curr_location: &'a Location,
}

#[derive(Debug)]
pub struct Map {
    directions: Vec<Direction>,
    network: HashMap<Location, Destinations>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Destinations {
    left: Location,
    right: Location,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Map {
    pub fn from_strings(dir_string: &String, network_strings: Vec<String>) -> Map {
        // Build direction list
        let mut dir_list = Vec::new();

        for c in dir_string.chars() {
            dir_list.push(match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("BAD DIRECTION"),
            });
        }

        // Build network map
        let mut map = HashMap::new();

        for line in network_strings {
            let key = Location::from(&line[0..line.find(" ").unwrap()]);
            assert_eq!(key.len(), LOCATION_LEN);
            let value_str = &line[line.find("=").unwrap() + 3..line.len() - 1];

            let value = Destinations {
                left: Location::from(&value_str[0..value_str.find(",").unwrap()]),
                right: Location::from(&value_str[value_str.find(",").unwrap() + 2..]),
            };
            assert_eq!(value.left.len(), LOCATION_LEN);
            assert_eq!(value.right.len(), LOCATION_LEN);

            map.insert(key, value);
        }

        Map {
            directions: dir_list,
            network: map,
        }
    }
}

impl<'a> MapTraverse<'a> {
    pub fn from_map(map: &'a Map, start: &'a Location) -> MapTraverse<'a> {
        assert_eq!(start.len(), LOCATION_LEN);
        MapTraverse {
            map,
            state: TraverseState {
                direction_idx: 0,
                curr_location: start,
            },
        }
    }

    pub fn traverse(&mut self) -> Location {
        let direction = self.map.directions[self.state.direction_idx];
        let new_destinations = self.map.network.get(self.state.curr_location).unwrap();
        self.state.curr_location = match direction {
            Direction::Left => &new_destinations.left,
            Direction::Right => &new_destinations.right,
        };

        self.state.direction_idx += 1;
        if self.state.direction_idx == self.map.directions.len() {
            self.state.direction_idx = 0;
        }

        self.state.curr_location.clone()
    }
}
