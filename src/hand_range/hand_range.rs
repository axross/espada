use super::{CardPair, RankPair};
use crate::card::{Card, Rank, RankRange, Suit, SuitRange};
use crate::hand_range::{HandRangeToken, HandRangeTokenKind};
use fxhash::FxBuildHasher;
use std::collections::{hash_map, HashMap};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct HandRange(HashMap<CardPair, f32, FxBuildHasher>);

impl HandRange {
    pub fn empty() -> HandRange {
        HandRange(HashMap::with_hasher(FxBuildHasher::default()))
    }

    pub fn card_pairs(&self) -> &HashMap<CardPair, f32, FxBuildHasher> {
        &self.0
    }

    pub fn rank_pairs(&self) -> HashMap<RankPair, f32, FxBuildHasher> {
        let mut rank_pairs = HashMap::with_hasher(FxBuildHasher::default());

        for rank in RankRange::all() {
            let example_pocket =
                CardPair::new(Card::new(rank, Suit::Spade), Card::new(rank, Suit::Heart));

            if self.0.contains_key(&example_pocket) {
                let pocket = RankPair::Pocket(rank);
                let probability = self.0.get(&example_pocket).unwrap();

                if pocket
                    .into_iter()
                    .all(|cp| self.0.get(&cp).is_some_and(|p| p == probability))
                {
                    rank_pairs.insert(pocket, *probability);
                }
            }
        }

        for high in RankRange::inclusive(Rank::Ace, Rank::Trey) {
            for kicker in RankRange::inclusive(high.next().unwrap(), Rank::Deuce) {
                let example_suited =
                    CardPair::new(Card::new(high, Suit::Spade), Card::new(kicker, Suit::Spade));

                if self.0.contains_key(&example_suited) {
                    let suited = RankPair::Suited(high, kicker);
                    let probability = self.0.get(&example_suited).unwrap();

                    if suited
                        .into_iter()
                        .all(|cp| self.0.get(&cp).is_some_and(|p| p == probability))
                    {
                        rank_pairs.insert(suited, *probability);
                    }
                }

                let example_ofsuit =
                    CardPair::new(Card::new(high, Suit::Spade), Card::new(kicker, Suit::Heart));

                if self.0.contains_key(&example_ofsuit) {
                    let ofsuit = RankPair::Ofsuit(high, kicker);
                    let probability = self.0.get(&example_ofsuit).unwrap();

                    if ofsuit
                        .into_iter()
                        .all(|cp| self.0.get(&cp).is_some_and(|p| p == probability))
                    {
                        rank_pairs.insert(ofsuit, *probability);
                    }
                }
            }
        }

        rank_pairs
    }

    // TODO:
    // this logic sucks. we gotta revisit and rewrite in some appropriate way.
    pub fn orphan_card_pairs(&self) -> HashMap<CardPair, f32, FxBuildHasher> {
        let mut clone = self.0.clone();
        let rank_pairs = self.rank_pairs();

        for (rank_pair, _) in rank_pairs {
            for card_pair in rank_pair {
                clone.remove(&card_pair);
            }
        }

        clone
    }
}

impl<'a> IntoIterator for &'a HandRange {
    type Item = (&'a CardPair, &'a f32);

    type IntoIter = hash_map::Iter<'a, CardPair, f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Display for HandRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let rank_pairs = self.rank_pairs();
        let orphan_card_pairs = self.orphan_card_pairs();
        let mut tokens = vec![];

        let mut pocket_start_rank = None;

        for rank in RankRange::all() {
            let probability = rank_pairs.get(&RankPair::Pocket(rank));

            if let Some(start_rank) = pocket_start_rank {
                let start_probability = rank_pairs.get(&RankPair::Pocket(start_rank)).unwrap();

                if probability.is_none() || probability.unwrap_or(&0_f32) != start_probability {
                    let prev_rank = rank.prev().unwrap();

                    if start_rank == Rank::Ace && prev_rank != Rank::Ace {
                        tokens.push(HandRangeToken::new(
                            HandRangeTokenKind::BottomClosedRankPairRange(RankPair::Pocket(
                                prev_rank,
                            )),
                            *start_probability,
                        ));
                    } else if start_rank == prev_rank {
                        tokens.push(HandRangeToken::new(
                            HandRangeTokenKind::SingleRankPair(RankPair::Pocket(prev_rank)),
                            *start_probability,
                        ));
                    } else {
                        tokens.push(HandRangeToken::new(
                            HandRangeTokenKind::DoubleClosedRankPairRange(
                                RankPair::Pocket(start_rank),
                                prev_rank,
                            ),
                            *start_probability,
                        ));
                    }

                    pocket_start_rank = None;
                }
            }

            if pocket_start_rank.is_none() && probability.is_some() {
                pocket_start_rank = Some(rank);
            }
        }

        if let Some(start_rank) = pocket_start_rank {
            let last_rank = Rank::Deuce;
            let probability = rank_pairs.get(&RankPair::Pocket(start_rank)).unwrap();

            if start_rank == Rank::Ace {
                tokens.push(HandRangeToken::new(
                    HandRangeTokenKind::BottomClosedRankPairRange(RankPair::Pocket(last_rank)),
                    *probability,
                ));
            } else if start_rank == last_rank {
                tokens.push(HandRangeToken::new(
                    HandRangeTokenKind::SingleRankPair(RankPair::Pocket(start_rank)),
                    *probability,
                ));
            } else {
                tokens.push(HandRangeToken::new(
                    HandRangeTokenKind::DoubleClosedRankPairRange(
                        RankPair::Pocket(start_rank),
                        last_rank,
                    ),
                    *probability,
                ));
            }
        }

        for high in RankRange::inclusive(Rank::Ace, Rank::Trey) {
            let first_rank = high.next().unwrap();

            let mut suited_start_rank = None;

            for kicker in RankRange::inclusive(first_rank, Rank::Deuce) {
                let probability = rank_pairs.get(&RankPair::Suited(high, kicker));

                if let Some(start_rank) = suited_start_rank {
                    let start_probability =
                        rank_pairs.get(&RankPair::Suited(high, start_rank)).unwrap();

                    if probability.is_none() || probability.unwrap_or(&0_f32) != start_probability {
                        let prev_rank = kicker.prev().unwrap();

                        if start_rank == first_rank && prev_rank != first_rank {
                            tokens.push(HandRangeToken::new(
                                HandRangeTokenKind::BottomClosedRankPairRange(RankPair::Suited(
                                    high, prev_rank,
                                )),
                                *start_probability,
                            ));
                        } else if start_rank == prev_rank {
                            tokens.push(HandRangeToken::new(
                                HandRangeTokenKind::SingleRankPair(RankPair::Suited(
                                    high, prev_rank,
                                )),
                                *start_probability,
                            ));
                        } else {
                            tokens.push(HandRangeToken::new(
                                HandRangeTokenKind::DoubleClosedRankPairRange(
                                    RankPair::Suited(high, start_rank),
                                    prev_rank,
                                ),
                                *start_probability,
                            ));
                        }

                        suited_start_rank = None;
                    }
                }

                if suited_start_rank.is_none() && probability.is_some() {
                    suited_start_rank = Some(kicker);
                }
            }

            if let Some(suited_start_rank) = suited_start_rank {
                let probability = rank_pairs
                    .get(&RankPair::Suited(high, suited_start_rank))
                    .unwrap();
                let last_rank = Rank::Deuce;

                if suited_start_rank == first_rank && last_rank != first_rank {
                    tokens.push(HandRangeToken::new(
                        HandRangeTokenKind::BottomClosedRankPairRange(RankPair::Suited(
                            high, last_rank,
                        )),
                        *probability,
                    ));
                } else if suited_start_rank == last_rank {
                    tokens.push(HandRangeToken::new(
                        HandRangeTokenKind::SingleRankPair(RankPair::Suited(
                            high,
                            suited_start_rank,
                        )),
                        *probability,
                    ));
                } else {
                    tokens.push(HandRangeToken::new(
                        HandRangeTokenKind::DoubleClosedRankPairRange(
                            RankPair::Suited(high, suited_start_rank),
                            last_rank,
                        ),
                        *probability,
                    ));
                }
            }

            let mut ofsuit_start_rank = None;

            for kicker in RankRange::inclusive(first_rank, Rank::Deuce) {
                let probability = rank_pairs.get(&RankPair::Ofsuit(high, kicker));

                if let Some(start_rank) = ofsuit_start_rank {
                    let start_probability =
                        rank_pairs.get(&RankPair::Ofsuit(high, start_rank)).unwrap();

                    if probability.is_none() || probability.unwrap_or(&0_f32) != start_probability {
                        let prev_rank = kicker.prev().unwrap();

                        if start_rank == first_rank && prev_rank != first_rank {
                            tokens.push(HandRangeToken::new(
                                HandRangeTokenKind::BottomClosedRankPairRange(RankPair::Ofsuit(
                                    high, prev_rank,
                                )),
                                *start_probability,
                            ));
                        } else if start_rank == prev_rank {
                            tokens.push(HandRangeToken::new(
                                HandRangeTokenKind::SingleRankPair(RankPair::Ofsuit(
                                    high, prev_rank,
                                )),
                                *start_probability,
                            ));
                        } else {
                            tokens.push(HandRangeToken::new(
                                HandRangeTokenKind::DoubleClosedRankPairRange(
                                    RankPair::Ofsuit(high, start_rank),
                                    prev_rank,
                                ),
                                *start_probability,
                            ));
                        }

                        ofsuit_start_rank = None;
                    }
                }

                if ofsuit_start_rank.is_none() && probability.is_some() {
                    ofsuit_start_rank = Some(kicker);
                }
            }

            if let Some(ofsuit_start_rank) = ofsuit_start_rank {
                let probability = rank_pairs
                    .get(&RankPair::Ofsuit(high, ofsuit_start_rank))
                    .unwrap();
                let last_rank = Rank::Deuce;

                if ofsuit_start_rank == first_rank && last_rank != first_rank {
                    tokens.push(HandRangeToken::new(
                        HandRangeTokenKind::BottomClosedRankPairRange(RankPair::Ofsuit(
                            high, last_rank,
                        )),
                        *probability,
                    ));
                } else if ofsuit_start_rank == last_rank {
                    tokens.push(HandRangeToken::new(
                        HandRangeTokenKind::SingleRankPair(RankPair::Ofsuit(
                            high,
                            ofsuit_start_rank,
                        )),
                        *probability,
                    ));
                } else {
                    tokens.push(HandRangeToken::new(
                        HandRangeTokenKind::DoubleClosedRankPairRange(
                            RankPair::Ofsuit(high, ofsuit_start_rank),
                            last_rank,
                        ),
                        *probability,
                    ));
                }
            }
        }

        for high_rank in RankRange::all() {
            for kicker_rank in RankRange::inclusive(high_rank, Rank::Deuce) {
                for high_suit in SuitRange::all() {
                    for kicker_suit in SuitRange::all() {
                        let pair = CardPair::new(
                            Card::new(high_rank, high_suit),
                            Card::new(kicker_rank, kicker_suit),
                        );
                        let probability = orphan_card_pairs.get(&pair);

                        if let Some(probability) = probability {
                            tokens.push(HandRangeToken::new(
                                HandRangeTokenKind::SingleCardPair(pair),
                                *probability,
                            ));
                        }
                    }
                }
            }
        }

        let mut res = f.write_str("");
        let mut is_empty = true;

        for token in tokens {
            if is_empty {
                res = res.and(token.fmt(f));
            } else {
                res = res.and(write!(f, ",{}", token));
            }

            is_empty = false;
        }

        res
    }
}

impl FromStr for HandRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::with_hasher(FxBuildHasher::default());
        let trimmed = s.replace(" ", "");

        if trimmed.len() == 0 {
            return Ok(HandRange(HashMap::with_hasher(FxBuildHasher::default())));
        }

        let haystacks = trimmed.split(",");

        for h in haystacks {
            if let Ok(token) = HandRangeToken::from_str(h) {
                for (card_pair, prob) in token {
                    map.insert(card_pair, prob);
                }
            }
        }

        Ok(HandRange(map))
    }
}

impl FromIterator<(CardPair, f32)> for HandRange {
    fn from_iter<T: IntoIterator<Item = (CardPair, f32)>>(iter: T) -> Self {
        HandRange(iter.into_iter().collect())
    }
}

impl FromIterator<CardPair> for HandRange {
    fn from_iter<T: IntoIterator<Item = CardPair>>(iter: T) -> Self {
        iter.into_iter().map(|cs| (cs, 1.0)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod display {
        use super::*;

        #[test]
        fn it_formats_pocket_deuce_plus() {
            assert_eq!(
                RankRange::all()
                    .into_iter()
                    .flat_map(|r| RankPair::Pocket(r).into_iter())
                    .collect::<HandRange>()
                    .to_string(),
                "22+"
            );
        }

        #[test]
        fn it_formats_pocket_king_to_jack() {
            assert_eq!(
                RankRange::inclusive(Rank::King, Rank::Jack)
                    .into_iter()
                    .flat_map(|r| RankPair::Pocket(r).into_iter())
                    .collect::<HandRange>()
                    .to_string(),
                "KK-JJ"
            );
        }

        #[test]
        fn it_formats_pocket_five_to_deuce() {
            assert_eq!(
                RankRange::inclusive(Rank::Five, Rank::Deuce)
                    .into_iter()
                    .flat_map(|r| RankPair::Pocket(r).into_iter())
                    .collect::<HandRange>()
                    .to_string(),
                "55-22"
            );
        }

        #[test]
        fn it_formats_pocket_ace() {
            assert_eq!(
                HandRange::from_iter(RankPair::Pocket(Rank::Ace)).to_string(),
                "AA"
            );
        }

        #[test]
        fn it_formats_pocket_deuce() {
            assert_eq!(
                RankPair::Pocket(Rank::Deuce)
                    .into_iter()
                    .collect::<HandRange>()
                    .to_string(),
                "22"
            );
        }

        #[test]
        fn it_combines_and_formats_pocket_eight_to_four() {
            assert_eq!(
                RankRange::inclusive(Rank::Eight, Rank::Six)
                    .into_iter()
                    .flat_map(|r| RankPair::Pocket(r).into_iter())
                    .chain(
                        RankRange::inclusive(Rank::Five, Rank::Four)
                            .into_iter()
                            .flat_map(|r| RankPair::Pocket(r))
                    )
                    .collect::<HandRange>()
                    .to_string(),
                "88-44"
            );
        }

        #[test]
        fn it_does_not_combines_but_formats_pocket_ace_to_queen_and_ten_to_eight() {
            assert_eq!(
                RankRange::inclusive(Rank::Ace, Rank::Queen)
                    .into_iter()
                    .flat_map(|r| RankPair::Pocket(r).into_iter())
                    .chain(
                        RankRange::inclusive(Rank::Ten, Rank::Eight)
                            .into_iter()
                            .flat_map(|r| RankPair::Pocket(r))
                    )
                    .collect::<HandRange>()
                    .to_string(),
                "QQ+,TT-88"
            );
        }

        #[test]
        fn it_does_not_combines_but_formats_pocket_eight_to_six_and_five_to_four_with_different_prob(
        ) {
            assert_eq!(
                RankRange::inclusive(Rank::Eight, Rank::Six)
                    .into_iter()
                    .flat_map(|r| RankPair::Pocket(r).into_iter().map(|cs| (cs, 0.5_f32)),)
                    .chain(
                        RankRange::inclusive(Rank::Five, Rank::Four)
                            .into_iter()
                            .flat_map(|r| RankPair::Pocket(r).into_iter().map(|cs| (cs, 1_f32)))
                    )
                    .collect::<HandRange>()
                    .to_string(),
                "88-66:0.5,55-44"
            );
        }

        #[test]
        fn it_formats_combined_pocket_seven_to_six() {
            assert_eq!(
                [
                    CardPair::new(
                        Card::new(Rank::Seven, Suit::Spade),
                        Card::new(Rank::Seven, Suit::Heart)
                    ),
                    CardPair::new(
                        Card::new(Rank::Seven, Suit::Spade),
                        Card::new(Rank::Seven, Suit::Diamond)
                    ),
                    CardPair::new(
                        Card::new(Rank::Seven, Suit::Spade),
                        Card::new(Rank::Seven, Suit::Club)
                    ),
                    CardPair::new(
                        Card::new(Rank::Seven, Suit::Heart),
                        Card::new(Rank::Seven, Suit::Diamond)
                    ),
                    CardPair::new(
                        Card::new(Rank::Seven, Suit::Heart),
                        Card::new(Rank::Seven, Suit::Club)
                    ),
                    CardPair::new(
                        Card::new(Rank::Seven, Suit::Diamond),
                        Card::new(Rank::Seven, Suit::Club)
                    ),
                    CardPair::new(
                        Card::new(Rank::Six, Suit::Spade),
                        Card::new(Rank::Six, Suit::Heart)
                    ),
                    CardPair::new(
                        Card::new(Rank::Six, Suit::Spade),
                        Card::new(Rank::Six, Suit::Diamond)
                    ),
                    CardPair::new(
                        Card::new(Rank::Six, Suit::Spade),
                        Card::new(Rank::Six, Suit::Club)
                    ),
                    CardPair::new(
                        Card::new(Rank::Six, Suit::Heart),
                        Card::new(Rank::Six, Suit::Diamond)
                    ),
                    CardPair::new(
                        Card::new(Rank::Six, Suit::Heart),
                        Card::new(Rank::Six, Suit::Club)
                    ),
                    CardPair::new(
                        Card::new(Rank::Six, Suit::Diamond),
                        Card::new(Rank::Six, Suit::Club)
                    ),
                ]
                .into_iter()
                .collect::<HandRange>()
                .to_string(),
                "77-66"
            );
        }

        #[test]
        fn it_formats_combined_pocket_jack() {
            assert_eq!(
                [
                    CardPair::new(
                        Card::new(Rank::Jack, Suit::Spade),
                        Card::new(Rank::Jack, Suit::Heart)
                    ),
                    CardPair::new(
                        Card::new(Rank::Jack, Suit::Spade),
                        Card::new(Rank::Jack, Suit::Diamond)
                    ),
                    CardPair::new(
                        Card::new(Rank::Jack, Suit::Spade),
                        Card::new(Rank::Jack, Suit::Club)
                    ),
                    CardPair::new(
                        Card::new(Rank::Jack, Suit::Heart),
                        Card::new(Rank::Jack, Suit::Diamond)
                    ),
                    CardPair::new(
                        Card::new(Rank::Jack, Suit::Heart),
                        Card::new(Rank::Jack, Suit::Club)
                    ),
                    CardPair::new(
                        Card::new(Rank::Jack, Suit::Diamond),
                        Card::new(Rank::Jack, Suit::Club)
                    ),
                ]
                .into_iter()
                .collect::<HandRange>()
                .to_string(),
                "JJ"
            );
        }

        #[test]
        fn it_formats_incomplete_pocket_jacks() {
            assert_eq!(
                [
                    CardPair::new(
                        Card::new(Rank::Jack, Suit::Spade),
                        Card::new(Rank::Jack, Suit::Heart)
                    ),
                    CardPair::new(
                        Card::new(Rank::Jack, Suit::Spade),
                        Card::new(Rank::Jack, Suit::Diamond)
                    ),
                    CardPair::new(
                        Card::new(Rank::Jack, Suit::Spade),
                        Card::new(Rank::Jack, Suit::Club)
                    ),
                    CardPair::new(
                        Card::new(Rank::Jack, Suit::Heart),
                        Card::new(Rank::Jack, Suit::Diamond)
                    ),
                    CardPair::new(
                        Card::new(Rank::Jack, Suit::Heart),
                        Card::new(Rank::Jack, Suit::Club)
                    ),
                ]
                .into_iter()
                .collect::<HandRange>()
                .to_string(),
                "JsJh,JsJd,JsJc,JsJh,JhJd,JhJc,JsJd,JhJd,JsJc,JhJc"
            );
        }

        #[test]
        fn it_formats_incomplete_pocket_jacks_with_different_prob() {
            assert_eq!(
            [
                (
                    CardPair::new(
                        Card::new(Rank::Jack, Suit::Spade),
                        Card::new(Rank::Jack, Suit::Heart)
                    ),
                    0.5
                ),
                (
                    CardPair::new(
                        Card::new(Rank::Jack, Suit::Spade),
                        Card::new(Rank::Jack, Suit::Diamond)
                    ),
                    1.0
                ),
                (
                    CardPair::new(
                        Card::new(Rank::Jack, Suit::Spade),
                        Card::new(Rank::Jack, Suit::Club)
                    ),
                    0.5
                ),
                (
                    CardPair::new(
                        Card::new(Rank::Jack, Suit::Heart),
                        Card::new(Rank::Jack, Suit::Diamond)
                    ),
                    0.5
                ),
                (
                    CardPair::new(
                        Card::new(Rank::Jack, Suit::Heart),
                        Card::new(Rank::Jack, Suit::Club)
                    ),
                    0.5
                ),
                (
                    CardPair::new(
                        Card::new(Rank::Jack, Suit::Diamond),
                        Card::new(Rank::Jack, Suit::Club)
                    ),
                    0.5
                ),
            ].into_iter().collect::<HandRange>()
            .to_string(),
            "JsJh:0.5,JsJd,JsJc:0.5,JsJh:0.5,JhJd:0.5,JhJc:0.5,JsJd,JhJd:0.5,JdJc:0.5,JsJc:0.5,JhJc:0.5,JdJc:0.5"
        );
        }

        #[test]
        fn it_formats_suited_six_two_plus() {
            assert_eq!(
                RankRange::inclusive(Rank::Five, Rank::Deuce)
                    .into_iter()
                    .flat_map(|r| RankPair::Suited(Rank::Six, r))
                    .collect::<HandRange>()
                    .to_string(),
                "62s+"
            );
        }

        #[test]
        fn it_formats_suited_six_four_to_trey() {
            assert_eq!(
                RankRange::inclusive(Rank::Four, Rank::Trey)
                    .into_iter()
                    .flat_map(|r| RankPair::Suited(Rank::Six, r))
                    .collect::<HandRange>()
                    .to_string(),
                "64s-63s"
            );
        }

        #[test]
        fn it_formats_suited_six_five() {
            assert_eq!(
                RankPair::Suited(Rank::Six, Rank::Five)
                    .into_iter()
                    .collect::<HandRange>()
                    .to_string(),
                "65s"
            );
        }

        #[test]
        fn it_formats_suited_trey_deuce() {
            assert_eq!(
                RankPair::Suited(Rank::Trey, Rank::Deuce)
                    .into_iter()
                    .collect::<HandRange>()
                    .to_string(),
                "32s"
            );
        }

        #[test]
        fn it_formats_combined_suited_ace_king() {
            assert_eq!(
                [
                    CardPair::new(
                        Card::new(Rank::Ace, Suit::Spade),
                        Card::new(Rank::King, Suit::Spade)
                    ),
                    CardPair::new(
                        Card::new(Rank::Ace, Suit::Heart),
                        Card::new(Rank::King, Suit::Heart)
                    ),
                    CardPair::new(
                        Card::new(Rank::Ace, Suit::Diamond),
                        Card::new(Rank::King, Suit::Diamond)
                    ),
                    CardPair::new(
                        Card::new(Rank::Ace, Suit::Club),
                        Card::new(Rank::King, Suit::Club)
                    ),
                ]
                .into_iter()
                .collect::<HandRange>()
                .to_string(),
                "AKs"
            );
        }

        #[test]
        fn it_formats_incomplete_suited_ace_king() {
            assert_eq!(
                [
                    CardPair::new(
                        Card::new(Rank::Ace, Suit::Spade),
                        Card::new(Rank::King, Suit::Spade)
                    ),
                    CardPair::new(
                        Card::new(Rank::Ace, Suit::Heart),
                        Card::new(Rank::King, Suit::Heart)
                    ),
                    CardPair::new(
                        Card::new(Rank::Ace, Suit::Club),
                        Card::new(Rank::King, Suit::Club)
                    ),
                ]
                .into_iter()
                .collect::<HandRange>()
                .to_string(),
                "AsKs,AhKh,AcKc"
            );
        }

        #[test]
        fn it_formats_incomplete_suited_ace_king_with_different_prob() {
            assert_eq!(
                [
                    (
                        CardPair::new(
                            Card::new(Rank::Ace, Suit::Spade),
                            Card::new(Rank::King, Suit::Spade)
                        ),
                        0.80
                    ),
                    (
                        CardPair::new(
                            Card::new(Rank::Ace, Suit::Heart),
                            Card::new(Rank::King, Suit::Heart)
                        ),
                        0.80
                    ),
                    (
                        CardPair::new(
                            Card::new(Rank::Ace, Suit::Diamond),
                            Card::new(Rank::King, Suit::Diamond)
                        ),
                        0.75
                    ),
                    (
                        CardPair::new(
                            Card::new(Rank::Ace, Suit::Club),
                            Card::new(Rank::King, Suit::Club)
                        ),
                        0.80
                    ),
                ]
                .into_iter()
                .collect::<HandRange>()
                .to_string(),
                "AsKs:0.8,AhKh:0.8,AdKd:0.75,AcKc:0.8"
            );
        }

        #[test]
        fn it_formats_ofsuit_six_two_plus() {
            assert_eq!(
                RankRange::inclusive(Rank::Six, Rank::Deuce)
                    .into_iter()
                    .flat_map(|r| RankPair::Ofsuit(Rank::Seven, r))
                    .collect::<HandRange>()
                    .to_string(),
                "72o+"
            );
        }

        #[test]
        fn it_formats_ofsuit_seven_five_to_deuce() {
            assert_eq!(
                RankRange::inclusive(Rank::Five, Rank::Deuce)
                    .into_iter()
                    .flat_map(|r| RankPair::Ofsuit(Rank::Seven, r))
                    .collect::<HandRange>()
                    .to_string(),
                "75o-72o"
            );
        }

        #[test]
        fn it_formats_ofsuit_six_five() {
            assert_eq!(
                RankPair::Ofsuit(Rank::Seven, Rank::Five)
                    .into_iter()
                    .collect::<HandRange>()
                    .to_string(),
                "75o"
            );
        }

        #[test]
        fn it_formats_ofsuit_trey_deuce() {
            assert_eq!(
                RankPair::Ofsuit(Rank::Four, Rank::Deuce)
                    .into_iter()
                    .collect::<HandRange>()
                    .to_string(),
                "42o"
            );
        }

        #[test]
        fn it_formats_combined_ofsuit_queen_nine() {
            assert_eq!(
                HandRange::from_iter([
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Spade),
                        Card::new(Rank::Nine, Suit::Heart)
                    ),
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Spade),
                        Card::new(Rank::Nine, Suit::Diamond)
                    ),
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Spade),
                        Card::new(Rank::Nine, Suit::Club)
                    ),
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Heart),
                        Card::new(Rank::Nine, Suit::Spade)
                    ),
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Heart),
                        Card::new(Rank::Nine, Suit::Diamond)
                    ),
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Heart),
                        Card::new(Rank::Nine, Suit::Club)
                    ),
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Diamond),
                        Card::new(Rank::Nine, Suit::Spade)
                    ),
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Diamond),
                        Card::new(Rank::Nine, Suit::Heart)
                    ),
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Diamond),
                        Card::new(Rank::Nine, Suit::Club)
                    ),
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Club),
                        Card::new(Rank::Nine, Suit::Spade)
                    ),
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Club),
                        Card::new(Rank::Nine, Suit::Heart)
                    ),
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Club),
                        Card::new(Rank::Nine, Suit::Diamond)
                    ),
                ])
                .to_string(),
                "Q9o"
            );
        }

        #[test]
        fn it_formats_incomplete_ofsuit_queen_nine() {
            assert_eq!(
                HandRange::from_iter([
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Spade),
                        Card::new(Rank::Nine, Suit::Heart)
                    ),
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Spade),
                        Card::new(Rank::Nine, Suit::Club)
                    ),
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Heart),
                        Card::new(Rank::Nine, Suit::Spade)
                    ),
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Heart),
                        Card::new(Rank::Nine, Suit::Diamond)
                    ),
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Heart),
                        Card::new(Rank::Nine, Suit::Club)
                    ),
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Diamond),
                        Card::new(Rank::Nine, Suit::Spade)
                    ),
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Diamond),
                        Card::new(Rank::Nine, Suit::Heart)
                    ),
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Diamond),
                        Card::new(Rank::Nine, Suit::Club)
                    ),
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Club),
                        Card::new(Rank::Nine, Suit::Heart)
                    ),
                    CardPair::new(
                        Card::new(Rank::Queen, Suit::Club),
                        Card::new(Rank::Nine, Suit::Diamond)
                    ),
                ])
                .to_string(),
                "Qs9h,Qs9c,Qh9s,Qh9d,Qh9c,Qd9s,Qd9h,Qd9c,Qc9h,Qc9d"
            );
        }

        #[test]
        fn it_formats_incomplete_ofsuit_queen_nine_with_different_prob() {
            assert_eq!(
                HandRange::from_iter([
                    (
                        CardPair::new(
                            Card::new(Rank::Queen, Suit::Spade),
                            Card::new(Rank::Nine, Suit::Heart)
                        ),
                        1.0
                    ),
                    (
                        CardPair::new(
                            Card::new(Rank::Queen, Suit::Spade),
                            Card::new(Rank::Nine, Suit::Diamond)
                        ),
                        1.0
                    ),
                    (
                        CardPair::new(
                            Card::new(Rank::Queen, Suit::Spade),
                            Card::new(Rank::Nine, Suit::Club)
                        ),
                        1.0
                    ),
                    (
                        CardPair::new(
                            Card::new(Rank::Queen, Suit::Heart),
                            Card::new(Rank::Nine, Suit::Spade)
                        ),
                        1.0
                    ),
                    (
                        CardPair::new(
                            Card::new(Rank::Queen, Suit::Heart),
                            Card::new(Rank::Nine, Suit::Diamond)
                        ),
                        1.0
                    ),
                    (
                        CardPair::new(
                            Card::new(Rank::Queen, Suit::Heart),
                            Card::new(Rank::Nine, Suit::Club)
                        ),
                        0.2
                    ),
                    (
                        CardPair::new(
                            Card::new(Rank::Queen, Suit::Diamond),
                            Card::new(Rank::Nine, Suit::Spade)
                        ),
                        1.0
                    ),
                    (
                        CardPair::new(
                            Card::new(Rank::Queen, Suit::Diamond),
                            Card::new(Rank::Nine, Suit::Heart)
                        ),
                        1.0
                    ),
                    (
                        CardPair::new(
                            Card::new(Rank::Queen, Suit::Diamond),
                            Card::new(Rank::Nine, Suit::Club)
                        ),
                        1.0
                    ),
                    (
                        CardPair::new(
                            Card::new(Rank::Queen, Suit::Club),
                            Card::new(Rank::Nine, Suit::Spade)
                        ),
                        1.0
                    ),
                    (
                        CardPair::new(
                            Card::new(Rank::Queen, Suit::Club),
                            Card::new(Rank::Nine, Suit::Heart)
                        ),
                        1.0
                    ),
                    (
                        CardPair::new(
                            Card::new(Rank::Queen, Suit::Club),
                            Card::new(Rank::Nine, Suit::Diamond)
                        ),
                        1.0
                    ),
                ])
                .to_string(),
                "Qs9h,Qs9d,Qs9c,Qh9s,Qh9d,Qh9c:0.2,Qd9s,Qd9h,Qd9c,Qc9s,Qc9h,Qc9d"
            );
        }

        #[test]
        fn it_formats_complex_hand_range() {
            assert_eq!(
            HandRange::from_str("2d2c: 1,2h2c: 1,2h2d: 1,2s2c: 1,2s2d: 1,2s2h: 1,3d3c: 1,3h3c: 1,3h3d: 1,3s3c: 1,3s3d: 1,3s3h: 1,4d4c: 1,4h4c: 1,4h4d: 1,4s4c: 1,4s4d: 1,4s4h: 1,5c3c: 0.095,5c4c: 1,5d3d: 0.095,5d4d: 1,5d5c: 1,5h3h: 0.095,5h4h: 1,5h5c: 1,5h5d: 1,5s3s: 0.095,5s4s: 1,5s5c: 1,5s5d: 1,5s5h: 1,6c4c: 1,6c5c: 1,6d4d: 1,6d5d: 1,6d6c: 1,6h4h: 1,6h5h: 1,6h6c: 1,6h6d: 1,6s4s: 1,6s5s: 1,6s6c: 1,6s6d: 1,6s6h: 1,7c5c: 1,7c6c: 1,7d5d: 1,7d6d: 1,7d7c: 1,7h5h: 1,7h6h: 1,7h7c: 1,7h7d: 1,7s5s: 1,7s6s: 1,7s7c: 1,7s7d: 1,7s7h: 1,8c5c: 0.835,8c6c: 1,8c7c: 1,8d5d: 0.835,8d6d: 1,8d7d: 1,8d8c: 1,8h5h: 0.835,8h6h: 1,8h7h: 1,8h8c: 1,8h8d: 1,8s5s: 0.835,8s6s: 1,8s7s: 1,8s8c: 1,8s8d: 1,8s8h: 1,9c6c: 1,9c7c: 1,9c8c: 1,9c8d: 0.685,9c8h: 0.685,9c8s: 0.685,9d6d: 1,9d7d: 1,9d8c: 0.685,9d8d: 1,9d8h: 0.685,9d8s: 0.685,9d9c: 1,9h6h: 1,9h7h: 1,9h8c: 0.685,9h8d: 0.685,9h8h: 1,9h8s: 0.685,9h9c: 1,9h9d: 1,9s6s: 1,9s7s: 1,9s8c: 0.685,9s8d: 0.685,9s8h: 0.685,9s8s: 1,9s9c: 1,9s9d: 1,9s9h: 1,Tc5c: 0.305,Tc6c: 1,Tc7c: 1,Tc8c: 1,Tc8d: 0.955,Tc8h: 0.955,Tc8s: 0.955,Tc9c: 1,Tc9d: 1,Tc9h: 1,Tc9s: 1,Td5d: 0.305,Td6d: 1,Td7d: 1,Td8c: 0.955,Td8d: 1,Td8h: 0.955,Td8s: 0.955,Td9c: 1,Td9d: 1,Td9h: 1,Td9s: 1,TdTc: 1,Th5h: 0.305,Th6h: 1,Th7h: 1,Th8c: 0.955,Th8d: 0.955,Th8h: 1,Th8s: 0.955,Th9c: 1,Th9d: 1,Th9h: 1,Th9s: 1,ThTc: 1,ThTd: 1,Ts5s: 0.305,Ts6s: 1,Ts7s: 1,Ts8c: 0.955,Ts8d: 0.955,Ts8h: 0.955,Ts8s: 1,Ts9c: 1,Ts9d: 1,Ts9h: 1,Ts9s: 1,TsTc: 1,TsTd: 1,TsTh: 1,Jc3c: 0.315,Jc4c: 1,Jc5c: 1,Jc6c: 1,Jc7c: 1,Jc8c: 1,Jc8d: 0.34,Jc8h: 0.34,Jc8s: 0.34,Jc9c: 1,Jc9d: 1,Jc9h: 1,Jc9s: 1,JcTc: 1,JcTd: 1,JcTh: 1,JcTs: 1,Jd3d: 0.315,Jd4d: 1,Jd5d: 1,Jd6d: 1,Jd7d: 1,Jd8c: 0.34,Jd8d: 1,Jd8h: 0.34,Jd8s: 0.34,Jd9c: 1,Jd9d: 1,Jd9h: 1,Jd9s: 1,JdTc: 1,JdTd: 1,JdTh: 1,JdTs: 1,JdJc: 1,Jh3h: 0.315,Jh4h: 1,Jh5h: 1,Jh6h: 1,Jh7h: 1,Jh8c: 0.34,Jh8d: 0.34,Jh8h: 1,Jh8s: 0.34,Jh9c: 1,Jh9d: 1,Jh9h: 1,Jh9s: 1,JhTc: 1,JhTd: 1,JhTh: 1,JhTs: 1,JhJc: 1,JhJd: 1,Js3s: 0.315,Js4s: 1,Js5s: 1,Js6s: 1,Js7s: 1,Js8c: 0.34,Js8d: 0.34,Js8h: 0.34,Js8s: 1,Js9c: 1,Js9d: 1,Js9h: 1,Js9s: 1,JsTc: 1,JsTd: 1,JsTh: 1,JsTs: 1,JsJc: 1,JsJd: 1,JsJh: 1,Qc2c: 1,Qc3c: 1,Qc4c: 1,Qc5c: 1,Qc6c: 1,Qc7c: 1,Qc8c: 1,Qc9c: 1,Qc9d: 1,Qc9h: 1,Qc9s: 1,QcTc: 1,QcTd: 1,QcTh: 1,QcTs: 1,QcJc: 1,QcJd: 1,QcJh: 1,QcJs: 1,Qd2d: 1,Qd3d: 1,Qd4d: 1,Qd5d: 1,Qd6d: 1,Qd7d: 1,Qd8d: 1,Qd9c: 1,Qd9d: 1,Qd9h: 1,Qd9s: 1,QdTc: 1,QdTd: 1,QdTh: 1,QdTs: 1,QdJc: 1,QdJd: 1,QdJh: 1,QdJs: 1,QdQc: 1,Qh2h: 1,Qh3h: 1,Qh4h: 1,Qh5h: 1,Qh6h: 1,Qh7h: 1,Qh8h: 1,Qh9c: 1,Qh9d: 1,Qh9h: 1,Qh9s: 1,QhTc: 1,QhTd: 1,QhTh: 1,QhTs: 1,QhJc: 1,QhJd: 1,QhJh: 1,QhJs: 1,QhQc: 1,QhQd: 1,Qs2s: 1,Qs3s: 1,Qs4s: 1,Qs5s: 1,Qs6s: 1,Qs7s: 1,Qs8s: 1,Qs9c: 1,Qs9d: 1,Qs9h: 1,Qs9s: 1,QsTc: 1,QsTd: 1,QsTh: 1,QsTs: 1,QsJc: 1,QsJd: 1,QsJh: 1,QsJs: 1,QsQc: 1,QsQd: 1,QsQh: 1,Kc2c: 1,Kc3c: 1,Kc4c: 1,Kc5c: 1,Kc6c: 1,Kc7c: 1,Kc7d: 0.275,Kc7h: 0.275,Kc7s: 0.275,Kc8c: 1,Kc8d: 0.685,Kc8h: 0.685,Kc8s: 0.685,Kc9c: 1,Kc9d: 1,Kc9h: 1,Kc9s: 1,KcTc: 1,KcTd: 1,KcTh: 1,KcTs: 1,KcJc: 1,KcJd: 1,KcJh: 1,KcJs: 1,KcQc: 1,KcQd: 1,KcQh: 1,KcQs: 1,Kd2d: 1,Kd3d: 1,Kd4d: 1,Kd5d: 1,Kd6d: 1,Kd7c: 0.275,Kd7d: 1,Kd7h: 0.275,Kd7s: 0.275,Kd8c: 0.685,Kd8d: 1,Kd8h: 0.685,Kd8s: 0.685,Kd9c: 1,Kd9d: 1,Kd9h: 1,Kd9s: 1,KdTc: 1,KdTd: 1,KdTh: 1,KdTs: 1,KdJc: 1,KdJd: 1,KdJh: 1,KdJs: 1,KdQc: 1,KdQd: 1,KdQh: 1,KdQs: 1,KdKc: 1,Kh2h: 1,Kh3h: 1,Kh4h: 1,Kh5h: 1,Kh6h: 1,Kh7c: 0.275,Kh7d: 0.275,Kh7h: 1,Kh7s: 0.275,Kh8c: 0.685,Kh8d: 0.685,Kh8h: 1,Kh8s: 0.685,Kh9c: 1,Kh9d: 1,Kh9h: 1,Kh9s: 1,KhTc: 1,KhTd: 1,KhTh: 1,KhTs: 1,KhJc: 1,KhJd: 1,KhJh: 1,KhJs: 1,KhQc: 1,KhQd: 1,KhQh: 1,KhQs: 1,KhKc: 1,KhKd: 1,Ks2s: 1,Ks3s: 1,Ks4s: 1,Ks5s: 1,Ks6s: 1,Ks7c: 0.275,Ks7d: 0.275,Ks7h: 0.275,Ks7s: 1,Ks8c: 0.685,Ks8d: 0.685,Ks8h: 0.685,Ks8s: 1,Ks9c: 1,Ks9d: 1,Ks9h: 1,Ks9s: 1,KsTc: 1,KsTd: 1,KsTh: 1,KsTs: 1,KsJc: 1,KsJd: 1,KsJh: 1,KsJs: 1,KsQc: 1,KsQd: 1,KsQh: 1,KsQs: 1,KsKc: 1,KsKd: 1,KsKh: 1,Ac2c: 1,Ac3c: 1,Ac3d: 0.155,Ac3h: 0.155,Ac3s: 0.155,Ac4c: 1,Ac4d: 1,Ac4h: 1,Ac4s: 1,Ac5c: 1,Ac5d: 1,Ac5h: 1,Ac5s: 1,Ac6c: 1,Ac6d: 1,Ac6h: 1,Ac6s: 1,Ac7c: 1,Ac7d: 1,Ac7h: 1,Ac7s: 1,Ac8c: 1,Ac8d: 1,Ac8h: 1,Ac8s: 1,Ac9c: 1,Ac9d: 1,Ac9h: 1,Ac9s: 1,AcTc: 1,AcTd: 1,AcTh: 1,AcTs: 1,AcJc: 1,AcJd: 1,AcJh: 1,AcJs: 1,AcQc: 1,AcQd: 1,AcQh: 1,AcQs: 1,AcKc: 1,AcKd: 1,AcKh: 1,AcKs: 1,Ad2d: 1,Ad3c: 0.155,Ad3d: 1,Ad3h: 0.155,Ad3s: 0.155,Ad4c: 1,Ad4d: 1,Ad4h: 1,Ad4s: 1,Ad5c: 1,Ad5d: 1,Ad5h: 1,Ad5s: 1,Ad6c: 1,Ad6d: 1,Ad6h: 1,Ad6s: 1,Ad7c: 1,Ad7d: 1,Ad7h: 1,Ad7s: 1,Ad8c: 1,Ad8d: 1,Ad8h: 1,Ad8s: 1,Ad9c: 1,Ad9d: 1,Ad9h: 1,Ad9s: 1,AdTc: 1,AdTd: 1,AdTh: 1,AdTs: 1,AdJc: 1,AdJd: 1,AdJh: 1,AdJs: 1,AdQc: 1,AdQd: 1,AdQh: 1,AdQs: 1,AdKc: 1,AdKd: 1,AdKh: 1,AdKs: 1,AdAc: 1,Ah2h: 1,Ah3c: 0.155,Ah3d: 0.155,Ah3h: 1,Ah3s: 0.155,Ah4c: 1,Ah4d: 1,Ah4h: 1,Ah4s: 1,Ah5c: 1,Ah5d: 1,Ah5h: 1,Ah5s: 1,Ah6c: 1,Ah6d: 1,Ah6h: 1,Ah6s: 1,Ah7c: 1,Ah7d: 1,Ah7h: 1,Ah7s: 1,Ah8c: 1,Ah8d: 1,Ah8h: 1,Ah8s: 1,Ah9c: 1,Ah9d: 1,Ah9h: 1,Ah9s: 1,AhTc: 1,AhTd: 1,AhTh: 1,AhTs: 1,AhJc: 1,AhJd: 1,AhJh: 1,AhJs: 1,AhQc: 1,AhQd: 1,AhQh: 1,AhQs: 1,AhKc: 1,AhKd: 1,AhKh: 1,AhKs: 1,AhAc: 1,AhAd: 1,As2s: 1,As3c: 0.155,As3d: 0.155,As3h: 0.155,As3s: 1,As4c: 1,As4d: 1,As4h: 1,As4s: 1,As5c: 1,As5d: 1,As5h: 1,As5s: 1,As6c: 1,As6d: 1,As6h: 1,As6s: 1,As7c: 1,As7d: 1,As7h: 1,As7s: 1,As8c: 1,As8d: 1,As8h: 1,As8s: 1,As9c: 1,As9d: 1,As9h: 1,As9s: 1,AsTc: 1,AsTd: 1,AsTh: 1,AsTs: 1,AsJc: 1,AsJd: 1,AsJh: 1,AsJs: 1,AsQc: 1,AsQd: 1,AsQh: 1,AsQs: 1,AsKc: 1,AsKd: 1,AsKh: 1,AsKs: 1,AsAc: 1,AsAd: 1,AsAh: 1").unwrap().to_string(),
            "22+,A2s+,A4o+,A3o:0.155,K2s+,K9o+,K8o:0.685,K7o:0.275,Q2s+,Q9o+,J4s+,J3s:0.315,J9o+,J8o:0.34,T6s+,T5s:0.305,T9o,T8o:0.955,96s+,98o:0.685,86s+,85s:0.835,75s+,64s+,54s,53s:0.095",
        );
        }

        #[test]
        fn it_formats_empty_hand_range() {
            assert_eq!(
                HandRange::from_iter(std::iter::empty::<CardPair>()).to_string(),
                ""
            );
        }
    }

    mod from_str {
        use super::*;

        #[test]
        fn it_parses_str_complex_hand_range() {
            assert_eq!(
                HandRange::from_str("88-66,JJ+,44,AQs-A9s,98o-96o,K8s+,ATo+,44,JTs,72o,AsKs,7d6h")
                    .unwrap(),
                HandRange::from_iter(
                    std::iter::empty()
                        .chain(
                            RankRange::inclusive(Rank::Eight, Rank::Six)
                                .into_iter()
                                .flat_map(|r| RankPair::Pocket(r))
                        )
                        .chain(
                            RankRange::inclusive(Rank::Ace, Rank::Jack)
                                .into_iter()
                                .flat_map(|r| RankPair::Pocket(r))
                        )
                        .chain(
                            RankRange::inclusive(Rank::Queen, Rank::Nine)
                                .into_iter()
                                .flat_map(|r| RankPair::Suited(Rank::Ace, r))
                        )
                        .chain(
                            RankRange::inclusive(Rank::Eight, Rank::Six)
                                .into_iter()
                                .flat_map(|r| RankPair::Ofsuit(Rank::Nine, r))
                        )
                        .chain(
                            RankRange::inclusive(Rank::Queen, Rank::Eight)
                                .into_iter()
                                .flat_map(|r| RankPair::Suited(Rank::King, r))
                        )
                        .chain(
                            RankRange::inclusive(Rank::King, Rank::Ten)
                                .into_iter()
                                .flat_map(|r| RankPair::Ofsuit(Rank::Ace, r))
                        )
                        .chain(RankPair::Pocket(Rank::Four))
                        .chain(RankPair::Suited(Rank::Jack, Rank::Ten))
                        .chain(RankPair::Ofsuit(Rank::Seven, Rank::Deuce))
                        .chain([CardPair::new(
                            Card::new(Rank::Ace, Suit::Spade),
                            Card::new(Rank::King, Suit::Spade),
                        )])
                        .chain([CardPair::new(
                            Card::new(Rank::Seven, Suit::Diamond),
                            Card::new(Rank::Six, Suit::Heart),
                        )])
                )
            );
        }

        #[test]
        fn it_parses_str_complex_hand_range_with_prob() {
            assert_eq!(
                HandRange::from_str("88-66:0.66,JJ+:0.5,44,AQs-A9s:0.2,98o-96o:0.999,K8s+:0.80,ATo+:1,44:0.44,JTs:0.25,72o:0.27,AsKs:0.4,7d6h:0.67")
                    .unwrap(),
                HandRange::from_iter(
                    std::iter::empty()
                        .chain(
                            RankRange::inclusive(Rank::Eight, Rank::Six)
                                .into_iter()
                                .flat_map(|r| RankPair::Pocket(r).into_iter().map(|cs| (cs, 0.66)))
                        )
                        .chain(
                            RankRange::inclusive(Rank::Eight, Rank::Six)
                                .into_iter()
                                .flat_map(|r| RankPair::Pocket(r).into_iter().map(|cs| (cs, 0.66)))
                        )
                        .chain(
                            RankRange::inclusive(Rank::Ace, Rank::Jack)
                                .into_iter()
                                .flat_map(|r| RankPair::Pocket(r).into_iter().map(|cs| (cs, 0.5))
                        )
                        .chain(
                            RankRange::inclusive(Rank::Queen, Rank::Nine)
                                .into_iter()
                                .flat_map(|r| RankPair::Suited(Rank::Ace, r).into_iter().map(|cs| (cs, 0.2)))
                        )
                        .chain(
                            RankRange::inclusive(Rank::Eight, Rank::Six)
                                .into_iter()
                                .flat_map(|r| RankPair::Ofsuit(Rank::Nine, r).into_iter().map(|cs| (cs, 0.999)))
                        )
                        .chain(
                            RankRange::inclusive(Rank::Queen, Rank::Eight)
                                .into_iter()
                                .flat_map(|r| RankPair::Suited(Rank::King, r).into_iter().map(|cs|(cs, 0.8)))
                        )
                        .chain(
                            RankRange::inclusive(Rank::King, Rank::Ten)
                                .into_iter()
                                .flat_map(|r| RankPair::Ofsuit(Rank::Ace, r).into_iter().map(|cs| (cs, 1_f32)))
                        )
                        .chain(
                            RankPair::Pocket(Rank::Four).into_iter().map(|cs| (cs, 0.44))
                        )
                        .chain(
                            RankPair::Suited(Rank::Jack, Rank::Ten).into_iter().map(|cs| (cs, 0.25))
                        )
                        .chain(
                            RankPair::Ofsuit(Rank::Seven, Rank::Deuce).into_iter().map(|cs| (cs, 0.27))
                        )
                        .chain([(CardPair::new(Card::new(Rank::Ace,Suit::Spade),Card::new(Rank::King,Suit::Spade)), 0.4)])
                        .chain([(CardPair::new(Card::new(Rank::Seven,Suit::Diamond),Card::new(Rank::Six,Suit::Heart)), 0.67)])
                    )
                )
            );
        }

        #[test]
        fn it_parses_empty_str() {
            assert_eq!(HandRange::from_str("").unwrap(), HandRange::empty());
        }
    }
}
