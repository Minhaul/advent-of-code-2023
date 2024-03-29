use d11::*;

const INPUT_FILENAME: &str = "input.txt";

fn main() {
    let Ok(input_str) = utils::file_to_vec(INPUT_FILENAME) else {
        panic!("Couldn't turn file into vec!");
    };

    let mut universe = Universe::new(&input_str);
    universe.expand(2);

    let mut distances = 0;
    for (idx, galaxy) in universe.galaxies.iter().enumerate() {
        for neighbor in universe.galaxies[idx + 1..].into_iter() {
            distances += galaxy.x.abs_diff(neighbor.x) + galaxy.y.abs_diff(neighbor.y);
        }
    }

    let result = distances;
    println!("{:?}", result);
}
