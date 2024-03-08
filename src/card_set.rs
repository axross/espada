use core::fmt::{Debug, Display, Formatter};
use core::str::FromStr;

use crate::card::Card;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct CardSet {
    bit: u64,
    len: u8,
}

impl CardSet {
    fn new(bit: u64, len: u8) -> CardSet {
        CardSet { bit, len }
    }

    pub fn empty() -> CardSet {
        CardSet::new(0, 0)
    }

    pub fn full() -> CardSet {
        CardSet::new(0b1111111111111111111111111111111111111111111111111111, 52)
    }

    pub fn iter(&self) -> CardSetIterator {
        CardSetIterator::new(self.bit)
    }

    pub fn len(&self) -> u8 {
        self.len
    }

    pub fn include_all(&self, other: &CardSet) -> bool {
        let other_bit: u64 = (*other).into();

        (self.bit & other_bit) == other_bit
    }

    pub fn include_any(&self, other: &CardSet) -> bool {
        let other_bit: u64 = (*other).into();
        let intersection = self.bit & other_bit;

        intersection > 0
    }

    pub fn include(&self, card: &Card) -> bool {
        let card_bit: u64 = card.into();

        (self.bit & card_bit) == card_bit
    }

    pub fn insert_all(&mut self, other: &CardSet) -> bool {
        let other_bit: u64 = (*other).into();
        let diff = !self.bit & other_bit;

        self.bit = self.bit | other_bit;
        self.len += get_bit_len(&diff);

        diff >= 1
    }

    pub fn insert(&mut self, card: &Card) -> bool {
        let card_bit: u64 = card.into();
        let diff = !self.bit & card_bit;

        self.bit = self.bit | card_bit;

        if diff == card_bit {
            self.len += 1;
        }

        diff == card_bit
    }

    pub fn remove_all(&mut self, other: &CardSet) -> bool {
        let other_bit: u64 = (*other).into();
        let intersection = self.bit & other_bit;

        self.bit = self.bit & !other_bit;
        self.len -= get_bit_len(&intersection);

        intersection >= 1
    }

    pub fn remove(&mut self, card: &Card) -> bool {
        let card_bit: u64 = card.into();
        let intersection = self.bit & card_bit;

        self.bit = self.bit & !card_bit;

        if intersection == card_bit {
            self.len -= 1;
        }

        intersection == card_bit
    }
}

#[cfg(test)]
mod tests_include_all {
    use crate::rank::Rank;
    use crate::suit::Suit;

    use super::*;

    #[test]
    fn card_set_askcqhjd2c_is_superset_of_card_set_as() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
            .include_all(&CardSet::from_iter([Card::new(Rank::Ace, Suit::Spade)])),
            true
        );
    }

    #[test]
    fn card_set_askcqhjd2c_is_superset_of_card_set_kc() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
            .include_all(&CardSet::from_iter([Card::new(Rank::King, Suit::Club)])),
            true
        );
    }

    #[test]
    fn card_set_askcqhjd2c_is_superset_of_card_set_qh() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
            .include_all(&CardSet::from_iter([Card::new(Rank::Queen, Suit::Heart)])),
            true
        );
    }

    #[test]
    fn card_set_askcqhjd2c_is_superset_of_card_set_jd() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
            .include_all(&CardSet::from_iter([Card::new(Rank::Deuce, Suit::Club)])),
            true
        );
    }

    #[test]
    fn card_set_askcqhjd2c_is_superset_of_card_set_dc() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
            .include_all(&CardSet::from_iter([Card::new(Rank::Deuce, Suit::Club)])),
            true
        );
    }

    #[test]
    fn card_set_askcqhjd2c_is_superset_of_card_set_askcqhjd2c() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
            .include_all(&CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
            ])),
            true
        );
    }

    #[test]
    fn card_set_askcqhjd2c_is_superset_of_card_set_qh2c() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
            .include_all(&CardSet::from_iter([
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Deuce, Suit::Club),
            ])),
            true
        );
    }

    #[test]
    fn full_card_set_is_superset_of_card_set_askcqhjd2c() {
        assert_eq!(
            CardSet::full().include_all(&CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])),
            true
        );
    }

    #[test]
    fn card_set_askcqhjd2c_is_superset_of_empty_card_set() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
            .include_all(&CardSet::empty()),
            true
        );
    }

    #[test]
    fn empty_card_set_is_superset_of_empty_card_set() {
        assert_eq!(CardSet::empty().include_all(&CardSet::empty()), true);
    }

    #[test]
    fn card_set_askcqhjd2c_is_not_superset_of_card_set_8s() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
            .include_all(&CardSet::from_iter([Card::new(Rank::Eight, Suit::Spade),])),
            false
        );
    }

    #[test]
    fn card_set_askcqhjd_is_not_superset_of_card_set_askcqhjd2c() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
            ])
            .include_all(&CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])),
            false
        );
    }
}

#[cfg(test)]
mod tests_include_any {
    use crate::rank::Rank;
    use crate::suit::Suit;

    use super::*;

    #[test]
    fn card_set_askcqhjd2c_intersects_card_set_as() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
            .include_any(&CardSet::from_iter([Card::new(Rank::Ace, Suit::Spade)])),
            true
        );
    }

    #[test]
    fn card_set_askcqhjd2c_intersects_card_set_kc() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
            .include_any(&CardSet::from_iter([Card::new(Rank::King, Suit::Club)])),
            true
        );
    }

    #[test]
    fn card_set_askcqhjd2c_intersects_card_set_qh() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
            .include_any(&CardSet::from_iter([Card::new(Rank::Queen, Suit::Heart)])),
            true
        );
    }

    #[test]
    fn card_set_askcqhjd2c_intersects_card_set_jd() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
            .include_any(&CardSet::from_iter([Card::new(Rank::Deuce, Suit::Club)])),
            true
        );
    }

    #[test]
    fn card_set_askcqhjd2c_intersects_card_set_dc() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
            .include_any(&CardSet::from_iter([Card::new(Rank::Deuce, Suit::Club)])),
            true
        );
    }

    #[test]
    fn card_set_askcqhjd2c_intersects_card_set_askcqhjd2c() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
            .include_any(&CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
            ])),
            true
        );
    }

    #[test]
    fn full_card_set_intersects_card_set_askcqhjd2c() {
        assert_eq!(
            CardSet::full().include_any(&CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])),
            true
        );
    }

    #[test]
    fn card_set_askcqhjd_intersect_superset_of_card_set_askcqhjd2c() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
            ])
            .include_any(&CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])),
            true
        );
    }

    #[test]
    fn card_set_askcqhjd_intersect_superset_of_card_set_ahkdqhjc() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
            ])
            .include_any(&CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Heart),
                Card::new(Rank::King, Suit::Diamond),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Club),
            ])),
            true
        );
    }

    #[test]
    fn card_set_askcqhjd2c_does_not_intersect_superset_of_card_set_8s() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
            .include_any(&CardSet::from_iter([Card::new(Rank::Eight, Suit::Spade),])),
            false
        );
    }

    #[test]
    fn card_set_askcqhjd2c_intersects_empty_card_set() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
            .include_any(&CardSet::empty()),
            false
        );
    }

    #[test]
    fn empty_card_set_intersects_empty_card_set() {
        assert_eq!(CardSet::empty().include_any(&CardSet::empty()), false);
    }
}

#[cfg(test)]
mod tests_include {
    use crate::rank::Rank;
    use crate::suit::Suit;

    use super::*;

    #[test]
    fn card_set_askcqhjd2c_is_supperset_of_card_as() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
            .include(&Card::new(Rank::Ace, Suit::Spade)),
            true
        );
    }

    #[test]
    fn card_set_askcqhjd2c_is_supperset_of_card_kc() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
            .include(&Card::new(Rank::King, Suit::Club)),
            true
        );
    }

    #[test]
    fn card_set_askcqhjd2c_is_supperset_of_card_qh() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
            .include(&Card::new(Rank::Queen, Suit::Heart)),
            true
        );
    }

    #[test]
    fn card_set_askcqhjd2c_is_supperset_of_card_jd() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
            .include(&Card::new(Rank::Jack, Suit::Diamond)),
            true
        );
    }

    #[test]
    fn card_set_askcqhjd2c_is_supperset_of_card_2c() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
            .include(&Card::new(Rank::Deuce, Suit::Club)),
            true
        );
    }

    #[test]
    fn card_set_askcqhjd2c_is_not_supperset_of_card_ah() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
            .include(&Card::new(Rank::Ace, Suit::Heart)),
            false
        );
    }

    #[test]
    fn card_set_askcqhjd2c_is_not_supperset_of_card_2s() {
        assert_eq!(
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
            .include(&Card::new(Rank::Deuce, Suit::Spade)),
            false
        );
    }
}

#[cfg(test)]
mod tests_insert_all {
    use crate::rank::Rank;
    use crate::suit::Suit;

    use super::*;

    #[test]
    fn it_inserts_all_in_card_set_ahksqdjc() {
        let mut card_set = CardSet::from_iter([
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::King, Suit::Club),
            Card::new(Rank::Queen, Suit::Heart),
            Card::new(Rank::Jack, Suit::Diamond),
            Card::new(Rank::Deuce, Suit::Club),
        ]);

        card_set.insert_all(&CardSet::from_iter([
            Card::new(Rank::Ace, Suit::Heart),
            Card::new(Rank::King, Suit::Spade),
            Card::new(Rank::Queen, Suit::Diamond),
            Card::new(Rank::Jack, Suit::Club),
        ]));

        assert_eq!(
            card_set,
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::Ace, Suit::Heart),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::King, Suit::Spade),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Queen, Suit::Diamond),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Jack, Suit::Club),
                Card::new(Rank::Deuce, Suit::Club),
            ])
        );
    }

    #[test]
    fn it_inserts_only_diff_with_card_set_asksqhjd() {
        let mut card_set = CardSet::from_iter([
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::King, Suit::Club),
            Card::new(Rank::Queen, Suit::Heart),
            Card::new(Rank::Jack, Suit::Diamond),
            Card::new(Rank::Deuce, Suit::Club),
        ]);

        card_set.insert_all(&CardSet::from_iter([
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::King, Suit::Spade),
            Card::new(Rank::Queen, Suit::Heart),
            Card::new(Rank::Jack, Suit::Diamond),
        ]));

        assert_eq!(
            card_set,
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::King, Suit::Spade),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
        );
    }

    #[test]
    fn it_inserts_all_in_card_set_askcqhjd_to_empty_card_set() {
        let mut card_set = CardSet::empty();

        card_set.insert_all(&CardSet::from_iter([
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::King, Suit::Club),
            Card::new(Rank::Queen, Suit::Heart),
            Card::new(Rank::Jack, Suit::Diamond),
        ]));

        assert_eq!(
            card_set,
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
            ])
        );
    }

    #[test]
    fn it_returns_false_when_all_in_card_set_already_included() {
        let mut card_set = CardSet::from_iter([
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::King, Suit::Club),
            Card::new(Rank::Queen, Suit::Heart),
            Card::new(Rank::Jack, Suit::Diamond),
        ]);

        let result = card_set.insert_all(&CardSet::from_iter([
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::King, Suit::Club),
            Card::new(Rank::Queen, Suit::Heart),
            Card::new(Rank::Jack, Suit::Diamond),
        ]));

        assert_eq!(
            card_set,
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
            ])
        );
        assert_eq!(result, false);
    }
}

#[cfg(test)]
mod tests_insert {
    use crate::rank::Rank;
    use crate::suit::Suit;

    use super::*;

    #[test]
    fn it_inserts_2c_to_askcqhjd() {
        let mut card_set = CardSet::from_iter([
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::King, Suit::Club),
            Card::new(Rank::Queen, Suit::Heart),
            Card::new(Rank::Jack, Suit::Diamond),
        ]);

        card_set.insert(&Card::new(Rank::Deuce, Suit::Club));

        assert_eq!(
            card_set,
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
        );
    }

    #[test]
    fn it_inserts_as_to_empty_card_set() {
        let mut card_set: CardSet = CardSet::empty();
        card_set.insert(&Card::new(Rank::Ace, Suit::Spade));

        assert_eq!(
            card_set,
            CardSet::from_iter([Card::new(Rank::Ace, Suit::Spade),])
        );
    }

    #[test]
    fn it_returns_false_when_card_already_included() {
        let mut card_set: CardSet = CardSet::from_iter([
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::King, Suit::Club),
            Card::new(Rank::Queen, Suit::Heart),
            Card::new(Rank::Jack, Suit::Diamond),
        ]);

        let result = card_set.insert(&Card::new(Rank::Jack, Suit::Diamond));

        assert_eq!(
            card_set,
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
            ])
        );
        assert_eq!(result, false);
    }
}

#[cfg(test)]
mod tests_remove_all {
    use crate::rank::Rank;
    use crate::suit::Suit;

    use super::*;

    #[test]
    fn it_removes_card_set_asqh_from_card_set_askcqhjd() {
        let mut card_set: CardSet = CardSet::from_iter([
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::King, Suit::Club),
            Card::new(Rank::Queen, Suit::Heart),
            Card::new(Rank::Jack, Suit::Diamond),
        ]);

        card_set.remove_all(&CardSet::from_iter([
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Queen, Suit::Heart),
        ]));

        assert_eq!(
            card_set,
            CardSet::from_iter([
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Jack, Suit::Diamond),
            ])
        );
    }

    #[test]
    fn it_removes_card_set_asqh2c_from_card_set_askcqhjd() {
        let mut card_set: CardSet = CardSet::from_iter([
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::King, Suit::Club),
            Card::new(Rank::Queen, Suit::Heart),
            Card::new(Rank::Jack, Suit::Diamond),
        ]);

        card_set.remove_all(&CardSet::from_iter([
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Queen, Suit::Heart),
            Card::new(Rank::Deuce, Suit::Club),
        ]));

        assert_eq!(
            card_set,
            CardSet::from_iter([
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Jack, Suit::Diamond),
            ])
        );
    }

    #[test]
    fn it_returns_false_when_cards_not_included() {
        let mut card_set: CardSet = CardSet::from_iter([
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::King, Suit::Club),
            Card::new(Rank::Queen, Suit::Heart),
            Card::new(Rank::Jack, Suit::Diamond),
        ]);

        let result = card_set.remove_all(&CardSet::from_iter([
            Card::new(Rank::Seven, Suit::Spade),
            Card::new(Rank::Deuce, Suit::Club),
        ]));

        assert_eq!(
            card_set,
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
            ])
        );
        assert_eq!(result, false);
    }
}

#[cfg(test)]
mod tests_remove {
    use crate::rank::Rank;
    use crate::suit::Suit;

    use super::*;

    #[test]
    fn it_removes_card_qh_from_card_set_askcqhjd() {
        let mut card_set: CardSet = CardSet::from_iter([
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::King, Suit::Club),
            Card::new(Rank::Queen, Suit::Heart),
            Card::new(Rank::Jack, Suit::Diamond),
        ]);

        card_set.remove(&Card::new(Rank::Queen, Suit::Heart));

        assert_eq!(
            card_set,
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Jack, Suit::Diamond),
            ])
        );
    }

    #[test]
    fn it_returns_false_when_card_not_included() {
        let mut card_set: CardSet = CardSet::from_iter([
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::King, Suit::Club),
            Card::new(Rank::Queen, Suit::Heart),
            Card::new(Rank::Jack, Suit::Diamond),
        ]);

        let result = card_set.remove(&Card::new(Rank::Seven, Suit::Spade));

        assert_eq!(
            card_set,
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
            ])
        );
        assert_eq!(result, false);
    }
}

impl IntoIterator for &CardSet {
    type Item = Card;

    type IntoIter = CardSetIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl IntoIterator for &mut CardSet {
    type Item = Card;

    type IntoIter = CardSetIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl IntoIterator for CardSet {
    type Item = Card;

    type IntoIter = CardSetIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests_into_iterator {
    use crate::rank::Rank;
    use crate::suit::Suit;

    use super::*;

    #[test]
    fn it_iterates_over_card_set_askcqhjd2c() {
        let card_set = CardSet::from_iter([
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::King, Suit::Club),
            Card::new(Rank::Queen, Suit::Heart),
            Card::new(Rank::Jack, Suit::Diamond),
            Card::new(Rank::Deuce, Suit::Club),
        ]);

        let mut iter = card_set.into_iter();

        assert_eq!(iter.next(), Some(Card::new(Rank::Ace, Suit::Spade)));
        assert_eq!(iter.next(), Some(Card::new(Rank::King, Suit::Club)));
        assert_eq!(iter.next(), Some(Card::new(Rank::Queen, Suit::Heart)));
        assert_eq!(iter.next(), Some(Card::new(Rank::Jack, Suit::Diamond)));
        assert_eq!(iter.next(), Some(Card::new(Rank::Deuce, Suit::Club)));
        assert_eq!(iter.next(), None);
    }
}

impl Display for CardSet {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        let mut cards: Vec<Card> = self.into_iter().collect();

        cards.sort();

        let mut output: String = "".to_string();

        for card in cards {
            output.push_str(&card.to_string());
        }

        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests_display {
    use crate::rank::Rank;
    use crate::suit::Suit;

    use super::*;

    #[test]
    fn it_displays_card_set_askcqhjd2c() {
        let card_set = CardSet::from_iter([
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::King, Suit::Club),
            Card::new(Rank::Queen, Suit::Heart),
            Card::new(Rank::Jack, Suit::Diamond),
            Card::new(Rank::Deuce, Suit::Club),
        ]);

        assert_eq!(card_set.to_string(), "AsKcQhJd2c");
    }

    #[test]
    fn it_displays_full_card_set() {
        let card_set = CardSet::full();

        assert_eq!(card_set.to_string(), "AsAhAdAcKsKhKdKcQsQhQdQcJsJhJdJcTsThTdTc9s9h9d9c8s8h8d8c7s7h7d7c6s6h6d6c5s5h5d5c4s4h4d4c3s3h3d3c2s2h2d2c");
    }

    #[test]
    fn it_displays_empty_card_set() {
        let card_set = CardSet::empty();

        assert_eq!(card_set.to_string(), "");
    }
}

impl Debug for CardSet {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        write!(f, "CardSet({}; {})", self.to_string(), self.len())
    }
}

impl FromStr for CardSet {
    type Err = ParseCardSetError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() % 2 == 0 {
            let mut bit = 0;
            let mut len = 0;

            for i in 0..value.len() / 2 {
                let card = match Card::from_str(&value[i * 2..i * 2 + 2]) {
                    Ok(card) => card,
                    _ => {
                        return Err(ParseCardSetError(value.to_string()));
                    }
                };
                let card_bit: u64 = card.into();

                bit = bit | card_bit;
                len += 1;
            }

            return Ok(CardSet { bit, len });
        }

        Err(ParseCardSetError(value.to_string()))
    }
}

#[cfg(test)]
mod tests_from_str {
    use crate::rank::Rank;
    use crate::suit::Suit;

    use super::*;

    #[test]
    fn it_parses_str_into_card_set_askcqhjd() {
        assert_eq!(
            CardSet::from_str("AsKcQhJd2c").unwrap(),
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
        );
    }

    #[test]
    fn it_returns_error_when_invalid_string() {
        assert_eq!(
            CardSet::from_str("AsKj").unwrap_err().to_string(),
            "AsKj is invalid string as a card set."
        );
    }
}

impl From<u64> for CardSet {
    fn from(bit: u64) -> Self {
        CardSet::new(bit, get_bit_len(&bit))
    }
}

#[cfg(test)]
mod tests_from_u64 {
    use crate::rank::Rank;
    use crate::suit::Suit;

    use super::*;

    #[test]
    fn it_converts_u64_into_card_set_askcqhjd2c() {
        assert_eq!(
            CardSet::from(0b1000000000000000000000000000000000000100001010000001),
            CardSet::from_iter([
                Card::new(Rank::Ace, Suit::Spade),
                Card::new(Rank::King, Suit::Club),
                Card::new(Rank::Queen, Suit::Heart),
                Card::new(Rank::Jack, Suit::Diamond),
                Card::new(Rank::Deuce, Suit::Club),
            ])
        );
    }
}

impl FromIterator<Card> for CardSet {
    fn from_iter<I: IntoIterator<Item = Card>>(iter: I) -> Self {
        let mut bit = 0_u64;
        let mut len = 0_u8;

        for card in iter {
            bit = bit | u64::from(card);
            len += 1;
        }

        CardSet::new(bit, len)
    }
}

pub struct CardSetIterator {
    original: u64,
    current: u64,
}

impl CardSetIterator {
    fn new(original: u64) -> Self {
        CardSetIterator {
            original: original,
            current: 0,
        }
    }
}

impl Iterator for CardSetIterator {
    type Item = Card;

    fn next(&mut self) -> Option<Card> {
        let mut card = None;

        while card == None && self.current <= 52 {
            let card_bit = self.original & (1 << self.current);

            if card_bit > 0 {
                card = Some(Card::from(card_bit));
            }

            self.current += 1;
        }

        card
    }
}

#[derive(Debug)]
pub struct ParseCardSetError(String);

impl Display for ParseCardSetError {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        write!(f, "{} is invalid string as a card set.", self.0)
    }
}

impl From<CardSet> for u64 {
    fn from(card_set: CardSet) -> Self {
        card_set.bit
    }
}

fn get_bit_len(bit: &u64) -> u8 {
    let mut bit = *bit;
    let mut len = 0;

    while bit != 0 {
        len += 1;
        bit = bit & (bit - 1);
    }

    len
}
