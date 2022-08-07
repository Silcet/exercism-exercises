use std::{cmp::Ordering, collections::BTreeMap};

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    hands
        .iter()
        .map(|hand| PokerHand::new(hand))
        .inspect(|x| println!("Poker hand: {:?}", x))
        .fold(Vec::<PokerHand>::new(), |mut winning_hands, hand| {
            println!("Got hand: {:?}", hand);
            if winning_hands.is_empty() {
                println!("List was empty.");
                winning_hands.push(hand);
            } else {
                match winning_hands.first().unwrap().partial_cmp(&hand) {
                    Some(Ordering::Greater) => {
                        println!("It's bigger");
                        winning_hands.clear();
                        winning_hands.push(hand);
                    }
                    Some(Ordering::Equal) => {
                        println!("It's the same");
                        winning_hands.push(hand);
                    }
                    Some(Ordering::Less) => {
                        println!("It's less");
                    }
                    None => panic!("Can't compare hands"),
                }
            }
            winning_hands
        })
        .iter()
        .inspect(|x| println!("Winning hand: {:?}", x))
        .map(|poker_hand| poker_hand.cards)
        .collect()
}

#[derive(Ord, Eq, PartialOrd, PartialEq, Debug)]
enum Hands {
    StraightFlush(u8),
    FourOfAKind(u8, u8),
    FullHouse(u8, u8),
    Flush(u8, u8),
    Straight(u8),
    ThreeOfAKind(u8),
    TwoPair(u8, u8),
    OnePair(u8, u8),
    HighCard(u8),
}

#[derive(PartialEq, Debug)]
struct PokerHand<'a> {
    pub cards: &'a str,
    hand: Hands,
}

impl<'a> PokerHand<'a> {
    pub fn new(cards: &'a str) -> Self {
        let cards_map = Self::cards_to_map(cards);
        println!("Cards: {}", cards);
        println!("Cards map: {:?}", cards_map);
        Self {
            cards,
            hand: Self::parse_cards(cards_map),
        }
    }

    fn cards_to_map(cards: &'a str) -> BTreeMap<u8, Vec<char>> {
        cards
            .split(' ')
            .fold(BTreeMap::<u8, Vec<char>>::new(), |mut map, card| {
                let chars = card.chars().rev().collect::<String>();
                let split_chars = chars.split_at(1);
                let number = Self::parse_number(split_chars.1);
                let suit = split_chars.0.chars().next().unwrap();

                let entry = map.get_mut(&number);
                if let Some(value) = entry {
                    value.push(suit);
                } else {
                    map.insert(number, [suit].into_iter().collect());
                }
                map
            })
    }

    fn parse_number(number: &str) -> u8 {
        match number {
            "A" => 1,
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            "6" => 6,
            "7" => 7,
            "8" => 8,
            "9" => 9,
            "10" => 10,
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            _ => 0,
        }
    }

    fn parse_cards(cards: BTreeMap<u8, Vec<char>>) -> Hands {
        if cards.len() == 2 {
            let keys: Vec<&u8> = cards.keys().collect();
            match (cards[keys[0]].len(), cards[keys[1]].len()) {
                (4, 1) => Hands::FourOfAKind(*keys[0], *keys[1]),
                (1, 4) => Hands::FourOfAKind(*keys[1], *keys[0]),
                _ => Hands::OnePair(2, 1),
            }
        } else {
            Hands::HighCard(1)
        }
    }
}

impl<'a> PartialOrd for PokerHand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.hand.cmp(&other.hand))
    }
}
