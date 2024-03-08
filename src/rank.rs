use core::fmt::{Display, Formatter};
use core::ops::BitOr;
use core::str::FromStr;

use crate::card::Card;
use crate::suit::Suit;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Trey,
    Deuce,
}

impl Rank {
    pub fn prev(&self) -> Option<Self> {
        match self {
            Rank::Ace => None,
            Rank::King => Some(Rank::Ace),
            Rank::Queen => Some(Rank::King),
            Rank::Jack => Some(Rank::Queen),
            Rank::Ten => Some(Rank::Jack),
            Rank::Nine => Some(Rank::Ten),
            Rank::Eight => Some(Rank::Nine),
            Rank::Seven => Some(Rank::Eight),
            Rank::Six => Some(Rank::Seven),
            Rank::Five => Some(Rank::Six),
            Rank::Four => Some(Rank::Five),
            Rank::Trey => Some(Rank::Four),
            Rank::Deuce => Some(Rank::Trey),
        }
    }

    pub fn next(&self) -> Option<Self> {
        match self {
            Rank::Ace => Some(Rank::King),
            Rank::King => Some(Rank::Queen),
            Rank::Queen => Some(Rank::Jack),
            Rank::Jack => Some(Rank::Ten),
            Rank::Ten => Some(Rank::Nine),
            Rank::Nine => Some(Rank::Eight),
            Rank::Eight => Some(Rank::Seven),
            Rank::Seven => Some(Rank::Six),
            Rank::Six => Some(Rank::Five),
            Rank::Five => Some(Rank::Four),
            Rank::Four => Some(Rank::Trey),
            Rank::Trey => Some(Rank::Deuce),
            Rank::Deuce => None,
        }
    }
}

impl BitOr<Suit> for Rank {
    type Output = Card;

    fn bitor(self, suit: Suit) -> Self::Output {
        Card::new(self, suit)
    }
}

#[cfg(test)]
mod tests_bitor_with_suit {
    use super::*;

    #[test]
    fn it_creates_card_by_bitor_with_suit() {
        assert_eq!(Rank::Ace | Suit::Spade, Card::new(Rank::Ace, Suit::Spade));
        assert_eq!(
            Rank::Queen | Suit::Heart,
            Card::new(Rank::Queen, Suit::Heart)
        );
        assert_eq!(
            Rank::Eight | Suit::Diamond,
            Card::new(Rank::Eight, Suit::Diamond)
        );
        assert_eq!(Rank::Deuce | Suit::Club, Card::new(Rank::Deuce, Suit::Club));
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        match self {
            Rank::Ace => write!(f, "A"),
            Rank::King => write!(f, "K"),
            Rank::Queen => write!(f, "Q"),
            Rank::Jack => write!(f, "J"),
            Rank::Ten => write!(f, "T"),
            Rank::Nine => write!(f, "9"),
            Rank::Eight => write!(f, "8"),
            Rank::Seven => write!(f, "7"),
            Rank::Six => write!(f, "6"),
            Rank::Five => write!(f, "5"),
            Rank::Four => write!(f, "4"),
            Rank::Trey => write!(f, "3"),
            Rank::Deuce => write!(f, "2"),
        }
    }
}

#[cfg(test)]
mod tests_display {
    use super::*;

    #[test]
    fn it_formats() {
        assert_eq!(Rank::Ace.to_string(), "A");
        assert_eq!(Rank::King.to_string(), "K");
        assert_eq!(Rank::Queen.to_string(), "Q");
        assert_eq!(Rank::Jack.to_string(), "J");
        assert_eq!(Rank::Ten.to_string(), "T");
        assert_eq!(Rank::Nine.to_string(), "9");
        assert_eq!(Rank::Eight.to_string(), "8");
        assert_eq!(Rank::Seven.to_string(), "7");
        assert_eq!(Rank::Six.to_string(), "6");
        assert_eq!(Rank::Five.to_string(), "5");
        assert_eq!(Rank::Four.to_string(), "4");
        assert_eq!(Rank::Trey.to_string(), "3");
        assert_eq!(Rank::Deuce.to_string(), "2");
    }
}

impl From<&Rank> for usize {
    fn from(rank: &Rank) -> Self {
        match rank {
            Rank::Ace => 0,
            Rank::King => 1,
            Rank::Queen => 2,
            Rank::Jack => 3,
            Rank::Ten => 4,
            Rank::Nine => 5,
            Rank::Eight => 6,
            Rank::Seven => 7,
            Rank::Six => 8,
            Rank::Five => 9,
            Rank::Four => 10,
            Rank::Trey => 11,
            Rank::Deuce => 12,
        }
    }
}

impl From<Rank> for usize {
    fn from(rank: Rank) -> Self {
        usize::from(&rank)
    }
}

#[cfg(test)]
mod tests_usize_from_rank {
    use super::*;

    #[test]
    fn it_converts_to_usize() {
        assert_eq!(usize::from(Rank::Ace), 0);
        assert_eq!(usize::from(Rank::King), 1);
        assert_eq!(usize::from(Rank::Queen), 2);
        assert_eq!(usize::from(Rank::Jack), 3);
        assert_eq!(usize::from(Rank::Ten), 4);
        assert_eq!(usize::from(Rank::Nine), 5);
        assert_eq!(usize::from(Rank::Eight), 6);
        assert_eq!(usize::from(Rank::Seven), 7);
        assert_eq!(usize::from(Rank::Six), 8);
        assert_eq!(usize::from(Rank::Five), 9);
        assert_eq!(usize::from(Rank::Four), 10);
        assert_eq!(usize::from(Rank::Trey), 11);
        assert_eq!(usize::from(Rank::Deuce), 12);
    }
}

impl FromStr for Rank {
    type Err = ParseRankError;

    fn from_str(v: &str) -> Result<Self, Self::Err> {
        match v {
            "A" => Ok(Rank::Ace),
            "K" => Ok(Rank::King),
            "Q" => Ok(Rank::Queen),
            "J" => Ok(Rank::Jack),
            "T" => Ok(Rank::Ten),
            "9" => Ok(Rank::Nine),
            "8" => Ok(Rank::Eight),
            "7" => Ok(Rank::Seven),
            "6" => Ok(Rank::Six),
            "5" => Ok(Rank::Five),
            "4" => Ok(Rank::Four),
            "3" => Ok(Rank::Trey),
            "2" => Ok(Rank::Deuce),
            &_ => Err(ParseRankError(v.to_string())),
        }
    }
}

#[cfg(test)]
mod tests_from_str {
    use super::*;

    #[test]
    fn it_can_be_created_from_str() {
        assert_eq!("A".parse::<Rank>().unwrap(), Rank::Ace);
        assert_eq!("K".parse::<Rank>().unwrap(), Rank::King);
        assert_eq!("Q".parse::<Rank>().unwrap(), Rank::Queen);
        assert_eq!("J".parse::<Rank>().unwrap(), Rank::Jack);
        assert_eq!("T".parse::<Rank>().unwrap(), Rank::Ten);
        assert_eq!("9".parse::<Rank>().unwrap(), Rank::Nine);
        assert_eq!("8".parse::<Rank>().unwrap(), Rank::Eight);
        assert_eq!("7".parse::<Rank>().unwrap(), Rank::Seven);
        assert_eq!("6".parse::<Rank>().unwrap(), Rank::Six);
        assert_eq!("5".parse::<Rank>().unwrap(), Rank::Five);
        assert_eq!("4".parse::<Rank>().unwrap(), Rank::Four);
        assert_eq!("3".parse::<Rank>().unwrap(), Rank::Trey);
        assert_eq!("2".parse::<Rank>().unwrap(), Rank::Deuce);
        assert_eq!(
            "X".parse::<Rank>().unwrap_err().to_string(),
            "X is not a valid string for a rank.".to_string()
        );
    }
}

#[derive(Debug)]
pub struct ParseRankError(String);

impl Display for ParseRankError {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        write!(f, "{} is not a valid string for a rank.", self.0)
    }
}
