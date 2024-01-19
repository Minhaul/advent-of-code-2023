use utils::Coord;

const GALAXY: char = '#';

#[derive(Debug)]
pub struct Universe {
    pub galaxies: Vec<Coord>,
    width: usize,
    height: usize,
}

impl Universe {
    pub fn new(input: &Vec<String>) -> Universe {
        let mut galaxies = Vec::new();

        for (y, line) in input.into_iter().enumerate() {
            let mut idx = 0;
            while let Some(x) = line[idx..].find(GALAXY) {
                galaxies.push(Coord { x: idx + x, y });
                idx = idx + x + 1;
            }
        }

        Universe {
            width: input[0].len(),
            height: input.len(),
            galaxies,
        }
    }

    pub fn expand(&mut self, multiplier: usize) {
        let mut rows_temp = Vec::new();
        let mut cols_temp = Vec::new();

        let max = self.width.max(self.height);

        for i in 0..max {
            if i < self.width {
                cols_temp.push(Some(i));
            }

            if i < self.height {
                rows_temp.push(Some(i));
            }
        }

        for galaxy in self.galaxies.iter() {
            rows_temp[galaxy.y] = None;
            cols_temp[galaxy.x] = None;
        }

        let rows: Vec<usize> = rows_temp.into_iter().filter_map(|r| r).collect();
        let cols: Vec<usize> = cols_temp.into_iter().filter_map(|c| c).collect();

        for galaxy in self.galaxies.iter_mut() {
            let mut incr = 0;
            for row in rows.iter() {
                if row > &galaxy.y {
                    break;
                }

                incr += multiplier - 1;
            }
            galaxy.y += incr;

            incr = 0;
            for col in cols.iter() {
                if col > &galaxy.x {
                    break;
                }

                incr += multiplier - 1;
            }
            galaxy.x += incr;
        }
    }
}
