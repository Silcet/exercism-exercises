/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    unimplemented!("Out of {:?}, which hand wins?", hands)
}

enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    pub fn new(suit: char) -> Result<Self, ()> {
        match suit {
            'H' => Ok(Suit::Hearts),
            'D' => Ok(Suit::Diamonds),
            'C' => Ok(Suit::Clubs),
            'S' => Ok(Suit::Spades),
            _ => Err(()),
        }
    }
}

struct Card {
    number: u8,
    suit: Suit,
}

impl Card {
    pub fn new(hand: &str) -> Result<Self, ()> {
        if hand.len() != 2 {
            return Err(());
        }

        let mut parts = hand.chars();
        Ok(Self {
            number: parts.next().unwrap().to_digit(10).ok_or(())? as u8,
            suit: Suit::new(parts.next().unwrap())?,
        })
    }
}
