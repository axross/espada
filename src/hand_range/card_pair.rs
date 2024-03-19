use crate::card::Card;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Index;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct CardPair(Card, Card);

impl CardPair {
    pub fn new(left: Card, right: Card) -> CardPair {
        if left > right {
            CardPair(right, left)
        } else {
            CardPair(left, right)
        }
    }
}

impl Index<usize> for CardPair {
    type Output = Card;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("index out of range."),
        }
    }
}

impl Display for CardPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl Debug for CardPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CardPair({}{})", self.0, self.1)
    }
}

impl FromStr for CardPair {
    type Err = ParseCardPairError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() != 4 {
            return Err(Self::Err::InvalidLength(value.len()));
        }

        match (Card::from_str(&value[0..2]), Card::from_str(&value[2..4])) {
            (Ok(l), Ok(r)) => Ok(CardPair::new(l, r)),
            (Err(_), _) => Err(Self::Err::InvalidCardStr((&value[0..2]).to_string())),
            (Ok(_), Err(_)) => Err(Self::Err::InvalidCardStr((&value[2..4]).to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tests_from_str {
        use super::*;
        use crate::card::{Rank, Suit};

        #[test]
        fn it_parses_str_into_card_set_askc() {
            assert_eq!(
                CardPair::from_str("AsKc").unwrap(),
                CardPair::new(
                    Card::new(Rank::Ace, Suit::Spade),
                    Card::new(Rank::King, Suit::Club),
                )
            );
        }

        #[test]
        fn it_returns_error_when_invalid_length() {
            assert_eq!(
                CardPair::from_str("As Kc").unwrap_err(),
                ParseCardPairError::InvalidLength(5),
            );
        }

        #[test]
        fn it_returns_error_when_invalid_string() {
            assert_eq!(
                CardPair::from_str("AsKj").unwrap_err(),
                ParseCardPairError::InvalidCardStr("Kj".to_string()),
            );
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseCardPairError {
    InvalidLength(usize),
    InvalidCardStr(String),
}
