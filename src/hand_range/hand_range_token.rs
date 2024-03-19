use super::{CardPair, RankPair};
use crate::card::{Rank, RankRange};
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct HandRangeToken {
    kind: HandRangeTokenKind,
    probability: f32,
}

#[derive(Debug, PartialEq)]
pub enum HandRangeTokenKind {
    BottomClosedRankPairRange(RankPair),
    DoubleClosedRankPairRange(RankPair, Rank),
    SingleRankPair(RankPair),
    SingleCardPair(CardPair),
}

impl HandRangeToken {
    pub fn new(kind: HandRangeTokenKind, probability: f32) -> HandRangeToken {
        HandRangeToken { kind, probability }
    }
}

impl Display for HandRangeToken {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        let res = match self.kind {
            HandRangeTokenKind::BottomClosedRankPairRange(rank_pair) => {
                write!(f, "{}+", rank_pair)
            }
            HandRangeTokenKind::DoubleClosedRankPairRange(start, end) => match start {
                RankPair::Pocket(_) => write!(f, "{}-{}", start, RankPair::Pocket(end)),
                RankPair::Suited(high, kicker) => write!(f, "{}{}s-{}{}s", high, kicker, high, end),
                RankPair::Ofsuit(high, kicker) => write!(f, "{}{}o-{}{}o", high, kicker, high, end),
            },
            HandRangeTokenKind::SingleRankPair(rank_pair) => rank_pair.fmt(f),
            HandRangeTokenKind::SingleCardPair(card_pair) => card_pair.fmt(f),
        };

        if self.probability == 1.0 {
            res
        } else {
            res.and(write!(f, ":{}", self.probability))
        }
    }
}

impl IntoIterator for HandRangeToken {
    type Item = (CardPair, f32);

    type IntoIter = std::vec::IntoIter<(CardPair, f32)>;

    fn into_iter(self) -> Self::IntoIter {
        match self.kind {
            HandRangeTokenKind::BottomClosedRankPairRange(rank_pair) => match rank_pair {
                RankPair::Pocket(rank) => RankRange::inclusive(Rank::Ace, rank)
                    .into_iter()
                    .flat_map(|r| {
                        RankPair::Pocket(r)
                            .into_iter()
                            .map(|cp| (cp, self.probability))
                    })
                    .collect::<Vec<(CardPair, f32)>>()
                    .into_iter(),
                RankPair::Suited(high, kicker) => {
                    RankRange::inclusive(high.next().unwrap(), kicker)
                        .into_iter()
                        .flat_map(|r| {
                            RankPair::Suited(high, r)
                                .into_iter()
                                .map(|cp| (cp, self.probability))
                        })
                        .collect::<Vec<(CardPair, f32)>>()
                        .into_iter()
                }
                RankPair::Ofsuit(high, kicker) => {
                    RankRange::inclusive(high.next().unwrap(), kicker)
                        .into_iter()
                        .flat_map(|r| {
                            RankPair::Ofsuit(high, r)
                                .into_iter()
                                .map(|cp| (cp, self.probability))
                        })
                        .collect::<Vec<(CardPair, f32)>>()
                        .into_iter()
                }
            },
            HandRangeTokenKind::DoubleClosedRankPairRange(rank_pair, end) => match rank_pair {
                RankPair::Pocket(rank) => RankRange::inclusive(rank, end)
                    .into_iter()
                    .flat_map(|r| {
                        RankPair::Pocket(r)
                            .into_iter()
                            .map(|cp| (cp, self.probability))
                    })
                    .collect::<Vec<(CardPair, f32)>>()
                    .into_iter(),
                RankPair::Suited(high, kicker) => RankRange::inclusive(kicker, end)
                    .into_iter()
                    .flat_map(|r| {
                        RankPair::Suited(high, r)
                            .into_iter()
                            .map(|cp| (cp, self.probability))
                    })
                    .collect::<Vec<(CardPair, f32)>>()
                    .into_iter(),
                RankPair::Ofsuit(high, kicker) => RankRange::inclusive(kicker, end)
                    .into_iter()
                    .flat_map(|r| {
                        RankPair::Ofsuit(high, r)
                            .into_iter()
                            .map(|cp| (cp, self.probability))
                    })
                    .collect::<Vec<(CardPair, f32)>>()
                    .into_iter(),
            },
            HandRangeTokenKind::SingleRankPair(rank_pair) => rank_pair
                .into_iter()
                .map(|cp| (cp, self.probability))
                .collect::<Vec<(CardPair, f32)>>()
                .into_iter(),
            HandRangeTokenKind::SingleCardPair(card_pair) => {
                core::iter::once((card_pair, self.probability))
                    .collect::<Vec<(CardPair, f32)>>()
                    .into_iter()
            }
        }
    }
}

impl FromStr for HandRangeToken {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let double_closed_pocket_pair_range_regex =
            Regex::new(r"^[AKQJT98765432]{2}-[AKQJT98765432]{2}(:[01](\.[0-9]+)?)?$").unwrap();
        let double_rank_pair_range_regex =
            Regex::new(r"^[AKQJT98765432]{2}[so]-[AKQJT98765432]{2}[so](:[01](\.[0-9]+)?)?$")
                .unwrap();
        let bottom_closed_pocket_pair_range_regex =
            Regex::new(r"^[AKQJT98765432]{2}\+(:[01](\.[0-9]+)?)?$").unwrap();
        let bottom_closed_rank_pair_range_regex =
            Regex::new(r"^[AKQJT98765432]{2}[so]\+(:[01](\.[0-9]+)?)?$").unwrap();
        let single_pocket_pair_regex =
            Regex::new(r"^[AKQJT98765432]{2}(:[01](\.[0-9]+)?)?$").unwrap();
        let single_rank_pair_regex =
            Regex::new(r"^[AKQJT98765432]{2}[so](:[01](\.[0-9]+)?)?$").unwrap();
        let single_card_pair_regex =
            Regex::new(r"^([AKQJT98765432][shdc]){2}(:[01](\.[0-9]+)?)?$").unwrap();

        if double_closed_pocket_pair_range_regex.is_match(s)
            && s[0..1] == s[1..2]
            && s[3..4] == s[4..5]
        {
            if let (Ok(top), Ok(bottom)) = (Rank::from_str(&s[0..1]), Rank::from_str(&s[3..4])) {
                return Ok(HandRangeToken::new(
                    HandRangeTokenKind::DoubleClosedRankPairRange(RankPair::Pocket(top), bottom),
                    parse_probability(&s[5..]),
                ));
            }
        }

        if double_rank_pair_range_regex.is_match(s)
            && s[0..1] == s[4..5]
            && s[1..2] != s[5..6]
            && s[2..3] == s[6..7]
        {
            if let (Ok(high), Ok(kicker_top), Ok(kicker_bottom)) = (
                Rank::from_str(&s[0..1]),
                Rank::from_str(&s[1..2]),
                Rank::from_str(&s[5..6]),
            ) {
                if high < kicker_top && kicker_top < kicker_bottom {
                    if &s[2..3] == "s" {
                        return Ok(HandRangeToken::new(
                            HandRangeTokenKind::DoubleClosedRankPairRange(
                                RankPair::Suited(high, kicker_top),
                                kicker_bottom,
                            ),
                            parse_probability(&s[7..]),
                        ));
                    }

                    return Ok(HandRangeToken::new(
                        HandRangeTokenKind::DoubleClosedRankPairRange(
                            RankPair::Ofsuit(high, kicker_top),
                            kicker_bottom,
                        ),
                        parse_probability(&s[7..]),
                    ));
                }
            }
        }

        if bottom_closed_pocket_pair_range_regex.is_match(s) && s[0..1] == s[1..2] {
            if let Ok(bottom) = Rank::from_str(&s[0..1]) {
                return Ok(HandRangeToken::new(
                    HandRangeTokenKind::BottomClosedRankPairRange(RankPair::Pocket(bottom)),
                    parse_probability(&s[3..]),
                ));
            }
        }

        if bottom_closed_rank_pair_range_regex.is_match(s) && s[0..1] != s[1..2] {
            if let (Ok(high), Ok(kicker_bottom)) =
                (Rank::from_str(&s[0..1]), Rank::from_str(&s[1..2]))
            {
                if &s[2..3] == "s" {
                    return Ok(HandRangeToken::new(
                        HandRangeTokenKind::BottomClosedRankPairRange(RankPair::Suited(
                            high,
                            kicker_bottom,
                        )),
                        parse_probability(&s[4..]),
                    ));
                }

                return Ok(HandRangeToken::new(
                    HandRangeTokenKind::BottomClosedRankPairRange(RankPair::Ofsuit(
                        high,
                        kicker_bottom,
                    )),
                    parse_probability(&s[4..]),
                ));
            }
        }

        if single_pocket_pair_regex.is_match(s) && s[0..1] == s[1..2] {
            if let Ok(rank) = Rank::from_str(&s[0..1]) {
                return Ok(HandRangeToken::new(
                    HandRangeTokenKind::SingleRankPair(RankPair::Pocket(rank)),
                    parse_probability(&s[2..]),
                ));
            }
        }

        if single_rank_pair_regex.is_match(s) && s[0..1] != s[1..2] {
            if let (Ok(high), Ok(kicker)) = (Rank::from_str(&s[0..1]), Rank::from_str(&s[1..2])) {
                if &s[2..3] == "s" {
                    return Ok(HandRangeToken::new(
                        HandRangeTokenKind::SingleRankPair(RankPair::Suited(high, kicker)),
                        parse_probability(&s[3..]),
                    ));
                }

                return Ok(HandRangeToken::new(
                    HandRangeTokenKind::SingleRankPair(RankPair::Ofsuit(high, kicker)),
                    parse_probability(&s[3..]),
                ));
            }
        }

        if single_card_pair_regex.is_match(s) {
            if let Ok(card_pair) = (&s[0..4]).parse::<CardPair>() {
                return Ok(HandRangeToken::new(
                    HandRangeTokenKind::SingleCardPair(card_pair),
                    parse_probability(&s[4..]),
                ));
            }
        }

        Err(())
    }
}

fn parse_probability(value: &str) -> f32 {
    let mut value = value;

    if value.len() >= 1 && value.starts_with(":") {
        value = &value[1..];
    }

    f32::from_str(value).unwrap_or(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod display {
        use super::*;

        #[test]
        fn it_formats_bottom_closed_pocket_pair_range_with_prob() {
            let token = HandRangeToken::new(
                HandRangeTokenKind::BottomClosedRankPairRange(RankPair::Pocket(Rank::Queen)),
                0.8,
            );

            assert_eq!(token.to_string(), "QQ+:0.8");
        }

        #[test]
        fn it_formats_bottom_closed_pocket_pair_range() {
            let token = HandRangeToken::new(
                HandRangeTokenKind::BottomClosedRankPairRange(RankPair::Pocket(Rank::Queen)),
                1.0,
            );

            assert_eq!(token.to_string(), "QQ+");
        }

        #[test]
        fn it_formats_bottom_closed_suited_rank_pair_range_with_prob() {
            let token = HandRangeToken::new(
                HandRangeTokenKind::BottomClosedRankPairRange(RankPair::Suited(
                    Rank::King,
                    Rank::Nine,
                )),
                0.8,
            );

            assert_eq!(token.to_string(), "K9s+:0.8");
        }

        #[test]
        fn it_formats_bottom_closed_suited_rank_pair_range() {
            let token = HandRangeToken::new(
                HandRangeTokenKind::BottomClosedRankPairRange(RankPair::Suited(
                    Rank::King,
                    Rank::Nine,
                )),
                1.0,
            );

            assert_eq!(token.to_string(), "K9s+");
        }

        #[test]
        fn it_formats_bottom_closed_ofsuit_rank_pair_range_with_prob() {
            let token = HandRangeToken::new(
                HandRangeTokenKind::BottomClosedRankPairRange(RankPair::Ofsuit(
                    Rank::Eight,
                    Rank::Four,
                )),
                0.8,
            );

            assert_eq!(token.to_string(), "84o+:0.8");
        }

        #[test]
        fn it_formats_bottom_closed_ofsuit_rank_pair_range() {
            let token = HandRangeToken::new(
                HandRangeTokenKind::BottomClosedRankPairRange(RankPair::Ofsuit(
                    Rank::Eight,
                    Rank::Four,
                )),
                1.0,
            );

            assert_eq!(token.to_string(), "84o+");
        }

        #[test]
        fn it_formats_double_closed_pocket_pair_range_with_prob() {
            let token = HandRangeToken::new(
                HandRangeTokenKind::DoubleClosedRankPairRange(
                    RankPair::Pocket(Rank::Jack),
                    Rank::Seven,
                ),
                0.8,
            );

            assert_eq!(token.to_string(), "JJ-77:0.8");
        }

        #[test]
        fn it_formats_double_closed_pocket_pair_range() {
            let token = HandRangeToken::new(
                HandRangeTokenKind::DoubleClosedRankPairRange(
                    RankPair::Pocket(Rank::Jack),
                    Rank::Seven,
                ),
                1.0,
            );

            assert_eq!(token.to_string(), "JJ-77");
        }

        #[test]
        fn it_formats_double_closed_suited_rank_pair_range_with_prob() {
            let token = HandRangeToken::new(
                HandRangeTokenKind::DoubleClosedRankPairRange(
                    RankPair::Suited(Rank::King, Rank::Nine),
                    Rank::Five,
                ),
                0.8,
            );

            assert_eq!(token.to_string(), "K9s-K5s:0.8");
        }

        #[test]
        fn it_formats_double_closed_suited_rank_pair_range() {
            let token = HandRangeToken::new(
                HandRangeTokenKind::DoubleClosedRankPairRange(
                    RankPair::Suited(Rank::King, Rank::Nine),
                    Rank::Five,
                ),
                1.0,
            );

            assert_eq!(token.to_string(), "K9s-K5s");
        }

        #[test]
        fn it_formats_double_closed_ofsuit_rank_pair_range_with_prob() {
            let token = HandRangeToken::new(
                HandRangeTokenKind::DoubleClosedRankPairRange(
                    RankPair::Ofsuit(Rank::Nine, Rank::Seven),
                    Rank::Four,
                ),
                0.8,
            );

            assert_eq!(token.to_string(), "97o-94o:0.8");
        }

        #[test]
        fn it_formats_double_closed_ofsuit_rank_pair_range() {
            let token = HandRangeToken::new(
                HandRangeTokenKind::DoubleClosedRankPairRange(
                    RankPair::Ofsuit(Rank::Nine, Rank::Seven),
                    Rank::Four,
                ),
                1.0,
            );

            assert_eq!(token.to_string(), "97o-94o");
        }
    }

    mod into_iter {
        use super::*;
        use crate::card::{Card, Suit};
        use insta::*;

        #[test]
        fn it_iterates_bottom_closed_pocket_pair_range() {
            let token = HandRangeToken::new(
                HandRangeTokenKind::BottomClosedRankPairRange(RankPair::Pocket(Rank::Queen)),
                0.9,
            );

            let tokens: Vec<(CardPair, f32)> = token.into_iter().collect();

            assert_debug_snapshot!(tokens);
        }

        #[test]
        fn it_iterates_bottom_closed_suited_pair_range() {
            let token = HandRangeToken::new(
                HandRangeTokenKind::BottomClosedRankPairRange(RankPair::Suited(
                    Rank::Jack,
                    Rank::Eight,
                )),
                0.7,
            );

            let tokens: Vec<(CardPair, f32)> = token.into_iter().collect();

            assert_debug_snapshot!(tokens);
        }

        #[test]
        fn it_iterates_bottom_closed_ofsuit_pair_range() {
            let token = HandRangeToken::new(
                HandRangeTokenKind::BottomClosedRankPairRange(RankPair::Ofsuit(
                    Rank::Ace,
                    Rank::Ten,
                )),
                0.5,
            );

            let tokens: Vec<(CardPair, f32)> = token.into_iter().collect();

            assert_debug_snapshot!(tokens);
        }

        #[test]
        fn it_iterates_double_closed_pocket_pair_range() {
            let token = HandRangeToken::new(
                HandRangeTokenKind::DoubleClosedRankPairRange(
                    RankPair::Pocket(Rank::Queen),
                    Rank::Nine,
                ),
                0.8,
            );

            let tokens: Vec<(CardPair, f32)> = token.into_iter().collect();

            assert_debug_snapshot!(tokens);
        }

        #[test]
        fn it_iterates_double_closed_suited_pair_range() {
            let token = HandRangeToken::new(
                HandRangeTokenKind::DoubleClosedRankPairRange(
                    RankPair::Suited(Rank::Jack, Rank::Nine),
                    Rank::Six,
                ),
                0.6,
            );

            let tokens: Vec<(CardPair, f32)> = token.into_iter().collect();

            assert_debug_snapshot!(tokens);
        }

        #[test]
        fn it_iterates_double_closed_ofsuit_pair_range() {
            let token = HandRangeToken::new(
                HandRangeTokenKind::DoubleClosedRankPairRange(
                    RankPair::Ofsuit(Rank::King, Rank::Nine),
                    Rank::Seven,
                ),
                0.6,
            );

            let tokens: Vec<(CardPair, f32)> = token.into_iter().collect();

            assert_debug_snapshot!(tokens);
        }

        #[test]
        fn it_iterates_single_pocket_pair_range() {
            let token = HandRangeToken::new(
                HandRangeTokenKind::SingleRankPair(RankPair::Pocket(Rank::Deuce)),
                0.3,
            );

            let tokens: Vec<(CardPair, f32)> = token.into_iter().collect();

            assert_debug_snapshot!(tokens);
        }

        #[test]
        fn it_iterates_single_suited_pair_range() {
            let token = HandRangeToken::new(
                HandRangeTokenKind::SingleRankPair(RankPair::Suited(Rank::King, Rank::Six)),
                0.1,
            );

            let tokens: Vec<(CardPair, f32)> = token.into_iter().collect();

            assert_debug_snapshot!(tokens);
        }

        #[test]
        fn it_iterates_single_ofsuit_pair_range() {
            let token = HandRangeToken::new(
                HandRangeTokenKind::SingleRankPair(RankPair::Ofsuit(Rank::Ace, Rank::Five)),
                0.4,
            );

            let tokens: Vec<(CardPair, f32)> = token.into_iter().collect();

            assert_debug_snapshot!(tokens);
        }

        #[test]
        fn it_iterates_single_card_pair_range() {
            let token = HandRangeToken::new(
                HandRangeTokenKind::SingleCardPair(CardPair::new(
                    Card::new(Rank::Five, Suit::Club),
                    Card::new(Rank::Deuce, Suit::Diamond),
                )),
                0.2,
            );

            let tokens: Vec<(CardPair, f32)> = token.into_iter().collect();

            assert_debug_snapshot!(tokens);
        }
    }

    mod from_str {
        use super::*;
        use crate::card::{Card, Suit};

        #[test]
        fn it_parses_str_pocket_eight_to_six() {
            assert_eq!(
                "88-66".parse::<HandRangeToken>().unwrap(),
                HandRangeToken::new(
                    HandRangeTokenKind::DoubleClosedRankPairRange(
                        RankPair::Pocket(Rank::Eight),
                        Rank::Six
                    ),
                    1.0
                )
            );
        }

        #[test]
        fn it_parses_str_pocket_eight_to_six_with_prob() {
            assert_eq!(
                "88-66:0.66".parse::<HandRangeToken>().unwrap(),
                HandRangeToken::new(
                    HandRangeTokenKind::DoubleClosedRankPairRange(
                        RankPair::Pocket(Rank::Eight),
                        Rank::Six
                    ),
                    0.66
                )
            );
        }

        #[test]
        fn it_parses_str_pocket_jack_plus() {
            assert_eq!(
                "JJ+".parse::<HandRangeToken>().unwrap(),
                HandRangeToken::new(
                    HandRangeTokenKind::BottomClosedRankPairRange(RankPair::Pocket(Rank::Jack)),
                    1.0
                )
            );
        }

        #[test]
        fn it_parses_str_pocket_jack_plus_with_prob() {
            assert_eq!(
                "JJ+:0.5".parse::<HandRangeToken>().unwrap(),
                HandRangeToken::new(
                    HandRangeTokenKind::BottomClosedRankPairRange(RankPair::Pocket(Rank::Jack)),
                    0.5
                )
            );
        }

        #[test]
        fn it_parses_str_suited_ace_queen_to_nine() {
            assert_eq!(
                "AQs-A9s".parse::<HandRangeToken>().unwrap(),
                HandRangeToken::new(
                    HandRangeTokenKind::DoubleClosedRankPairRange(
                        RankPair::Suited(Rank::Ace, Rank::Queen),
                        Rank::Nine
                    ),
                    1.0
                )
            );
        }

        #[test]
        fn it_parses_str_suited_ace_queen_to_nine_with_prob() {
            assert_eq!(
                "AQs-A9s:0.2".parse::<HandRangeToken>().unwrap(),
                HandRangeToken::new(
                    HandRangeTokenKind::DoubleClosedRankPairRange(
                        RankPair::Suited(Rank::Ace, Rank::Queen),
                        Rank::Nine
                    ),
                    0.2
                )
            );
        }

        #[test]
        fn it_parses_str_ofsuit_nine_eight_to_six() {
            assert_eq!(
                "98o-96o".parse::<HandRangeToken>().unwrap(),
                HandRangeToken::new(
                    HandRangeTokenKind::DoubleClosedRankPairRange(
                        RankPair::Ofsuit(Rank::Nine, Rank::Eight),
                        Rank::Six
                    ),
                    1.0
                )
            );
        }

        #[test]
        fn it_parses_str_ofsuit_nine_eight_to_six_with_prob() {
            assert_eq!(
                "98o-96o:0.999".parse::<HandRangeToken>().unwrap(),
                HandRangeToken::new(
                    HandRangeTokenKind::DoubleClosedRankPairRange(
                        RankPair::Ofsuit(Rank::Nine, Rank::Eight),
                        Rank::Six
                    ),
                    0.999
                )
            );
        }

        #[test]
        fn it_parses_str_suited_king_eight_plus() {
            assert_eq!(
                "K8s+".parse::<HandRangeToken>().unwrap(),
                HandRangeToken::new(
                    HandRangeTokenKind::BottomClosedRankPairRange(RankPair::Suited(
                        Rank::King,
                        Rank::Eight
                    )),
                    1.0
                )
            );
        }

        #[test]
        fn it_parses_str_suited_king_eight_plus_with_prob() {
            assert_eq!(
                "K8s+:0.80".parse::<HandRangeToken>().unwrap(),
                HandRangeToken::new(
                    HandRangeTokenKind::BottomClosedRankPairRange(RankPair::Suited(
                        Rank::King,
                        Rank::Eight
                    )),
                    0.80
                )
            );
        }

        #[test]
        fn it_parses_str_ofsuit_ace_ten_plus() {
            assert_eq!(
                "ATo+".parse::<HandRangeToken>().unwrap(),
                HandRangeToken::new(
                    HandRangeTokenKind::BottomClosedRankPairRange(RankPair::Ofsuit(
                        Rank::Ace,
                        Rank::Ten
                    )),
                    1.0
                )
            );
        }

        #[test]
        fn it_parses_str_ofsuit_ace_ten_plus_with_prob() {
            assert_eq!(
                "ATo+:1".parse::<HandRangeToken>().unwrap(),
                HandRangeToken::new(
                    HandRangeTokenKind::BottomClosedRankPairRange(RankPair::Ofsuit(
                        Rank::Ace,
                        Rank::Ten
                    )),
                    1.0
                )
            );
        }

        #[test]
        fn it_parses_str_pocket_four() {
            assert_eq!(
                "44".parse::<HandRangeToken>().unwrap(),
                HandRangeToken::new(
                    HandRangeTokenKind::SingleRankPair(RankPair::Pocket(Rank::Four)),
                    1.0
                )
            );
        }

        #[test]
        fn it_parses_str_pocket_four_with_prob() {
            assert_eq!(
                "44:0.44".parse::<HandRangeToken>().unwrap(),
                HandRangeToken::new(
                    HandRangeTokenKind::SingleRankPair(RankPair::Pocket(Rank::Four)),
                    0.44
                )
            );
        }

        #[test]
        fn it_parses_str_suited_jack_ten() {
            assert_eq!(
                "JTs".parse::<HandRangeToken>().unwrap(),
                HandRangeToken::new(
                    HandRangeTokenKind::SingleRankPair(RankPair::Suited(Rank::Jack, Rank::Ten)),
                    1.0
                )
            );
        }

        #[test]
        fn it_parses_str_suited_jack_ten_with_prob() {
            assert_eq!(
                "JTs:0.25".parse::<HandRangeToken>().unwrap(),
                HandRangeToken::new(
                    HandRangeTokenKind::SingleRankPair(RankPair::Suited(Rank::Jack, Rank::Ten)),
                    0.25
                )
            );
        }

        #[test]
        fn it_parses_str_ofsuite_seven_deuce() {
            assert_eq!(
                "72o".parse::<HandRangeToken>().unwrap(),
                HandRangeToken::new(
                    HandRangeTokenKind::SingleRankPair(RankPair::Ofsuit(Rank::Seven, Rank::Deuce)),
                    1.0
                )
            );
        }

        #[test]
        fn it_parses_str_ofsuite_seven_deuce_with_prob() {
            assert_eq!(
                "72o:0.27".parse::<HandRangeToken>().unwrap(),
                HandRangeToken::new(
                    HandRangeTokenKind::SingleRankPair(RankPair::Ofsuit(Rank::Seven, Rank::Deuce)),
                    0.27
                )
            );
        }

        #[test]
        fn it_parses_str_ace_spade_king_spade() {
            assert_eq!(
                "AsKs".parse::<HandRangeToken>().unwrap(),
                HandRangeToken::new(
                    HandRangeTokenKind::SingleCardPair(CardPair::new(
                        Card::new(Rank::Ace, Suit::Spade),
                        Card::new(Rank::King, Suit::Spade),
                    )),
                    1.0
                )
            );
        }

        #[test]
        fn it_parses_str_ace_spade_king_spade_with_prob() {
            assert_eq!(
                "AsKs:0.4".parse::<HandRangeToken>().unwrap(),
                HandRangeToken::new(
                    HandRangeTokenKind::SingleCardPair(CardPair::new(
                        Card::new(Rank::Ace, Suit::Spade),
                        Card::new(Rank::King, Suit::Spade),
                    )),
                    0.4
                )
            );
        }

        #[test]
        fn it_parses_str_seven_diamond_six_heart() {
            assert_eq!(
                "7d6h".parse::<HandRangeToken>().unwrap(),
                HandRangeToken::new(
                    HandRangeTokenKind::SingleCardPair(CardPair::new(
                        Card::new(Rank::Seven, Suit::Diamond),
                        Card::new(Rank::Six, Suit::Heart),
                    )),
                    1.0
                )
            );
        }

        #[test]
        fn it_parses_str_seven_diamond_six_heart_with_prob() {
            assert_eq!(
                "7d6h:0.67".parse::<HandRangeToken>().unwrap(),
                HandRangeToken::new(
                    HandRangeTokenKind::SingleCardPair(CardPair::new(
                        Card::new(Rank::Seven, Suit::Diamond),
                        Card::new(Rank::Six, Suit::Heart),
                    )),
                    0.67
                )
            );
        }

        #[test]
        fn it_fails_parsing_qwe() {
            assert!("qwe".parse::<HandRangeToken>().is_err());
        }

        #[test]
        fn it_fails_parsing_akto_plus() {
            assert!("AKTo+".parse::<HandRangeToken>().is_err());
        }

        #[test]
        fn it_fails_parsing_jj_plus_plus() {
            assert!("JJ++".parse::<HandRangeToken>().is_err());
        }
    }
}
