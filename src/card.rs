use core::fmt::Display;
use core::fmt::Formatter;
use core::str::FromStr;

use crate::rank::Rank;
use crate::suit::Suit;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub struct Card(Rank, Suit);

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Card(rank, suit)
    }

    pub fn rank(&self) -> &Rank {
        &self.0
    }

    pub fn suit(&self) -> &Suit {
        &self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_turns_into_rank() {
        assert_eq!(Card::new(Rank::Ace, Suit::Spade).rank(), &Rank::Ace);
        assert_eq!(Card::new(Rank::King, Suit::Heart).rank(), &Rank::King);
        assert_eq!(Card::new(Rank::Queen, Suit::Diamond).rank(), &Rank::Queen);
        assert_eq!(Card::new(Rank::Jack, Suit::Club).rank(), &Rank::Jack);
        assert_eq!(Card::new(Rank::Ten, Suit::Spade).rank(), &Rank::Ten);
        assert_eq!(Card::new(Rank::Nine, Suit::Heart).rank(), &Rank::Nine);
        assert_eq!(Card::new(Rank::Eight, Suit::Diamond).rank(), &Rank::Eight);
        assert_eq!(Card::new(Rank::Seven, Suit::Club).rank(), &Rank::Seven);
        assert_eq!(Card::new(Rank::Six, Suit::Spade).rank(), &Rank::Six);
        assert_eq!(Card::new(Rank::Five, Suit::Heart).rank(), &Rank::Five);
        assert_eq!(Card::new(Rank::Four, Suit::Diamond).rank(), &Rank::Four);
        assert_eq!(Card::new(Rank::Trey, Suit::Club).rank(), &Rank::Trey);
        assert_eq!(Card::new(Rank::Deuce, Suit::Spade).rank(), &Rank::Deuce);
    }

    #[test]
    fn it_turns_into_suit() {
        assert_eq!(Card::new(Rank::Ace, Suit::Spade).suit(), &Suit::Spade);
        assert_eq!(Card::new(Rank::King, Suit::Heart).suit(), &Suit::Heart);
        assert_eq!(Card::new(Rank::Queen, Suit::Diamond).suit(), &Suit::Diamond);
        assert_eq!(Card::new(Rank::Jack, Suit::Club).suit(), &Suit::Club);
        assert_eq!(Card::new(Rank::Ten, Suit::Spade).suit(), &Suit::Spade);
        assert_eq!(Card::new(Rank::Nine, Suit::Heart).suit(), &Suit::Heart);
        assert_eq!(Card::new(Rank::Eight, Suit::Diamond).suit(), &Suit::Diamond);
        assert_eq!(Card::new(Rank::Seven, Suit::Club).suit(), &Suit::Club);
        assert_eq!(Card::new(Rank::Six, Suit::Spade).suit(), &Suit::Spade);
        assert_eq!(Card::new(Rank::Five, Suit::Heart).suit(), &Suit::Heart);
        assert_eq!(Card::new(Rank::Four, Suit::Diamond).suit(), &Suit::Diamond);
        assert_eq!(Card::new(Rank::Trey, Suit::Club).suit(), &Suit::Club);
        assert_eq!(Card::new(Rank::Deuce, Suit::Spade).suit(), &Suit::Spade);
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

#[cfg(test)]
mod tests_display {
    use super::*;

    #[test]
    fn it_turns_into_str() {
        assert_eq!(Card::new(Rank::Ace, Suit::Spade).to_string(), "As");
        assert_eq!(Card::new(Rank::King, Suit::Heart).to_string(), "Kh");
        assert_eq!(Card::new(Rank::Queen, Suit::Diamond).to_string(), "Qd");
        assert_eq!(Card::new(Rank::Jack, Suit::Club).to_string(), "Jc");
        assert_eq!(Card::new(Rank::Ten, Suit::Spade).to_string(), "Ts");
        assert_eq!(Card::new(Rank::Nine, Suit::Heart).to_string(), "9h");
        assert_eq!(Card::new(Rank::Eight, Suit::Diamond).to_string(), "8d");
        assert_eq!(Card::new(Rank::Seven, Suit::Club).to_string(), "7c");
        assert_eq!(Card::new(Rank::Six, Suit::Spade).to_string(), "6s");
        assert_eq!(Card::new(Rank::Five, Suit::Heart).to_string(), "5h");
        assert_eq!(Card::new(Rank::Four, Suit::Diamond).to_string(), "4d");
        assert_eq!(Card::new(Rank::Trey, Suit::Club).to_string(), "3c");
        assert_eq!(Card::new(Rank::Deuce, Suit::Spade).to_string(), "2s");
    }
}

const SPADE_MASK: u64 = 0b0001000100010001000100010001000100010001000100010001;
const HEART_MASK: u64 = 0b0010001000100010001000100010001000100010001000100010;
const DIAMOND_MASK: u64 = 0b0100010001000100010001000100010001000100010001000100;
const CLUB_MASK: u64 = 0b1000100010001000100010001000100010001000100010001000;

const ACE_MASK: u64 = 0b0000000000000000000000000000000000000000000000001111;
const KING_MASK: u64 = 0b0000000000000000000000000000000000000000000011110000;
const QUEEN_MASK: u64 = 0b0000000000000000000000000000000000000000111100000000;
const JACK_MASK: u64 = 0b0000000000000000000000000000000000001111000000000000;
const TEN_MASK: u64 = 0b0000000000000000000000000000000011110000000000000000;
const NINE_MASK: u64 = 0b0000000000000000000000000000111100000000000000000000;
const EIGHT_MASK: u64 = 0b0000000000000000000000001111000000000000000000000000;
const SEVEN_MASK: u64 = 0b0000000000000000000011110000000000000000000000000000;
const SIX_MASK: u64 = 0b0000000000000000111100000000000000000000000000000000;
const FIVE_MASK: u64 = 0b0000000000001111000000000000000000000000000000000000;
const FOUR_MASK: u64 = 0b0000000011110000000000000000000000000000000000000000;
const TREY_MASK: u64 = 0b0000111100000000000000000000000000000000000000000000;
const DEUCE_MASK: u64 = 0b1111000000000000000000000000000000000000000000000000;

impl From<&u64> for Card {
    fn from(value: &u64) -> Self {
        let suit = if value & SPADE_MASK >= 1 {
            Suit::Spade
        } else if value & HEART_MASK >= 1 {
            Suit::Heart
        } else if value & DIAMOND_MASK >= 1 {
            Suit::Diamond
        } else if value & CLUB_MASK >= 1 {
            Suit::Club
        } else {
            panic!();
        };

        let rank = if value & ACE_MASK >= 1 {
            Rank::Ace
        } else if value & KING_MASK >= 1 {
            Rank::King
        } else if value & QUEEN_MASK >= 1 {
            Rank::Queen
        } else if value & JACK_MASK >= 1 {
            Rank::Jack
        } else if value & TEN_MASK >= 1 {
            Rank::Ten
        } else if value & NINE_MASK >= 1 {
            Rank::Nine
        } else if value & EIGHT_MASK >= 1 {
            Rank::Eight
        } else if value & SEVEN_MASK >= 1 {
            Rank::Seven
        } else if value & SIX_MASK >= 1 {
            Rank::Six
        } else if value & FIVE_MASK >= 1 {
            Rank::Five
        } else if value & FOUR_MASK >= 1 {
            Rank::Four
        } else if value & TREY_MASK >= 1 {
            Rank::Trey
        } else if value & DEUCE_MASK >= 1 {
            Rank::Deuce
        } else {
            panic!();
        };

        Card::new(rank, suit)
    }
}

impl From<u64> for Card {
    fn from(value: u64) -> Self {
        Card::from(&value)
    }
}

#[cfg(test)]
mod tests_card_from_u64 {
    use super::*;

    #[test]
    fn it_turns_into_u64() {
        assert_eq!(
            Card::from(0b0000000000000000000000000000000000000000000000000001),
            Card::new(Rank::Ace, Suit::Spade)
        );
        assert_eq!(
            Card::from(0b0000000000000000000000000000000000000000000000100000),
            Card::new(Rank::King, Suit::Heart)
        );
        assert_eq!(
            Card::from(0b0000000000000000000000000000000000000000010000000000),
            Card::new(Rank::Queen, Suit::Diamond)
        );
        assert_eq!(
            Card::from(0b0000000000000000000000000000000000001000000000000000),
            Card::new(Rank::Jack, Suit::Club)
        );
        assert_eq!(
            Card::from(0b0000000000000000000000000000000000010000000000000000),
            Card::new(Rank::Ten, Suit::Spade)
        );
        assert_eq!(
            Card::from(0b0000000000000000000000000000001000000000000000000000),
            Card::new(Rank::Nine, Suit::Heart)
        );
        assert_eq!(
            Card::from(0b0000000000000000000000000100000000000000000000000000),
            Card::new(Rank::Eight, Suit::Diamond)
        );
        assert_eq!(
            Card::from(0b0000000000000000000010000000000000000000000000000000),
            Card::new(Rank::Seven, Suit::Club)
        );
        assert_eq!(
            Card::from(0b0000000000000000000100000000000000000000000000000000),
            Card::new(Rank::Six, Suit::Spade)
        );
        assert_eq!(
            Card::from(0b0000000000000010000000000000000000000000000000000000),
            Card::new(Rank::Five, Suit::Heart)
        );
        assert_eq!(
            Card::from(0b0000000001000000000000000000000000000000000000000000),
            Card::new(Rank::Four, Suit::Diamond)
        );
        assert_eq!(
            Card::from(0b0000100000000000000000000000000000000000000000000000),
            Card::new(Rank::Trey, Suit::Club)
        );
        assert_eq!(
            Card::from(0b0001000000000000000000000000000000000000000000000000),
            Card::new(Rank::Deuce, Suit::Spade)
        );
    }
}

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(v: &str) -> Result<Self, Self::Err> {
        if v.len() == 2 {
            if let (Ok(rank), Ok(suit)) = (Rank::from_str(&v[0..1]), Suit::from_str(&v[1..2])) {
                return Ok(Card(rank, suit));
            };
        };

        Err(ParseCardError(v.to_string()))
    }
}

#[cfg(test)]
mod tests_card_from_str {
    use super::*;

    #[test]
    fn it_turns_into_card() {
        assert_eq!(
            Card::from_str("As").unwrap(),
            Card::new(Rank::Ace, Suit::Spade)
        );
        assert_eq!(
            Card::from_str("Kh").unwrap(),
            Card::new(Rank::King, Suit::Heart)
        );
        assert_eq!(
            Card::from_str("Qd").unwrap(),
            Card::new(Rank::Queen, Suit::Diamond)
        );
        assert_eq!(
            Card::from_str("Jc").unwrap(),
            Card::new(Rank::Jack, Suit::Club)
        );
        assert_eq!(
            Card::from_str("Ts").unwrap(),
            Card::new(Rank::Ten, Suit::Spade)
        );
        assert_eq!(
            Card::from_str("9h").unwrap(),
            Card::new(Rank::Nine, Suit::Heart)
        );
        assert_eq!(
            Card::from_str("8d").unwrap(),
            Card::new(Rank::Eight, Suit::Diamond)
        );
        assert_eq!(
            Card::from_str("7c").unwrap(),
            Card::new(Rank::Seven, Suit::Club)
        );
        assert_eq!(
            Card::from_str("6s").unwrap(),
            Card::new(Rank::Six, Suit::Spade)
        );
        assert_eq!(
            Card::from_str("5h").unwrap(),
            Card::new(Rank::Five, Suit::Heart)
        );
        assert_eq!(
            Card::from_str("4d").unwrap(),
            Card::new(Rank::Four, Suit::Diamond)
        );
        assert_eq!(
            Card::from_str("3c").unwrap(),
            Card::new(Rank::Trey, Suit::Club)
        );
        assert_eq!(
            Card::from_str("2s").unwrap(),
            Card::new(Rank::Deuce, Suit::Spade)
        );
    }
}

#[derive(Debug)]
pub struct ParseCardError(String);

impl Display for ParseCardError {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        write!(f, "{} is not a valid string for a card.", self.0)
    }
}

impl From<&Card> for u64 {
    fn from(card: &Card) -> Self {
        let rank_bit = match card.0 {
            Rank::Ace => ACE_MASK,
            Rank::King => KING_MASK,
            Rank::Queen => QUEEN_MASK,
            Rank::Jack => JACK_MASK,
            Rank::Ten => TEN_MASK,
            Rank::Nine => NINE_MASK,
            Rank::Eight => EIGHT_MASK,
            Rank::Seven => SEVEN_MASK,
            Rank::Six => SIX_MASK,
            Rank::Five => FIVE_MASK,
            Rank::Four => FOUR_MASK,
            Rank::Trey => TREY_MASK,
            Rank::Deuce => DEUCE_MASK,
        };

        let suit_bit = match card.1 {
            Suit::Spade => SPADE_MASK,
            Suit::Heart => HEART_MASK,
            Suit::Diamond => DIAMOND_MASK,
            Suit::Club => CLUB_MASK,
        };

        rank_bit & suit_bit
    }
}

impl From<Card> for u64 {
    fn from(card: Card) -> Self {
        u64::from(&card)
    }
}

#[cfg(test)]
mod tests_u64_from_card {
    use super::*;

    #[test]
    fn it_turns_into_u64() {
        assert_eq!(
            u64::from(Card::new(Rank::Ace, Suit::Spade)),
            0b0000000000000000000000000000000000000000000000000001
        );
        assert_eq!(
            u64::from(Card::new(Rank::King, Suit::Heart)),
            0b0000000000000000000000000000000000000000000000100000
        );
        assert_eq!(
            u64::from(Card::new(Rank::Queen, Suit::Diamond)),
            0b0000000000000000000000000000000000000000010000000000
        );
        assert_eq!(
            u64::from(Card::new(Rank::Jack, Suit::Club)),
            0b0000000000000000000000000000000000001000000000000000
        );
        assert_eq!(
            u64::from(Card::new(Rank::Ten, Suit::Spade)),
            0b0000000000000000000000000000000000010000000000000000
        );
        assert_eq!(
            u64::from(Card::new(Rank::Nine, Suit::Heart)),
            0b0000000000000000000000000000001000000000000000000000
        );
        assert_eq!(
            u64::from(Card::new(Rank::Eight, Suit::Diamond)),
            0b0000000000000000000000000100000000000000000000000000
        );
        assert_eq!(
            u64::from(Card::new(Rank::Seven, Suit::Club)),
            0b0000000000000000000010000000000000000000000000000000
        );
        assert_eq!(
            u64::from(Card::new(Rank::Six, Suit::Spade)),
            0b0000000000000000000100000000000000000000000000000000
        );
        assert_eq!(
            u64::from(Card::new(Rank::Five, Suit::Heart)),
            0b0000000000000010000000000000000000000000000000000000
        );
        assert_eq!(
            u64::from(Card::new(Rank::Four, Suit::Diamond)),
            0b0000000001000000000000000000000000000000000000000000
        );
        assert_eq!(
            u64::from(Card::new(Rank::Trey, Suit::Club)),
            0b0000100000000000000000000000000000000000000000000000
        );
        assert_eq!(
            u64::from(Card::new(Rank::Deuce, Suit::Spade)),
            0b0001000000000000000000000000000000000000000000000000
        );
    }
}
