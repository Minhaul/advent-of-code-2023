//const RADIX: u32 = 10; UNUSED

#[derive(Debug)]
pub struct Card {
    //id: u32, UNUSED
    winners: Vec<u32>,
    held: Vec<u32>,
}

impl Card {
    pub fn from_string(s: &String) -> Card {
        //let id: u32 = s[s.find(|c: char| c.is_digit(RADIX)).unwrap()..s.find(":").unwrap()]
        //    .parse::<u32>()
        //    .unwrap();
        //UNUSED

        let mut winners: Vec<u32> = s[s.find(":").unwrap() + 2..s.find(" |").unwrap()]
            .split(" ")
            .filter(|s| s != &"")
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        let mut held: Vec<u32> = s[s.find("|").unwrap() + 2..]
            .split(" ")
            .filter(|s| s != &"")
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        winners.sort();
        held.sort();

        Card { /*id,*/ winners, held }
    }

    pub fn num_winners(&self) -> u32 {
        let mut num_winners = 0;
        for num in &self.held {
            match &self.winners.binary_search(&num) {
                Ok(_) => num_winners += 1,
                Err(_) => (),
            }
        }

        num_winners
    }
}
