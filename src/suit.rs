use core::fmt::{Display, Formatter};
use core::ops::BitOr;
use core::str::FromStr;

use crate::card::Card;
use crate::rank::Rank;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

impl BitOr<Rank> for Suit {
    type Output = Card;

    fn bitor(self, rank: Rank) -> Self::Output {
        Card::new(rank, self)
    }
}

#[cfg(test)]
mod tests_bitor_with_rank {
    use super::*;

    #[test]
    fn it_creates_card_by_bitor_with_rank() {
        assert_eq!(Suit::Spade | Rank::Ace, Card::new(Rank::Ace, Suit::Spade));
        assert_eq!(
            Suit::Heart | Rank::Queen,
            Card::new(Rank::Queen, Suit::Heart)
        );
        assert_eq!(
            Suit::Diamond | Rank::Eight,
            Card::new(Rank::Eight, Suit::Diamond)
        );
        assert_eq!(Suit::Club | Rank::Deuce, Card::new(Rank::Deuce, Suit::Club));
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        match self {
            Suit::Spade => write!(f, "s"),
            Suit::Heart => write!(f, "h"),
            Suit::Diamond => write!(f, "d"),
            Suit::Club => write!(f, "c"),
        }
    }
}

#[cfg(test)]
mod tests_display {
    use super::*;

    #[test]
    fn it_formats() {
        assert_eq!(format!("{}", Suit::Spade), "s");
        assert_eq!(format!("{}", Suit::Heart), "h");
        assert_eq!(format!("{}", Suit::Diamond), "d");
        assert_eq!(format!("{}", Suit::Club), "c");
    }
}

impl FromStr for Suit {
    type Err = ParseSuitError;

    fn from_str(v: &str) -> Result<Self, Self::Err> {
        match v {
            "s" => Ok(Suit::Spade),
            "h" => Ok(Suit::Heart),
            "d" => Ok(Suit::Diamond),
            "c" => Ok(Suit::Club),
            _ => Err(ParseSuitError(v.to_string())),
        }
    }
}

#[cfg(test)]
mod tests_from_str {
    use super::*;

    #[test]
    fn it_can_be_created_from_str() {
        assert_eq!("s".parse::<Suit>().unwrap(), Suit::Spade);
        assert_eq!("h".parse::<Suit>().unwrap(), Suit::Heart);
        assert_eq!("d".parse::<Suit>().unwrap(), Suit::Diamond);
        assert_eq!("c".parse::<Suit>().unwrap(), Suit::Club);
        assert_eq!(
            "x".parse::<Suit>().unwrap_err().to_string(),
            "x is not a valid string for a suit.".to_string()
        );
    }
}

impl From<&Suit> for usize {
    fn from(suit: &Suit) -> Self {
        match suit {
            Suit::Spade => 0,
            Suit::Heart => 1,
            Suit::Diamond => 2,
            Suit::Club => 3,
        }
    }
}

impl From<Suit> for usize {
    fn from(suit: Suit) -> Self {
        usize::from(&suit)
    }
}

#[cfg(test)]
mod tests_usize_from_suit {
    use super::*;

    #[test]
    fn it_converts_to_usize() {
        assert_eq!(usize::from(&Suit::Spade), 0);
        assert_eq!(usize::from(&Suit::Heart), 1);
        assert_eq!(usize::from(&Suit::Diamond), 2);
        assert_eq!(usize::from(&Suit::Club), 3);
    }
}

#[derive(Debug)]
pub struct ParseSuitError(String);

impl Display for ParseSuitError {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        write!(f, "{} is not a valid string for a suit.", self.0)
    }
}
