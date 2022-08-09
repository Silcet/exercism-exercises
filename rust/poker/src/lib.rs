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
                    }
                    Some(Ordering::Equal) => {
                        println!("It's the same");
                        winning_hands.push(hand);
                    }
                    Some(Ordering::Less) => {
                        println!("It's less");
                        winning_hands.clear();
                        winning_hands.push(hand);
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

#[derive(Debug)]
enum Hands {
    StraightFlush(u32),
    FourOfAKind(u32, u32),
    FullHouse(u32, u32),
    Flush(u32, u32),
    Straight(u32),
    ThreeOfAKind(u32, u32),
    TwoPair(u32, u32, u32),
    OnePair(u32, u32),
    HighCard(u32),
}

impl Hands {
    pub fn value(&self) -> u32 {
        match self {
            Self::HighCard(high) => *high,
            Self::OnePair(pair, high) => 10000 + pair * 100 + high,
            Self::TwoPair(pair1, pair2, high) => 20000 + pair1 * 100 + pair2,
            Self::ThreeOfAKind(three, high) => 30000 + three * 100 + high,
            Self::Straight(straight) => 40000 + straight * 100,
            Self::Flush(high1, high2) => 50000 + high1 * 100 + high2,
            Self::FullHouse(three, two) => 60000 + three * 100 + two,
            Self::FourOfAKind(four, high) => 70000 + four * 100 + high,
            Self::StraightFlush(straight) => 80000 + straight * 100,
        }
    }
}

#[derive(PartialEq, Debug)]
struct PokerHand<'a> {
    pub cards: &'a str,
    value: u32,
}

impl<'a> PokerHand<'a> {
    pub fn new(cards: &'a str) -> Self {
        let cards_map = Self::cards_to_map(cards);
        println!("Cards: {}", cards);
        println!("Cards map: {:?}", cards_map);
        Self {
            cards,
            value: Self::parse_cards(cards_map),
        }
    }

    fn cards_to_map(cards: &'a str) -> BTreeMap<u32, Vec<char>> {
        cards
            .split(' ')
            .fold(BTreeMap::<u32, Vec<char>>::new(), |mut map, card| {
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

    fn parse_number(number: &str) -> u32 {
        match number {
            "A" => 14,
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

    fn parse_cards(cards: BTreeMap<u32, Vec<char>>) -> u32 {
        let keys: Vec<&u32> = cards.keys().collect();
        (match cards.len() {
            2 => match (cards[keys[0]].len(), cards[keys[1]].len()) {
                (4, 1) => Hands::FourOfAKind(*keys[0], *keys[1]),
                (1, 4) => Hands::FourOfAKind(*keys[1], *keys[0]),
                (3, 2) => Hands::FullHouse(*keys[0], *keys[1]),
                (2, 3) => Hands::FullHouse(*keys[1], *keys[0]),
                _ => panic!("Impossible card combination!"),
            },
            3 => match (
                cards[keys[0]].len(),
                cards[keys[1]].len(),
                cards[keys[2]].len(),
            ) {
                (2, 2, 1) => Hands::TwoPair(*keys[0], *keys[1], *keys[2]),
                (2, 1, 2) => Hands::TwoPair(*keys[0], *keys[2], *keys[1]),
                (1, 2, 2) => Hands::TwoPair(*keys[1], *keys[2], *keys[0]),
                _ => panic!("Impossible card combination!"),
            },
            _ => Hands::HighCard(1),
        })
        .value()
    }
}

impl<'a> PartialOrd for PokerHand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
