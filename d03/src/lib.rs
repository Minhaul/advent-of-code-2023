pub const RADIX: u32 = 10;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Number {
    pub start_coord: Coord,
    pub len: usize,
    pub value: u32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}
