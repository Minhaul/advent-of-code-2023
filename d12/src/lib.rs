use anyhow::{anyhow, Error};
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    str::FromStr,
};

const UNKNOWN: char = '?';
const OPERATIONAL: char = '.';
const DAMAGED: char = '#';

#[derive(Debug)]
pub struct SpringCondition {
    row: Vec<SpringType>,
    contiguous: Vec<usize>,
}

#[derive(Debug, PartialEq, Clone, Copy, Hash)]
enum SpringType {
    Unknown,
    Operational,
    Damaged,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum NumArrangementsErr {
    CannotFit,
    InvalidLoc,
}

impl SpringCondition {
    pub fn unfold(&mut self, mult: usize) {
        if mult < 2 {
            return;
        }

        self.row.push(SpringType::Unknown);
        let row_len = self.row.len();
        let con_len = self.contiguous.len();
        for _ in 1..mult {
            self.row.extend_from_within(..row_len);
            self.contiguous.extend_from_within(..con_len);
        }
        self.row.pop();
    }

    pub fn num_arrangements(&self) -> u64 {
        let mut memoized = HashMap::new();
        Self::prv_num_arrangements(&self.row, &self.contiguous, &mut memoized).unwrap_or(0)
    }

    fn prv_num_arrangements(
        row: &[SpringType],
        contiguous: &[usize],
        memoized: &mut HashMap<u64, Result<u64, NumArrangementsErr>>,
    ) -> Result<u64, NumArrangementsErr> {
        let Some((&num_con, next_con)) = contiguous.split_first() else {
            // `contiguous` is empty
            return Ok(1);
        };

        if row.len() < num_con {
            return Err(NumArrangementsErr::CannotFit);
        }

        // Generate hash for this input
        let mut hasher = DefaultHasher::new();
        for spring in row {
            spring.hash(&mut hasher);
        }
        for num in contiguous {
            num.hash(&mut hasher);
        }
        let hash = hasher.finish();

        // Check to see if this result has been saved
        if let Some(ret) = memoized.get(&hash) {
            return *ret;
        }

        // Is the start of this row a valid place for a set of contiguous damaged springs?
        let (left, right) = row.split_at(num_con);
        let valid = !left.contains(&SpringType::Operational)
            && right.first().unwrap_or(&SpringType::Operational) != &SpringType::Damaged
            && !(next_con.is_empty() && right.contains(&SpringType::Damaged));

        let num_arrangements = match valid {
            false => 0,
            true => {
                let next_row = match right.split_first() {
                    Some((_, r)) => r,
                    None => &[],
                };
                match Self::prv_num_arrangements(next_row, next_con, memoized) {
                    Ok(num) => num,
                    Err(NumArrangementsErr::InvalidLoc) => 0,
                    Err(NumArrangementsErr::CannotFit) => {
                        let ret = Err(NumArrangementsErr::CannotFit);
                        memoized.insert(hash, ret);
                        return ret;
                    }
                }
            }
        };

        // Can we keep pushing this set of contiguous damaged springs forward?
        let can_push = row.first().unwrap() != &SpringType::Damaged;
        // Find the next potentially valid position to check number of arrangements
        let maybe_next_idx = row[1..].iter().position(|x| x != &SpringType::Operational);

        if !can_push || maybe_next_idx.is_none() {
            let ret = match num_arrangements {
                0 => Err(NumArrangementsErr::InvalidLoc),
                num => Ok(num),
            };
            memoized.insert(hash, ret);
            return ret;
        }

        let next_idx = maybe_next_idx.unwrap();
        let ret = match (
            num_arrangements,
            Self::prv_num_arrangements(&row[next_idx + 1..], contiguous, memoized),
        ) {
            (num_arrangements, Ok(num)) => Ok(num_arrangements + num),
            (0, Err(e)) => Err(e),
            (num_arrangements, Err(_)) => Ok(num_arrangements),
        };
        memoized.insert(hash, ret);
        ret
    }
}

impl FromStr for SpringCondition {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let delimiter_idx = s.find(' ').ok_or(anyhow!("Invalid format"))?;
        let mut row = Vec::new();
        for c in s[..delimiter_idx].chars() {
            row.push(match c {
                UNKNOWN => SpringType::Unknown,
                OPERATIONAL => SpringType::Operational,
                DAMAGED => SpringType::Damaged,
                _ => return Err(anyhow!("Invalid format")),
            });
        }

        let contiguous = s[delimiter_idx + 1..]
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(SpringCondition { row, contiguous })
    }
}
