use utils::Coord;

pub const NS: char = '|';
pub const EW: char = '-';
pub const NE: char = 'L';
pub const NW: char = 'J';
pub const SW: char = '7';
pub const SE: char = 'F';
pub const START: char = 'S';

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub fn find_start_coord(s: &Vec<String>) -> Coord {
    let mut start_coord = Coord { x: 0, y: 0 };

    for (y, line) in s.into_iter().enumerate() {
        match line.find(START) {
            Some(x) => {
                start_coord = Coord { x, y };
                break;
            }
            None => (),
        }
    }

    start_coord
}

pub fn find_connecting_pipe(s: &Vec<String>, start: Coord) -> (Direction, Coord) {
    let mut dir = Direction::North;
    let mut coord = start;

    // Look North
    let n_coord = Coord {
        x: start.x,
        y: start.y - 1,
    };
    match get_pipe_at_coord(s, n_coord) {
        NS | SE | SW => {
            dir = Direction::North;
            coord = n_coord;
        }
        _ => (),
    }

    // Look East
    let e_coord = Coord {
        x: start.x + 1,
        y: start.y,
    };
    match get_pipe_at_coord(s, e_coord) {
        EW | NW | SW => {
            dir = Direction::East;
            coord = e_coord;
        }
        _ => (),
    }

    // Look South
    let s_coord = Coord {
        x: start.x,
        y: start.y + 1,
    };
    match get_pipe_at_coord(s, s_coord) {
        NS | NE | NW => {
            dir = Direction::South;
            coord = s_coord;
        }
        _ => (),
    }

    // Look West
    let w_coord = Coord {
        x: start.x - 1,
        y: start.y,
    };
    match get_pipe_at_coord(s, w_coord) {
        EW | NE | SE => {
            dir = Direction::West;
            coord = w_coord;
        }
        _ => (),
    }

    (dir, coord)
}

pub fn find_next_pipe(s: &Vec<String>, travel_dir: Direction, start: Coord) -> (Direction, Coord) {
    let new_dir = match get_pipe_at_coord(s, start) {
        NS => {
            match travel_dir {
                Direction::North => Direction::North,
                Direction::South => Direction::South,
                _ => panic!("BAD DIRECTION")
            }
        }
        EW => {
            match travel_dir {
                Direction::East => Direction::East,
                Direction::West => Direction::West,
                _ => panic!("BAD DIRECTION")
            }
        }
        NE => {
            match travel_dir {
                Direction::South => Direction::East,
                Direction::West => Direction::North,
                _ => panic!("BAD DIRECTION")
            }
        }
        NW => {
            match travel_dir {
                Direction::East => Direction::North,
                Direction::South => Direction::West,
                _ => panic!("BAD DIRECTION")
            }
        }
        SW => {
            match travel_dir {
                Direction::North => Direction::West,
                Direction::East => Direction::South,
                _ => panic!("BAD DIRECTION")
            }
        }
        SE => {
            match travel_dir {
                Direction::North => Direction::East,
                Direction::West => Direction::South,
                _ => panic!("BAD DIRECTION")
            }
        }
        _ => panic!("BAD PIPE")
    };

    let mut new_coord = start;
    match new_dir {
        Direction::North => new_coord.y -= 1,
        Direction::East => new_coord.x += 1,
        Direction::South => new_coord.y += 1,
        Direction::West => new_coord.x -= 1,
    }

    (new_dir, new_coord)
}

#[inline]
pub fn get_pipe_at_coord(s: &Vec<String>, coord: Coord) -> char {
    s[coord.y].chars().nth(coord.x).unwrap()
}
