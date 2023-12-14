use std::cmp::Ordering;

const HAND_SIZE: usize = 5;

#[derive(Debug, Clone, Ord, Eq)]
pub struct Hand {
    cards: [Card; HAND_SIZE],
    hand_type: HandType,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    pub fn from_str_p1(s: &str) -> Hand {
        assert_eq!(HAND_SIZE, s.len());
        let mut cards_vec = Vec::new();

        for c in s.chars() {
            cards_vec.push(match c {
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                'T' => Card::Ten,
                'J' => Card::Jack,
                'Q' => Card::Queen,
                'K' => Card::King,
                'A' => Card::Ace,
                _ => panic!("BAD CARD"),
            });
        }

        let mut hand = Hand {
            cards: cards_vec.try_into().unwrap(),
            hand_type: HandType::HighCard,
        };

        hand.hand_type = hand.calc_hand_type();

        hand
    }

    pub fn from_str_p2(s: &str) -> Hand {
        assert_eq!(HAND_SIZE, s.len());
        let mut cards_vec = Vec::new();

        for c in s.chars() {
            cards_vec.push(match c {
                'J' => Card::Joker,
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                'T' => Card::Ten,
                'Q' => Card::Queen,
                'K' => Card::King,
                'A' => Card::Ace,
                _ => panic!("BAD CARD"),
            });
        }

        let mut hand = Hand {
            cards: cards_vec.try_into().unwrap(),
            hand_type: HandType::HighCard,
        };

        hand.hand_type = hand.calc_hand_type();

        hand
    }

    fn calc_hand_type(&self) -> HandType {
        let mut sorted_hand = self.clone();
        sorted_hand.cards.sort();

        type CardCount = (u32, Card);

        let mut card_counts: Vec<CardCount> = Vec::new();
        for card in sorted_hand.cards {
            if card_counts.last().is_some() && card_counts.last().unwrap().1 == card {
                let mut new_count = card_counts.last().unwrap().clone();
                new_count.0 += 1;
                *card_counts.last_mut().unwrap() = new_count;
            } else {
                card_counts.push((1, card));
            }
        }

        card_counts.sort_by_key(|c| c.0);
        card_counts.reverse();

        if card_counts.len() > 1 {
            let mut joker_count = 0;
            for idx in 0..card_counts.len() {
                if card_counts[idx].1 == Card::Joker {
                    (joker_count, _) = card_counts.remove(idx);
                    break;
                }
            }

            let mut new_count = card_counts[0].clone();
            new_count.0 += joker_count;
            card_counts[0] = new_count;
        }

        let mut intermediate_type = HandType::HighCard;
        for card_count in card_counts {
            let count = card_count.0;
            match count {
                1 => (),
                2 => {
                    intermediate_type = match intermediate_type {
                        HandType::HighCard => HandType::OnePair,
                        HandType::OnePair => HandType::TwoPair,
                        HandType::ThreeOfAKind => HandType::FullHouse,
                        _ => panic!("BAD HAND TYPE"),
                    };
                }
                3 => {
                    intermediate_type = match intermediate_type {
                        HandType::HighCard => HandType::ThreeOfAKind,
                        HandType::OnePair => HandType::FullHouse,
                        _ => panic!("BAD HAND TYPE"),
                    };
                }
                4 => intermediate_type = HandType::FourOfAKind,
                5 => intermediate_type = HandType::FiveOfAKind,
                _ => panic!("BAD COUNT"),
            }
        }

        intermediate_type
    }

    pub fn hand_type(&self) -> HandType {
        self.hand_type.clone()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.eq(other) {
            return Some(Ordering::Equal);
        }

        let type_self = self.hand_type();
        let type_other = other.hand_type();

        if type_self > type_other {
            return Some(Ordering::Greater);
        } else if type_self < type_other {
            return Some(Ordering::Less);
        } else {
            for idx in 0..self.cards.len() {
                if self.cards[idx] != other.cards[idx] {
                    return self.cards[idx].partial_cmp(&other.cards[idx]);
                }
            }
        }

        // Should never get here
        return None;
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        for idx in 0..self.cards.len() {
            if self.cards[idx] != other.cards[idx] {
                return false;
            }
        }

        return true;
    }
}
