use std::fmt::Display;

use crate::card::{Card, Rank, Suit};
use crate::hand_range::CardPair;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct RankPair {
    high: Rank,
    kicker: Rank,
    is_suited: bool,
}

impl RankPair {
    pub fn pocket(rank: Rank) -> Self {
        RankPair {
            high: rank,
            kicker: rank,
            is_suited: false,
        }
    }

    pub fn suited(high: Rank, kicker: Rank) -> Self {
        debug_assert!(
            high < kicker,
            "high ({:?}) needs to be higher rank than kicker ({:?}).",
            high,
            kicker
        );

        RankPair {
            high: high,
            kicker: kicker,
            is_suited: true,
        }
    }

    pub fn ofsuit(high: Rank, kicker: Rank) -> Self {
        debug_assert!(
            high < kicker,
            "high ({:?}) needs to be higher rank than kicker ({:?}).",
            high,
            kicker
        );

        RankPair {
            high: high,
            kicker: kicker,
            is_suited: false,
        }
    }

    pub fn high(&self) -> Rank {
        self.high
    }

    pub fn kicker(&self) -> Rank {
        self.kicker
    }

    pub fn is_suited(&self) -> bool {
        self.is_suited
    }

    pub fn is_pocket(&self) -> bool {
        self.high == self.kicker
    }
}

impl Display for RankPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_pocket() {
            write!(f, "{}{}", self.high, self.kicker)
        } else if self.is_suited() {
            write!(f, "{}{}s", self.high, self.kicker)
        } else {
            write!(f, "{}{}o", self.high, self.kicker)
        }
    }
}

impl IntoIterator for RankPair {
    type Item = CardPair;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match (self.high == self.kicker, self.is_suited) {
            (true, false) => vec![
                CardPair::new(
                    Card::new(self.high, Suit::Spade),
                    Card::new(self.high, Suit::Heart),
                ),
                CardPair::new(
                    Card::new(self.high, Suit::Spade),
                    Card::new(self.high, Suit::Diamond),
                ),
                CardPair::new(
                    Card::new(self.high, Suit::Spade),
                    Card::new(self.high, Suit::Club),
                ),
                CardPair::new(
                    Card::new(self.high, Suit::Heart),
                    Card::new(self.high, Suit::Diamond),
                ),
                CardPair::new(
                    Card::new(self.high, Suit::Heart),
                    Card::new(self.high, Suit::Club),
                ),
                CardPair::new(
                    Card::new(self.high, Suit::Diamond),
                    Card::new(self.high, Suit::Club),
                ),
            ]
            .into_iter(),
            (false, true) => vec![
                CardPair::new(
                    Card::new(self.high, Suit::Spade),
                    Card::new(self.kicker, Suit::Spade),
                ),
                CardPair::new(
                    Card::new(self.high, Suit::Heart),
                    Card::new(self.kicker, Suit::Heart),
                ),
                CardPair::new(
                    Card::new(self.high, Suit::Diamond),
                    Card::new(self.kicker, Suit::Diamond),
                ),
                CardPair::new(
                    Card::new(self.high, Suit::Club),
                    Card::new(self.kicker, Suit::Club),
                ),
            ]
            .into_iter(),
            (false, false) => vec![
                CardPair::new(
                    Card::new(self.high, Suit::Spade),
                    Card::new(self.kicker, Suit::Heart),
                ),
                CardPair::new(
                    Card::new(self.high, Suit::Spade),
                    Card::new(self.kicker, Suit::Diamond),
                ),
                CardPair::new(
                    Card::new(self.high, Suit::Spade),
                    Card::new(self.kicker, Suit::Club),
                ),
                CardPair::new(
                    Card::new(self.high, Suit::Heart),
                    Card::new(self.kicker, Suit::Spade),
                ),
                CardPair::new(
                    Card::new(self.high, Suit::Heart),
                    Card::new(self.kicker, Suit::Diamond),
                ),
                CardPair::new(
                    Card::new(self.high, Suit::Heart),
                    Card::new(self.kicker, Suit::Club),
                ),
                CardPair::new(
                    Card::new(self.high, Suit::Diamond),
                    Card::new(self.kicker, Suit::Spade),
                ),
                CardPair::new(
                    Card::new(self.high, Suit::Diamond),
                    Card::new(self.kicker, Suit::Heart),
                ),
                CardPair::new(
                    Card::new(self.high, Suit::Diamond),
                    Card::new(self.kicker, Suit::Club),
                ),
                CardPair::new(
                    Card::new(self.high, Suit::Club),
                    Card::new(self.kicker, Suit::Spade),
                ),
                CardPair::new(
                    Card::new(self.high, Suit::Club),
                    Card::new(self.kicker, Suit::Heart),
                ),
                CardPair::new(
                    Card::new(self.high, Suit::Club),
                    Card::new(self.kicker, Suit::Diamond),
                ),
            ]
            .into_iter(),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod suited {
        use super::*;

        #[test]
        #[should_panic]
        fn it_panics_when_suited_king_ace() {
            RankPair::suited(Rank::King, Rank::Ace);
        }

        #[test]
        #[should_panic]
        fn it_panics_when_suited_deuce_ace() {
            RankPair::suited(Rank::Deuce, Rank::Ace);
        }

        #[test]
        #[should_panic]
        fn it_panics_when_suited_deuce_trey() {
            RankPair::suited(Rank::Deuce, Rank::Trey);
        }
    }

    mod ofsuit {
        use super::*;

        #[test]
        #[should_panic]
        fn it_panics_when_ofsuit_king_ace() {
            RankPair::ofsuit(Rank::King, Rank::Ace);
        }

        #[test]
        #[should_panic]
        fn it_panics_when_ofsuit_deuce_ace() {
            RankPair::ofsuit(Rank::Deuce, Rank::Ace);
        }

        #[test]
        #[should_panic]
        fn it_panics_when_ofsuit_deuce_trey() {
            RankPair::ofsuit(Rank::Deuce, Rank::Trey);
        }
    }

    mod into_iter {
        use super::*;

        #[test]
        fn it_turns_into_pocket_card_set_iter() {
            assert_eq!(
                RankPair::pocket(Rank::Jack).into_iter().collect::<Vec<_>>(),
                vec![
                    CardPair::new(
                        Card::new(Rank::Jack, Suit::Spade),
                        Card::new(Rank::Jack, Suit::Heart),
                    ),
                    CardPair::new(
                        Card::new(Rank::Jack, Suit::Spade),
                        Card::new(Rank::Jack, Suit::Diamond),
                    ),
                    CardPair::new(
                        Card::new(Rank::Jack, Suit::Spade),
                        Card::new(Rank::Jack, Suit::Club),
                    ),
                    CardPair::new(
                        Card::new(Rank::Jack, Suit::Heart),
                        Card::new(Rank::Jack, Suit::Diamond),
                    ),
                    CardPair::new(
                        Card::new(Rank::Jack, Suit::Heart),
                        Card::new(Rank::Jack, Suit::Club),
                    ),
                    CardPair::new(
                        Card::new(Rank::Jack, Suit::Diamond),
                        Card::new(Rank::Jack, Suit::Club),
                    ),
                ],
            );
        }

        #[test]
        fn it_turns_into_suited_card_set_iter() {
            assert_eq!(
                RankPair::suited(Rank::Queen, Rank::Ten)
                    .into_iter()
                    .collect::<Vec<_>>(),
                vec![
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Spade),
                        Card::new(Rank::Ten, Suit::Spade)
                    ),
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Heart),
                        Card::new(Rank::Ten, Suit::Heart)
                    ),
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Diamond),
                        Card::new(Rank::Ten, Suit::Diamond)
                    ),
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Club),
                        Card::new(Rank::Ten, Suit::Club)
                    ),
                ],
            );
        }

        #[test]
        fn it_turns_into_ofsuit_card_set_iter() {
            assert_eq!(
                RankPair::ofsuit(Rank::Nine, Rank::Five)
                    .into_iter()
                    .collect::<Vec<_>>(),
                vec![
                    CardPair::new(
                        Card::new(Rank::Nine, Suit::Spade),
                        Card::new(Rank::Five, Suit::Heart)
                    ),
                    CardPair::new(
                        Card::new(Rank::Nine, Suit::Spade),
                        Card::new(Rank::Five, Suit::Diamond)
                    ),
                    CardPair::new(
                        Card::new(Rank::Nine, Suit::Spade),
                        Card::new(Rank::Five, Suit::Club)
                    ),
                    CardPair::new(
                        Card::new(Rank::Nine, Suit::Heart),
                        Card::new(Rank::Five, Suit::Spade)
                    ),
                    CardPair::new(
                        Card::new(Rank::Nine, Suit::Heart),
                        Card::new(Rank::Five, Suit::Diamond)
                    ),
                    CardPair::new(
                        Card::new(Rank::Nine, Suit::Heart),
                        Card::new(Rank::Five, Suit::Club)
                    ),
                    CardPair::new(
                        Card::new(Rank::Nine, Suit::Diamond),
                        Card::new(Rank::Five, Suit::Spade)
                    ),
                    CardPair::new(
                        Card::new(Rank::Nine, Suit::Diamond),
                        Card::new(Rank::Five, Suit::Heart)
                    ),
                    CardPair::new(
                        Card::new(Rank::Nine, Suit::Diamond),
                        Card::new(Rank::Five, Suit::Club)
                    ),
                    CardPair::new(
                        Card::new(Rank::Nine, Suit::Club),
                        Card::new(Rank::Five, Suit::Spade)
                    ),
                    CardPair::new(
                        Card::new(Rank::Nine, Suit::Club),
                        Card::new(Rank::Five, Suit::Heart)
                    ),
                    CardPair::new(
                        Card::new(Rank::Nine, Suit::Club),
                        Card::new(Rank::Five, Suit::Diamond)
                    ),
                ],
            );
        }
    }
}
