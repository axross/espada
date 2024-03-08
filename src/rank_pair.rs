use crate::card::Card;
use crate::card_set::CardSet;
use crate::rank::Rank;
use crate::suit::Suit;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct RankPair {
    high: Rank,
    kicker: Rank,
    suited: bool,
}

impl RankPair {
    pub fn pocket(rank: Rank) -> Self {
        RankPair {
            high: rank,
            kicker: rank,
            suited: false,
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
            suited: true,
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
            suited: false,
        }
    }
}

#[cfg(test)]
mod tests_suited {
    use super::*;

    #[test]
    #[should_panic]
    fn it_panics_when_suited_kinf_ace() {
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

#[cfg(test)]
mod tests_ofsuit {
    use super::*;

    #[test]
    #[should_panic]
    fn it_panics_when_ofsuit_kinf_ace() {
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

impl IntoIterator for RankPair {
    type Item = CardSet;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match (self.high == self.kicker, self.suited) {
            (true, false) => vec![
                CardSet::from_iter([
                    Card::new(self.high, Suit::Spade),
                    Card::new(self.high, Suit::Heart),
                ]),
                CardSet::from_iter([
                    Card::new(self.high, Suit::Spade),
                    Card::new(self.high, Suit::Diamond),
                ]),
                CardSet::from_iter([
                    Card::new(self.high, Suit::Spade),
                    Card::new(self.high, Suit::Club),
                ]),
                CardSet::from_iter([
                    Card::new(self.high, Suit::Heart),
                    Card::new(self.high, Suit::Diamond),
                ]),
                CardSet::from_iter([
                    Card::new(self.high, Suit::Heart),
                    Card::new(self.high, Suit::Club),
                ]),
                CardSet::from_iter([
                    Card::new(self.high, Suit::Diamond),
                    Card::new(self.high, Suit::Club),
                ]),
            ]
            .into_iter(),
            (false, true) => vec![
                CardSet::from_iter([
                    Card::new(self.high, Suit::Spade),
                    Card::new(self.kicker, Suit::Spade),
                ]),
                CardSet::from_iter([
                    Card::new(self.high, Suit::Heart),
                    Card::new(self.kicker, Suit::Heart),
                ]),
                CardSet::from_iter([
                    Card::new(self.high, Suit::Diamond),
                    Card::new(self.kicker, Suit::Diamond),
                ]),
                CardSet::from_iter([
                    Card::new(self.high, Suit::Club),
                    Card::new(self.kicker, Suit::Club),
                ]),
            ]
            .into_iter(),
            (false, false) => vec![
                CardSet::from_iter([
                    Card::new(self.high, Suit::Spade),
                    Card::new(self.kicker, Suit::Heart),
                ]),
                CardSet::from_iter([
                    Card::new(self.high, Suit::Spade),
                    Card::new(self.kicker, Suit::Diamond),
                ]),
                CardSet::from_iter([
                    Card::new(self.high, Suit::Spade),
                    Card::new(self.kicker, Suit::Club),
                ]),
                CardSet::from_iter([
                    Card::new(self.high, Suit::Heart),
                    Card::new(self.kicker, Suit::Spade),
                ]),
                CardSet::from_iter([
                    Card::new(self.high, Suit::Heart),
                    Card::new(self.kicker, Suit::Diamond),
                ]),
                CardSet::from_iter([
                    Card::new(self.high, Suit::Heart),
                    Card::new(self.kicker, Suit::Club),
                ]),
                CardSet::from_iter([
                    Card::new(self.high, Suit::Diamond),
                    Card::new(self.kicker, Suit::Spade),
                ]),
                CardSet::from_iter([
                    Card::new(self.high, Suit::Diamond),
                    Card::new(self.kicker, Suit::Heart),
                ]),
                CardSet::from_iter([
                    Card::new(self.high, Suit::Diamond),
                    Card::new(self.kicker, Suit::Club),
                ]),
                CardSet::from_iter([
                    Card::new(self.high, Suit::Club),
                    Card::new(self.kicker, Suit::Spade),
                ]),
                CardSet::from_iter([
                    Card::new(self.high, Suit::Club),
                    Card::new(self.kicker, Suit::Heart),
                ]),
                CardSet::from_iter([
                    Card::new(self.high, Suit::Club),
                    Card::new(self.kicker, Suit::Diamond),
                ]),
            ]
            .into_iter(),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests_into_iter {
    use super::*;

    #[test]
    fn it_turns_into_pocket_card_set_iter() {
        assert_eq!(
            RankPair::pocket(Rank::Jack).into_iter().collect::<Vec<_>>(),
            vec![
                CardSet::from_iter([
                    Card::new(Rank::Jack, Suit::Spade),
                    Card::new(Rank::Jack, Suit::Heart),
                ]),
                CardSet::from_iter([
                    Card::new(Rank::Jack, Suit::Spade),
                    Card::new(Rank::Jack, Suit::Diamond),
                ]),
                CardSet::from_iter([
                    Card::new(Rank::Jack, Suit::Spade),
                    Card::new(Rank::Jack, Suit::Club),
                ]),
                CardSet::from_iter([
                    Card::new(Rank::Jack, Suit::Heart),
                    Card::new(Rank::Jack, Suit::Diamond),
                ]),
                CardSet::from_iter([
                    Card::new(Rank::Jack, Suit::Heart),
                    Card::new(Rank::Jack, Suit::Club),
                ]),
                CardSet::from_iter([
                    Card::new(Rank::Jack, Suit::Diamond),
                    Card::new(Rank::Jack, Suit::Club),
                ]),
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
                CardSet::from_iter([
                    Card::new(Rank::Queen, Suit::Spade),
                    Card::new(Rank::Ten, Suit::Spade)
                ]),
                CardSet::from_iter([
                    Card::new(Rank::Queen, Suit::Heart),
                    Card::new(Rank::Ten, Suit::Heart)
                ]),
                CardSet::from_iter([
                    Card::new(Rank::Queen, Suit::Diamond),
                    Card::new(Rank::Ten, Suit::Diamond)
                ]),
                CardSet::from_iter([
                    Card::new(Rank::Queen, Suit::Club),
                    Card::new(Rank::Ten, Suit::Club)
                ]),
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
                CardSet::from_iter([
                    Card::new(Rank::Nine, Suit::Spade),
                    Card::new(Rank::Five, Suit::Heart)
                ]),
                CardSet::from_iter([
                    Card::new(Rank::Nine, Suit::Spade),
                    Card::new(Rank::Five, Suit::Diamond)
                ]),
                CardSet::from_iter([
                    Card::new(Rank::Nine, Suit::Spade),
                    Card::new(Rank::Five, Suit::Club)
                ]),
                CardSet::from_iter([
                    Card::new(Rank::Nine, Suit::Heart),
                    Card::new(Rank::Five, Suit::Spade)
                ]),
                CardSet::from_iter([
                    Card::new(Rank::Nine, Suit::Heart),
                    Card::new(Rank::Five, Suit::Diamond)
                ]),
                CardSet::from_iter([
                    Card::new(Rank::Nine, Suit::Heart),
                    Card::new(Rank::Five, Suit::Club)
                ]),
                CardSet::from_iter([
                    Card::new(Rank::Nine, Suit::Diamond),
                    Card::new(Rank::Five, Suit::Spade)
                ]),
                CardSet::from_iter([
                    Card::new(Rank::Nine, Suit::Diamond),
                    Card::new(Rank::Five, Suit::Heart)
                ]),
                CardSet::from_iter([
                    Card::new(Rank::Nine, Suit::Diamond),
                    Card::new(Rank::Five, Suit::Club)
                ]),
                CardSet::from_iter([
                    Card::new(Rank::Nine, Suit::Club),
                    Card::new(Rank::Five, Suit::Spade)
                ]),
                CardSet::from_iter([
                    Card::new(Rank::Nine, Suit::Club),
                    Card::new(Rank::Five, Suit::Heart)
                ]),
                CardSet::from_iter([
                    Card::new(Rank::Nine, Suit::Club),
                    Card::new(Rank::Five, Suit::Diamond)
                ]),
            ],
        );
    }
}
