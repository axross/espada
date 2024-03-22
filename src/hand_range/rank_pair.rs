use crate::card::{Card, Rank, Suit};
use crate::hand_range::CardPair;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RankPair {
    Pocket(Rank),
    Suited(Rank, Rank),
    Ofsuit(Rank, Rank),
}

impl Display for RankPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RankPair::Pocket(rank) => write!(f, "{}{}", rank, rank),
            RankPair::Suited(high, kicker) => write!(f, "{}{}s", high, kicker),
            RankPair::Ofsuit(high, kicker) => write!(f, "{}{}o", high, kicker),
        }
    }
}

impl IntoIterator for RankPair {
    type Item = CardPair;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            RankPair::Pocket(rank) => vec![
                CardPair::new(Card::new(rank, Suit::Spade), Card::new(rank, Suit::Heart)),
                CardPair::new(Card::new(rank, Suit::Spade), Card::new(rank, Suit::Diamond)),
                CardPair::new(Card::new(rank, Suit::Spade), Card::new(rank, Suit::Club)),
                CardPair::new(Card::new(rank, Suit::Heart), Card::new(rank, Suit::Diamond)),
                CardPair::new(Card::new(rank, Suit::Heart), Card::new(rank, Suit::Club)),
                CardPair::new(Card::new(rank, Suit::Diamond), Card::new(rank, Suit::Club)),
            ]
            .into_iter(),
            RankPair::Suited(high, kicker) => vec![
                CardPair::new(Card::new(high, Suit::Spade), Card::new(kicker, Suit::Spade)),
                CardPair::new(Card::new(high, Suit::Heart), Card::new(kicker, Suit::Heart)),
                CardPair::new(
                    Card::new(high, Suit::Diamond),
                    Card::new(kicker, Suit::Diamond),
                ),
                CardPair::new(Card::new(high, Suit::Club), Card::new(kicker, Suit::Club)),
            ]
            .into_iter(),
            RankPair::Ofsuit(high, kicker) => vec![
                CardPair::new(Card::new(high, Suit::Spade), Card::new(kicker, Suit::Heart)),
                CardPair::new(
                    Card::new(high, Suit::Spade),
                    Card::new(kicker, Suit::Diamond),
                ),
                CardPair::new(Card::new(high, Suit::Spade), Card::new(kicker, Suit::Club)),
                CardPair::new(Card::new(high, Suit::Heart), Card::new(kicker, Suit::Spade)),
                CardPair::new(
                    Card::new(high, Suit::Heart),
                    Card::new(kicker, Suit::Diamond),
                ),
                CardPair::new(Card::new(high, Suit::Heart), Card::new(kicker, Suit::Club)),
                CardPair::new(
                    Card::new(high, Suit::Diamond),
                    Card::new(kicker, Suit::Spade),
                ),
                CardPair::new(
                    Card::new(high, Suit::Diamond),
                    Card::new(kicker, Suit::Heart),
                ),
                CardPair::new(
                    Card::new(high, Suit::Diamond),
                    Card::new(kicker, Suit::Club),
                ),
                CardPair::new(Card::new(high, Suit::Club), Card::new(kicker, Suit::Spade)),
                CardPair::new(Card::new(high, Suit::Club), Card::new(kicker, Suit::Heart)),
                CardPair::new(
                    Card::new(high, Suit::Club),
                    Card::new(kicker, Suit::Diamond),
                ),
            ]
            .into_iter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod into_iter {
        use super::*;

        #[test]
        fn it_turns_into_pocket_card_set_iter() {
            assert_eq!(
                RankPair::Pocket(Rank::Jack).into_iter().collect::<Vec<_>>(),
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
                RankPair::Suited(Rank::Queen, Rank::Ten)
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
                RankPair::Ofsuit(Rank::Nine, Rank::Five)
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
