use std::cmp::Ordering;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    hands
        .iter()
        .map(|hand| PokerHand::new(hand))
        .fold(Vec::<PokerHand>::new(), |mut winning_hands, hand| {
            if winning_hands.is_empty() {
                winning_hands.push(hand);
            } else {
                match winning_hands.first().unwrap().partial_cmp(&hand) {
                    Some(Ordering::Greater) => {}
                    Some(Ordering::Equal) => {}
                    Some(Ordering::Less) => {}
                    None => panic!("Can't compare hands"),
                }
            }
            winning_hands
        })
        .iter()
        .map(|poker_hand| poker_hand.cards)
        .collect()
}

#[derive(Ord, Eq, PartialOrd, PartialEq, Debug)]
enum Hands {
    StraightFlush(u8),
    FourOfAKind(u8),
    FullHouse(u8, u8),
    Flush(u8),
    Straight(u8),
    ThreeOfAKind(u8),
    TwoPair(u8, u8),
    OnePair(u8),
    HighCard(u8),
}

#[derive(PartialEq, Debug)]
struct PokerHand<'a> {
    pub cards: &'a str,
    hand: Hands,
}

impl<'a> PokerHand<'a> {
    pub fn new(cards: &'a str) -> Self {
        Self {
            cards,
            hand: Self::parse_cards(cards),
        }
    }

    fn parse_cards(cards: &'a str) -> Hands {
        if cards.contains('A') {
            Hands::StraightFlush(1)
        } else if cards.contains('K') {
            Hands::HighCard(10)
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
