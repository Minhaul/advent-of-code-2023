#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub pulls: Vec<CubePull>,
}

#[derive(Debug)]
pub struct CubePull {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

const RED_STR: &str = "red";
const GREEN_STR: &str = "green";
const BLUE_STR: &str = "blue";

impl Game {
    pub fn from_string(s: &String) -> Game {
        let id: u32 = s[s.find(" ").unwrap() + 1..s.find(":").unwrap()]
            .parse::<u32>()
            .unwrap();
        let mut game = Game {
            id,
            pulls: Vec::new(),
        };

        let pulls = s[s.find(":").unwrap() + 2..].split("; ");
        for pull in pulls {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            let cubes = pull.split(", ");
            for cube in cubes {
                let num = cube[..cube.find(" ").unwrap()].parse::<u32>().unwrap();
                match &cube[cube.find(" ").unwrap() + 1..] {
                    RED_STR => red = num,
                    GREEN_STR => green = num,
                    BLUE_STR => blue = num,
                    _ => println!("BAD COLOR"),
                }
            }

            game.pulls.push(CubePull { red, green, blue });
        }

        game
    }
}
