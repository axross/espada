use crate::card_set::CardSet;
use crate::rank::Rank;
use crate::suit::Suit;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum MadeHandType {
    HighCard,
    Pair,
    TwoPair,
    Trips,
    Straight,
    Flush,
    FullHouse,
    Quads,
    StraightFlush,
}

#[derive(Debug, PartialEq, Eq, Ord, Copy, Clone)]
pub struct MadeHand(u16);

impl MadeHand {
    pub fn power_index(&self) -> u16 {
        self.0
    }

    pub fn hand_type(&self) -> MadeHandType {
        match self.0 {
            0..=9 => MadeHandType::StraightFlush,
            10..=165 => MadeHandType::Quads,
            166..=321 => MadeHandType::FullHouse,
            322..=1598 => MadeHandType::Flush,
            1599..=1608 => MadeHandType::Straight,
            1609..=2466 => MadeHandType::Trips,
            2467..=3324 => MadeHandType::TwoPair,
            3325..=6184 => MadeHandType::Pair,
            _ => MadeHandType::HighCard,
        }
    }
}

impl PartialOrd for MadeHand {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl From<&CardSet> for MadeHand {
    fn from(cards: &CardSet) -> Self {
        debug_assert!(cards.len() == 7);

        let flash_suit = find_flush_suit(cards);

        match flash_suit {
            Some(suit) => MadeHand(AS_FLUSH[hash_for_flush(cards, &suit) as usize]),
            _ => MadeHand(AS_RAINBOW[hash_for_rainbow(cards) as usize]),
        }
    }
}

impl From<CardSet> for MadeHand {
    fn from(cards: CardSet) -> Self {
        (&cards).into()
    }
}

#[cfg(test)]
mod tests_from_card_set {
    use super::*;

    use core::str::FromStr;

    #[test]
    fn it_creates_made_hand_with_correct_power_index() {
        assert_eq!(
            MadeHand::from(CardSet::from_str("4c8hKhQc4s6hJd").unwrap()).power_index(),
            5581
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2d5sJc6s3s3dQh").unwrap()).power_index(),
            5850
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7h9c5h5d4d7s4s").unwrap()).power_index(),
            3177
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5h7d7cTd8dAd2c").unwrap()).power_index(),
            4894
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5d2h7c3d5h7sKh").unwrap()).power_index(),
            3173
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Js8s5s4sAs7dQs").unwrap()).power_index(),
            504
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9hTc3sAd2s6cAh").unwrap()).power_index(),
            3464
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6h4sKh7h8d3hAd").unwrap()).power_index(),
            6315
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Td9s5cTsJc6dAh").unwrap()).power_index(),
            4225
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6s2d8c6d6h4dTh").unwrap()).power_index(),
            2177
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AsJsJh5s9d7d9h").unwrap()).power_index(),
            2842
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jd4d5d5s9d7h4c").unwrap()).power_index(),
            3263
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KcKs2h6hAc9c7d").unwrap()).power_index(),
            3574
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2s8c3sKdQs8d7d").unwrap()).power_index(),
            4704
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7c9s6c2hAdQd3c").unwrap()).power_index(),
            6420
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8d7s5cAcKs7h9d").unwrap()).power_index(),
            4869
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8c8s3s9c8h3c9h").unwrap()).power_index(),
            244
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2hThJh8sTs7dJd").unwrap()).power_index(),
            2835
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AsKcAhJsThAc4d").unwrap()).power_index(),
            1611
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4sQh3h7s9s7hQd").unwrap()).power_index(),
            2769
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4h5d6dAh3d5sTh").unwrap()).power_index(),
            5336
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4dTsQd8c6s3c7s").unwrap()).power_index(),
            7112
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4d3s2c7d7s2d2s").unwrap()).power_index(),
            318
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8cAsQsKd2s7hTh").unwrap()).power_index(),
            6195
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7s6hKc2hQsTsJc").unwrap()).power_index(),
            6680
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ah7hJs5h4cTs6d").unwrap()).power_index(),
            6483
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kc4h6hQs3h7cTh").unwrap()).power_index(),
            6727
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("ThKs9d4s2s8d3s").unwrap()).power_index(),
            6885
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kd3hTh4c5h8h2s").unwrap()).power_index(),
            6912
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3s6s8h9cJc2dQs").unwrap()).power_index(),
            7036
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4d2c3cAd3h4s8h").unwrap()).power_index(),
            3293
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8d2sTs5c8s9cAc").unwrap()).power_index(),
            4673
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9c6d4cKs7c6sQs").unwrap()).power_index(),
            5143
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4c3hKc5cQdQcTd").unwrap()).power_index(),
            3834
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3d9h6dTc7s9c5s").unwrap()).power_index(),
            4596
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7c4cTcKs3d2s6c").unwrap()).power_index(),
            6919
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6cJhTsQdTc2sJd").unwrap()).power_index(),
            2833
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qd4cKh2h2s3cAh").unwrap()).power_index(),
            5966
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6d2dKd3h8sAs9c").unwrap()).power_index(),
            6295
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4s3hQc2c5hKhTs").unwrap()).power_index(),
            6736
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4s6hQh4d2s7hQc").unwrap()).power_index(),
            2804
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3d6d3h3s4sKh9h").unwrap()).power_index(),
            2350
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KsQdKc4d5d3cTs").unwrap()).power_index(),
            3614
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jh6s5s5d4sQh2h").unwrap()).power_index(),
            5410
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Th5d5c9c2sJsQs").unwrap()).power_index(),
            5406
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Th9h6cKs5hKh4s").unwrap()).power_index(),
            3684
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7dJs6h5d2s2dTd").unwrap()).power_index(),
            6104
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("As9d8c6dQd6sJh").unwrap()).power_index(),
            5096
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jd4d2c6sAd4s5c").unwrap()).power_index(),
            5549
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2d7c6sKd6cKsKh").unwrap()).power_index(),
            186
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7dKdJc4cTcKsAs").unwrap()).power_index(),
            3556
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qd2d5cJd7c6cAc").unwrap()).power_index(),
            6371
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("JhAd3d3cTs4d5h").unwrap()).power_index(),
            5765
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8dKd7cKsTc6sQd").unwrap()).power_index(),
            3611
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6h3s7d8h7c9h3d").unwrap()).power_index(),
            3199
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Tc2s2c4c8d9h6d").unwrap()).power_index(),
            6130
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5hJd4s2h3d7d9h").unwrap()).power_index(),
            7291
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7c5sTsTcQcAcJd").unwrap()).power_index(),
            4216
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jd7dAcQd7c2cJs").unwrap()).power_index(),
            2864
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4cKc6h9cAc9sAd").unwrap()).power_index(),
            2512
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KsAdJc7dQs2h7c").unwrap()).power_index(),
            4866
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2dJc8h9cJd7c9d").unwrap()).power_index(),
            2846
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2hAc8cJdTd5d6h").unwrap()).power_index(),
            6478
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qh6s5c8d7d5s9c").unwrap()).power_index(),
            1605
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8dKdAh2cQd7cQh").unwrap()).power_index(),
            3769
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2h3h8dJc7h3dQs").unwrap()).power_index(),
            5848
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9s8cKs5c2d9c3s").unwrap()).power_index(),
            4507
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5sTd5c6hQs8d3h").unwrap()).power_index(),
            5415
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3h6hTsAdJc3d4d").unwrap()).power_index(),
            5765
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("ThAs2dKs3s8hAd").unwrap()).power_index(),
            3346
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6h8hTs8c6c4h2s").unwrap()).power_index(),
            3110
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3hTsJcQhQd2s9h").unwrap()).power_index(),
            3866
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("JcAs6c8c9d5h2s").unwrap()).power_index(),
            6499
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AcTsQs9sJs4d3h").unwrap()).power_index(),
            6350
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5dKh9sQs2c7hAs").unwrap()).power_index(),
            6203
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Js2cKs5sAhTsQh").unwrap()).power_index(),
            1600
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2c4hKs6c8s6s5d").unwrap()).power_index(),
            5172
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2d3sThAsJhJd8d").unwrap()).power_index(),
            4006
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("TsJs9c8d9h4cJh").unwrap()).power_index(),
            2845
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2cAhAd8h6c4s4h").unwrap()).power_index(),
            2572
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qc8d7cQdKh4s3c").unwrap()).power_index(),
            3845
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jd8sKhTh4hJsQs").unwrap()).power_index(),
            4041
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QcTc6hQd6c8s7h").unwrap()).power_index(),
            2779
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qd3dKdJd8h6sJc").unwrap()).power_index(),
            4043
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5cTs4s9s4d7sAc").unwrap()).power_index(),
            5553
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qd5s2s4sAdTh9h").unwrap()).power_index(),
            6389
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5c2h9dQc2s3d4h").unwrap()).power_index(),
            6084
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6dKh7hTd3s7d2s").unwrap()).power_index(),
            4940
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9h2dKhTd9c5h4h").unwrap()).power_index(),
            4501
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5s8d2d7cKh3h5d").unwrap()).power_index(),
            5391
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8hTh9s7cQc4sTd").unwrap()).power_index(),
            4314
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("JdKdJh8c5h8h3c").unwrap()).power_index(),
            2854
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4s3s4c6hAhAdQc").unwrap()).power_index(),
            2568
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jh7d6cAsAc3dKs").unwrap()).power_index(),
            3339
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3sQc8h3hTh9sAc").unwrap()).power_index(),
            5757
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7d4h4s7h9s8d6s").unwrap()).power_index(),
            3188
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5s4s7sKc3dQs9c").unwrap()).power_index(),
            6749
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4s6c9d8cQcAc4h").unwrap()).power_index(),
            5538
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6hJcJd9d3c9h2c").unwrap()).power_index(),
            2848
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6c7dQd2cAsTdJc").unwrap()).power_index(),
            6352
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8s6d2c8c7c2sKd").unwrap()).power_index(),
            3151
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("JdTcTsJh5dAs7c").unwrap()).power_index(),
            2831
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Th4h3c9d9s4dAc").unwrap()).power_index(),
            3062
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4hAhTs9h8d9c2c").unwrap()).power_index(),
            4453
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3sAc3dQcTdKs4c").unwrap()).power_index(),
            5746
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2c2sQcQd4hAs9s").unwrap()).power_index(),
            2820
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9d8d3cAdTdJhAs").unwrap()).power_index(),
            3426
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3d4sTc4d2hAh3h").unwrap()).power_index(),
            3293
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ks9d6hJs4h8sQh").unwrap()).power_index(),
            6686
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2cAdKh6d8hAh4s").unwrap()).power_index(),
            3361
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("JhJd7s8s8c2s4s").unwrap()).power_index(),
            2858
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Js9d5dAh7dQhKc").unwrap()).power_index(),
            6186
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QcJh6sQs8hTs4c").unwrap()).power_index(),
            3867
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KsAhJdKcKhJc7h").unwrap()).power_index(),
            181
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9d6c2cTdJc5h5d").unwrap()).power_index(),
            5442
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4s9c5s6d2dTc8d").unwrap()).power_index(),
            7346
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7cKh4sAd6hTs8d").unwrap()).power_index(),
            6273
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("TdTsJdTc6hKc4s").unwrap()).power_index(),
            1886
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7s4h2d5sKs5h4d").unwrap()).power_index(),
            3261
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6s8c3d8dKcQhAh").unwrap()).power_index(),
            4646
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7s9dKhKd6hThQd").unwrap()).power_index(),
            3610
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7h2c5h4s3sAd6c").unwrap()).power_index(),
            1607
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8c2h3cKc9h7c3h").unwrap()).power_index(),
            5825
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kh5s6s2h7hTs9c").unwrap()).power_index(),
            6888
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qs9d9hAh4h4c3s").unwrap()).power_index(),
            3062
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9c9hJs5s3c6sJd").unwrap()).power_index(),
            2848
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kc8cJhTd7c9d2c").unwrap()).power_index(),
            1603
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9d2d6s9hTs3sTd").unwrap()).power_index(),
            2936
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3hTsAh5c8h4d6c").unwrap()).power_index(),
            6580
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4d3sQh2s3c7c9s").unwrap()).power_index(),
            5862
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5c4sTd6dTc2sJd").unwrap()).power_index(),
            4360
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6d6h8cTs8s5s2h").unwrap()).power_index(),
            3110
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3dTdJsQhJhQs4h").unwrap()).power_index(),
            2723
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6dKh5d6cKc2c2s").unwrap()).power_index(),
            2673
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9c4s6d6s7h3s8h").unwrap()).power_index(),
            5271
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2dTd5s8dKhAdKd").unwrap()).power_index(),
            415
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("JsAhJd4c7hTcKc").unwrap()).power_index(),
            3987
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8cQh5hJs8sJc2d").unwrap()).power_index(),
            2855
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6dJd7d8s2hTc4d").unwrap()).power_index(),
            7237
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ad6c7sQs4d9d2s").unwrap()).power_index(),
            6420
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QsQc3hJd3c9dQh").unwrap()).power_index(),
            201
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qc9h6d4d3dKc5c").unwrap()).power_index(),
            6753
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Td8cJd7c9s9h8s").unwrap()).power_index(),
            1603
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9s4cAcThJc7cAd").unwrap()).power_index(),
            3426
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7sAh8sKd3c6s2s").unwrap()).power_index(),
            6315
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3dAs3cAh4c9dQh").unwrap()).power_index(),
            2579
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5h4h8s2d7cQcAs").unwrap()).power_index(),
            6436
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3h9h5d6d6cQh4h").unwrap()).power_index(),
            5203
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3h9h7s9dQc8d2s").unwrap()).power_index(),
            4541
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6sThAc5c6dJd4h").unwrap()).power_index(),
            5105
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jc5d5c7dJs6s3d").unwrap()).power_index(),
            2892
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Td4sJsTsAc7d5d").unwrap()).power_index(),
            4227
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QdKsQh7h2c6s4h").unwrap()).power_index(),
            3851
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ah2h9s9dQh2d7c").unwrap()).power_index(),
            3084
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7hQc3c9cJc5sAd").unwrap()).power_index(),
            6359
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5sTdAh8dQcQs2h").unwrap()).power_index(),
            3786
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qd8c4s6dQc6s9c").unwrap()).power_index(),
            2780
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3c5dQh2c9c8hKd").unwrap()).power_index(),
            6744
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4hTsJc7h8c6c5c").unwrap()).power_index(),
            1606
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ts4c5c7cTcJc6c").unwrap()).power_index(),
            1389
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Th6d9s5s7s2sQd").unwrap()).power_index(),
            7097
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5c5dAc2c9hAdJd").unwrap()).power_index(),
            2558
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5s5d3sTsKd3h7c").unwrap()).power_index(),
            3272
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2c3h5c3s5s6d4c").unwrap()).power_index(),
            1608
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AhKc3c8h3s3hJh").unwrap()).power_index(),
            2336
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("JhTdJd2sKhTh6d").unwrap()).power_index(),
            2832
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QcTd4hJc8s2s4c").unwrap()).power_index(),
            5626
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kc9c2dKs8s7cAd").unwrap()).power_index(),
            3573
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3d5c9c8c2s2h3h").unwrap()).power_index(),
            3320
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AdAs5hQd6dAc7h").unwrap()).power_index(),
            1625
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4hJd8c7d7c4d8h").unwrap()).power_index(),
            3098
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4dTs4cAsKs8c5s").unwrap()).power_index(),
            5528
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qs4d5h7s2cAc9s").unwrap()).power_index(),
            6421
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("TdAc6c8hTs3h6h").unwrap()).power_index(),
            2963
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5h8cAcJsAh6s8h").unwrap()).power_index(),
            2525
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9s2d3c8h7s5h9h").unwrap()).power_index(),
            4612
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Th9d2c8s6c8hTc").unwrap()).power_index(),
            2945
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5cQdJh7h6s2cKd").unwrap()).power_index(),
            6699
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9d3c6c7sKcKs5h").unwrap()).power_index(),
            3716
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KsJc7sJdKc7hQs").unwrap()).power_index(),
            2612
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Tc8h2sAc5c4s8s").unwrap()).power_index(),
            4676
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ac5h9hQs6s5s9s").unwrap()).power_index(),
            3051
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qs9cQhJh8c6sQc").unwrap()).power_index(),
            1764
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7h5hKs6hTd6s2d").unwrap()).power_index(),
            5160
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Th4s4hAd3s9sJs").unwrap()).power_index(),
            5545
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("TsJhQsKdQh3d2d").unwrap()).power_index(),
            3821
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qd6h9hJdAsKd6d").unwrap()).power_index(),
            5086
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KdQc8c3h7s6c4h").unwrap()).power_index(),
            6763
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6sJcTs9s6d8c9d").unwrap()).power_index(),
            3043
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8cQc2h2d3dTcKs").unwrap()).power_index(),
            6022
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qd6c8d5d9d7d5c").unwrap()).power_index(),
            1285
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5hQd7d8h9sTs9h").unwrap()).power_index(),
            4534
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QhTh6cQcQd6d4c").unwrap()).power_index(),
            198
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6s6c8c2dAsJdKc").unwrap()).power_index(),
            5087
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7h4dAd2hQs6d6s").unwrap()).power_index(),
            5100
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KdQhJd4d3hAhQd").unwrap()).power_index(),
            3766
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ts5c8d7sAh9cTd").unwrap()).power_index(),
            4233
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4dAcKc3d2d2hQd").unwrap()).power_index(),
            5966
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9h7sJd3c5c7dAc").unwrap()).power_index(),
            4886
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8c3s5hJh7s4h3h").unwrap()).power_index(),
            5895
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qh8c3dQc9d5sAc").unwrap()).power_index(),
            3793
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ah8c9hJh7cQs2s").unwrap()).power_index(),
            6358
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7c6dAs3hKdTd4c").unwrap()).power_index(),
            6279
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kd3cTh8cQc4d5s").unwrap()).power_index(),
            6723
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ts9d5d6sJd2s6h").unwrap()).power_index(),
            5222
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("As2h6dTdTh9dQc").unwrap()).power_index(),
            4217
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7c8c4s4c3sTh2d").unwrap()).power_index(),
            5696
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4dJc7d4s3hTcAc").unwrap()).power_index(),
            5545
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9cAd8c7dAc6cQd").unwrap()).power_index(),
            3398
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6d9s2hJdTs3cQd").unwrap()).power_index(),
            7009
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2s5c2hKsKh7s4c").unwrap()).power_index(),
            2716
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3c9sAs6sAc7d4h").unwrap()).power_index(),
            3496
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qd4h3s9c6dTdJc").unwrap()).power_index(),
            7009
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ah2d5dQhQc4hAs").unwrap()).power_index(),
            2486
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4h3sJc7c6hTh2s").unwrap()).power_index(),
            7253
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6h5h9c8h7s2h7d").unwrap()).power_index(),
            1605
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8d4d3sTc8s8c8h").unwrap()).power_index(),
            87
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7h8c4dJc4c6c2c").unwrap()).power_index(),
            1458
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6dJs4cJh2d3d9s").unwrap()).power_index(),
            4162
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9h7hAd7c4dKd8d").unwrap()).power_index(),
            4869
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jc5h2c2h9sTc8s").unwrap()).power_index(),
            6102
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6d5c8h9sQcKd9h").unwrap()).power_index(),
            4483
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6d6hAd9dQs3d2s").unwrap()).power_index(),
            5098
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4cQcKc6s9h3dQs").unwrap()).power_index(),
            3840
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4cTdTh2c6dAh5d").unwrap()).power_index(),
            4251
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7d2d2h2cTsJdKc").unwrap()).power_index(),
            2414
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5h2c5c9dTcJs6c").unwrap()).power_index(),
            5442
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5h5c6d2dKh8hJh").unwrap()).power_index(),
            5372
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2d2s9h4dQh6d5s").unwrap()).power_index(),
            6083
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4dAd5sTdKdQc2d").unwrap()).power_index(),
            429
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8cThAs7s9cTc6c").unwrap()).power_index(),
            1604
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3hAh7c5dTcTs5c").unwrap()).power_index(),
            2974
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2h7d3cKc6sAc8d").unwrap()).power_index(),
            6315
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2c6sQdTh4hTd7d").unwrap()).power_index(),
            4327
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5d7d6c9s2d4sAs").unwrap()).power_index(),
            6625
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("JsQh6s5s9c6d2d").unwrap()).power_index(),
            5187
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kc8c2c6sQs4cTh").unwrap()).power_index(),
            6722
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kd2sTh2dJdTsKc").unwrap()).power_index(),
            2624
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jc4d8dQs2h2c4h").unwrap()).power_index(),
            3306
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7d5d9sQd7hTc2d").unwrap()).power_index(),
            4974
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9c3s9sKd6sAs6c").unwrap()).power_index(),
            3040
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qd4c8dKc5dJcQc").unwrap()).power_index(),
            3823
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2h2c3c9d8c9cAs").unwrap()).power_index(),
            3084
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AcKh5c9s9h7h7c").unwrap()).power_index(),
            3029
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3hJcTdTcAhKd6s").unwrap()).power_index(),
            4207
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7c6s9cJcAhQs3h").unwrap()).power_index(),
            6359
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qd3hJc2s4hJd3d").unwrap()).power_index(),
            2910
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QsAs3hJhJc2cTh").unwrap()).power_index(),
            3996
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("JhKc9h6sTc4c9d").unwrap()).power_index(),
            4490
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6sAh9c9dQh2h6h").unwrap()).power_index(),
            3040
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8s8dQd2d8h4sQc").unwrap()).power_index(),
            241
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KdTc2h7sTsKs2c").unwrap()).power_index(),
            2627
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KhKsQd9d9s2h8s").unwrap()).power_index(),
            2634
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3sTsJh5dQhQd9h").unwrap()).power_index(),
            3866
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jh6d9s2hKs3d2c").unwrap()).power_index(),
            6031
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9sAc3cTd4s9h6s").unwrap()).power_index(),
            4455
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kd5dAd7c7dKcJs").unwrap()).power_index(),
            2655
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5c4d3sKdAh9cKc").unwrap()).power_index(),
            3576
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qc3cKhQhJd9s5d").unwrap()).power_index(),
            3822
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8hAsKh7sJh7hTh").unwrap()).power_index(),
            942
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5d4dKd2h2dQhTs").unwrap()).power_index(),
            6022
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("TcAhQs2hAs4h6c").unwrap()).power_index(),
            3393
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9c4s6d3dAhJcJh").unwrap()).power_index(),
            4015
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ks7cTs2sJc9dTd").unwrap()).power_index(),
            4270
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QdKc3h4c5c5h2h").unwrap()).power_index(),
            5367
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6d8h2c5d5sKd8s").unwrap()).power_index(),
            3118
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5hJh3s2s9sQdQh").unwrap()).power_index(),
            3877
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AsJsQcAd5sKc8h").unwrap()).power_index(),
            3326
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6sJd3h3dJsKc9s").unwrap()).power_index(),
            2909
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3h3s5s2s7dAs7s").unwrap()).power_index(),
            810
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qd6h4d6dTh8c9s").unwrap()).power_index(),
            5194
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7c9sJc2dQc7h8h").unwrap()).power_index(),
            4967
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7dQc5d6hKhTcQs").unwrap()).power_index(),
            3832
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4s5dTsQh2s3hQd").unwrap()).power_index(),
            3924
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4dJc3d4h8c2s2d").unwrap()).power_index(),
            3307
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4d9sAs6cKdTdQd").unwrap()).power_index(),
            6194
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AsThQdJc6dTd5d").unwrap()).power_index(),
            4216
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Td3cJc4s7dKh9h").unwrap()).power_index(),
            6799
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5sTsTh3s9hTcQs").unwrap()).power_index(),
            1896
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9d3sAc5c4cKcTh").unwrap()).power_index(),
            6269
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kh7c6c2d5d4d5s").unwrap()).power_index(),
            5396
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5dKcJcTcQd7d4c").unwrap()).power_index(),
            6680
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kc4hJdAcJc3dJh").unwrap()).power_index(),
            1808
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9c5c3c2h2sJd5d").unwrap()).power_index(),
            3285
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AcQh7h6hKs3s5h").unwrap()).power_index(),
            6215
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KcAd3h2dAsTc4c").unwrap()).power_index(),
            3350
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4c5dAc6c8c6sKs").unwrap()).power_index(),
            5090
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6d2d4d6s9sQd8d").unwrap()).power_index(),
            1333
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qh5h4d8h6c3cJd").unwrap()).power_index(),
            7061
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AcAs6dAhJd5c5d").unwrap()).power_index(),
            175
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jc8dTh7s9s2d6s").unwrap()).power_index(),
            1603
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8dJs6d5dKh9sAd").unwrap()).power_index(),
            6238
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9s9cJh8hQc2dKd").unwrap()).power_index(),
            4481
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AdJh6sKc5d6h8s").unwrap()).power_index(),
            5087
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7c4c8hKs2h7s9s").unwrap()).power_index(),
            4945
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jh4s8d3s9h6dAc").unwrap()).power_index(),
            6499
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2cQdQc4c7h4s5h").unwrap()).power_index(),
            2804
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6c3h6d8c7h2d2s").unwrap()).power_index(),
            3255
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jc7s6cKhKc2d5s").unwrap()).power_index(),
            3667
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kc7s8d6h8cTcAd").unwrap()).power_index(),
            4648
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2d7h2hQh3dQcKd").unwrap()).power_index(),
            2821
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ah3c8s8hQc4sTc").unwrap()).power_index(),
            4657
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8sAd4c9s8h2c6c").unwrap()).power_index(),
            4681
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QsQc6sThAs9s6h").unwrap()).power_index(),
            2776
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6c4s9h9dAcQhTs").unwrap()).power_index(),
            4437
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3d4hAs2s2h4d4s").unwrap()).power_index(),
            298
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jd9d4hTc9s8c7c").unwrap()).power_index(),
            1603
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qd2dAs3s4sKd5c").unwrap()).power_index(),
            1609
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KsJd6d9c8c7cTd").unwrap()).power_index(),
            1603
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QsTs6d6c8h2c9s").unwrap()).power_index(),
            5194
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8hAh8s7s2c2sAc").unwrap()).power_index(),
            2528
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6s6hAs8dTcKd2h").unwrap()).power_index(),
            5088
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7c4sAs7h9d9h3s").unwrap()).power_index(),
            3029
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jc3s3h7h7cTs5s").unwrap()).power_index(),
            3197
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kc4c9c7c2dJs4h").unwrap()).power_index(),
            5591
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kh9d2s8d2h7d2d").unwrap()).power_index(),
            2416
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5dJhKc6c8c3h3c").unwrap()).power_index(),
            5812
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6sKc6c7s4d5dKd").unwrap()).power_index(),
            2672
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4h2sJsKs3s3cQc").unwrap()).power_index(),
            5801
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8c4d2sKdQd3h6d").unwrap()).power_index(),
            6769
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7d6sKhTs7c3d9c").unwrap()).power_index(),
            4938
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KdAc7s2s4d5s7h").unwrap()).power_index(),
            4872
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QsAc5d7c9s6d8s").unwrap()).power_index(),
            1605
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ts4d2c8s6h3hKs").unwrap()).power_index(),
            6909
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3d2c4cTdTs7d4s").unwrap()).power_index(),
            2991
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("ThJs3c4s9d2c8c").unwrap()).power_index(),
            7219
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9h9s7hKsQsKc5h").unwrap()).power_index(),
            2634
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QdAh3c5s3s8sJs").unwrap()).power_index(),
            5756
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("As2d4h9cJh6c5h").unwrap()).power_index(),
            6509
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5sKcKs7d8d5hJc").unwrap()).power_index(),
            2679
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4s6c7d2sQc6h9h").unwrap()).power_index(),
            5202
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ks8h6h2d5dAd5h").unwrap()).power_index(),
            5310
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5d6s7s5h8hQhJc").unwrap()).power_index(),
            5408
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3hQdJd9h6h5c6d").unwrap()).power_index(),
            5187
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9dAs4d4s5hTc9s").unwrap()).power_index(),
            3062
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ts7s8c2dAh4c9c").unwrap()).power_index(),
            6554
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7hTc6hJdQh3d4h").unwrap()).power_index(),
            7020
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9d4sTd2hQsKs8d").unwrap()).power_index(),
            6714
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6s2s8sKs5c6h2d").unwrap()).power_index(),
            3250
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9h8d8cKs9d6h4c").unwrap()).power_index(),
            3019
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Js3s6h9hQd4sQs").unwrap()).power_index(),
            3876
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2hQh4dTd2c3d7c").unwrap()).power_index(),
            6076
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qh7sAc4s6dQd9s").unwrap()).power_index(),
            3794
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qs6s6c7sAh6hQd").unwrap()).power_index(),
            265
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9sJhQd6c7s3hAd").unwrap()).power_index(),
            6359
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AhQs8dKh3c4cJd").unwrap()).power_index(),
            6187
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2d6d9d2hTh3s7d").unwrap()).power_index(),
            6131
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8dQcAs5cJd6sJh").unwrap()).power_index(),
            3998
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4sKd2cJc4c2dQd").unwrap()).power_index(),
            3305
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6sTd7h7c8hKd4d").unwrap()).power_index(),
            4939
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3h6sJhAdQd2sKd").unwrap()).power_index(),
            6189
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8d3c2h6hThKs5s").unwrap()).power_index(),
            6908
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ah4sJh5h6d9dTc").unwrap()).power_index(),
            6472
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6c7h5d6s2sAh8c").unwrap()).power_index(),
            5126
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("JcKh7h4d4s4c4h").unwrap()).power_index(),
            132
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("TcKc2c8d5h9hTh").unwrap()).power_index(),
            4278
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8cJh7h5sKcAdQd").unwrap()).power_index(),
            6187
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8s4h9sQc3dAc6c").unwrap()).power_index(),
            6415
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Js3c9c4c2h9sAs").unwrap()).power_index(),
            4450
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7dQsAd9h7s4c3d").unwrap()).power_index(),
            4878
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KdTcQhAh9d6h5c").unwrap()).power_index(),
            6194
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KhAs2hQs2sJs3h").unwrap()).power_index(),
            5966
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9d3d3sQs8hJh2s").unwrap()).power_index(),
            5847
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jc4h2c7h5h2s3h").unwrap()).power_index(),
            6121
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6s3h3sJh7hTh9c").unwrap()).power_index(),
            5882
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6h9dAhTh9cAcTc").unwrap()).power_index(),
            2504
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3c8h7d6sJcQh2d").unwrap()).power_index(),
            7056
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5d8c5h2sTs9dKh").unwrap()).power_index(),
            5378
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8cKd2s2cQcJc3s").unwrap()).power_index(),
            6021
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9dAs5s4dJd3c7s").unwrap()).power_index(),
            6505
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2c8hAhKd4s8s3d").unwrap()).power_index(),
            4653
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qd6sAs6d5s7h8h").unwrap()).power_index(),
            5099
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7h4cTcQd5cKs8s").unwrap()).power_index(),
            6721
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ks9d2dKcAh6d2c").unwrap()).power_index(),
            2710
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9sTc3hKcAd2h8c").unwrap()).power_index(),
            6266
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8hQcTc2h5c2d3s").unwrap()).power_index(),
            6075
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7d4h9dKc2sAs5s").unwrap()).power_index(),
            6301
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ts2hKd5d2cKcTd").unwrap()).power_index(),
            2629
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4s6s9hKhAdKc3s").unwrap()).power_index(),
            3575
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Js8sAd3c3dAcTh").unwrap()).power_index(),
            2580
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9d8c4h5d6h9hAh").unwrap()).power_index(),
            4461
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Tc2s5dJs4cJcAs").unwrap()).power_index(),
            4009
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6d6cKsTd9dAh5h").unwrap()).power_index(),
            5088
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3hQc5d8dQh6cKd").unwrap()).power_index(),
            3846
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("JsTcTh4cAd3c8s").unwrap()).power_index(),
            4226
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6hAh2sTc2dJh6c").unwrap()).power_index(),
            3249
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7d3c7hQhAs5hJs").unwrap()).power_index(),
            4876
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4s8s6c4c8hJh9h").unwrap()).power_index(),
            3131
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kh6c7h8hTcTs4s").unwrap()).power_index(),
            4285
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8s9h3h3c7dKs9d").unwrap()).power_index(),
            3074
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8sAc5sQc6dAh6h").unwrap()).power_index(),
            2546
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QsAd7d4cTd6h9d").unwrap()).power_index(),
            6387
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Js7hAc9d5h2hKd").unwrap()).power_index(),
            6239
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3c3d5cJd7s8hQd").unwrap()).power_index(),
            5848
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2dAcJcKh3cQcAh").unwrap()).power_index(),
            3326
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jh4s6dTd8c3sQh").unwrap()).power_index(),
            7015
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2d6hAd3hKs4c8d").unwrap()).power_index(),
            6321
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jc8sAc8hTh4sJs").unwrap()).power_index(),
            2853
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qd8h4h9s6c8cAc").unwrap()).power_index(),
            4658
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QcAh5hAdAc2s7c").unwrap()).power_index(),
            1625
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7h9h4dTd5c6c7d").unwrap()).power_index(),
            5031
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9h6c2hJdTs8d5h").unwrap()).power_index(),
            7217
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7h3h9d9sJh3d5h").unwrap()).power_index(),
            3076
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AcJh2s7d7s6c2h").unwrap()).power_index(),
            3205
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5c2sKsTd6s9cTs").unwrap()).power_index(),
            4280
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6s2c8s2sTs5h3s").unwrap()).power_index(),
            1528
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9s5s3s7h4dTsTd").unwrap()).power_index(),
            4377
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QcQd2s2cAs9hTs").unwrap()).power_index(),
            2820
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9hKd4sKc8dTc7s").unwrap()).power_index(),
            3682
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9c6dKd3d7h9d5d").unwrap()).power_index(),
            1101
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("TdKh3h4c8h3sKs").unwrap()).power_index(),
            2702
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4s9dTc2sAsKd7h").unwrap()).power_index(),
            6267
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KcAd7hTh4hQs8h").unwrap()).power_index(),
            6195
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qs7sJhQdAhTd9c").unwrap()).power_index(),
            3776
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Th3s9h2cTd5c6c").unwrap()).power_index(),
            4381
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6hQd8hKc8sAdQs").unwrap()).power_index(),
            2754
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Js9h8c7cKhQh9d").unwrap()).power_index(),
            4481
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6h2sTdAd6dKs2c").unwrap()).power_index(),
            3249
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AhQh5cJc7d7s6s").unwrap()).power_index(),
            4876
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KdTdTs2cAd4c6h").unwrap()).power_index(),
            4211
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kh3h7s8cJh8dAh").unwrap()).power_index(),
            4647
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8s3cQs9d8d6hQh").unwrap()).power_index(),
            2758
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7cAd3c9h6dQc8h").unwrap()).power_index(),
            6414
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5s6c4c7s5h9s3d").unwrap()).power_index(),
            1607
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3hJdAh3s8hTh4d").unwrap()).power_index(),
            5765
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3sJdTs4h5h2h8c").unwrap()).power_index(),
            7246
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Td5d3hJsQh4sTc").unwrap()).power_index(),
            4310
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Js8d2cAd3sJd7s").unwrap()).power_index(),
            4020
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8hJc3h3cTd2sAc").unwrap()).power_index(),
            5765
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QdAhJcKdTc2hJs").unwrap()).power_index(),
            1600
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qs5h2dQd8hKdKh").unwrap()).power_index(),
            2604
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QhQdJc9c2d8hJh").unwrap()).power_index(),
            2724
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9c4dJc2d2h7d9d").unwrap()).power_index(),
            3087
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ac8cTcThJs8h7s").unwrap()).power_index(),
            2941
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9d5c8d4hKsTdJd").unwrap()).power_index(),
            6798
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2hKs4cTd6cJsKd").unwrap()).power_index(),
            3649
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6hQc8s9c9d4sJh").unwrap()).power_index(),
            4527
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9cKcJdAcTd7s6s").unwrap()).power_index(),
            6230
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KhTcTs2d6hTd2c").unwrap()).power_index(),
            226
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qh2c3d5c8sQd9s").unwrap()).power_index(),
            3932
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6cKd2d4hAc8hJh").unwrap()).power_index(),
            6246
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7c2s6sJdAs6h2d").unwrap()).power_index(),
            3249
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ad6c5cQc4c5hTh").unwrap()).power_index(),
            5317
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8dJc2d3s9hJd8s").unwrap()).power_index(),
            2857
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("JdQc9c3s7d2h5h").unwrap()).power_index(),
            7042
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7dJhKdAsJs7s5d").unwrap()).power_index(),
            2864
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7c5s4cKc7sTs8d").unwrap()).power_index(),
            4939
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5h6d4s4h2d2sQd").unwrap()).power_index(),
            3306
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kh8dKsTc3dQs7d").unwrap()).power_index(),
            3611
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9sJdKh4c7c5cJc").unwrap()).power_index(),
            4059
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qd6sJs8dKd3c3h").unwrap()).power_index(),
            5801
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ad2d2cJcAc7d3h").unwrap()).power_index(),
            2591
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5s8dQh5h2d3d4h").unwrap()).power_index(),
            5429
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KsQc7s5c6h4sKd").unwrap()).power_index(),
            3631
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3h7s9sTd3cTc5c").unwrap()).power_index(),
            3000
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5hKhJdAhKs6h2s").unwrap()).power_index(),
            3560
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4cTd2sJhQc3s7c").unwrap()).power_index(),
            7022
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6s8sThJhJdJs2d").unwrap()).power_index(),
            1839
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9hQdJd6dTh5c2c").unwrap()).power_index(),
            7009
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4dThJs9s9c6d7s").unwrap()).power_index(),
            4563
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6s8h8d5sKdQsKc").unwrap()).power_index(),
            2645
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2sTsKdTh7c4cKh").unwrap()).power_index(),
            2627
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AsKdQc5h9c9hTd").unwrap()).power_index(),
            4426
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ad8h5d4cJhThKs").unwrap()).power_index(),
            6231
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8cTdJd4s5d4d7h").unwrap()).power_index(),
            5663
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5s8sQc8dTd4h7d").unwrap()).power_index(),
            4755
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2dAd4c2hQcKc7h").unwrap()).power_index(),
            5966
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7sKs3dKc5d7d6h").unwrap()).power_index(),
            2661
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7s5d6h9dJc5s6d").unwrap()).power_index(),
            3219
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7hTc9d2s8h5sKc").unwrap()).power_index(),
            6882
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kd2dQhKs6h4hJs").unwrap()).power_index(),
            3605
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8s2c7c5c4dKc4s").unwrap()).power_index(),
            5611
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ks7h9sAc5hJdAd").unwrap()).power_index(),
            3337
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9cAs7sAcTc4c8d").unwrap()).power_index(),
            3462
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7s4cThTs3dJdQh").unwrap()).power_index(),
            4308
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6d5h2hThTs3s3c").unwrap()).power_index(),
            3003
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5sTsJhKh9s8hKs").unwrap()).power_index(),
            3646
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6c9cKh2s3c5sKc").unwrap()).power_index(),
            3721
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9sKcKh7sQcKd8h").unwrap()).power_index(),
            1689
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3hKc7cAc6s5c9d").unwrap()).power_index(),
            6300
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3s8c5s4d7h9d7d").unwrap()).power_index(),
            5052
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5hAd5dTd4d9c6h").unwrap()).power_index(),
            5333
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KsAsKh3dJh6c7c").unwrap()).power_index(),
            3559
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kh8sQd2h6s6dJs").unwrap()).power_index(),
            5141
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jd2cKh3sKdQh2s").unwrap()).power_index(),
            2711
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AsKc9d4c5dKh9s").unwrap()).power_index(),
            2633
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KdTh5hAd9dQs4c").unwrap()).power_index(),
            6194
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ks5hJdJh5d9d6d").unwrap()).power_index(),
            2887
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5c8hAhKs9sAs3h").unwrap()).power_index(),
            3353
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4dQs8cKsQcAcKh").unwrap()).power_index(),
            2600
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9hTcJc5hAc9c4d").unwrap()).power_index(),
            4445
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("ThAh6c4h4c4s3c").unwrap()).power_index(),
            2273
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6h8c9d3d4cAdAh").unwrap()).power_index(),
            3491
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7s8d5hKh2hTh4c").unwrap()).power_index(),
            6904
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ac8c9cTd4dAs5d").unwrap()).power_index(),
            3462
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KhJd8h5sAcAd4h").unwrap()).power_index(),
            3338
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2c6d5d7d2s4s7c").unwrap()).power_index(),
            3212
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kh6h7c3h6c4h5h").unwrap()).power_index(),
            1140
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8d7c8s8hAs5hTh").unwrap()).power_index(),
            2009
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3h8s5s2cTcQh4c").unwrap()).power_index(),
            7121
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QcJc6d7hTh2c4s").unwrap()).power_index(),
            7020
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Tc5hTh2cQh4c8c").unwrap()).power_index(),
            4323
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3s7sQc9cTd8sQh").unwrap()).power_index(),
            3902
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ts4dJd7cKc3c9h").unwrap()).power_index(),
            6799
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6hKhJd8sKd6dKs").unwrap()).power_index(),
            186
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7c3dTc4cQhTd6s").unwrap()).power_index(),
            4327
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5d8d4hJd6hTd9d").unwrap()).power_index(),
            1355
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7c8hAdQh9c6h8s").unwrap()).power_index(),
            4658
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2d3cTcJs7hJdJh").unwrap()).power_index(),
            1840
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5cQsAsQdKh3d6s").unwrap()).power_index(),
            3771
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6sJsJh3cJcAhKc").unwrap()).power_index(),
            1808
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Tc8h5s2sAcJhJs").unwrap()).power_index(),
            4006
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8s5d2d4d6d5hJd").unwrap()).power_index(),
            1475
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8c5d9sQd2h2s5c").unwrap()).power_index(),
            3284
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4dThQh9h4cKd2s").unwrap()).power_index(),
            5582
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5h5s2hAh9hTc5d").unwrap()).power_index(),
            2207
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("TcTsJc9c7c4d9s").unwrap()).power_index(),
            2933
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jh9sJd6c5d7c9h").unwrap()).power_index(),
            2847
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5hTcKd2sThAsAd").unwrap()).power_index(),
            2501
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6d9dQsQcJh7d4d").unwrap()).power_index(),
            3875
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9cQs3hQdQcKd9d").unwrap()).power_index(),
            195
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jh5dJcTh7hQs2c").unwrap()).power_index(),
            4088
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9h5hAc5d7sQs2d").unwrap()).power_index(),
            5318
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2hTc6d3cTh2c4c").unwrap()).power_index(),
            3014
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9d8s8hQdKdJh3c").unwrap()).power_index(),
            4701
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2c8s6dJdJh4dQh").unwrap()).power_index(),
            4102
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3s6h3dTsKhQh5s").unwrap()).power_index(),
            5802
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qs3d5dJcTsQc9c").unwrap()).power_index(),
            3866
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9hJh3h9d6c4c8h").unwrap()).power_index(),
            4570
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jd7c9d3dQh3h2d").unwrap()).power_index(),
            5847
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qh5c6d8d9hJs7c").unwrap()).power_index(),
            1605
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5cJs2d6sJcTd4d").unwrap()).power_index(),
            4140
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5dTc7sQc9s5h6s").unwrap()).power_index(),
            5414
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ad6s2d7cJs7d4c").unwrap()).power_index(),
            4888
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7s6sQc4h2h8c8s").unwrap()).power_index(),
            4767
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Js9c5c9sKsTh8d").unwrap()).power_index(),
            4490
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7dTsQdKdKc5dAd").unwrap()).power_index(),
            353
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6hJdKcAd5h7d4h").unwrap()).power_index(),
            6251
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jc5h9sQh5d9hJs").unwrap()).power_index(),
            2844
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8sAdAc3s9dQdKs").unwrap()).power_index(),
            3328
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Th4h2h4d7sTs8d").unwrap()).power_index(),
            2990
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QdQhTh5sAd3sKh").unwrap()).power_index(),
            3767
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2dJsQs7h5hTs9d").unwrap()).power_index(),
            7008
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KhKd2cQd3d9h3c").unwrap()).power_index(),
            2700
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4cKc6sJh5h6c9c").unwrap()).power_index(),
            5151
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9cJh5h2s4hJs7h").unwrap()).power_index(),
            4157
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("As5hQc2s7c8s4c").unwrap()).power_index(),
            6436
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5sKsAcJc3sTdTh").unwrap()).power_index(),
            4207
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QcJh5dAh7dAc9d").unwrap()).power_index(),
            3382
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QdAhKhJcJd7s4c").unwrap()).power_index(),
            3986
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4sAh4cTh7cTsQd").unwrap()).power_index(),
            2985
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("JhTh6h3c8s5cTc").unwrap()).power_index(),
            4350
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9c3s4dKcTd8hQh").unwrap()).power_index(),
            6714
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jc3h9s4h3sQc8d").unwrap()).power_index(),
            5847
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6cJh2c8c7cAd7s").unwrap()).power_index(),
            4887
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3cQs6d4c7h9h9s").unwrap()).power_index(),
            4547
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2h3h8dKc8s5h6s").unwrap()).power_index(),
            4736
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7sKh8s6h4s7hKs").unwrap()).power_index(),
            2660
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6h4h3sTh3d4cKc").unwrap()).power_index(),
            3294
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("ThQsAsJh3d9h4s").unwrap()).power_index(),
            6350
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KsJd9s3d3c4d6s").unwrap()).power_index(),
            5811
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9s4cAd7c6sAcKc").unwrap()).power_index(),
            3354
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AhJs7h4hAc9s5s").unwrap()).power_index(),
            3435
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5dAh9d9cJh8c8h").unwrap()).power_index(),
            3018
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6c3s6hKcAs5s6s").unwrap()).power_index(),
            2138
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3h7d4c7s7cTc8h").unwrap()).power_index(),
            2111
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QsAc3dJc2d9hTc").unwrap()).power_index(),
            6350
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3s8s7s6sAs3cTh").unwrap()).power_index(),
            784
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4s2hAh8h3d7c8d").unwrap()).power_index(),
            4688
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9hJs8sQcQs5hTs").unwrap()).power_index(),
            1602
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kd7cQd5c3h6dKh").unwrap()).power_index(),
            3631
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5c6h8c3c2hJd4c").unwrap()).power_index(),
            1608
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6cTcJhAhQd2cJd").unwrap()).power_index(),
            3996
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KcAd4c6c4hTc7h").unwrap()).power_index(),
            5528
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3sAs3d6sKh7c5d").unwrap()).power_index(),
            5751
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7h8c9sAdQdJdJs").unwrap()).power_index(),
            3997
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9h9s7sAc7dTs5c").unwrap()).power_index(),
            3029
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4sJsAs2d2h4c6c").unwrap()).power_index(),
            3304
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6s6d8s7hQs8hQd").unwrap()).power_index(),
            2759
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4s7d7hQc4d2d4c").unwrap()).power_index(),
            294
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7c6hJc9d5d9sKc").unwrap()).power_index(),
            4492
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QsKh8d3s8c4s6c").unwrap()).power_index(),
            4705
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("TsQs6h9cQh3s9d").unwrap()).power_index(),
            2746
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7cKs6sJd4h6dAh").unwrap()).power_index(),
            5087
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6c9cTsTh5sJd8s").unwrap()).power_index(),
            4342
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5h3cTc7s5d9sJc").unwrap()).power_index(),
            5442
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6s7d5sKh9d9sQd").unwrap()).power_index(),
            4484
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ks2hKd3cQh8h8c").unwrap()).power_index(),
            2645
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7dJcAd8c2sJh6c").unwrap()).power_index(),
            4020
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2h3sAh7c8dTh9h").unwrap()).power_index(),
            6554
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4hTcQhAd4sJh3h").unwrap()).power_index(),
            5536
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7d8cKs8hAh5c2h").unwrap()).power_index(),
            4650
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9dTsJdAs5s4s5h").unwrap()).power_index(),
            5325
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3cQdQsJc7c3s2s").unwrap()).power_index(),
            2811
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AsTs6c7h4h2dJh").unwrap()).power_index(),
            6483
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kc7sKs9h2cTs3s").unwrap()).power_index(),
            3683
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6h8cTs3d6d4sJc").unwrap()).power_index(),
            5223
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2hKh8dAc4sTd9d").unwrap()).power_index(),
            6266
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4d3sKs7s2s9cAd").unwrap()).power_index(),
            6302
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3d6dQh7h6h4d2d").unwrap()).power_index(),
            5213
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2d8h2h9hAhJcQd").unwrap()).power_index(),
            5976
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9sJcTs7cAs5dJh").unwrap()).power_index(),
            4005
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9dKs8h4sQs4hKh").unwrap()).power_index(),
            2689
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4d7d4s4h3s9s5c").unwrap()).power_index(),
            2316
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jd2dQdAc7d8d8h").unwrap()).power_index(),
            1197
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6dTh7s8cJdTd4c").unwrap()).power_index(),
            4349
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AsAcQs6d2c7hJc").unwrap()).power_index(),
            3384
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2d9hQc8d6sQd4c").unwrap()).power_index(),
            3931
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3sTs6cQs7c4cAh").unwrap()).power_index(),
            6399
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qd5h9d6c5s8cAd").unwrap()).power_index(),
            5318
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jh9d6c8h5d7sQc").unwrap()).power_index(),
            1605
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3d4h8s4sJc7cQs").unwrap()).power_index(),
            5628
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jc9d9c5cJsAd6d").unwrap()).power_index(),
            2842
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QhJsJcJh8d4c5c").unwrap()).power_index(),
            1831
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3h7s9d5hAcTd5s").unwrap()).power_index(),
            5333
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Td4c2d4s6cQdQs").unwrap()).power_index(),
            2801
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9s4s2s8sQsTd2d").unwrap()).power_index(),
            1297
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6c4h7sJd2hQsQc").unwrap()).power_index(),
            3887
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8sKsTd7d9sTh8d").unwrap()).power_index(),
            2942
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7c9d8s2s3s9sJh").unwrap()).power_index(),
            4569
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jd6sQd7d9s5hKc").unwrap()).power_index(),
            6687
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("JcAc6d2s8h9h6s").unwrap()).power_index(),
            5106
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8d6c4h8c6dQc9d").unwrap()).power_index(),
            3108
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5d2dTsJdAs9c3c").unwrap()).power_index(),
            6473
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qc3cJh8c3dKd9d").unwrap()).power_index(),
            5801
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6s8d5s9c4s3cKs").unwrap()).power_index(),
            6943
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3s8cKdKhJdTs9s").unwrap()).power_index(),
            3646
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7d8h3c7hJd5h6d").unwrap()).power_index(),
            5015
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3d3h9sQs7s9c8h").unwrap()).power_index(),
            3075
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("TdQs3s3c6c8d4d").unwrap()).power_index(),
            5855
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3h9s7c7d2s9h4h").unwrap()).power_index(),
            3037
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4sJh7dQh9d5c8c").unwrap()).power_index(),
            7035
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4c6s6d2dAd9c8s").unwrap()).power_index(),
            5120
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jh7s2s5cJsQh9h").unwrap()).power_index(),
            4095
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ks7sJhAhKdQc6h").unwrap()).power_index(),
            3546
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4cJhQs8hKc2sTd").unwrap()).power_index(),
            6679
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("TcJc9s2d5c9dQc").unwrap()).power_index(),
            4526
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8d5dJd7d8h3c6c").unwrap()).power_index(),
            4795
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5s7hJdKd3c4dQd").unwrap()).power_index(),
            6700
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kh6h5h8dKcQd6s").unwrap()).power_index(),
            2667
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("TsTdQs4h2cJs4d").unwrap()).power_index(),
            2987
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5c3h9dJdKc2s9h").unwrap()).power_index(),
            4494
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8sQsTs4h5sQhAc").unwrap()).power_index(),
            3786
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4c2sTc6h7cJh9s").unwrap()).power_index(),
            7222
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qs2hKs4sJh9sQc").unwrap()).power_index(),
            3822
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jc2s9h6sQc7cTc").unwrap()).power_index(),
            7008
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8dKh8c9d9sJd7d").unwrap()).power_index(),
            3019
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AcTc8d6d2sAsKc").unwrap()).power_index(),
            3346
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8dThJh2dTs4sAc").unwrap()).power_index(),
            4226
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7dJc2h8s4c9hJs").unwrap()).power_index(),
            4150
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QsJhKdTcKhAsJd").unwrap()).power_index(),
            1600
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7s7cJdAs6s2cKd").unwrap()).power_index(),
            4867
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kd2c6sAc2hJc7h").unwrap()).power_index(),
            5967
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qs2d4d9d7s6h3c").unwrap()).power_index(),
            7163
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("As4cKs4sQsJsQc").unwrap()).power_index(),
            328
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2sAh5s8h4h7c9s").unwrap()).power_index(),
            6611
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7sKd4cThQh6d5d").unwrap()).power_index(),
            6727
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ac8d7cJh5s9h8h").unwrap()).power_index(),
            4666
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3c4sJd4d8d7hKd").unwrap()).power_index(),
            5592
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2s9cJc8c9hJs5d").unwrap()).power_index(),
            2846
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("JsTsKc4h8sKs6c").unwrap()).power_index(),
            3647
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("ThAd2s6hTdKh3s").unwrap()).power_index(),
            4211
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2s8h8c3s9dJdJh").unwrap()).power_index(),
            2857
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6cQh2h7c8dJh6h").unwrap()).power_index(),
            5188
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7c8s8hJs5h7d8c").unwrap()).power_index(),
            245
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8h6cKd7dQs5h2c").unwrap()).power_index(),
            6763
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qc2c9c9dJd4s2s").unwrap()).power_index(),
            3086
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2d4sJs6sTsTcQc").unwrap()).power_index(),
            4309
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3c4cKhJh3hAcKc").unwrap()).power_index(),
            2699
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Js8d7dTdAcQsTs").unwrap()).power_index(),
            4216
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5d3cKh6d9cQc4c").unwrap()).power_index(),
            6753
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("As6c6s9c3hJd8c").unwrap()).power_index(),
            5106
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7cQc9cJc3hAdTc").unwrap()).power_index(),
            1145
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3c6cTd9d7s8dTc").unwrap()).power_index(),
            1604
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5sKs7h6c4sKd7c").unwrap()).power_index(),
            2661
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3c4sJc7d4d2d6s").unwrap()).power_index(),
            5680
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Th5d7s7h9s6s9c").unwrap()).power_index(),
            3033
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("TsTh4hKc6d3sAd").unwrap()).power_index(),
            4211
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5hAc9s5sAd4c6d").unwrap()).power_index(),
            2560
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4cAd7d2hKcAc9d").unwrap()).power_index(),
            3354
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ks4s2d4hJs3sJc").unwrap()).power_index(),
            2898
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jh4d6d8dKdJs7h").unwrap()).power_index(),
            4065
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2h2c2d4cTdThQd").unwrap()).power_index(),
            315
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AsJhTd7d9dQcJd").unwrap()).power_index(),
            3996
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qh7d3sJs3c4dAc").unwrap()).power_index(),
            5756
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4c7d5dKsTdQd7s").unwrap()).power_index(),
            4922
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KcTcJs7cKh6s4c").unwrap()).power_index(),
            3648
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KhAhQhTc5dKc7s").unwrap()).power_index(),
            3547
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5c8sKc2d4h5d2s").unwrap()).power_index(),
            3283
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6s6dKh4h4c4d2c").unwrap()).power_index(),
            295
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9sTh2c3dTs4hJs").unwrap()).power_index(),
            4346
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qd3dKs9sKdAd5d").unwrap()).power_index(),
            362
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5h7d8hAd5cAs2s").unwrap()).power_index(),
            2561
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3hAcQh8s8c6cTh").unwrap()).power_index(),
            4657
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9dQsQd2c5h7s2d").unwrap()).power_index(),
            2824
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ks8cJd9d8h5c6s").unwrap()).power_index(),
            4711
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9d8s5s7sJs4h4s").unwrap()).power_index(),
            1448
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9d8hAhAs5h4h3d").unwrap()).power_index(),
            3492
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6c2cQcJd7s4h9s").unwrap()).power_index(),
            7041
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("TcKd4sAs9sTd4d").unwrap()).power_index(),
            2985
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Js3cQh4h8s7dQd").unwrap()).power_index(),
            3881
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KcJdTsJcAc7hKd").unwrap()).power_index(),
            2611
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Td2sQcQd2cKd3h").unwrap()).power_index(),
            2821
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8s7h6hTc7d2s3c").unwrap()).power_index(),
            5036
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8dJd6c4hQd8h2h").unwrap()).power_index(),
            4749
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3hQs2hJcKcKh5d").unwrap()).power_index(),
            3606
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9c9h8c6s3c7s2s").unwrap()).power_index(),
            4611
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QcKhKs9dTh3d2h").unwrap()).power_index(),
            3610
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3s4cQs4hTd3d5d").unwrap()).power_index(),
            3295
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5s6d6h8d5c9sQs").unwrap()).power_index(),
            3218
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AcKsJdQd4sJs9s").unwrap()).power_index(),
            3986
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6cAs3sQh8h3h9c").unwrap()).power_index(),
            5758
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("JcAsTh3cTc8s7h").unwrap()).power_index(),
            4226
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5dJd7c5cKc9sJh").unwrap()).power_index(),
            2887
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7dKhJc5hTd3s5d").unwrap()).power_index(),
            5370
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3dJs2c9hKc9d4c").unwrap()).power_index(),
            4495
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Tc8sQh5h3sAd9c").unwrap()).power_index(),
            6386
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("JsKc2d8sTd8cJd").unwrap()).power_index(),
            2854
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6s8sAd9h5sAs6d").unwrap()).power_index(),
            2549
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9d8d2c8h4hKc3s").unwrap()).power_index(),
            4728
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AdJhQc7c4d5sTd").unwrap()).power_index(),
            6352
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qs9d8cJsTdKd8s").unwrap()).power_index(),
            1601
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QsJs8c9sKdAh8h").unwrap()).power_index(),
            4646
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9dJd6sKdThAd3s").unwrap()).power_index(),
            6230
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kh3c6h2h7dQh9h").unwrap()).power_index(),
            893
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7sJd6dTd9d4c4h").unwrap()).power_index(),
            5662
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2d5sTc4d8sAd6c").unwrap()).power_index(),
            6580
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Th8h5cKsJh7dTc").unwrap()).power_index(),
            4271
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KsJs6cJh7s4dTs").unwrap()).power_index(),
            4052
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("JsKsJdAd4s2c2h").unwrap()).power_index(),
            2919
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5h5sQs8d6d7s3d").unwrap()).power_index(),
            5427
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KsQhQc8d2d9c2s").unwrap()).power_index(),
            2821
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AcJs9c6hThTsQh").unwrap()).power_index(),
            4216
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2h6s4c3c4dKh9d").unwrap()).power_index(),
            5607
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6c8c2cKhKcTc7h").unwrap()).power_index(),
            1048
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Td3d6d4dQdQc8d").unwrap()).power_index(),
            1255
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2cTs7d7h3s6c4c").unwrap()).power_index(),
            5042
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7h8dAhQc3cJs4d").unwrap()).power_index(),
            6365
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ac4h8h5s9sQdKs").unwrap()).power_index(),
            6202
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AhJhKc8dTs4sKh").unwrap()).power_index(),
            3556
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8sKs2d7d3c8h7c").unwrap()).power_index(),
            3096
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Tc4dTsQh4c7hAs").unwrap()).power_index(),
            2985
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qs9h6sKc7c4h2s").unwrap()).power_index(),
            6748
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jh8c3cQd5d9hAd").unwrap()).power_index(),
            6358
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2hTd7hAh7dAcKd").unwrap()).power_index(),
            2534
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("JcKc5h9cQhJdJh").unwrap()).power_index(),
            1819
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6d9s9dAs3d8s5d").unwrap()).power_index(),
            4461
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8h5s9s7sKh2d3d").unwrap()).power_index(),
            6939
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2h3c8s9d9c8cJs").unwrap()).power_index(),
            3021
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9c2cQd4s3cTc2d").unwrap()).power_index(),
            6074
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qd6cJd5cKs2c9h").unwrap()).power_index(),
            6688
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6d2d6s7h3h9s8c").unwrap()).power_index(),
            5271
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6c8h5h9h2h6d8s").unwrap()).power_index(),
            3111
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3d6c8h2h3h6s4d").unwrap()).power_index(),
            3244
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Tc6s5h5dQc2c8c").unwrap()).power_index(),
            5415
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Th9d2hQcKs3s4s").unwrap()).power_index(),
            6718
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Th5h8cAcKd5dTd").unwrap()).power_index(),
            2974
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4sAsJd9s8s7h7s").unwrap()).power_index(),
            749
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3c8s5cKsAs8h7c").unwrap()).power_index(),
            4650
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7s8s7c3c2s7dQs").unwrap()).power_index(),
            2096
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8cKh5s7d4d3sAh").unwrap()).power_index(),
            6316
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4c8h5s7cJh4dQc").unwrap()).power_index(),
            5628
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3cJsQhTsAd5cQd").unwrap()).power_index(),
            3776
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jh4h9sQh7s8dQc").unwrap()).power_index(),
            3874
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ah7c2hTs6c9d9s").unwrap()).power_index(),
            4454
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KcJs5h3c9s9dAd").unwrap()).power_index(),
            4427
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7h2h5c8s7c6c3h").unwrap()).power_index(),
            5066
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AsQh3hKdAhJhQs").unwrap()).power_index(),
            2479
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3d5h9sAd2h4s8s").unwrap()).power_index(),
            1609
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AcKh4hTcQhQcTs").unwrap()).power_index(),
            2732
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5h7sJs4c5cKd7h").unwrap()).power_index(),
            3173
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kc6c8h5c8s2c5d").unwrap()).power_index(),
            3118
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7d7h9h6d3hTdTh").unwrap()).power_index(),
            2956
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5sJsQsTs5dKhKc").unwrap()).power_index(),
            2678
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kc2s9hQh6dKhTd").unwrap()).power_index(),
            3610
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6h7c9c5c9sQs4h").unwrap()).power_index(),
            4547
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8hKc8s5c4s8c3s").unwrap()).power_index(),
            2023
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5d6cAcJcJh4s4h").unwrap()).power_index(),
            2897
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ac6dQdQs2d5sKd").unwrap()).power_index(),
            3771
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KcAc5s3s6s4s3d").unwrap()).power_index(),
            5752
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ts3dJc3sJh4dKc").unwrap()).power_index(),
            2909
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ks6cJd9c2d7hJs").unwrap()).power_index(),
            4059
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("TdJdAh6h2s9sTs").unwrap()).power_index(),
            4225
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5h2cJc7h2h6dTc").unwrap()).power_index(),
            6104
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4d2h3cTh4sQc2s").unwrap()).power_index(),
            3306
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ad9d4s8sTd2c8c").unwrap()).power_index(),
            4673
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9sTsJhJd9c4s3c").unwrap()).power_index(),
            2845
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8hTcQd7s7hJdAd").unwrap()).power_index(),
            4876
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9h9c8h4sJcKc9s").unwrap()).power_index(),
            1952
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Tc8sQsKh5s9c6d").unwrap()).power_index(),
            6714
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3dAs4d9h8sTsJc").unwrap()).power_index(),
            6470
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4h7d2s3dAcKs9c").unwrap()).power_index(),
            6302
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8h9h9d4h5sAh6h").unwrap()).power_index(),
            753
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8hAhQcTh6sQh7d").unwrap()).power_index(),
            3786
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5c3hQcJs5dJc7s").unwrap()).power_index(),
            2888
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9sThKsAc6c5dJc").unwrap()).power_index(),
            6230
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6c4d9s6d7d3sJd").unwrap()).power_index(),
            5230
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ad3dAc2dQs3s4s").unwrap()).power_index(),
            2579
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3d3cTdTcJc4cJd").unwrap()).power_index(),
            2839
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7s8hJsTsAs2s2d").unwrap()).power_index(),
            624
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6hJhJd3s6d5d2c").unwrap()).power_index(),
            2882
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4sJc8sTc6cQcKs").unwrap()).power_index(),
            6679
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KhTd7sQhTsJh5h").unwrap()).power_index(),
            4261
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9sJc2hKsAd9d6h").unwrap()).power_index(),
            4427
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3h2sTd7d5c5h9c").unwrap()).power_index(),
            5471
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7cKh2d6c3cAhKc").unwrap()).power_index(),
            3586
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4s6dJd8cAhQd2d").unwrap()).power_index(),
            6366
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7dKhThAs4hAcKs").unwrap()).power_index(),
            2470
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9cTc2sAdQh6c5h").unwrap()).power_index(),
            6388
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("JcJhAdQd4h7cJd").unwrap()).power_index(),
            1809
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9s7sTsJs6c5sTh").unwrap()).power_index(),
            1360
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4d3hTcAcAh9s5c").unwrap()).power_index(),
            3465
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Tc3s8d2dTsKsKc").unwrap()).power_index(),
            2626
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ah7s4cQh6hAd2h").unwrap()).power_index(),
            3411
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KhTd7s5sTc9cJd").unwrap()).power_index(),
            4270
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3sJs7c4h2h7hTs").unwrap()).power_index(),
            5006
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9d9cQcQhTd9hKd").unwrap()).power_index(),
            229
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5c5dJd7h5sAhTs").unwrap()).power_index(),
            2206
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QhQd4sAdTd4c3c").unwrap()).power_index(),
            2798
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7h3c3d5cJcKhTs").unwrap()).power_index(),
            5810
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4c4dKhQs9h6cKc").unwrap()).power_index(),
            2689
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Td4d8sJdKs2sQs").unwrap()).power_index(),
            6679
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2dJc5cJh9hAcQh").unwrap()).power_index(),
            3997
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6hQd9d2dKs3dKh").unwrap()).power_index(),
            3620
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9s3s6sJcJsAdAc").unwrap()).power_index(),
            2493
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QsTc8d5h6s6dQd").unwrap()).power_index(),
            2779
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jd6c7c7hJcJsKh").unwrap()).power_index(),
            209
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KcJhQs4sAdJsQd").unwrap()).power_index(),
            2721
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AsQc9h8d9c2cTd").unwrap()).power_index(),
            4437
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8hKd3hTs4d6hJd").unwrap()).power_index(),
            6806
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("JdTd4c7dKcAh2h").unwrap()).power_index(),
            6232
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2h6hThKc6s8sQd").unwrap()).power_index(),
            5142
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5s8d8s7h9sQcTd").unwrap()).power_index(),
            4754
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7c6h4hQc2dTcTd").unwrap()).power_index(),
            4327
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QhKh6h2c3dQs6s").unwrap()).power_index(),
            2777
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8cTdJhKd3s2cAs").unwrap()).power_index(),
            6231
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Th6h2sKs2hQsJc").unwrap()).power_index(),
            6021
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8h6s9dKhJh2h5d").unwrap()).power_index(),
            6827
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4h7dQcJh4s8s6s").unwrap()).power_index(),
            5628
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3cAd9hKh5h4h4c").unwrap()).power_index(),
            5529
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jd8h8c3c6d8s5s").unwrap()).power_index(),
            2039
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6d4c8d8h4d2dJd").unwrap()).power_index(),
            1458
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9c6c9sAsKh8hTh").unwrap()).power_index(),
            4428
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4c6d4s8s5d4hQh").unwrap()).power_index(),
            2294
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jh4s4h6hAh4cTh").unwrap()).power_index(),
            626
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("ThKs8h5d3c3s8c").unwrap()).power_index(),
            3140
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ks7dJs2dTcJc4d").unwrap()).power_index(),
            4052
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2h7s2c9c6d8hJh").unwrap()).power_index(),
            6109
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5s5d8c6cKd8sKh").unwrap()).power_index(),
            2650
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8s3cKsAdAs4dAc").unwrap()).power_index(),
            1614
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3sQd7d9d3h4cQh").unwrap()).power_index(),
            2813
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("TdAh5s5d6s4d7d").unwrap()).power_index(),
            5335
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9c4d4hTs9h6cQh").unwrap()).power_index(),
            3064
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8h8c4h6dQd3c9d").unwrap()).power_index(),
            4762
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6dAs7s8cJhJcTh").unwrap()).power_index(),
            4006
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8sJh6s7sAsJd2h").unwrap()).power_index(),
            4020
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QcTc3s7d8s3cAh").unwrap()).power_index(),
            5757
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("As8d9dAc2cKc9h").unwrap()).power_index(),
            2512
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9cTsThJhKh7d5c").unwrap()).power_index(),
            4270
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8cKh9hKcJhJsJc").unwrap()).power_index(),
            204
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qc7hAh2d4cJd5s").unwrap()).power_index(),
            6372
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5s2c6s2dAs7dKh").unwrap()).power_index(),
            5971
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kh7d9sKd3cKc5h").unwrap()).power_index(),
            1715
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("TdAdJc7s6d4dKh").unwrap()).power_index(),
            6232
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8dKh7c2h9s9hAd").unwrap()).power_index(),
            4429
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("TsQd3s4sJdKhKc").unwrap()).power_index(),
            3601
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6s3dAs5hQd9dKc").unwrap()).power_index(),
            6204
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AsQs3d2cJh7dAc").unwrap()).power_index(),
            3384
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2sKcQs2d9sTc5h").unwrap()).power_index(),
            6022
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9s2d2s9dQh9hJh").unwrap()).power_index(),
            238
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2dAc8d9dKd3c9s").unwrap()).power_index(),
            4429
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("JcJd9h4sQdJh5h").unwrap()).power_index(),
            1830
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jh2s6s9dAd5cQd").unwrap()).power_index(),
            6360
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Td5h6c4hKd7h6d").unwrap()).power_index(),
            5160
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8dTc8c6d8h5h6s").unwrap()).power_index(),
            246
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9hKc6cQd5hTd8c").unwrap()).power_index(),
            6714
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6cAdAh7hQd9dKh").unwrap()).power_index(),
            3328
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Js8h2d8cTs2c9s").unwrap()).power_index(),
            3153
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2h5c3sKs9dAd4d").unwrap()).power_index(),
            1609
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2hKc5c4sTdQc3d").unwrap()).power_index(),
            6736
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3c5h8d2c5s2dAh").unwrap()).power_index(),
            3282
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("TsAd7d8s9d7h7c").unwrap()).power_index(),
            2075
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7sTd9cTs2dKhKd").unwrap()).power_index(),
            2625
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4c4hQhAcKd8hKs").unwrap()).power_index(),
            2688
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2c9dTcTs4sKd3s").unwrap()).power_index(),
            4282
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9sJsAd4h4d3h3c").unwrap()).power_index(),
            3293
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AsAhQd3sQs2s5s").unwrap()).power_index(),
            605
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7d6c5cQhTs7s2d").unwrap()).power_index(),
            4976
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8d6dJhAc7d6h4d").unwrap()).power_index(),
            5107
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5s8cTcJc6c2cTs").unwrap()).power_index(),
            1382
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ah7d5dAdQh2d4d").unwrap()).power_index(),
            809
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qd4hTd5c4cKd8c").unwrap()).power_index(),
            5582
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kd7d3cKc8cJd4h").unwrap()).power_index(),
            3661
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8hJsJhQdKh3c6h").unwrap()).power_index(),
            4043
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6c6dKd3h2sJhKs").unwrap()).power_index(),
            2668
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2h4c7s6sJdQcKd").unwrap()).power_index(),
            6699
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8hQd4s2h8c3cJs").unwrap()).power_index(),
            4751
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jh5c7d6s5hTs7h").unwrap()).power_index(),
            3175
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AcAdKh5c8hQhKc").unwrap()).power_index(),
            2468
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7h8hTs9sKd2s7d").unwrap()).power_index(),
            4938
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kc6s4sQd8h3d9d").unwrap()).power_index(),
            6743
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4d2c3s7d4c7s7h").unwrap()).power_index(),
            260
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AcJhTs5s9c4hAs").unwrap()).power_index(),
            3426
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AsJcJh9s8h2cTd").unwrap()).power_index(),
            4005
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6dQd9s3cJh2sTs").unwrap()).power_index(),
            7009
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4dKc9sQs3c7sTc").unwrap()).power_index(),
            6715
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KdQcJd9dAh9s4c").unwrap()).power_index(),
            4426
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kc4dAhTd6c7h3c").unwrap()).power_index(),
            6279
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("JdAc3hKc9sJh2h").unwrap()).power_index(),
            3988
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7h4d6c2h8h7d8d").unwrap()).power_index(),
            3101
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Td6sAs8hTcKcQc").unwrap()).power_index(),
            4206
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7h7c3dQdQs4dJd").unwrap()).power_index(),
            2767
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AhKs7h8c5h4sTd").unwrap()).power_index(),
            6273
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8c9dQh6c7h3d9h").unwrap()).power_index(),
            4541
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7s3cAc7cJhKc9s").unwrap()).power_index(),
            4867
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4dThTc5hAs8s9c").unwrap()).power_index(),
            4233
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8c2dQhKs7h3s6c").unwrap()).power_index(),
            6763
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AdQs2dAc3cTcTs").unwrap()).power_index(),
            2502
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9d5sAcAh2d6c6s").unwrap()).power_index(),
            2549
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kd9c4h8c7c7h6s").unwrap()).power_index(),
            4945
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("JhQc5sKc7c3dKd").unwrap()).power_index(),
            3604
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qh3dJs8sKd3cTh").unwrap()).power_index(),
            5801
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2h7c8c8dAd4dJd").unwrap()).power_index(),
            4667
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qd7hAd9s9d6sKh").unwrap()).power_index(),
            4426
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9c8c3d8hTs6cKd").unwrap()).power_index(),
            4718
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3c5hThKsAh6cQs").unwrap()).power_index(),
            6197
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8d7s9dQh5c4cKs").unwrap()).power_index(),
            6742
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3sAh6hJh7hJd2s").unwrap()).power_index(),
            4026
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9hJd8s8c8h4d2h").unwrap()).power_index(),
            2037
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7c3sKh6s8dTcJc").unwrap()).power_index(),
            6805
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ac8sAs3cKd5cTh").unwrap()).power_index(),
            3346
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3c3s4h7dAsQsKc").unwrap()).power_index(),
            5746
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6d5cQcJcJdJs5s").unwrap()).power_index(),
            211
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5c7hQcKdAh2d9d").unwrap()).power_index(),
            6203
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("QcKsTh2h5hJs3c").unwrap()).power_index(),
            6682
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ac7hJh5dThAs3h").unwrap()).power_index(),
            3428
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9sJh9c7hAs2sQc").unwrap()).power_index(),
            4436
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qd5d7c4s9dKh2h").unwrap()).power_index(),
            6749
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("AdTdQsAc8dKcTh").unwrap()).power_index(),
            2501
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4hThJs5cAd8d5h").unwrap()).power_index(),
            5325
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("7sQcQdKd9d5d7c").unwrap()).power_index(),
            2766
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9cAc3h3sJc4sJd").unwrap()).power_index(),
            2908
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6c7d6sJdQh9sTd").unwrap()).power_index(),
            5186
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6hAc5h4h6dAsQc").unwrap()).power_index(),
            2546
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KsAh2h5s9h7s4c").unwrap()).power_index(),
            6301
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9dAd8sAhQc6h7h").unwrap()).power_index(),
            3398
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qd5cKh9hQc6cKc").unwrap()).power_index(),
            2603
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2d2hKh6s4c2s5c").unwrap()).power_index(),
            2419
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Js9cTh8hAcTs7s").unwrap()).power_index(),
            1603
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9hQh4dJc2d4h6d").unwrap()).power_index(),
            5627
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2c5h8sAhQd2h4d").unwrap()).power_index(),
            5979
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kd4c6dTs7h4h6c").unwrap()).power_index(),
            3228
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jh8h3s9d2sQh6d").unwrap()).power_index(),
            7036
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("KhKsQc3cQs3d9s").unwrap()).power_index(),
            2603
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Js3s8h5s7hTd2h").unwrap()).power_index(),
            7238
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6d8cAd5s6sKc5h").unwrap()).power_index(),
            3216
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6h3dKs8s9d3s4h").unwrap()).power_index(),
            5825
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3hJh2s7c8s3s9s").unwrap()).power_index(),
            5889
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9h2sJhQs6d3s8h").unwrap()).power_index(),
            7036
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4c2s7hKcTh5sAs").unwrap()).power_index(),
            6280
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2dQh4d9d2h8dKh").unwrap()).power_index(),
            6023
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ac5h9dTs3hKc4h").unwrap()).power_index(),
            6269
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("4h9s7dJd6cQh2d").unwrap()).power_index(),
            7041
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2hKh7hAc6hQcAh").unwrap()).power_index(),
            470
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3dQc3s4c9h4hAs").unwrap()).power_index(),
            3293
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2dQhTh2s9d9hQd").unwrap()).power_index(),
            2746
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3c4s6s6d7c9c6h").unwrap()).power_index(),
            2184
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2c6h8sKc2h5s7h").unwrap()).power_index(),
            6051
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("5d2d8sAh5cAc4s").unwrap()).power_index(),
            2561
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jh2sQd9s7s6hJs").unwrap()).power_index(),
            4095
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jd3s7h2cKc9s2s").unwrap()).power_index(),
            6031
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2sKsJc7dKhTd8d").unwrap()).power_index(),
            3647
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Th4s9c3c6h2s6s").unwrap()).power_index(),
            5253
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8sTs2cTd7dQs3h").unwrap()).power_index(),
            4321
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("6sTh6hAs6d2c2h").unwrap()).power_index(),
            274
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qd6d8sAc8d4cQs").unwrap()).power_index(),
            2754
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3dAdKsTdQc2d4h").unwrap()).power_index(),
            6199
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9d4c8h5dAc2c5c").unwrap()).power_index(),
            5340
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kc4c9cAs8s4sQs").unwrap()).power_index(),
            5526
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Qs8hQh9s7d4d2h").unwrap()).power_index(),
            3930
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9s2s6dAcTs3dKh").unwrap()).power_index(),
            6268
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3d6c7hQcTsTd5c").unwrap()).power_index(),
            4327
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Kh5d6s6d2c5cJc").unwrap()).power_index(),
            3217
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("9dQh4s2s2cAhTc").unwrap()).power_index(),
            5977
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3hAs5hQs8hAh3c").unwrap()).power_index(),
            2579
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("2s8s8h6dJc7sJh").unwrap()).power_index(),
            2858
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3s7h5h7s5dAc9c").unwrap()).power_index(),
            3172
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3h9s8c7d3c2d6c").unwrap()).power_index(),
            5931
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3d9s2c2d7sAh8s").unwrap()).power_index(),
            6000
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8cTc5s2c7d5c8s").unwrap()).power_index(),
            3121
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8s7s8h6sAd5s9s").unwrap()).power_index(),
            6
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8c7s8h8dAdQh7h").unwrap()).power_index(),
            245
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("TsJc6s3hTdAh6d").unwrap()).power_index(),
            2963
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3h9h7s6c4hJhKc").unwrap()).power_index(),
            6832
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Jd7h4d2sTc9c9s").unwrap()).power_index(),
            4563
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8dThKd6s8sAh3s").unwrap()).power_index(),
            4648
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Th7sJs3cKhKs4d").unwrap()).power_index(),
            3648
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ts6d7d2sJh7c5c").unwrap()).power_index(),
            5004
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("8d7cJdJhQd9s9c").unwrap()).power_index(),
            2844
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("Ks4c7d2d4s8c3s").unwrap()).power_index(),
            5611
        );
        assert_eq!(
            MadeHand::from(CardSet::from_str("3d7c5d6c3c2d8c").unwrap()).power_index(),
            5946
        );
    }
}

fn find_flush_suit<'s>(cards: &CardSet) -> Option<Suit> {
    let mut suit_counts = [0; 4];

    for card in cards {
        let suit = card.suit();
        let suit_index: usize = suit.into();

        suit_counts[suit_index] += 1;

        if suit_counts[suit_index] >= 5 {
            return Some(*suit);
        }
    }

    None
}

fn hash_for_flush(cards: &CardSet, suit: &Suit) -> u32 {
    let mut hash: u32 = 0;

    for card in cards {
        if card.suit() == suit {
            hash += match card.rank() {
                Rank::Ace => 0x1000,
                Rank::King => 0x800,
                Rank::Queen => 0x400,
                Rank::Jack => 0x200,
                Rank::Ten => 0x100,
                Rank::Nine => 0x80,
                Rank::Eight => 0x40,
                Rank::Seven => 0x20,
                Rank::Six => 0x10,
                Rank::Five => 0x8,
                Rank::Four => 0x4,
                Rank::Trey => 0x2,
                Rank::Deuce => 0x1,
            };
        }
    }

    hash
}

const RANKS: [Rank; 13] = [
    Rank::Deuce,
    Rank::Trey,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
    Rank::Nine,
    Rank::Ten,
    Rank::Jack,
    Rank::Queen,
    Rank::King,
    Rank::Ace,
];

fn hash_for_rainbow(cards: &CardSet) -> u32 {
    let mut card_len_each_rank: [u8; 13] = [0; 13];
    let mut remaining_card_len: u8 = 0;

    for card in cards {
        let card_i: usize = card.rank().into();

        card_len_each_rank[card_i] += 1;
        remaining_card_len += 1;
    }

    let mut hash: u32 = 0;

    for rank in RANKS {
        let rank_i: usize = rank.into();
        let len: u8 = card_len_each_rank[rank_i];

        if len == 0 {
            continue;
        }

        hash += dp_ref(len, &rank, remaining_card_len);

        remaining_card_len -= len;

        if remaining_card_len <= 0 {
            break;
        }
    }

    hash
}

const REF_ONE_A: [u32; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const REF_ONE_K: [u32; 10] = [1, 1, 1, 1, 1, 0, 0, 0, 0, 0];
const REF_ONE_Q: [u32; 10] = [1, 2, 3, 4, 5, 4, 3, 2, 1, 0];
const REF_ONE_J: [u32; 10] = [1, 3, 6, 10, 15, 18, 19, 18, 15, 10];
const REF_ONE_T: [u32; 10] = [1, 4, 10, 20, 35, 52, 68, 80, 85, 80];
const REF_ONE_9: [u32; 10] = [1, 5, 15, 35, 70, 121, 185, 255, 320, 365];
const REF_ONE_8: [u32; 10] = [1, 6, 21, 56, 126, 246, 426, 666, 951, 1246];
const REF_ONE_7: [u32; 10] = [1, 7, 28, 84, 210, 455, 875, 1520, 2415, 3535];
const REF_ONE_6: [u32; 10] = [1, 8, 36, 120, 330, 784, 1652, 3144, 5475, 8800];
const REF_ONE_5: [u32; 10] = [1, 9, 45, 165, 495, 1278, 2922, 6030, 11385, 19855];
const REF_ONE_4: [u32; 10] = [1, 10, 55, 220, 715, 1992, 4905, 10890, 22110, 41470];
const REF_ONE_3: [u32; 10] = [1, 11, 66, 286, 1001, 2992, 7887, 18722, 40612, 81367];
const REF_ONE_2: [u32; 10] = [1, 12, 78, 364, 1365, 4356, 12232, 30888, 71214, 151580];
const REF_TWO_A: [u32; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const REF_TWO_K: [u32; 10] = [1, 2, 2, 2, 2, 1, 0, 0, 0, 0];
const REF_TWO_Q: [u32; 10] = [1, 3, 5, 7, 9, 9, 7, 5, 3, 1];
const REF_TWO_J: [u32; 10] = [1, 4, 9, 16, 25, 33, 37, 37, 33, 25];
const REF_TWO_T: [u32; 10] = [1, 5, 14, 30, 55, 87, 120, 148, 165, 165];
const REF_TWO_9: [u32; 10] = [1, 6, 20, 50, 105, 191, 306, 440, 575, 685];
const REF_TWO_8: [u32; 10] = [1, 7, 27, 77, 182, 372, 672, 1092, 1617, 2197];
const REF_TWO_7: [u32; 10] = [1, 8, 35, 112, 294, 665, 1330, 2395, 3935, 5950];
const REF_TWO_6: [u32; 10] = [1, 9, 44, 156, 450, 1114, 2436, 4796, 8619, 14275];
const REF_TWO_5: [u32; 10] = [1, 10, 54, 210, 660, 1773, 4200, 8952, 17415, 31240];
const REF_TWO_4: [u32; 10] = [1, 11, 65, 275, 935, 2707, 6897, 15795, 33000, 63580];
const REF_TWO_3: [u32; 10] = [1, 12, 77, 352, 1287, 3993, 10879, 26609, 59334, 121979];
const REF_TWO_2: [u32; 10] = [1, 13, 90, 442, 1729, 5721, 16588, 43120, 102102, 222794];
const REF_THREE_A: [u32; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const REF_THREE_K: [u32; 10] = [1, 2, 3, 3, 3, 2, 1, 0, 0, 0];
const REF_THREE_Q: [u32; 10] = [1, 3, 6, 9, 12, 13, 12, 9, 6, 3];
const REF_THREE_J: [u32; 10] = [1, 4, 10, 19, 31, 43, 52, 55, 52, 43];
const REF_THREE_T: [u32; 10] = [1, 5, 15, 34, 65, 107, 155, 200, 233, 245];
const REF_THREE_9: [u32; 10] = [1, 6, 21, 55, 120, 226, 376, 561, 760, 940];
const REF_THREE_8: [u32; 10] = [1, 7, 28, 83, 203, 428, 798, 1338, 2043, 2863];
const REF_THREE_7: [u32; 10] = [1, 8, 36, 119, 322, 749, 1540, 2850, 4810, 7470];
const REF_THREE_6: [u32; 10] = [1, 9, 45, 164, 486, 1234, 2766, 5580, 10271, 17419];
const REF_THREE_5: [u32; 10] = [1, 10, 55, 219, 705, 1938, 4695, 10230, 20337, 37270];
const REF_THREE_4: [u32; 10] = [1, 11, 66, 285, 990, 2927, 7612, 17787, 37905, 74470];
const REF_THREE_3: [u32; 10] = [1, 12, 78, 363, 1353, 4279, 11880, 29601, 67221, 140701];
const REF_THREE_2: [u32; 10] = [1, 13, 91, 454, 1807, 6085, 17953, 47476, 114334, 253682];
const REF_FOUR_A: [u32; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const REF_FOUR_K: [u32; 10] = [1, 2, 3, 4, 4, 3, 2, 1, 0, 0];
const REF_FOUR_Q: [u32; 10] = [1, 3, 6, 10, 14, 16, 16, 14, 10, 6];
const REF_FOUR_J: [u32; 10] = [1, 4, 10, 20, 34, 49, 62, 70, 70, 62];
const REF_FOUR_T: [u32; 10] = [1, 5, 15, 35, 69, 117, 175, 235, 285, 313];
const REF_FOUR_9: [u32; 10] = [1, 6, 21, 56, 125, 241, 411, 631, 881, 1125];
const REF_FOUR_8: [u32; 10] = [1, 7, 28, 84, 209, 449, 854, 1464, 2289, 3289];
const REF_FOUR_7: [u32; 10] = [1, 8, 36, 120, 329, 777, 1624, 3060, 5265, 8345];
const REF_FOUR_6: [u32; 10] = [1, 9, 45, 165, 494, 1270, 2886, 5910, 11055, 19071];
const REF_FOUR_5: [u32; 10] = [1, 10, 55, 220, 714, 1983, 4860, 10725, 21615, 40192];
const REF_FOUR_4: [u32; 10] = [1, 11, 66, 286, 1000, 2982, 7832, 18502, 39897, 79375];
const REF_FOUR_3: [u32; 10] = [1, 12, 78, 364, 1364, 4345, 12166, 30602, 70213, 148588];
const REF_FOUR_2: [u32; 10] = [1, 13, 91, 455, 1819, 6163, 18317, 48841, 118690, 265914];

fn dp_ref(len: u8, rank: &Rank, remaining_len: u8) -> u32 {
    match len {
        1 => match rank {
            Rank::Ace => REF_ONE_A[remaining_len as usize],
            Rank::King => REF_ONE_K[remaining_len as usize],
            Rank::Queen => REF_ONE_Q[remaining_len as usize],
            Rank::Jack => REF_ONE_J[remaining_len as usize],
            Rank::Ten => REF_ONE_T[remaining_len as usize],
            Rank::Nine => REF_ONE_9[remaining_len as usize],
            Rank::Eight => REF_ONE_8[remaining_len as usize],
            Rank::Seven => REF_ONE_7[remaining_len as usize],
            Rank::Six => REF_ONE_6[remaining_len as usize],
            Rank::Five => REF_ONE_5[remaining_len as usize],
            Rank::Four => REF_ONE_4[remaining_len as usize],
            Rank::Trey => REF_ONE_3[remaining_len as usize],
            Rank::Deuce => REF_ONE_2[remaining_len as usize],
        },
        2 => match rank {
            Rank::Ace => REF_TWO_A[remaining_len as usize],
            Rank::King => REF_TWO_K[remaining_len as usize],
            Rank::Queen => REF_TWO_Q[remaining_len as usize],
            Rank::Jack => REF_TWO_J[remaining_len as usize],
            Rank::Ten => REF_TWO_T[remaining_len as usize],
            Rank::Nine => REF_TWO_9[remaining_len as usize],
            Rank::Eight => REF_TWO_8[remaining_len as usize],
            Rank::Seven => REF_TWO_7[remaining_len as usize],
            Rank::Six => REF_TWO_6[remaining_len as usize],
            Rank::Five => REF_TWO_5[remaining_len as usize],
            Rank::Four => REF_TWO_4[remaining_len as usize],
            Rank::Trey => REF_TWO_3[remaining_len as usize],
            Rank::Deuce => REF_TWO_2[remaining_len as usize],
        },
        3 => match rank {
            Rank::Ace => REF_THREE_A[remaining_len as usize],
            Rank::King => REF_THREE_K[remaining_len as usize],
            Rank::Queen => REF_THREE_Q[remaining_len as usize],
            Rank::Jack => REF_THREE_J[remaining_len as usize],
            Rank::Ten => REF_THREE_T[remaining_len as usize],
            Rank::Nine => REF_THREE_9[remaining_len as usize],
            Rank::Eight => REF_THREE_8[remaining_len as usize],
            Rank::Seven => REF_THREE_7[remaining_len as usize],
            Rank::Six => REF_THREE_6[remaining_len as usize],
            Rank::Five => REF_THREE_5[remaining_len as usize],
            Rank::Four => REF_THREE_4[remaining_len as usize],
            Rank::Trey => REF_THREE_3[remaining_len as usize],
            Rank::Deuce => REF_THREE_2[remaining_len as usize],
        },
        4 => match rank {
            Rank::Ace => REF_FOUR_A[remaining_len as usize],
            Rank::King => REF_FOUR_K[remaining_len as usize],
            Rank::Queen => REF_FOUR_Q[remaining_len as usize],
            Rank::Jack => REF_FOUR_J[remaining_len as usize],
            Rank::Ten => REF_FOUR_T[remaining_len as usize],
            Rank::Nine => REF_FOUR_9[remaining_len as usize],
            Rank::Eight => REF_FOUR_8[remaining_len as usize],
            Rank::Seven => REF_FOUR_7[remaining_len as usize],
            Rank::Six => REF_FOUR_6[remaining_len as usize],
            Rank::Five => REF_FOUR_5[remaining_len as usize],
            Rank::Four => REF_FOUR_4[remaining_len as usize],
            Rank::Trey => REF_FOUR_3[remaining_len as usize],
            Rank::Deuce => REF_FOUR_2[remaining_len as usize],
        },
        _ => panic!("Invalid length"),
    }
}

const AS_FLUSH: [u16; 8192] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1599, 0, 0, 0, 0, 0, 0, 0, 1598, 0, 0, 0, 1597, 0,
    1596, 8, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1595, 0, 0, 0, 0, 0, 0, 0, 1594, 0, 0,
    0, 1593, 0, 1592, 1591, 9, 0, 0, 0, 0, 0, 0, 0, 1590, 0, 0, 0, 1589, 0, 1588, 1587, 1587, 0, 0,
    0, 1586, 0, 1585, 1584, 1584, 0, 1583, 1582, 1582, 7, 7, 7, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 1581, 0, 0, 0, 0, 0, 0, 0, 1580, 0, 0, 0, 1579, 0, 1578, 1577, 9, 0, 0, 0, 0, 0, 0,
    0, 1576, 0, 0, 0, 1575, 0, 1574, 1573, 1573, 0, 0, 0, 1572, 0, 1571, 1570, 1570, 0, 1569, 1568,
    1568, 1567, 1567, 8, 8, 0, 0, 0, 0, 0, 0, 0, 1566, 0, 0, 0, 1565, 0, 1564, 1563, 1563, 0, 0, 0,
    1562, 0, 1561, 1560, 1560, 0, 1559, 1558, 1558, 1557, 1557, 1557, 9, 0, 0, 0, 1556, 0, 1555,
    1554, 1554, 0, 1553, 1552, 1552, 1551, 1551, 1551, 1551, 0, 1550, 1549, 1549, 1548, 1548, 1548,
    1548, 6, 6, 6, 6, 6, 6, 6, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1547, 0, 0, 0, 0, 0,
    0, 0, 1546, 0, 0, 0, 1545, 0, 1544, 1543, 9, 0, 0, 0, 0, 0, 0, 0, 1542, 0, 0, 0, 1541, 0, 1540,
    1539, 1539, 0, 0, 0, 1538, 0, 1537, 1536, 1536, 0, 1535, 1534, 1534, 1533, 1533, 8, 8, 0, 0, 0,
    0, 0, 0, 0, 1532, 0, 0, 0, 1531, 0, 1530, 1529, 1529, 0, 0, 0, 1528, 0, 1527, 1526, 1526, 0,
    1525, 1524, 1524, 1523, 1523, 1523, 9, 0, 0, 0, 1522, 0, 1521, 1520, 1520, 0, 1519, 1518, 1518,
    1517, 1517, 1517, 1517, 0, 1516, 1515, 1515, 1514, 1514, 1514, 1514, 1513, 1513, 1513, 1513, 7,
    7, 7, 7, 0, 0, 0, 0, 0, 0, 0, 1512, 0, 0, 0, 1511, 0, 1510, 1509, 1509, 0, 0, 0, 1508, 0, 1507,
    1506, 1506, 0, 1505, 1504, 1504, 1503, 1503, 1503, 9, 0, 0, 0, 1502, 0, 1501, 1500, 1500, 0,
    1499, 1498, 1498, 1497, 1497, 1497, 1497, 0, 1496, 1495, 1495, 1494, 1494, 1494, 1494, 1493,
    1493, 1493, 1493, 1493, 1493, 8, 8, 0, 0, 0, 1492, 0, 1491, 1490, 1490, 0, 1489, 1488, 1488,
    1487, 1487, 1487, 1487, 0, 1486, 1485, 1485, 1484, 1484, 1484, 1484, 1483, 1483, 1483, 1483,
    1483, 1483, 1483, 9, 0, 1482, 1481, 1481, 1480, 1480, 1480, 1480, 1479, 1479, 1479, 1479, 1479,
    1479, 1479, 1479, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 1478, 0, 0, 0, 0, 0, 0, 0, 1477, 0, 0, 0, 1476, 0, 1475, 1474, 9, 0, 0, 0, 0, 0,
    0, 0, 1473, 0, 0, 0, 1472, 0, 1471, 1470, 1470, 0, 0, 0, 1469, 0, 1468, 1467, 1467, 0, 1466,
    1465, 1465, 1464, 1464, 8, 8, 0, 0, 0, 0, 0, 0, 0, 1463, 0, 0, 0, 1462, 0, 1461, 1460, 1460, 0,
    0, 0, 1459, 0, 1458, 1457, 1457, 0, 1456, 1455, 1455, 1454, 1454, 1454, 9, 0, 0, 0, 1453, 0,
    1452, 1451, 1451, 0, 1450, 1449, 1449, 1448, 1448, 1448, 1448, 0, 1447, 1446, 1446, 1445, 1445,
    1445, 1445, 1444, 1444, 1444, 1444, 7, 7, 7, 7, 0, 0, 0, 0, 0, 0, 0, 1443, 0, 0, 0, 1442, 0,
    1441, 1440, 1440, 0, 0, 0, 1439, 0, 1438, 1437, 1437, 0, 1436, 1435, 1435, 1434, 1434, 1434, 9,
    0, 0, 0, 1433, 0, 1432, 1431, 1431, 0, 1430, 1429, 1429, 1428, 1428, 1428, 1428, 0, 1427, 1426,
    1426, 1425, 1425, 1425, 1425, 1424, 1424, 1424, 1424, 1424, 1424, 8, 8, 0, 0, 0, 1423, 0, 1422,
    1421, 1421, 0, 1420, 1419, 1419, 1418, 1418, 1418, 1418, 0, 1417, 1416, 1416, 1415, 1415, 1415,
    1415, 1414, 1414, 1414, 1414, 1414, 1414, 1414, 9, 0, 1413, 1412, 1412, 1411, 1411, 1411, 1411,
    1410, 1410, 1410, 1410, 1410, 1410, 1410, 1410, 1409, 1409, 1409, 1409, 1409, 1409, 1409, 1409,
    6, 6, 6, 6, 6, 6, 6, 6, 0, 0, 0, 0, 0, 0, 0, 1408, 0, 0, 0, 1407, 0, 1406, 1405, 1405, 0, 0, 0,
    1404, 0, 1403, 1402, 1402, 0, 1401, 1400, 1400, 1399, 1399, 1399, 9, 0, 0, 0, 1398, 0, 1397,
    1396, 1396, 0, 1395, 1394, 1394, 1393, 1393, 1393, 1393, 0, 1392, 1391, 1391, 1390, 1390, 1390,
    1390, 1389, 1389, 1389, 1389, 1389, 1389, 8, 8, 0, 0, 0, 1388, 0, 1387, 1386, 1386, 0, 1385,
    1384, 1384, 1383, 1383, 1383, 1383, 0, 1382, 1381, 1381, 1380, 1380, 1380, 1380, 1379, 1379,
    1379, 1379, 1379, 1379, 1379, 9, 0, 1378, 1377, 1377, 1376, 1376, 1376, 1376, 1375, 1375, 1375,
    1375, 1375, 1375, 1375, 1375, 1374, 1374, 1374, 1374, 1374, 1374, 1374, 1374, 1374, 1374, 1374,
    1374, 7, 7, 7, 7, 0, 0, 0, 1373, 0, 1372, 1371, 1371, 0, 1370, 1369, 1369, 1368, 1368, 1368,
    1368, 0, 1367, 1366, 1366, 1365, 1365, 1365, 1365, 1364, 1364, 1364, 1364, 1364, 1364, 1364, 9,
    0, 1363, 1362, 1362, 1361, 1361, 1361, 1361, 1360, 1360, 1360, 1360, 1360, 1360, 1360, 1360,
    1359, 1359, 1359, 1359, 1359, 1359, 1359, 1359, 1359, 1359, 1359, 1359, 1359, 1359, 8, 8, 0,
    1358, 1357, 1357, 1356, 1356, 1356, 1356, 1355, 1355, 1355, 1355, 1355, 1355, 1355, 1355, 1354,
    1354, 1354, 1354, 1354, 1354, 1354, 1354, 1354, 1354, 1354, 1354, 1354, 1354, 1354, 9, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1353, 0, 0, 0, 0, 0, 0, 0, 1352, 0, 0, 0, 1351, 0, 1350,
    1349, 9, 0, 0, 0, 0, 0, 0, 0, 1348, 0, 0, 0, 1347, 0, 1346, 1345, 1345, 0, 0, 0, 1344, 0, 1343,
    1342, 1342, 0, 1341, 1340, 1340, 1339, 1339, 8, 8, 0, 0, 0, 0, 0, 0, 0, 1338, 0, 0, 0, 1337, 0,
    1336, 1335, 1335, 0, 0, 0, 1334, 0, 1333, 1332, 1332, 0, 1331, 1330, 1330, 1329, 1329, 1329, 9,
    0, 0, 0, 1328, 0, 1327, 1326, 1326, 0, 1325, 1324, 1324, 1323, 1323, 1323, 1323, 0, 1322, 1321,
    1321, 1320, 1320, 1320, 1320, 1319, 1319, 1319, 1319, 7, 7, 7, 7, 0, 0, 0, 0, 0, 0, 0, 1318, 0,
    0, 0, 1317, 0, 1316, 1315, 1315, 0, 0, 0, 1314, 0, 1313, 1312, 1312, 0, 1311, 1310, 1310, 1309,
    1309, 1309, 9, 0, 0, 0, 1308, 0, 1307, 1306, 1306, 0, 1305, 1304, 1304, 1303, 1303, 1303, 1303,
    0, 1302, 1301, 1301, 1300, 1300, 1300, 1300, 1299, 1299, 1299, 1299, 1299, 1299, 8, 8, 0, 0, 0,
    1298, 0, 1297, 1296, 1296, 0, 1295, 1294, 1294, 1293, 1293, 1293, 1293, 0, 1292, 1291, 1291,
    1290, 1290, 1290, 1290, 1289, 1289, 1289, 1289, 1289, 1289, 1289, 9, 0, 1288, 1287, 1287, 1286,
    1286, 1286, 1286, 1285, 1285, 1285, 1285, 1285, 1285, 1285, 1285, 1284, 1284, 1284, 1284, 1284,
    1284, 1284, 1284, 6, 6, 6, 6, 6, 6, 6, 6, 0, 0, 0, 0, 0, 0, 0, 1283, 0, 0, 0, 1282, 0, 1281,
    1280, 1280, 0, 0, 0, 1279, 0, 1278, 1277, 1277, 0, 1276, 1275, 1275, 1274, 1274, 1274, 9, 0, 0,
    0, 1273, 0, 1272, 1271, 1271, 0, 1270, 1269, 1269, 1268, 1268, 1268, 1268, 0, 1267, 1266, 1266,
    1265, 1265, 1265, 1265, 1264, 1264, 1264, 1264, 1264, 1264, 8, 8, 0, 0, 0, 1263, 0, 1262, 1261,
    1261, 0, 1260, 1259, 1259, 1258, 1258, 1258, 1258, 0, 1257, 1256, 1256, 1255, 1255, 1255, 1255,
    1254, 1254, 1254, 1254, 1254, 1254, 1254, 9, 0, 1253, 1252, 1252, 1251, 1251, 1251, 1251, 1250,
    1250, 1250, 1250, 1250, 1250, 1250, 1250, 1249, 1249, 1249, 1249, 1249, 1249, 1249, 1249, 1249,
    1249, 1249, 1249, 7, 7, 7, 7, 0, 0, 0, 1248, 0, 1247, 1246, 1246, 0, 1245, 1244, 1244, 1243,
    1243, 1243, 1243, 0, 1242, 1241, 1241, 1240, 1240, 1240, 1240, 1239, 1239, 1239, 1239, 1239,
    1239, 1239, 9, 0, 1238, 1237, 1237, 1236, 1236, 1236, 1236, 1235, 1235, 1235, 1235, 1235, 1235,
    1235, 1235, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234,
    8, 8, 0, 1233, 1232, 1232, 1231, 1231, 1231, 1231, 1230, 1230, 1230, 1230, 1230, 1230, 1230,
    1230, 1229, 1229, 1229, 1229, 1229, 1229, 1229, 1229, 1229, 1229, 1229, 1229, 1229, 1229, 1229,
    9, 1228, 1228, 1228, 1228, 1228, 1228, 1228, 1228, 1228, 1228, 1228, 1228, 1228, 1228, 1228,
    1228, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 1227, 0, 0, 0, 1226,
    0, 1225, 1224, 1224, 0, 0, 0, 1223, 0, 1222, 1221, 1221, 0, 1220, 1219, 1219, 1218, 1218, 1218,
    9, 0, 0, 0, 1217, 0, 1216, 1215, 1215, 0, 1214, 1213, 1213, 1212, 1212, 1212, 1212, 0, 1211,
    1210, 1210, 1209, 1209, 1209, 1209, 1208, 1208, 1208, 1208, 1208, 1208, 8, 8, 0, 0, 0, 1207, 0,
    1206, 1205, 1205, 0, 1204, 1203, 1203, 1202, 1202, 1202, 1202, 0, 1201, 1200, 1200, 1199, 1199,
    1199, 1199, 1198, 1198, 1198, 1198, 1198, 1198, 1198, 9, 0, 1197, 1196, 1196, 1195, 1195, 1195,
    1195, 1194, 1194, 1194, 1194, 1194, 1194, 1194, 1194, 1193, 1193, 1193, 1193, 1193, 1193, 1193,
    1193, 1193, 1193, 1193, 1193, 7, 7, 7, 7, 0, 0, 0, 1192, 0, 1191, 1190, 1190, 0, 1189, 1188,
    1188, 1187, 1187, 1187, 1187, 0, 1186, 1185, 1185, 1184, 1184, 1184, 1184, 1183, 1183, 1183,
    1183, 1183, 1183, 1183, 9, 0, 1182, 1181, 1181, 1180, 1180, 1180, 1180, 1179, 1179, 1179, 1179,
    1179, 1179, 1179, 1179, 1178, 1178, 1178, 1178, 1178, 1178, 1178, 1178, 1178, 1178, 1178, 1178,
    1178, 1178, 8, 8, 0, 1177, 1176, 1176, 1175, 1175, 1175, 1175, 1174, 1174, 1174, 1174, 1174,
    1174, 1174, 1174, 1173, 1173, 1173, 1173, 1173, 1173, 1173, 1173, 1173, 1173, 1173, 1173, 1173,
    1173, 1173, 9, 1172, 1172, 1172, 1172, 1172, 1172, 1172, 1172, 1172, 1172, 1172, 1172, 1172,
    1172, 1172, 1172, 1172, 1172, 1172, 1172, 1172, 1172, 1172, 1172, 6, 6, 6, 6, 6, 6, 6, 0, 0, 0,
    0, 1171, 0, 1170, 1169, 1169, 0, 1168, 1167, 1167, 1166, 1166, 1166, 1166, 0, 1165, 1164, 1164,
    1163, 1163, 1163, 1163, 1162, 1162, 1162, 1162, 1162, 1162, 1162, 9, 0, 1161, 1160, 1160, 1159,
    1159, 1159, 1159, 1158, 1158, 1158, 1158, 1158, 1158, 1158, 1158, 1157, 1157, 1157, 1157, 1157,
    1157, 1157, 1157, 1157, 1157, 1157, 1157, 1157, 1157, 8, 8, 0, 1156, 1155, 1155, 1154, 1154,
    1154, 1154, 1153, 1153, 1153, 1153, 1153, 1153, 1153, 1153, 1152, 1152, 1152, 1152, 1152, 1152,
    1152, 1152, 1152, 1152, 1152, 1152, 1152, 1152, 1152, 9, 1151, 1151, 1151, 1151, 1151, 1151,
    1151, 1151, 1151, 1151, 1151, 1151, 1151, 1151, 1151, 1151, 1151, 1151, 1151, 1151, 1151, 1151,
    1151, 1151, 1151, 1151, 1151, 1151, 7, 7, 7, 0, 0, 1150, 1149, 1149, 1148, 1148, 1148, 1148,
    1147, 1147, 1147, 1147, 1147, 1147, 1147, 1147, 1146, 1146, 1146, 1146, 1146, 1146, 1146, 1146,
    1146, 1146, 1146, 1146, 1146, 1146, 1146, 9, 1145, 1145, 1145, 1145, 1145, 1145, 1145, 1145,
    1145, 1145, 1145, 1145, 1145, 1145, 1145, 1145, 1145, 1145, 1145, 1145, 1145, 1145, 1145, 1145,
    1145, 1145, 1145, 1145, 1145, 1145, 8, 0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0, 3, 3,
    3, 3, 3, 3, 3, 0, 3, 3, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1144, 0,
    0, 0, 0, 0, 0, 0, 1143, 0, 0, 0, 1142, 0, 1141, 1140, 9, 0, 0, 0, 0, 0, 0, 0, 1139, 0, 0, 0,
    1138, 0, 1137, 1136, 1136, 0, 0, 0, 1135, 0, 1134, 1133, 1133, 0, 1132, 1131, 1131, 1130, 1130,
    8, 8, 0, 0, 0, 0, 0, 0, 0, 1129, 0, 0, 0, 1128, 0, 1127, 1126, 1126, 0, 0, 0, 1125, 0, 1124,
    1123, 1123, 0, 1122, 1121, 1121, 1120, 1120, 1120, 9, 0, 0, 0, 1119, 0, 1118, 1117, 1117, 0,
    1116, 1115, 1115, 1114, 1114, 1114, 1114, 0, 1113, 1112, 1112, 1111, 1111, 1111, 1111, 1110,
    1110, 1110, 1110, 7, 7, 7, 7, 0, 0, 0, 0, 0, 0, 0, 1109, 0, 0, 0, 1108, 0, 1107, 1106, 1106, 0,
    0, 0, 1105, 0, 1104, 1103, 1103, 0, 1102, 1101, 1101, 1100, 1100, 1100, 9, 0, 0, 0, 1099, 0,
    1098, 1097, 1097, 0, 1096, 1095, 1095, 1094, 1094, 1094, 1094, 0, 1093, 1092, 1092, 1091, 1091,
    1091, 1091, 1090, 1090, 1090, 1090, 1090, 1090, 8, 8, 0, 0, 0, 1089, 0, 1088, 1087, 1087, 0,
    1086, 1085, 1085, 1084, 1084, 1084, 1084, 0, 1083, 1082, 1082, 1081, 1081, 1081, 1081, 1080,
    1080, 1080, 1080, 1080, 1080, 1080, 9, 0, 1079, 1078, 1078, 1077, 1077, 1077, 1077, 1076, 1076,
    1076, 1076, 1076, 1076, 1076, 1076, 1075, 1075, 1075, 1075, 1075, 1075, 1075, 1075, 6, 6, 6, 6,
    6, 6, 6, 6, 0, 0, 0, 0, 0, 0, 0, 1074, 0, 0, 0, 1073, 0, 1072, 1071, 1071, 0, 0, 0, 1070, 0,
    1069, 1068, 1068, 0, 1067, 1066, 1066, 1065, 1065, 1065, 9, 0, 0, 0, 1064, 0, 1063, 1062, 1062,
    0, 1061, 1060, 1060, 1059, 1059, 1059, 1059, 0, 1058, 1057, 1057, 1056, 1056, 1056, 1056, 1055,
    1055, 1055, 1055, 1055, 1055, 8, 8, 0, 0, 0, 1054, 0, 1053, 1052, 1052, 0, 1051, 1050, 1050,
    1049, 1049, 1049, 1049, 0, 1048, 1047, 1047, 1046, 1046, 1046, 1046, 1045, 1045, 1045, 1045,
    1045, 1045, 1045, 9, 0, 1044, 1043, 1043, 1042, 1042, 1042, 1042, 1041, 1041, 1041, 1041, 1041,
    1041, 1041, 1041, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 7, 7,
    7, 7, 0, 0, 0, 1039, 0, 1038, 1037, 1037, 0, 1036, 1035, 1035, 1034, 1034, 1034, 1034, 0, 1033,
    1032, 1032, 1031, 1031, 1031, 1031, 1030, 1030, 1030, 1030, 1030, 1030, 1030, 9, 0, 1029, 1028,
    1028, 1027, 1027, 1027, 1027, 1026, 1026, 1026, 1026, 1026, 1026, 1026, 1026, 1025, 1025, 1025,
    1025, 1025, 1025, 1025, 1025, 1025, 1025, 1025, 1025, 1025, 1025, 8, 8, 0, 1024, 1023, 1023,
    1022, 1022, 1022, 1022, 1021, 1021, 1021, 1021, 1021, 1021, 1021, 1021, 1020, 1020, 1020, 1020,
    1020, 1020, 1020, 1020, 1020, 1020, 1020, 1020, 1020, 1020, 1020, 9, 1019, 1019, 1019, 1019,
    1019, 1019, 1019, 1019, 1019, 1019, 1019, 1019, 1019, 1019, 1019, 1019, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 1018, 0, 0, 0, 1017, 0, 1016, 1015, 1015, 0, 0, 0,
    1014, 0, 1013, 1012, 1012, 0, 1011, 1010, 1010, 1009, 1009, 1009, 9, 0, 0, 0, 1008, 0, 1007,
    1006, 1006, 0, 1005, 1004, 1004, 1003, 1003, 1003, 1003, 0, 1002, 1001, 1001, 1000, 1000, 1000,
    1000, 999, 999, 999, 999, 999, 999, 8, 8, 0, 0, 0, 998, 0, 997, 996, 996, 0, 995, 994, 994,
    993, 993, 993, 993, 0, 992, 991, 991, 990, 990, 990, 990, 989, 989, 989, 989, 989, 989, 989, 9,
    0, 988, 987, 987, 986, 986, 986, 986, 985, 985, 985, 985, 985, 985, 985, 985, 984, 984, 984,
    984, 984, 984, 984, 984, 984, 984, 984, 984, 7, 7, 7, 7, 0, 0, 0, 983, 0, 982, 981, 981, 0,
    980, 979, 979, 978, 978, 978, 978, 0, 977, 976, 976, 975, 975, 975, 975, 974, 974, 974, 974,
    974, 974, 974, 9, 0, 973, 972, 972, 971, 971, 971, 971, 970, 970, 970, 970, 970, 970, 970, 970,
    969, 969, 969, 969, 969, 969, 969, 969, 969, 969, 969, 969, 969, 969, 8, 8, 0, 968, 967, 967,
    966, 966, 966, 966, 965, 965, 965, 965, 965, 965, 965, 965, 964, 964, 964, 964, 964, 964, 964,
    964, 964, 964, 964, 964, 964, 964, 964, 9, 963, 963, 963, 963, 963, 963, 963, 963, 963, 963,
    963, 963, 963, 963, 963, 963, 963, 963, 963, 963, 963, 963, 963, 963, 6, 6, 6, 6, 6, 6, 6, 0,
    0, 0, 0, 962, 0, 961, 960, 960, 0, 959, 958, 958, 957, 957, 957, 957, 0, 956, 955, 955, 954,
    954, 954, 954, 953, 953, 953, 953, 953, 953, 953, 9, 0, 952, 951, 951, 950, 950, 950, 950, 949,
    949, 949, 949, 949, 949, 949, 949, 948, 948, 948, 948, 948, 948, 948, 948, 948, 948, 948, 948,
    948, 948, 8, 8, 0, 947, 946, 946, 945, 945, 945, 945, 944, 944, 944, 944, 944, 944, 944, 944,
    943, 943, 943, 943, 943, 943, 943, 943, 943, 943, 943, 943, 943, 943, 943, 9, 942, 942, 942,
    942, 942, 942, 942, 942, 942, 942, 942, 942, 942, 942, 942, 942, 942, 942, 942, 942, 942, 942,
    942, 942, 942, 942, 942, 942, 7, 7, 7, 0, 0, 941, 940, 940, 939, 939, 939, 939, 938, 938, 938,
    938, 938, 938, 938, 938, 937, 937, 937, 937, 937, 937, 937, 937, 937, 937, 937, 937, 937, 937,
    937, 9, 936, 936, 936, 936, 936, 936, 936, 936, 936, 936, 936, 936, 936, 936, 936, 936, 936,
    936, 936, 936, 936, 936, 936, 936, 936, 936, 936, 936, 936, 936, 8, 0, 935, 935, 935, 935, 935,
    935, 935, 935, 935, 935, 935, 935, 935, 935, 935, 935, 935, 935, 935, 935, 935, 935, 935, 935,
    935, 935, 935, 935, 935, 935, 935, 0, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 0, 4, 4, 4,
    4, 4, 4, 4, 0, 4, 4, 4, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 934, 0, 0, 0, 933, 0, 932, 931,
    931, 0, 0, 0, 930, 0, 929, 928, 928, 0, 927, 926, 926, 925, 925, 925, 9, 0, 0, 0, 924, 0, 923,
    922, 922, 0, 921, 920, 920, 919, 919, 919, 919, 0, 918, 917, 917, 916, 916, 916, 916, 915, 915,
    915, 915, 915, 915, 8, 8, 0, 0, 0, 914, 0, 913, 912, 912, 0, 911, 910, 910, 909, 909, 909, 909,
    0, 908, 907, 907, 906, 906, 906, 906, 905, 905, 905, 905, 905, 905, 905, 9, 0, 904, 903, 903,
    902, 902, 902, 902, 901, 901, 901, 901, 901, 901, 901, 901, 900, 900, 900, 900, 900, 900, 900,
    900, 900, 900, 900, 900, 7, 7, 7, 7, 0, 0, 0, 899, 0, 898, 897, 897, 0, 896, 895, 895, 894,
    894, 894, 894, 0, 893, 892, 892, 891, 891, 891, 891, 890, 890, 890, 890, 890, 890, 890, 9, 0,
    889, 888, 888, 887, 887, 887, 887, 886, 886, 886, 886, 886, 886, 886, 886, 885, 885, 885, 885,
    885, 885, 885, 885, 885, 885, 885, 885, 885, 885, 8, 8, 0, 884, 883, 883, 882, 882, 882, 882,
    881, 881, 881, 881, 881, 881, 881, 881, 880, 880, 880, 880, 880, 880, 880, 880, 880, 880, 880,
    880, 880, 880, 880, 9, 879, 879, 879, 879, 879, 879, 879, 879, 879, 879, 879, 879, 879, 879,
    879, 879, 879, 879, 879, 879, 879, 879, 879, 879, 6, 6, 6, 6, 6, 6, 6, 0, 0, 0, 0, 878, 0, 877,
    876, 876, 0, 875, 874, 874, 873, 873, 873, 873, 0, 872, 871, 871, 870, 870, 870, 870, 869, 869,
    869, 869, 869, 869, 869, 9, 0, 868, 867, 867, 866, 866, 866, 866, 865, 865, 865, 865, 865, 865,
    865, 865, 864, 864, 864, 864, 864, 864, 864, 864, 864, 864, 864, 864, 864, 864, 8, 8, 0, 863,
    862, 862, 861, 861, 861, 861, 860, 860, 860, 860, 860, 860, 860, 860, 859, 859, 859, 859, 859,
    859, 859, 859, 859, 859, 859, 859, 859, 859, 859, 9, 858, 858, 858, 858, 858, 858, 858, 858,
    858, 858, 858, 858, 858, 858, 858, 858, 858, 858, 858, 858, 858, 858, 858, 858, 858, 858, 858,
    858, 7, 7, 7, 0, 0, 857, 856, 856, 855, 855, 855, 855, 854, 854, 854, 854, 854, 854, 854, 854,
    853, 853, 853, 853, 853, 853, 853, 853, 853, 853, 853, 853, 853, 853, 853, 9, 852, 852, 852,
    852, 852, 852, 852, 852, 852, 852, 852, 852, 852, 852, 852, 852, 852, 852, 852, 852, 852, 852,
    852, 852, 852, 852, 852, 852, 852, 852, 8, 0, 851, 851, 851, 851, 851, 851, 851, 851, 851, 851,
    851, 851, 851, 851, 851, 851, 851, 851, 851, 851, 851, 851, 851, 851, 851, 851, 851, 851, 851,
    851, 851, 0, 851, 851, 851, 851, 851, 851, 851, 851, 851, 851, 851, 851, 851, 851, 851, 0, 5,
    5, 5, 5, 5, 5, 5, 0, 5, 5, 5, 0, 5, 0, 0, 0, 0, 0, 0, 850, 0, 849, 848, 848, 0, 847, 846, 846,
    845, 845, 845, 845, 0, 844, 843, 843, 842, 842, 842, 842, 841, 841, 841, 841, 841, 841, 841, 9,
    0, 840, 839, 839, 838, 838, 838, 838, 837, 837, 837, 837, 837, 837, 837, 837, 836, 836, 836,
    836, 836, 836, 836, 836, 836, 836, 836, 836, 836, 836, 8, 8, 0, 835, 834, 834, 833, 833, 833,
    833, 832, 832, 832, 832, 832, 832, 832, 832, 831, 831, 831, 831, 831, 831, 831, 831, 831, 831,
    831, 831, 831, 831, 831, 9, 830, 830, 830, 830, 830, 830, 830, 830, 830, 830, 830, 830, 830,
    830, 830, 830, 830, 830, 830, 830, 830, 830, 830, 830, 830, 830, 830, 830, 7, 7, 7, 0, 0, 829,
    828, 828, 827, 827, 827, 827, 826, 826, 826, 826, 826, 826, 826, 826, 825, 825, 825, 825, 825,
    825, 825, 825, 825, 825, 825, 825, 825, 825, 825, 9, 824, 824, 824, 824, 824, 824, 824, 824,
    824, 824, 824, 824, 824, 824, 824, 824, 824, 824, 824, 824, 824, 824, 824, 824, 824, 824, 824,
    824, 824, 824, 8, 0, 823, 823, 823, 823, 823, 823, 823, 823, 823, 823, 823, 823, 823, 823, 823,
    823, 823, 823, 823, 823, 823, 823, 823, 823, 823, 823, 823, 823, 823, 823, 823, 0, 823, 823,
    823, 823, 823, 823, 823, 823, 823, 823, 823, 823, 823, 823, 823, 0, 823, 823, 823, 823, 823,
    823, 823, 0, 6, 6, 6, 0, 6, 0, 0, 0, 0, 822, 821, 821, 820, 820, 820, 820, 819, 819, 819, 819,
    819, 819, 819, 819, 818, 818, 818, 818, 818, 818, 818, 818, 818, 818, 818, 818, 818, 818, 818,
    9, 817, 817, 817, 817, 817, 817, 817, 817, 817, 817, 817, 817, 817, 817, 817, 817, 817, 817,
    817, 817, 817, 817, 817, 817, 817, 817, 817, 817, 817, 817, 8, 0, 816, 816, 816, 816, 816, 816,
    816, 816, 816, 816, 816, 816, 816, 816, 816, 816, 816, 816, 816, 816, 816, 816, 816, 816, 816,
    816, 816, 816, 816, 816, 816, 0, 816, 816, 816, 816, 816, 816, 816, 816, 816, 816, 816, 816,
    816, 816, 816, 0, 816, 816, 816, 816, 816, 816, 816, 0, 816, 816, 816, 0, 7, 0, 0, 0, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 0, 2, 0, 0, 0, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 0, 2, 0, 0, 0, 2, 2, 2,
    2, 2, 2, 2, 0, 2, 2, 2, 0, 2, 0, 0, 0, 2, 2, 2, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 815, 0, 0, 0, 814, 0, 813, 812, 9,
    0, 0, 0, 0, 0, 0, 0, 811, 0, 0, 0, 810, 0, 809, 808, 10, 0, 0, 0, 807, 0, 806, 805, 805, 0,
    804, 803, 803, 802, 802, 8, 8, 0, 0, 0, 0, 0, 0, 0, 801, 0, 0, 0, 800, 0, 799, 798, 10, 0, 0,
    0, 797, 0, 796, 795, 795, 0, 794, 793, 793, 792, 792, 792, 9, 0, 0, 0, 791, 0, 790, 789, 789,
    0, 788, 787, 787, 786, 786, 786, 10, 0, 785, 784, 784, 783, 783, 783, 783, 782, 782, 782, 782,
    7, 7, 7, 7, 0, 0, 0, 0, 0, 0, 0, 781, 0, 0, 0, 780, 0, 779, 778, 10, 0, 0, 0, 777, 0, 776, 775,
    775, 0, 774, 773, 773, 772, 772, 772, 9, 0, 0, 0, 771, 0, 770, 769, 769, 0, 768, 767, 767, 766,
    766, 766, 10, 0, 765, 764, 764, 763, 763, 763, 763, 762, 762, 762, 762, 762, 762, 8, 8, 0, 0,
    0, 761, 0, 760, 759, 759, 0, 758, 757, 757, 756, 756, 756, 10, 0, 755, 754, 754, 753, 753, 753,
    753, 752, 752, 752, 752, 752, 752, 752, 9, 0, 751, 750, 750, 749, 749, 749, 749, 748, 748, 748,
    748, 748, 748, 748, 10, 747, 747, 747, 747, 747, 747, 747, 747, 6, 6, 6, 6, 6, 6, 6, 6, 0, 0,
    0, 0, 0, 0, 0, 746, 0, 0, 0, 745, 0, 744, 743, 10, 0, 0, 0, 742, 0, 741, 740, 740, 0, 739, 738,
    738, 737, 737, 737, 9, 0, 0, 0, 736, 0, 735, 734, 734, 0, 733, 732, 732, 731, 731, 731, 10, 0,
    730, 729, 729, 728, 728, 728, 728, 727, 727, 727, 727, 727, 727, 8, 8, 0, 0, 0, 726, 0, 725,
    724, 724, 0, 723, 722, 722, 721, 721, 721, 10, 0, 720, 719, 719, 718, 718, 718, 718, 717, 717,
    717, 717, 717, 717, 717, 9, 0, 716, 715, 715, 714, 714, 714, 714, 713, 713, 713, 713, 713, 713,
    713, 10, 712, 712, 712, 712, 712, 712, 712, 712, 712, 712, 712, 712, 7, 7, 7, 7, 0, 0, 0, 711,
    0, 710, 709, 709, 0, 708, 707, 707, 706, 706, 706, 10, 0, 705, 704, 704, 703, 703, 703, 703,
    702, 702, 702, 702, 702, 702, 702, 9, 0, 701, 700, 700, 699, 699, 699, 699, 698, 698, 698, 698,
    698, 698, 698, 10, 697, 697, 697, 697, 697, 697, 697, 697, 697, 697, 697, 697, 697, 697, 8, 8,
    0, 696, 695, 695, 694, 694, 694, 694, 693, 693, 693, 693, 693, 693, 693, 10, 692, 692, 692,
    692, 692, 692, 692, 692, 692, 692, 692, 692, 692, 692, 692, 9, 691, 691, 691, 691, 691, 691,
    691, 691, 691, 691, 691, 691, 691, 691, 691, 10, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    0, 0, 0, 0, 0, 0, 0, 0, 690, 0, 0, 0, 689, 0, 688, 687, 10, 0, 0, 0, 686, 0, 685, 684, 684, 0,
    683, 682, 682, 681, 681, 681, 9, 0, 0, 0, 680, 0, 679, 678, 678, 0, 677, 676, 676, 675, 675,
    675, 10, 0, 674, 673, 673, 672, 672, 672, 672, 671, 671, 671, 671, 671, 671, 8, 8, 0, 0, 0,
    670, 0, 669, 668, 668, 0, 667, 666, 666, 665, 665, 665, 10, 0, 664, 663, 663, 662, 662, 662,
    662, 661, 661, 661, 661, 661, 661, 661, 9, 0, 660, 659, 659, 658, 658, 658, 658, 657, 657, 657,
    657, 657, 657, 657, 10, 656, 656, 656, 656, 656, 656, 656, 656, 656, 656, 656, 656, 7, 7, 7, 7,
    0, 0, 0, 655, 0, 654, 653, 653, 0, 652, 651, 651, 650, 650, 650, 10, 0, 649, 648, 648, 647,
    647, 647, 647, 646, 646, 646, 646, 646, 646, 646, 9, 0, 645, 644, 644, 643, 643, 643, 643, 642,
    642, 642, 642, 642, 642, 642, 10, 641, 641, 641, 641, 641, 641, 641, 641, 641, 641, 641, 641,
    641, 641, 8, 8, 0, 640, 639, 639, 638, 638, 638, 638, 637, 637, 637, 637, 637, 637, 637, 10,
    636, 636, 636, 636, 636, 636, 636, 636, 636, 636, 636, 636, 636, 636, 636, 9, 635, 635, 635,
    635, 635, 635, 635, 635, 635, 635, 635, 635, 635, 635, 635, 10, 635, 635, 635, 635, 635, 635,
    635, 635, 6, 6, 6, 6, 6, 6, 6, 0, 0, 0, 0, 634, 0, 633, 632, 632, 0, 631, 630, 630, 629, 629,
    629, 10, 0, 628, 627, 627, 626, 626, 626, 626, 625, 625, 625, 625, 625, 625, 625, 9, 0, 624,
    623, 623, 622, 622, 622, 622, 621, 621, 621, 621, 621, 621, 621, 10, 620, 620, 620, 620, 620,
    620, 620, 620, 620, 620, 620, 620, 620, 620, 8, 8, 0, 619, 618, 618, 617, 617, 617, 617, 616,
    616, 616, 616, 616, 616, 616, 10, 615, 615, 615, 615, 615, 615, 615, 615, 615, 615, 615, 615,
    615, 615, 615, 9, 614, 614, 614, 614, 614, 614, 614, 614, 614, 614, 614, 614, 614, 614, 614,
    10, 614, 614, 614, 614, 614, 614, 614, 614, 614, 614, 614, 614, 7, 7, 7, 0, 0, 613, 612, 612,
    611, 611, 611, 611, 610, 610, 610, 610, 610, 610, 610, 10, 609, 609, 609, 609, 609, 609, 609,
    609, 609, 609, 609, 609, 609, 609, 609, 9, 608, 608, 608, 608, 608, 608, 608, 608, 608, 608,
    608, 608, 608, 608, 608, 10, 608, 608, 608, 608, 608, 608, 608, 608, 608, 608, 608, 608, 608,
    608, 8, 0, 607, 607, 607, 607, 607, 607, 607, 607, 607, 607, 607, 607, 607, 607, 607, 10, 607,
    607, 607, 607, 607, 607, 607, 607, 607, 607, 607, 607, 607, 607, 607, 0, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    606, 0, 0, 0, 605, 0, 604, 603, 10, 0, 0, 0, 602, 0, 601, 600, 600, 0, 599, 598, 598, 597, 597,
    597, 9, 0, 0, 0, 596, 0, 595, 594, 594, 0, 593, 592, 592, 591, 591, 591, 10, 0, 590, 589, 589,
    588, 588, 588, 588, 587, 587, 587, 587, 587, 587, 8, 8, 0, 0, 0, 586, 0, 585, 584, 584, 0, 583,
    582, 582, 581, 581, 581, 10, 0, 580, 579, 579, 578, 578, 578, 578, 577, 577, 577, 577, 577,
    577, 577, 9, 0, 576, 575, 575, 574, 574, 574, 574, 573, 573, 573, 573, 573, 573, 573, 10, 572,
    572, 572, 572, 572, 572, 572, 572, 572, 572, 572, 572, 7, 7, 7, 7, 0, 0, 0, 571, 0, 570, 569,
    569, 0, 568, 567, 567, 566, 566, 566, 10, 0, 565, 564, 564, 563, 563, 563, 563, 562, 562, 562,
    562, 562, 562, 562, 9, 0, 561, 560, 560, 559, 559, 559, 559, 558, 558, 558, 558, 558, 558, 558,
    10, 557, 557, 557, 557, 557, 557, 557, 557, 557, 557, 557, 557, 557, 557, 8, 8, 0, 556, 555,
    555, 554, 554, 554, 554, 553, 553, 553, 553, 553, 553, 553, 10, 552, 552, 552, 552, 552, 552,
    552, 552, 552, 552, 552, 552, 552, 552, 552, 9, 551, 551, 551, 551, 551, 551, 551, 551, 551,
    551, 551, 551, 551, 551, 551, 10, 551, 551, 551, 551, 551, 551, 551, 551, 6, 6, 6, 6, 6, 6, 6,
    0, 0, 0, 0, 550, 0, 549, 548, 548, 0, 547, 546, 546, 545, 545, 545, 10, 0, 544, 543, 543, 542,
    542, 542, 542, 541, 541, 541, 541, 541, 541, 541, 9, 0, 540, 539, 539, 538, 538, 538, 538, 537,
    537, 537, 537, 537, 537, 537, 10, 536, 536, 536, 536, 536, 536, 536, 536, 536, 536, 536, 536,
    536, 536, 8, 8, 0, 535, 534, 534, 533, 533, 533, 533, 532, 532, 532, 532, 532, 532, 532, 10,
    531, 531, 531, 531, 531, 531, 531, 531, 531, 531, 531, 531, 531, 531, 531, 9, 530, 530, 530,
    530, 530, 530, 530, 530, 530, 530, 530, 530, 530, 530, 530, 10, 530, 530, 530, 530, 530, 530,
    530, 530, 530, 530, 530, 530, 7, 7, 7, 0, 0, 529, 528, 528, 527, 527, 527, 527, 526, 526, 526,
    526, 526, 526, 526, 10, 525, 525, 525, 525, 525, 525, 525, 525, 525, 525, 525, 525, 525, 525,
    525, 9, 524, 524, 524, 524, 524, 524, 524, 524, 524, 524, 524, 524, 524, 524, 524, 10, 524,
    524, 524, 524, 524, 524, 524, 524, 524, 524, 524, 524, 524, 524, 8, 0, 523, 523, 523, 523, 523,
    523, 523, 523, 523, 523, 523, 523, 523, 523, 523, 10, 523, 523, 523, 523, 523, 523, 523, 523,
    523, 523, 523, 523, 523, 523, 523, 0, 523, 523, 523, 523, 523, 523, 523, 523, 523, 523, 523,
    523, 523, 523, 523, 0, 5, 5, 5, 5, 5, 5, 5, 0, 5, 5, 5, 0, 5, 0, 0, 0, 0, 0, 0, 522, 0, 521,
    520, 520, 0, 519, 518, 518, 517, 517, 517, 10, 0, 516, 515, 515, 514, 514, 514, 514, 513, 513,
    513, 513, 513, 513, 513, 9, 0, 512, 511, 511, 510, 510, 510, 510, 509, 509, 509, 509, 509, 509,
    509, 10, 508, 508, 508, 508, 508, 508, 508, 508, 508, 508, 508, 508, 508, 508, 8, 8, 0, 507,
    506, 506, 505, 505, 505, 505, 504, 504, 504, 504, 504, 504, 504, 10, 503, 503, 503, 503, 503,
    503, 503, 503, 503, 503, 503, 503, 503, 503, 503, 9, 502, 502, 502, 502, 502, 502, 502, 502,
    502, 502, 502, 502, 502, 502, 502, 10, 502, 502, 502, 502, 502, 502, 502, 502, 502, 502, 502,
    502, 7, 7, 7, 0, 0, 501, 500, 500, 499, 499, 499, 499, 498, 498, 498, 498, 498, 498, 498, 10,
    497, 497, 497, 497, 497, 497, 497, 497, 497, 497, 497, 497, 497, 497, 497, 9, 496, 496, 496,
    496, 496, 496, 496, 496, 496, 496, 496, 496, 496, 496, 496, 10, 496, 496, 496, 496, 496, 496,
    496, 496, 496, 496, 496, 496, 496, 496, 8, 0, 495, 495, 495, 495, 495, 495, 495, 495, 495, 495,
    495, 495, 495, 495, 495, 10, 495, 495, 495, 495, 495, 495, 495, 495, 495, 495, 495, 495, 495,
    495, 495, 0, 495, 495, 495, 495, 495, 495, 495, 495, 495, 495, 495, 495, 495, 495, 495, 0, 495,
    495, 495, 495, 495, 495, 495, 0, 6, 6, 6, 0, 6, 0, 0, 0, 0, 494, 493, 493, 492, 492, 492, 492,
    491, 491, 491, 491, 491, 491, 491, 10, 490, 490, 490, 490, 490, 490, 490, 490, 490, 490, 490,
    490, 490, 490, 490, 9, 489, 489, 489, 489, 489, 489, 489, 489, 489, 489, 489, 489, 489, 489,
    489, 10, 489, 489, 489, 489, 489, 489, 489, 489, 489, 489, 489, 489, 489, 489, 8, 0, 488, 488,
    488, 488, 488, 488, 488, 488, 488, 488, 488, 488, 488, 488, 488, 10, 488, 488, 488, 488, 488,
    488, 488, 488, 488, 488, 488, 488, 488, 488, 488, 0, 488, 488, 488, 488, 488, 488, 488, 488,
    488, 488, 488, 488, 488, 488, 488, 0, 488, 488, 488, 488, 488, 488, 488, 0, 488, 488, 488, 0,
    7, 0, 0, 0, 487, 487, 487, 487, 487, 487, 487, 487, 487, 487, 487, 487, 487, 487, 487, 10, 487,
    487, 487, 487, 487, 487, 487, 487, 487, 487, 487, 487, 487, 487, 487, 0, 487, 487, 487, 487,
    487, 487, 487, 487, 487, 487, 487, 487, 487, 487, 487, 0, 487, 487, 487, 487, 487, 487, 487, 0,
    487, 487, 487, 0, 487, 0, 0, 0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 3,
    3, 3, 0, 3, 3, 3, 0, 3, 0, 0, 0, 3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 0, 3, 0, 0, 0, 3, 3, 3, 0, 3,
    0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 486, 0, 0, 0, 485, 0, 484, 483, 10, 0, 0,
    0, 482, 0, 481, 480, 480, 0, 479, 478, 478, 477, 477, 477, 9, 0, 0, 0, 476, 0, 475, 474, 474,
    0, 473, 472, 472, 471, 471, 471, 10, 0, 470, 469, 469, 468, 468, 468, 468, 467, 467, 467, 467,
    467, 467, 8, 8, 0, 0, 0, 466, 0, 465, 464, 464, 0, 463, 462, 462, 461, 461, 461, 10, 0, 460,
    459, 459, 458, 458, 458, 458, 457, 457, 457, 457, 457, 457, 457, 9, 0, 456, 455, 455, 454, 454,
    454, 454, 453, 453, 453, 453, 453, 453, 453, 10, 452, 452, 452, 452, 452, 452, 452, 452, 452,
    452, 452, 452, 7, 7, 7, 7, 0, 0, 0, 451, 0, 450, 449, 449, 0, 448, 447, 447, 446, 446, 446, 10,
    0, 445, 444, 444, 443, 443, 443, 443, 442, 442, 442, 442, 442, 442, 442, 9, 0, 441, 440, 440,
    439, 439, 439, 439, 438, 438, 438, 438, 438, 438, 438, 10, 437, 437, 437, 437, 437, 437, 437,
    437, 437, 437, 437, 437, 437, 437, 8, 8, 0, 436, 435, 435, 434, 434, 434, 434, 433, 433, 433,
    433, 433, 433, 433, 10, 432, 432, 432, 432, 432, 432, 432, 432, 432, 432, 432, 432, 432, 432,
    432, 9, 431, 431, 431, 431, 431, 431, 431, 431, 431, 431, 431, 431, 431, 431, 431, 10, 431,
    431, 431, 431, 431, 431, 431, 431, 6, 6, 6, 6, 6, 6, 6, 0, 0, 0, 0, 430, 0, 429, 428, 428, 0,
    427, 426, 426, 425, 425, 425, 10, 0, 424, 423, 423, 422, 422, 422, 422, 421, 421, 421, 421,
    421, 421, 421, 9, 0, 420, 419, 419, 418, 418, 418, 418, 417, 417, 417, 417, 417, 417, 417, 10,
    416, 416, 416, 416, 416, 416, 416, 416, 416, 416, 416, 416, 416, 416, 8, 8, 0, 415, 414, 414,
    413, 413, 413, 413, 412, 412, 412, 412, 412, 412, 412, 10, 411, 411, 411, 411, 411, 411, 411,
    411, 411, 411, 411, 411, 411, 411, 411, 9, 410, 410, 410, 410, 410, 410, 410, 410, 410, 410,
    410, 410, 410, 410, 410, 10, 410, 410, 410, 410, 410, 410, 410, 410, 410, 410, 410, 410, 7, 7,
    7, 0, 0, 409, 408, 408, 407, 407, 407, 407, 406, 406, 406, 406, 406, 406, 406, 10, 405, 405,
    405, 405, 405, 405, 405, 405, 405, 405, 405, 405, 405, 405, 405, 9, 404, 404, 404, 404, 404,
    404, 404, 404, 404, 404, 404, 404, 404, 404, 404, 10, 404, 404, 404, 404, 404, 404, 404, 404,
    404, 404, 404, 404, 404, 404, 8, 0, 403, 403, 403, 403, 403, 403, 403, 403, 403, 403, 403, 403,
    403, 403, 403, 10, 403, 403, 403, 403, 403, 403, 403, 403, 403, 403, 403, 403, 403, 403, 403,
    0, 403, 403, 403, 403, 403, 403, 403, 403, 403, 403, 403, 403, 403, 403, 403, 0, 5, 5, 5, 5, 5,
    5, 5, 0, 5, 5, 5, 0, 5, 0, 0, 0, 0, 0, 0, 402, 0, 401, 400, 400, 0, 399, 398, 398, 397, 397,
    397, 10, 0, 396, 395, 395, 394, 394, 394, 394, 393, 393, 393, 393, 393, 393, 393, 9, 0, 392,
    391, 391, 390, 390, 390, 390, 389, 389, 389, 389, 389, 389, 389, 10, 388, 388, 388, 388, 388,
    388, 388, 388, 388, 388, 388, 388, 388, 388, 8, 8, 0, 387, 386, 386, 385, 385, 385, 385, 384,
    384, 384, 384, 384, 384, 384, 10, 383, 383, 383, 383, 383, 383, 383, 383, 383, 383, 383, 383,
    383, 383, 383, 9, 382, 382, 382, 382, 382, 382, 382, 382, 382, 382, 382, 382, 382, 382, 382,
    10, 382, 382, 382, 382, 382, 382, 382, 382, 382, 382, 382, 382, 7, 7, 7, 0, 0, 381, 380, 380,
    379, 379, 379, 379, 378, 378, 378, 378, 378, 378, 378, 10, 377, 377, 377, 377, 377, 377, 377,
    377, 377, 377, 377, 377, 377, 377, 377, 9, 376, 376, 376, 376, 376, 376, 376, 376, 376, 376,
    376, 376, 376, 376, 376, 10, 376, 376, 376, 376, 376, 376, 376, 376, 376, 376, 376, 376, 376,
    376, 8, 0, 375, 375, 375, 375, 375, 375, 375, 375, 375, 375, 375, 375, 375, 375, 375, 10, 375,
    375, 375, 375, 375, 375, 375, 375, 375, 375, 375, 375, 375, 375, 375, 0, 375, 375, 375, 375,
    375, 375, 375, 375, 375, 375, 375, 375, 375, 375, 375, 0, 375, 375, 375, 375, 375, 375, 375, 0,
    6, 6, 6, 0, 6, 0, 0, 0, 0, 374, 373, 373, 372, 372, 372, 372, 371, 371, 371, 371, 371, 371,
    371, 10, 370, 370, 370, 370, 370, 370, 370, 370, 370, 370, 370, 370, 370, 370, 370, 9, 369,
    369, 369, 369, 369, 369, 369, 369, 369, 369, 369, 369, 369, 369, 369, 10, 369, 369, 369, 369,
    369, 369, 369, 369, 369, 369, 369, 369, 369, 369, 8, 0, 368, 368, 368, 368, 368, 368, 368, 368,
    368, 368, 368, 368, 368, 368, 368, 10, 368, 368, 368, 368, 368, 368, 368, 368, 368, 368, 368,
    368, 368, 368, 368, 0, 368, 368, 368, 368, 368, 368, 368, 368, 368, 368, 368, 368, 368, 368,
    368, 0, 368, 368, 368, 368, 368, 368, 368, 0, 368, 368, 368, 0, 7, 0, 0, 0, 367, 367, 367, 367,
    367, 367, 367, 367, 367, 367, 367, 367, 367, 367, 367, 10, 367, 367, 367, 367, 367, 367, 367,
    367, 367, 367, 367, 367, 367, 367, 367, 0, 367, 367, 367, 367, 367, 367, 367, 367, 367, 367,
    367, 367, 367, 367, 367, 0, 367, 367, 367, 367, 367, 367, 367, 0, 367, 367, 367, 0, 367, 0, 0,
    0, 367, 367, 367, 367, 367, 367, 367, 367, 367, 367, 367, 367, 367, 367, 367, 0, 367, 367, 367,
    367, 367, 367, 367, 0, 367, 367, 367, 0, 367, 0, 0, 0, 4, 4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 0, 4,
    0, 0, 0, 4, 4, 4, 0, 4, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 366, 0, 365, 364, 364, 0,
    363, 362, 362, 361, 361, 361, 10, 0, 360, 359, 359, 358, 358, 358, 358, 357, 357, 357, 357,
    357, 357, 357, 9, 0, 356, 355, 355, 354, 354, 354, 354, 353, 353, 353, 353, 353, 353, 353, 10,
    352, 352, 352, 352, 352, 352, 352, 352, 352, 352, 352, 352, 352, 352, 8, 8, 0, 351, 350, 350,
    349, 349, 349, 349, 348, 348, 348, 348, 348, 348, 348, 10, 347, 347, 347, 347, 347, 347, 347,
    347, 347, 347, 347, 347, 347, 347, 347, 9, 346, 346, 346, 346, 346, 346, 346, 346, 346, 346,
    346, 346, 346, 346, 346, 10, 346, 346, 346, 346, 346, 346, 346, 346, 346, 346, 346, 346, 7, 7,
    7, 0, 0, 345, 344, 344, 343, 343, 343, 343, 342, 342, 342, 342, 342, 342, 342, 10, 341, 341,
    341, 341, 341, 341, 341, 341, 341, 341, 341, 341, 341, 341, 341, 9, 340, 340, 340, 340, 340,
    340, 340, 340, 340, 340, 340, 340, 340, 340, 340, 10, 340, 340, 340, 340, 340, 340, 340, 340,
    340, 340, 340, 340, 340, 340, 8, 0, 339, 339, 339, 339, 339, 339, 339, 339, 339, 339, 339, 339,
    339, 339, 339, 10, 339, 339, 339, 339, 339, 339, 339, 339, 339, 339, 339, 339, 339, 339, 339,
    0, 339, 339, 339, 339, 339, 339, 339, 339, 339, 339, 339, 339, 339, 339, 339, 0, 339, 339, 339,
    339, 339, 339, 339, 0, 6, 6, 6, 0, 6, 0, 0, 0, 0, 338, 337, 337, 336, 336, 336, 336, 335, 335,
    335, 335, 335, 335, 335, 10, 334, 334, 334, 334, 334, 334, 334, 334, 334, 334, 334, 334, 334,
    334, 334, 9, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 10,
    333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 8, 0, 332, 332, 332, 332,
    332, 332, 332, 332, 332, 332, 332, 332, 332, 332, 332, 10, 332, 332, 332, 332, 332, 332, 332,
    332, 332, 332, 332, 332, 332, 332, 332, 0, 332, 332, 332, 332, 332, 332, 332, 332, 332, 332,
    332, 332, 332, 332, 332, 0, 332, 332, 332, 332, 332, 332, 332, 0, 332, 332, 332, 0, 7, 0, 0, 0,
    331, 331, 331, 331, 331, 331, 331, 331, 331, 331, 331, 331, 331, 331, 331, 10, 331, 331, 331,
    331, 331, 331, 331, 331, 331, 331, 331, 331, 331, 331, 331, 0, 331, 331, 331, 331, 331, 331,
    331, 331, 331, 331, 331, 331, 331, 331, 331, 0, 331, 331, 331, 331, 331, 331, 331, 0, 331, 331,
    331, 0, 331, 0, 0, 0, 331, 331, 331, 331, 331, 331, 331, 331, 331, 331, 331, 331, 331, 331,
    331, 0, 331, 331, 331, 331, 331, 331, 331, 0, 331, 331, 331, 0, 331, 0, 0, 0, 331, 331, 331,
    331, 331, 331, 331, 0, 331, 331, 331, 0, 331, 0, 0, 0, 5, 5, 5, 0, 5, 0, 0, 0, 5, 0, 0, 0, 0,
    0, 0, 0, 0, 330, 329, 329, 328, 328, 328, 328, 327, 327, 327, 327, 327, 327, 327, 10, 326, 326,
    326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 9, 325, 325, 325, 325, 325,
    325, 325, 325, 325, 325, 325, 325, 325, 325, 325, 10, 325, 325, 325, 325, 325, 325, 325, 325,
    325, 325, 325, 325, 325, 325, 8, 0, 324, 324, 324, 324, 324, 324, 324, 324, 324, 324, 324, 324,
    324, 324, 324, 10, 324, 324, 324, 324, 324, 324, 324, 324, 324, 324, 324, 324, 324, 324, 324,
    0, 324, 324, 324, 324, 324, 324, 324, 324, 324, 324, 324, 324, 324, 324, 324, 0, 324, 324, 324,
    324, 324, 324, 324, 0, 324, 324, 324, 0, 7, 0, 0, 0, 323, 323, 323, 323, 323, 323, 323, 323,
    323, 323, 323, 323, 323, 323, 323, 10, 323, 323, 323, 323, 323, 323, 323, 323, 323, 323, 323,
    323, 323, 323, 323, 0, 323, 323, 323, 323, 323, 323, 323, 323, 323, 323, 323, 323, 323, 323,
    323, 0, 323, 323, 323, 323, 323, 323, 323, 0, 323, 323, 323, 0, 323, 0, 0, 0, 323, 323, 323,
    323, 323, 323, 323, 323, 323, 323, 323, 323, 323, 323, 323, 0, 323, 323, 323, 323, 323, 323,
    323, 0, 323, 323, 323, 0, 323, 0, 0, 0, 323, 323, 323, 323, 323, 323, 323, 0, 323, 323, 323, 0,
    323, 0, 0, 0, 323, 323, 323, 0, 323, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0,
    1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0,
    1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0,
    1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0,
    1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const AS_RAINBOW: [u16; 49205] = [
    11, 23, 11, 167, 23, 11, 167, 179, 23, 12, 168, 191, 180, 24, 35, 35, 35, 36, 11, 167, 23, 11,
    167, 179, 23, 12, 168, 2468, 180, 24, 168, 191, 192, 180, 35, 35, 36, 11, 167, 179, 23, 12,
    169, 2468, 181, 24, 168, 2479, 2600, 180, 191, 193, 192, 35, 36, 13, 169, 203, 181, 25, 169,
    203, 204, 181, 203, 205, 204, 193, 193, 37, 47, 47, 47, 48, 47, 47, 48, 47, 48, 49, 11, 167,
    23, 11, 167, 179, 23, 12, 168, 2468, 180, 24, 168, 191, 192, 180, 35, 35, 36, 11, 167, 179, 23,
    12, 1600, 1600, 1600, 24, 168, 1600, 1600, 180, 191, 1600, 192, 35, 36, 13, 169, 2469, 181, 25,
    169, 1600, 1600, 181, 2480, 1600, 2601, 193, 193, 37, 169, 203, 204, 181, 203, 1600, 204, 205,
    205, 193, 47, 47, 48, 47, 48, 49, 11, 167, 179, 23, 12, 170, 2468, 182, 24, 168, 2479, 2600,
    180, 191, 194, 192, 35, 36, 13, 170, 2469, 182, 25, 170, 1600, 1600, 182, 2480, 1600, 2601,
    194, 194, 37, 169, 2490, 2611, 181, 2491, 1600, 2612, 2721, 2722, 193, 203, 206, 204, 206, 206,
    205, 47, 48, 49, 14, 170, 215, 182, 26, 170, 215, 216, 182, 215, 217, 216, 194, 194, 38, 170,
    215, 216, 182, 215, 1600, 216, 217, 217, 194, 215, 218, 216, 218, 218, 217, 206, 206, 206, 50,
    59, 59, 59, 60, 59, 59, 60, 59, 60, 61, 59, 59, 60, 59, 60, 61, 59, 60, 61, 62, 11, 167, 23,
    11, 167, 179, 23, 12, 168, 2468, 180, 24, 168, 191, 192, 180, 35, 35, 36, 11, 167, 179, 23, 12,
    1610, 2468, 1676, 24, 168, 2479, 2600, 180, 191, 1742, 192, 35, 36, 13, 169, 2469, 181, 25,
    169, 2490, 2611, 181, 2480, 2721, 2601, 193, 193, 37, 169, 203, 204, 181, 203, 1808, 204, 205,
    205, 193, 47, 47, 48, 47, 48, 49, 11, 167, 179, 23, 12, 1610, 2468, 1676, 24, 168, 2479, 2600,
    180, 191, 1742, 192, 35, 36, 13, 1611, 2469, 1677, 25, 1621, 1600, 1600, 1601, 2480, 1600,
    1601, 1743, 1601, 37, 169, 2490, 2611, 181, 2491, 1600, 1601, 2721, 1601, 193, 203, 1808, 204,
    1809, 1601, 205, 47, 48, 49, 14, 170, 2470, 182, 26, 170, 2501, 2622, 182, 2481, 2732, 2602,
    194, 194, 38, 170, 2501, 2622, 182, 2502, 1600, 1601, 2732, 1601, 194, 2492, 2831, 2613, 2831,
    1601, 2723, 206, 206, 206, 50, 170, 215, 216, 182, 215, 1874, 216, 217, 217, 194, 215, 1874,
    216, 1875, 1601, 217, 218, 218, 218, 206, 59, 59, 60, 59, 60, 61, 59, 60, 61, 62, 11, 167, 179,
    23, 12, 171, 2468, 183, 24, 168, 2479, 2600, 180, 191, 195, 192, 35, 36, 13, 171, 2469, 183,
    25, 171, 2512, 2633, 183, 2480, 2743, 2601, 195, 195, 37, 169, 2490, 2611, 181, 2491, 2842,
    2612, 2721, 2722, 193, 203, 207, 204, 207, 207, 205, 47, 48, 49, 14, 171, 2470, 183, 26, 171,
    2512, 2633, 183, 2481, 2743, 2602, 195, 195, 38, 171, 2512, 2633, 183, 2513, 1600, 1601, 2743,
    1601, 195, 2492, 2842, 2613, 2842, 1601, 2723, 207, 207, 207, 50, 170, 2501, 2622, 182, 2502,
    2930, 2623, 2732, 2733, 194, 2503, 2930, 2624, 2930, 1601, 2734, 2831, 2832, 2833, 206, 215,
    219, 216, 219, 219, 217, 219, 219, 219, 218, 59, 60, 61, 62, 15, 171, 227, 183, 27, 171, 227,
    228, 183, 227, 229, 228, 195, 195, 39, 171, 227, 228, 183, 227, 1940, 228, 229, 229, 195, 227,
    230, 228, 230, 230, 229, 207, 207, 207, 51, 171, 227, 228, 183, 227, 1940, 228, 229, 229, 195,
    227, 1940, 228, 1941, 1601, 229, 230, 230, 230, 207, 227, 231, 228, 231, 231, 229, 231, 231,
    231, 230, 219, 219, 219, 219, 63, 71, 71, 71, 72, 71, 71, 72, 71, 72, 73, 71, 71, 72, 71, 72,
    73, 71, 72, 73, 74, 71, 71, 72, 71, 72, 73, 71, 72, 73, 74, 71, 72, 73, 74, 75, 11, 167, 23,
    11, 167, 179, 23, 12, 168, 2468, 180, 24, 168, 191, 192, 180, 35, 35, 36, 11, 167, 179, 23, 12,
    1610, 2468, 1676, 24, 168, 2479, 2600, 180, 191, 1742, 192, 35, 36, 13, 169, 2469, 181, 25,
    169, 2490, 2611, 181, 2480, 2721, 2601, 193, 193, 37, 169, 203, 204, 181, 203, 1808, 204, 205,
    205, 193, 47, 47, 48, 47, 48, 49, 11, 167, 179, 23, 12, 1610, 2468, 1676, 24, 168, 2479, 2600,
    180, 191, 1742, 192, 35, 36, 13, 1611, 2469, 1677, 25, 1621, 1600, 1600, 1687, 2480, 1600,
    2601, 1743, 1753, 37, 169, 2490, 2611, 181, 2491, 1600, 2612, 2721, 2722, 193, 203, 1808, 204,
    1809, 1819, 205, 47, 48, 49, 14, 170, 2470, 182, 26, 170, 2501, 2622, 182, 2481, 2732, 2602,
    194, 194, 38, 170, 2501, 2622, 182, 2502, 1600, 2623, 2732, 2733, 194, 2492, 2831, 2613, 2831,
    2832, 2723, 206, 206, 206, 50, 170, 215, 216, 182, 215, 1874, 216, 217, 217, 194, 215, 1874,
    216, 1875, 1885, 217, 218, 218, 218, 206, 59, 59, 60, 59, 60, 61, 59, 60, 61, 62, 11, 167, 179,
    23, 12, 1610, 2468, 1676, 24, 168, 2479, 2600, 180, 191, 1742, 192, 35, 36, 13, 1611, 2469,
    1677, 25, 1621, 3326, 3546, 1687, 2480, 3766, 2601, 1743, 1753, 37, 169, 2490, 2611, 181, 2491,
    3986, 2612, 2721, 2722, 193, 203, 1808, 204, 1809, 1819, 205, 47, 48, 49, 14, 1612, 2470, 1678,
    26, 1622, 3327, 3547, 1688, 2481, 3767, 2602, 1744, 1754, 38, 1631, 3336, 3556, 1697, 1602,
    1600, 1601, 1602, 1601, 1602, 2492, 3987, 2613, 1602, 1601, 1602, 1810, 1820, 1602, 50, 170,
    2501, 2622, 182, 2502, 4206, 2623, 2732, 2733, 194, 2503, 4207, 2624, 1602, 1601, 1602, 2831,
    2832, 1602, 206, 215, 1874, 216, 1875, 1885, 217, 1876, 1886, 1602, 218, 59, 60, 61, 62, 15,
    171, 2471, 183, 27, 171, 2512, 2633, 183, 2482, 2743, 2603, 195, 195, 39, 171, 2512, 2633, 183,
    2513, 4426, 2634, 2743, 2744, 195, 2493, 2842, 2614, 2842, 2843, 2724, 207, 207, 207, 51, 171,
    2512, 2633, 183, 2513, 4426, 2634, 2743, 2744, 195, 2514, 4427, 2635, 1602, 1601, 1602, 2842,
    2843, 1602, 207, 2504, 2930, 2625, 2930, 2931, 2735, 2930, 2931, 1602, 2834, 219, 219, 219,
    219, 63, 171, 227, 228, 183, 227, 1940, 228, 229, 229, 195, 227, 1940, 228, 1941, 1951, 229,
    230, 230, 230, 207, 227, 1940, 228, 1941, 1951, 229, 1942, 1952, 1602, 230, 231, 231, 231, 231,
    219, 71, 71, 72, 71, 72, 73, 71, 72, 73, 74, 71, 72, 73, 74, 75, 11, 167, 179, 23, 12, 172,
    2468, 184, 24, 168, 2479, 2600, 180, 191, 196, 192, 35, 36, 13, 172, 2469, 184, 25, 172, 2523,
    2644, 184, 2480, 2754, 2601, 196, 196, 37, 169, 2490, 2611, 181, 2491, 2853, 2612, 2721, 2722,
    193, 203, 208, 204, 208, 208, 205, 47, 48, 49, 14, 172, 2470, 184, 26, 172, 2523, 2644, 184,
    2481, 2754, 2602, 196, 196, 38, 172, 2523, 2644, 184, 2524, 1600, 2645, 2754, 2755, 196, 2492,
    2853, 2613, 2853, 2854, 2723, 208, 208, 208, 50, 170, 2501, 2622, 182, 2502, 2941, 2623, 2732,
    2733, 194, 2503, 2941, 2624, 2941, 2942, 2734, 2831, 2832, 2833, 206, 215, 220, 216, 220, 220,
    217, 220, 220, 220, 218, 59, 60, 61, 62, 15, 172, 2471, 184, 27, 172, 2523, 2644, 184, 2482,
    2754, 2603, 196, 196, 39, 172, 2523, 2644, 184, 2524, 4646, 2645, 2754, 2755, 196, 2493, 2853,
    2614, 2853, 2854, 2724, 208, 208, 208, 51, 172, 2523, 2644, 184, 2524, 4646, 2645, 2754, 2755,
    196, 2525, 4647, 2646, 1602, 1601, 1602, 2853, 2854, 1602, 208, 2504, 2941, 2625, 2941, 2942,
    2735, 2941, 2942, 1602, 2834, 220, 220, 220, 220, 63, 171, 2512, 2633, 183, 2513, 3018, 2634,
    2743, 2744, 195, 2514, 3018, 2635, 3018, 3019, 2745, 2842, 2843, 2844, 207, 2515, 3018, 2636,
    3018, 3019, 2746, 3018, 3019, 1602, 2845, 2930, 2931, 2932, 2933, 219, 227, 232, 228, 232, 232,
    229, 232, 232, 232, 230, 232, 232, 232, 232, 231, 71, 72, 73, 74, 75, 16, 172, 239, 184, 28,
    172, 239, 240, 184, 239, 241, 240, 196, 196, 40, 172, 239, 240, 184, 239, 2006, 240, 241, 241,
    196, 239, 242, 240, 242, 242, 241, 208, 208, 208, 52, 172, 239, 240, 184, 239, 2006, 240, 241,
    241, 196, 239, 2006, 240, 2007, 2017, 241, 242, 242, 242, 208, 239, 243, 240, 243, 243, 241,
    243, 243, 243, 242, 220, 220, 220, 220, 64, 172, 239, 240, 184, 239, 2006, 240, 241, 241, 196,
    239, 2006, 240, 2007, 2017, 241, 242, 242, 242, 208, 239, 2006, 240, 2007, 2017, 241, 2008,
    2018, 1602, 242, 243, 243, 243, 243, 220, 239, 244, 240, 244, 244, 241, 244, 244, 244, 242,
    244, 244, 244, 244, 243, 232, 232, 232, 232, 232, 76, 83, 83, 83, 84, 83, 83, 84, 83, 84, 85,
    83, 83, 84, 83, 84, 85, 83, 84, 85, 86, 83, 83, 84, 83, 84, 85, 83, 84, 85, 86, 83, 84, 85, 86,
    87, 83, 83, 84, 83, 84, 85, 83, 84, 85, 86, 83, 84, 85, 86, 87, 83, 84, 85, 86, 87, 88, 11,
    167, 23, 11, 167, 179, 23, 12, 168, 2468, 180, 24, 168, 191, 192, 180, 35, 35, 36, 11, 167,
    179, 23, 12, 1610, 2468, 1676, 24, 168, 2479, 2600, 180, 191, 1742, 192, 35, 36, 13, 169, 2469,
    181, 25, 169, 2490, 2611, 181, 2480, 2721, 2601, 193, 193, 37, 169, 203, 204, 181, 203, 1808,
    204, 205, 205, 193, 47, 47, 48, 47, 48, 49, 11, 167, 179, 23, 12, 1610, 2468, 1676, 24, 168,
    2479, 2600, 180, 191, 1742, 192, 35, 36, 13, 1611, 2469, 1677, 25, 1621, 1600, 1600, 1687,
    2480, 1600, 2601, 1743, 1753, 37, 169, 2490, 2611, 181, 2491, 1600, 2612, 2721, 2722, 193, 203,
    1808, 204, 1809, 1819, 205, 47, 48, 49, 14, 170, 2470, 182, 26, 170, 2501, 2622, 182, 2481,
    2732, 2602, 194, 194, 38, 170, 2501, 2622, 182, 2502, 1600, 2623, 2732, 2733, 194, 2492, 2831,
    2613, 2831, 2832, 2723, 206, 206, 206, 50, 170, 215, 216, 182, 215, 1874, 216, 217, 217, 194,
    215, 1874, 216, 1875, 1885, 217, 218, 218, 218, 206, 59, 59, 60, 59, 60, 61, 59, 60, 61, 62,
    11, 167, 179, 23, 12, 1610, 2468, 1676, 24, 168, 2479, 2600, 180, 191, 1742, 192, 35, 36, 13,
    1611, 2469, 1677, 25, 1621, 3326, 3546, 1687, 2480, 3766, 2601, 1743, 1753, 37, 169, 2490,
    2611, 181, 2491, 3986, 2612, 2721, 2722, 193, 203, 1808, 204, 1809, 1819, 205, 47, 48, 49, 14,
    1612, 2470, 1678, 26, 1622, 3327, 3547, 1688, 2481, 3767, 2602, 1744, 1754, 38, 1631, 3336,
    3556, 1697, 3381, 1600, 1601, 3776, 1601, 1763, 2492, 3987, 2613, 3996, 1601, 2723, 1810, 1820,
    1829, 50, 170, 2501, 2622, 182, 2502, 4206, 2623, 2732, 2733, 194, 2503, 4207, 2624, 4216,
    1601, 2734, 2831, 2832, 2833, 206, 215, 1874, 216, 1875, 1885, 217, 1876, 1886, 1895, 218, 59,
    60, 61, 62, 15, 171, 2471, 183, 27, 171, 2512, 2633, 183, 2482, 2743, 2603, 195, 195, 39, 171,
    2512, 2633, 183, 2513, 4426, 2634, 2743, 2744, 195, 2493, 2842, 2614, 2842, 2843, 2724, 207,
    207, 207, 51, 171, 2512, 2633, 183, 2513, 4426, 2634, 2743, 2744, 195, 2514, 4427, 2635, 4436,
    1601, 2745, 2842, 2843, 2844, 207, 2504, 2930, 2625, 2930, 2931, 2735, 2930, 2931, 2932, 2834,
    219, 219, 219, 219, 63, 171, 227, 228, 183, 227, 1940, 228, 229, 229, 195, 227, 1940, 228,
    1941, 1951, 229, 230, 230, 230, 207, 227, 1940, 228, 1941, 1951, 229, 1942, 1952, 1961, 230,
    231, 231, 231, 231, 219, 71, 71, 72, 71, 72, 73, 71, 72, 73, 74, 71, 72, 73, 74, 75, 11, 167,
    179, 23, 12, 1610, 2468, 1676, 24, 168, 2479, 2600, 180, 191, 1742, 192, 35, 36, 13, 1611,
    2469, 1677, 25, 1621, 3326, 3546, 1687, 2480, 3766, 2601, 1743, 1753, 37, 169, 2490, 2611, 181,
    2491, 3986, 2612, 2721, 2722, 193, 203, 1808, 204, 1809, 1819, 205, 47, 48, 49, 14, 1612, 2470,
    1678, 26, 1622, 3327, 3547, 1688, 2481, 3767, 2602, 1744, 1754, 38, 1631, 3336, 3556, 1697,
    3381, 1600, 3601, 3776, 3821, 1763, 2492, 3987, 2613, 3996, 4041, 2723, 1810, 1820, 1829, 50,
    170, 2501, 2622, 182, 2502, 4206, 2623, 2732, 2733, 194, 2503, 4207, 2624, 4216, 4261, 2734,
    2831, 2832, 2833, 206, 215, 1874, 216, 1875, 1885, 217, 1876, 1886, 1895, 218, 59, 60, 61, 62,
    15, 1613, 2471, 1679, 27, 1623, 3328, 3548, 1689, 2482, 3768, 2603, 1745, 1755, 39, 1632, 3337,
    3557, 1698, 3382, 6186, 3602, 3777, 3822, 1764, 2493, 3988, 2614, 3997, 4042, 2724, 1811, 1821,
    1830, 51, 1640, 3345, 3565, 1706, 3390, 6194, 3610, 3785, 3830, 1772, 1603, 1603, 1603, 1602,
    1601, 1602, 1603, 1603, 1602, 1603, 2504, 4208, 2625, 4217, 4262, 2735, 1603, 1603, 1602, 1603,
    1877, 1887, 1896, 1603, 63, 171, 2512, 2633, 183, 2513, 4426, 2634, 2743, 2744, 195, 2514,
    4427, 2635, 4436, 4481, 2745, 2842, 2843, 2844, 207, 2515, 4428, 2636, 4437, 4482, 2746, 1603,
    1603, 1602, 1603, 2930, 2931, 2932, 1603, 219, 227, 1940, 228, 1941, 1951, 229, 1942, 1952,
    1961, 230, 1943, 1953, 1962, 1603, 231, 71, 72, 73, 74, 75, 16, 172, 2472, 184, 28, 172, 2523,
    2644, 184, 2483, 2754, 2604, 196, 196, 40, 172, 2523, 2644, 184, 2524, 4646, 2645, 2754, 2755,
    196, 2494, 2853, 2615, 2853, 2854, 2725, 208, 208, 208, 52, 172, 2523, 2644, 184, 2524, 4646,
    2645, 2754, 2755, 196, 2525, 4647, 2646, 4656, 4701, 2756, 2853, 2854, 2855, 208, 2505, 2941,
    2626, 2941, 2942, 2736, 2941, 2942, 2943, 2835, 220, 220, 220, 220, 64, 172, 2523, 2644, 184,
    2524, 4646, 2645, 2754, 2755, 196, 2525, 4647, 2646, 4656, 4701, 2756, 2853, 2854, 2855, 208,
    2526, 4648, 2647, 4657, 4702, 2757, 1603, 1603, 1602, 1603, 2941, 2942, 2943, 1603, 220, 2516,
    3018, 2637, 3018, 3019, 2747, 3018, 3019, 3020, 2846, 3018, 3019, 3020, 1603, 2934, 232, 232,
    232, 232, 232, 76, 172, 239, 240, 184, 239, 2006, 240, 241, 241, 196, 239, 2006, 240, 2007,
    2017, 241, 242, 242, 242, 208, 239, 2006, 240, 2007, 2017, 241, 2008, 2018, 2027, 242, 243,
    243, 243, 243, 220, 239, 2006, 240, 2007, 2017, 241, 2008, 2018, 2027, 242, 2009, 2019, 2028,
    1603, 243, 244, 244, 244, 244, 244, 232, 83, 83, 84, 83, 84, 85, 83, 84, 85, 86, 83, 84, 85,
    86, 87, 83, 84, 85, 86, 87, 88, 11, 167, 179, 23, 12, 173, 2468, 185, 24, 168, 2479, 2600, 180,
    191, 197, 192, 35, 36, 13, 173, 2469, 185, 25, 173, 2534, 2655, 185, 2480, 2765, 2601, 197,
    197, 37, 169, 2490, 2611, 181, 2491, 2864, 2612, 2721, 2722, 193, 203, 209, 204, 209, 209, 205,
    47, 48, 49, 14, 173, 2470, 185, 26, 173, 2534, 2655, 185, 2481, 2765, 2602, 197, 197, 38, 173,
    2534, 2655, 185, 2535, 1600, 2656, 2765, 2766, 197, 2492, 2864, 2613, 2864, 2865, 2723, 209,
    209, 209, 50, 170, 2501, 2622, 182, 2502, 2952, 2623, 2732, 2733, 194, 2503, 2952, 2624, 2952,
    2953, 2734, 2831, 2832, 2833, 206, 215, 221, 216, 221, 221, 217, 221, 221, 221, 218, 59, 60,
    61, 62, 15, 173, 2471, 185, 27, 173, 2534, 2655, 185, 2482, 2765, 2603, 197, 197, 39, 173,
    2534, 2655, 185, 2535, 4866, 2656, 2765, 2766, 197, 2493, 2864, 2614, 2864, 2865, 2724, 209,
    209, 209, 51, 173, 2534, 2655, 185, 2535, 4866, 2656, 2765, 2766, 197, 2536, 4867, 2657, 4876,
    1601, 2767, 2864, 2865, 2866, 209, 2504, 2952, 2625, 2952, 2953, 2735, 2952, 2953, 2954, 2834,
    221, 221, 221, 221, 63, 171, 2512, 2633, 183, 2513, 3029, 2634, 2743, 2744, 195, 2514, 3029,
    2635, 3029, 3030, 2745, 2842, 2843, 2844, 207, 2515, 3029, 2636, 3029, 3030, 2746, 3029, 3030,
    3031, 2845, 2930, 2931, 2932, 2933, 219, 227, 233, 228, 233, 233, 229, 233, 233, 233, 230, 233,
    233, 233, 233, 231, 71, 72, 73, 74, 75, 16, 173, 2472, 185, 28, 173, 2534, 2655, 185, 2483,
    2765, 2604, 197, 197, 40, 173, 2534, 2655, 185, 2535, 4866, 2656, 2765, 2766, 197, 2494, 2864,
    2615, 2864, 2865, 2725, 209, 209, 209, 52, 173, 2534, 2655, 185, 2535, 4866, 2656, 2765, 2766,
    197, 2536, 4867, 2657, 4876, 4921, 2767, 2864, 2865, 2866, 209, 2505, 2952, 2626, 2952, 2953,
    2736, 2952, 2953, 2954, 2835, 221, 221, 221, 221, 64, 173, 2534, 2655, 185, 2535, 4866, 2656,
    2765, 2766, 197, 2536, 4867, 2657, 4876, 4921, 2767, 2864, 2865, 2866, 209, 2537, 4868, 2658,
    4877, 4922, 2768, 1603, 1603, 1602, 1603, 2952, 2953, 2954, 1603, 221, 2516, 3029, 2637, 3029,
    3030, 2747, 3029, 3030, 3031, 2846, 3029, 3030, 3031, 1603, 2934, 233, 233, 233, 233, 233, 76,
    172, 2523, 2644, 184, 2524, 3095, 2645, 2754, 2755, 196, 2525, 3095, 2646, 3095, 3096, 2756,
    2853, 2854, 2855, 208, 2526, 3095, 2647, 3095, 3096, 2757, 3095, 3096, 3097, 2856, 2941, 2942,
    2943, 2944, 220, 2527, 3095, 2648, 3095, 3096, 2758, 3095, 3096, 3097, 2857, 3095, 3096, 3097,
    1603, 2945, 3018, 3019, 3020, 3021, 3022, 232, 239, 245, 240, 245, 245, 241, 245, 245, 245,
    242, 245, 245, 245, 245, 243, 245, 245, 245, 245, 245, 244, 83, 84, 85, 86, 87, 88, 17, 173,
    251, 185, 29, 173, 251, 252, 185, 251, 253, 252, 197, 197, 41, 173, 251, 252, 185, 251, 2072,
    252, 253, 253, 197, 251, 254, 252, 254, 254, 253, 209, 209, 209, 53, 173, 251, 252, 185, 251,
    2072, 252, 253, 253, 197, 251, 2072, 252, 2073, 2083, 253, 254, 254, 254, 209, 251, 255, 252,
    255, 255, 253, 255, 255, 255, 254, 221, 221, 221, 221, 65, 173, 251, 252, 185, 251, 2072, 252,
    253, 253, 197, 251, 2072, 252, 2073, 2083, 253, 254, 254, 254, 209, 251, 2072, 252, 2073, 2083,
    253, 2074, 2084, 2093, 254, 255, 255, 255, 255, 221, 251, 256, 252, 256, 256, 253, 256, 256,
    256, 254, 256, 256, 256, 256, 255, 233, 233, 233, 233, 233, 77, 173, 251, 252, 185, 251, 2072,
    252, 253, 253, 197, 251, 2072, 252, 2073, 2083, 253, 254, 254, 254, 209, 251, 2072, 252, 2073,
    2083, 253, 2074, 2084, 2093, 254, 255, 255, 255, 255, 221, 251, 2072, 252, 2073, 2083, 253,
    2074, 2084, 2093, 254, 2075, 2085, 2094, 1603, 255, 256, 256, 256, 256, 256, 233, 251, 257,
    252, 257, 257, 253, 257, 257, 257, 254, 257, 257, 257, 257, 255, 257, 257, 257, 257, 257, 256,
    245, 245, 245, 245, 245, 245, 89, 95, 95, 95, 96, 95, 95, 96, 95, 96, 97, 95, 95, 96, 95, 96,
    97, 95, 96, 97, 98, 95, 95, 96, 95, 96, 97, 95, 96, 97, 98, 95, 96, 97, 98, 99, 95, 95, 96, 95,
    96, 97, 95, 96, 97, 98, 95, 96, 97, 98, 99, 95, 96, 97, 98, 99, 100, 95, 95, 96, 95, 96, 97,
    95, 96, 97, 98, 95, 96, 97, 98, 99, 95, 96, 97, 98, 99, 100, 95, 96, 97, 98, 99, 100, 101, 11,
    167, 23, 11, 167, 179, 23, 12, 168, 2468, 180, 24, 168, 191, 192, 180, 35, 35, 36, 11, 167,
    179, 23, 12, 1610, 2468, 1676, 24, 168, 2479, 2600, 180, 191, 1742, 192, 35, 36, 13, 169, 2469,
    181, 25, 169, 2490, 2611, 181, 2480, 2721, 2601, 193, 193, 37, 169, 203, 204, 181, 203, 1808,
    204, 205, 205, 193, 47, 47, 48, 47, 48, 49, 11, 167, 179, 23, 12, 1610, 2468, 1676, 24, 168,
    2479, 2600, 180, 191, 1742, 192, 35, 36, 13, 1611, 2469, 1677, 25, 1621, 1600, 1600, 1687,
    2480, 1600, 2601, 1743, 1753, 37, 169, 2490, 2611, 181, 2491, 1600, 2612, 2721, 2722, 193, 203,
    1808, 204, 1809, 1819, 205, 47, 48, 49, 14, 170, 2470, 182, 26, 170, 2501, 2622, 182, 2481,
    2732, 2602, 194, 194, 38, 170, 2501, 2622, 182, 2502, 1600, 2623, 2732, 2733, 194, 2492, 2831,
    2613, 2831, 2832, 2723, 206, 206, 206, 50, 170, 215, 216, 182, 215, 1874, 216, 217, 217, 194,
    215, 1874, 216, 1875, 1885, 217, 218, 218, 218, 206, 59, 59, 60, 59, 60, 61, 59, 60, 61, 62,
    11, 167, 179, 23, 12, 1610, 2468, 1676, 24, 168, 2479, 2600, 180, 191, 1742, 192, 35, 36, 13,
    1611, 2469, 1677, 25, 1621, 3326, 3546, 1687, 2480, 3766, 2601, 1743, 1753, 37, 169, 2490,
    2611, 181, 2491, 3986, 2612, 2721, 2722, 193, 203, 1808, 204, 1809, 1819, 205, 47, 48, 49, 14,
    1612, 2470, 1678, 26, 1622, 3327, 3547, 1688, 2481, 3767, 2602, 1744, 1754, 38, 1631, 3336,
    3556, 1697, 3381, 1600, 1601, 3776, 1601, 1763, 2492, 3987, 2613, 3996, 1601, 2723, 1810, 1820,
    1829, 50, 170, 2501, 2622, 182, 2502, 4206, 2623, 2732, 2733, 194, 2503, 4207, 2624, 4216,
    1601, 2734, 2831, 2832, 2833, 206, 215, 1874, 216, 1875, 1885, 217, 1876, 1886, 1895, 218, 59,
    60, 61, 62, 15, 171, 2471, 183, 27, 171, 2512, 2633, 183, 2482, 2743, 2603, 195, 195, 39, 171,
    2512, 2633, 183, 2513, 4426, 2634, 2743, 2744, 195, 2493, 2842, 2614, 2842, 2843, 2724, 207,
    207, 207, 51, 171, 2512, 2633, 183, 2513, 4426, 2634, 2743, 2744, 195, 2514, 4427, 2635, 4436,
    1601, 2745, 2842, 2843, 2844, 207, 2504, 2930, 2625, 2930, 2931, 2735, 2930, 2931, 2932, 2834,
    219, 219, 219, 219, 63, 171, 227, 228, 183, 227, 1940, 228, 229, 229, 195, 227, 1940, 228,
    1941, 1951, 229, 230, 230, 230, 207, 227, 1940, 228, 1941, 1951, 229, 1942, 1952, 1961, 230,
    231, 231, 231, 231, 219, 71, 71, 72, 71, 72, 73, 71, 72, 73, 74, 71, 72, 73, 74, 75, 11, 167,
    179, 23, 12, 1610, 2468, 1676, 24, 168, 2479, 2600, 180, 191, 1742, 192, 35, 36, 13, 1611,
    2469, 1677, 25, 1621, 3326, 3546, 1687, 2480, 3766, 2601, 1743, 1753, 37, 169, 2490, 2611, 181,
    2491, 3986, 2612, 2721, 2722, 193, 203, 1808, 204, 1809, 1819, 205, 47, 48, 49, 14, 1612, 2470,
    1678, 26, 1622, 3327, 3547, 1688, 2481, 3767, 2602, 1744, 1754, 38, 1631, 3336, 3556, 1697,
    3381, 1600, 3601, 3776, 3821, 1763, 2492, 3987, 2613, 3996, 4041, 2723, 1810, 1820, 1829, 50,
    170, 2501, 2622, 182, 2502, 4206, 2623, 2732, 2733, 194, 2503, 4207, 2624, 4216, 4261, 2734,
    2831, 2832, 2833, 206, 215, 1874, 216, 1875, 1885, 217, 1876, 1886, 1895, 218, 59, 60, 61, 62,
    15, 1613, 2471, 1679, 27, 1623, 3328, 3548, 1689, 2482, 3768, 2603, 1745, 1755, 39, 1632, 3337,
    3557, 1698, 3382, 6186, 3602, 3777, 3822, 1764, 2493, 3988, 2614, 3997, 4042, 2724, 1811, 1821,
    1830, 51, 1640, 3345, 3565, 1706, 3390, 6194, 3610, 3785, 3830, 1772, 3426, 6230, 3646, 1602,
    1601, 1602, 4005, 4050, 1602, 1838, 2504, 4208, 2625, 4217, 4262, 2735, 4225, 4270, 1602, 2834,
    1877, 1887, 1896, 1904, 63, 171, 2512, 2633, 183, 2513, 4426, 2634, 2743, 2744, 195, 2514,
    4427, 2635, 4436, 4481, 2745, 2842, 2843, 2844, 207, 2515, 4428, 2636, 4437, 4482, 2746, 4445,
    4490, 1602, 2845, 2930, 2931, 2932, 2933, 219, 227, 1940, 228, 1941, 1951, 229, 1942, 1952,
    1961, 230, 1943, 1953, 1962, 1970, 231, 71, 72, 73, 74, 75, 16, 172, 2472, 184, 28, 172, 2523,
    2644, 184, 2483, 2754, 2604, 196, 196, 40, 172, 2523, 2644, 184, 2524, 4646, 2645, 2754, 2755,
    196, 2494, 2853, 2615, 2853, 2854, 2725, 208, 208, 208, 52, 172, 2523, 2644, 184, 2524, 4646,
    2645, 2754, 2755, 196, 2525, 4647, 2646, 4656, 4701, 2756, 2853, 2854, 2855, 208, 2505, 2941,
    2626, 2941, 2942, 2736, 2941, 2942, 2943, 2835, 220, 220, 220, 220, 64, 172, 2523, 2644, 184,
    2524, 4646, 2645, 2754, 2755, 196, 2525, 4647, 2646, 4656, 4701, 2756, 2853, 2854, 2855, 208,
    2526, 4648, 2647, 4657, 4702, 2757, 4665, 4710, 1602, 2856, 2941, 2942, 2943, 2944, 220, 2516,
    3018, 2637, 3018, 3019, 2747, 3018, 3019, 3020, 2846, 3018, 3019, 3020, 3021, 2934, 232, 232,
    232, 232, 232, 76, 172, 239, 240, 184, 239, 2006, 240, 241, 241, 196, 239, 2006, 240, 2007,
    2017, 241, 242, 242, 242, 208, 239, 2006, 240, 2007, 2017, 241, 2008, 2018, 2027, 242, 243,
    243, 243, 243, 220, 239, 2006, 240, 2007, 2017, 241, 2008, 2018, 2027, 242, 2009, 2019, 2028,
    2036, 243, 244, 244, 244, 244, 244, 232, 83, 83, 84, 83, 84, 85, 83, 84, 85, 86, 83, 84, 85,
    86, 87, 83, 84, 85, 86, 87, 88, 11, 167, 179, 23, 12, 1610, 2468, 1676, 24, 168, 2479, 2600,
    180, 191, 1742, 192, 35, 36, 13, 1611, 2469, 1677, 25, 1621, 3326, 3546, 1687, 2480, 3766,
    2601, 1743, 1753, 37, 169, 2490, 2611, 181, 2491, 3986, 2612, 2721, 2722, 193, 203, 1808, 204,
    1809, 1819, 205, 47, 48, 49, 14, 1612, 2470, 1678, 26, 1622, 3327, 3547, 1688, 2481, 3767,
    2602, 1744, 1754, 38, 1631, 3336, 3556, 1697, 3381, 1600, 3601, 3776, 3821, 1763, 2492, 3987,
    2613, 3996, 4041, 2723, 1810, 1820, 1829, 50, 170, 2501, 2622, 182, 2502, 4206, 2623, 2732,
    2733, 194, 2503, 4207, 2624, 4216, 4261, 2734, 2831, 2832, 2833, 206, 215, 1874, 216, 1875,
    1885, 217, 1876, 1886, 1895, 218, 59, 60, 61, 62, 15, 1613, 2471, 1679, 27, 1623, 3328, 3548,
    1689, 2482, 3768, 2603, 1745, 1755, 39, 1632, 3337, 3557, 1698, 3382, 6186, 3602, 3777, 3822,
    1764, 2493, 3988, 2614, 3997, 4042, 2724, 1811, 1821, 1830, 51, 1640, 3345, 3565, 1706, 3390,
    6194, 3610, 3785, 3830, 1772, 3426, 6230, 3646, 6350, 1601, 3866, 4005, 4050, 4086, 1838, 2504,
    4208, 2625, 4217, 4262, 2735, 4225, 4270, 4306, 2834, 1877, 1887, 1896, 1904, 63, 171, 2512,
    2633, 183, 2513, 4426, 2634, 2743, 2744, 195, 2514, 4427, 2635, 4436, 4481, 2745, 2842, 2843,
    2844, 207, 2515, 4428, 2636, 4437, 4482, 2746, 4445, 4490, 4526, 2845, 2930, 2931, 2932, 2933,
    219, 227, 1940, 228, 1941, 1951, 229, 1942, 1952, 1961, 230, 1943, 1953, 1962, 1970, 231, 71,
    72, 73, 74, 75, 16, 1614, 2472, 1680, 28, 1624, 3329, 3549, 1690, 2483, 3769, 2604, 1746, 1756,
    40, 1633, 3338, 3558, 1699, 3383, 6187, 3603, 3778, 3823, 1765, 2494, 3989, 2615, 3998, 4043,
    2725, 1812, 1822, 1831, 52, 1641, 3346, 3566, 1707, 3391, 6195, 3611, 3786, 3831, 1773, 3427,
    6231, 3647, 6351, 6679, 3867, 4006, 4051, 4087, 1839, 2505, 4209, 2626, 4218, 4263, 2736, 4226,
    4271, 4307, 2835, 1878, 1888, 1897, 1905, 64, 1648, 3353, 3573, 1714, 3398, 6202, 3618, 3793,
    3838, 1780, 3434, 6238, 3654, 6358, 6686, 3874, 4013, 4058, 4094, 1846, 1604, 1604, 1604, 1604,
    1604, 1604, 1603, 1603, 1602, 1603, 1604, 1604, 1604, 1603, 1604, 2516, 4429, 2637, 4438, 4483,
    2747, 4446, 4491, 4527, 2846, 1604, 1604, 1604, 1603, 1604, 1944, 1954, 1963, 1971, 1604, 76,
    172, 2523, 2644, 184, 2524, 4646, 2645, 2754, 2755, 196, 2525, 4647, 2646, 4656, 4701, 2756,
    2853, 2854, 2855, 208, 2526, 4648, 2647, 4657, 4702, 2757, 4665, 4710, 4746, 2856, 2941, 2942,
    2943, 2944, 220, 2527, 4649, 2648, 4658, 4703, 2758, 4666, 4711, 4747, 2857, 1604, 1604, 1604,
    1603, 1604, 3018, 3019, 3020, 3021, 1604, 232, 239, 2006, 240, 2007, 2017, 241, 2008, 2018,
    2027, 242, 2009, 2019, 2028, 2036, 243, 2010, 2020, 2029, 2037, 1604, 244, 83, 84, 85, 86, 87,
    88, 17, 173, 2473, 185, 29, 173, 2534, 2655, 185, 2484, 2765, 2605, 197, 197, 41, 173, 2534,
    2655, 185, 2535, 4866, 2656, 2765, 2766, 197, 2495, 2864, 2616, 2864, 2865, 2726, 209, 209,
    209, 53, 173, 2534, 2655, 185, 2535, 4866, 2656, 2765, 2766, 197, 2536, 4867, 2657, 4876, 4921,
    2767, 2864, 2865, 2866, 209, 2506, 2952, 2627, 2952, 2953, 2737, 2952, 2953, 2954, 2836, 221,
    221, 221, 221, 65, 173, 2534, 2655, 185, 2535, 4866, 2656, 2765, 2766, 197, 2536, 4867, 2657,
    4876, 4921, 2767, 2864, 2865, 2866, 209, 2537, 4868, 2658, 4877, 4922, 2768, 4885, 4930, 4966,
    2867, 2952, 2953, 2954, 2955, 221, 2517, 3029, 2638, 3029, 3030, 2748, 3029, 3030, 3031, 2847,
    3029, 3030, 3031, 3032, 2935, 233, 233, 233, 233, 233, 77, 173, 2534, 2655, 185, 2535, 4866,
    2656, 2765, 2766, 197, 2536, 4867, 2657, 4876, 4921, 2767, 2864, 2865, 2866, 209, 2537, 4868,
    2658, 4877, 4922, 2768, 4885, 4930, 4966, 2867, 2952, 2953, 2954, 2955, 221, 2538, 4869, 2659,
    4878, 4923, 2769, 4886, 4931, 4967, 2868, 1604, 1604, 1604, 1603, 1604, 3029, 3030, 3031, 3032,
    1604, 233, 2528, 3095, 2649, 3095, 3096, 2759, 3095, 3096, 3097, 2858, 3095, 3096, 3097, 3098,
    2946, 3095, 3096, 3097, 3098, 1604, 3023, 245, 245, 245, 245, 245, 245, 89, 173, 251, 252, 185,
    251, 2072, 252, 253, 253, 197, 251, 2072, 252, 2073, 2083, 253, 254, 254, 254, 209, 251, 2072,
    252, 2073, 2083, 253, 2074, 2084, 2093, 254, 255, 255, 255, 255, 221, 251, 2072, 252, 2073,
    2083, 253, 2074, 2084, 2093, 254, 2075, 2085, 2094, 2102, 255, 256, 256, 256, 256, 256, 233,
    251, 2072, 252, 2073, 2083, 253, 2074, 2084, 2093, 254, 2075, 2085, 2094, 2102, 255, 2076,
    2086, 2095, 2103, 1604, 256, 257, 257, 257, 257, 257, 257, 245, 95, 95, 96, 95, 96, 97, 95, 96,
    97, 98, 95, 96, 97, 98, 99, 95, 96, 97, 98, 99, 100, 95, 96, 97, 98, 99, 100, 101, 11, 167,
    179, 23, 12, 174, 2468, 186, 24, 168, 2479, 2600, 180, 191, 198, 192, 35, 36, 13, 174, 2469,
    186, 25, 174, 2545, 2666, 186, 2480, 2776, 2601, 198, 198, 37, 169, 2490, 2611, 181, 2491,
    2875, 2612, 2721, 2722, 193, 203, 210, 204, 210, 210, 205, 47, 48, 49, 14, 174, 2470, 186, 26,
    174, 2545, 2666, 186, 2481, 2776, 2602, 198, 198, 38, 174, 2545, 2666, 186, 2546, 1600, 2667,
    2776, 2777, 198, 2492, 2875, 2613, 2875, 2876, 2723, 210, 210, 210, 50, 170, 2501, 2622, 182,
    2502, 2963, 2623, 2732, 2733, 194, 2503, 2963, 2624, 2963, 2964, 2734, 2831, 2832, 2833, 206,
    215, 222, 216, 222, 222, 217, 222, 222, 222, 218, 59, 60, 61, 62, 15, 174, 2471, 186, 27, 174,
    2545, 2666, 186, 2482, 2776, 2603, 198, 198, 39, 174, 2545, 2666, 186, 2546, 5086, 2667, 2776,
    2777, 198, 2493, 2875, 2614, 2875, 2876, 2724, 210, 210, 210, 51, 174, 2545, 2666, 186, 2546,
    5086, 2667, 2776, 2777, 198, 2547, 5087, 2668, 5096, 1601, 2778, 2875, 2876, 2877, 210, 2504,
    2963, 2625, 2963, 2964, 2735, 2963, 2964, 2965, 2834, 222, 222, 222, 222, 63, 171, 2512, 2633,
    183, 2513, 3040, 2634, 2743, 2744, 195, 2514, 3040, 2635, 3040, 3041, 2745, 2842, 2843, 2844,
    207, 2515, 3040, 2636, 3040, 3041, 2746, 3040, 3041, 3042, 2845, 2930, 2931, 2932, 2933, 219,
    227, 234, 228, 234, 234, 229, 234, 234, 234, 230, 234, 234, 234, 234, 231, 71, 72, 73, 74, 75,
    16, 174, 2472, 186, 28, 174, 2545, 2666, 186, 2483, 2776, 2604, 198, 198, 40, 174, 2545, 2666,
    186, 2546, 5086, 2667, 2776, 2777, 198, 2494, 2875, 2615, 2875, 2876, 2725, 210, 210, 210, 52,
    174, 2545, 2666, 186, 2546, 5086, 2667, 2776, 2777, 198, 2547, 5087, 2668, 5096, 5141, 2778,
    2875, 2876, 2877, 210, 2505, 2963, 2626, 2963, 2964, 2736, 2963, 2964, 2965, 2835, 222, 222,
    222, 222, 64, 174, 2545, 2666, 186, 2546, 5086, 2667, 2776, 2777, 198, 2547, 5087, 2668, 5096,
    5141, 2778, 2875, 2876, 2877, 210, 2548, 5088, 2669, 5097, 5142, 2779, 5105, 5150, 1602, 2878,
    2963, 2964, 2965, 2966, 222, 2516, 3040, 2637, 3040, 3041, 2747, 3040, 3041, 3042, 2846, 3040,
    3041, 3042, 3043, 2934, 234, 234, 234, 234, 234, 76, 172, 2523, 2644, 184, 2524, 3106, 2645,
    2754, 2755, 196, 2525, 3106, 2646, 3106, 3107, 2756, 2853, 2854, 2855, 208, 2526, 3106, 2647,
    3106, 3107, 2757, 3106, 3107, 3108, 2856, 2941, 2942, 2943, 2944, 220, 2527, 3106, 2648, 3106,
    3107, 2758, 3106, 3107, 3108, 2857, 3106, 3107, 3108, 3109, 2945, 3018, 3019, 3020, 3021, 3022,
    232, 239, 246, 240, 246, 246, 241, 246, 246, 246, 242, 246, 246, 246, 246, 243, 246, 246, 246,
    246, 246, 244, 83, 84, 85, 86, 87, 88, 17, 174, 2473, 186, 29, 174, 2545, 2666, 186, 2484,
    2776, 2605, 198, 198, 41, 174, 2545, 2666, 186, 2546, 5086, 2667, 2776, 2777, 198, 2495, 2875,
    2616, 2875, 2876, 2726, 210, 210, 210, 53, 174, 2545, 2666, 186, 2546, 5086, 2667, 2776, 2777,
    198, 2547, 5087, 2668, 5096, 5141, 2778, 2875, 2876, 2877, 210, 2506, 2963, 2627, 2963, 2964,
    2737, 2963, 2964, 2965, 2836, 222, 222, 222, 222, 65, 174, 2545, 2666, 186, 2546, 5086, 2667,
    2776, 2777, 198, 2547, 5087, 2668, 5096, 5141, 2778, 2875, 2876, 2877, 210, 2548, 5088, 2669,
    5097, 5142, 2779, 5105, 5150, 5186, 2878, 2963, 2964, 2965, 2966, 222, 2517, 3040, 2638, 3040,
    3041, 2748, 3040, 3041, 3042, 2847, 3040, 3041, 3042, 3043, 2935, 234, 234, 234, 234, 234, 77,
    174, 2545, 2666, 186, 2546, 5086, 2667, 2776, 2777, 198, 2547, 5087, 2668, 5096, 5141, 2778,
    2875, 2876, 2877, 210, 2548, 5088, 2669, 5097, 5142, 2779, 5105, 5150, 5186, 2878, 2963, 2964,
    2965, 2966, 222, 2549, 5089, 2670, 5098, 5143, 2780, 5106, 5151, 5187, 2879, 1604, 1604, 1604,
    1603, 1604, 3040, 3041, 3042, 3043, 1604, 234, 2528, 3106, 2649, 3106, 3107, 2759, 3106, 3107,
    3108, 2858, 3106, 3107, 3108, 3109, 2946, 3106, 3107, 3108, 3109, 1604, 3023, 246, 246, 246,
    246, 246, 246, 89, 173, 2534, 2655, 185, 2535, 3161, 2656, 2765, 2766, 197, 2536, 3161, 2657,
    3161, 3162, 2767, 2864, 2865, 2866, 209, 2537, 3161, 2658, 3161, 3162, 2768, 3161, 3162, 3163,
    2867, 2952, 2953, 2954, 2955, 221, 2538, 3161, 2659, 3161, 3162, 2769, 3161, 3162, 3163, 2868,
    3161, 3162, 3163, 3164, 2956, 3029, 3030, 3031, 3032, 3033, 233, 2539, 3161, 2660, 3161, 3162,
    2770, 3161, 3162, 3163, 2869, 3161, 3162, 3163, 3164, 2957, 3161, 3162, 3163, 3164, 1604, 3034,
    3095, 3096, 3097, 3098, 3099, 3100, 245, 251, 258, 252, 258, 258, 253, 258, 258, 258, 254, 258,
    258, 258, 258, 255, 258, 258, 258, 258, 258, 256, 258, 258, 258, 258, 258, 258, 257, 95, 96,
    97, 98, 99, 100, 101, 18, 174, 263, 186, 30, 174, 263, 264, 186, 263, 265, 264, 198, 198, 42,
    174, 263, 264, 186, 263, 2138, 264, 265, 265, 198, 263, 266, 264, 266, 266, 265, 210, 210, 210,
    54, 174, 263, 264, 186, 263, 2138, 264, 265, 265, 198, 263, 2138, 264, 2139, 2149, 265, 266,
    266, 266, 210, 263, 267, 264, 267, 267, 265, 267, 267, 267, 266, 222, 222, 222, 222, 66, 174,
    263, 264, 186, 263, 2138, 264, 265, 265, 198, 263, 2138, 264, 2139, 2149, 265, 266, 266, 266,
    210, 263, 2138, 264, 2139, 2149, 265, 2140, 2150, 2159, 266, 267, 267, 267, 267, 222, 263, 268,
    264, 268, 268, 265, 268, 268, 268, 266, 268, 268, 268, 268, 267, 234, 234, 234, 234, 234, 78,
    174, 263, 264, 186, 263, 2138, 264, 265, 265, 198, 263, 2138, 264, 2139, 2149, 265, 266, 266,
    266, 210, 263, 2138, 264, 2139, 2149, 265, 2140, 2150, 2159, 266, 267, 267, 267, 267, 222, 263,
    2138, 264, 2139, 2149, 265, 2140, 2150, 2159, 266, 2141, 2151, 2160, 2168, 267, 268, 268, 268,
    268, 268, 234, 263, 269, 264, 269, 269, 265, 269, 269, 269, 266, 269, 269, 269, 269, 267, 269,
    269, 269, 269, 269, 268, 246, 246, 246, 246, 246, 246, 90, 174, 263, 264, 186, 263, 2138, 264,
    265, 265, 198, 263, 2138, 264, 2139, 2149, 265, 266, 266, 266, 210, 263, 2138, 264, 2139, 2149,
    265, 2140, 2150, 2159, 266, 267, 267, 267, 267, 222, 263, 2138, 264, 2139, 2149, 265, 2140,
    2150, 2159, 266, 2141, 2151, 2160, 2168, 267, 268, 268, 268, 268, 268, 234, 263, 2138, 264,
    2139, 2149, 265, 2140, 2150, 2159, 266, 2141, 2151, 2160, 2168, 267, 2142, 2152, 2161, 2169,
    1604, 268, 269, 269, 269, 269, 269, 269, 246, 263, 270, 264, 270, 270, 265, 270, 270, 270, 266,
    270, 270, 270, 270, 267, 270, 270, 270, 270, 270, 268, 270, 270, 270, 270, 270, 270, 269, 258,
    258, 258, 258, 258, 258, 258, 102, 107, 107, 107, 108, 107, 107, 108, 107, 108, 109, 107, 107,
    108, 107, 108, 109, 107, 108, 109, 110, 107, 107, 108, 107, 108, 109, 107, 108, 109, 110, 107,
    108, 109, 110, 111, 107, 107, 108, 107, 108, 109, 107, 108, 109, 110, 107, 108, 109, 110, 111,
    107, 108, 109, 110, 111, 112, 107, 107, 108, 107, 108, 109, 107, 108, 109, 110, 107, 108, 109,
    110, 111, 107, 108, 109, 110, 111, 112, 107, 108, 109, 110, 111, 112, 113, 107, 107, 108, 107,
    108, 109, 107, 108, 109, 110, 107, 108, 109, 110, 111, 107, 108, 109, 110, 111, 112, 107, 108,
    109, 110, 111, 112, 113, 107, 108, 109, 110, 111, 112, 113, 114, 11, 167, 23, 11, 167, 179, 23,
    12, 168, 2468, 180, 24, 168, 191, 192, 180, 35, 35, 36, 11, 167, 179, 23, 12, 1610, 2468, 1676,
    24, 168, 2479, 2600, 180, 191, 1742, 192, 35, 36, 13, 169, 2469, 181, 25, 169, 2490, 2611, 181,
    2480, 2721, 2601, 193, 193, 37, 169, 203, 204, 181, 203, 1808, 204, 205, 205, 193, 47, 47, 48,
    47, 48, 49, 11, 167, 179, 23, 12, 1610, 2468, 1676, 24, 168, 2479, 2600, 180, 191, 1742, 192,
    35, 36, 13, 1611, 2469, 1677, 25, 1621, 1600, 1600, 1687, 2480, 1600, 2601, 1743, 1753, 37,
    169, 2490, 2611, 181, 2491, 1600, 2612, 2721, 2722, 193, 203, 1808, 204, 1809, 1819, 205, 47,
    48, 49, 14, 170, 2470, 182, 26, 170, 2501, 2622, 182, 2481, 2732, 2602, 194, 194, 38, 170,
    2501, 2622, 182, 2502, 1600, 2623, 2732, 2733, 194, 2492, 2831, 2613, 2831, 2832, 2723, 206,
    206, 206, 50, 170, 215, 216, 182, 215, 1874, 216, 217, 217, 194, 215, 1874, 216, 1875, 1885,
    217, 218, 218, 218, 206, 59, 59, 60, 59, 60, 61, 59, 60, 61, 62, 11, 167, 179, 23, 12, 1610,
    2468, 1676, 24, 168, 2479, 2600, 180, 191, 1742, 192, 35, 36, 13, 1611, 2469, 1677, 25, 1621,
    3326, 3546, 1687, 2480, 3766, 2601, 1743, 1753, 37, 169, 2490, 2611, 181, 2491, 3986, 2612,
    2721, 2722, 193, 203, 1808, 204, 1809, 1819, 205, 47, 48, 49, 14, 1612, 2470, 1678, 26, 1622,
    3327, 3547, 1688, 2481, 3767, 2602, 1744, 1754, 38, 1631, 3336, 3556, 1697, 3381, 1600, 1601,
    3776, 1601, 1763, 2492, 3987, 2613, 3996, 1601, 2723, 1810, 1820, 1829, 50, 170, 2501, 2622,
    182, 2502, 4206, 2623, 2732, 2733, 194, 2503, 4207, 2624, 4216, 1601, 2734, 2831, 2832, 2833,
    206, 215, 1874, 216, 1875, 1885, 217, 1876, 1886, 1895, 218, 59, 60, 61, 62, 15, 171, 2471,
    183, 27, 171, 2512, 2633, 183, 2482, 2743, 2603, 195, 195, 39, 171, 2512, 2633, 183, 2513,
    4426, 2634, 2743, 2744, 195, 2493, 2842, 2614, 2842, 2843, 2724, 207, 207, 207, 51, 171, 2512,
    2633, 183, 2513, 4426, 2634, 2743, 2744, 195, 2514, 4427, 2635, 4436, 1601, 2745, 2842, 2843,
    2844, 207, 2504, 2930, 2625, 2930, 2931, 2735, 2930, 2931, 2932, 2834, 219, 219, 219, 219, 63,
    171, 227, 228, 183, 227, 1940, 228, 229, 229, 195, 227, 1940, 228, 1941, 1951, 229, 230, 230,
    230, 207, 227, 1940, 228, 1941, 1951, 229, 1942, 1952, 1961, 230, 231, 231, 231, 231, 219, 71,
    71, 72, 71, 72, 73, 71, 72, 73, 74, 71, 72, 73, 74, 75, 11, 167, 179, 23, 12, 1610, 2468, 1676,
    24, 168, 2479, 2600, 180, 191, 1742, 192, 35, 36, 13, 1611, 2469, 1677, 25, 1621, 3326, 3546,
    1687, 2480, 3766, 2601, 1743, 1753, 37, 169, 2490, 2611, 181, 2491, 3986, 2612, 2721, 2722,
    193, 203, 1808, 204, 1809, 1819, 205, 47, 48, 49, 14, 1612, 2470, 1678, 26, 1622, 3327, 3547,
    1688, 2481, 3767, 2602, 1744, 1754, 38, 1631, 3336, 3556, 1697, 3381, 1600, 3601, 3776, 3821,
    1763, 2492, 3987, 2613, 3996, 4041, 2723, 1810, 1820, 1829, 50, 170, 2501, 2622, 182, 2502,
    4206, 2623, 2732, 2733, 194, 2503, 4207, 2624, 4216, 4261, 2734, 2831, 2832, 2833, 206, 215,
    1874, 216, 1875, 1885, 217, 1876, 1886, 1895, 218, 59, 60, 61, 62, 15, 1613, 2471, 1679, 27,
    1623, 3328, 3548, 1689, 2482, 3768, 2603, 1745, 1755, 39, 1632, 3337, 3557, 1698, 3382, 6186,
    3602, 3777, 3822, 1764, 2493, 3988, 2614, 3997, 4042, 2724, 1811, 1821, 1830, 51, 1640, 3345,
    3565, 1706, 3390, 6194, 3610, 3785, 3830, 1772, 3426, 6230, 3646, 1602, 1601, 1602, 4005, 4050,
    1602, 1838, 2504, 4208, 2625, 4217, 4262, 2735, 4225, 4270, 1602, 2834, 1877, 1887, 1896, 1904,
    63, 171, 2512, 2633, 183, 2513, 4426, 2634, 2743, 2744, 195, 2514, 4427, 2635, 4436, 4481,
    2745, 2842, 2843, 2844, 207, 2515, 4428, 2636, 4437, 4482, 2746, 4445, 4490, 1602, 2845, 2930,
    2931, 2932, 2933, 219, 227, 1940, 228, 1941, 1951, 229, 1942, 1952, 1961, 230, 1943, 1953,
    1962, 1970, 231, 71, 72, 73, 74, 75, 16, 172, 2472, 184, 28, 172, 2523, 2644, 184, 2483, 2754,
    2604, 196, 196, 40, 172, 2523, 2644, 184, 2524, 4646, 2645, 2754, 2755, 196, 2494, 2853, 2615,
    2853, 2854, 2725, 208, 208, 208, 52, 172, 2523, 2644, 184, 2524, 4646, 2645, 2754, 2755, 196,
    2525, 4647, 2646, 4656, 4701, 2756, 2853, 2854, 2855, 208, 2505, 2941, 2626, 2941, 2942, 2736,
    2941, 2942, 2943, 2835, 220, 220, 220, 220, 64, 172, 2523, 2644, 184, 2524, 4646, 2645, 2754,
    2755, 196, 2525, 4647, 2646, 4656, 4701, 2756, 2853, 2854, 2855, 208, 2526, 4648, 2647, 4657,
    4702, 2757, 4665, 4710, 1602, 2856, 2941, 2942, 2943, 2944, 220, 2516, 3018, 2637, 3018, 3019,
    2747, 3018, 3019, 3020, 2846, 3018, 3019, 3020, 3021, 2934, 232, 232, 232, 232, 232, 76, 172,
    239, 240, 184, 239, 2006, 240, 241, 241, 196, 239, 2006, 240, 2007, 2017, 241, 242, 242, 242,
    208, 239, 2006, 240, 2007, 2017, 241, 2008, 2018, 2027, 242, 243, 243, 243, 243, 220, 239,
    2006, 240, 2007, 2017, 241, 2008, 2018, 2027, 242, 2009, 2019, 2028, 2036, 243, 244, 244, 244,
    244, 244, 232, 83, 83, 84, 83, 84, 85, 83, 84, 85, 86, 83, 84, 85, 86, 87, 83, 84, 85, 86, 87,
    88, 11, 167, 179, 23, 12, 1610, 2468, 1676, 24, 168, 2479, 2600, 180, 191, 1742, 192, 35, 36,
    13, 1611, 2469, 1677, 25, 1621, 3326, 3546, 1687, 2480, 3766, 2601, 1743, 1753, 37, 169, 2490,
    2611, 181, 2491, 3986, 2612, 2721, 2722, 193, 203, 1808, 204, 1809, 1819, 205, 47, 48, 49, 14,
    1612, 2470, 1678, 26, 1622, 3327, 3547, 1688, 2481, 3767, 2602, 1744, 1754, 38, 1631, 3336,
    3556, 1697, 3381, 1600, 3601, 3776, 3821, 1763, 2492, 3987, 2613, 3996, 4041, 2723, 1810, 1820,
    1829, 50, 170, 2501, 2622, 182, 2502, 4206, 2623, 2732, 2733, 194, 2503, 4207, 2624, 4216,
    4261, 2734, 2831, 2832, 2833, 206, 215, 1874, 216, 1875, 1885, 217, 1876, 1886, 1895, 218, 59,
    60, 61, 62, 15, 1613, 2471, 1679, 27, 1623, 3328, 3548, 1689, 2482, 3768, 2603, 1745, 1755, 39,
    1632, 3337, 3557, 1698, 3382, 6186, 3602, 3777, 3822, 1764, 2493, 3988, 2614, 3997, 4042, 2724,
    1811, 1821, 1830, 51, 1640, 3345, 3565, 1706, 3390, 6194, 3610, 3785, 3830, 1772, 3426, 6230,
    3646, 6350, 1601, 3866, 4005, 4050, 4086, 1838, 2504, 4208, 2625, 4217, 4262, 2735, 4225, 4270,
    4306, 2834, 1877, 1887, 1896, 1904, 63, 171, 2512, 2633, 183, 2513, 4426, 2634, 2743, 2744,
    195, 2514, 4427, 2635, 4436, 4481, 2745, 2842, 2843, 2844, 207, 2515, 4428, 2636, 4437, 4482,
    2746, 4445, 4490, 4526, 2845, 2930, 2931, 2932, 2933, 219, 227, 1940, 228, 1941, 1951, 229,
    1942, 1952, 1961, 230, 1943, 1953, 1962, 1970, 231, 71, 72, 73, 74, 75, 16, 1614, 2472, 1680,
    28, 1624, 3329, 3549, 1690, 2483, 3769, 2604, 1746, 1756, 40, 1633, 3338, 3558, 1699, 3383,
    6187, 3603, 3778, 3823, 1765, 2494, 3989, 2615, 3998, 4043, 2725, 1812, 1822, 1831, 52, 1641,
    3346, 3566, 1707, 3391, 6195, 3611, 3786, 3831, 1773, 3427, 6231, 3647, 6351, 6679, 3867, 4006,
    4051, 4087, 1839, 2505, 4209, 2626, 4218, 4263, 2736, 4226, 4271, 4307, 2835, 1878, 1888, 1897,
    1905, 64, 1648, 3353, 3573, 1714, 3398, 6202, 3618, 3793, 3838, 1780, 3434, 6238, 3654, 6358,
    6686, 3874, 4013, 4058, 4094, 1846, 3462, 6266, 3682, 6386, 6714, 3902, 1603, 1603, 1602, 1603,
    4233, 4278, 4314, 1603, 1912, 2516, 4429, 2637, 4438, 4483, 2747, 4446, 4491, 4527, 2846, 4453,
    4498, 4534, 1603, 2934, 1944, 1954, 1963, 1971, 1978, 76, 172, 2523, 2644, 184, 2524, 4646,
    2645, 2754, 2755, 196, 2525, 4647, 2646, 4656, 4701, 2756, 2853, 2854, 2855, 208, 2526, 4648,
    2647, 4657, 4702, 2757, 4665, 4710, 4746, 2856, 2941, 2942, 2943, 2944, 220, 2527, 4649, 2648,
    4658, 4703, 2758, 4666, 4711, 4747, 2857, 4673, 4718, 4754, 1603, 2945, 3018, 3019, 3020, 3021,
    3022, 232, 239, 2006, 240, 2007, 2017, 241, 2008, 2018, 2027, 242, 2009, 2019, 2028, 2036, 243,
    2010, 2020, 2029, 2037, 2044, 244, 83, 84, 85, 86, 87, 88, 17, 173, 2473, 185, 29, 173, 2534,
    2655, 185, 2484, 2765, 2605, 197, 197, 41, 173, 2534, 2655, 185, 2535, 4866, 2656, 2765, 2766,
    197, 2495, 2864, 2616, 2864, 2865, 2726, 209, 209, 209, 53, 173, 2534, 2655, 185, 2535, 4866,
    2656, 2765, 2766, 197, 2536, 4867, 2657, 4876, 4921, 2767, 2864, 2865, 2866, 209, 2506, 2952,
    2627, 2952, 2953, 2737, 2952, 2953, 2954, 2836, 221, 221, 221, 221, 65, 173, 2534, 2655, 185,
    2535, 4866, 2656, 2765, 2766, 197, 2536, 4867, 2657, 4876, 4921, 2767, 2864, 2865, 2866, 209,
    2537, 4868, 2658, 4877, 4922, 2768, 4885, 4930, 4966, 2867, 2952, 2953, 2954, 2955, 221, 2517,
    3029, 2638, 3029, 3030, 2748, 3029, 3030, 3031, 2847, 3029, 3030, 3031, 3032, 2935, 233, 233,
    233, 233, 233, 77, 173, 2534, 2655, 185, 2535, 4866, 2656, 2765, 2766, 197, 2536, 4867, 2657,
    4876, 4921, 2767, 2864, 2865, 2866, 209, 2537, 4868, 2658, 4877, 4922, 2768, 4885, 4930, 4966,
    2867, 2952, 2953, 2954, 2955, 221, 2538, 4869, 2659, 4878, 4923, 2769, 4886, 4931, 4967, 2868,
    4893, 4938, 4974, 1603, 2956, 3029, 3030, 3031, 3032, 3033, 233, 2528, 3095, 2649, 3095, 3096,
    2759, 3095, 3096, 3097, 2858, 3095, 3096, 3097, 3098, 2946, 3095, 3096, 3097, 3098, 3099, 3023,
    245, 245, 245, 245, 245, 245, 89, 173, 251, 252, 185, 251, 2072, 252, 253, 253, 197, 251, 2072,
    252, 2073, 2083, 253, 254, 254, 254, 209, 251, 2072, 252, 2073, 2083, 253, 2074, 2084, 2093,
    254, 255, 255, 255, 255, 221, 251, 2072, 252, 2073, 2083, 253, 2074, 2084, 2093, 254, 2075,
    2085, 2094, 2102, 255, 256, 256, 256, 256, 256, 233, 251, 2072, 252, 2073, 2083, 253, 2074,
    2084, 2093, 254, 2075, 2085, 2094, 2102, 255, 2076, 2086, 2095, 2103, 2110, 256, 257, 257, 257,
    257, 257, 257, 245, 95, 95, 96, 95, 96, 97, 95, 96, 97, 98, 95, 96, 97, 98, 99, 95, 96, 97, 98,
    99, 100, 95, 96, 97, 98, 99, 100, 101, 11, 167, 179, 23, 12, 1610, 2468, 1676, 24, 168, 2479,
    2600, 180, 191, 1742, 192, 35, 36, 13, 1611, 2469, 1677, 25, 1621, 3326, 3546, 1687, 2480,
    3766, 2601, 1743, 1753, 37, 169, 2490, 2611, 181, 2491, 3986, 2612, 2721, 2722, 193, 203, 1808,
    204, 1809, 1819, 205, 47, 48, 49, 14, 1612, 2470, 1678, 26, 1622, 3327, 3547, 1688, 2481, 3767,
    2602, 1744, 1754, 38, 1631, 3336, 3556, 1697, 3381, 1600, 3601, 3776, 3821, 1763, 2492, 3987,
    2613, 3996, 4041, 2723, 1810, 1820, 1829, 50, 170, 2501, 2622, 182, 2502, 4206, 2623, 2732,
    2733, 194, 2503, 4207, 2624, 4216, 4261, 2734, 2831, 2832, 2833, 206, 215, 1874, 216, 1875,
    1885, 217, 1876, 1886, 1895, 218, 59, 60, 61, 62, 15, 1613, 2471, 1679, 27, 1623, 3328, 3548,
    1689, 2482, 3768, 2603, 1745, 1755, 39, 1632, 3337, 3557, 1698, 3382, 6186, 3602, 3777, 3822,
    1764, 2493, 3988, 2614, 3997, 4042, 2724, 1811, 1821, 1830, 51, 1640, 3345, 3565, 1706, 3390,
    6194, 3610, 3785, 3830, 1772, 3426, 6230, 3646, 6350, 1601, 3866, 4005, 4050, 4086, 1838, 2504,
    4208, 2625, 4217, 4262, 2735, 4225, 4270, 4306, 2834, 1877, 1887, 1896, 1904, 63, 171, 2512,
    2633, 183, 2513, 4426, 2634, 2743, 2744, 195, 2514, 4427, 2635, 4436, 4481, 2745, 2842, 2843,
    2844, 207, 2515, 4428, 2636, 4437, 4482, 2746, 4445, 4490, 4526, 2845, 2930, 2931, 2932, 2933,
    219, 227, 1940, 228, 1941, 1951, 229, 1942, 1952, 1961, 230, 1943, 1953, 1962, 1970, 231, 71,
    72, 73, 74, 75, 16, 1614, 2472, 1680, 28, 1624, 3329, 3549, 1690, 2483, 3769, 2604, 1746, 1756,
    40, 1633, 3338, 3558, 1699, 3383, 6187, 3603, 3778, 3823, 1765, 2494, 3989, 2615, 3998, 4043,
    2725, 1812, 1822, 1831, 52, 1641, 3346, 3566, 1707, 3391, 6195, 3611, 3786, 3831, 1773, 3427,
    6231, 3647, 6351, 6679, 3867, 4006, 4051, 4087, 1839, 2505, 4209, 2626, 4218, 4263, 2736, 4226,
    4271, 4307, 2835, 1878, 1888, 1897, 1905, 64, 1648, 3353, 3573, 1714, 3398, 6202, 3618, 3793,
    3838, 1780, 3434, 6238, 3654, 6358, 6686, 3874, 4013, 4058, 4094, 1846, 3462, 6266, 3682, 6386,
    6714, 3902, 6470, 6798, 1602, 4122, 4233, 4278, 4314, 4342, 1912, 2516, 4429, 2637, 4438, 4483,
    2747, 4446, 4491, 4527, 2846, 4453, 4498, 4534, 4562, 2934, 1944, 1954, 1963, 1971, 1978, 76,
    172, 2523, 2644, 184, 2524, 4646, 2645, 2754, 2755, 196, 2525, 4647, 2646, 4656, 4701, 2756,
    2853, 2854, 2855, 208, 2526, 4648, 2647, 4657, 4702, 2757, 4665, 4710, 4746, 2856, 2941, 2942,
    2943, 2944, 220, 2527, 4649, 2648, 4658, 4703, 2758, 4666, 4711, 4747, 2857, 4673, 4718, 4754,
    4782, 2945, 3018, 3019, 3020, 3021, 3022, 232, 239, 2006, 240, 2007, 2017, 241, 2008, 2018,
    2027, 242, 2009, 2019, 2028, 2036, 243, 2010, 2020, 2029, 2037, 2044, 244, 83, 84, 85, 86, 87,
    88, 17, 1615, 2473, 1681, 29, 1625, 3330, 3550, 1691, 2484, 3770, 2605, 1747, 1757, 41, 1634,
    3339, 3559, 1700, 3384, 6188, 3604, 3779, 3824, 1766, 2495, 3990, 2616, 3999, 4044, 2726, 1813,
    1823, 1832, 53, 1642, 3347, 3567, 1708, 3392, 6196, 3612, 3787, 3832, 1774, 3428, 6232, 3648,
    6352, 6680, 3868, 4007, 4052, 4088, 1840, 2506, 4210, 2627, 4219, 4264, 2737, 4227, 4272, 4308,
    2836, 1879, 1889, 1898, 1906, 65, 1649, 3354, 3574, 1715, 3399, 6203, 3619, 3794, 3839, 1781,
    3435, 6239, 3655, 6359, 6687, 3875, 4014, 4059, 4095, 1847, 3463, 6267, 3683, 6387, 6715, 3903,
    6471, 6799, 7008, 4123, 4234, 4279, 4315, 4343, 1913, 2517, 4430, 2638, 4439, 4484, 2748, 4447,
    4492, 4528, 2847, 4454, 4499, 4535, 4563, 2935, 1945, 1955, 1964, 1972, 1979, 77, 1655, 3360,
    3580, 1721, 3405, 6209, 3625, 3800, 3845, 1787, 3441, 6245, 3661, 6365, 6693, 3881, 4020, 4065,
    4101, 1853, 3469, 6273, 3689, 6393, 6721, 3909, 6477, 6805, 7014, 4129, 4240, 4285, 4321, 4349,
    1919, 1605, 1605, 1605, 1605, 1605, 1605, 1605, 1605, 1605, 1605, 1604, 1604, 1604, 1603, 1604,
    1605, 1605, 1605, 1605, 1604, 1605, 2528, 4650, 2649, 4659, 4704, 2759, 4667, 4712, 4748, 2858,
    4674, 4719, 4755, 4783, 2946, 1605, 1605, 1605, 1605, 1604, 1605, 2011, 2021, 2030, 2038, 2045,
    1605, 89, 173, 2534, 2655, 185, 2535, 4866, 2656, 2765, 2766, 197, 2536, 4867, 2657, 4876,
    4921, 2767, 2864, 2865, 2866, 209, 2537, 4868, 2658, 4877, 4922, 2768, 4885, 4930, 4966, 2867,
    2952, 2953, 2954, 2955, 221, 2538, 4869, 2659, 4878, 4923, 2769, 4886, 4931, 4967, 2868, 4893,
    4938, 4974, 5002, 2956, 3029, 3030, 3031, 3032, 3033, 233, 2539, 4870, 2660, 4879, 4924, 2770,
    4887, 4932, 4968, 2869, 4894, 4939, 4975, 5003, 2957, 1605, 1605, 1605, 1605, 1604, 1605, 3095,
    3096, 3097, 3098, 3099, 1605, 245, 251, 2072, 252, 2073, 2083, 253, 2074, 2084, 2093, 254,
    2075, 2085, 2094, 2102, 255, 2076, 2086, 2095, 2103, 2110, 256, 2077, 2087, 2096, 2104, 2111,
    1605, 257, 95, 96, 97, 98, 99, 100, 101, 18, 174, 2474, 186, 30, 174, 2545, 2666, 186, 2485,
    2776, 2606, 198, 198, 42, 174, 2545, 2666, 186, 2546, 5086, 2667, 2776, 2777, 198, 2496, 2875,
    2617, 2875, 2876, 2727, 210, 210, 210, 54, 174, 2545, 2666, 186, 2546, 5086, 2667, 2776, 2777,
    198, 2547, 5087, 2668, 5096, 5141, 2778, 2875, 2876, 2877, 210, 2507, 2963, 2628, 2963, 2964,
    2738, 2963, 2964, 2965, 2837, 222, 222, 222, 222, 66, 174, 2545, 2666, 186, 2546, 5086, 2667,
    2776, 2777, 198, 2547, 5087, 2668, 5096, 5141, 2778, 2875, 2876, 2877, 210, 2548, 5088, 2669,
    5097, 5142, 2779, 5105, 5150, 5186, 2878, 2963, 2964, 2965, 2966, 222, 2518, 3040, 2639, 3040,
    3041, 2749, 3040, 3041, 3042, 2848, 3040, 3041, 3042, 3043, 2936, 234, 234, 234, 234, 234, 78,
    174, 2545, 2666, 186, 2546, 5086, 2667, 2776, 2777, 198, 2547, 5087, 2668, 5096, 5141, 2778,
    2875, 2876, 2877, 210, 2548, 5088, 2669, 5097, 5142, 2779, 5105, 5150, 5186, 2878, 2963, 2964,
    2965, 2966, 222, 2549, 5089, 2670, 5098, 5143, 2780, 5106, 5151, 5187, 2879, 5113, 5158, 5194,
    5222, 2967, 3040, 3041, 3042, 3043, 3044, 234, 2529, 3106, 2650, 3106, 3107, 2760, 3106, 3107,
    3108, 2859, 3106, 3107, 3108, 3109, 2947, 3106, 3107, 3108, 3109, 3110, 3024, 246, 246, 246,
    246, 246, 246, 90, 174, 2545, 2666, 186, 2546, 5086, 2667, 2776, 2777, 198, 2547, 5087, 2668,
    5096, 5141, 2778, 2875, 2876, 2877, 210, 2548, 5088, 2669, 5097, 5142, 2779, 5105, 5150, 5186,
    2878, 2963, 2964, 2965, 2966, 222, 2549, 5089, 2670, 5098, 5143, 2780, 5106, 5151, 5187, 2879,
    5113, 5158, 5194, 5222, 2967, 3040, 3041, 3042, 3043, 3044, 234, 2550, 5090, 2671, 5099, 5144,
    2781, 5107, 5152, 5188, 2880, 5114, 5159, 5195, 5223, 2968, 1605, 1605, 1605, 1605, 1604, 1605,
    3106, 3107, 3108, 3109, 3110, 1605, 246, 2540, 3161, 2661, 3161, 3162, 2771, 3161, 3162, 3163,
    2870, 3161, 3162, 3163, 3164, 2958, 3161, 3162, 3163, 3164, 3165, 3035, 3161, 3162, 3163, 3164,
    3165, 1605, 3101, 258, 258, 258, 258, 258, 258, 258, 102, 174, 263, 264, 186, 263, 2138, 264,
    265, 265, 198, 263, 2138, 264, 2139, 2149, 265, 266, 266, 266, 210, 263, 2138, 264, 2139, 2149,
    265, 2140, 2150, 2159, 266, 267, 267, 267, 267, 222, 263, 2138, 264, 2139, 2149, 265, 2140,
    2150, 2159, 266, 2141, 2151, 2160, 2168, 267, 268, 268, 268, 268, 268, 234, 263, 2138, 264,
    2139, 2149, 265, 2140, 2150, 2159, 266, 2141, 2151, 2160, 2168, 267, 2142, 2152, 2161, 2169,
    2176, 268, 269, 269, 269, 269, 269, 269, 246, 263, 2138, 264, 2139, 2149, 265, 2140, 2150,
    2159, 266, 2141, 2151, 2160, 2168, 267, 2142, 2152, 2161, 2169, 2176, 268, 2143, 2153, 2162,
    2170, 2177, 1605, 269, 270, 270, 270, 270, 270, 270, 270, 258, 107, 107, 108, 107, 108, 109,
    107, 108, 109, 110, 107, 108, 109, 110, 111, 107, 108, 109, 110, 111, 112, 107, 108, 109, 110,
    111, 112, 113, 107, 108, 109, 110, 111, 112, 113, 114, 11, 167, 179, 23, 12, 175, 2468, 187,
    24, 168, 2479, 2600, 180, 191, 199, 192, 35, 36, 13, 175, 2469, 187, 25, 175, 2556, 2677, 187,
    2480, 2787, 2601, 199, 199, 37, 169, 2490, 2611, 181, 2491, 2886, 2612, 2721, 2722, 193, 203,
    211, 204, 211, 211, 205, 47, 48, 49, 14, 175, 2470, 187, 26, 175, 2556, 2677, 187, 2481, 2787,
    2602, 199, 199, 38, 175, 2556, 2677, 187, 2557, 1600, 2678, 2787, 2788, 199, 2492, 2886, 2613,
    2886, 2887, 2723, 211, 211, 211, 50, 170, 2501, 2622, 182, 2502, 2974, 2623, 2732, 2733, 194,
    2503, 2974, 2624, 2974, 2975, 2734, 2831, 2832, 2833, 206, 215, 223, 216, 223, 223, 217, 223,
    223, 223, 218, 59, 60, 61, 62, 15, 175, 2471, 187, 27, 175, 2556, 2677, 187, 2482, 2787, 2603,
    199, 199, 39, 175, 2556, 2677, 187, 2557, 5306, 2678, 2787, 2788, 199, 2493, 2886, 2614, 2886,
    2887, 2724, 211, 211, 211, 51, 175, 2556, 2677, 187, 2557, 5306, 2678, 2787, 2788, 199, 2558,
    5307, 2679, 5316, 1601, 2789, 2886, 2887, 2888, 211, 2504, 2974, 2625, 2974, 2975, 2735, 2974,
    2975, 2976, 2834, 223, 223, 223, 223, 63, 171, 2512, 2633, 183, 2513, 3051, 2634, 2743, 2744,
    195, 2514, 3051, 2635, 3051, 3052, 2745, 2842, 2843, 2844, 207, 2515, 3051, 2636, 3051, 3052,
    2746, 3051, 3052, 3053, 2845, 2930, 2931, 2932, 2933, 219, 227, 235, 228, 235, 235, 229, 235,
    235, 235, 230, 235, 235, 235, 235, 231, 71, 72, 73, 74, 75, 16, 175, 2472, 187, 28, 175, 2556,
    2677, 187, 2483, 2787, 2604, 199, 199, 40, 175, 2556, 2677, 187, 2557, 5306, 2678, 2787, 2788,
    199, 2494, 2886, 2615, 2886, 2887, 2725, 211, 211, 211, 52, 175, 2556, 2677, 187, 2557, 5306,
    2678, 2787, 2788, 199, 2558, 5307, 2679, 5316, 5361, 2789, 2886, 2887, 2888, 211, 2505, 2974,
    2626, 2974, 2975, 2736, 2974, 2975, 2976, 2835, 223, 223, 223, 223, 64, 175, 2556, 2677, 187,
    2557, 5306, 2678, 2787, 2788, 199, 2558, 5307, 2679, 5316, 5361, 2789, 2886, 2887, 2888, 211,
    2559, 5308, 2680, 5317, 5362, 2790, 5325, 5370, 1602, 2889, 2974, 2975, 2976, 2977, 223, 2516,
    3051, 2637, 3051, 3052, 2747, 3051, 3052, 3053, 2846, 3051, 3052, 3053, 3054, 2934, 235, 235,
    235, 235, 235, 76, 172, 2523, 2644, 184, 2524, 3117, 2645, 2754, 2755, 196, 2525, 3117, 2646,
    3117, 3118, 2756, 2853, 2854, 2855, 208, 2526, 3117, 2647, 3117, 3118, 2757, 3117, 3118, 3119,
    2856, 2941, 2942, 2943, 2944, 220, 2527, 3117, 2648, 3117, 3118, 2758, 3117, 3118, 3119, 2857,
    3117, 3118, 3119, 3120, 2945, 3018, 3019, 3020, 3021, 3022, 232, 239, 247, 240, 247, 247, 241,
    247, 247, 247, 242, 247, 247, 247, 247, 243, 247, 247, 247, 247, 247, 244, 83, 84, 85, 86, 87,
    88, 17, 175, 2473, 187, 29, 175, 2556, 2677, 187, 2484, 2787, 2605, 199, 199, 41, 175, 2556,
    2677, 187, 2557, 5306, 2678, 2787, 2788, 199, 2495, 2886, 2616, 2886, 2887, 2726, 211, 211,
    211, 53, 175, 2556, 2677, 187, 2557, 5306, 2678, 2787, 2788, 199, 2558, 5307, 2679, 5316, 5361,
    2789, 2886, 2887, 2888, 211, 2506, 2974, 2627, 2974, 2975, 2737, 2974, 2975, 2976, 2836, 223,
    223, 223, 223, 65, 175, 2556, 2677, 187, 2557, 5306, 2678, 2787, 2788, 199, 2558, 5307, 2679,
    5316, 5361, 2789, 2886, 2887, 2888, 211, 2559, 5308, 2680, 5317, 5362, 2790, 5325, 5370, 5406,
    2889, 2974, 2975, 2976, 2977, 223, 2517, 3051, 2638, 3051, 3052, 2748, 3051, 3052, 3053, 2847,
    3051, 3052, 3053, 3054, 2935, 235, 235, 235, 235, 235, 77, 175, 2556, 2677, 187, 2557, 5306,
    2678, 2787, 2788, 199, 2558, 5307, 2679, 5316, 5361, 2789, 2886, 2887, 2888, 211, 2559, 5308,
    2680, 5317, 5362, 2790, 5325, 5370, 5406, 2889, 2974, 2975, 2976, 2977, 223, 2560, 5309, 2681,
    5318, 5363, 2791, 5326, 5371, 5407, 2890, 5333, 5378, 5414, 1603, 2978, 3051, 3052, 3053, 3054,
    3055, 235, 2528, 3117, 2649, 3117, 3118, 2759, 3117, 3118, 3119, 2858, 3117, 3118, 3119, 3120,
    2946, 3117, 3118, 3119, 3120, 3121, 3023, 247, 247, 247, 247, 247, 247, 89, 173, 2534, 2655,
    185, 2535, 3172, 2656, 2765, 2766, 197, 2536, 3172, 2657, 3172, 3173, 2767, 2864, 2865, 2866,
    209, 2537, 3172, 2658, 3172, 3173, 2768, 3172, 3173, 3174, 2867, 2952, 2953, 2954, 2955, 221,
    2538, 3172, 2659, 3172, 3173, 2769, 3172, 3173, 3174, 2868, 3172, 3173, 3174, 3175, 2956, 3029,
    3030, 3031, 3032, 3033, 233, 2539, 3172, 2660, 3172, 3173, 2770, 3172, 3173, 3174, 2869, 3172,
    3173, 3174, 3175, 2957, 3172, 3173, 3174, 3175, 3176, 3034, 3095, 3096, 3097, 3098, 3099, 3100,
    245, 251, 259, 252, 259, 259, 253, 259, 259, 259, 254, 259, 259, 259, 259, 255, 259, 259, 259,
    259, 259, 256, 259, 259, 259, 259, 259, 259, 257, 95, 96, 97, 98, 99, 100, 101, 18, 175, 2474,
    187, 30, 175, 2556, 2677, 187, 2485, 2787, 2606, 199, 199, 42, 175, 2556, 2677, 187, 2557,
    5306, 2678, 2787, 2788, 199, 2496, 2886, 2617, 2886, 2887, 2727, 211, 211, 211, 54, 175, 2556,
    2677, 187, 2557, 5306, 2678, 2787, 2788, 199, 2558, 5307, 2679, 5316, 5361, 2789, 2886, 2887,
    2888, 211, 2507, 2974, 2628, 2974, 2975, 2738, 2974, 2975, 2976, 2837, 223, 223, 223, 223, 66,
    175, 2556, 2677, 187, 2557, 5306, 2678, 2787, 2788, 199, 2558, 5307, 2679, 5316, 5361, 2789,
    2886, 2887, 2888, 211, 2559, 5308, 2680, 5317, 5362, 2790, 5325, 5370, 5406, 2889, 2974, 2975,
    2976, 2977, 223, 2518, 3051, 2639, 3051, 3052, 2749, 3051, 3052, 3053, 2848, 3051, 3052, 3053,
    3054, 2936, 235, 235, 235, 235, 235, 78, 175, 2556, 2677, 187, 2557, 5306, 2678, 2787, 2788,
    199, 2558, 5307, 2679, 5316, 5361, 2789, 2886, 2887, 2888, 211, 2559, 5308, 2680, 5317, 5362,
    2790, 5325, 5370, 5406, 2889, 2974, 2975, 2976, 2977, 223, 2560, 5309, 2681, 5318, 5363, 2791,
    5326, 5371, 5407, 2890, 5333, 5378, 5414, 5442, 2978, 3051, 3052, 3053, 3054, 3055, 235, 2529,
    3117, 2650, 3117, 3118, 2760, 3117, 3118, 3119, 2859, 3117, 3118, 3119, 3120, 2947, 3117, 3118,
    3119, 3120, 3121, 3024, 247, 247, 247, 247, 247, 247, 90, 175, 2556, 2677, 187, 2557, 5306,
    2678, 2787, 2788, 199, 2558, 5307, 2679, 5316, 5361, 2789, 2886, 2887, 2888, 211, 2559, 5308,
    2680, 5317, 5362, 2790, 5325, 5370, 5406, 2889, 2974, 2975, 2976, 2977, 223, 2560, 5309, 2681,
    5318, 5363, 2791, 5326, 5371, 5407, 2890, 5333, 5378, 5414, 5442, 2978, 3051, 3052, 3053, 3054,
    3055, 235, 2561, 5310, 2682, 5319, 5364, 2792, 5327, 5372, 5408, 2891, 5334, 5379, 5415, 5443,
    2979, 1605, 1605, 1605, 1605, 1604, 1605, 3117, 3118, 3119, 3120, 3121, 1605, 247, 2540, 3172,
    2661, 3172, 3173, 2771, 3172, 3173, 3174, 2870, 3172, 3173, 3174, 3175, 2958, 3172, 3173, 3174,
    3175, 3176, 3035, 3172, 3173, 3174, 3175, 3176, 1605, 3101, 259, 259, 259, 259, 259, 259, 259,
    102, 174, 2545, 2666, 186, 2546, 3216, 2667, 2776, 2777, 198, 2547, 3216, 2668, 3216, 3217,
    2778, 2875, 2876, 2877, 210, 2548, 3216, 2669, 3216, 3217, 2779, 3216, 3217, 3218, 2878, 2963,
    2964, 2965, 2966, 222, 2549, 3216, 2670, 3216, 3217, 2780, 3216, 3217, 3218, 2879, 3216, 3217,
    3218, 3219, 2967, 3040, 3041, 3042, 3043, 3044, 234, 2550, 3216, 2671, 3216, 3217, 2781, 3216,
    3217, 3218, 2880, 3216, 3217, 3218, 3219, 2968, 3216, 3217, 3218, 3219, 3220, 3045, 3106, 3107,
    3108, 3109, 3110, 3111, 246, 2551, 3216, 2672, 3216, 3217, 2782, 3216, 3217, 3218, 2881, 3216,
    3217, 3218, 3219, 2969, 3216, 3217, 3218, 3219, 3220, 3046, 3216, 3217, 3218, 3219, 3220, 1605,
    3112, 3161, 3162, 3163, 3164, 3165, 3166, 3167, 258, 263, 271, 264, 271, 271, 265, 271, 271,
    271, 266, 271, 271, 271, 271, 267, 271, 271, 271, 271, 271, 268, 271, 271, 271, 271, 271, 271,
    269, 271, 271, 271, 271, 271, 271, 271, 270, 107, 108, 109, 110, 111, 112, 113, 114, 19, 175,
    275, 187, 31, 175, 275, 276, 187, 275, 277, 276, 199, 199, 43, 175, 275, 276, 187, 275, 2204,
    276, 277, 277, 199, 275, 278, 276, 278, 278, 277, 211, 211, 211, 55, 175, 275, 276, 187, 275,
    2204, 276, 277, 277, 199, 275, 2204, 276, 2205, 2215, 277, 278, 278, 278, 211, 275, 279, 276,
    279, 279, 277, 279, 279, 279, 278, 223, 223, 223, 223, 67, 175, 275, 276, 187, 275, 2204, 276,
    277, 277, 199, 275, 2204, 276, 2205, 2215, 277, 278, 278, 278, 211, 275, 2204, 276, 2205, 2215,
    277, 2206, 2216, 2225, 278, 279, 279, 279, 279, 223, 275, 280, 276, 280, 280, 277, 280, 280,
    280, 278, 280, 280, 280, 280, 279, 235, 235, 235, 235, 235, 79, 175, 275, 276, 187, 275, 2204,
    276, 277, 277, 199, 275, 2204, 276, 2205, 2215, 277, 278, 278, 278, 211, 275, 2204, 276, 2205,
    2215, 277, 2206, 2216, 2225, 278, 279, 279, 279, 279, 223, 275, 2204, 276, 2205, 2215, 277,
    2206, 2216, 2225, 278, 2207, 2217, 2226, 2234, 279, 280, 280, 280, 280, 280, 235, 275, 281,
    276, 281, 281, 277, 281, 281, 281, 278, 281, 281, 281, 281, 279, 281, 281, 281, 281, 281, 280,
    247, 247, 247, 247, 247, 247, 91, 175, 275, 276, 187, 275, 2204, 276, 277, 277, 199, 275, 2204,
    276, 2205, 2215, 277, 278, 278, 278, 211, 275, 2204, 276, 2205, 2215, 277, 2206, 2216, 2225,
    278, 279, 279, 279, 279, 223, 275, 2204, 276, 2205, 2215, 277, 2206, 2216, 2225, 278, 2207,
    2217, 2226, 2234, 279, 280, 280, 280, 280, 280, 235, 275, 2204, 276, 2205, 2215, 277, 2206,
    2216, 2225, 278, 2207, 2217, 2226, 2234, 279, 2208, 2218, 2227, 2235, 2242, 280, 281, 281, 281,
    281, 281, 281, 247, 275, 282, 276, 282, 282, 277, 282, 282, 282, 278, 282, 282, 282, 282, 279,
    282, 282, 282, 282, 282, 280, 282, 282, 282, 282, 282, 282, 281, 259, 259, 259, 259, 259, 259,
    259, 103, 175, 275, 276, 187, 275, 2204, 276, 277, 277, 199, 275, 2204, 276, 2205, 2215, 277,
    278, 278, 278, 211, 275, 2204, 276, 2205, 2215, 277, 2206, 2216, 2225, 278, 279, 279, 279, 279,
    223, 275, 2204, 276, 2205, 2215, 277, 2206, 2216, 2225, 278, 2207, 2217, 2226, 2234, 279, 280,
    280, 280, 280, 280, 235, 275, 2204, 276, 2205, 2215, 277, 2206, 2216, 2225, 278, 2207, 2217,
    2226, 2234, 279, 2208, 2218, 2227, 2235, 2242, 280, 281, 281, 281, 281, 281, 281, 247, 275,
    2204, 276, 2205, 2215, 277, 2206, 2216, 2225, 278, 2207, 2217, 2226, 2234, 279, 2208, 2218,
    2227, 2235, 2242, 280, 2209, 2219, 2228, 2236, 2243, 1605, 281, 282, 282, 282, 282, 282, 282,
    282, 259, 275, 283, 276, 283, 283, 277, 283, 283, 283, 278, 283, 283, 283, 283, 279, 283, 283,
    283, 283, 283, 280, 283, 283, 283, 283, 283, 283, 281, 283, 283, 283, 283, 283, 283, 283, 282,
    271, 271, 271, 271, 271, 271, 271, 271, 115, 119, 119, 119, 120, 119, 119, 120, 119, 120, 121,
    119, 119, 120, 119, 120, 121, 119, 120, 121, 122, 119, 119, 120, 119, 120, 121, 119, 120, 121,
    122, 119, 120, 121, 122, 123, 119, 119, 120, 119, 120, 121, 119, 120, 121, 122, 119, 120, 121,
    122, 123, 119, 120, 121, 122, 123, 124, 119, 119, 120, 119, 120, 121, 119, 120, 121, 122, 119,
    120, 121, 122, 123, 119, 120, 121, 122, 123, 124, 119, 120, 121, 122, 123, 124, 125, 119, 119,
    120, 119, 120, 121, 119, 120, 121, 122, 119, 120, 121, 122, 123, 119, 120, 121, 122, 123, 124,
    119, 120, 121, 122, 123, 124, 125, 119, 120, 121, 122, 123, 124, 125, 126, 119, 119, 120, 119,
    120, 121, 119, 120, 121, 122, 119, 120, 121, 122, 123, 119, 120, 121, 122, 123, 124, 119, 120,
    121, 122, 123, 124, 125, 119, 120, 121, 122, 123, 124, 125, 126, 119, 120, 121, 122, 123, 124,
    125, 126, 127, 11, 167, 23, 11, 167, 179, 23, 12, 168, 2468, 180, 24, 168, 191, 192, 180, 35,
    35, 36, 11, 167, 179, 23, 12, 1610, 2468, 1676, 24, 168, 2479, 2600, 180, 191, 1742, 192, 35,
    36, 13, 169, 2469, 181, 25, 169, 2490, 2611, 181, 2480, 2721, 2601, 193, 193, 37, 169, 203,
    204, 181, 203, 1808, 204, 205, 205, 193, 47, 47, 48, 47, 48, 49, 11, 167, 179, 23, 12, 1610,
    2468, 1676, 24, 168, 2479, 2600, 180, 191, 1742, 192, 35, 36, 13, 1611, 2469, 1677, 25, 1621,
    1600, 1600, 1687, 2480, 1600, 2601, 1743, 1753, 37, 169, 2490, 2611, 181, 2491, 1600, 2612,
    2721, 2722, 193, 203, 1808, 204, 1809, 1819, 205, 47, 48, 49, 14, 170, 2470, 182, 26, 170,
    2501, 2622, 182, 2481, 2732, 2602, 194, 194, 38, 170, 2501, 2622, 182, 2502, 1600, 2623, 2732,
    2733, 194, 2492, 2831, 2613, 2831, 2832, 2723, 206, 206, 206, 50, 170, 215, 216, 182, 215,
    1874, 216, 217, 217, 194, 215, 1874, 216, 1875, 1885, 217, 218, 218, 218, 206, 59, 59, 60, 59,
    60, 61, 59, 60, 61, 62, 11, 167, 179, 23, 12, 1610, 2468, 1676, 24, 168, 2479, 2600, 180, 191,
    1742, 192, 35, 36, 13, 1611, 2469, 1677, 25, 1621, 3326, 3546, 1687, 2480, 3766, 2601, 1743,
    1753, 37, 169, 2490, 2611, 181, 2491, 3986, 2612, 2721, 2722, 193, 203, 1808, 204, 1809, 1819,
    205, 47, 48, 49, 14, 1612, 2470, 1678, 26, 1622, 3327, 3547, 1688, 2481, 3767, 2602, 1744,
    1754, 38, 1631, 3336, 3556, 1697, 3381, 1600, 1601, 3776, 1601, 1763, 2492, 3987, 2613, 3996,
    1601, 2723, 1810, 1820, 1829, 50, 170, 2501, 2622, 182, 2502, 4206, 2623, 2732, 2733, 194,
    2503, 4207, 2624, 4216, 1601, 2734, 2831, 2832, 2833, 206, 215, 1874, 216, 1875, 1885, 217,
    1876, 1886, 1895, 218, 59, 60, 61, 62, 15, 171, 2471, 183, 27, 171, 2512, 2633, 183, 2482,
    2743, 2603, 195, 195, 39, 171, 2512, 2633, 183, 2513, 4426, 2634, 2743, 2744, 195, 2493, 2842,
    2614, 2842, 2843, 2724, 207, 207, 207, 51, 171, 2512, 2633, 183, 2513, 4426, 2634, 2743, 2744,
    195, 2514, 4427, 2635, 4436, 1601, 2745, 2842, 2843, 2844, 207, 2504, 2930, 2625, 2930, 2931,
    2735, 2930, 2931, 2932, 2834, 219, 219, 219, 219, 63, 171, 227, 228, 183, 227, 1940, 228, 229,
    229, 195, 227, 1940, 228, 1941, 1951, 229, 230, 230, 230, 207, 227, 1940, 228, 1941, 1951, 229,
    1942, 1952, 1961, 230, 231, 231, 231, 231, 219, 71, 71, 72, 71, 72, 73, 71, 72, 73, 74, 71, 72,
    73, 74, 75, 11, 167, 179, 23, 12, 1610, 2468, 1676, 24, 168, 2479, 2600, 180, 191, 1742, 192,
    35, 36, 13, 1611, 2469, 1677, 25, 1621, 3326, 3546, 1687, 2480, 3766, 2601, 1743, 1753, 37,
    169, 2490, 2611, 181, 2491, 3986, 2612, 2721, 2722, 193, 203, 1808, 204, 1809, 1819, 205, 47,
    48, 49, 14, 1612, 2470, 1678, 26, 1622, 3327, 3547, 1688, 2481, 3767, 2602, 1744, 1754, 38,
    1631, 3336, 3556, 1697, 3381, 1600, 3601, 3776, 3821, 1763, 2492, 3987, 2613, 3996, 4041, 2723,
    1810, 1820, 1829, 50, 170, 2501, 2622, 182, 2502, 4206, 2623, 2732, 2733, 194, 2503, 4207,
    2624, 4216, 4261, 2734, 2831, 2832, 2833, 206, 215, 1874, 216, 1875, 1885, 217, 1876, 1886,
    1895, 218, 59, 60, 61, 62, 15, 1613, 2471, 1679, 27, 1623, 3328, 3548, 1689, 2482, 3768, 2603,
    1745, 1755, 39, 1632, 3337, 3557, 1698, 3382, 6186, 3602, 3777, 3822, 1764, 2493, 3988, 2614,
    3997, 4042, 2724, 1811, 1821, 1830, 51, 1640, 3345, 3565, 1706, 3390, 6194, 3610, 3785, 3830,
    1772, 3426, 6230, 3646, 1602, 1601, 1602, 4005, 4050, 1602, 1838, 2504, 4208, 2625, 4217, 4262,
    2735, 4225, 4270, 1602, 2834, 1877, 1887, 1896, 1904, 63, 171, 2512, 2633, 183, 2513, 4426,
    2634, 2743, 2744, 195, 2514, 4427, 2635, 4436, 4481, 2745, 2842, 2843, 2844, 207, 2515, 4428,
    2636, 4437, 4482, 2746, 4445, 4490, 1602, 2845, 2930, 2931, 2932, 2933, 219, 227, 1940, 228,
    1941, 1951, 229, 1942, 1952, 1961, 230, 1943, 1953, 1962, 1970, 231, 71, 72, 73, 74, 75, 16,
    172, 2472, 184, 28, 172, 2523, 2644, 184, 2483, 2754, 2604, 196, 196, 40, 172, 2523, 2644, 184,
    2524, 4646, 2645, 2754, 2755, 196, 2494, 2853, 2615, 2853, 2854, 2725, 208, 208, 208, 52, 172,
    2523, 2644, 184, 2524, 4646, 2645, 2754, 2755, 196, 2525, 4647, 2646, 4656, 4701, 2756, 2853,
    2854, 2855, 208, 2505, 2941, 2626, 2941, 2942, 2736, 2941, 2942, 2943, 2835, 220, 220, 220,
    220, 64, 172, 2523, 2644, 184, 2524, 4646, 2645, 2754, 2755, 196, 2525, 4647, 2646, 4656, 4701,
    2756, 2853, 2854, 2855, 208, 2526, 4648, 2647, 4657, 4702, 2757, 4665, 4710, 1602, 2856, 2941,
    2942, 2943, 2944, 220, 2516, 3018, 2637, 3018, 3019, 2747, 3018, 3019, 3020, 2846, 3018, 3019,
    3020, 3021, 2934, 232, 232, 232, 232, 232, 76, 172, 239, 240, 184, 239, 2006, 240, 241, 241,
    196, 239, 2006, 240, 2007, 2017, 241, 242, 242, 242, 208, 239, 2006, 240, 2007, 2017, 241,
    2008, 2018, 2027, 242, 243, 243, 243, 243, 220, 239, 2006, 240, 2007, 2017, 241, 2008, 2018,
    2027, 242, 2009, 2019, 2028, 2036, 243, 244, 244, 244, 244, 244, 232, 83, 83, 84, 83, 84, 85,
    83, 84, 85, 86, 83, 84, 85, 86, 87, 83, 84, 85, 86, 87, 88, 11, 167, 179, 23, 12, 1610, 2468,
    1676, 24, 168, 2479, 2600, 180, 191, 1742, 192, 35, 36, 13, 1611, 2469, 1677, 25, 1621, 3326,
    3546, 1687, 2480, 3766, 2601, 1743, 1753, 37, 169, 2490, 2611, 181, 2491, 3986, 2612, 2721,
    2722, 193, 203, 1808, 204, 1809, 1819, 205, 47, 48, 49, 14, 1612, 2470, 1678, 26, 1622, 3327,
    3547, 1688, 2481, 3767, 2602, 1744, 1754, 38, 1631, 3336, 3556, 1697, 3381, 1600, 3601, 3776,
    3821, 1763, 2492, 3987, 2613, 3996, 4041, 2723, 1810, 1820, 1829, 50, 170, 2501, 2622, 182,
    2502, 4206, 2623, 2732, 2733, 194, 2503, 4207, 2624, 4216, 4261, 2734, 2831, 2832, 2833, 206,
    215, 1874, 216, 1875, 1885, 217, 1876, 1886, 1895, 218, 59, 60, 61, 62, 15, 1613, 2471, 1679,
    27, 1623, 3328, 3548, 1689, 2482, 3768, 2603, 1745, 1755, 39, 1632, 3337, 3557, 1698, 3382,
    6186, 3602, 3777, 3822, 1764, 2493, 3988, 2614, 3997, 4042, 2724, 1811, 1821, 1830, 51, 1640,
    3345, 3565, 1706, 3390, 6194, 3610, 3785, 3830, 1772, 3426, 6230, 3646, 6350, 1601, 3866, 4005,
    4050, 4086, 1838, 2504, 4208, 2625, 4217, 4262, 2735, 4225, 4270, 4306, 2834, 1877, 1887, 1896,
    1904, 63, 171, 2512, 2633, 183, 2513, 4426, 2634, 2743, 2744, 195, 2514, 4427, 2635, 4436,
    4481, 2745, 2842, 2843, 2844, 207, 2515, 4428, 2636, 4437, 4482, 2746, 4445, 4490, 4526, 2845,
    2930, 2931, 2932, 2933, 219, 227, 1940, 228, 1941, 1951, 229, 1942, 1952, 1961, 230, 1943,
    1953, 1962, 1970, 231, 71, 72, 73, 74, 75, 16, 1614, 2472, 1680, 28, 1624, 3329, 3549, 1690,
    2483, 3769, 2604, 1746, 1756, 40, 1633, 3338, 3558, 1699, 3383, 6187, 3603, 3778, 3823, 1765,
    2494, 3989, 2615, 3998, 4043, 2725, 1812, 1822, 1831, 52, 1641, 3346, 3566, 1707, 3391, 6195,
    3611, 3786, 3831, 1773, 3427, 6231, 3647, 6351, 6679, 3867, 4006, 4051, 4087, 1839, 2505, 4209,
    2626, 4218, 4263, 2736, 4226, 4271, 4307, 2835, 1878, 1888, 1897, 1905, 64, 1648, 3353, 3573,
    1714, 3398, 6202, 3618, 3793, 3838, 1780, 3434, 6238, 3654, 6358, 6686, 3874, 4013, 4058, 4094,
    1846, 3462, 6266, 3682, 6386, 6714, 3902, 1603, 1603, 1602, 1603, 4233, 4278, 4314, 1603, 1912,
    2516, 4429, 2637, 4438, 4483, 2747, 4446, 4491, 4527, 2846, 4453, 4498, 4534, 1603, 2934, 1944,
    1954, 1963, 1971, 1978, 76, 172, 2523, 2644, 184, 2524, 4646, 2645, 2754, 2755, 196, 2525,
    4647, 2646, 4656, 4701, 2756, 2853, 2854, 2855, 208, 2526, 4648, 2647, 4657, 4702, 2757, 4665,
    4710, 4746, 2856, 2941, 2942, 2943, 2944, 220, 2527, 4649, 2648, 4658, 4703, 2758, 4666, 4711,
    4747, 2857, 4673, 4718, 4754, 1603, 2945, 3018, 3019, 3020, 3021, 3022, 232, 239, 2006, 240,
    2007, 2017, 241, 2008, 2018, 2027, 242, 2009, 2019, 2028, 2036, 243, 2010, 2020, 2029, 2037,
    2044, 244, 83, 84, 85, 86, 87, 88, 17, 173, 2473, 185, 29, 173, 2534, 2655, 185, 2484, 2765,
    2605, 197, 197, 41, 173, 2534, 2655, 185, 2535, 4866, 2656, 2765, 2766, 197, 2495, 2864, 2616,
    2864, 2865, 2726, 209, 209, 209, 53, 173, 2534, 2655, 185, 2535, 4866, 2656, 2765, 2766, 197,
    2536, 4867, 2657, 4876, 4921, 2767, 2864, 2865, 2866, 209, 2506, 2952, 2627, 2952, 2953, 2737,
    2952, 2953, 2954, 2836, 221, 221, 221, 221, 65, 173, 2534, 2655, 185, 2535, 4866, 2656, 2765,
    2766, 197, 2536, 4867, 2657, 4876, 4921, 2767, 2864, 2865, 2866, 209, 2537, 4868, 2658, 4877,
    4922, 2768, 4885, 4930, 4966, 2867, 2952, 2953, 2954, 2955, 221, 2517, 3029, 2638, 3029, 3030,
    2748, 3029, 3030, 3031, 2847, 3029, 3030, 3031, 3032, 2935, 233, 233, 233, 233, 233, 77, 173,
    2534, 2655, 185, 2535, 4866, 2656, 2765, 2766, 197, 2536, 4867, 2657, 4876, 4921, 2767, 2864,
    2865, 2866, 209, 2537, 4868, 2658, 4877, 4922, 2768, 4885, 4930, 4966, 2867, 2952, 2953, 2954,
    2955, 221, 2538, 4869, 2659, 4878, 4923, 2769, 4886, 4931, 4967, 2868, 4893, 4938, 4974, 1603,
    2956, 3029, 3030, 3031, 3032, 3033, 233, 2528, 3095, 2649, 3095, 3096, 2759, 3095, 3096, 3097,
    2858, 3095, 3096, 3097, 3098, 2946, 3095, 3096, 3097, 3098, 3099, 3023, 245, 245, 245, 245,
    245, 245, 89, 173, 251, 252, 185, 251, 2072, 252, 253, 253, 197, 251, 2072, 252, 2073, 2083,
    253, 254, 254, 254, 209, 251, 2072, 252, 2073, 2083, 253, 2074, 2084, 2093, 254, 255, 255, 255,
    255, 221, 251, 2072, 252, 2073, 2083, 253, 2074, 2084, 2093, 254, 2075, 2085, 2094, 2102, 255,
    256, 256, 256, 256, 256, 233, 251, 2072, 252, 2073, 2083, 253, 2074, 2084, 2093, 254, 2075,
    2085, 2094, 2102, 255, 2076, 2086, 2095, 2103, 2110, 256, 257, 257, 257, 257, 257, 257, 245,
    95, 95, 96, 95, 96, 97, 95, 96, 97, 98, 95, 96, 97, 98, 99, 95, 96, 97, 98, 99, 100, 95, 96,
    97, 98, 99, 100, 101, 11, 167, 179, 23, 12, 1610, 2468, 1676, 24, 168, 2479, 2600, 180, 191,
    1742, 192, 35, 36, 13, 1611, 2469, 1677, 25, 1621, 3326, 3546, 1687, 2480, 3766, 2601, 1743,
    1753, 37, 169, 2490, 2611, 181, 2491, 3986, 2612, 2721, 2722, 193, 203, 1808, 204, 1809, 1819,
    205, 47, 48, 49, 14, 1612, 2470, 1678, 26, 1622, 3327, 3547, 1688, 2481, 3767, 2602, 1744,
    1754, 38, 1631, 3336, 3556, 1697, 3381, 1600, 3601, 3776, 3821, 1763, 2492, 3987, 2613, 3996,
    4041, 2723, 1810, 1820, 1829, 50, 170, 2501, 2622, 182, 2502, 4206, 2623, 2732, 2733, 194,
    2503, 4207, 2624, 4216, 4261, 2734, 2831, 2832, 2833, 206, 215, 1874, 216, 1875, 1885, 217,
    1876, 1886, 1895, 218, 59, 60, 61, 62, 15, 1613, 2471, 1679, 27, 1623, 3328, 3548, 1689, 2482,
    3768, 2603, 1745, 1755, 39, 1632, 3337, 3557, 1698, 3382, 6186, 3602, 3777, 3822, 1764, 2493,
    3988, 2614, 3997, 4042, 2724, 1811, 1821, 1830, 51, 1640, 3345, 3565, 1706, 3390, 6194, 3610,
    3785, 3830, 1772, 3426, 6230, 3646, 6350, 1601, 3866, 4005, 4050, 4086, 1838, 2504, 4208, 2625,
    4217, 4262, 2735, 4225, 4270, 4306, 2834, 1877, 1887, 1896, 1904, 63, 171, 2512, 2633, 183,
    2513, 4426, 2634, 2743, 2744, 195, 2514, 4427, 2635, 4436, 4481, 2745, 2842, 2843, 2844, 207,
    2515, 4428, 2636, 4437, 4482, 2746, 4445, 4490, 4526, 2845, 2930, 2931, 2932, 2933, 219, 227,
    1940, 228, 1941, 1951, 229, 1942, 1952, 1961, 230, 1943, 1953, 1962, 1970, 231, 71, 72, 73, 74,
    75, 16, 1614, 2472, 1680, 28, 1624, 3329, 3549, 1690, 2483, 3769, 2604, 1746, 1756, 40, 1633,
    3338, 3558, 1699, 3383, 6187, 3603, 3778, 3823, 1765, 2494, 3989, 2615, 3998, 4043, 2725, 1812,
    1822, 1831, 52, 1641, 3346, 3566, 1707, 3391, 6195, 3611, 3786, 3831, 1773, 3427, 6231, 3647,
    6351, 6679, 3867, 4006, 4051, 4087, 1839, 2505, 4209, 2626, 4218, 4263, 2736, 4226, 4271, 4307,
    2835, 1878, 1888, 1897, 1905, 64, 1648, 3353, 3573, 1714, 3398, 6202, 3618, 3793, 3838, 1780,
    3434, 6238, 3654, 6358, 6686, 3874, 4013, 4058, 4094, 1846, 3462, 6266, 3682, 6386, 6714, 3902,
    6470, 6798, 1602, 4122, 4233, 4278, 4314, 4342, 1912, 2516, 4429, 2637, 4438, 4483, 2747, 4446,
    4491, 4527, 2846, 4453, 4498, 4534, 4562, 2934, 1944, 1954, 1963, 1971, 1978, 76, 172, 2523,
    2644, 184, 2524, 4646, 2645, 2754, 2755, 196, 2525, 4647, 2646, 4656, 4701, 2756, 2853, 2854,
    2855, 208, 2526, 4648, 2647, 4657, 4702, 2757, 4665, 4710, 4746, 2856, 2941, 2942, 2943, 2944,
    220, 2527, 4649, 2648, 4658, 4703, 2758, 4666, 4711, 4747, 2857, 4673, 4718, 4754, 4782, 2945,
    3018, 3019, 3020, 3021, 3022, 232, 239, 2006, 240, 2007, 2017, 241, 2008, 2018, 2027, 242,
    2009, 2019, 2028, 2036, 243, 2010, 2020, 2029, 2037, 2044, 244, 83, 84, 85, 86, 87, 88, 17,
    1615, 2473, 1681, 29, 1625, 3330, 3550, 1691, 2484, 3770, 2605, 1747, 1757, 41, 1634, 3339,
    3559, 1700, 3384, 6188, 3604, 3779, 3824, 1766, 2495, 3990, 2616, 3999, 4044, 2726, 1813, 1823,
    1832, 53, 1642, 3347, 3567, 1708, 3392, 6196, 3612, 3787, 3832, 1774, 3428, 6232, 3648, 6352,
    6680, 3868, 4007, 4052, 4088, 1840, 2506, 4210, 2627, 4219, 4264, 2737, 4227, 4272, 4308, 2836,
    1879, 1889, 1898, 1906, 65, 1649, 3354, 3574, 1715, 3399, 6203, 3619, 3794, 3839, 1781, 3435,
    6239, 3655, 6359, 6687, 3875, 4014, 4059, 4095, 1847, 3463, 6267, 3683, 6387, 6715, 3903, 6471,
    6799, 7008, 4123, 4234, 4279, 4315, 4343, 1913, 2517, 4430, 2638, 4439, 4484, 2748, 4447, 4492,
    4528, 2847, 4454, 4499, 4535, 4563, 2935, 1945, 1955, 1964, 1972, 1979, 77, 1655, 3360, 3580,
    1721, 3405, 6209, 3625, 3800, 3845, 1787, 3441, 6245, 3661, 6365, 6693, 3881, 4020, 4065, 4101,
    1853, 3469, 6273, 3689, 6393, 6721, 3909, 6477, 6805, 7014, 4129, 4240, 4285, 4321, 4349, 1919,
    3490, 6294, 3710, 6414, 6742, 3930, 6498, 6826, 7035, 4150, 1604, 1604, 1604, 1603, 1604, 4460,
    4505, 4541, 4569, 1604, 1985, 2528, 4650, 2649, 4659, 4704, 2759, 4667, 4712, 4748, 2858, 4674,
    4719, 4755, 4783, 2946, 4680, 4725, 4761, 4789, 1604, 3023, 2011, 2021, 2030, 2038, 2045, 2051,
    89, 173, 2534, 2655, 185, 2535, 4866, 2656, 2765, 2766, 197, 2536, 4867, 2657, 4876, 4921,
    2767, 2864, 2865, 2866, 209, 2537, 4868, 2658, 4877, 4922, 2768, 4885, 4930, 4966, 2867, 2952,
    2953, 2954, 2955, 221, 2538, 4869, 2659, 4878, 4923, 2769, 4886, 4931, 4967, 2868, 4893, 4938,
    4974, 5002, 2956, 3029, 3030, 3031, 3032, 3033, 233, 2539, 4870, 2660, 4879, 4924, 2770, 4887,
    4932, 4968, 2869, 4894, 4939, 4975, 5003, 2957, 4900, 4945, 4981, 5009, 1604, 3034, 3095, 3096,
    3097, 3098, 3099, 3100, 245, 251, 2072, 252, 2073, 2083, 253, 2074, 2084, 2093, 254, 2075,
    2085, 2094, 2102, 255, 2076, 2086, 2095, 2103, 2110, 256, 2077, 2087, 2096, 2104, 2111, 2117,
    257, 95, 96, 97, 98, 99, 100, 101, 18, 174, 2474, 186, 30, 174, 2545, 2666, 186, 2485, 2776,
    2606, 198, 198, 42, 174, 2545, 2666, 186, 2546, 5086, 2667, 2776, 2777, 198, 2496, 2875, 2617,
    2875, 2876, 2727, 210, 210, 210, 54, 174, 2545, 2666, 186, 2546, 5086, 2667, 2776, 2777, 198,
    2547, 5087, 2668, 5096, 5141, 2778, 2875, 2876, 2877, 210, 2507, 2963, 2628, 2963, 2964, 2738,
    2963, 2964, 2965, 2837, 222, 222, 222, 222, 66, 174, 2545, 2666, 186, 2546, 5086, 2667, 2776,
    2777, 198, 2547, 5087, 2668, 5096, 5141, 2778, 2875, 2876, 2877, 210, 2548, 5088, 2669, 5097,
    5142, 2779, 5105, 5150, 5186, 2878, 2963, 2964, 2965, 2966, 222, 2518, 3040, 2639, 3040, 3041,
    2749, 3040, 3041, 3042, 2848, 3040, 3041, 3042, 3043, 2936, 234, 234, 234, 234, 234, 78, 174,
    2545, 2666, 186, 2546, 5086, 2667, 2776, 2777, 198, 2547, 5087, 2668, 5096, 5141, 2778, 2875,
    2876, 2877, 210, 2548, 5088, 2669, 5097, 5142, 2779, 5105, 5150, 5186, 2878, 2963, 2964, 2965,
    2966, 222, 2549, 5089, 2670, 5098, 5143, 2780, 5106, 5151, 5187, 2879, 5113, 5158, 5194, 5222,
    2967, 3040, 3041, 3042, 3043, 3044, 234, 2529, 3106, 2650, 3106, 3107, 2760, 3106, 3107, 3108,
    2859, 3106, 3107, 3108, 3109, 2947, 3106, 3107, 3108, 3109, 3110, 3024, 246, 246, 246, 246,
    246, 246, 90, 174, 2545, 2666, 186, 2546, 5086, 2667, 2776, 2777, 198, 2547, 5087, 2668, 5096,
    5141, 2778, 2875, 2876, 2877, 210, 2548, 5088, 2669, 5097, 5142, 2779, 5105, 5150, 5186, 2878,
    2963, 2964, 2965, 2966, 222, 2549, 5089, 2670, 5098, 5143, 2780, 5106, 5151, 5187, 2879, 5113,
    5158, 5194, 5222, 2967, 3040, 3041, 3042, 3043, 3044, 234, 2550, 5090, 2671, 5099, 5144, 2781,
    5107, 5152, 5188, 2880, 5114, 5159, 5195, 5223, 2968, 5120, 5165, 5201, 5229, 1604, 3045, 3106,
    3107, 3108, 3109, 3110, 3111, 246, 2540, 3161, 2661, 3161, 3162, 2771, 3161, 3162, 3163, 2870,
    3161, 3162, 3163, 3164, 2958, 3161, 3162, 3163, 3164, 3165, 3035, 3161, 3162, 3163, 3164, 3165,
    3166, 3101, 258, 258, 258, 258, 258, 258, 258, 102, 174, 263, 264, 186, 263, 2138, 264, 265,
    265, 198, 263, 2138, 264, 2139, 2149, 265, 266, 266, 266, 210, 263, 2138, 264, 2139, 2149, 265,
    2140, 2150, 2159, 266, 267, 267, 267, 267, 222, 263, 2138, 264, 2139, 2149, 265, 2140, 2150,
    2159, 266, 2141, 2151, 2160, 2168, 267, 268, 268, 268, 268, 268, 234, 263, 2138, 264, 2139,
    2149, 265, 2140, 2150, 2159, 266, 2141, 2151, 2160, 2168, 267, 2142, 2152, 2161, 2169, 2176,
    268, 269, 269, 269, 269, 269, 269, 246, 263, 2138, 264, 2139, 2149, 265, 2140, 2150, 2159, 266,
    2141, 2151, 2160, 2168, 267, 2142, 2152, 2161, 2169, 2176, 268, 2143, 2153, 2162, 2170, 2177,
    2183, 269, 270, 270, 270, 270, 270, 270, 270, 258, 107, 107, 108, 107, 108, 109, 107, 108, 109,
    110, 107, 108, 109, 110, 111, 107, 108, 109, 110, 111, 112, 107, 108, 109, 110, 111, 112, 113,
    107, 108, 109, 110, 111, 112, 113, 114, 11, 167, 179, 23, 12, 1610, 2468, 1676, 24, 168, 2479,
    2600, 180, 191, 1742, 192, 35, 36, 13, 1611, 2469, 1677, 25, 1621, 3326, 3546, 1687, 2480,
    3766, 2601, 1743, 1753, 37, 169, 2490, 2611, 181, 2491, 3986, 2612, 2721, 2722, 193, 203, 1808,
    204, 1809, 1819, 205, 47, 48, 49, 14, 1612, 2470, 1678, 26, 1622, 3327, 3547, 1688, 2481, 3767,
    2602, 1744, 1754, 38, 1631, 3336, 3556, 1697, 3381, 1600, 3601, 3776, 3821, 1763, 2492, 3987,
    2613, 3996, 4041, 2723, 1810, 1820, 1829, 50, 170, 2501, 2622, 182, 2502, 4206, 2623, 2732,
    2733, 194, 2503, 4207, 2624, 4216, 4261, 2734, 2831, 2832, 2833, 206, 215, 1874, 216, 1875,
    1885, 217, 1876, 1886, 1895, 218, 59, 60, 61, 62, 15, 1613, 2471, 1679, 27, 1623, 3328, 3548,
    1689, 2482, 3768, 2603, 1745, 1755, 39, 1632, 3337, 3557, 1698, 3382, 6186, 3602, 3777, 3822,
    1764, 2493, 3988, 2614, 3997, 4042, 2724, 1811, 1821, 1830, 51, 1640, 3345, 3565, 1706, 3390,
    6194, 3610, 3785, 3830, 1772, 3426, 6230, 3646, 6350, 1601, 3866, 4005, 4050, 4086, 1838, 2504,
    4208, 2625, 4217, 4262, 2735, 4225, 4270, 4306, 2834, 1877, 1887, 1896, 1904, 63, 171, 2512,
    2633, 183, 2513, 4426, 2634, 2743, 2744, 195, 2514, 4427, 2635, 4436, 4481, 2745, 2842, 2843,
    2844, 207, 2515, 4428, 2636, 4437, 4482, 2746, 4445, 4490, 4526, 2845, 2930, 2931, 2932, 2933,
    219, 227, 1940, 228, 1941, 1951, 229, 1942, 1952, 1961, 230, 1943, 1953, 1962, 1970, 231, 71,
    72, 73, 74, 75, 16, 1614, 2472, 1680, 28, 1624, 3329, 3549, 1690, 2483, 3769, 2604, 1746, 1756,
    40, 1633, 3338, 3558, 1699, 3383, 6187, 3603, 3778, 3823, 1765, 2494, 3989, 2615, 3998, 4043,
    2725, 1812, 1822, 1831, 52, 1641, 3346, 3566, 1707, 3391, 6195, 3611, 3786, 3831, 1773, 3427,
    6231, 3647, 6351, 6679, 3867, 4006, 4051, 4087, 1839, 2505, 4209, 2626, 4218, 4263, 2736, 4226,
    4271, 4307, 2835, 1878, 1888, 1897, 1905, 64, 1648, 3353, 3573, 1714, 3398, 6202, 3618, 3793,
    3838, 1780, 3434, 6238, 3654, 6358, 6686, 3874, 4013, 4058, 4094, 1846, 3462, 6266, 3682, 6386,
    6714, 3902, 6470, 6798, 1602, 4122, 4233, 4278, 4314, 4342, 1912, 2516, 4429, 2637, 4438, 4483,
    2747, 4446, 4491, 4527, 2846, 4453, 4498, 4534, 4562, 2934, 1944, 1954, 1963, 1971, 1978, 76,
    172, 2523, 2644, 184, 2524, 4646, 2645, 2754, 2755, 196, 2525, 4647, 2646, 4656, 4701, 2756,
    2853, 2854, 2855, 208, 2526, 4648, 2647, 4657, 4702, 2757, 4665, 4710, 4746, 2856, 2941, 2942,
    2943, 2944, 220, 2527, 4649, 2648, 4658, 4703, 2758, 4666, 4711, 4747, 2857, 4673, 4718, 4754,
    4782, 2945, 3018, 3019, 3020, 3021, 3022, 232, 239, 2006, 240, 2007, 2017, 241, 2008, 2018,
    2027, 242, 2009, 2019, 2028, 2036, 243, 2010, 2020, 2029, 2037, 2044, 244, 83, 84, 85, 86, 87,
    88, 17, 1615, 2473, 1681, 29, 1625, 3330, 3550, 1691, 2484, 3770, 2605, 1747, 1757, 41, 1634,
    3339, 3559, 1700, 3384, 6188, 3604, 3779, 3824, 1766, 2495, 3990, 2616, 3999, 4044, 2726, 1813,
    1823, 1832, 53, 1642, 3347, 3567, 1708, 3392, 6196, 3612, 3787, 3832, 1774, 3428, 6232, 3648,
    6352, 6680, 3868, 4007, 4052, 4088, 1840, 2506, 4210, 2627, 4219, 4264, 2737, 4227, 4272, 4308,
    2836, 1879, 1889, 1898, 1906, 65, 1649, 3354, 3574, 1715, 3399, 6203, 3619, 3794, 3839, 1781,
    3435, 6239, 3655, 6359, 6687, 3875, 4014, 4059, 4095, 1847, 3463, 6267, 3683, 6387, 6715, 3903,
    6471, 6799, 7008, 4123, 4234, 4279, 4315, 4343, 1913, 2517, 4430, 2638, 4439, 4484, 2748, 4447,
    4492, 4528, 2847, 4454, 4499, 4535, 4563, 2935, 1945, 1955, 1964, 1972, 1979, 77, 1655, 3360,
    3580, 1721, 3405, 6209, 3625, 3800, 3845, 1787, 3441, 6245, 3661, 6365, 6693, 3881, 4020, 4065,
    4101, 1853, 3469, 6273, 3689, 6393, 6721, 3909, 6477, 6805, 7014, 4129, 4240, 4285, 4321, 4349,
    1919, 3490, 6294, 3710, 6414, 6742, 3930, 6498, 6826, 7035, 4150, 6554, 6882, 7091, 1603, 4370,
    4460, 4505, 4541, 4569, 4590, 1985, 2528, 4650, 2649, 4659, 4704, 2759, 4667, 4712, 4748, 2858,
    4674, 4719, 4755, 4783, 2946, 4680, 4725, 4761, 4789, 4810, 3023, 2011, 2021, 2030, 2038, 2045,
    2051, 89, 173, 2534, 2655, 185, 2535, 4866, 2656, 2765, 2766, 197, 2536, 4867, 2657, 4876,
    4921, 2767, 2864, 2865, 2866, 209, 2537, 4868, 2658, 4877, 4922, 2768, 4885, 4930, 4966, 2867,
    2952, 2953, 2954, 2955, 221, 2538, 4869, 2659, 4878, 4923, 2769, 4886, 4931, 4967, 2868, 4893,
    4938, 4974, 5002, 2956, 3029, 3030, 3031, 3032, 3033, 233, 2539, 4870, 2660, 4879, 4924, 2770,
    4887, 4932, 4968, 2869, 4894, 4939, 4975, 5003, 2957, 4900, 4945, 4981, 5009, 5030, 3034, 3095,
    3096, 3097, 3098, 3099, 3100, 245, 251, 2072, 252, 2073, 2083, 253, 2074, 2084, 2093, 254,
    2075, 2085, 2094, 2102, 255, 2076, 2086, 2095, 2103, 2110, 256, 2077, 2087, 2096, 2104, 2111,
    2117, 257, 95, 96, 97, 98, 99, 100, 101, 18, 1616, 2474, 1682, 30, 1626, 3331, 3551, 1692,
    2485, 3771, 2606, 1748, 1758, 42, 1635, 3340, 3560, 1701, 3385, 6189, 3605, 3780, 3825, 1767,
    2496, 3991, 2617, 4000, 4045, 2727, 1814, 1824, 1833, 54, 1643, 3348, 3568, 1709, 3393, 6197,
    3613, 3788, 3833, 1775, 3429, 6233, 3649, 6353, 6681, 3869, 4008, 4053, 4089, 1841, 2507, 4211,
    2628, 4220, 4265, 2738, 4228, 4273, 4309, 2837, 1880, 1890, 1899, 1907, 66, 1650, 3355, 3575,
    1716, 3400, 6204, 3620, 3795, 3840, 1782, 3436, 6240, 3656, 6360, 6688, 3876, 4015, 4060, 4096,
    1848, 3464, 6268, 3684, 6388, 6716, 3904, 6472, 6800, 7009, 4124, 4235, 4280, 4316, 4344, 1914,
    2518, 4431, 2639, 4440, 4485, 2749, 4448, 4493, 4529, 2848, 4455, 4500, 4536, 4564, 2936, 1946,
    1956, 1965, 1973, 1980, 78, 1656, 3361, 3581, 1722, 3406, 6210, 3626, 3801, 3846, 1788, 3442,
    6246, 3662, 6366, 6694, 3882, 4021, 4066, 4102, 1854, 3470, 6274, 3690, 6394, 6722, 3910, 6478,
    6806, 7015, 4130, 4241, 4286, 4322, 4350, 1920, 3491, 6295, 3711, 6415, 6743, 3931, 6499, 6827,
    7036, 4151, 6555, 6883, 7092, 7217, 4371, 4461, 4506, 4542, 4570, 4591, 1986, 2529, 4651, 2650,
    4660, 4705, 2760, 4668, 4713, 4749, 2859, 4675, 4720, 4756, 4784, 2947, 4681, 4726, 4762, 4790,
    4811, 3024, 2012, 2022, 2031, 2039, 2046, 2052, 90, 1661, 3366, 3586, 1727, 3411, 6215, 3631,
    3806, 3851, 1793, 3447, 6251, 3667, 6371, 6699, 3887, 4026, 4071, 4107, 1859, 3475, 6279, 3695,
    6399, 6727, 3915, 6483, 6811, 7020, 4135, 4246, 4291, 4327, 4355, 1925, 3496, 6300, 3716, 6420,
    6748, 3936, 6504, 6832, 7041, 4156, 6560, 6888, 7097, 7222, 4376, 4466, 4511, 4547, 4575, 4596,
    1991, 1606, 1606, 1606, 1606, 1606, 1606, 1606, 1606, 1606, 1606, 1606, 1606, 1606, 1606, 1606,
    1605, 1605, 1605, 1605, 1604, 1605, 1606, 1606, 1606, 1606, 1606, 1605, 1606, 2540, 4871, 2661,
    4880, 4925, 2771, 4888, 4933, 4969, 2870, 4895, 4940, 4976, 5004, 2958, 4901, 4946, 4982, 5010,
    5031, 3035, 1606, 1606, 1606, 1606, 1606, 1605, 1606, 2078, 2088, 2097, 2105, 2112, 2118, 1606,
    102, 174, 2545, 2666, 186, 2546, 5086, 2667, 2776, 2777, 198, 2547, 5087, 2668, 5096, 5141,
    2778, 2875, 2876, 2877, 210, 2548, 5088, 2669, 5097, 5142, 2779, 5105, 5150, 5186, 2878, 2963,
    2964, 2965, 2966, 222, 2549, 5089, 2670, 5098, 5143, 2780, 5106, 5151, 5187, 2879, 5113, 5158,
    5194, 5222, 2967, 3040, 3041, 3042, 3043, 3044, 234, 2550, 5090, 2671, 5099, 5144, 2781, 5107,
    5152, 5188, 2880, 5114, 5159, 5195, 5223, 2968, 5120, 5165, 5201, 5229, 5250, 3045, 3106, 3107,
    3108, 3109, 3110, 3111, 246, 2551, 5091, 2672, 5100, 5145, 2782, 5108, 5153, 5189, 2881, 5115,
    5160, 5196, 5224, 2969, 5121, 5166, 5202, 5230, 5251, 3046, 1606, 1606, 1606, 1606, 1606, 1605,
    1606, 3161, 3162, 3163, 3164, 3165, 3166, 1606, 258, 263, 2138, 264, 2139, 2149, 265, 2140,
    2150, 2159, 266, 2141, 2151, 2160, 2168, 267, 2142, 2152, 2161, 2169, 2176, 268, 2143, 2153,
    2162, 2170, 2177, 2183, 269, 2144, 2154, 2163, 2171, 2178, 2184, 1606, 270, 107, 108, 109, 110,
    111, 112, 113, 114, 19, 175, 2475, 187, 31, 175, 2556, 2677, 187, 2486, 2787, 2607, 199, 199,
    43, 175, 2556, 2677, 187, 2557, 5306, 2678, 2787, 2788, 199, 2497, 2886, 2618, 2886, 2887,
    2728, 211, 211, 211, 55, 175, 2556, 2677, 187, 2557, 5306, 2678, 2787, 2788, 199, 2558, 5307,
    2679, 5316, 5361, 2789, 2886, 2887, 2888, 211, 2508, 2974, 2629, 2974, 2975, 2739, 2974, 2975,
    2976, 2838, 223, 223, 223, 223, 67, 175, 2556, 2677, 187, 2557, 5306, 2678, 2787, 2788, 199,
    2558, 5307, 2679, 5316, 5361, 2789, 2886, 2887, 2888, 211, 2559, 5308, 2680, 5317, 5362, 2790,
    5325, 5370, 5406, 2889, 2974, 2975, 2976, 2977, 223, 2519, 3051, 2640, 3051, 3052, 2750, 3051,
    3052, 3053, 2849, 3051, 3052, 3053, 3054, 2937, 235, 235, 235, 235, 235, 79, 175, 2556, 2677,
    187, 2557, 5306, 2678, 2787, 2788, 199, 2558, 5307, 2679, 5316, 5361, 2789, 2886, 2887, 2888,
    211, 2559, 5308, 2680, 5317, 5362, 2790, 5325, 5370, 5406, 2889, 2974, 2975, 2976, 2977, 223,
    2560, 5309, 2681, 5318, 5363, 2791, 5326, 5371, 5407, 2890, 5333, 5378, 5414, 5442, 2978, 3051,
    3052, 3053, 3054, 3055, 235, 2530, 3117, 2651, 3117, 3118, 2761, 3117, 3118, 3119, 2860, 3117,
    3118, 3119, 3120, 2948, 3117, 3118, 3119, 3120, 3121, 3025, 247, 247, 247, 247, 247, 247, 91,
    175, 2556, 2677, 187, 2557, 5306, 2678, 2787, 2788, 199, 2558, 5307, 2679, 5316, 5361, 2789,
    2886, 2887, 2888, 211, 2559, 5308, 2680, 5317, 5362, 2790, 5325, 5370, 5406, 2889, 2974, 2975,
    2976, 2977, 223, 2560, 5309, 2681, 5318, 5363, 2791, 5326, 5371, 5407, 2890, 5333, 5378, 5414,
    5442, 2978, 3051, 3052, 3053, 3054, 3055, 235, 2561, 5310, 2682, 5319, 5364, 2792, 5327, 5372,
    5408, 2891, 5334, 5379, 5415, 5443, 2979, 5340, 5385, 5421, 5449, 5470, 3056, 3117, 3118, 3119,
    3120, 3121, 3122, 247, 2541, 3172, 2662, 3172, 3173, 2772, 3172, 3173, 3174, 2871, 3172, 3173,
    3174, 3175, 2959, 3172, 3173, 3174, 3175, 3176, 3036, 3172, 3173, 3174, 3175, 3176, 3177, 3102,
    259, 259, 259, 259, 259, 259, 259, 103, 175, 2556, 2677, 187, 2557, 5306, 2678, 2787, 2788,
    199, 2558, 5307, 2679, 5316, 5361, 2789, 2886, 2887, 2888, 211, 2559, 5308, 2680, 5317, 5362,
    2790, 5325, 5370, 5406, 2889, 2974, 2975, 2976, 2977, 223, 2560, 5309, 2681, 5318, 5363, 2791,
    5326, 5371, 5407, 2890, 5333, 5378, 5414, 5442, 2978, 3051, 3052, 3053, 3054, 3055, 235, 2561,
    5310, 2682, 5319, 5364, 2792, 5327, 5372, 5408, 2891, 5334, 5379, 5415, 5443, 2979, 5340, 5385,
    5421, 5449, 5470, 3056, 3117, 3118, 3119, 3120, 3121, 3122, 247, 2562, 5311, 2683, 5320, 5365,
    2793, 5328, 5373, 5409, 2892, 5335, 5380, 5416, 5444, 2980, 5341, 5386, 5422, 5450, 5471, 3057,
    1606, 1606, 1606, 1606, 1606, 1605, 1606, 3172, 3173, 3174, 3175, 3176, 3177, 1606, 259, 2552,
    3216, 2673, 3216, 3217, 2783, 3216, 3217, 3218, 2882, 3216, 3217, 3218, 3219, 2970, 3216, 3217,
    3218, 3219, 3220, 3047, 3216, 3217, 3218, 3219, 3220, 3221, 3113, 3216, 3217, 3218, 3219, 3220,
    3221, 1606, 3168, 271, 271, 271, 271, 271, 271, 271, 271, 115, 175, 275, 276, 187, 275, 2204,
    276, 277, 277, 199, 275, 2204, 276, 2205, 2215, 277, 278, 278, 278, 211, 275, 2204, 276, 2205,
    2215, 277, 2206, 2216, 2225, 278, 279, 279, 279, 279, 223, 275, 2204, 276, 2205, 2215, 277,
    2206, 2216, 2225, 278, 2207, 2217, 2226, 2234, 279, 280, 280, 280, 280, 280, 235, 275, 2204,
    276, 2205, 2215, 277, 2206, 2216, 2225, 278, 2207, 2217, 2226, 2234, 279, 2208, 2218, 2227,
    2235, 2242, 280, 281, 281, 281, 281, 281, 281, 247, 275, 2204, 276, 2205, 2215, 277, 2206,
    2216, 2225, 278, 2207, 2217, 2226, 2234, 279, 2208, 2218, 2227, 2235, 2242, 280, 2209, 2219,
    2228, 2236, 2243, 2249, 281, 282, 282, 282, 282, 282, 282, 282, 259, 275, 2204, 276, 2205,
    2215, 277, 2206, 2216, 2225, 278, 2207, 2217, 2226, 2234, 279, 2208, 2218, 2227, 2235, 2242,
    280, 2209, 2219, 2228, 2236, 2243, 2249, 281, 2210, 2220, 2229, 2237, 2244, 2250, 1606, 282,
    283, 283, 283, 283, 283, 283, 283, 283, 271, 119, 119, 120, 119, 120, 121, 119, 120, 121, 122,
    119, 120, 121, 122, 123, 119, 120, 121, 122, 123, 124, 119, 120, 121, 122, 123, 124, 125, 119,
    120, 121, 122, 123, 124, 125, 126, 119, 120, 121, 122, 123, 124, 125, 126, 127, 11, 167, 179,
    23, 12, 176, 2468, 188, 24, 168, 2479, 2600, 180, 191, 200, 192, 35, 36, 13, 176, 2469, 188,
    25, 176, 2567, 2688, 188, 2480, 2798, 2601, 200, 200, 37, 169, 2490, 2611, 181, 2491, 2897,
    2612, 2721, 2722, 193, 203, 212, 204, 212, 212, 205, 47, 48, 49, 14, 176, 2470, 188, 26, 176,
    2567, 2688, 188, 2481, 2798, 2602, 200, 200, 38, 176, 2567, 2688, 188, 2568, 1600, 2689, 2798,
    2799, 200, 2492, 2897, 2613, 2897, 2898, 2723, 212, 212, 212, 50, 170, 2501, 2622, 182, 2502,
    2985, 2623, 2732, 2733, 194, 2503, 2985, 2624, 2985, 2986, 2734, 2831, 2832, 2833, 206, 215,
    224, 216, 224, 224, 217, 224, 224, 224, 218, 59, 60, 61, 62, 15, 176, 2471, 188, 27, 176, 2567,
    2688, 188, 2482, 2798, 2603, 200, 200, 39, 176, 2567, 2688, 188, 2568, 5526, 2689, 2798, 2799,
    200, 2493, 2897, 2614, 2897, 2898, 2724, 212, 212, 212, 51, 176, 2567, 2688, 188, 2568, 5526,
    2689, 2798, 2799, 200, 2569, 5527, 2690, 5536, 1601, 2800, 2897, 2898, 2899, 212, 2504, 2985,
    2625, 2985, 2986, 2735, 2985, 2986, 2987, 2834, 224, 224, 224, 224, 63, 171, 2512, 2633, 183,
    2513, 3062, 2634, 2743, 2744, 195, 2514, 3062, 2635, 3062, 3063, 2745, 2842, 2843, 2844, 207,
    2515, 3062, 2636, 3062, 3063, 2746, 3062, 3063, 3064, 2845, 2930, 2931, 2932, 2933, 219, 227,
    236, 228, 236, 236, 229, 236, 236, 236, 230, 236, 236, 236, 236, 231, 71, 72, 73, 74, 75, 16,
    176, 2472, 188, 28, 176, 2567, 2688, 188, 2483, 2798, 2604, 200, 200, 40, 176, 2567, 2688, 188,
    2568, 5526, 2689, 2798, 2799, 200, 2494, 2897, 2615, 2897, 2898, 2725, 212, 212, 212, 52, 176,
    2567, 2688, 188, 2568, 5526, 2689, 2798, 2799, 200, 2569, 5527, 2690, 5536, 5581, 2800, 2897,
    2898, 2899, 212, 2505, 2985, 2626, 2985, 2986, 2736, 2985, 2986, 2987, 2835, 224, 224, 224,
    224, 64, 176, 2567, 2688, 188, 2568, 5526, 2689, 2798, 2799, 200, 2569, 5527, 2690, 5536, 5581,
    2800, 2897, 2898, 2899, 212, 2570, 5528, 2691, 5537, 5582, 2801, 5545, 5590, 1602, 2900, 2985,
    2986, 2987, 2988, 224, 2516, 3062, 2637, 3062, 3063, 2747, 3062, 3063, 3064, 2846, 3062, 3063,
    3064, 3065, 2934, 236, 236, 236, 236, 236, 76, 172, 2523, 2644, 184, 2524, 3128, 2645, 2754,
    2755, 196, 2525, 3128, 2646, 3128, 3129, 2756, 2853, 2854, 2855, 208, 2526, 3128, 2647, 3128,
    3129, 2757, 3128, 3129, 3130, 2856, 2941, 2942, 2943, 2944, 220, 2527, 3128, 2648, 3128, 3129,
    2758, 3128, 3129, 3130, 2857, 3128, 3129, 3130, 3131, 2945, 3018, 3019, 3020, 3021, 3022, 232,
    239, 248, 240, 248, 248, 241, 248, 248, 248, 242, 248, 248, 248, 248, 243, 248, 248, 248, 248,
    248, 244, 83, 84, 85, 86, 87, 88, 17, 176, 2473, 188, 29, 176, 2567, 2688, 188, 2484, 2798,
    2605, 200, 200, 41, 176, 2567, 2688, 188, 2568, 5526, 2689, 2798, 2799, 200, 2495, 2897, 2616,
    2897, 2898, 2726, 212, 212, 212, 53, 176, 2567, 2688, 188, 2568, 5526, 2689, 2798, 2799, 200,
    2569, 5527, 2690, 5536, 5581, 2800, 2897, 2898, 2899, 212, 2506, 2985, 2627, 2985, 2986, 2737,
    2985, 2986, 2987, 2836, 224, 224, 224, 224, 65, 176, 2567, 2688, 188, 2568, 5526, 2689, 2798,
    2799, 200, 2569, 5527, 2690, 5536, 5581, 2800, 2897, 2898, 2899, 212, 2570, 5528, 2691, 5537,
    5582, 2801, 5545, 5590, 5626, 2900, 2985, 2986, 2987, 2988, 224, 2517, 3062, 2638, 3062, 3063,
    2748, 3062, 3063, 3064, 2847, 3062, 3063, 3064, 3065, 2935, 236, 236, 236, 236, 236, 77, 176,
    2567, 2688, 188, 2568, 5526, 2689, 2798, 2799, 200, 2569, 5527, 2690, 5536, 5581, 2800, 2897,
    2898, 2899, 212, 2570, 5528, 2691, 5537, 5582, 2801, 5545, 5590, 5626, 2900, 2985, 2986, 2987,
    2988, 224, 2571, 5529, 2692, 5538, 5583, 2802, 5546, 5591, 5627, 2901, 5553, 5598, 5634, 1603,
    2989, 3062, 3063, 3064, 3065, 3066, 236, 2528, 3128, 2649, 3128, 3129, 2759, 3128, 3129, 3130,
    2858, 3128, 3129, 3130, 3131, 2946, 3128, 3129, 3130, 3131, 3132, 3023, 248, 248, 248, 248,
    248, 248, 89, 173, 2534, 2655, 185, 2535, 3183, 2656, 2765, 2766, 197, 2536, 3183, 2657, 3183,
    3184, 2767, 2864, 2865, 2866, 209, 2537, 3183, 2658, 3183, 3184, 2768, 3183, 3184, 3185, 2867,
    2952, 2953, 2954, 2955, 221, 2538, 3183, 2659, 3183, 3184, 2769, 3183, 3184, 3185, 2868, 3183,
    3184, 3185, 3186, 2956, 3029, 3030, 3031, 3032, 3033, 233, 2539, 3183, 2660, 3183, 3184, 2770,
    3183, 3184, 3185, 2869, 3183, 3184, 3185, 3186, 2957, 3183, 3184, 3185, 3186, 3187, 3034, 3095,
    3096, 3097, 3098, 3099, 3100, 245, 251, 260, 252, 260, 260, 253, 260, 260, 260, 254, 260, 260,
    260, 260, 255, 260, 260, 260, 260, 260, 256, 260, 260, 260, 260, 260, 260, 257, 95, 96, 97, 98,
    99, 100, 101, 18, 176, 2474, 188, 30, 176, 2567, 2688, 188, 2485, 2798, 2606, 200, 200, 42,
    176, 2567, 2688, 188, 2568, 5526, 2689, 2798, 2799, 200, 2496, 2897, 2617, 2897, 2898, 2727,
    212, 212, 212, 54, 176, 2567, 2688, 188, 2568, 5526, 2689, 2798, 2799, 200, 2569, 5527, 2690,
    5536, 5581, 2800, 2897, 2898, 2899, 212, 2507, 2985, 2628, 2985, 2986, 2738, 2985, 2986, 2987,
    2837, 224, 224, 224, 224, 66, 176, 2567, 2688, 188, 2568, 5526, 2689, 2798, 2799, 200, 2569,
    5527, 2690, 5536, 5581, 2800, 2897, 2898, 2899, 212, 2570, 5528, 2691, 5537, 5582, 2801, 5545,
    5590, 5626, 2900, 2985, 2986, 2987, 2988, 224, 2518, 3062, 2639, 3062, 3063, 2749, 3062, 3063,
    3064, 2848, 3062, 3063, 3064, 3065, 2936, 236, 236, 236, 236, 236, 78, 176, 2567, 2688, 188,
    2568, 5526, 2689, 2798, 2799, 200, 2569, 5527, 2690, 5536, 5581, 2800, 2897, 2898, 2899, 212,
    2570, 5528, 2691, 5537, 5582, 2801, 5545, 5590, 5626, 2900, 2985, 2986, 2987, 2988, 224, 2571,
    5529, 2692, 5538, 5583, 2802, 5546, 5591, 5627, 2901, 5553, 5598, 5634, 5662, 2989, 3062, 3063,
    3064, 3065, 3066, 236, 2529, 3128, 2650, 3128, 3129, 2760, 3128, 3129, 3130, 2859, 3128, 3129,
    3130, 3131, 2947, 3128, 3129, 3130, 3131, 3132, 3024, 248, 248, 248, 248, 248, 248, 90, 176,
    2567, 2688, 188, 2568, 5526, 2689, 2798, 2799, 200, 2569, 5527, 2690, 5536, 5581, 2800, 2897,
    2898, 2899, 212, 2570, 5528, 2691, 5537, 5582, 2801, 5545, 5590, 5626, 2900, 2985, 2986, 2987,
    2988, 224, 2571, 5529, 2692, 5538, 5583, 2802, 5546, 5591, 5627, 2901, 5553, 5598, 5634, 5662,
    2989, 3062, 3063, 3064, 3065, 3066, 236, 2572, 5530, 2693, 5539, 5584, 2803, 5547, 5592, 5628,
    2902, 5554, 5599, 5635, 5663, 2990, 5560, 5605, 5641, 5669, 1604, 3067, 3128, 3129, 3130, 3131,
    3132, 3133, 248, 2540, 3183, 2661, 3183, 3184, 2771, 3183, 3184, 3185, 2870, 3183, 3184, 3185,
    3186, 2958, 3183, 3184, 3185, 3186, 3187, 3035, 3183, 3184, 3185, 3186, 3187, 3188, 3101, 260,
    260, 260, 260, 260, 260, 260, 102, 174, 2545, 2666, 186, 2546, 3227, 2667, 2776, 2777, 198,
    2547, 3227, 2668, 3227, 3228, 2778, 2875, 2876, 2877, 210, 2548, 3227, 2669, 3227, 3228, 2779,
    3227, 3228, 3229, 2878, 2963, 2964, 2965, 2966, 222, 2549, 3227, 2670, 3227, 3228, 2780, 3227,
    3228, 3229, 2879, 3227, 3228, 3229, 3230, 2967, 3040, 3041, 3042, 3043, 3044, 234, 2550, 3227,
    2671, 3227, 3228, 2781, 3227, 3228, 3229, 2880, 3227, 3228, 3229, 3230, 2968, 3227, 3228, 3229,
    3230, 3231, 3045, 3106, 3107, 3108, 3109, 3110, 3111, 246, 2551, 3227, 2672, 3227, 3228, 2782,
    3227, 3228, 3229, 2881, 3227, 3228, 3229, 3230, 2969, 3227, 3228, 3229, 3230, 3231, 3046, 3227,
    3228, 3229, 3230, 3231, 3232, 3112, 3161, 3162, 3163, 3164, 3165, 3166, 3167, 258, 263, 272,
    264, 272, 272, 265, 272, 272, 272, 266, 272, 272, 272, 272, 267, 272, 272, 272, 272, 272, 268,
    272, 272, 272, 272, 272, 272, 269, 272, 272, 272, 272, 272, 272, 272, 270, 107, 108, 109, 110,
    111, 112, 113, 114, 19, 176, 2475, 188, 31, 176, 2567, 2688, 188, 2486, 2798, 2607, 200, 200,
    43, 176, 2567, 2688, 188, 2568, 5526, 2689, 2798, 2799, 200, 2497, 2897, 2618, 2897, 2898,
    2728, 212, 212, 212, 55, 176, 2567, 2688, 188, 2568, 5526, 2689, 2798, 2799, 200, 2569, 5527,
    2690, 5536, 5581, 2800, 2897, 2898, 2899, 212, 2508, 2985, 2629, 2985, 2986, 2739, 2985, 2986,
    2987, 2838, 224, 224, 224, 224, 67, 176, 2567, 2688, 188, 2568, 5526, 2689, 2798, 2799, 200,
    2569, 5527, 2690, 5536, 5581, 2800, 2897, 2898, 2899, 212, 2570, 5528, 2691, 5537, 5582, 2801,
    5545, 5590, 5626, 2900, 2985, 2986, 2987, 2988, 224, 2519, 3062, 2640, 3062, 3063, 2750, 3062,
    3063, 3064, 2849, 3062, 3063, 3064, 3065, 2937, 236, 236, 236, 236, 236, 79, 176, 2567, 2688,
    188, 2568, 5526, 2689, 2798, 2799, 200, 2569, 5527, 2690, 5536, 5581, 2800, 2897, 2898, 2899,
    212, 2570, 5528, 2691, 5537, 5582, 2801, 5545, 5590, 5626, 2900, 2985, 2986, 2987, 2988, 224,
    2571, 5529, 2692, 5538, 5583, 2802, 5546, 5591, 5627, 2901, 5553, 5598, 5634, 5662, 2989, 3062,
    3063, 3064, 3065, 3066, 236, 2530, 3128, 2651, 3128, 3129, 2761, 3128, 3129, 3130, 2860, 3128,
    3129, 3130, 3131, 2948, 3128, 3129, 3130, 3131, 3132, 3025, 248, 248, 248, 248, 248, 248, 91,
    176, 2567, 2688, 188, 2568, 5526, 2689, 2798, 2799, 200, 2569, 5527, 2690, 5536, 5581, 2800,
    2897, 2898, 2899, 212, 2570, 5528, 2691, 5537, 5582, 2801, 5545, 5590, 5626, 2900, 2985, 2986,
    2987, 2988, 224, 2571, 5529, 2692, 5538, 5583, 2802, 5546, 5591, 5627, 2901, 5553, 5598, 5634,
    5662, 2989, 3062, 3063, 3064, 3065, 3066, 236, 2572, 5530, 2693, 5539, 5584, 2803, 5547, 5592,
    5628, 2902, 5554, 5599, 5635, 5663, 2990, 5560, 5605, 5641, 5669, 5690, 3067, 3128, 3129, 3130,
    3131, 3132, 3133, 248, 2541, 3183, 2662, 3183, 3184, 2772, 3183, 3184, 3185, 2871, 3183, 3184,
    3185, 3186, 2959, 3183, 3184, 3185, 3186, 3187, 3036, 3183, 3184, 3185, 3186, 3187, 3188, 3102,
    260, 260, 260, 260, 260, 260, 260, 103, 176, 2567, 2688, 188, 2568, 5526, 2689, 2798, 2799,
    200, 2569, 5527, 2690, 5536, 5581, 2800, 2897, 2898, 2899, 212, 2570, 5528, 2691, 5537, 5582,
    2801, 5545, 5590, 5626, 2900, 2985, 2986, 2987, 2988, 224, 2571, 5529, 2692, 5538, 5583, 2802,
    5546, 5591, 5627, 2901, 5553, 5598, 5634, 5662, 2989, 3062, 3063, 3064, 3065, 3066, 236, 2572,
    5530, 2693, 5539, 5584, 2803, 5547, 5592, 5628, 2902, 5554, 5599, 5635, 5663, 2990, 5560, 5605,
    5641, 5669, 5690, 3067, 3128, 3129, 3130, 3131, 3132, 3133, 248, 2573, 5531, 2694, 5540, 5585,
    2804, 5548, 5593, 5629, 2903, 5555, 5600, 5636, 5664, 2991, 5561, 5606, 5642, 5670, 5691, 3068,
    1606, 1606, 1606, 1606, 1606, 1605, 1606, 3183, 3184, 3185, 3186, 3187, 3188, 1606, 260, 2552,
    3227, 2673, 3227, 3228, 2783, 3227, 3228, 3229, 2882, 3227, 3228, 3229, 3230, 2970, 3227, 3228,
    3229, 3230, 3231, 3047, 3227, 3228, 3229, 3230, 3231, 3232, 3113, 3227, 3228, 3229, 3230, 3231,
    3232, 1606, 3168, 272, 272, 272, 272, 272, 272, 272, 272, 115, 175, 2556, 2677, 187, 2557,
    3260, 2678, 2787, 2788, 199, 2558, 3260, 2679, 3260, 3261, 2789, 2886, 2887, 2888, 211, 2559,
    3260, 2680, 3260, 3261, 2790, 3260, 3261, 3262, 2889, 2974, 2975, 2976, 2977, 223, 2560, 3260,
    2681, 3260, 3261, 2791, 3260, 3261, 3262, 2890, 3260, 3261, 3262, 3263, 2978, 3051, 3052, 3053,
    3054, 3055, 235, 2561, 3260, 2682, 3260, 3261, 2792, 3260, 3261, 3262, 2891, 3260, 3261, 3262,
    3263, 2979, 3260, 3261, 3262, 3263, 3264, 3056, 3117, 3118, 3119, 3120, 3121, 3122, 247, 2562,
    3260, 2683, 3260, 3261, 2793, 3260, 3261, 3262, 2892, 3260, 3261, 3262, 3263, 2980, 3260, 3261,
    3262, 3263, 3264, 3057, 3260, 3261, 3262, 3263, 3264, 3265, 3123, 3172, 3173, 3174, 3175, 3176,
    3177, 3178, 259, 2563, 3260, 2684, 3260, 3261, 2794, 3260, 3261, 3262, 2893, 3260, 3261, 3262,
    3263, 2981, 3260, 3261, 3262, 3263, 3264, 3058, 3260, 3261, 3262, 3263, 3264, 3265, 3124, 3260,
    3261, 3262, 3263, 3264, 3265, 1606, 3179, 3216, 3217, 3218, 3219, 3220, 3221, 3222, 3223, 271,
    275, 284, 276, 284, 284, 277, 284, 284, 284, 278, 284, 284, 284, 284, 279, 284, 284, 284, 284,
    284, 280, 284, 284, 284, 284, 284, 284, 281, 284, 284, 284, 284, 284, 284, 284, 282, 284, 284,
    284, 284, 284, 284, 284, 284, 283, 119, 120, 121, 122, 123, 124, 125, 126, 127, 20, 176, 287,
    188, 32, 176, 287, 288, 188, 287, 289, 288, 200, 200, 44, 176, 287, 288, 188, 287, 2270, 288,
    289, 289, 200, 287, 290, 288, 290, 290, 289, 212, 212, 212, 56, 176, 287, 288, 188, 287, 2270,
    288, 289, 289, 200, 287, 2270, 288, 2271, 2281, 289, 290, 290, 290, 212, 287, 291, 288, 291,
    291, 289, 291, 291, 291, 290, 224, 224, 224, 224, 68, 176, 287, 288, 188, 287, 2270, 288, 289,
    289, 200, 287, 2270, 288, 2271, 2281, 289, 290, 290, 290, 212, 287, 2270, 288, 2271, 2281, 289,
    2272, 2282, 2291, 290, 291, 291, 291, 291, 224, 287, 292, 288, 292, 292, 289, 292, 292, 292,
    290, 292, 292, 292, 292, 291, 236, 236, 236, 236, 236, 80, 176, 287, 288, 188, 287, 2270, 288,
    289, 289, 200, 287, 2270, 288, 2271, 2281, 289, 290, 290, 290, 212, 287, 2270, 288, 2271, 2281,
    289, 2272, 2282, 2291, 290, 291, 291, 291, 291, 224, 287, 2270, 288, 2271, 2281, 289, 2272,
    2282, 2291, 290, 2273, 2283, 2292, 2300, 291, 292, 292, 292, 292, 292, 236, 287, 293, 288, 293,
    293, 289, 293, 293, 293, 290, 293, 293, 293, 293, 291, 293, 293, 293, 293, 293, 292, 248, 248,
    248, 248, 248, 248, 92, 176, 287, 288, 188, 287, 2270, 288, 289, 289, 200, 287, 2270, 288,
    2271, 2281, 289, 290, 290, 290, 212, 287, 2270, 288, 2271, 2281, 289, 2272, 2282, 2291, 290,
    291, 291, 291, 291, 224, 287, 2270, 288, 2271, 2281, 289, 2272, 2282, 2291, 290, 2273, 2283,
    2292, 2300, 291, 292, 292, 292, 292, 292, 236, 287, 2270, 288, 2271, 2281, 289, 2272, 2282,
    2291, 290, 2273, 2283, 2292, 2300, 291, 2274, 2284, 2293, 2301, 2308, 292, 293, 293, 293, 293,
    293, 293, 248, 287, 294, 288, 294, 294, 289, 294, 294, 294, 290, 294, 294, 294, 294, 291, 294,
    294, 294, 294, 294, 292, 294, 294, 294, 294, 294, 294, 293, 260, 260, 260, 260, 260, 260, 260,
    104, 176, 287, 288, 188, 287, 2270, 288, 289, 289, 200, 287, 2270, 288, 2271, 2281, 289, 290,
    290, 290, 212, 287, 2270, 288, 2271, 2281, 289, 2272, 2282, 2291, 290, 291, 291, 291, 291, 224,
    287, 2270, 288, 2271, 2281, 289, 2272, 2282, 2291, 290, 2273, 2283, 2292, 2300, 291, 292, 292,
    292, 292, 292, 236, 287, 2270, 288, 2271, 2281, 289, 2272, 2282, 2291, 290, 2273, 2283, 2292,
    2300, 291, 2274, 2284, 2293, 2301, 2308, 292, 293, 293, 293, 293, 293, 293, 248, 287, 2270,
    288, 2271, 2281, 289, 2272, 2282, 2291, 290, 2273, 2283, 2292, 2300, 291, 2274, 2284, 2293,
    2301, 2308, 292, 2275, 2285, 2294, 2302, 2309, 2315, 293, 294, 294, 294, 294, 294, 294, 294,
    260, 287, 295, 288, 295, 295, 289, 295, 295, 295, 290, 295, 295, 295, 295, 291, 295, 295, 295,
    295, 295, 292, 295, 295, 295, 295, 295, 295, 293, 295, 295, 295, 295, 295, 295, 295, 294, 272,
    272, 272, 272, 272, 272, 272, 272, 116, 176, 287, 288, 188, 287, 2270, 288, 289, 289, 200, 287,
    2270, 288, 2271, 2281, 289, 290, 290, 290, 212, 287, 2270, 288, 2271, 2281, 289, 2272, 2282,
    2291, 290, 291, 291, 291, 291, 224, 287, 2270, 288, 2271, 2281, 289, 2272, 2282, 2291, 290,
    2273, 2283, 2292, 2300, 291, 292, 292, 292, 292, 292, 236, 287, 2270, 288, 2271, 2281, 289,
    2272, 2282, 2291, 290, 2273, 2283, 2292, 2300, 291, 2274, 2284, 2293, 2301, 2308, 292, 293,
    293, 293, 293, 293, 293, 248, 287, 2270, 288, 2271, 2281, 289, 2272, 2282, 2291, 290, 2273,
    2283, 2292, 2300, 291, 2274, 2284, 2293, 2301, 2308, 292, 2275, 2285, 2294, 2302, 2309, 2315,
    293, 294, 294, 294, 294, 294, 294, 294, 260, 287, 2270, 288, 2271, 2281, 289, 2272, 2282, 2291,
    290, 2273, 2283, 2292, 2300, 291, 2274, 2284, 2293, 2301, 2308, 292, 2275, 2285, 2294, 2302,
    2309, 2315, 293, 2276, 2286, 2295, 2303, 2310, 2316, 1606, 294, 295, 295, 295, 295, 295, 295,
    295, 295, 272, 287, 296, 288, 296, 296, 289, 296, 296, 296, 290, 296, 296, 296, 296, 291, 296,
    296, 296, 296, 296, 292, 296, 296, 296, 296, 296, 296, 293, 296, 296, 296, 296, 296, 296, 296,
    294, 296, 296, 296, 296, 296, 296, 296, 296, 295, 284, 284, 284, 284, 284, 284, 284, 284, 284,
    128, 131, 131, 131, 132, 131, 131, 132, 131, 132, 133, 131, 131, 132, 131, 132, 133, 131, 132,
    133, 134, 131, 131, 132, 131, 132, 133, 131, 132, 133, 134, 131, 132, 133, 134, 135, 131, 131,
    132, 131, 132, 133, 131, 132, 133, 134, 131, 132, 133, 134, 135, 131, 132, 133, 134, 135, 136,
    131, 131, 132, 131, 132, 133, 131, 132, 133, 134, 131, 132, 133, 134, 135, 131, 132, 133, 134,
    135, 136, 131, 132, 133, 134, 135, 136, 137, 131, 131, 132, 131, 132, 133, 131, 132, 133, 134,
    131, 132, 133, 134, 135, 131, 132, 133, 134, 135, 136, 131, 132, 133, 134, 135, 136, 137, 131,
    132, 133, 134, 135, 136, 137, 138, 131, 131, 132, 131, 132, 133, 131, 132, 133, 134, 131, 132,
    133, 134, 135, 131, 132, 133, 134, 135, 136, 131, 132, 133, 134, 135, 136, 137, 131, 132, 133,
    134, 135, 136, 137, 138, 131, 132, 133, 134, 135, 136, 137, 138, 139, 131, 131, 132, 131, 132,
    133, 131, 132, 133, 134, 131, 132, 133, 134, 135, 131, 132, 133, 134, 135, 136, 131, 132, 133,
    134, 135, 136, 137, 131, 132, 133, 134, 135, 136, 137, 138, 131, 132, 133, 134, 135, 136, 137,
    138, 139, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 11, 167, 23, 11, 167, 179, 23, 12,
    168, 2468, 180, 24, 168, 191, 192, 180, 35, 35, 36, 11, 167, 179, 23, 12, 1610, 2468, 1676, 24,
    168, 2479, 2600, 180, 191, 1742, 192, 35, 36, 13, 169, 2469, 181, 25, 169, 2490, 2611, 181,
    2480, 2721, 2601, 193, 193, 37, 169, 203, 204, 181, 203, 1808, 204, 205, 205, 193, 47, 47, 48,
    47, 48, 49, 11, 167, 179, 23, 12, 1610, 2468, 1676, 24, 168, 2479, 2600, 180, 191, 1742, 192,
    35, 36, 13, 1611, 2469, 1677, 25, 1621, 1600, 1600, 1687, 2480, 1600, 2601, 1743, 1753, 37,
    169, 2490, 2611, 181, 2491, 1600, 2612, 2721, 2722, 193, 203, 1808, 204, 1809, 1819, 205, 47,
    48, 49, 14, 170, 2470, 182, 26, 170, 2501, 2622, 182, 2481, 2732, 2602, 194, 194, 38, 170,
    2501, 2622, 182, 2502, 1600, 2623, 2732, 2733, 194, 2492, 2831, 2613, 2831, 2832, 2723, 206,
    206, 206, 50, 170, 215, 216, 182, 215, 1874, 216, 217, 217, 194, 215, 1874, 216, 1875, 1885,
    217, 218, 218, 218, 206, 59, 59, 60, 59, 60, 61, 59, 60, 61, 62, 11, 167, 179, 23, 12, 1610,
    2468, 1676, 24, 168, 2479, 2600, 180, 191, 1742, 192, 35, 36, 13, 1611, 2469, 1677, 25, 1621,
    3326, 3546, 1687, 2480, 3766, 2601, 1743, 1753, 37, 169, 2490, 2611, 181, 2491, 3986, 2612,
    2721, 2722, 193, 203, 1808, 204, 1809, 1819, 205, 47, 48, 49, 14, 1612, 2470, 1678, 26, 1622,
    3327, 3547, 1688, 2481, 3767, 2602, 1744, 1754, 38, 1631, 3336, 3556, 1697, 3381, 1600, 1601,
    3776, 1601, 1763, 2492, 3987, 2613, 3996, 1601, 2723, 1810, 1820, 1829, 50, 170, 2501, 2622,
    182, 2502, 4206, 2623, 2732, 2733, 194, 2503, 4207, 2624, 4216, 1601, 2734, 2831, 2832, 2833,
    206, 215, 1874, 216, 1875, 1885, 217, 1876, 1886, 1895, 218, 59, 60, 61, 62, 15, 171, 2471,
    183, 27, 171, 2512, 2633, 183, 2482, 2743, 2603, 195, 195, 39, 171, 2512, 2633, 183, 2513,
    4426, 2634, 2743, 2744, 195, 2493, 2842, 2614, 2842, 2843, 2724, 207, 207, 207, 51, 171, 2512,
    2633, 183, 2513, 4426, 2634, 2743, 2744, 195, 2514, 4427, 2635, 4436, 1601, 2745, 2842, 2843,
    2844, 207, 2504, 2930, 2625, 2930, 2931, 2735, 2930, 2931, 2932, 2834, 219, 219, 219, 219, 63,
    171, 227, 228, 183, 227, 1940, 228, 229, 229, 195, 227, 1940, 228, 1941, 1951, 229, 230, 230,
    230, 207, 227, 1940, 228, 1941, 1951, 229, 1942, 1952, 1961, 230, 231, 231, 231, 231, 219, 71,
    71, 72, 71, 72, 73, 71, 72, 73, 74, 71, 72, 73, 74, 75, 11, 167, 179, 23, 12, 1610, 2468, 1676,
    24, 168, 2479, 2600, 180, 191, 1742, 192, 35, 36, 13, 1611, 2469, 1677, 25, 1621, 3326, 3546,
    1687, 2480, 3766, 2601, 1743, 1753, 37, 169, 2490, 2611, 181, 2491, 3986, 2612, 2721, 2722,
    193, 203, 1808, 204, 1809, 1819, 205, 47, 48, 49, 14, 1612, 2470, 1678, 26, 1622, 3327, 3547,
    1688, 2481, 3767, 2602, 1744, 1754, 38, 1631, 3336, 3556, 1697, 3381, 1600, 3601, 3776, 3821,
    1763, 2492, 3987, 2613, 3996, 4041, 2723, 1810, 1820, 1829, 50, 170, 2501, 2622, 182, 2502,
    4206, 2623, 2732, 2733, 194, 2503, 4207, 2624, 4216, 4261, 2734, 2831, 2832, 2833, 206, 215,
    1874, 216, 1875, 1885, 217, 1876, 1886, 1895, 218, 59, 60, 61, 62, 15, 1613, 2471, 1679, 27,
    1623, 3328, 3548, 1689, 2482, 3768, 2603, 1745, 1755, 39, 1632, 3337, 3557, 1698, 3382, 6186,
    3602, 3777, 3822, 1764, 2493, 3988, 2614, 3997, 4042, 2724, 1811, 1821, 1830, 51, 1640, 3345,
    3565, 1706, 3390, 6194, 3610, 3785, 3830, 1772, 3426, 6230, 3646, 1602, 1601, 1602, 4005, 4050,
    1602, 1838, 2504, 4208, 2625, 4217, 4262, 2735, 4225, 4270, 1602, 2834, 1877, 1887, 1896, 1904,
    63, 171, 2512, 2633, 183, 2513, 4426, 2634, 2743, 2744, 195, 2514, 4427, 2635, 4436, 4481,
    2745, 2842, 2843, 2844, 207, 2515, 4428, 2636, 4437, 4482, 2746, 4445, 4490, 1602, 2845, 2930,
    2931, 2932, 2933, 219, 227, 1940, 228, 1941, 1951, 229, 1942, 1952, 1961, 230, 1943, 1953,
    1962, 1970, 231, 71, 72, 73, 74, 75, 16, 172, 2472, 184, 28, 172, 2523, 2644, 184, 2483, 2754,
    2604, 196, 196, 40, 172, 2523, 2644, 184, 2524, 4646, 2645, 2754, 2755, 196, 2494, 2853, 2615,
    2853, 2854, 2725, 208, 208, 208, 52, 172, 2523, 2644, 184, 2524, 4646, 2645, 2754, 2755, 196,
    2525, 4647, 2646, 4656, 4701, 2756, 2853, 2854, 2855, 208, 2505, 2941, 2626, 2941, 2942, 2736,
    2941, 2942, 2943, 2835, 220, 220, 220, 220, 64, 172, 2523, 2644, 184, 2524, 4646, 2645, 2754,
    2755, 196, 2525, 4647, 2646, 4656, 4701, 2756, 2853, 2854, 2855, 208, 2526, 4648, 2647, 4657,
    4702, 2757, 4665, 4710, 1602, 2856, 2941, 2942, 2943, 2944, 220, 2516, 3018, 2637, 3018, 3019,
    2747, 3018, 3019, 3020, 2846, 3018, 3019, 3020, 3021, 2934, 232, 232, 232, 232, 232, 76, 172,
    239, 240, 184, 239, 2006, 240, 241, 241, 196, 239, 2006, 240, 2007, 2017, 241, 242, 242, 242,
    208, 239, 2006, 240, 2007, 2017, 241, 2008, 2018, 2027, 242, 243, 243, 243, 243, 220, 239,
    2006, 240, 2007, 2017, 241, 2008, 2018, 2027, 242, 2009, 2019, 2028, 2036, 243, 244, 244, 244,
    244, 244, 232, 83, 83, 84, 83, 84, 85, 83, 84, 85, 86, 83, 84, 85, 86, 87, 83, 84, 85, 86, 87,
    88, 11, 167, 179, 23, 12, 1610, 2468, 1676, 24, 168, 2479, 2600, 180, 191, 1742, 192, 35, 36,
    13, 1611, 2469, 1677, 25, 1621, 3326, 3546, 1687, 2480, 3766, 2601, 1743, 1753, 37, 169, 2490,
    2611, 181, 2491, 3986, 2612, 2721, 2722, 193, 203, 1808, 204, 1809, 1819, 205, 47, 48, 49, 14,
    1612, 2470, 1678, 26, 1622, 3327, 3547, 1688, 2481, 3767, 2602, 1744, 1754, 38, 1631, 3336,
    3556, 1697, 3381, 1600, 3601, 3776, 3821, 1763, 2492, 3987, 2613, 3996, 4041, 2723, 1810, 1820,
    1829, 50, 170, 2501, 2622, 182, 2502, 4206, 2623, 2732, 2733, 194, 2503, 4207, 2624, 4216,
    4261, 2734, 2831, 2832, 2833, 206, 215, 1874, 216, 1875, 1885, 217, 1876, 1886, 1895, 218, 59,
    60, 61, 62, 15, 1613, 2471, 1679, 27, 1623, 3328, 3548, 1689, 2482, 3768, 2603, 1745, 1755, 39,
    1632, 3337, 3557, 1698, 3382, 6186, 3602, 3777, 3822, 1764, 2493, 3988, 2614, 3997, 4042, 2724,
    1811, 1821, 1830, 51, 1640, 3345, 3565, 1706, 3390, 6194, 3610, 3785, 3830, 1772, 3426, 6230,
    3646, 6350, 1601, 3866, 4005, 4050, 4086, 1838, 2504, 4208, 2625, 4217, 4262, 2735, 4225, 4270,
    4306, 2834, 1877, 1887, 1896, 1904, 63, 171, 2512, 2633, 183, 2513, 4426, 2634, 2743, 2744,
    195, 2514, 4427, 2635, 4436, 4481, 2745, 2842, 2843, 2844, 207, 2515, 4428, 2636, 4437, 4482,
    2746, 4445, 4490, 4526, 2845, 2930, 2931, 2932, 2933, 219, 227, 1940, 228, 1941, 1951, 229,
    1942, 1952, 1961, 230, 1943, 1953, 1962, 1970, 231, 71, 72, 73, 74, 75, 16, 1614, 2472, 1680,
    28, 1624, 3329, 3549, 1690, 2483, 3769, 2604, 1746, 1756, 40, 1633, 3338, 3558, 1699, 3383,
    6187, 3603, 3778, 3823, 1765, 2494, 3989, 2615, 3998, 4043, 2725, 1812, 1822, 1831, 52, 1641,
    3346, 3566, 1707, 3391, 6195, 3611, 3786, 3831, 1773, 3427, 6231, 3647, 6351, 6679, 3867, 4006,
    4051, 4087, 1839, 2505, 4209, 2626, 4218, 4263, 2736, 4226, 4271, 4307, 2835, 1878, 1888, 1897,
    1905, 64, 1648, 3353, 3573, 1714, 3398, 6202, 3618, 3793, 3838, 1780, 3434, 6238, 3654, 6358,
    6686, 3874, 4013, 4058, 4094, 1846, 3462, 6266, 3682, 6386, 6714, 3902, 1603, 1603, 1602, 1603,
    4233, 4278, 4314, 1603, 1912, 2516, 4429, 2637, 4438, 4483, 2747, 4446, 4491, 4527, 2846, 4453,
    4498, 4534, 1603, 2934, 1944, 1954, 1963, 1971, 1978, 76, 172, 2523, 2644, 184, 2524, 4646,
    2645, 2754, 2755, 196, 2525, 4647, 2646, 4656, 4701, 2756, 2853, 2854, 2855, 208, 2526, 4648,
    2647, 4657, 4702, 2757, 4665, 4710, 4746, 2856, 2941, 2942, 2943, 2944, 220, 2527, 4649, 2648,
    4658, 4703, 2758, 4666, 4711, 4747, 2857, 4673, 4718, 4754, 1603, 2945, 3018, 3019, 3020, 3021,
    3022, 232, 239, 2006, 240, 2007, 2017, 241, 2008, 2018, 2027, 242, 2009, 2019, 2028, 2036, 243,
    2010, 2020, 2029, 2037, 2044, 244, 83, 84, 85, 86, 87, 88, 17, 173, 2473, 185, 29, 173, 2534,
    2655, 185, 2484, 2765, 2605, 197, 197, 41, 173, 2534, 2655, 185, 2535, 4866, 2656, 2765, 2766,
    197, 2495, 2864, 2616, 2864, 2865, 2726, 209, 209, 209, 53, 173, 2534, 2655, 185, 2535, 4866,
    2656, 2765, 2766, 197, 2536, 4867, 2657, 4876, 4921, 2767, 2864, 2865, 2866, 209, 2506, 2952,
    2627, 2952, 2953, 2737, 2952, 2953, 2954, 2836, 221, 221, 221, 221, 65, 173, 2534, 2655, 185,
    2535, 4866, 2656, 2765, 2766, 197, 2536, 4867, 2657, 4876, 4921, 2767, 2864, 2865, 2866, 209,
    2537, 4868, 2658, 4877, 4922, 2768, 4885, 4930, 4966, 2867, 2952, 2953, 2954, 2955, 221, 2517,
    3029, 2638, 3029, 3030, 2748, 3029, 3030, 3031, 2847, 3029, 3030, 3031, 3032, 2935, 233, 233,
    233, 233, 233, 77, 173, 2534, 2655, 185, 2535, 4866, 2656, 2765, 2766, 197, 2536, 4867, 2657,
    4876, 4921, 2767, 2864, 2865, 2866, 209, 2537, 4868, 2658, 4877, 4922, 2768, 4885, 4930, 4966,
    2867, 2952, 2953, 2954, 2955, 221, 2538, 4869, 2659, 4878, 4923, 2769, 4886, 4931, 4967, 2868,
    4893, 4938, 4974, 1603, 2956, 3029, 3030, 3031, 3032, 3033, 233, 2528, 3095, 2649, 3095, 3096,
    2759, 3095, 3096, 3097, 2858, 3095, 3096, 3097, 3098, 2946, 3095, 3096, 3097, 3098, 3099, 3023,
    245, 245, 245, 245, 245, 245, 89, 173, 251, 252, 185, 251, 2072, 252, 253, 253, 197, 251, 2072,
    252, 2073, 2083, 253, 254, 254, 254, 209, 251, 2072, 252, 2073, 2083, 253, 2074, 2084, 2093,
    254, 255, 255, 255, 255, 221, 251, 2072, 252, 2073, 2083, 253, 2074, 2084, 2093, 254, 2075,
    2085, 2094, 2102, 255, 256, 256, 256, 256, 256, 233, 251, 2072, 252, 2073, 2083, 253, 2074,
    2084, 2093, 254, 2075, 2085, 2094, 2102, 255, 2076, 2086, 2095, 2103, 2110, 256, 257, 257, 257,
    257, 257, 257, 245, 95, 95, 96, 95, 96, 97, 95, 96, 97, 98, 95, 96, 97, 98, 99, 95, 96, 97, 98,
    99, 100, 95, 96, 97, 98, 99, 100, 101, 11, 167, 179, 23, 12, 1610, 2468, 1676, 24, 168, 2479,
    2600, 180, 191, 1742, 192, 35, 36, 13, 1611, 2469, 1677, 25, 1621, 3326, 3546, 1687, 2480,
    3766, 2601, 1743, 1753, 37, 169, 2490, 2611, 181, 2491, 3986, 2612, 2721, 2722, 193, 203, 1808,
    204, 1809, 1819, 205, 47, 48, 49, 14, 1612, 2470, 1678, 26, 1622, 3327, 3547, 1688, 2481, 3767,
    2602, 1744, 1754, 38, 1631, 3336, 3556, 1697, 3381, 1600, 3601, 3776, 3821, 1763, 2492, 3987,
    2613, 3996, 4041, 2723, 1810, 1820, 1829, 50, 170, 2501, 2622, 182, 2502, 4206, 2623, 2732,
    2733, 194, 2503, 4207, 2624, 4216, 4261, 2734, 2831, 2832, 2833, 206, 215, 1874, 216, 1875,
    1885, 217, 1876, 1886, 1895, 218, 59, 60, 61, 62, 15, 1613, 2471, 1679, 27, 1623, 3328, 3548,
    1689, 2482, 3768, 2603, 1745, 1755, 39, 1632, 3337, 3557, 1698, 3382, 6186, 3602, 3777, 3822,
    1764, 2493, 3988, 2614, 3997, 4042, 2724, 1811, 1821, 1830, 51, 1640, 3345, 3565, 1706, 3390,
    6194, 3610, 3785, 3830, 1772, 3426, 6230, 3646, 6350, 1601, 3866, 4005, 4050, 4086, 1838, 2504,
    4208, 2625, 4217, 4262, 2735, 4225, 4270, 4306, 2834, 1877, 1887, 1896, 1904, 63, 171, 2512,
    2633, 183, 2513, 4426, 2634, 2743, 2744, 195, 2514, 4427, 2635, 4436, 4481, 2745, 2842, 2843,
    2844, 207, 2515, 4428, 2636, 4437, 4482, 2746, 4445, 4490, 4526, 2845, 2930, 2931, 2932, 2933,
    219, 227, 1940, 228, 1941, 1951, 229, 1942, 1952, 1961, 230, 1943, 1953, 1962, 1970, 231, 71,
    72, 73, 74, 75, 16, 1614, 2472, 1680, 28, 1624, 3329, 3549, 1690, 2483, 3769, 2604, 1746, 1756,
    40, 1633, 3338, 3558, 1699, 3383, 6187, 3603, 3778, 3823, 1765, 2494, 3989, 2615, 3998, 4043,
    2725, 1812, 1822, 1831, 52, 1641, 3346, 3566, 1707, 3391, 6195, 3611, 3786, 3831, 1773, 3427,
    6231, 3647, 6351, 6679, 3867, 4006, 4051, 4087, 1839, 2505, 4209, 2626, 4218, 4263, 2736, 4226,
    4271, 4307, 2835, 1878, 1888, 1897, 1905, 64, 1648, 3353, 3573, 1714, 3398, 6202, 3618, 3793,
    3838, 1780, 3434, 6238, 3654, 6358, 6686, 3874, 4013, 4058, 4094, 1846, 3462, 6266, 3682, 6386,
    6714, 3902, 6470, 6798, 1602, 4122, 4233, 4278, 4314, 4342, 1912, 2516, 4429, 2637, 4438, 4483,
    2747, 4446, 4491, 4527, 2846, 4453, 4498, 4534, 4562, 2934, 1944, 1954, 1963, 1971, 1978, 76,
    172, 2523, 2644, 184, 2524, 4646, 2645, 2754, 2755, 196, 2525, 4647, 2646, 4656, 4701, 2756,
    2853, 2854, 2855, 208, 2526, 4648, 2647, 4657, 4702, 2757, 4665, 4710, 4746, 2856, 2941, 2942,
    2943, 2944, 220, 2527, 4649, 2648, 4658, 4703, 2758, 4666, 4711, 4747, 2857, 4673, 4718, 4754,
    4782, 2945, 3018, 3019, 3020, 3021, 3022, 232, 239, 2006, 240, 2007, 2017, 241, 2008, 2018,
    2027, 242, 2009, 2019, 2028, 2036, 243, 2010, 2020, 2029, 2037, 2044, 244, 83, 84, 85, 86, 87,
    88, 17, 1615, 2473, 1681, 29, 1625, 3330, 3550, 1691, 2484, 3770, 2605, 1747, 1757, 41, 1634,
    3339, 3559, 1700, 3384, 6188, 3604, 3779, 3824, 1766, 2495, 3990, 2616, 3999, 4044, 2726, 1813,
    1823, 1832, 53, 1642, 3347, 3567, 1708, 3392, 6196, 3612, 3787, 3832, 1774, 3428, 6232, 3648,
    6352, 6680, 3868, 4007, 4052, 4088, 1840, 2506, 4210, 2627, 4219, 4264, 2737, 4227, 4272, 4308,
    2836, 1879, 1889, 1898, 1906, 65, 1649, 3354, 3574, 1715, 3399, 6203, 3619, 3794, 3839, 1781,
    3435, 6239, 3655, 6359, 6687, 3875, 4014, 4059, 4095, 1847, 3463, 6267, 3683, 6387, 6715, 3903,
    6471, 6799, 7008, 4123, 4234, 4279, 4315, 4343, 1913, 2517, 4430, 2638, 4439, 4484, 2748, 4447,
    4492, 4528, 2847, 4454, 4499, 4535, 4563, 2935, 1945, 1955, 1964, 1972, 1979, 77, 1655, 3360,
    3580, 1721, 3405, 6209, 3625, 3800, 3845, 1787, 3441, 6245, 3661, 6365, 6693, 3881, 4020, 4065,
    4101, 1853, 3469, 6273, 3689, 6393, 6721, 3909, 6477, 6805, 7014, 4129, 4240, 4285, 4321, 4349,
    1919, 3490, 6294, 3710, 6414, 6742, 3930, 6498, 6826, 7035, 4150, 1604, 1604, 1604, 1603, 1604,
    4460, 4505, 4541, 4569, 1604, 1985, 2528, 4650, 2649, 4659, 4704, 2759, 4667, 4712, 4748, 2858,
    4674, 4719, 4755, 4783, 2946, 4680, 4725, 4761, 4789, 1604, 3023, 2011, 2021, 2030, 2038, 2045,
    2051, 89, 173, 2534, 2655, 185, 2535, 4866, 2656, 2765, 2766, 197, 2536, 4867, 2657, 4876,
    4921, 2767, 2864, 2865, 2866, 209, 2537, 4868, 2658, 4877, 4922, 2768, 4885, 4930, 4966, 2867,
    2952, 2953, 2954, 2955, 221, 2538, 4869, 2659, 4878, 4923, 2769, 4886, 4931, 4967, 2868, 4893,
    4938, 4974, 5002, 2956, 3029, 3030, 3031, 3032, 3033, 233, 2539, 4870, 2660, 4879, 4924, 2770,
    4887, 4932, 4968, 2869, 4894, 4939, 4975, 5003, 2957, 4900, 4945, 4981, 5009, 1604, 3034, 3095,
    3096, 3097, 3098, 3099, 3100, 245, 251, 2072, 252, 2073, 2083, 253, 2074, 2084, 2093, 254,
    2075, 2085, 2094, 2102, 255, 2076, 2086, 2095, 2103, 2110, 256, 2077, 2087, 2096, 2104, 2111,
    2117, 257, 95, 96, 97, 98, 99, 100, 101, 18, 174, 2474, 186, 30, 174, 2545, 2666, 186, 2485,
    2776, 2606, 198, 198, 42, 174, 2545, 2666, 186, 2546, 5086, 2667, 2776, 2777, 198, 2496, 2875,
    2617, 2875, 2876, 2727, 210, 210, 210, 54, 174, 2545, 2666, 186, 2546, 5086, 2667, 2776, 2777,
    198, 2547, 5087, 2668, 5096, 5141, 2778, 2875, 2876, 2877, 210, 2507, 2963, 2628, 2963, 2964,
    2738, 2963, 2964, 2965, 2837, 222, 222, 222, 222, 66, 174, 2545, 2666, 186, 2546, 5086, 2667,
    2776, 2777, 198, 2547, 5087, 2668, 5096, 5141, 2778, 2875, 2876, 2877, 210, 2548, 5088, 2669,
    5097, 5142, 2779, 5105, 5150, 5186, 2878, 2963, 2964, 2965, 2966, 222, 2518, 3040, 2639, 3040,
    3041, 2749, 3040, 3041, 3042, 2848, 3040, 3041, 3042, 3043, 2936, 234, 234, 234, 234, 234, 78,
    174, 2545, 2666, 186, 2546, 5086, 2667, 2776, 2777, 198, 2547, 5087, 2668, 5096, 5141, 2778,
    2875, 2876, 2877, 210, 2548, 5088, 2669, 5097, 5142, 2779, 5105, 5150, 5186, 2878, 2963, 2964,
    2965, 2966, 222, 2549, 5089, 2670, 5098, 5143, 2780, 5106, 5151, 5187, 2879, 5113, 5158, 5194,
    5222, 2967, 3040, 3041, 3042, 3043, 3044, 234, 2529, 3106, 2650, 3106, 3107, 2760, 3106, 3107,
    3108, 2859, 3106, 3107, 3108, 3109, 2947, 3106, 3107, 3108, 3109, 3110, 3024, 246, 246, 246,
    246, 246, 246, 90, 174, 2545, 2666, 186, 2546, 5086, 2667, 2776, 2777, 198, 2547, 5087, 2668,
    5096, 5141, 2778, 2875, 2876, 2877, 210, 2548, 5088, 2669, 5097, 5142, 2779, 5105, 5150, 5186,
    2878, 2963, 2964, 2965, 2966, 222, 2549, 5089, 2670, 5098, 5143, 2780, 5106, 5151, 5187, 2879,
    5113, 5158, 5194, 5222, 2967, 3040, 3041, 3042, 3043, 3044, 234, 2550, 5090, 2671, 5099, 5144,
    2781, 5107, 5152, 5188, 2880, 5114, 5159, 5195, 5223, 2968, 5120, 5165, 5201, 5229, 1604, 3045,
    3106, 3107, 3108, 3109, 3110, 3111, 246, 2540, 3161, 2661, 3161, 3162, 2771, 3161, 3162, 3163,
    2870, 3161, 3162, 3163, 3164, 2958, 3161, 3162, 3163, 3164, 3165, 3035, 3161, 3162, 3163, 3164,
    3165, 3166, 3101, 258, 258, 258, 258, 258, 258, 258, 102, 174, 263, 264, 186, 263, 2138, 264,
    265, 265, 198, 263, 2138, 264, 2139, 2149, 265, 266, 266, 266, 210, 263, 2138, 264, 2139, 2149,
    265, 2140, 2150, 2159, 266, 267, 267, 267, 267, 222, 263, 2138, 264, 2139, 2149, 265, 2140,
    2150, 2159, 266, 2141, 2151, 2160, 2168, 267, 268, 268, 268, 268, 268, 234, 263, 2138, 264,
    2139, 2149, 265, 2140, 2150, 2159, 266, 2141, 2151, 2160, 2168, 267, 2142, 2152, 2161, 2169,
    2176, 268, 269, 269, 269, 269, 269, 269, 246, 263, 2138, 264, 2139, 2149, 265, 2140, 2150,
    2159, 266, 2141, 2151, 2160, 2168, 267, 2142, 2152, 2161, 2169, 2176, 268, 2143, 2153, 2162,
    2170, 2177, 2183, 269, 270, 270, 270, 270, 270, 270, 270, 258, 107, 107, 108, 107, 108, 109,
    107, 108, 109, 110, 107, 108, 109, 110, 111, 107, 108, 109, 110, 111, 112, 107, 108, 109, 110,
    111, 112, 113, 107, 108, 109, 110, 111, 112, 113, 114, 11, 167, 179, 23, 12, 1610, 2468, 1676,
    24, 168, 2479, 2600, 180, 191, 1742, 192, 35, 36, 13, 1611, 2469, 1677, 25, 1621, 3326, 3546,
    1687, 2480, 3766, 2601, 1743, 1753, 37, 169, 2490, 2611, 181, 2491, 3986, 2612, 2721, 2722,
    193, 203, 1808, 204, 1809, 1819, 205, 47, 48, 49, 14, 1612, 2470, 1678, 26, 1622, 3327, 3547,
    1688, 2481, 3767, 2602, 1744, 1754, 38, 1631, 3336, 3556, 1697, 3381, 1600, 3601, 3776, 3821,
    1763, 2492, 3987, 2613, 3996, 4041, 2723, 1810, 1820, 1829, 50, 170, 2501, 2622, 182, 2502,
    4206, 2623, 2732, 2733, 194, 2503, 4207, 2624, 4216, 4261, 2734, 2831, 2832, 2833, 206, 215,
    1874, 216, 1875, 1885, 217, 1876, 1886, 1895, 218, 59, 60, 61, 62, 15, 1613, 2471, 1679, 27,
    1623, 3328, 3548, 1689, 2482, 3768, 2603, 1745, 1755, 39, 1632, 3337, 3557, 1698, 3382, 6186,
    3602, 3777, 3822, 1764, 2493, 3988, 2614, 3997, 4042, 2724, 1811, 1821, 1830, 51, 1640, 3345,
    3565, 1706, 3390, 6194, 3610, 3785, 3830, 1772, 3426, 6230, 3646, 6350, 1601, 3866, 4005, 4050,
    4086, 1838, 2504, 4208, 2625, 4217, 4262, 2735, 4225, 4270, 4306, 2834, 1877, 1887, 1896, 1904,
    63, 171, 2512, 2633, 183, 2513, 4426, 2634, 2743, 2744, 195, 2514, 4427, 2635, 4436, 4481,
    2745, 2842, 2843, 2844, 207, 2515, 4428, 2636, 4437, 4482, 2746, 4445, 4490, 4526, 2845, 2930,
    2931, 2932, 2933, 219, 227, 1940, 228, 1941, 1951, 229, 1942, 1952, 1961, 230, 1943, 1953,
    1962, 1970, 231, 71, 72, 73, 74, 75, 16, 1614, 2472, 1680, 28, 1624, 3329, 3549, 1690, 2483,
    3769, 2604, 1746, 1756, 40, 1633, 3338, 3558, 1699, 3383, 6187, 3603, 3778, 3823, 1765, 2494,
    3989, 2615, 3998, 4043, 2725, 1812, 1822, 1831, 52, 1641, 3346, 3566, 1707, 3391, 6195, 3611,
    3786, 3831, 1773, 3427, 6231, 3647, 6351, 6679, 3867, 4006, 4051, 4087, 1839, 2505, 4209, 2626,
    4218, 4263, 2736, 4226, 4271, 4307, 2835, 1878, 1888, 1897, 1905, 64, 1648, 3353, 3573, 1714,
    3398, 6202, 3618, 3793, 3838, 1780, 3434, 6238, 3654, 6358, 6686, 3874, 4013, 4058, 4094, 1846,
    3462, 6266, 3682, 6386, 6714, 3902, 6470, 6798, 1602, 4122, 4233, 4278, 4314, 4342, 1912, 2516,
    4429, 2637, 4438, 4483, 2747, 4446, 4491, 4527, 2846, 4453, 4498, 4534, 4562, 2934, 1944, 1954,
    1963, 1971, 1978, 76, 172, 2523, 2644, 184, 2524, 4646, 2645, 2754, 2755, 196, 2525, 4647,
    2646, 4656, 4701, 2756, 2853, 2854, 2855, 208, 2526, 4648, 2647, 4657, 4702, 2757, 4665, 4710,
    4746, 2856, 2941, 2942, 2943, 2944, 220, 2527, 4649, 2648, 4658, 4703, 2758, 4666, 4711, 4747,
    2857, 4673, 4718, 4754, 4782, 2945, 3018, 3019, 3020, 3021, 3022, 232, 239, 2006, 240, 2007,
    2017, 241, 2008, 2018, 2027, 242, 2009, 2019, 2028, 2036, 243, 2010, 2020, 2029, 2037, 2044,
    244, 83, 84, 85, 86, 87, 88, 17, 1615, 2473, 1681, 29, 1625, 3330, 3550, 1691, 2484, 3770,
    2605, 1747, 1757, 41, 1634, 3339, 3559, 1700, 3384, 6188, 3604, 3779, 3824, 1766, 2495, 3990,
    2616, 3999, 4044, 2726, 1813, 1823, 1832, 53, 1642, 3347, 3567, 1708, 3392, 6196, 3612, 3787,
    3832, 1774, 3428, 6232, 3648, 6352, 6680, 3868, 4007, 4052, 4088, 1840, 2506, 4210, 2627, 4219,
    4264, 2737, 4227, 4272, 4308, 2836, 1879, 1889, 1898, 1906, 65, 1649, 3354, 3574, 1715, 3399,
    6203, 3619, 3794, 3839, 1781, 3435, 6239, 3655, 6359, 6687, 3875, 4014, 4059, 4095, 1847, 3463,
    6267, 3683, 6387, 6715, 3903, 6471, 6799, 7008, 4123, 4234, 4279, 4315, 4343, 1913, 2517, 4430,
    2638, 4439, 4484, 2748, 4447, 4492, 4528, 2847, 4454, 4499, 4535, 4563, 2935, 1945, 1955, 1964,
    1972, 1979, 77, 1655, 3360, 3580, 1721, 3405, 6209, 3625, 3800, 3845, 1787, 3441, 6245, 3661,
    6365, 6693, 3881, 4020, 4065, 4101, 1853, 3469, 6273, 3689, 6393, 6721, 3909, 6477, 6805, 7014,
    4129, 4240, 4285, 4321, 4349, 1919, 3490, 6294, 3710, 6414, 6742, 3930, 6498, 6826, 7035, 4150,
    6554, 6882, 7091, 1603, 4370, 4460, 4505, 4541, 4569, 4590, 1985, 2528, 4650, 2649, 4659, 4704,
    2759, 4667, 4712, 4748, 2858, 4674, 4719, 4755, 4783, 2946, 4680, 4725, 4761, 4789, 4810, 3023,
    2011, 2021, 2030, 2038, 2045, 2051, 89, 173, 2534, 2655, 185, 2535, 4866, 2656, 2765, 2766,
    197, 2536, 4867, 2657, 4876, 4921, 2767, 2864, 2865, 2866, 209, 2537, 4868, 2658, 4877, 4922,
    2768, 4885, 4930, 4966, 2867, 2952, 2953, 2954, 2955, 221, 2538, 4869, 2659, 4878, 4923, 2769,
    4886, 4931, 4967, 2868, 4893, 4938, 4974, 5002, 2956, 3029, 3030, 3031, 3032, 3033, 233, 2539,
    4870, 2660, 4879, 4924, 2770, 4887, 4932, 4968, 2869, 4894, 4939, 4975, 5003, 2957, 4900, 4945,
    4981, 5009, 5030, 3034, 3095, 3096, 3097, 3098, 3099, 3100, 245, 251, 2072, 252, 2073, 2083,
    253, 2074, 2084, 2093, 254, 2075, 2085, 2094, 2102, 255, 2076, 2086, 2095, 2103, 2110, 256,
    2077, 2087, 2096, 2104, 2111, 2117, 257, 95, 96, 97, 98, 99, 100, 101, 18, 1616, 2474, 1682,
    30, 1626, 3331, 3551, 1692, 2485, 3771, 2606, 1748, 1758, 42, 1635, 3340, 3560, 1701, 3385,
    6189, 3605, 3780, 3825, 1767, 2496, 3991, 2617, 4000, 4045, 2727, 1814, 1824, 1833, 54, 1643,
    3348, 3568, 1709, 3393, 6197, 3613, 3788, 3833, 1775, 3429, 6233, 3649, 6353, 6681, 3869, 4008,
    4053, 4089, 1841, 2507, 4211, 2628, 4220, 4265, 2738, 4228, 4273, 4309, 2837, 1880, 1890, 1899,
    1907, 66, 1650, 3355, 3575, 1716, 3400, 6204, 3620, 3795, 3840, 1782, 3436, 6240, 3656, 6360,
    6688, 3876, 4015, 4060, 4096, 1848, 3464, 6268, 3684, 6388, 6716, 3904, 6472, 6800, 7009, 4124,
    4235, 4280, 4316, 4344, 1914, 2518, 4431, 2639, 4440, 4485, 2749, 4448, 4493, 4529, 2848, 4455,
    4500, 4536, 4564, 2936, 1946, 1956, 1965, 1973, 1980, 78, 1656, 3361, 3581, 1722, 3406, 6210,
    3626, 3801, 3846, 1788, 3442, 6246, 3662, 6366, 6694, 3882, 4021, 4066, 4102, 1854, 3470, 6274,
    3690, 6394, 6722, 3910, 6478, 6806, 7015, 4130, 4241, 4286, 4322, 4350, 1920, 3491, 6295, 3711,
    6415, 6743, 3931, 6499, 6827, 7036, 4151, 6555, 6883, 7092, 7217, 4371, 4461, 4506, 4542, 4570,
    4591, 1986, 2529, 4651, 2650, 4660, 4705, 2760, 4668, 4713, 4749, 2859, 4675, 4720, 4756, 4784,
    2947, 4681, 4726, 4762, 4790, 4811, 3024, 2012, 2022, 2031, 2039, 2046, 2052, 90, 1661, 3366,
    3586, 1727, 3411, 6215, 3631, 3806, 3851, 1793, 3447, 6251, 3667, 6371, 6699, 3887, 4026, 4071,
    4107, 1859, 3475, 6279, 3695, 6399, 6727, 3915, 6483, 6811, 7020, 4135, 4246, 4291, 4327, 4355,
    1925, 3496, 6300, 3716, 6420, 6748, 3936, 6504, 6832, 7041, 4156, 6560, 6888, 7097, 7222, 4376,
    4466, 4511, 4547, 4575, 4596, 1991, 3511, 6315, 3731, 6435, 6763, 3951, 6519, 6847, 7056, 4171,
    6575, 6903, 7112, 7237, 4391, 1605, 1605, 1605, 1605, 1604, 1605, 4686, 4731, 4767, 4795, 4816,
    1605, 2057, 2540, 4871, 2661, 4880, 4925, 2771, 4888, 4933, 4969, 2870, 4895, 4940, 4976, 5004,
    2958, 4901, 4946, 4982, 5010, 5031, 3035, 4906, 4951, 4987, 5015, 5036, 1605, 3101, 2078, 2088,
    2097, 2105, 2112, 2118, 2123, 102, 174, 2545, 2666, 186, 2546, 5086, 2667, 2776, 2777, 198,
    2547, 5087, 2668, 5096, 5141, 2778, 2875, 2876, 2877, 210, 2548, 5088, 2669, 5097, 5142, 2779,
    5105, 5150, 5186, 2878, 2963, 2964, 2965, 2966, 222, 2549, 5089, 2670, 5098, 5143, 2780, 5106,
    5151, 5187, 2879, 5113, 5158, 5194, 5222, 2967, 3040, 3041, 3042, 3043, 3044, 234, 2550, 5090,
    2671, 5099, 5144, 2781, 5107, 5152, 5188, 2880, 5114, 5159, 5195, 5223, 2968, 5120, 5165, 5201,
    5229, 5250, 3045, 3106, 3107, 3108, 3109, 3110, 3111, 246, 2551, 5091, 2672, 5100, 5145, 2782,
    5108, 5153, 5189, 2881, 5115, 5160, 5196, 5224, 2969, 5121, 5166, 5202, 5230, 5251, 3046, 5126,
    5171, 5207, 5235, 5256, 1605, 3112, 3161, 3162, 3163, 3164, 3165, 3166, 3167, 258, 263, 2138,
    264, 2139, 2149, 265, 2140, 2150, 2159, 266, 2141, 2151, 2160, 2168, 267, 2142, 2152, 2161,
    2169, 2176, 268, 2143, 2153, 2162, 2170, 2177, 2183, 269, 2144, 2154, 2163, 2171, 2178, 2184,
    2189, 270, 107, 108, 109, 110, 111, 112, 113, 114, 19, 175, 2475, 187, 31, 175, 2556, 2677,
    187, 2486, 2787, 2607, 199, 199, 43, 175, 2556, 2677, 187, 2557, 5306, 2678, 2787, 2788, 199,
    2497, 2886, 2618, 2886, 2887, 2728, 211, 211, 211, 55, 175, 2556, 2677, 187, 2557, 5306, 2678,
    2787, 2788, 199, 2558, 5307, 2679, 5316, 5361, 2789, 2886, 2887, 2888, 211, 2508, 2974, 2629,
    2974, 2975, 2739, 2974, 2975, 2976, 2838, 223, 223, 223, 223, 67, 175, 2556, 2677, 187, 2557,
    5306, 2678, 2787, 2788, 199, 2558, 5307, 2679, 5316, 5361, 2789, 2886, 2887, 2888, 211, 2559,
    5308, 2680, 5317, 5362, 2790, 5325, 5370, 5406, 2889, 2974, 2975, 2976, 2977, 223, 2519, 3051,
    2640, 3051, 3052, 2750, 3051, 3052, 3053, 2849, 3051, 3052, 3053, 3054, 2937, 235, 235, 235,
    235, 235, 79, 175, 2556, 2677, 187, 2557, 5306, 2678, 2787, 2788, 199, 2558, 5307, 2679, 5316,
    5361, 2789, 2886, 2887, 2888, 211, 2559, 5308, 2680, 5317, 5362, 2790, 5325, 5370, 5406, 2889,
    2974, 2975, 2976, 2977, 223, 2560, 5309, 2681, 5318, 5363, 2791, 5326, 5371, 5407, 2890, 5333,
    5378, 5414, 5442, 2978, 3051, 3052, 3053, 3054, 3055, 235, 2530, 3117, 2651, 3117, 3118, 2761,
    3117, 3118, 3119, 2860, 3117, 3118, 3119, 3120, 2948, 3117, 3118, 3119, 3120, 3121, 3025, 247,
    247, 247, 247, 247, 247, 91, 175, 2556, 2677, 187, 2557, 5306, 2678, 2787, 2788, 199, 2558,
    5307, 2679, 5316, 5361, 2789, 2886, 2887, 2888, 211, 2559, 5308, 2680, 5317, 5362, 2790, 5325,
    5370, 5406, 2889, 2974, 2975, 2976, 2977, 223, 2560, 5309, 2681, 5318, 5363, 2791, 5326, 5371,
    5407, 2890, 5333, 5378, 5414, 5442, 2978, 3051, 3052, 3053, 3054, 3055, 235, 2561, 5310, 2682,
    5319, 5364, 2792, 5327, 5372, 5408, 2891, 5334, 5379, 5415, 5443, 2979, 5340, 5385, 5421, 5449,
    5470, 3056, 3117, 3118, 3119, 3120, 3121, 3122, 247, 2541, 3172, 2662, 3172, 3173, 2772, 3172,
    3173, 3174, 2871, 3172, 3173, 3174, 3175, 2959, 3172, 3173, 3174, 3175, 3176, 3036, 3172, 3173,
    3174, 3175, 3176, 3177, 3102, 259, 259, 259, 259, 259, 259, 259, 103, 175, 2556, 2677, 187,
    2557, 5306, 2678, 2787, 2788, 199, 2558, 5307, 2679, 5316, 5361, 2789, 2886, 2887, 2888, 211,
    2559, 5308, 2680, 5317, 5362, 2790, 5325, 5370, 5406, 2889, 2974, 2975, 2976, 2977, 223, 2560,
    5309, 2681, 5318, 5363, 2791, 5326, 5371, 5407, 2890, 5333, 5378, 5414, 5442, 2978, 3051, 3052,
    3053, 3054, 3055, 235, 2561, 5310, 2682, 5319, 5364, 2792, 5327, 5372, 5408, 2891, 5334, 5379,
    5415, 5443, 2979, 5340, 5385, 5421, 5449, 5470, 3056, 3117, 3118, 3119, 3120, 3121, 3122, 247,
    2562, 5311, 2683, 5320, 5365, 2793, 5328, 5373, 5409, 2892, 5335, 5380, 5416, 5444, 2980, 5341,
    5386, 5422, 5450, 5471, 3057, 5346, 5391, 5427, 5455, 5476, 1605, 3123, 3172, 3173, 3174, 3175,
    3176, 3177, 3178, 259, 2552, 3216, 2673, 3216, 3217, 2783, 3216, 3217, 3218, 2882, 3216, 3217,
    3218, 3219, 2970, 3216, 3217, 3218, 3219, 3220, 3047, 3216, 3217, 3218, 3219, 3220, 3221, 3113,
    3216, 3217, 3218, 3219, 3220, 3221, 3222, 3168, 271, 271, 271, 271, 271, 271, 271, 271, 115,
    175, 275, 276, 187, 275, 2204, 276, 277, 277, 199, 275, 2204, 276, 2205, 2215, 277, 278, 278,
    278, 211, 275, 2204, 276, 2205, 2215, 277, 2206, 2216, 2225, 278, 279, 279, 279, 279, 223, 275,
    2204, 276, 2205, 2215, 277, 2206, 2216, 2225, 278, 2207, 2217, 2226, 2234, 279, 280, 280, 280,
    280, 280, 235, 275, 2204, 276, 2205, 2215, 277, 2206, 2216, 2225, 278, 2207, 2217, 2226, 2234,
    279, 2208, 2218, 2227, 2235, 2242, 280, 281, 281, 281, 281, 281, 281, 247, 275, 2204, 276,
    2205, 2215, 277, 2206, 2216, 2225, 278, 2207, 2217, 2226, 2234, 279, 2208, 2218, 2227, 2235,
    2242, 280, 2209, 2219, 2228, 2236, 2243, 2249, 281, 282, 282, 282, 282, 282, 282, 282, 259,
    275, 2204, 276, 2205, 2215, 277, 2206, 2216, 2225, 278, 2207, 2217, 2226, 2234, 279, 2208,
    2218, 2227, 2235, 2242, 280, 2209, 2219, 2228, 2236, 2243, 2249, 281, 2210, 2220, 2229, 2237,
    2244, 2250, 2255, 282, 283, 283, 283, 283, 283, 283, 283, 283, 271, 119, 119, 120, 119, 120,
    121, 119, 120, 121, 122, 119, 120, 121, 122, 123, 119, 120, 121, 122, 123, 124, 119, 120, 121,
    122, 123, 124, 125, 119, 120, 121, 122, 123, 124, 125, 126, 119, 120, 121, 122, 123, 124, 125,
    126, 127, 11, 167, 179, 23, 12, 1610, 2468, 1676, 24, 168, 2479, 2600, 180, 191, 1742, 192, 35,
    36, 13, 1611, 2469, 1677, 25, 1621, 3326, 3546, 1687, 2480, 3766, 2601, 1743, 1753, 37, 169,
    2490, 2611, 181, 2491, 3986, 2612, 2721, 2722, 193, 203, 1808, 204, 1809, 1819, 205, 47, 48,
    49, 14, 1612, 2470, 1678, 26, 1622, 3327, 3547, 1688, 2481, 3767, 2602, 1744, 1754, 38, 1631,
    3336, 3556, 1697, 3381, 1600, 3601, 3776, 3821, 1763, 2492, 3987, 2613, 3996, 4041, 2723, 1810,
    1820, 1829, 50, 170, 2501, 2622, 182, 2502, 4206, 2623, 2732, 2733, 194, 2503, 4207, 2624,
    4216, 4261, 2734, 2831, 2832, 2833, 206, 215, 1874, 216, 1875, 1885, 217, 1876, 1886, 1895,
    218, 59, 60, 61, 62, 15, 1613, 2471, 1679, 27, 1623, 3328, 3548, 1689, 2482, 3768, 2603, 1745,
    1755, 39, 1632, 3337, 3557, 1698, 3382, 6186, 3602, 3777, 3822, 1764, 2493, 3988, 2614, 3997,
    4042, 2724, 1811, 1821, 1830, 51, 1640, 3345, 3565, 1706, 3390, 6194, 3610, 3785, 3830, 1772,
    3426, 6230, 3646, 6350, 1601, 3866, 4005, 4050, 4086, 1838, 2504, 4208, 2625, 4217, 4262, 2735,
    4225, 4270, 4306, 2834, 1877, 1887, 1896, 1904, 63, 171, 2512, 2633, 183, 2513, 4426, 2634,
    2743, 2744, 195, 2514, 4427, 2635, 4436, 4481, 2745, 2842, 2843, 2844, 207, 2515, 4428, 2636,
    4437, 4482, 2746, 4445, 4490, 4526, 2845, 2930, 2931, 2932, 2933, 219, 227, 1940, 228, 1941,
    1951, 229, 1942, 1952, 1961, 230, 1943, 1953, 1962, 1970, 231, 71, 72, 73, 74, 75, 16, 1614,
    2472, 1680, 28, 1624, 3329, 3549, 1690, 2483, 3769, 2604, 1746, 1756, 40, 1633, 3338, 3558,
    1699, 3383, 6187, 3603, 3778, 3823, 1765, 2494, 3989, 2615, 3998, 4043, 2725, 1812, 1822, 1831,
    52, 1641, 3346, 3566, 1707, 3391, 6195, 3611, 3786, 3831, 1773, 3427, 6231, 3647, 6351, 6679,
    3867, 4006, 4051, 4087, 1839, 2505, 4209, 2626, 4218, 4263, 2736, 4226, 4271, 4307, 2835, 1878,
    1888, 1897, 1905, 64, 1648, 3353, 3573, 1714, 3398, 6202, 3618, 3793, 3838, 1780, 3434, 6238,
    3654, 6358, 6686, 3874, 4013, 4058, 4094, 1846, 3462, 6266, 3682, 6386, 6714, 3902, 6470, 6798,
    1602, 4122, 4233, 4278, 4314, 4342, 1912, 2516, 4429, 2637, 4438, 4483, 2747, 4446, 4491, 4527,
    2846, 4453, 4498, 4534, 4562, 2934, 1944, 1954, 1963, 1971, 1978, 76, 172, 2523, 2644, 184,
    2524, 4646, 2645, 2754, 2755, 196, 2525, 4647, 2646, 4656, 4701, 2756, 2853, 2854, 2855, 208,
    2526, 4648, 2647, 4657, 4702, 2757, 4665, 4710, 4746, 2856, 2941, 2942, 2943, 2944, 220, 2527,
    4649, 2648, 4658, 4703, 2758, 4666, 4711, 4747, 2857, 4673, 4718, 4754, 4782, 2945, 3018, 3019,
    3020, 3021, 3022, 232, 239, 2006, 240, 2007, 2017, 241, 2008, 2018, 2027, 242, 2009, 2019,
    2028, 2036, 243, 2010, 2020, 2029, 2037, 2044, 244, 83, 84, 85, 86, 87, 88, 17, 1615, 2473,
    1681, 29, 1625, 3330, 3550, 1691, 2484, 3770, 2605, 1747, 1757, 41, 1634, 3339, 3559, 1700,
    3384, 6188, 3604, 3779, 3824, 1766, 2495, 3990, 2616, 3999, 4044, 2726, 1813, 1823, 1832, 53,
    1642, 3347, 3567, 1708, 3392, 6196, 3612, 3787, 3832, 1774, 3428, 6232, 3648, 6352, 6680, 3868,
    4007, 4052, 4088, 1840, 2506, 4210, 2627, 4219, 4264, 2737, 4227, 4272, 4308, 2836, 1879, 1889,
    1898, 1906, 65, 1649, 3354, 3574, 1715, 3399, 6203, 3619, 3794, 3839, 1781, 3435, 6239, 3655,
    6359, 6687, 3875, 4014, 4059, 4095, 1847, 3463, 6267, 3683, 6387, 6715, 3903, 6471, 6799, 7008,
    4123, 4234, 4279, 4315, 4343, 1913, 2517, 4430, 2638, 4439, 4484, 2748, 4447, 4492, 4528, 2847,
    4454, 4499, 4535, 4563, 2935, 1945, 1955, 1964, 1972, 1979, 77, 1655, 3360, 3580, 1721, 3405,
    6209, 3625, 3800, 3845, 1787, 3441, 6245, 3661, 6365, 6693, 3881, 4020, 4065, 4101, 1853, 3469,
    6273, 3689, 6393, 6721, 3909, 6477, 6805, 7014, 4129, 4240, 4285, 4321, 4349, 1919, 3490, 6294,
    3710, 6414, 6742, 3930, 6498, 6826, 7035, 4150, 6554, 6882, 7091, 1603, 4370, 4460, 4505, 4541,
    4569, 4590, 1985, 2528, 4650, 2649, 4659, 4704, 2759, 4667, 4712, 4748, 2858, 4674, 4719, 4755,
    4783, 2946, 4680, 4725, 4761, 4789, 4810, 3023, 2011, 2021, 2030, 2038, 2045, 2051, 89, 173,
    2534, 2655, 185, 2535, 4866, 2656, 2765, 2766, 197, 2536, 4867, 2657, 4876, 4921, 2767, 2864,
    2865, 2866, 209, 2537, 4868, 2658, 4877, 4922, 2768, 4885, 4930, 4966, 2867, 2952, 2953, 2954,
    2955, 221, 2538, 4869, 2659, 4878, 4923, 2769, 4886, 4931, 4967, 2868, 4893, 4938, 4974, 5002,
    2956, 3029, 3030, 3031, 3032, 3033, 233, 2539, 4870, 2660, 4879, 4924, 2770, 4887, 4932, 4968,
    2869, 4894, 4939, 4975, 5003, 2957, 4900, 4945, 4981, 5009, 5030, 3034, 3095, 3096, 3097, 3098,
    3099, 3100, 245, 251, 2072, 252, 2073, 2083, 253, 2074, 2084, 2093, 254, 2075, 2085, 2094,
    2102, 255, 2076, 2086, 2095, 2103, 2110, 256, 2077, 2087, 2096, 2104, 2111, 2117, 257, 95, 96,
    97, 98, 99, 100, 101, 18, 1616, 2474, 1682, 30, 1626, 3331, 3551, 1692, 2485, 3771, 2606, 1748,
    1758, 42, 1635, 3340, 3560, 1701, 3385, 6189, 3605, 3780, 3825, 1767, 2496, 3991, 2617, 4000,
    4045, 2727, 1814, 1824, 1833, 54, 1643, 3348, 3568, 1709, 3393, 6197, 3613, 3788, 3833, 1775,
    3429, 6233, 3649, 6353, 6681, 3869, 4008, 4053, 4089, 1841, 2507, 4211, 2628, 4220, 4265, 2738,
    4228, 4273, 4309, 2837, 1880, 1890, 1899, 1907, 66, 1650, 3355, 3575, 1716, 3400, 6204, 3620,
    3795, 3840, 1782, 3436, 6240, 3656, 6360, 6688, 3876, 4015, 4060, 4096, 1848, 3464, 6268, 3684,
    6388, 6716, 3904, 6472, 6800, 7009, 4124, 4235, 4280, 4316, 4344, 1914, 2518, 4431, 2639, 4440,
    4485, 2749, 4448, 4493, 4529, 2848, 4455, 4500, 4536, 4564, 2936, 1946, 1956, 1965, 1973, 1980,
    78, 1656, 3361, 3581, 1722, 3406, 6210, 3626, 3801, 3846, 1788, 3442, 6246, 3662, 6366, 6694,
    3882, 4021, 4066, 4102, 1854, 3470, 6274, 3690, 6394, 6722, 3910, 6478, 6806, 7015, 4130, 4241,
    4286, 4322, 4350, 1920, 3491, 6295, 3711, 6415, 6743, 3931, 6499, 6827, 7036, 4151, 6555, 6883,
    7092, 7217, 4371, 4461, 4506, 4542, 4570, 4591, 1986, 2529, 4651, 2650, 4660, 4705, 2760, 4668,
    4713, 4749, 2859, 4675, 4720, 4756, 4784, 2947, 4681, 4726, 4762, 4790, 4811, 3024, 2012, 2022,
    2031, 2039, 2046, 2052, 90, 1661, 3366, 3586, 1727, 3411, 6215, 3631, 3806, 3851, 1793, 3447,
    6251, 3667, 6371, 6699, 3887, 4026, 4071, 4107, 1859, 3475, 6279, 3695, 6399, 6727, 3915, 6483,
    6811, 7020, 4135, 4246, 4291, 4327, 4355, 1925, 3496, 6300, 3716, 6420, 6748, 3936, 6504, 6832,
    7041, 4156, 6560, 6888, 7097, 7222, 4376, 4466, 4511, 4547, 4575, 4596, 1991, 3511, 6315, 3731,
    6435, 6763, 3951, 6519, 6847, 7056, 4171, 6575, 6903, 7112, 7237, 4391, 6610, 6938, 7147, 7272,
    1604, 4611, 4686, 4731, 4767, 4795, 4816, 4831, 2057, 2540, 4871, 2661, 4880, 4925, 2771, 4888,
    4933, 4969, 2870, 4895, 4940, 4976, 5004, 2958, 4901, 4946, 4982, 5010, 5031, 3035, 4906, 4951,
    4987, 5015, 5036, 5051, 3101, 2078, 2088, 2097, 2105, 2112, 2118, 2123, 102, 174, 2545, 2666,
    186, 2546, 5086, 2667, 2776, 2777, 198, 2547, 5087, 2668, 5096, 5141, 2778, 2875, 2876, 2877,
    210, 2548, 5088, 2669, 5097, 5142, 2779, 5105, 5150, 5186, 2878, 2963, 2964, 2965, 2966, 222,
    2549, 5089, 2670, 5098, 5143, 2780, 5106, 5151, 5187, 2879, 5113, 5158, 5194, 5222, 2967, 3040,
    3041, 3042, 3043, 3044, 234, 2550, 5090, 2671, 5099, 5144, 2781, 5107, 5152, 5188, 2880, 5114,
    5159, 5195, 5223, 2968, 5120, 5165, 5201, 5229, 5250, 3045, 3106, 3107, 3108, 3109, 3110, 3111,
    246, 2551, 5091, 2672, 5100, 5145, 2782, 5108, 5153, 5189, 2881, 5115, 5160, 5196, 5224, 2969,
    5121, 5166, 5202, 5230, 5251, 3046, 5126, 5171, 5207, 5235, 5256, 5271, 3112, 3161, 3162, 3163,
    3164, 3165, 3166, 3167, 258, 263, 2138, 264, 2139, 2149, 265, 2140, 2150, 2159, 266, 2141,
    2151, 2160, 2168, 267, 2142, 2152, 2161, 2169, 2176, 268, 2143, 2153, 2162, 2170, 2177, 2183,
    269, 2144, 2154, 2163, 2171, 2178, 2184, 2189, 270, 107, 108, 109, 110, 111, 112, 113, 114, 19,
    1617, 2475, 1683, 31, 1627, 3332, 3552, 1693, 2486, 3772, 2607, 1749, 1759, 43, 1636, 3341,
    3561, 1702, 3386, 6190, 3606, 3781, 3826, 1768, 2497, 3992, 2618, 4001, 4046, 2728, 1815, 1825,
    1834, 55, 1644, 3349, 3569, 1710, 3394, 6198, 3614, 3789, 3834, 1776, 3430, 6234, 3650, 6354,
    6682, 3870, 4009, 4054, 4090, 1842, 2508, 4212, 2629, 4221, 4266, 2739, 4229, 4274, 4310, 2838,
    1881, 1891, 1900, 1908, 67, 1651, 3356, 3576, 1717, 3401, 6205, 3621, 3796, 3841, 1783, 3437,
    6241, 3657, 6361, 6689, 3877, 4016, 4061, 4097, 1849, 3465, 6269, 3685, 6389, 6717, 3905, 6473,
    6801, 7010, 4125, 4236, 4281, 4317, 4345, 1915, 2519, 4432, 2640, 4441, 4486, 2750, 4449, 4494,
    4530, 2849, 4456, 4501, 4537, 4565, 2937, 1947, 1957, 1966, 1974, 1981, 79, 1657, 3362, 3582,
    1723, 3407, 6211, 3627, 3802, 3847, 1789, 3443, 6247, 3663, 6367, 6695, 3883, 4022, 4067, 4103,
    1855, 3471, 6275, 3691, 6395, 6723, 3911, 6479, 6807, 7016, 4131, 4242, 4287, 4323, 4351, 1921,
    3492, 6296, 3712, 6416, 6744, 3932, 6500, 6828, 7037, 4152, 6556, 6884, 7093, 7218, 4372, 4462,
    4507, 4543, 4571, 4592, 1987, 2530, 4652, 2651, 4661, 4706, 2761, 4669, 4714, 4750, 2860, 4676,
    4721, 4757, 4785, 2948, 4682, 4727, 4763, 4791, 4812, 3025, 2013, 2023, 2032, 2040, 2047, 2053,
    91, 1662, 3367, 3587, 1728, 3412, 6216, 3632, 3807, 3852, 1794, 3448, 6252, 3668, 6372, 6700,
    3888, 4027, 4072, 4108, 1860, 3476, 6280, 3696, 6400, 6728, 3916, 6484, 6812, 7021, 4136, 4247,
    4292, 4328, 4356, 1926, 3497, 6301, 3717, 6421, 6749, 3937, 6505, 6833, 7042, 4157, 6561, 6889,
    7098, 7223, 4377, 4467, 4512, 4548, 4576, 4597, 1992, 3512, 6316, 3732, 6436, 6764, 3952, 6520,
    6848, 7057, 4172, 6576, 6904, 7113, 7238, 4392, 6611, 6939, 7148, 7273, 7342, 4612, 4687, 4732,
    4768, 4796, 4817, 4832, 2058, 2541, 4872, 2662, 4881, 4926, 2772, 4889, 4934, 4970, 2871, 4896,
    4941, 4977, 5005, 2959, 4902, 4947, 4983, 5011, 5032, 3036, 4907, 4952, 4988, 5016, 5037, 5052,
    3102, 2079, 2089, 2098, 2106, 2113, 2119, 2124, 103, 1666, 3371, 3591, 1732, 3416, 6220, 3636,
    3811, 3856, 1798, 3452, 6256, 3672, 6376, 6704, 3892, 4031, 4076, 4112, 1864, 3480, 6284, 3700,
    6404, 6732, 3920, 6488, 6816, 7025, 4140, 4251, 4296, 4332, 4360, 1930, 3501, 6305, 3721, 6425,
    6753, 3941, 6509, 6837, 7046, 4161, 6565, 6893, 7102, 7227, 4381, 4471, 4516, 4552, 4580, 4601,
    1996, 3516, 6320, 3736, 6440, 6768, 3956, 6524, 6852, 7061, 4176, 6580, 6908, 7117, 7242, 4396,
    6615, 6943, 7152, 7277, 7346, 4616, 4691, 4736, 4772, 4800, 4821, 4836, 2062, 1607, 1607, 1607,
    1607, 1607, 1607, 1607, 1607, 1607, 1607, 1607, 1607, 1607, 1607, 1607, 1607, 1607, 1607, 1607,
    1607, 1607, 1606, 1606, 1606, 1606, 1606, 1605, 1606, 1607, 1607, 1607, 1607, 1607, 1607, 1606,
    1607, 2552, 5092, 2673, 5101, 5146, 2783, 5109, 5154, 5190, 2882, 5116, 5161, 5197, 5225, 2970,
    5122, 5167, 5203, 5231, 5252, 3047, 5127, 5172, 5208, 5236, 5257, 5272, 3113, 1607, 1607, 1607,
    1607, 1607, 1607, 1606, 1607, 2145, 2155, 2164, 2172, 2179, 2185, 2190, 1607, 115, 175, 2556,
    2677, 187, 2557, 5306, 2678, 2787, 2788, 199, 2558, 5307, 2679, 5316, 5361, 2789, 2886, 2887,
    2888, 211, 2559, 5308, 2680, 5317, 5362, 2790, 5325, 5370, 5406, 2889, 2974, 2975, 2976, 2977,
    223, 2560, 5309, 2681, 5318, 5363, 2791, 5326, 5371, 5407, 2890, 5333, 5378, 5414, 5442, 2978,
    3051, 3052, 3053, 3054, 3055, 235, 2561, 5310, 2682, 5319, 5364, 2792, 5327, 5372, 5408, 2891,
    5334, 5379, 5415, 5443, 2979, 5340, 5385, 5421, 5449, 5470, 3056, 3117, 3118, 3119, 3120, 3121,
    3122, 247, 2562, 5311, 2683, 5320, 5365, 2793, 5328, 5373, 5409, 2892, 5335, 5380, 5416, 5444,
    2980, 5341, 5386, 5422, 5450, 5471, 3057, 5346, 5391, 5427, 5455, 5476, 5491, 3123, 3172, 3173,
    3174, 3175, 3176, 3177, 3178, 259, 2563, 5312, 2684, 5321, 5366, 2794, 5329, 5374, 5410, 2893,
    5336, 5381, 5417, 5445, 2981, 5342, 5387, 5423, 5451, 5472, 3058, 5347, 5392, 5428, 5456, 5477,
    5492, 3124, 1607, 1607, 1607, 1607, 1607, 1607, 1606, 1607, 3216, 3217, 3218, 3219, 3220, 3221,
    3222, 1607, 271, 275, 2204, 276, 2205, 2215, 277, 2206, 2216, 2225, 278, 2207, 2217, 2226,
    2234, 279, 2208, 2218, 2227, 2235, 2242, 280, 2209, 2219, 2228, 2236, 2243, 2249, 281, 2210,
    2220, 2229, 2237, 2244, 2250, 2255, 282, 2211, 2221, 2230, 2238, 2245, 2251, 2256, 1607, 283,
    119, 120, 121, 122, 123, 124, 125, 126, 127, 20, 176, 2476, 188, 32, 176, 2567, 2688, 188,
    2487, 2798, 2608, 200, 200, 44, 176, 2567, 2688, 188, 2568, 5526, 2689, 2798, 2799, 200, 2498,
    2897, 2619, 2897, 2898, 2729, 212, 212, 212, 56, 176, 2567, 2688, 188, 2568, 5526, 2689, 2798,
    2799, 200, 2569, 5527, 2690, 5536, 5581, 2800, 2897, 2898, 2899, 212, 2509, 2985, 2630, 2985,
    2986, 2740, 2985, 2986, 2987, 2839, 224, 224, 224, 224, 68, 176, 2567, 2688, 188, 2568, 5526,
    2689, 2798, 2799, 200, 2569, 5527, 2690, 5536, 5581, 2800, 2897, 2898, 2899, 212, 2570, 5528,
    2691, 5537, 5582, 2801, 5545, 5590, 5626, 2900, 2985, 2986, 2987, 2988, 224, 2520, 3062, 2641,
    3062, 3063, 2751, 3062, 3063, 3064, 2850, 3062, 3063, 3064, 3065, 2938, 236, 236, 236, 236,
    236, 80, 176, 2567, 2688, 188, 2568, 5526, 2689, 2798, 2799, 200, 2569, 5527, 2690, 5536, 5581,
    2800, 2897, 2898, 2899, 212, 2570, 5528, 2691, 5537, 5582, 2801, 5545, 5590, 5626, 2900, 2985,
    2986, 2987, 2988, 224, 2571, 5529, 2692, 5538, 5583, 2802, 5546, 5591, 5627, 2901, 5553, 5598,
    5634, 5662, 2989, 3062, 3063, 3064, 3065, 3066, 236, 2531, 3128, 2652, 3128, 3129, 2762, 3128,
    3129, 3130, 2861, 3128, 3129, 3130, 3131, 2949, 3128, 3129, 3130, 3131, 3132, 3026, 248, 248,
    248, 248, 248, 248, 92, 176, 2567, 2688, 188, 2568, 5526, 2689, 2798, 2799, 200, 2569, 5527,
    2690, 5536, 5581, 2800, 2897, 2898, 2899, 212, 2570, 5528, 2691, 5537, 5582, 2801, 5545, 5590,
    5626, 2900, 2985, 2986, 2987, 2988, 224, 2571, 5529, 2692, 5538, 5583, 2802, 5546, 5591, 5627,
    2901, 5553, 5598, 5634, 5662, 2989, 3062, 3063, 3064, 3065, 3066, 236, 2572, 5530, 2693, 5539,
    5584, 2803, 5547, 5592, 5628, 2902, 5554, 5599, 5635, 5663, 2990, 5560, 5605, 5641, 5669, 5690,
    3067, 3128, 3129, 3130, 3131, 3132, 3133, 248, 2542, 3183, 2663, 3183, 3184, 2773, 3183, 3184,
    3185, 2872, 3183, 3184, 3185, 3186, 2960, 3183, 3184, 3185, 3186, 3187, 3037, 3183, 3184, 3185,
    3186, 3187, 3188, 3103, 260, 260, 260, 260, 260, 260, 260, 104, 176, 2567, 2688, 188, 2568,
    5526, 2689, 2798, 2799, 200, 2569, 5527, 2690, 5536, 5581, 2800, 2897, 2898, 2899, 212, 2570,
    5528, 2691, 5537, 5582, 2801, 5545, 5590, 5626, 2900, 2985, 2986, 2987, 2988, 224, 2571, 5529,
    2692, 5538, 5583, 2802, 5546, 5591, 5627, 2901, 5553, 5598, 5634, 5662, 2989, 3062, 3063, 3064,
    3065, 3066, 236, 2572, 5530, 2693, 5539, 5584, 2803, 5547, 5592, 5628, 2902, 5554, 5599, 5635,
    5663, 2990, 5560, 5605, 5641, 5669, 5690, 3067, 3128, 3129, 3130, 3131, 3132, 3133, 248, 2573,
    5531, 2694, 5540, 5585, 2804, 5548, 5593, 5629, 2903, 5555, 5600, 5636, 5664, 2991, 5561, 5606,
    5642, 5670, 5691, 3068, 5566, 5611, 5647, 5675, 5696, 5711, 3134, 3183, 3184, 3185, 3186, 3187,
    3188, 3189, 260, 2553, 3227, 2674, 3227, 3228, 2784, 3227, 3228, 3229, 2883, 3227, 3228, 3229,
    3230, 2971, 3227, 3228, 3229, 3230, 3231, 3048, 3227, 3228, 3229, 3230, 3231, 3232, 3114, 3227,
    3228, 3229, 3230, 3231, 3232, 3233, 3169, 272, 272, 272, 272, 272, 272, 272, 272, 116, 176,
    2567, 2688, 188, 2568, 5526, 2689, 2798, 2799, 200, 2569, 5527, 2690, 5536, 5581, 2800, 2897,
    2898, 2899, 212, 2570, 5528, 2691, 5537, 5582, 2801, 5545, 5590, 5626, 2900, 2985, 2986, 2987,
    2988, 224, 2571, 5529, 2692, 5538, 5583, 2802, 5546, 5591, 5627, 2901, 5553, 5598, 5634, 5662,
    2989, 3062, 3063, 3064, 3065, 3066, 236, 2572, 5530, 2693, 5539, 5584, 2803, 5547, 5592, 5628,
    2902, 5554, 5599, 5635, 5663, 2990, 5560, 5605, 5641, 5669, 5690, 3067, 3128, 3129, 3130, 3131,
    3132, 3133, 248, 2573, 5531, 2694, 5540, 5585, 2804, 5548, 5593, 5629, 2903, 5555, 5600, 5636,
    5664, 2991, 5561, 5606, 5642, 5670, 5691, 3068, 5566, 5611, 5647, 5675, 5696, 5711, 3134, 3183,
    3184, 3185, 3186, 3187, 3188, 3189, 260, 2574, 5532, 2695, 5541, 5586, 2805, 5549, 5594, 5630,
    2904, 5556, 5601, 5637, 5665, 2992, 5562, 5607, 5643, 5671, 5692, 3069, 5567, 5612, 5648, 5676,
    5697, 5712, 3135, 1607, 1607, 1607, 1607, 1607, 1607, 1606, 1607, 3227, 3228, 3229, 3230, 3231,
    3232, 3233, 1607, 272, 2564, 3260, 2685, 3260, 3261, 2795, 3260, 3261, 3262, 2894, 3260, 3261,
    3262, 3263, 2982, 3260, 3261, 3262, 3263, 3264, 3059, 3260, 3261, 3262, 3263, 3264, 3265, 3125,
    3260, 3261, 3262, 3263, 3264, 3265, 3266, 3180, 3260, 3261, 3262, 3263, 3264, 3265, 3266, 1607,
    3224, 284, 284, 284, 284, 284, 284, 284, 284, 284, 128, 176, 287, 288, 188, 287, 2270, 288,
    289, 289, 200, 287, 2270, 288, 2271, 2281, 289, 290, 290, 290, 212, 287, 2270, 288, 2271, 2281,
    289, 2272, 2282, 2291, 290, 291, 291, 291, 291, 224, 287, 2270, 288, 2271, 2281, 289, 2272,
    2282, 2291, 290, 2273, 2283, 2292, 2300, 291, 292, 292, 292, 292, 292, 236, 287, 2270, 288,
    2271, 2281, 289, 2272, 2282, 2291, 290, 2273, 2283, 2292, 2300, 291, 2274, 2284, 2293, 2301,
    2308, 292, 293, 293, 293, 293, 293, 293, 248, 287, 2270, 288, 2271, 2281, 289, 2272, 2282,
    2291, 290, 2273, 2283, 2292, 2300, 291, 2274, 2284, 2293, 2301, 2308, 292, 2275, 2285, 2294,
    2302, 2309, 2315, 293, 294, 294, 294, 294, 294, 294, 294, 260, 287, 2270, 288, 2271, 2281, 289,
    2272, 2282, 2291, 290, 2273, 2283, 2292, 2300, 291, 2274, 2284, 2293, 2301, 2308, 292, 2275,
    2285, 2294, 2302, 2309, 2315, 293, 2276, 2286, 2295, 2303, 2310, 2316, 2321, 294, 295, 295,
    295, 295, 295, 295, 295, 295, 272, 287, 2270, 288, 2271, 2281, 289, 2272, 2282, 2291, 290,
    2273, 2283, 2292, 2300, 291, 2274, 2284, 2293, 2301, 2308, 292, 2275, 2285, 2294, 2302, 2309,
    2315, 293, 2276, 2286, 2295, 2303, 2310, 2316, 2321, 294, 2277, 2287, 2296, 2304, 2311, 2317,
    2322, 1607, 295, 296, 296, 296, 296, 296, 296, 296, 296, 296, 284, 131, 131, 132, 131, 132,
    133, 131, 132, 133, 134, 131, 132, 133, 134, 135, 131, 132, 133, 134, 135, 136, 131, 132, 133,
    134, 135, 136, 137, 131, 132, 133, 134, 135, 136, 137, 138, 131, 132, 133, 134, 135, 136, 137,
    138, 139, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 11, 167, 179, 23, 12, 177, 2468,
    189, 24, 168, 2479, 2600, 180, 191, 201, 192, 35, 36, 13, 177, 2469, 189, 25, 177, 2578, 2699,
    189, 2480, 2809, 2601, 201, 201, 37, 169, 2490, 2611, 181, 2491, 2908, 2612, 2721, 2722, 193,
    203, 213, 204, 213, 213, 205, 47, 48, 49, 14, 177, 2470, 189, 26, 177, 2578, 2699, 189, 2481,
    2809, 2602, 201, 201, 38, 177, 2578, 2699, 189, 2579, 1600, 2700, 2809, 2810, 201, 2492, 2908,
    2613, 2908, 2909, 2723, 213, 213, 213, 50, 170, 2501, 2622, 182, 2502, 2996, 2623, 2732, 2733,
    194, 2503, 2996, 2624, 2996, 2997, 2734, 2831, 2832, 2833, 206, 215, 225, 216, 225, 225, 217,
    225, 225, 225, 218, 59, 60, 61, 62, 15, 177, 2471, 189, 27, 177, 2578, 2699, 189, 2482, 2809,
    2603, 201, 201, 39, 177, 2578, 2699, 189, 2579, 5746, 2700, 2809, 2810, 201, 2493, 2908, 2614,
    2908, 2909, 2724, 213, 213, 213, 51, 177, 2578, 2699, 189, 2579, 5746, 2700, 2809, 2810, 201,
    2580, 5747, 2701, 5756, 1601, 2811, 2908, 2909, 2910, 213, 2504, 2996, 2625, 2996, 2997, 2735,
    2996, 2997, 2998, 2834, 225, 225, 225, 225, 63, 171, 2512, 2633, 183, 2513, 3073, 2634, 2743,
    2744, 195, 2514, 3073, 2635, 3073, 3074, 2745, 2842, 2843, 2844, 207, 2515, 3073, 2636, 3073,
    3074, 2746, 3073, 3074, 3075, 2845, 2930, 2931, 2932, 2933, 219, 227, 237, 228, 237, 237, 229,
    237, 237, 237, 230, 237, 237, 237, 237, 231, 71, 72, 73, 74, 75, 16, 177, 2472, 189, 28, 177,
    2578, 2699, 189, 2483, 2809, 2604, 201, 201, 40, 177, 2578, 2699, 189, 2579, 5746, 2700, 2809,
    2810, 201, 2494, 2908, 2615, 2908, 2909, 2725, 213, 213, 213, 52, 177, 2578, 2699, 189, 2579,
    5746, 2700, 2809, 2810, 201, 2580, 5747, 2701, 5756, 5801, 2811, 2908, 2909, 2910, 213, 2505,
    2996, 2626, 2996, 2997, 2736, 2996, 2997, 2998, 2835, 225, 225, 225, 225, 64, 177, 2578, 2699,
    189, 2579, 5746, 2700, 2809, 2810, 201, 2580, 5747, 2701, 5756, 5801, 2811, 2908, 2909, 2910,
    213, 2581, 5748, 2702, 5757, 5802, 2812, 5765, 5810, 1602, 2911, 2996, 2997, 2998, 2999, 225,
    2516, 3073, 2637, 3073, 3074, 2747, 3073, 3074, 3075, 2846, 3073, 3074, 3075, 3076, 2934, 237,
    237, 237, 237, 237, 76, 172, 2523, 2644, 184, 2524, 3139, 2645, 2754, 2755, 196, 2525, 3139,
    2646, 3139, 3140, 2756, 2853, 2854, 2855, 208, 2526, 3139, 2647, 3139, 3140, 2757, 3139, 3140,
    3141, 2856, 2941, 2942, 2943, 2944, 220, 2527, 3139, 2648, 3139, 3140, 2758, 3139, 3140, 3141,
    2857, 3139, 3140, 3141, 3142, 2945, 3018, 3019, 3020, 3021, 3022, 232, 239, 249, 240, 249, 249,
    241, 249, 249, 249, 242, 249, 249, 249, 249, 243, 249, 249, 249, 249, 249, 244, 83, 84, 85, 86,
    87, 88, 17, 177, 2473, 189, 29, 177, 2578, 2699, 189, 2484, 2809, 2605, 201, 201, 41, 177,
    2578, 2699, 189, 2579, 5746, 2700, 2809, 2810, 201, 2495, 2908, 2616, 2908, 2909, 2726, 213,
    213, 213, 53, 177, 2578, 2699, 189, 2579, 5746, 2700, 2809, 2810, 201, 2580, 5747, 2701, 5756,
    5801, 2811, 2908, 2909, 2910, 213, 2506, 2996, 2627, 2996, 2997, 2737, 2996, 2997, 2998, 2836,
    225, 225, 225, 225, 65, 177, 2578, 2699, 189, 2579, 5746, 2700, 2809, 2810, 201, 2580, 5747,
    2701, 5756, 5801, 2811, 2908, 2909, 2910, 213, 2581, 5748, 2702, 5757, 5802, 2812, 5765, 5810,
    5846, 2911, 2996, 2997, 2998, 2999, 225, 2517, 3073, 2638, 3073, 3074, 2748, 3073, 3074, 3075,
    2847, 3073, 3074, 3075, 3076, 2935, 237, 237, 237, 237, 237, 77, 177, 2578, 2699, 189, 2579,
    5746, 2700, 2809, 2810, 201, 2580, 5747, 2701, 5756, 5801, 2811, 2908, 2909, 2910, 213, 2581,
    5748, 2702, 5757, 5802, 2812, 5765, 5810, 5846, 2911, 2996, 2997, 2998, 2999, 225, 2582, 5749,
    2703, 5758, 5803, 2813, 5766, 5811, 5847, 2912, 5773, 5818, 5854, 1603, 3000, 3073, 3074, 3075,
    3076, 3077, 237, 2528, 3139, 2649, 3139, 3140, 2759, 3139, 3140, 3141, 2858, 3139, 3140, 3141,
    3142, 2946, 3139, 3140, 3141, 3142, 3143, 3023, 249, 249, 249, 249, 249, 249, 89, 173, 2534,
    2655, 185, 2535, 3194, 2656, 2765, 2766, 197, 2536, 3194, 2657, 3194, 3195, 2767, 2864, 2865,
    2866, 209, 2537, 3194, 2658, 3194, 3195, 2768, 3194, 3195, 3196, 2867, 2952, 2953, 2954, 2955,
    221, 2538, 3194, 2659, 3194, 3195, 2769, 3194, 3195, 3196, 2868, 3194, 3195, 3196, 3197, 2956,
    3029, 3030, 3031, 3032, 3033, 233, 2539, 3194, 2660, 3194, 3195, 2770, 3194, 3195, 3196, 2869,
    3194, 3195, 3196, 3197, 2957, 3194, 3195, 3196, 3197, 3198, 3034, 3095, 3096, 3097, 3098, 3099,
    3100, 245, 251, 261, 252, 261, 261, 253, 261, 261, 261, 254, 261, 261, 261, 261, 255, 261, 261,
    261, 261, 261, 256, 261, 261, 261, 261, 261, 261, 257, 95, 96, 97, 98, 99, 100, 101, 18, 177,
    2474, 189, 30, 177, 2578, 2699, 189, 2485, 2809, 2606, 201, 201, 42, 177, 2578, 2699, 189,
    2579, 5746, 2700, 2809, 2810, 201, 2496, 2908, 2617, 2908, 2909, 2727, 213, 213, 213, 54, 177,
    2578, 2699, 189, 2579, 5746, 2700, 2809, 2810, 201, 2580, 5747, 2701, 5756, 5801, 2811, 2908,
    2909, 2910, 213, 2507, 2996, 2628, 2996, 2997, 2738, 2996, 2997, 2998, 2837, 225, 225, 225,
    225, 66, 177, 2578, 2699, 189, 2579, 5746, 2700, 2809, 2810, 201, 2580, 5747, 2701, 5756, 5801,
    2811, 2908, 2909, 2910, 213, 2581, 5748, 2702, 5757, 5802, 2812, 5765, 5810, 5846, 2911, 2996,
    2997, 2998, 2999, 225, 2518, 3073, 2639, 3073, 3074, 2749, 3073, 3074, 3075, 2848, 3073, 3074,
    3075, 3076, 2936, 237, 237, 237, 237, 237, 78, 177, 2578, 2699, 189, 2579, 5746, 2700, 2809,
    2810, 201, 2580, 5747, 2701, 5756, 5801, 2811, 2908, 2909, 2910, 213, 2581, 5748, 2702, 5757,
    5802, 2812, 5765, 5810, 5846, 2911, 2996, 2997, 2998, 2999, 225, 2582, 5749, 2703, 5758, 5803,
    2813, 5766, 5811, 5847, 2912, 5773, 5818, 5854, 5882, 3000, 3073, 3074, 3075, 3076, 3077, 237,
    2529, 3139, 2650, 3139, 3140, 2760, 3139, 3140, 3141, 2859, 3139, 3140, 3141, 3142, 2947, 3139,
    3140, 3141, 3142, 3143, 3024, 249, 249, 249, 249, 249, 249, 90, 177, 2578, 2699, 189, 2579,
    5746, 2700, 2809, 2810, 201, 2580, 5747, 2701, 5756, 5801, 2811, 2908, 2909, 2910, 213, 2581,
    5748, 2702, 5757, 5802, 2812, 5765, 5810, 5846, 2911, 2996, 2997, 2998, 2999, 225, 2582, 5749,
    2703, 5758, 5803, 2813, 5766, 5811, 5847, 2912, 5773, 5818, 5854, 5882, 3000, 3073, 3074, 3075,
    3076, 3077, 237, 2583, 5750, 2704, 5759, 5804, 2814, 5767, 5812, 5848, 2913, 5774, 5819, 5855,
    5883, 3001, 5780, 5825, 5861, 5889, 1604, 3078, 3139, 3140, 3141, 3142, 3143, 3144, 249, 2540,
    3194, 2661, 3194, 3195, 2771, 3194, 3195, 3196, 2870, 3194, 3195, 3196, 3197, 2958, 3194, 3195,
    3196, 3197, 3198, 3035, 3194, 3195, 3196, 3197, 3198, 3199, 3101, 261, 261, 261, 261, 261, 261,
    261, 102, 174, 2545, 2666, 186, 2546, 3238, 2667, 2776, 2777, 198, 2547, 3238, 2668, 3238,
    3239, 2778, 2875, 2876, 2877, 210, 2548, 3238, 2669, 3238, 3239, 2779, 3238, 3239, 3240, 2878,
    2963, 2964, 2965, 2966, 222, 2549, 3238, 2670, 3238, 3239, 2780, 3238, 3239, 3240, 2879, 3238,
    3239, 3240, 3241, 2967, 3040, 3041, 3042, 3043, 3044, 234, 2550, 3238, 2671, 3238, 3239, 2781,
    3238, 3239, 3240, 2880, 3238, 3239, 3240, 3241, 2968, 3238, 3239, 3240, 3241, 3242, 3045, 3106,
    3107, 3108, 3109, 3110, 3111, 246, 2551, 3238, 2672, 3238, 3239, 2782, 3238, 3239, 3240, 2881,
    3238, 3239, 3240, 3241, 2969, 3238, 3239, 3240, 3241, 3242, 3046, 3238, 3239, 3240, 3241, 3242,
    3243, 3112, 3161, 3162, 3163, 3164, 3165, 3166, 3167, 258, 263, 273, 264, 273, 273, 265, 273,
    273, 273, 266, 273, 273, 273, 273, 267, 273, 273, 273, 273, 273, 268, 273, 273, 273, 273, 273,
    273, 269, 273, 273, 273, 273, 273, 273, 273, 270, 107, 108, 109, 110, 111, 112, 113, 114, 19,
    177, 2475, 189, 31, 177, 2578, 2699, 189, 2486, 2809, 2607, 201, 201, 43, 177, 2578, 2699, 189,
    2579, 5746, 2700, 2809, 2810, 201, 2497, 2908, 2618, 2908, 2909, 2728, 213, 213, 213, 55, 177,
    2578, 2699, 189, 2579, 5746, 2700, 2809, 2810, 201, 2580, 5747, 2701, 5756, 5801, 2811, 2908,
    2909, 2910, 213, 2508, 2996, 2629, 2996, 2997, 2739, 2996, 2997, 2998, 2838, 225, 225, 225,
    225, 67, 177, 2578, 2699, 189, 2579, 5746, 2700, 2809, 2810, 201, 2580, 5747, 2701, 5756, 5801,
    2811, 2908, 2909, 2910, 213, 2581, 5748, 2702, 5757, 5802, 2812, 5765, 5810, 5846, 2911, 2996,
    2997, 2998, 2999, 225, 2519, 3073, 2640, 3073, 3074, 2750, 3073, 3074, 3075, 2849, 3073, 3074,
    3075, 3076, 2937, 237, 237, 237, 237, 237, 79, 177, 2578, 2699, 189, 2579, 5746, 2700, 2809,
    2810, 201, 2580, 5747, 2701, 5756, 5801, 2811, 2908, 2909, 2910, 213, 2581, 5748, 2702, 5757,
    5802, 2812, 5765, 5810, 5846, 2911, 2996, 2997, 2998, 2999, 225, 2582, 5749, 2703, 5758, 5803,
    2813, 5766, 5811, 5847, 2912, 5773, 5818, 5854, 5882, 3000, 3073, 3074, 3075, 3076, 3077, 237,
    2530, 3139, 2651, 3139, 3140, 2761, 3139, 3140, 3141, 2860, 3139, 3140, 3141, 3142, 2948, 3139,
    3140, 3141, 3142, 3143, 3025, 249, 249, 249, 249, 249, 249, 91, 177, 2578, 2699, 189, 2579,
    5746, 2700, 2809, 2810, 201, 2580, 5747, 2701, 5756, 5801, 2811, 2908, 2909, 2910, 213, 2581,
    5748, 2702, 5757, 5802, 2812, 5765, 5810, 5846, 2911, 2996, 2997, 2998, 2999, 225, 2582, 5749,
    2703, 5758, 5803, 2813, 5766, 5811, 5847, 2912, 5773, 5818, 5854, 5882, 3000, 3073, 3074, 3075,
    3076, 3077, 237, 2583, 5750, 2704, 5759, 5804, 2814, 5767, 5812, 5848, 2913, 5774, 5819, 5855,
    5883, 3001, 5780, 5825, 5861, 5889, 5910, 3078, 3139, 3140, 3141, 3142, 3143, 3144, 249, 2541,
    3194, 2662, 3194, 3195, 2772, 3194, 3195, 3196, 2871, 3194, 3195, 3196, 3197, 2959, 3194, 3195,
    3196, 3197, 3198, 3036, 3194, 3195, 3196, 3197, 3198, 3199, 3102, 261, 261, 261, 261, 261, 261,
    261, 103, 177, 2578, 2699, 189, 2579, 5746, 2700, 2809, 2810, 201, 2580, 5747, 2701, 5756,
    5801, 2811, 2908, 2909, 2910, 213, 2581, 5748, 2702, 5757, 5802, 2812, 5765, 5810, 5846, 2911,
    2996, 2997, 2998, 2999, 225, 2582, 5749, 2703, 5758, 5803, 2813, 5766, 5811, 5847, 2912, 5773,
    5818, 5854, 5882, 3000, 3073, 3074, 3075, 3076, 3077, 237, 2583, 5750, 2704, 5759, 5804, 2814,
    5767, 5812, 5848, 2913, 5774, 5819, 5855, 5883, 3001, 5780, 5825, 5861, 5889, 5910, 3078, 3139,
    3140, 3141, 3142, 3143, 3144, 249, 2584, 5751, 2705, 5760, 5805, 2815, 5768, 5813, 5849, 2914,
    5775, 5820, 5856, 5884, 3002, 5781, 5826, 5862, 5890, 5911, 3079, 5786, 5831, 5867, 5895, 5916,
    1605, 3145, 3194, 3195, 3196, 3197, 3198, 3199, 3200, 261, 2552, 3238, 2673, 3238, 3239, 2783,
    3238, 3239, 3240, 2882, 3238, 3239, 3240, 3241, 2970, 3238, 3239, 3240, 3241, 3242, 3047, 3238,
    3239, 3240, 3241, 3242, 3243, 3113, 3238, 3239, 3240, 3241, 3242, 3243, 3244, 3168, 273, 273,
    273, 273, 273, 273, 273, 273, 115, 175, 2556, 2677, 187, 2557, 3271, 2678, 2787, 2788, 199,
    2558, 3271, 2679, 3271, 3272, 2789, 2886, 2887, 2888, 211, 2559, 3271, 2680, 3271, 3272, 2790,
    3271, 3272, 3273, 2889, 2974, 2975, 2976, 2977, 223, 2560, 3271, 2681, 3271, 3272, 2791, 3271,
    3272, 3273, 2890, 3271, 3272, 3273, 3274, 2978, 3051, 3052, 3053, 3054, 3055, 235, 2561, 3271,
    2682, 3271, 3272, 2792, 3271, 3272, 3273, 2891, 3271, 3272, 3273, 3274, 2979, 3271, 3272, 3273,
    3274, 3275, 3056, 3117, 3118, 3119, 3120, 3121, 3122, 247, 2562, 3271, 2683, 3271, 3272, 2793,
    3271, 3272, 3273, 2892, 3271, 3272, 3273, 3274, 2980, 3271, 3272, 3273, 3274, 3275, 3057, 3271,
    3272, 3273, 3274, 3275, 3276, 3123, 3172, 3173, 3174, 3175, 3176, 3177, 3178, 259, 2563, 3271,
    2684, 3271, 3272, 2794, 3271, 3272, 3273, 2893, 3271, 3272, 3273, 3274, 2981, 3271, 3272, 3273,
    3274, 3275, 3058, 3271, 3272, 3273, 3274, 3275, 3276, 3124, 3271, 3272, 3273, 3274, 3275, 3276,
    3277, 3179, 3216, 3217, 3218, 3219, 3220, 3221, 3222, 3223, 271, 275, 285, 276, 285, 285, 277,
    285, 285, 285, 278, 285, 285, 285, 285, 279, 285, 285, 285, 285, 285, 280, 285, 285, 285, 285,
    285, 285, 281, 285, 285, 285, 285, 285, 285, 285, 282, 285, 285, 285, 285, 285, 285, 285, 285,
    283, 119, 120, 121, 122, 123, 124, 125, 126, 127, 20, 177, 2476, 189, 32, 177, 2578, 2699, 189,
    2487, 2809, 2608, 201, 201, 44, 177, 2578, 2699, 189, 2579, 5746, 2700, 2809, 2810, 201, 2498,
    2908, 2619, 2908, 2909, 2729, 213, 213, 213, 56, 177, 2578, 2699, 189, 2579, 5746, 2700, 2809,
    2810, 201, 2580, 5747, 2701, 5756, 5801, 2811, 2908, 2909, 2910, 213, 2509, 2996, 2630, 2996,
    2997, 2740, 2996, 2997, 2998, 2839, 225, 225, 225, 225, 68, 177, 2578, 2699, 189, 2579, 5746,
    2700, 2809, 2810, 201, 2580, 5747, 2701, 5756, 5801, 2811, 2908, 2909, 2910, 213, 2581, 5748,
    2702, 5757, 5802, 2812, 5765, 5810, 5846, 2911, 2996, 2997, 2998, 2999, 225, 2520, 3073, 2641,
    3073, 3074, 2751, 3073, 3074, 3075, 2850, 3073, 3074, 3075, 3076, 2938, 237, 237, 237, 237,
    237, 80, 177, 2578, 2699, 189, 2579, 5746, 2700, 2809, 2810, 201, 2580, 5747, 2701, 5756, 5801,
    2811, 2908, 2909, 2910, 213, 2581, 5748, 2702, 5757, 5802, 2812, 5765, 5810, 5846, 2911, 2996,
    2997, 2998, 2999, 225, 2582, 5749, 2703, 5758, 5803, 2813, 5766, 5811, 5847, 2912, 5773, 5818,
    5854, 5882, 3000, 3073, 3074, 3075, 3076, 3077, 237, 2531, 3139, 2652, 3139, 3140, 2762, 3139,
    3140, 3141, 2861, 3139, 3140, 3141, 3142, 2949, 3139, 3140, 3141, 3142, 3143, 3026, 249, 249,
    249, 249, 249, 249, 92, 177, 2578, 2699, 189, 2579, 5746, 2700, 2809, 2810, 201, 2580, 5747,
    2701, 5756, 5801, 2811, 2908, 2909, 2910, 213, 2581, 5748, 2702, 5757, 5802, 2812, 5765, 5810,
    5846, 2911, 2996, 2997, 2998, 2999, 225, 2582, 5749, 2703, 5758, 5803, 2813, 5766, 5811, 5847,
    2912, 5773, 5818, 5854, 5882, 3000, 3073, 3074, 3075, 3076, 3077, 237, 2583, 5750, 2704, 5759,
    5804, 2814, 5767, 5812, 5848, 2913, 5774, 5819, 5855, 5883, 3001, 5780, 5825, 5861, 5889, 5910,
    3078, 3139, 3140, 3141, 3142, 3143, 3144, 249, 2542, 3194, 2663, 3194, 3195, 2773, 3194, 3195,
    3196, 2872, 3194, 3195, 3196, 3197, 2960, 3194, 3195, 3196, 3197, 3198, 3037, 3194, 3195, 3196,
    3197, 3198, 3199, 3103, 261, 261, 261, 261, 261, 261, 261, 104, 177, 2578, 2699, 189, 2579,
    5746, 2700, 2809, 2810, 201, 2580, 5747, 2701, 5756, 5801, 2811, 2908, 2909, 2910, 213, 2581,
    5748, 2702, 5757, 5802, 2812, 5765, 5810, 5846, 2911, 2996, 2997, 2998, 2999, 225, 2582, 5749,
    2703, 5758, 5803, 2813, 5766, 5811, 5847, 2912, 5773, 5818, 5854, 5882, 3000, 3073, 3074, 3075,
    3076, 3077, 237, 2583, 5750, 2704, 5759, 5804, 2814, 5767, 5812, 5848, 2913, 5774, 5819, 5855,
    5883, 3001, 5780, 5825, 5861, 5889, 5910, 3078, 3139, 3140, 3141, 3142, 3143, 3144, 249, 2584,
    5751, 2705, 5760, 5805, 2815, 5768, 5813, 5849, 2914, 5775, 5820, 5856, 5884, 3002, 5781, 5826,
    5862, 5890, 5911, 3079, 5786, 5831, 5867, 5895, 5916, 5931, 3145, 3194, 3195, 3196, 3197, 3198,
    3199, 3200, 261, 2553, 3238, 2674, 3238, 3239, 2784, 3238, 3239, 3240, 2883, 3238, 3239, 3240,
    3241, 2971, 3238, 3239, 3240, 3241, 3242, 3048, 3238, 3239, 3240, 3241, 3242, 3243, 3114, 3238,
    3239, 3240, 3241, 3242, 3243, 3244, 3169, 273, 273, 273, 273, 273, 273, 273, 273, 116, 177,
    2578, 2699, 189, 2579, 5746, 2700, 2809, 2810, 201, 2580, 5747, 2701, 5756, 5801, 2811, 2908,
    2909, 2910, 213, 2581, 5748, 2702, 5757, 5802, 2812, 5765, 5810, 5846, 2911, 2996, 2997, 2998,
    2999, 225, 2582, 5749, 2703, 5758, 5803, 2813, 5766, 5811, 5847, 2912, 5773, 5818, 5854, 5882,
    3000, 3073, 3074, 3075, 3076, 3077, 237, 2583, 5750, 2704, 5759, 5804, 2814, 5767, 5812, 5848,
    2913, 5774, 5819, 5855, 5883, 3001, 5780, 5825, 5861, 5889, 5910, 3078, 3139, 3140, 3141, 3142,
    3143, 3144, 249, 2584, 5751, 2705, 5760, 5805, 2815, 5768, 5813, 5849, 2914, 5775, 5820, 5856,
    5884, 3002, 5781, 5826, 5862, 5890, 5911, 3079, 5786, 5831, 5867, 5895, 5916, 5931, 3145, 3194,
    3195, 3196, 3197, 3198, 3199, 3200, 261, 2585, 5752, 2706, 5761, 5806, 2816, 5769, 5814, 5850,
    2915, 5776, 5821, 5857, 5885, 3003, 5782, 5827, 5863, 5891, 5912, 3080, 5787, 5832, 5868, 5896,
    5917, 5932, 3146, 1607, 1607, 1607, 1607, 1607, 1607, 1606, 1607, 3238, 3239, 3240, 3241, 3242,
    3243, 3244, 1607, 273, 2564, 3271, 2685, 3271, 3272, 2795, 3271, 3272, 3273, 2894, 3271, 3272,
    3273, 3274, 2982, 3271, 3272, 3273, 3274, 3275, 3059, 3271, 3272, 3273, 3274, 3275, 3276, 3125,
    3271, 3272, 3273, 3274, 3275, 3276, 3277, 3180, 3271, 3272, 3273, 3274, 3275, 3276, 3277, 1607,
    3224, 285, 285, 285, 285, 285, 285, 285, 285, 285, 128, 176, 2567, 2688, 188, 2568, 3293, 2689,
    2798, 2799, 200, 2569, 3293, 2690, 3293, 3294, 2800, 2897, 2898, 2899, 212, 2570, 3293, 2691,
    3293, 3294, 2801, 3293, 3294, 3295, 2900, 2985, 2986, 2987, 2988, 224, 2571, 3293, 2692, 3293,
    3294, 2802, 3293, 3294, 3295, 2901, 3293, 3294, 3295, 3296, 2989, 3062, 3063, 3064, 3065, 3066,
    236, 2572, 3293, 2693, 3293, 3294, 2803, 3293, 3294, 3295, 2902, 3293, 3294, 3295, 3296, 2990,
    3293, 3294, 3295, 3296, 3297, 3067, 3128, 3129, 3130, 3131, 3132, 3133, 248, 2573, 3293, 2694,
    3293, 3294, 2804, 3293, 3294, 3295, 2903, 3293, 3294, 3295, 3296, 2991, 3293, 3294, 3295, 3296,
    3297, 3068, 3293, 3294, 3295, 3296, 3297, 3298, 3134, 3183, 3184, 3185, 3186, 3187, 3188, 3189,
    260, 2574, 3293, 2695, 3293, 3294, 2805, 3293, 3294, 3295, 2904, 3293, 3294, 3295, 3296, 2992,
    3293, 3294, 3295, 3296, 3297, 3069, 3293, 3294, 3295, 3296, 3297, 3298, 3135, 3293, 3294, 3295,
    3296, 3297, 3298, 3299, 3190, 3227, 3228, 3229, 3230, 3231, 3232, 3233, 3234, 272, 2575, 3293,
    2696, 3293, 3294, 2806, 3293, 3294, 3295, 2905, 3293, 3294, 3295, 3296, 2993, 3293, 3294, 3295,
    3296, 3297, 3070, 3293, 3294, 3295, 3296, 3297, 3298, 3136, 3293, 3294, 3295, 3296, 3297, 3298,
    3299, 3191, 3293, 3294, 3295, 3296, 3297, 3298, 3299, 1607, 3235, 3260, 3261, 3262, 3263, 3264,
    3265, 3266, 3267, 3268, 284, 287, 297, 288, 297, 297, 289, 297, 297, 297, 290, 297, 297, 297,
    297, 291, 297, 297, 297, 297, 297, 292, 297, 297, 297, 297, 297, 297, 293, 297, 297, 297, 297,
    297, 297, 297, 294, 297, 297, 297, 297, 297, 297, 297, 297, 295, 297, 297, 297, 297, 297, 297,
    297, 297, 297, 296, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 21, 177, 299, 189, 33,
    177, 299, 300, 189, 299, 301, 300, 201, 201, 45, 177, 299, 300, 189, 299, 2336, 300, 301, 301,
    201, 299, 302, 300, 302, 302, 301, 213, 213, 213, 57, 177, 299, 300, 189, 299, 2336, 300, 301,
    301, 201, 299, 2336, 300, 2337, 2347, 301, 302, 302, 302, 213, 299, 303, 300, 303, 303, 301,
    303, 303, 303, 302, 225, 225, 225, 225, 69, 177, 299, 300, 189, 299, 2336, 300, 301, 301, 201,
    299, 2336, 300, 2337, 2347, 301, 302, 302, 302, 213, 299, 2336, 300, 2337, 2347, 301, 2338,
    2348, 2357, 302, 303, 303, 303, 303, 225, 299, 304, 300, 304, 304, 301, 304, 304, 304, 302,
    304, 304, 304, 304, 303, 237, 237, 237, 237, 237, 81, 177, 299, 300, 189, 299, 2336, 300, 301,
    301, 201, 299, 2336, 300, 2337, 2347, 301, 302, 302, 302, 213, 299, 2336, 300, 2337, 2347, 301,
    2338, 2348, 2357, 302, 303, 303, 303, 303, 225, 299, 2336, 300, 2337, 2347, 301, 2338, 2348,
    2357, 302, 2339, 2349, 2358, 2366, 303, 304, 304, 304, 304, 304, 237, 299, 305, 300, 305, 305,
    301, 305, 305, 305, 302, 305, 305, 305, 305, 303, 305, 305, 305, 305, 305, 304, 249, 249, 249,
    249, 249, 249, 93, 177, 299, 300, 189, 299, 2336, 300, 301, 301, 201, 299, 2336, 300, 2337,
    2347, 301, 302, 302, 302, 213, 299, 2336, 300, 2337, 2347, 301, 2338, 2348, 2357, 302, 303,
    303, 303, 303, 225, 299, 2336, 300, 2337, 2347, 301, 2338, 2348, 2357, 302, 2339, 2349, 2358,
    2366, 303, 304, 304, 304, 304, 304, 237, 299, 2336, 300, 2337, 2347, 301, 2338, 2348, 2357,
    302, 2339, 2349, 2358, 2366, 303, 2340, 2350, 2359, 2367, 2374, 304, 305, 305, 305, 305, 305,
    305, 249, 299, 306, 300, 306, 306, 301, 306, 306, 306, 302, 306, 306, 306, 306, 303, 306, 306,
    306, 306, 306, 304, 306, 306, 306, 306, 306, 306, 305, 261, 261, 261, 261, 261, 261, 261, 105,
    177, 299, 300, 189, 299, 2336, 300, 301, 301, 201, 299, 2336, 300, 2337, 2347, 301, 302, 302,
    302, 213, 299, 2336, 300, 2337, 2347, 301, 2338, 2348, 2357, 302, 303, 303, 303, 303, 225, 299,
    2336, 300, 2337, 2347, 301, 2338, 2348, 2357, 302, 2339, 2349, 2358, 2366, 303, 304, 304, 304,
    304, 304, 237, 299, 2336, 300, 2337, 2347, 301, 2338, 2348, 2357, 302, 2339, 2349, 2358, 2366,
    303, 2340, 2350, 2359, 2367, 2374, 304, 305, 305, 305, 305, 305, 305, 249, 299, 2336, 300,
    2337, 2347, 301, 2338, 2348, 2357, 302, 2339, 2349, 2358, 2366, 303, 2340, 2350, 2359, 2367,
    2374, 304, 2341, 2351, 2360, 2368, 2375, 2381, 305, 306, 306, 306, 306, 306, 306, 306, 261,
    299, 307, 300, 307, 307, 301, 307, 307, 307, 302, 307, 307, 307, 307, 303, 307, 307, 307, 307,
    307, 304, 307, 307, 307, 307, 307, 307, 305, 307, 307, 307, 307, 307, 307, 307, 306, 273, 273,
    273, 273, 273, 273, 273, 273, 117, 177, 299, 300, 189, 299, 2336, 300, 301, 301, 201, 299,
    2336, 300, 2337, 2347, 301, 302, 302, 302, 213, 299, 2336, 300, 2337, 2347, 301, 2338, 2348,
    2357, 302, 303, 303, 303, 303, 225, 299, 2336, 300, 2337, 2347, 301, 2338, 2348, 2357, 302,
    2339, 2349, 2358, 2366, 303, 304, 304, 304, 304, 304, 237, 299, 2336, 300, 2337, 2347, 301,
    2338, 2348, 2357, 302, 2339, 2349, 2358, 2366, 303, 2340, 2350, 2359, 2367, 2374, 304, 305,
    305, 305, 305, 305, 305, 249, 299, 2336, 300, 2337, 2347, 301, 2338, 2348, 2357, 302, 2339,
    2349, 2358, 2366, 303, 2340, 2350, 2359, 2367, 2374, 304, 2341, 2351, 2360, 2368, 2375, 2381,
    305, 306, 306, 306, 306, 306, 306, 306, 261, 299, 2336, 300, 2337, 2347, 301, 2338, 2348, 2357,
    302, 2339, 2349, 2358, 2366, 303, 2340, 2350, 2359, 2367, 2374, 304, 2341, 2351, 2360, 2368,
    2375, 2381, 305, 2342, 2352, 2361, 2369, 2376, 2382, 2387, 306, 307, 307, 307, 307, 307, 307,
    307, 307, 273, 299, 308, 300, 308, 308, 301, 308, 308, 308, 302, 308, 308, 308, 308, 303, 308,
    308, 308, 308, 308, 304, 308, 308, 308, 308, 308, 308, 305, 308, 308, 308, 308, 308, 308, 308,
    306, 308, 308, 308, 308, 308, 308, 308, 308, 307, 285, 285, 285, 285, 285, 285, 285, 285, 285,
    129, 177, 299, 300, 189, 299, 2336, 300, 301, 301, 201, 299, 2336, 300, 2337, 2347, 301, 302,
    302, 302, 213, 299, 2336, 300, 2337, 2347, 301, 2338, 2348, 2357, 302, 303, 303, 303, 303, 225,
    299, 2336, 300, 2337, 2347, 301, 2338, 2348, 2357, 302, 2339, 2349, 2358, 2366, 303, 304, 304,
    304, 304, 304, 237, 299, 2336, 300, 2337, 2347, 301, 2338, 2348, 2357, 302, 2339, 2349, 2358,
    2366, 303, 2340, 2350, 2359, 2367, 2374, 304, 305, 305, 305, 305, 305, 305, 249, 299, 2336,
    300, 2337, 2347, 301, 2338, 2348, 2357, 302, 2339, 2349, 2358, 2366, 303, 2340, 2350, 2359,
    2367, 2374, 304, 2341, 2351, 2360, 2368, 2375, 2381, 305, 306, 306, 306, 306, 306, 306, 306,
    261, 299, 2336, 300, 2337, 2347, 301, 2338, 2348, 2357, 302, 2339, 2349, 2358, 2366, 303, 2340,
    2350, 2359, 2367, 2374, 304, 2341, 2351, 2360, 2368, 2375, 2381, 305, 2342, 2352, 2361, 2369,
    2376, 2382, 2387, 306, 307, 307, 307, 307, 307, 307, 307, 307, 273, 299, 2336, 300, 2337, 2347,
    301, 2338, 2348, 2357, 302, 2339, 2349, 2358, 2366, 303, 2340, 2350, 2359, 2367, 2374, 304,
    2341, 2351, 2360, 2368, 2375, 2381, 305, 2342, 2352, 2361, 2369, 2376, 2382, 2387, 306, 2343,
    2353, 2362, 2370, 2377, 2383, 2388, 1607, 307, 308, 308, 308, 308, 308, 308, 308, 308, 308,
    285, 299, 309, 300, 309, 309, 301, 309, 309, 309, 302, 309, 309, 309, 309, 303, 309, 309, 309,
    309, 309, 304, 309, 309, 309, 309, 309, 309, 305, 309, 309, 309, 309, 309, 309, 309, 306, 309,
    309, 309, 309, 309, 309, 309, 309, 307, 309, 309, 309, 309, 309, 309, 309, 309, 309, 308, 297,
    297, 297, 297, 297, 297, 297, 297, 297, 297, 141, 143, 143, 143, 144, 143, 143, 144, 143, 144,
    145, 143, 143, 144, 143, 144, 145, 143, 144, 145, 146, 143, 143, 144, 143, 144, 145, 143, 144,
    145, 146, 143, 144, 145, 146, 147, 143, 143, 144, 143, 144, 145, 143, 144, 145, 146, 143, 144,
    145, 146, 147, 143, 144, 145, 146, 147, 148, 143, 143, 144, 143, 144, 145, 143, 144, 145, 146,
    143, 144, 145, 146, 147, 143, 144, 145, 146, 147, 148, 143, 144, 145, 146, 147, 148, 149, 143,
    143, 144, 143, 144, 145, 143, 144, 145, 146, 143, 144, 145, 146, 147, 143, 144, 145, 146, 147,
    148, 143, 144, 145, 146, 147, 148, 149, 143, 144, 145, 146, 147, 148, 149, 150, 143, 143, 144,
    143, 144, 145, 143, 144, 145, 146, 143, 144, 145, 146, 147, 143, 144, 145, 146, 147, 148, 143,
    144, 145, 146, 147, 148, 149, 143, 144, 145, 146, 147, 148, 149, 150, 143, 144, 145, 146, 147,
    148, 149, 150, 151, 143, 143, 144, 143, 144, 145, 143, 144, 145, 146, 143, 144, 145, 146, 147,
    143, 144, 145, 146, 147, 148, 143, 144, 145, 146, 147, 148, 149, 143, 144, 145, 146, 147, 148,
    149, 150, 143, 144, 145, 146, 147, 148, 149, 150, 151, 143, 144, 145, 146, 147, 148, 149, 150,
    151, 152, 143, 143, 144, 143, 144, 145, 143, 144, 145, 146, 143, 144, 145, 146, 147, 143, 144,
    145, 146, 147, 148, 143, 144, 145, 146, 147, 148, 149, 143, 144, 145, 146, 147, 148, 149, 150,
    143, 144, 145, 146, 147, 148, 149, 150, 151, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152,
    143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 11, 167, 23, 11, 167, 179, 23, 12, 168,
    2468, 180, 24, 168, 191, 192, 180, 35, 35, 36, 11, 167, 179, 23, 12, 1610, 2468, 1676, 24, 168,
    2479, 2600, 180, 191, 1742, 192, 35, 36, 13, 169, 2469, 181, 25, 169, 2490, 2611, 181, 2480,
    2721, 2601, 193, 193, 37, 169, 203, 204, 181, 203, 1808, 204, 205, 205, 193, 47, 47, 48, 47,
    48, 49, 11, 167, 179, 23, 12, 1610, 2468, 1676, 24, 168, 2479, 2600, 180, 191, 1742, 192, 35,
    36, 13, 1611, 2469, 1677, 25, 1621, 1600, 1600, 1687, 2480, 1600, 2601, 1743, 1753, 37, 169,
    2490, 2611, 181, 2491, 1600, 2612, 2721, 2722, 193, 203, 1808, 204, 1809, 1819, 205, 47, 48,
    49, 14, 170, 2470, 182, 26, 170, 2501, 2622, 182, 2481, 2732, 2602, 194, 194, 38, 170, 2501,
    2622, 182, 2502, 1600, 2623, 2732, 2733, 194, 2492, 2831, 2613, 2831, 2832, 2723, 206, 206,
    206, 50, 170, 215, 216, 182, 215, 1874, 216, 217, 217, 194, 215, 1874, 216, 1875, 1885, 217,
    218, 218, 218, 206, 59, 59, 60, 59, 60, 61, 59, 60, 61, 62, 11, 167, 179, 23, 12, 1610, 2468,
    1676, 24, 168, 2479, 2600, 180, 191, 1742, 192, 35, 36, 13, 1611, 2469, 1677, 25, 1621, 3326,
    3546, 1687, 2480, 3766, 2601, 1743, 1753, 37, 169, 2490, 2611, 181, 2491, 3986, 2612, 2721,
    2722, 193, 203, 1808, 204, 1809, 1819, 205, 47, 48, 49, 14, 1612, 2470, 1678, 26, 1622, 3327,
    3547, 1688, 2481, 3767, 2602, 1744, 1754, 38, 1631, 3336, 3556, 1697, 3381, 1600, 1601, 3776,
    1601, 1763, 2492, 3987, 2613, 3996, 1601, 2723, 1810, 1820, 1829, 50, 170, 2501, 2622, 182,
    2502, 4206, 2623, 2732, 2733, 194, 2503, 4207, 2624, 4216, 1601, 2734, 2831, 2832, 2833, 206,
    215, 1874, 216, 1875, 1885, 217, 1876, 1886, 1895, 218, 59, 60, 61, 62, 15, 171, 2471, 183, 27,
    171, 2512, 2633, 183, 2482, 2743, 2603, 195, 195, 39, 171, 2512, 2633, 183, 2513, 4426, 2634,
    2743, 2744, 195, 2493, 2842, 2614, 2842, 2843, 2724, 207, 207, 207, 51, 171, 2512, 2633, 183,
    2513, 4426, 2634, 2743, 2744, 195, 2514, 4427, 2635, 4436, 1601, 2745, 2842, 2843, 2844, 207,
    2504, 2930, 2625, 2930, 2931, 2735, 2930, 2931, 2932, 2834, 219, 219, 219, 219, 63, 171, 227,
    228, 183, 227, 1940, 228, 229, 229, 195, 227, 1940, 228, 1941, 1951, 229, 230, 230, 230, 207,
    227, 1940, 228, 1941, 1951, 229, 1942, 1952, 1961, 230, 231, 231, 231, 231, 219, 71, 71, 72,
    71, 72, 73, 71, 72, 73, 74, 71, 72, 73, 74, 75, 11, 167, 179, 23, 12, 1610, 2468, 1676, 24,
    168, 2479, 2600, 180, 191, 1742, 192, 35, 36, 13, 1611, 2469, 1677, 25, 1621, 3326, 3546, 1687,
    2480, 3766, 2601, 1743, 1753, 37, 169, 2490, 2611, 181, 2491, 3986, 2612, 2721, 2722, 193, 203,
    1808, 204, 1809, 1819, 205, 47, 48, 49, 14, 1612, 2470, 1678, 26, 1622, 3327, 3547, 1688, 2481,
    3767, 2602, 1744, 1754, 38, 1631, 3336, 3556, 1697, 3381, 1600, 3601, 3776, 3821, 1763, 2492,
    3987, 2613, 3996, 4041, 2723, 1810, 1820, 1829, 50, 170, 2501, 2622, 182, 2502, 4206, 2623,
    2732, 2733, 194, 2503, 4207, 2624, 4216, 4261, 2734, 2831, 2832, 2833, 206, 215, 1874, 216,
    1875, 1885, 217, 1876, 1886, 1895, 218, 59, 60, 61, 62, 15, 1613, 2471, 1679, 27, 1623, 3328,
    3548, 1689, 2482, 3768, 2603, 1745, 1755, 39, 1632, 3337, 3557, 1698, 3382, 6186, 3602, 3777,
    3822, 1764, 2493, 3988, 2614, 3997, 4042, 2724, 1811, 1821, 1830, 51, 1640, 3345, 3565, 1706,
    3390, 6194, 3610, 3785, 3830, 1772, 3426, 6230, 3646, 1602, 1601, 1602, 4005, 4050, 1602, 1838,
    2504, 4208, 2625, 4217, 4262, 2735, 4225, 4270, 1602, 2834, 1877, 1887, 1896, 1904, 63, 171,
    2512, 2633, 183, 2513, 4426, 2634, 2743, 2744, 195, 2514, 4427, 2635, 4436, 4481, 2745, 2842,
    2843, 2844, 207, 2515, 4428, 2636, 4437, 4482, 2746, 4445, 4490, 1602, 2845, 2930, 2931, 2932,
    2933, 219, 227, 1940, 228, 1941, 1951, 229, 1942, 1952, 1961, 230, 1943, 1953, 1962, 1970, 231,
    71, 72, 73, 74, 75, 16, 172, 2472, 184, 28, 172, 2523, 2644, 184, 2483, 2754, 2604, 196, 196,
    40, 172, 2523, 2644, 184, 2524, 4646, 2645, 2754, 2755, 196, 2494, 2853, 2615, 2853, 2854,
    2725, 208, 208, 208, 52, 172, 2523, 2644, 184, 2524, 4646, 2645, 2754, 2755, 196, 2525, 4647,
    2646, 4656, 4701, 2756, 2853, 2854, 2855, 208, 2505, 2941, 2626, 2941, 2942, 2736, 2941, 2942,
    2943, 2835, 220, 220, 220, 220, 64, 172, 2523, 2644, 184, 2524, 4646, 2645, 2754, 2755, 196,
    2525, 4647, 2646, 4656, 4701, 2756, 2853, 2854, 2855, 208, 2526, 4648, 2647, 4657, 4702, 2757,
    4665, 4710, 1602, 2856, 2941, 2942, 2943, 2944, 220, 2516, 3018, 2637, 3018, 3019, 2747, 3018,
    3019, 3020, 2846, 3018, 3019, 3020, 3021, 2934, 232, 232, 232, 232, 232, 76, 172, 239, 240,
    184, 239, 2006, 240, 241, 241, 196, 239, 2006, 240, 2007, 2017, 241, 242, 242, 242, 208, 239,
    2006, 240, 2007, 2017, 241, 2008, 2018, 2027, 242, 243, 243, 243, 243, 220, 239, 2006, 240,
    2007, 2017, 241, 2008, 2018, 2027, 242, 2009, 2019, 2028, 2036, 243, 244, 244, 244, 244, 244,
    232, 83, 83, 84, 83, 84, 85, 83, 84, 85, 86, 83, 84, 85, 86, 87, 83, 84, 85, 86, 87, 88, 11,
    167, 179, 23, 12, 1610, 2468, 1676, 24, 168, 2479, 2600, 180, 191, 1742, 192, 35, 36, 13, 1611,
    2469, 1677, 25, 1621, 3326, 3546, 1687, 2480, 3766, 2601, 1743, 1753, 37, 169, 2490, 2611, 181,
    2491, 3986, 2612, 2721, 2722, 193, 203, 1808, 204, 1809, 1819, 205, 47, 48, 49, 14, 1612, 2470,
    1678, 26, 1622, 3327, 3547, 1688, 2481, 3767, 2602, 1744, 1754, 38, 1631, 3336, 3556, 1697,
    3381, 1600, 3601, 3776, 3821, 1763, 2492, 3987, 2613, 3996, 4041, 2723, 1810, 1820, 1829, 50,
    170, 2501, 2622, 182, 2502, 4206, 2623, 2732, 2733, 194, 2503, 4207, 2624, 4216, 4261, 2734,
    2831, 2832, 2833, 206, 215, 1874, 216, 1875, 1885, 217, 1876, 1886, 1895, 218, 59, 60, 61, 62,
    15, 1613, 2471, 1679, 27, 1623, 3328, 3548, 1689, 2482, 3768, 2603, 1745, 1755, 39, 1632, 3337,
    3557, 1698, 3382, 6186, 3602, 3777, 3822, 1764, 2493, 3988, 2614, 3997, 4042, 2724, 1811, 1821,
    1830, 51, 1640, 3345, 3565, 1706, 3390, 6194, 3610, 3785, 3830, 1772, 3426, 6230, 3646, 6350,
    1601, 3866, 4005, 4050, 4086, 1838, 2504, 4208, 2625, 4217, 4262, 2735, 4225, 4270, 4306, 2834,
    1877, 1887, 1896, 1904, 63, 171, 2512, 2633, 183, 2513, 4426, 2634, 2743, 2744, 195, 2514,
    4427, 2635, 4436, 4481, 2745, 2842, 2843, 2844, 207, 2515, 4428, 2636, 4437, 4482, 2746, 4445,
    4490, 4526, 2845, 2930, 2931, 2932, 2933, 219, 227, 1940, 228, 1941, 1951, 229, 1942, 1952,
    1961, 230, 1943, 1953, 1962, 1970, 231, 71, 72, 73, 74, 75, 16, 1614, 2472, 1680, 28, 1624,
    3329, 3549, 1690, 2483, 3769, 2604, 1746, 1756, 40, 1633, 3338, 3558, 1699, 3383, 6187, 3603,
    3778, 3823, 1765, 2494, 3989, 2615, 3998, 4043, 2725, 1812, 1822, 1831, 52, 1641, 3346, 3566,
    1707, 3391, 6195, 3611, 3786, 3831, 1773, 3427, 6231, 3647, 6351, 6679, 3867, 4006, 4051, 4087,
    1839, 2505, 4209, 2626, 4218, 4263, 2736, 4226, 4271, 4307, 2835, 1878, 1888, 1897, 1905, 64,
    1648, 3353, 3573, 1714, 3398, 6202, 3618, 3793, 3838, 1780, 3434, 6238, 3654, 6358, 6686, 3874,
    4013, 4058, 4094, 1846, 3462, 6266, 3682, 6386, 6714, 3902, 1603, 1603, 1602, 1603, 4233, 4278,
    4314, 1603, 1912, 2516, 4429, 2637, 4438, 4483, 2747, 4446, 4491, 4527, 2846, 4453, 4498, 4534,
    1603, 2934, 1944, 1954, 1963, 1971, 1978, 76, 172, 2523, 2644, 184, 2524, 4646, 2645, 2754,
    2755, 196, 2525, 4647, 2646, 4656, 4701, 2756, 2853, 2854, 2855, 208, 2526, 4648, 2647, 4657,
    4702, 2757, 4665, 4710, 4746, 2856, 2941, 2942, 2943, 2944, 220, 2527, 4649, 2648, 4658, 4703,
    2758, 4666, 4711, 4747, 2857, 4673, 4718, 4754, 1603, 2945, 3018, 3019, 3020, 3021, 3022, 232,
    239, 2006, 240, 2007, 2017, 241, 2008, 2018, 2027, 242, 2009, 2019, 2028, 2036, 243, 2010,
    2020, 2029, 2037, 2044, 244, 83, 84, 85, 86, 87, 88, 17, 173, 2473, 185, 29, 173, 2534, 2655,
    185, 2484, 2765, 2605, 197, 197, 41, 173, 2534, 2655, 185, 2535, 4866, 2656, 2765, 2766, 197,
    2495, 2864, 2616, 2864, 2865, 2726, 209, 209, 209, 53, 173, 2534, 2655, 185, 2535, 4866, 2656,
    2765, 2766, 197, 2536, 4867, 2657, 4876, 4921, 2767, 2864, 2865, 2866, 209, 2506, 2952, 2627,
    2952, 2953, 2737, 2952, 2953, 2954, 2836, 221, 221, 221, 221, 65, 173, 2534, 2655, 185, 2535,
    4866, 2656, 2765, 2766, 197, 2536, 4867, 2657, 4876, 4921, 2767, 2864, 2865, 2866, 209, 2537,
    4868, 2658, 4877, 4922, 2768, 4885, 4930, 4966, 2867, 2952, 2953, 2954, 2955, 221, 2517, 3029,
    2638, 3029, 3030, 2748, 3029, 3030, 3031, 2847, 3029, 3030, 3031, 3032, 2935, 233, 233, 233,
    233, 233, 77, 173, 2534, 2655, 185, 2535, 4866, 2656, 2765, 2766, 197, 2536, 4867, 2657, 4876,
    4921, 2767, 2864, 2865, 2866, 209, 2537, 4868, 2658, 4877, 4922, 2768, 4885, 4930, 4966, 2867,
    2952, 2953, 2954, 2955, 221, 2538, 4869, 2659, 4878, 4923, 2769, 4886, 4931, 4967, 2868, 4893,
    4938, 4974, 1603, 2956, 3029, 3030, 3031, 3032, 3033, 233, 2528, 3095, 2649, 3095, 3096, 2759,
    3095, 3096, 3097, 2858, 3095, 3096, 3097, 3098, 2946, 3095, 3096, 3097, 3098, 3099, 3023, 245,
    245, 245, 245, 245, 245, 89, 173, 251, 252, 185, 251, 2072, 252, 253, 253, 197, 251, 2072, 252,
    2073, 2083, 253, 254, 254, 254, 209, 251, 2072, 252, 2073, 2083, 253, 2074, 2084, 2093, 254,
    255, 255, 255, 255, 221, 251, 2072, 252, 2073, 2083, 253, 2074, 2084, 2093, 254, 2075, 2085,
    2094, 2102, 255, 256, 256, 256, 256, 256, 233, 251, 2072, 252, 2073, 2083, 253, 2074, 2084,
    2093, 254, 2075, 2085, 2094, 2102, 255, 2076, 2086, 2095, 2103, 2110, 256, 257, 257, 257, 257,
    257, 257, 245, 95, 95, 96, 95, 96, 97, 95, 96, 97, 98, 95, 96, 97, 98, 99, 95, 96, 97, 98, 99,
    100, 95, 96, 97, 98, 99, 100, 101, 11, 167, 179, 23, 12, 1610, 2468, 1676, 24, 168, 2479, 2600,
    180, 191, 1742, 192, 35, 36, 13, 1611, 2469, 1677, 25, 1621, 3326, 3546, 1687, 2480, 3766,
    2601, 1743, 1753, 37, 169, 2490, 2611, 181, 2491, 3986, 2612, 2721, 2722, 193, 203, 1808, 204,
    1809, 1819, 205, 47, 48, 49, 14, 1612, 2470, 1678, 26, 1622, 3327, 3547, 1688, 2481, 3767,
    2602, 1744, 1754, 38, 1631, 3336, 3556, 1697, 3381, 1600, 3601, 3776, 3821, 1763, 2492, 3987,
    2613, 3996, 4041, 2723, 1810, 1820, 1829, 50, 170, 2501, 2622, 182, 2502, 4206, 2623, 2732,
    2733, 194, 2503, 4207, 2624, 4216, 4261, 2734, 2831, 2832, 2833, 206, 215, 1874, 216, 1875,
    1885, 217, 1876, 1886, 1895, 218, 59, 60, 61, 62, 15, 1613, 2471, 1679, 27, 1623, 3328, 3548,
    1689, 2482, 3768, 2603, 1745, 1755, 39, 1632, 3337, 3557, 1698, 3382, 6186, 3602, 3777, 3822,
    1764, 2493, 3988, 2614, 3997, 4042, 2724, 1811, 1821, 1830, 51, 1640, 3345, 3565, 1706, 3390,
    6194, 3610, 3785, 3830, 1772, 3426, 6230, 3646, 6350, 1601, 3866, 4005, 4050, 4086, 1838, 2504,
    4208, 2625, 4217, 4262, 2735, 4225, 4270, 4306, 2834, 1877, 1887, 1896, 1904, 63, 171, 2512,
    2633, 183, 2513, 4426, 2634, 2743, 2744, 195, 2514, 4427, 2635, 4436, 4481, 2745, 2842, 2843,
    2844, 207, 2515, 4428, 2636, 4437, 4482, 2746, 4445, 4490, 4526, 2845, 2930, 2931, 2932, 2933,
    219, 227, 1940, 228, 1941, 1951, 229, 1942, 1952, 1961, 230, 1943, 1953, 1962, 1970, 231, 71,
    72, 73, 74, 75, 16, 1614, 2472, 1680, 28, 1624, 3329, 3549, 1690, 2483, 3769, 2604, 1746, 1756,
    40, 1633, 3338, 3558, 1699, 3383, 6187, 3603, 3778, 3823, 1765, 2494, 3989, 2615, 3998, 4043,
    2725, 1812, 1822, 1831, 52, 1641, 3346, 3566, 1707, 3391, 6195, 3611, 3786, 3831, 1773, 3427,
    6231, 3647, 6351, 6679, 3867, 4006, 4051, 4087, 1839, 2505, 4209, 2626, 4218, 4263, 2736, 4226,
    4271, 4307, 2835, 1878, 1888, 1897, 1905, 64, 1648, 3353, 3573, 1714, 3398, 6202, 3618, 3793,
    3838, 1780, 3434, 6238, 3654, 6358, 6686, 3874, 4013, 4058, 4094, 1846, 3462, 6266, 3682, 6386,
    6714, 3902, 6470, 6798, 1602, 4122, 4233, 4278, 4314, 4342, 1912, 2516, 4429, 2637, 4438, 4483,
    2747, 4446, 4491, 4527, 2846, 4453, 4498, 4534, 4562, 2934, 1944, 1954, 1963, 1971, 1978, 76,
    172, 2523, 2644, 184, 2524, 4646, 2645, 2754, 2755, 196, 2525, 4647, 2646, 4656, 4701, 2756,
    2853, 2854, 2855, 208, 2526, 4648, 2647, 4657, 4702, 2757, 4665, 4710, 4746, 2856, 2941, 2942,
    2943, 2944, 220, 2527, 4649, 2648, 4658, 4703, 2758, 4666, 4711, 4747, 2857, 4673, 4718, 4754,
    4782, 2945, 3018, 3019, 3020, 3021, 3022, 232, 239, 2006, 240, 2007, 2017, 241, 2008, 2018,
    2027, 242, 2009, 2019, 2028, 2036, 243, 2010, 2020, 2029, 2037, 2044, 244, 83, 84, 85, 86, 87,
    88, 17, 1615, 2473, 1681, 29, 1625, 3330, 3550, 1691, 2484, 3770, 2605, 1747, 1757, 41, 1634,
    3339, 3559, 1700, 3384, 6188, 3604, 3779, 3824, 1766, 2495, 3990, 2616, 3999, 4044, 2726, 1813,
    1823, 1832, 53, 1642, 3347, 3567, 1708, 3392, 6196, 3612, 3787, 3832, 1774, 3428, 6232, 3648,
    6352, 6680, 3868, 4007, 4052, 4088, 1840, 2506, 4210, 2627, 4219, 4264, 2737, 4227, 4272, 4308,
    2836, 1879, 1889, 1898, 1906, 65, 1649, 3354, 3574, 1715, 3399, 6203, 3619, 3794, 3839, 1781,
    3435, 6239, 3655, 6359, 6687, 3875, 4014, 4059, 4095, 1847, 3463, 6267, 3683, 6387, 6715, 3903,
    6471, 6799, 7008, 4123, 4234, 4279, 4315, 4343, 1913, 2517, 4430, 2638, 4439, 4484, 2748, 4447,
    4492, 4528, 2847, 4454, 4499, 4535, 4563, 2935, 1945, 1955, 1964, 1972, 1979, 77, 1655, 3360,
    3580, 1721, 3405, 6209, 3625, 3800, 3845, 1787, 3441, 6245, 3661, 6365, 6693, 3881, 4020, 4065,
    4101, 1853, 3469, 6273, 3689, 6393, 6721, 3909, 6477, 6805, 7014, 4129, 4240, 4285, 4321, 4349,
    1919, 3490, 6294, 3710, 6414, 6742, 3930, 6498, 6826, 7035, 4150, 1604, 1604, 1604, 1603, 1604,
    4460, 4505, 4541, 4569, 1604, 1985, 2528, 4650, 2649, 4659, 4704, 2759, 4667, 4712, 4748, 2858,
    4674, 4719, 4755, 4783, 2946, 4680, 4725, 4761, 4789, 1604, 3023, 2011, 2021, 2030, 2038, 2045,
    2051, 89, 173, 2534, 2655, 185, 2535, 4866, 2656, 2765, 2766, 197, 2536, 4867, 2657, 4876,
    4921, 2767, 2864, 2865, 2866, 209, 2537, 4868, 2658, 4877, 4922, 2768, 4885, 4930, 4966, 2867,
    2952, 2953, 2954, 2955, 221, 2538, 4869, 2659, 4878, 4923, 2769, 4886, 4931, 4967, 2868, 4893,
    4938, 4974, 5002, 2956, 3029, 3030, 3031, 3032, 3033, 233, 2539, 4870, 2660, 4879, 4924, 2770,
    4887, 4932, 4968, 2869, 4894, 4939, 4975, 5003, 2957, 4900, 4945, 4981, 5009, 1604, 3034, 3095,
    3096, 3097, 3098, 3099, 3100, 245, 251, 2072, 252, 2073, 2083, 253, 2074, 2084, 2093, 254,
    2075, 2085, 2094, 2102, 255, 2076, 2086, 2095, 2103, 2110, 256, 2077, 2087, 2096, 2104, 2111,
    2117, 257, 95, 96, 97, 98, 99, 100, 101, 18, 174, 2474, 186, 30, 174, 2545, 2666, 186, 2485,
    2776, 2606, 198, 198, 42, 174, 2545, 2666, 186, 2546, 5086, 2667, 2776, 2777, 198, 2496, 2875,
    2617, 2875, 2876, 2727, 210, 210, 210, 54, 174, 2545, 2666, 186, 2546, 5086, 2667, 2776, 2777,
    198, 2547, 5087, 2668, 5096, 5141, 2778, 2875, 2876, 2877, 210, 2507, 2963, 2628, 2963, 2964,
    2738, 2963, 2964, 2965, 2837, 222, 222, 222, 222, 66, 174, 2545, 2666, 186, 2546, 5086, 2667,
    2776, 2777, 198, 2547, 5087, 2668, 5096, 5141, 2778, 2875, 2876, 2877, 210, 2548, 5088, 2669,
    5097, 5142, 2779, 5105, 5150, 5186, 2878, 2963, 2964, 2965, 2966, 222, 2518, 3040, 2639, 3040,
    3041, 2749, 3040, 3041, 3042, 2848, 3040, 3041, 3042, 3043, 2936, 234, 234, 234, 234, 234, 78,
    174, 2545, 2666, 186, 2546, 5086, 2667, 2776, 2777, 198, 2547, 5087, 2668, 5096, 5141, 2778,
    2875, 2876, 2877, 210, 2548, 5088, 2669, 5097, 5142, 2779, 5105, 5150, 5186, 2878, 2963, 2964,
    2965, 2966, 222, 2549, 5089, 2670, 5098, 5143, 2780, 5106, 5151, 5187, 2879, 5113, 5158, 5194,
    5222, 2967, 3040, 3041, 3042, 3043, 3044, 234, 2529, 3106, 2650, 3106, 3107, 2760, 3106, 3107,
    3108, 2859, 3106, 3107, 3108, 3109, 2947, 3106, 3107, 3108, 3109, 3110, 3024, 246, 246, 246,
    246, 246, 246, 90, 174, 2545, 2666, 186, 2546, 5086, 2667, 2776, 2777, 198, 2547, 5087, 2668,
    5096, 5141, 2778, 2875, 2876, 2877, 210, 2548, 5088, 2669, 5097, 5142, 2779, 5105, 5150, 5186,
    2878, 2963, 2964, 2965, 2966, 222, 2549, 5089, 2670, 5098, 5143, 2780, 5106, 5151, 5187, 2879,
    5113, 5158, 5194, 5222, 2967, 3040, 3041, 3042, 3043, 3044, 234, 2550, 5090, 2671, 5099, 5144,
    2781, 5107, 5152, 5188, 2880, 5114, 5159, 5195, 5223, 2968, 5120, 5165, 5201, 5229, 1604, 3045,
    3106, 3107, 3108, 3109, 3110, 3111, 246, 2540, 3161, 2661, 3161, 3162, 2771, 3161, 3162, 3163,
    2870, 3161, 3162, 3163, 3164, 2958, 3161, 3162, 3163, 3164, 3165, 3035, 3161, 3162, 3163, 3164,
    3165, 3166, 3101, 258, 258, 258, 258, 258, 258, 258, 102, 174, 263, 264, 186, 263, 2138, 264,
    265, 265, 198, 263, 2138, 264, 2139, 2149, 265, 266, 266, 266, 210, 263, 2138, 264, 2139, 2149,
    265, 2140, 2150, 2159, 266, 267, 267, 267, 267, 222, 263, 2138, 264, 2139, 2149, 265, 2140,
    2150, 2159, 266, 2141, 2151, 2160, 2168, 267, 268, 268, 268, 268, 268, 234, 263, 2138, 264,
    2139, 2149, 265, 2140, 2150, 2159, 266, 2141, 2151, 2160, 2168, 267, 2142, 2152, 2161, 2169,
    2176, 268, 269, 269, 269, 269, 269, 269, 246, 263, 2138, 264, 2139, 2149, 265, 2140, 2150,
    2159, 266, 2141, 2151, 2160, 2168, 267, 2142, 2152, 2161, 2169, 2176, 268, 2143, 2153, 2162,
    2170, 2177, 2183, 269, 270, 270, 270, 270, 270, 270, 270, 258, 107, 107, 108, 107, 108, 109,
    107, 108, 109, 110, 107, 108, 109, 110, 111, 107, 108, 109, 110, 111, 112, 107, 108, 109, 110,
    111, 112, 113, 107, 108, 109, 110, 111, 112, 113, 114, 11, 167, 179, 23, 12, 1610, 2468, 1676,
    24, 168, 2479, 2600, 180, 191, 1742, 192, 35, 36, 13, 1611, 2469, 1677, 25, 1621, 3326, 3546,
    1687, 2480, 3766, 2601, 1743, 1753, 37, 169, 2490, 2611, 181, 2491, 3986, 2612, 2721, 2722,
    193, 203, 1808, 204, 1809, 1819, 205, 47, 48, 49, 14, 1612, 2470, 1678, 26, 1622, 3327, 3547,
    1688, 2481, 3767, 2602, 1744, 1754, 38, 1631, 3336, 3556, 1697, 3381, 1600, 3601, 3776, 3821,
    1763, 2492, 3987, 2613, 3996, 4041, 2723, 1810, 1820, 1829, 50, 170, 2501, 2622, 182, 2502,
    4206, 2623, 2732, 2733, 194, 2503, 4207, 2624, 4216, 4261, 2734, 2831, 2832, 2833, 206, 215,
    1874, 216, 1875, 1885, 217, 1876, 1886, 1895, 218, 59, 60, 61, 62, 15, 1613, 2471, 1679, 27,
    1623, 3328, 3548, 1689, 2482, 3768, 2603, 1745, 1755, 39, 1632, 3337, 3557, 1698, 3382, 6186,
    3602, 3777, 3822, 1764, 2493, 3988, 2614, 3997, 4042, 2724, 1811, 1821, 1830, 51, 1640, 3345,
    3565, 1706, 3390, 6194, 3610, 3785, 3830, 1772, 3426, 6230, 3646, 6350, 1601, 3866, 4005, 4050,
    4086, 1838, 2504, 4208, 2625, 4217, 4262, 2735, 4225, 4270, 4306, 2834, 1877, 1887, 1896, 1904,
    63, 171, 2512, 2633, 183, 2513, 4426, 2634, 2743, 2744, 195, 2514, 4427, 2635, 4436, 4481,
    2745, 2842, 2843, 2844, 207, 2515, 4428, 2636, 4437, 4482, 2746, 4445, 4490, 4526, 2845, 2930,
    2931, 2932, 2933, 219, 227, 1940, 228, 1941, 1951, 229, 1942, 1952, 1961, 230, 1943, 1953,
    1962, 1970, 231, 71, 72, 73, 74, 75, 16, 1614, 2472, 1680, 28, 1624, 3329, 3549, 1690, 2483,
    3769, 2604, 1746, 1756, 40, 1633, 3338, 3558, 1699, 3383, 6187, 3603, 3778, 3823, 1765, 2494,
    3989, 2615, 3998, 4043, 2725, 1812, 1822, 1831, 52, 1641, 3346, 3566, 1707, 3391, 6195, 3611,
    3786, 3831, 1773, 3427, 6231, 3647, 6351, 6679, 3867, 4006, 4051, 4087, 1839, 2505, 4209, 2626,
    4218, 4263, 2736, 4226, 4271, 4307, 2835, 1878, 1888, 1897, 1905, 64, 1648, 3353, 3573, 1714,
    3398, 6202, 3618, 3793, 3838, 1780, 3434, 6238, 3654, 6358, 6686, 3874, 4013, 4058, 4094, 1846,
    3462, 6266, 3682, 6386, 6714, 3902, 6470, 6798, 1602, 4122, 4233, 4278, 4314, 4342, 1912, 2516,
    4429, 2637, 4438, 4483, 2747, 4446, 4491, 4527, 2846, 4453, 4498, 4534, 4562, 2934, 1944, 1954,
    1963, 1971, 1978, 76, 172, 2523, 2644, 184, 2524, 4646, 2645, 2754, 2755, 196, 2525, 4647,
    2646, 4656, 4701, 2756, 2853, 2854, 2855, 208, 2526, 4648, 2647, 4657, 4702, 2757, 4665, 4710,
    4746, 2856, 2941, 2942, 2943, 2944, 220, 2527, 4649, 2648, 4658, 4703, 2758, 4666, 4711, 4747,
    2857, 4673, 4718, 4754, 4782, 2945, 3018, 3019, 3020, 3021, 3022, 232, 239, 2006, 240, 2007,
    2017, 241, 2008, 2018, 2027, 242, 2009, 2019, 2028, 2036, 243, 2010, 2020, 2029, 2037, 2044,
    244, 83, 84, 85, 86, 87, 88, 17, 1615, 2473, 1681, 29, 1625, 3330, 3550, 1691, 2484, 3770,
    2605, 1747, 1757, 41, 1634, 3339, 3559, 1700, 3384, 6188, 3604, 3779, 3824, 1766, 2495, 3990,
    2616, 3999, 4044, 2726, 1813, 1823, 1832, 53, 1642, 3347, 3567, 1708, 3392, 6196, 3612, 3787,
    3832, 1774, 3428, 6232, 3648, 6352, 6680, 3868, 4007, 4052, 4088, 1840, 2506, 4210, 2627, 4219,
    4264, 2737, 4227, 4272, 4308, 2836, 1879, 1889, 1898, 1906, 65, 1649, 3354, 3574, 1715, 3399,
    6203, 3619, 3794, 3839, 1781, 3435, 6239, 3655, 6359, 6687, 3875, 4014, 4059, 4095, 1847, 3463,
    6267, 3683, 6387, 6715, 3903, 6471, 6799, 7008, 4123, 4234, 4279, 4315, 4343, 1913, 2517, 4430,
    2638, 4439, 4484, 2748, 4447, 4492, 4528, 2847, 4454, 4499, 4535, 4563, 2935, 1945, 1955, 1964,
    1972, 1979, 77, 1655, 3360, 3580, 1721, 3405, 6209, 3625, 3800, 3845, 1787, 3441, 6245, 3661,
    6365, 6693, 3881, 4020, 4065, 4101, 1853, 3469, 6273, 3689, 6393, 6721, 3909, 6477, 6805, 7014,
    4129, 4240, 4285, 4321, 4349, 1919, 3490, 6294, 3710, 6414, 6742, 3930, 6498, 6826, 7035, 4150,
    6554, 6882, 7091, 1603, 4370, 4460, 4505, 4541, 4569, 4590, 1985, 2528, 4650, 2649, 4659, 4704,
    2759, 4667, 4712, 4748, 2858, 4674, 4719, 4755, 4783, 2946, 4680, 4725, 4761, 4789, 4810, 3023,
    2011, 2021, 2030, 2038, 2045, 2051, 89, 173, 2534, 2655, 185, 2535, 4866, 2656, 2765, 2766,
    197, 2536, 4867, 2657, 4876, 4921, 2767, 2864, 2865, 2866, 209, 2537, 4868, 2658, 4877, 4922,
    2768, 4885, 4930, 4966, 2867, 2952, 2953, 2954, 2955, 221, 2538, 4869, 2659, 4878, 4923, 2769,
    4886, 4931, 4967, 2868, 4893, 4938, 4974, 5002, 2956, 3029, 3030, 3031, 3032, 3033, 233, 2539,
    4870, 2660, 4879, 4924, 2770, 4887, 4932, 4968, 2869, 4894, 4939, 4975, 5003, 2957, 4900, 4945,
    4981, 5009, 5030, 3034, 3095, 3096, 3097, 3098, 3099, 3100, 245, 251, 2072, 252, 2073, 2083,
    253, 2074, 2084, 2093, 254, 2075, 2085, 2094, 2102, 255, 2076, 2086, 2095, 2103, 2110, 256,
    2077, 2087, 2096, 2104, 2111, 2117, 257, 95, 96, 97, 98, 99, 100, 101, 18, 1616, 2474, 1682,
    30, 1626, 3331, 3551, 1692, 2485, 3771, 2606, 1748, 1758, 42, 1635, 3340, 3560, 1701, 3385,
    6189, 3605, 3780, 3825, 1767, 2496, 3991, 2617, 4000, 4045, 2727, 1814, 1824, 1833, 54, 1643,
    3348, 3568, 1709, 3393, 6197, 3613, 3788, 3833, 1775, 3429, 6233, 3649, 6353, 6681, 3869, 4008,
    4053, 4089, 1841, 2507, 4211, 2628, 4220, 4265, 2738, 4228, 4273, 4309, 2837, 1880, 1890, 1899,
    1907, 66, 1650, 3355, 3575, 1716, 3400, 6204, 3620, 3795, 3840, 1782, 3436, 6240, 3656, 6360,
    6688, 3876, 4015, 4060, 4096, 1848, 3464, 6268, 3684, 6388, 6716, 3904, 6472, 6800, 7009, 4124,
    4235, 4280, 4316, 4344, 1914, 2518, 4431, 2639, 4440, 4485, 2749, 4448, 4493, 4529, 2848, 4455,
    4500, 4536, 4564, 2936, 1946, 1956, 1965, 1973, 1980, 78, 1656, 3361, 3581, 1722, 3406, 6210,
    3626, 3801, 3846, 1788, 3442, 6246, 3662, 6366, 6694, 3882, 4021, 4066, 4102, 1854, 3470, 6274,
    3690, 6394, 6722, 3910, 6478, 6806, 7015, 4130, 4241, 4286, 4322, 4350, 1920, 3491, 6295, 3711,
    6415, 6743, 3931, 6499, 6827, 7036, 4151, 6555, 6883, 7092, 7217, 4371, 4461, 4506, 4542, 4570,
    4591, 1986, 2529, 4651, 2650, 4660, 4705, 2760, 4668, 4713, 4749, 2859, 4675, 4720, 4756, 4784,
    2947, 4681, 4726, 4762, 4790, 4811, 3024, 2012, 2022, 2031, 2039, 2046, 2052, 90, 1661, 3366,
    3586, 1727, 3411, 6215, 3631, 3806, 3851, 1793, 3447, 6251, 3667, 6371, 6699, 3887, 4026, 4071,
    4107, 1859, 3475, 6279, 3695, 6399, 6727, 3915, 6483, 6811, 7020, 4135, 4246, 4291, 4327, 4355,
    1925, 3496, 6300, 3716, 6420, 6748, 3936, 6504, 6832, 7041, 4156, 6560, 6888, 7097, 7222, 4376,
    4466, 4511, 4547, 4575, 4596, 1991, 3511, 6315, 3731, 6435, 6763, 3951, 6519, 6847, 7056, 4171,
    6575, 6903, 7112, 7237, 4391, 1605, 1605, 1605, 1605, 1604, 1605, 4686, 4731, 4767, 4795, 4816,
    1605, 2057, 2540, 4871, 2661, 4880, 4925, 2771, 4888, 4933, 4969, 2870, 4895, 4940, 4976, 5004,
    2958, 4901, 4946, 4982, 5010, 5031, 3035, 4906, 4951, 4987, 5015, 5036, 1605, 3101, 2078, 2088,
    2097, 2105, 2112, 2118, 2123, 102, 174, 2545, 2666, 186, 2546, 5086, 2667, 2776, 2777, 198,
    2547, 5087, 2668, 5096, 5141, 2778, 2875, 2876, 2877, 210, 2548, 5088, 2669, 5097, 5142, 2779,
    5105, 5150, 5186, 2878, 2963, 2964, 2965, 2966, 222, 2549, 5089, 2670, 5098, 5143, 2780, 5106,
    5151, 5187, 2879, 5113, 5158, 5194, 5222, 2967, 3040, 3041, 3042, 3043, 3044, 234, 2550, 5090,
    2671, 5099, 5144, 2781, 5107, 5152, 5188, 2880, 5114, 5159, 5195, 5223, 2968, 5120, 5165, 5201,
    5229, 5250, 3045, 3106, 3107, 3108, 3109, 3110, 3111, 246, 2551, 5091, 2672, 5100, 5145, 2782,
    5108, 5153, 5189, 2881, 5115, 5160, 5196, 5224, 2969, 5121, 5166, 5202, 5230, 5251, 3046, 5126,
    5171, 5207, 5235, 5256, 1605, 3112, 3161, 3162, 3163, 3164, 3165, 3166, 3167, 258, 263, 2138,
    264, 2139, 2149, 265, 2140, 2150, 2159, 266, 2141, 2151, 2160, 2168, 267, 2142, 2152, 2161,
    2169, 2176, 268, 2143, 2153, 2162, 2170, 2177, 2183, 269, 2144, 2154, 2163, 2171, 2178, 2184,
    2189, 270, 107, 108, 109, 110, 111, 112, 113, 114, 19, 175, 2475, 187, 31, 175, 2556, 2677,
    187, 2486, 2787, 2607, 199, 199, 43, 175, 2556, 2677, 187, 2557, 5306, 2678, 2787, 2788, 199,
    2497, 2886, 2618, 2886, 2887, 2728, 211, 211, 211, 55, 175, 2556, 2677, 187, 2557, 5306, 2678,
    2787, 2788, 199, 2558, 5307, 2679, 5316, 5361, 2789, 2886, 2887, 2888, 211, 2508, 2974, 2629,
    2974, 2975, 2739, 2974, 2975, 2976, 2838, 223, 223, 223, 223, 67, 175, 2556, 2677, 187, 2557,
    5306, 2678, 2787, 2788, 199, 2558, 5307, 2679, 5316, 5361, 2789, 2886, 2887, 2888, 211, 2559,
    5308, 2680, 5317, 5362, 2790, 5325, 5370, 5406, 2889, 2974, 2975, 2976, 2977, 223, 2519, 3051,
    2640, 3051, 3052, 2750, 3051, 3052, 3053, 2849, 3051, 3052, 3053, 3054, 2937, 235, 235, 235,
    235, 235, 79, 175, 2556, 2677, 187, 2557, 5306, 2678, 2787, 2788, 199, 2558, 5307, 2679, 5316,
    5361, 2789, 2886, 2887, 2888, 211, 2559, 5308, 2680, 5317, 5362, 2790, 5325, 5370, 5406, 2889,
    2974, 2975, 2976, 2977, 223, 2560, 5309, 2681, 5318, 5363, 2791, 5326, 5371, 5407, 2890, 5333,
    5378, 5414, 5442, 2978, 3051, 3052, 3053, 3054, 3055, 235, 2530, 3117, 2651, 3117, 3118, 2761,
    3117, 3118, 3119, 2860, 3117, 3118, 3119, 3120, 2948, 3117, 3118, 3119, 3120, 3121, 3025, 247,
    247, 247, 247, 247, 247, 91, 175, 2556, 2677, 187, 2557, 5306, 2678, 2787, 2788, 199, 2558,
    5307, 2679, 5316, 5361, 2789, 2886, 2887, 2888, 211, 2559, 5308, 2680, 5317, 5362, 2790, 5325,
    5370, 5406, 2889, 2974, 2975, 2976, 2977, 223, 2560, 5309, 2681, 5318, 5363, 2791, 5326, 5371,
    5407, 2890, 5333, 5378, 5414, 5442, 2978, 3051, 3052, 3053, 3054, 3055, 235, 2561, 5310, 2682,
    5319, 5364, 2792, 5327, 5372, 5408, 2891, 5334, 5379, 5415, 5443, 2979, 5340, 5385, 5421, 5449,
    5470, 3056, 3117, 3118, 3119, 3120, 3121, 3122, 247, 2541, 3172, 2662, 3172, 3173, 2772, 3172,
    3173, 3174, 2871, 3172, 3173, 3174, 3175, 2959, 3172, 3173, 3174, 3175, 3176, 3036, 3172, 3173,
    3174, 3175, 3176, 3177, 3102, 259, 259, 259, 259, 259, 259, 259, 103, 175, 2556, 2677, 187,
    2557, 5306, 2678, 2787, 2788, 199, 2558, 5307, 2679, 5316, 5361, 2789, 2886, 2887, 2888, 211,
    2559, 5308, 2680, 5317, 5362, 2790, 5325, 5370, 5406, 2889, 2974, 2975, 2976, 2977, 223, 2560,
    5309, 2681, 5318, 5363, 2791, 5326, 5371, 5407, 2890, 5333, 5378, 5414, 5442, 2978, 3051, 3052,
    3053, 3054, 3055, 235, 2561, 5310, 2682, 5319, 5364, 2792, 5327, 5372, 5408, 2891, 5334, 5379,
    5415, 5443, 2979, 5340, 5385, 5421, 5449, 5470, 3056, 3117, 3118, 3119, 3120, 3121, 3122, 247,
    2562, 5311, 2683, 5320, 5365, 2793, 5328, 5373, 5409, 2892, 5335, 5380, 5416, 5444, 2980, 5341,
    5386, 5422, 5450, 5471, 3057, 5346, 5391, 5427, 5455, 5476, 1605, 3123, 3172, 3173, 3174, 3175,
    3176, 3177, 3178, 259, 2552, 3216, 2673, 3216, 3217, 2783, 3216, 3217, 3218, 2882, 3216, 3217,
    3218, 3219, 2970, 3216, 3217, 3218, 3219, 3220, 3047, 3216, 3217, 3218, 3219, 3220, 3221, 3113,
    3216, 3217, 3218, 3219, 3220, 3221, 3222, 3168, 271, 271, 271, 271, 271, 271, 271, 271, 115,
    175, 275, 276, 187, 275, 2204, 276, 277, 277, 199, 275, 2204, 276, 2205, 2215, 277, 278, 278,
    278, 211, 275, 2204, 276, 2205, 2215, 277, 2206, 2216, 2225, 278, 279, 279, 279, 279, 223, 275,
    2204, 276, 2205, 2215, 277, 2206, 2216, 2225, 278, 2207, 2217, 2226, 2234, 279, 280, 280, 280,
    280, 280, 235, 275, 2204, 276, 2205, 2215, 277, 2206, 2216, 2225, 278, 2207, 2217, 2226, 2234,
    279, 2208, 2218, 2227, 2235, 2242, 280, 281, 281, 281, 281, 281, 281, 247, 275, 2204, 276,
    2205, 2215, 277, 2206, 2216, 2225, 278, 2207, 2217, 2226, 2234, 279, 2208, 2218, 2227, 2235,
    2242, 280, 2209, 2219, 2228, 2236, 2243, 2249, 281, 282, 282, 282, 282, 282, 282, 282, 259,
    275, 2204, 276, 2205, 2215, 277, 2206, 2216, 2225, 278, 2207, 2217, 2226, 2234, 279, 2208,
    2218, 2227, 2235, 2242, 280, 2209, 2219, 2228, 2236, 2243, 2249, 281, 2210, 2220, 2229, 2237,
    2244, 2250, 2255, 282, 283, 283, 283, 283, 283, 283, 283, 283, 271, 119, 119, 120, 119, 120,
    121, 119, 120, 121, 122, 119, 120, 121, 122, 123, 119, 120, 121, 122, 123, 124, 119, 120, 121,
    122, 123, 124, 125, 119, 120, 121, 122, 123, 124, 125, 126, 119, 120, 121, 122, 123, 124, 125,
    126, 127, 11, 167, 179, 23, 12, 1610, 2468, 1676, 24, 168, 2479, 2600, 180, 191, 1742, 192, 35,
    36, 13, 1611, 2469, 1677, 25, 1621, 3326, 3546, 1687, 2480, 3766, 2601, 1743, 1753, 37, 169,
    2490, 2611, 181, 2491, 3986, 2612, 2721, 2722, 193, 203, 1808, 204, 1809, 1819, 205, 47, 48,
    49, 14, 1612, 2470, 1678, 26, 1622, 3327, 3547, 1688, 2481, 3767, 2602, 1744, 1754, 38, 1631,
    3336, 3556, 1697, 3381, 1600, 3601, 3776, 3821, 1763, 2492, 3987, 2613, 3996, 4041, 2723, 1810,
    1820, 1829, 50, 170, 2501, 2622, 182, 2502, 4206, 2623, 2732, 2733, 194, 2503, 4207, 2624,
    4216, 4261, 2734, 2831, 2832, 2833, 206, 215, 1874, 216, 1875, 1885, 217, 1876, 1886, 1895,
    218, 59, 60, 61, 62, 15, 1613, 2471, 1679, 27, 1623, 3328, 3548, 1689, 2482, 3768, 2603, 1745,
    1755, 39, 1632, 3337, 3557, 1698, 3382, 6186, 3602, 3777, 3822, 1764, 2493, 3988, 2614, 3997,
    4042, 2724, 1811, 1821, 1830, 51, 1640, 3345, 3565, 1706, 3390, 6194, 3610, 3785, 3830, 1772,
    3426, 6230, 3646, 6350, 1601, 3866, 4005, 4050, 4086, 1838, 2504, 4208, 2625, 4217, 4262, 2735,
    4225, 4270, 4306, 2834, 1877, 1887, 1896, 1904, 63, 171, 2512, 2633, 183, 2513, 4426, 2634,
    2743, 2744, 195, 2514, 4427, 2635, 4436, 4481, 2745, 2842, 2843, 2844, 207, 2515, 4428, 2636,
    4437, 4482, 2746, 4445, 4490, 4526, 2845, 2930, 2931, 2932, 2933, 219, 227, 1940, 228, 1941,
    1951, 229, 1942, 1952, 1961, 230, 1943, 1953, 1962, 1970, 231, 71, 72, 73, 74, 75, 16, 1614,
    2472, 1680, 28, 1624, 3329, 3549, 1690, 2483, 3769, 2604, 1746, 1756, 40, 1633, 3338, 3558,
    1699, 3383, 6187, 3603, 3778, 3823, 1765, 2494, 3989, 2615, 3998, 4043, 2725, 1812, 1822, 1831,
    52, 1641, 3346, 3566, 1707, 3391, 6195, 3611, 3786, 3831, 1773, 3427, 6231, 3647, 6351, 6679,
    3867, 4006, 4051, 4087, 1839, 2505, 4209, 2626, 4218, 4263, 2736, 4226, 4271, 4307, 2835, 1878,
    1888, 1897, 1905, 64, 1648, 3353, 3573, 1714, 3398, 6202, 3618, 3793, 3838, 1780, 3434, 6238,
    3654, 6358, 6686, 3874, 4013, 4058, 4094, 1846, 3462, 6266, 3682, 6386, 6714, 3902, 6470, 6798,
    1602, 4122, 4233, 4278, 4314, 4342, 1912, 2516, 4429, 2637, 4438, 4483, 2747, 4446, 4491, 4527,
    2846, 4453, 4498, 4534, 4562, 2934, 1944, 1954, 1963, 1971, 1978, 76, 172, 2523, 2644, 184,
    2524, 4646, 2645, 2754, 2755, 196, 2525, 4647, 2646, 4656, 4701, 2756, 2853, 2854, 2855, 208,
    2526, 4648, 2647, 4657, 4702, 2757, 4665, 4710, 4746, 2856, 2941, 2942, 2943, 2944, 220, 2527,
    4649, 2648, 4658, 4703, 2758, 4666, 4711, 4747, 2857, 4673, 4718, 4754, 4782, 2945, 3018, 3019,
    3020, 3021, 3022, 232, 239, 2006, 240, 2007, 2017, 241, 2008, 2018, 2027, 242, 2009, 2019,
    2028, 2036, 243, 2010, 2020, 2029, 2037, 2044, 244, 83, 84, 85, 86, 87, 88, 17, 1615, 2473,
    1681, 29, 1625, 3330, 3550, 1691, 2484, 3770, 2605, 1747, 1757, 41, 1634, 3339, 3559, 1700,
    3384, 6188, 3604, 3779, 3824, 1766, 2495, 3990, 2616, 3999, 4044, 2726, 1813, 1823, 1832, 53,
    1642, 3347, 3567, 1708, 3392, 6196, 3612, 3787, 3832, 1774, 3428, 6232, 3648, 6352, 6680, 3868,
    4007, 4052, 4088, 1840, 2506, 4210, 2627, 4219, 4264, 2737, 4227, 4272, 4308, 2836, 1879, 1889,
    1898, 1906, 65, 1649, 3354, 3574, 1715, 3399, 6203, 3619, 3794, 3839, 1781, 3435, 6239, 3655,
    6359, 6687, 3875, 4014, 4059, 4095, 1847, 3463, 6267, 3683, 6387, 6715, 3903, 6471, 6799, 7008,
    4123, 4234, 4279, 4315, 4343, 1913, 2517, 4430, 2638, 4439, 4484, 2748, 4447, 4492, 4528, 2847,
    4454, 4499, 4535, 4563, 2935, 1945, 1955, 1964, 1972, 1979, 77, 1655, 3360, 3580, 1721, 3405,
    6209, 3625, 3800, 3845, 1787, 3441, 6245, 3661, 6365, 6693, 3881, 4020, 4065, 4101, 1853, 3469,
    6273, 3689, 6393, 6721, 3909, 6477, 6805, 7014, 4129, 4240, 4285, 4321, 4349, 1919, 3490, 6294,
    3710, 6414, 6742, 3930, 6498, 6826, 7035, 4150, 6554, 6882, 7091, 1603, 4370, 4460, 4505, 4541,
    4569, 4590, 1985, 2528, 4650, 2649, 4659, 4704, 2759, 4667, 4712, 4748, 2858, 4674, 4719, 4755,
    4783, 2946, 4680, 4725, 4761, 4789, 4810, 3023, 2011, 2021, 2030, 2038, 2045, 2051, 89, 173,
    2534, 2655, 185, 2535, 4866, 2656, 2765, 2766, 197, 2536, 4867, 2657, 4876, 4921, 2767, 2864,
    2865, 2866, 209, 2537, 4868, 2658, 4877, 4922, 2768, 4885, 4930, 4966, 2867, 2952, 2953, 2954,
    2955, 221, 2538, 4869, 2659, 4878, 4923, 2769, 4886, 4931, 4967, 2868, 4893, 4938, 4974, 5002,
    2956, 3029, 3030, 3031, 3032, 3033, 233, 2539, 4870, 2660, 4879, 4924, 2770, 4887, 4932, 4968,
    2869, 4894, 4939, 4975, 5003, 2957, 4900, 4945, 4981, 5009, 5030, 3034, 3095, 3096, 3097, 3098,
    3099, 3100, 245, 251, 2072, 252, 2073, 2083, 253, 2074, 2084, 2093, 254, 2075, 2085, 2094,
    2102, 255, 2076, 2086, 2095, 2103, 2110, 256, 2077, 2087, 2096, 2104, 2111, 2117, 257, 95, 96,
    97, 98, 99, 100, 101, 18, 1616, 2474, 1682, 30, 1626, 3331, 3551, 1692, 2485, 3771, 2606, 1748,
    1758, 42, 1635, 3340, 3560, 1701, 3385, 6189, 3605, 3780, 3825, 1767, 2496, 3991, 2617, 4000,
    4045, 2727, 1814, 1824, 1833, 54, 1643, 3348, 3568, 1709, 3393, 6197, 3613, 3788, 3833, 1775,
    3429, 6233, 3649, 6353, 6681, 3869, 4008, 4053, 4089, 1841, 2507, 4211, 2628, 4220, 4265, 2738,
    4228, 4273, 4309, 2837, 1880, 1890, 1899, 1907, 66, 1650, 3355, 3575, 1716, 3400, 6204, 3620,
    3795, 3840, 1782, 3436, 6240, 3656, 6360, 6688, 3876, 4015, 4060, 4096, 1848, 3464, 6268, 3684,
    6388, 6716, 3904, 6472, 6800, 7009, 4124, 4235, 4280, 4316, 4344, 1914, 2518, 4431, 2639, 4440,
    4485, 2749, 4448, 4493, 4529, 2848, 4455, 4500, 4536, 4564, 2936, 1946, 1956, 1965, 1973, 1980,
    78, 1656, 3361, 3581, 1722, 3406, 6210, 3626, 3801, 3846, 1788, 3442, 6246, 3662, 6366, 6694,
    3882, 4021, 4066, 4102, 1854, 3470, 6274, 3690, 6394, 6722, 3910, 6478, 6806, 7015, 4130, 4241,
    4286, 4322, 4350, 1920, 3491, 6295, 3711, 6415, 6743, 3931, 6499, 6827, 7036, 4151, 6555, 6883,
    7092, 7217, 4371, 4461, 4506, 4542, 4570, 4591, 1986, 2529, 4651, 2650, 4660, 4705, 2760, 4668,
    4713, 4749, 2859, 4675, 4720, 4756, 4784, 2947, 4681, 4726, 4762, 4790, 4811, 3024, 2012, 2022,
    2031, 2039, 2046, 2052, 90, 1661, 3366, 3586, 1727, 3411, 6215, 3631, 3806, 3851, 1793, 3447,
    6251, 3667, 6371, 6699, 3887, 4026, 4071, 4107, 1859, 3475, 6279, 3695, 6399, 6727, 3915, 6483,
    6811, 7020, 4135, 4246, 4291, 4327, 4355, 1925, 3496, 6300, 3716, 6420, 6748, 3936, 6504, 6832,
    7041, 4156, 6560, 6888, 7097, 7222, 4376, 4466, 4511, 4547, 4575, 4596, 1991, 3511, 6315, 3731,
    6435, 6763, 3951, 6519, 6847, 7056, 4171, 6575, 6903, 7112, 7237, 4391, 6610, 6938, 7147, 7272,
    1604, 4611, 4686, 4731, 4767, 4795, 4816, 4831, 2057, 2540, 4871, 2661, 4880, 4925, 2771, 4888,
    4933, 4969, 2870, 4895, 4940, 4976, 5004, 2958, 4901, 4946, 4982, 5010, 5031, 3035, 4906, 4951,
    4987, 5015, 5036, 5051, 3101, 2078, 2088, 2097, 2105, 2112, 2118, 2123, 102, 174, 2545, 2666,
    186, 2546, 5086, 2667, 2776, 2777, 198, 2547, 5087, 2668, 5096, 5141, 2778, 2875, 2876, 2877,
    210, 2548, 5088, 2669, 5097, 5142, 2779, 5105, 5150, 5186, 2878, 2963, 2964, 2965, 2966, 222,
    2549, 5089, 2670, 5098, 5143, 2780, 5106, 5151, 5187, 2879, 5113, 5158, 5194, 5222, 2967, 3040,
    3041, 3042, 3043, 3044, 234, 2550, 5090, 2671, 5099, 5144, 2781, 5107, 5152, 5188, 2880, 5114,
    5159, 5195, 5223, 2968, 5120, 5165, 5201, 5229, 5250, 3045, 3106, 3107, 3108, 3109, 3110, 3111,
    246, 2551, 5091, 2672, 5100, 5145, 2782, 5108, 5153, 5189, 2881, 5115, 5160, 5196, 5224, 2969,
    5121, 5166, 5202, 5230, 5251, 3046, 5126, 5171, 5207, 5235, 5256, 5271, 3112, 3161, 3162, 3163,
    3164, 3165, 3166, 3167, 258, 263, 2138, 264, 2139, 2149, 265, 2140, 2150, 2159, 266, 2141,
    2151, 2160, 2168, 267, 2142, 2152, 2161, 2169, 2176, 268, 2143, 2153, 2162, 2170, 2177, 2183,
    269, 2144, 2154, 2163, 2171, 2178, 2184, 2189, 270, 107, 108, 109, 110, 111, 112, 113, 114, 19,
    1617, 2475, 1683, 31, 1627, 3332, 3552, 1693, 2486, 3772, 2607, 1749, 1759, 43, 1636, 3341,
    3561, 1702, 3386, 6190, 3606, 3781, 3826, 1768, 2497, 3992, 2618, 4001, 4046, 2728, 1815, 1825,
    1834, 55, 1644, 3349, 3569, 1710, 3394, 6198, 3614, 3789, 3834, 1776, 3430, 6234, 3650, 6354,
    6682, 3870, 4009, 4054, 4090, 1842, 2508, 4212, 2629, 4221, 4266, 2739, 4229, 4274, 4310, 2838,
    1881, 1891, 1900, 1908, 67, 1651, 3356, 3576, 1717, 3401, 6205, 3621, 3796, 3841, 1783, 3437,
    6241, 3657, 6361, 6689, 3877, 4016, 4061, 4097, 1849, 3465, 6269, 3685, 6389, 6717, 3905, 6473,
    6801, 7010, 4125, 4236, 4281, 4317, 4345, 1915, 2519, 4432, 2640, 4441, 4486, 2750, 4449, 4494,
    4530, 2849, 4456, 4501, 4537, 4565, 2937, 1947, 1957, 1966, 1974, 1981, 79, 1657, 3362, 3582,
    1723, 3407, 6211, 3627, 3802, 3847, 1789, 3443, 6247, 3663, 6367, 6695, 3883, 4022, 4067, 4103,
    1855, 3471, 6275, 3691, 6395, 6723, 3911, 6479, 6807, 7016, 4131, 4242, 4287, 4323, 4351, 1921,
    3492, 6296, 3712, 6416, 6744, 3932, 6500, 6828, 7037, 4152, 6556, 6884, 7093, 7218, 4372, 4462,
    4507, 4543, 4571, 4592, 1987, 2530, 4652, 2651, 4661, 4706, 2761, 4669, 4714, 4750, 2860, 4676,
    4721, 4757, 4785, 2948, 4682, 4727, 4763, 4791, 4812, 3025, 2013, 2023, 2032, 2040, 2047, 2053,
    91, 1662, 3367, 3587, 1728, 3412, 6216, 3632, 3807, 3852, 1794, 3448, 6252, 3668, 6372, 6700,
    3888, 4027, 4072, 4108, 1860, 3476, 6280, 3696, 6400, 6728, 3916, 6484, 6812, 7021, 4136, 4247,
    4292, 4328, 4356, 1926, 3497, 6301, 3717, 6421, 6749, 3937, 6505, 6833, 7042, 4157, 6561, 6889,
    7098, 7223, 4377, 4467, 4512, 4548, 4576, 4597, 1992, 3512, 6316, 3732, 6436, 6764, 3952, 6520,
    6848, 7057, 4172, 6576, 6904, 7113, 7238, 4392, 6611, 6939, 7148, 7273, 7342, 4612, 4687, 4732,
    4768, 4796, 4817, 4832, 2058, 2541, 4872, 2662, 4881, 4926, 2772, 4889, 4934, 4970, 2871, 4896,
    4941, 4977, 5005, 2959, 4902, 4947, 4983, 5011, 5032, 3036, 4907, 4952, 4988, 5016, 5037, 5052,
    3102, 2079, 2089, 2098, 2106, 2113, 2119, 2124, 103, 1666, 3371, 3591, 1732, 3416, 6220, 3636,
    3811, 3856, 1798, 3452, 6256, 3672, 6376, 6704, 3892, 4031, 4076, 4112, 1864, 3480, 6284, 3700,
    6404, 6732, 3920, 6488, 6816, 7025, 4140, 4251, 4296, 4332, 4360, 1930, 3501, 6305, 3721, 6425,
    6753, 3941, 6509, 6837, 7046, 4161, 6565, 6893, 7102, 7227, 4381, 4471, 4516, 4552, 4580, 4601,
    1996, 3516, 6320, 3736, 6440, 6768, 3956, 6524, 6852, 7061, 4176, 6580, 6908, 7117, 7242, 4396,
    6615, 6943, 7152, 7277, 7346, 4616, 4691, 4736, 4772, 4800, 4821, 4836, 2062, 3526, 6330, 3746,
    6450, 6778, 3966, 6534, 6862, 7071, 4186, 6590, 6918, 7127, 7252, 4406, 6625, 6953, 7162, 7287,
    7356, 4626, 1606, 1606, 1606, 1606, 1606, 1605, 1606, 4911, 4956, 4992, 5020, 5041, 5056, 1606,
    2128, 2552, 5092, 2673, 5101, 5146, 2783, 5109, 5154, 5190, 2882, 5116, 5161, 5197, 5225, 2970,
    5122, 5167, 5203, 5231, 5252, 3047, 5127, 5172, 5208, 5236, 5257, 5272, 3113, 5131, 5176, 5212,
    5240, 5261, 5276, 1606, 3168, 2145, 2155, 2164, 2172, 2179, 2185, 2190, 2194, 115, 175, 2556,
    2677, 187, 2557, 5306, 2678, 2787, 2788, 199, 2558, 5307, 2679, 5316, 5361, 2789, 2886, 2887,
    2888, 211, 2559, 5308, 2680, 5317, 5362, 2790, 5325, 5370, 5406, 2889, 2974, 2975, 2976, 2977,
    223, 2560, 5309, 2681, 5318, 5363, 2791, 5326, 5371, 5407, 2890, 5333, 5378, 5414, 5442, 2978,
    3051, 3052, 3053, 3054, 3055, 235, 2561, 5310, 2682, 5319, 5364, 2792, 5327, 5372, 5408, 2891,
    5334, 5379, 5415, 5443, 2979, 5340, 5385, 5421, 5449, 5470, 3056, 3117, 3118, 3119, 3120, 3121,
    3122, 247, 2562, 5311, 2683, 5320, 5365, 2793, 5328, 5373, 5409, 2892, 5335, 5380, 5416, 5444,
    2980, 5341, 5386, 5422, 5450, 5471, 3057, 5346, 5391, 5427, 5455, 5476, 5491, 3123, 3172, 3173,
    3174, 3175, 3176, 3177, 3178, 259, 2563, 5312, 2684, 5321, 5366, 2794, 5329, 5374, 5410, 2893,
    5336, 5381, 5417, 5445, 2981, 5342, 5387, 5423, 5451, 5472, 3058, 5347, 5392, 5428, 5456, 5477,
    5492, 3124, 5351, 5396, 5432, 5460, 5481, 5496, 1606, 3179, 3216, 3217, 3218, 3219, 3220, 3221,
    3222, 3223, 271, 275, 2204, 276, 2205, 2215, 277, 2206, 2216, 2225, 278, 2207, 2217, 2226,
    2234, 279, 2208, 2218, 2227, 2235, 2242, 280, 2209, 2219, 2228, 2236, 2243, 2249, 281, 2210,
    2220, 2229, 2237, 2244, 2250, 2255, 282, 2211, 2221, 2230, 2238, 2245, 2251, 2256, 2260, 283,
    119, 120, 121, 122, 123, 124, 125, 126, 127, 20, 176, 2476, 188, 32, 176, 2567, 2688, 188,
    2487, 2798, 2608, 200, 200, 44, 176, 2567, 2688, 188, 2568, 5526, 2689, 2798, 2799, 200, 2498,
    2897, 2619, 2897, 2898, 2729, 212, 212, 212, 56, 176, 2567, 2688, 188, 2568, 5526, 2689, 2798,
    2799, 200, 2569, 5527, 2690, 5536, 5581, 2800, 2897, 2898, 2899, 212, 2509, 2985, 2630, 2985,
    2986, 2740, 2985, 2986, 2987, 2839, 224, 224, 224, 224, 68, 176, 2567, 2688, 188, 2568, 5526,
    2689, 2798, 2799, 200, 2569, 5527, 2690, 5536, 5581, 2800, 2897, 2898, 2899, 212, 2570, 5528,
    2691, 5537, 5582, 2801, 5545, 5590, 5626, 2900, 2985, 2986, 2987, 2988, 224, 2520, 3062, 2641,
    3062, 3063, 2751, 3062, 3063, 3064, 2850, 3062, 3063, 3064, 3065, 2938, 236, 236, 236, 236,
    236, 80, 176, 2567, 2688, 188, 2568, 5526, 2689, 2798, 2799, 200, 2569, 5527, 2690, 5536, 5581,
    2800, 2897, 2898, 2899, 212, 2570, 5528, 2691, 5537, 5582, 2801, 5545, 5590, 5626, 2900, 2985,
    2986, 2987, 2988, 224, 2571, 5529, 2692, 5538, 5583, 2802, 5546, 5591, 5627, 2901, 5553, 5598,
    5634, 5662, 2989, 3062, 3063, 3064, 3065, 3066, 236, 2531, 3128, 2652, 3128, 3129, 2762, 3128,
    3129, 3130, 2861, 3128, 3129, 3130, 3131, 2949, 3128, 3129, 3130, 3131, 3132, 3026, 248, 248,
    248, 248, 248, 248, 92, 176, 2567, 2688, 188, 2568, 5526, 2689, 2798, 2799, 200, 2569, 5527,
    2690, 5536, 5581, 2800, 2897, 2898, 2899, 212, 2570, 5528, 2691, 5537, 5582, 2801, 5545, 5590,
    5626, 2900, 2985, 2986, 2987, 2988, 224, 2571, 5529, 2692, 5538, 5583, 2802, 5546, 5591, 5627,
    2901, 5553, 5598, 5634, 5662, 2989, 3062, 3063, 3064, 3065, 3066, 236, 2572, 5530, 2693, 5539,
    5584, 2803, 5547, 5592, 5628, 2902, 5554, 5599, 5635, 5663, 2990, 5560, 5605, 5641, 5669, 5690,
    3067, 3128, 3129, 3130, 3131, 3132, 3133, 248, 2542, 3183, 2663, 3183, 3184, 2773, 3183, 3184,
    3185, 2872, 3183, 3184, 3185, 3186, 2960, 3183, 3184, 3185, 3186, 3187, 3037, 3183, 3184, 3185,
    3186, 3187, 3188, 3103, 260, 260, 260, 260, 260, 260, 260, 104, 176, 2567, 2688, 188, 2568,
    5526, 2689, 2798, 2799, 200, 2569, 5527, 2690, 5536, 5581, 2800, 2897, 2898, 2899, 212, 2570,
    5528, 2691, 5537, 5582, 2801, 5545, 5590, 5626, 2900, 2985, 2986, 2987, 2988, 224, 2571, 5529,
    2692, 5538, 5583, 2802, 5546, 5591, 5627, 2901, 5553, 5598, 5634, 5662, 2989, 3062, 3063, 3064,
    3065, 3066, 236, 2572, 5530, 2693, 5539, 5584, 2803, 5547, 5592, 5628, 2902, 5554, 5599, 5635,
    5663, 2990, 5560, 5605, 5641, 5669, 5690, 3067, 3128, 3129, 3130, 3131, 3132, 3133, 248, 2573,
    5531, 2694, 5540, 5585, 2804, 5548, 5593, 5629, 2903, 5555, 5600, 5636, 5664, 2991, 5561, 5606,
    5642, 5670, 5691, 3068, 5566, 5611, 5647, 5675, 5696, 5711, 3134, 3183, 3184, 3185, 3186, 3187,
    3188, 3189, 260, 2553, 3227, 2674, 3227, 3228, 2784, 3227, 3228, 3229, 2883, 3227, 3228, 3229,
    3230, 2971, 3227, 3228, 3229, 3230, 3231, 3048, 3227, 3228, 3229, 3230, 3231, 3232, 3114, 3227,
    3228, 3229, 3230, 3231, 3232, 3233, 3169, 272, 272, 272, 272, 272, 272, 272, 272, 116, 176,
    2567, 2688, 188, 2568, 5526, 2689, 2798, 2799, 200, 2569, 5527, 2690, 5536, 5581, 2800, 2897,
    2898, 2899, 212, 2570, 5528, 2691, 5537, 5582, 2801, 5545, 5590, 5626, 2900, 2985, 2986, 2987,
    2988, 224, 2571, 5529, 2692, 5538, 5583, 2802, 5546, 5591, 5627, 2901, 5553, 5598, 5634, 5662,
    2989, 3062, 3063, 3064, 3065, 3066, 236, 2572, 5530, 2693, 5539, 5584, 2803, 5547, 5592, 5628,
    2902, 5554, 5599, 5635, 5663, 2990, 5560, 5605, 5641, 5669, 5690, 3067, 3128, 3129, 3130, 3131,
    3132, 3133, 248, 2573, 5531, 2694, 5540, 5585, 2804, 5548, 5593, 5629, 2903, 5555, 5600, 5636,
    5664, 2991, 5561, 5606, 5642, 5670, 5691, 3068, 5566, 5611, 5647, 5675, 5696, 5711, 3134, 3183,
    3184, 3185, 3186, 3187, 3188, 3189, 260, 2574, 5532, 2695, 5541, 5586, 2805, 5549, 5594, 5630,
    2904, 5556, 5601, 5637, 5665, 2992, 5562, 5607, 5643, 5671, 5692, 3069, 5567, 5612, 5648, 5676,
    5697, 5712, 3135, 5571, 5616, 5652, 5680, 5701, 5716, 1606, 3190, 3227, 3228, 3229, 3230, 3231,
    3232, 3233, 3234, 272, 2564, 3260, 2685, 3260, 3261, 2795, 3260, 3261, 3262, 2894, 3260, 3261,
    3262, 3263, 2982, 3260, 3261, 3262, 3263, 3264, 3059, 3260, 3261, 3262, 3263, 3264, 3265, 3125,
    3260, 3261, 3262, 3263, 3264, 3265, 3266, 3180, 3260, 3261, 3262, 3263, 3264, 3265, 3266, 3267,
    3224, 284, 284, 284, 284, 284, 284, 284, 284, 284, 128, 176, 287, 288, 188, 287, 2270, 288,
    289, 289, 200, 287, 2270, 288, 2271, 2281, 289, 290, 290, 290, 212, 287, 2270, 288, 2271, 2281,
    289, 2272, 2282, 2291, 290, 291, 291, 291, 291, 224, 287, 2270, 288, 2271, 2281, 289, 2272,
    2282, 2291, 290, 2273, 2283, 2292, 2300, 291, 292, 292, 292, 292, 292, 236, 287, 2270, 288,
    2271, 2281, 289, 2272, 2282, 2291, 290, 2273, 2283, 2292, 2300, 291, 2274, 2284, 2293, 2301,
    2308, 292, 293, 293, 293, 293, 293, 293, 248, 287, 2270, 288, 2271, 2281, 289, 2272, 2282,
    2291, 290, 2273, 2283, 2292, 2300, 291, 2274, 2284, 2293, 2301, 2308, 292, 2275, 2285, 2294,
    2302, 2309, 2315, 293, 294, 294, 294, 294, 294, 294, 294, 260, 287, 2270, 288, 2271, 2281, 289,
    2272, 2282, 2291, 290, 2273, 2283, 2292, 2300, 291, 2274, 2284, 2293, 2301, 2308, 292, 2275,
    2285, 2294, 2302, 2309, 2315, 293, 2276, 2286, 2295, 2303, 2310, 2316, 2321, 294, 295, 295,
    295, 295, 295, 295, 295, 295, 272, 287, 2270, 288, 2271, 2281, 289, 2272, 2282, 2291, 290,
    2273, 2283, 2292, 2300, 291, 2274, 2284, 2293, 2301, 2308, 292, 2275, 2285, 2294, 2302, 2309,
    2315, 293, 2276, 2286, 2295, 2303, 2310, 2316, 2321, 294, 2277, 2287, 2296, 2304, 2311, 2317,
    2322, 2326, 295, 296, 296, 296, 296, 296, 296, 296, 296, 296, 284, 131, 131, 132, 131, 132,
    133, 131, 132, 133, 134, 131, 132, 133, 134, 135, 131, 132, 133, 134, 135, 136, 131, 132, 133,
    134, 135, 136, 137, 131, 132, 133, 134, 135, 136, 137, 138, 131, 132, 133, 134, 135, 136, 137,
    138, 139, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 11, 167, 179, 23, 12, 1610, 2468,
    1676, 24, 168, 2479, 2600, 180, 191, 1742, 192, 35, 36, 13, 1611, 2469, 1677, 25, 1621, 3326,
    3546, 1687, 2480, 3766, 2601, 1743, 1753, 37, 169, 2490, 2611, 181, 2491, 3986, 2612, 2721,
    2722, 193, 203, 1808, 204, 1809, 1819, 205, 47, 48, 49, 14, 1612, 2470, 1678, 26, 1622, 3327,
    3547, 1688, 2481, 3767, 2602, 1744, 1754, 38, 1631, 3336, 3556, 1697, 3381, 1600, 3601, 3776,
    3821, 1763, 2492, 3987, 2613, 3996, 4041, 2723, 1810, 1820, 1829, 50, 170, 2501, 2622, 182,
    2502, 4206, 2623, 2732, 2733, 194, 2503, 4207, 2624, 4216, 4261, 2734, 2831, 2832, 2833, 206,
    215, 1874, 216, 1875, 1885, 217, 1876, 1886, 1895, 218, 59, 60, 61, 62, 15, 1613, 2471, 1679,
    27, 1623, 3328, 3548, 1689, 2482, 3768, 2603, 1745, 1755, 39, 1632, 3337, 3557, 1698, 3382,
    6186, 3602, 3777, 3822, 1764, 2493, 3988, 2614, 3997, 4042, 2724, 1811, 1821, 1830, 51, 1640,
    3345, 3565, 1706, 3390, 6194, 3610, 3785, 3830, 1772, 3426, 6230, 3646, 6350, 1601, 3866, 4005,
    4050, 4086, 1838, 2504, 4208, 2625, 4217, 4262, 2735, 4225, 4270, 4306, 2834, 1877, 1887, 1896,
    1904, 63, 171, 2512, 2633, 183, 2513, 4426, 2634, 2743, 2744, 195, 2514, 4427, 2635, 4436,
    4481, 2745, 2842, 2843, 2844, 207, 2515, 4428, 2636, 4437, 4482, 2746, 4445, 4490, 4526, 2845,
    2930, 2931, 2932, 2933, 219, 227, 1940, 228, 1941, 1951, 229, 1942, 1952, 1961, 230, 1943,
    1953, 1962, 1970, 231, 71, 72, 73, 74, 75, 16, 1614, 2472, 1680, 28, 1624, 3329, 3549, 1690,
    2483, 3769, 2604, 1746, 1756, 40, 1633, 3338, 3558, 1699, 3383, 6187, 3603, 3778, 3823, 1765,
    2494, 3989, 2615, 3998, 4043, 2725, 1812, 1822, 1831, 52, 1641, 3346, 3566, 1707, 3391, 6195,
    3611, 3786, 3831, 1773, 3427, 6231, 3647, 6351, 6679, 3867, 4006, 4051, 4087, 1839, 2505, 4209,
    2626, 4218, 4263, 2736, 4226, 4271, 4307, 2835, 1878, 1888, 1897, 1905, 64, 1648, 3353, 3573,
    1714, 3398, 6202, 3618, 3793, 3838, 1780, 3434, 6238, 3654, 6358, 6686, 3874, 4013, 4058, 4094,
    1846, 3462, 6266, 3682, 6386, 6714, 3902, 6470, 6798, 1602, 4122, 4233, 4278, 4314, 4342, 1912,
    2516, 4429, 2637, 4438, 4483, 2747, 4446, 4491, 4527, 2846, 4453, 4498, 4534, 4562, 2934, 1944,
    1954, 1963, 1971, 1978, 76, 172, 2523, 2644, 184, 2524, 4646, 2645, 2754, 2755, 196, 2525,
    4647, 2646, 4656, 4701, 2756, 2853, 2854, 2855, 208, 2526, 4648, 2647, 4657, 4702, 2757, 4665,
    4710, 4746, 2856, 2941, 2942, 2943, 2944, 220, 2527, 4649, 2648, 4658, 4703, 2758, 4666, 4711,
    4747, 2857, 4673, 4718, 4754, 4782, 2945, 3018, 3019, 3020, 3021, 3022, 232, 239, 2006, 240,
    2007, 2017, 241, 2008, 2018, 2027, 242, 2009, 2019, 2028, 2036, 243, 2010, 2020, 2029, 2037,
    2044, 244, 83, 84, 85, 86, 87, 88, 17, 1615, 2473, 1681, 29, 1625, 3330, 3550, 1691, 2484,
    3770, 2605, 1747, 1757, 41, 1634, 3339, 3559, 1700, 3384, 6188, 3604, 3779, 3824, 1766, 2495,
    3990, 2616, 3999, 4044, 2726, 1813, 1823, 1832, 53, 1642, 3347, 3567, 1708, 3392, 6196, 3612,
    3787, 3832, 1774, 3428, 6232, 3648, 6352, 6680, 3868, 4007, 4052, 4088, 1840, 2506, 4210, 2627,
    4219, 4264, 2737, 4227, 4272, 4308, 2836, 1879, 1889, 1898, 1906, 65, 1649, 3354, 3574, 1715,
    3399, 6203, 3619, 3794, 3839, 1781, 3435, 6239, 3655, 6359, 6687, 3875, 4014, 4059, 4095, 1847,
    3463, 6267, 3683, 6387, 6715, 3903, 6471, 6799, 7008, 4123, 4234, 4279, 4315, 4343, 1913, 2517,
    4430, 2638, 4439, 4484, 2748, 4447, 4492, 4528, 2847, 4454, 4499, 4535, 4563, 2935, 1945, 1955,
    1964, 1972, 1979, 77, 1655, 3360, 3580, 1721, 3405, 6209, 3625, 3800, 3845, 1787, 3441, 6245,
    3661, 6365, 6693, 3881, 4020, 4065, 4101, 1853, 3469, 6273, 3689, 6393, 6721, 3909, 6477, 6805,
    7014, 4129, 4240, 4285, 4321, 4349, 1919, 3490, 6294, 3710, 6414, 6742, 3930, 6498, 6826, 7035,
    4150, 6554, 6882, 7091, 1603, 4370, 4460, 4505, 4541, 4569, 4590, 1985, 2528, 4650, 2649, 4659,
    4704, 2759, 4667, 4712, 4748, 2858, 4674, 4719, 4755, 4783, 2946, 4680, 4725, 4761, 4789, 4810,
    3023, 2011, 2021, 2030, 2038, 2045, 2051, 89, 173, 2534, 2655, 185, 2535, 4866, 2656, 2765,
    2766, 197, 2536, 4867, 2657, 4876, 4921, 2767, 2864, 2865, 2866, 209, 2537, 4868, 2658, 4877,
    4922, 2768, 4885, 4930, 4966, 2867, 2952, 2953, 2954, 2955, 221, 2538, 4869, 2659, 4878, 4923,
    2769, 4886, 4931, 4967, 2868, 4893, 4938, 4974, 5002, 2956, 3029, 3030, 3031, 3032, 3033, 233,
    2539, 4870, 2660, 4879, 4924, 2770, 4887, 4932, 4968, 2869, 4894, 4939, 4975, 5003, 2957, 4900,
    4945, 4981, 5009, 5030, 3034, 3095, 3096, 3097, 3098, 3099, 3100, 245, 251, 2072, 252, 2073,
    2083, 253, 2074, 2084, 2093, 254, 2075, 2085, 2094, 2102, 255, 2076, 2086, 2095, 2103, 2110,
    256, 2077, 2087, 2096, 2104, 2111, 2117, 257, 95, 96, 97, 98, 99, 100, 101, 18, 1616, 2474,
    1682, 30, 1626, 3331, 3551, 1692, 2485, 3771, 2606, 1748, 1758, 42, 1635, 3340, 3560, 1701,
    3385, 6189, 3605, 3780, 3825, 1767, 2496, 3991, 2617, 4000, 4045, 2727, 1814, 1824, 1833, 54,
    1643, 3348, 3568, 1709, 3393, 6197, 3613, 3788, 3833, 1775, 3429, 6233, 3649, 6353, 6681, 3869,
    4008, 4053, 4089, 1841, 2507, 4211, 2628, 4220, 4265, 2738, 4228, 4273, 4309, 2837, 1880, 1890,
    1899, 1907, 66, 1650, 3355, 3575, 1716, 3400, 6204, 3620, 3795, 3840, 1782, 3436, 6240, 3656,
    6360, 6688, 3876, 4015, 4060, 4096, 1848, 3464, 6268, 3684, 6388, 6716, 3904, 6472, 6800, 7009,
    4124, 4235, 4280, 4316, 4344, 1914, 2518, 4431, 2639, 4440, 4485, 2749, 4448, 4493, 4529, 2848,
    4455, 4500, 4536, 4564, 2936, 1946, 1956, 1965, 1973, 1980, 78, 1656, 3361, 3581, 1722, 3406,
    6210, 3626, 3801, 3846, 1788, 3442, 6246, 3662, 6366, 6694, 3882, 4021, 4066, 4102, 1854, 3470,
    6274, 3690, 6394, 6722, 3910, 6478, 6806, 7015, 4130, 4241, 4286, 4322, 4350, 1920, 3491, 6295,
    3711, 6415, 6743, 3931, 6499, 6827, 7036, 4151, 6555, 6883, 7092, 7217, 4371, 4461, 4506, 4542,
    4570, 4591, 1986, 2529, 4651, 2650, 4660, 4705, 2760, 4668, 4713, 4749, 2859, 4675, 4720, 4756,
    4784, 2947, 4681, 4726, 4762, 4790, 4811, 3024, 2012, 2022, 2031, 2039, 2046, 2052, 90, 1661,
    3366, 3586, 1727, 3411, 6215, 3631, 3806, 3851, 1793, 3447, 6251, 3667, 6371, 6699, 3887, 4026,
    4071, 4107, 1859, 3475, 6279, 3695, 6399, 6727, 3915, 6483, 6811, 7020, 4135, 4246, 4291, 4327,
    4355, 1925, 3496, 6300, 3716, 6420, 6748, 3936, 6504, 6832, 7041, 4156, 6560, 6888, 7097, 7222,
    4376, 4466, 4511, 4547, 4575, 4596, 1991, 3511, 6315, 3731, 6435, 6763, 3951, 6519, 6847, 7056,
    4171, 6575, 6903, 7112, 7237, 4391, 6610, 6938, 7147, 7272, 1604, 4611, 4686, 4731, 4767, 4795,
    4816, 4831, 2057, 2540, 4871, 2661, 4880, 4925, 2771, 4888, 4933, 4969, 2870, 4895, 4940, 4976,
    5004, 2958, 4901, 4946, 4982, 5010, 5031, 3035, 4906, 4951, 4987, 5015, 5036, 5051, 3101, 2078,
    2088, 2097, 2105, 2112, 2118, 2123, 102, 174, 2545, 2666, 186, 2546, 5086, 2667, 2776, 2777,
    198, 2547, 5087, 2668, 5096, 5141, 2778, 2875, 2876, 2877, 210, 2548, 5088, 2669, 5097, 5142,
    2779, 5105, 5150, 5186, 2878, 2963, 2964, 2965, 2966, 222, 2549, 5089, 2670, 5098, 5143, 2780,
    5106, 5151, 5187, 2879, 5113, 5158, 5194, 5222, 2967, 3040, 3041, 3042, 3043, 3044, 234, 2550,
    5090, 2671, 5099, 5144, 2781, 5107, 5152, 5188, 2880, 5114, 5159, 5195, 5223, 2968, 5120, 5165,
    5201, 5229, 5250, 3045, 3106, 3107, 3108, 3109, 3110, 3111, 246, 2551, 5091, 2672, 5100, 5145,
    2782, 5108, 5153, 5189, 2881, 5115, 5160, 5196, 5224, 2969, 5121, 5166, 5202, 5230, 5251, 3046,
    5126, 5171, 5207, 5235, 5256, 5271, 3112, 3161, 3162, 3163, 3164, 3165, 3166, 3167, 258, 263,
    2138, 264, 2139, 2149, 265, 2140, 2150, 2159, 266, 2141, 2151, 2160, 2168, 267, 2142, 2152,
    2161, 2169, 2176, 268, 2143, 2153, 2162, 2170, 2177, 2183, 269, 2144, 2154, 2163, 2171, 2178,
    2184, 2189, 270, 107, 108, 109, 110, 111, 112, 113, 114, 19, 1617, 2475, 1683, 31, 1627, 3332,
    3552, 1693, 2486, 3772, 2607, 1749, 1759, 43, 1636, 3341, 3561, 1702, 3386, 6190, 3606, 3781,
    3826, 1768, 2497, 3992, 2618, 4001, 4046, 2728, 1815, 1825, 1834, 55, 1644, 3349, 3569, 1710,
    3394, 6198, 3614, 3789, 3834, 1776, 3430, 6234, 3650, 6354, 6682, 3870, 4009, 4054, 4090, 1842,
    2508, 4212, 2629, 4221, 4266, 2739, 4229, 4274, 4310, 2838, 1881, 1891, 1900, 1908, 67, 1651,
    3356, 3576, 1717, 3401, 6205, 3621, 3796, 3841, 1783, 3437, 6241, 3657, 6361, 6689, 3877, 4016,
    4061, 4097, 1849, 3465, 6269, 3685, 6389, 6717, 3905, 6473, 6801, 7010, 4125, 4236, 4281, 4317,
    4345, 1915, 2519, 4432, 2640, 4441, 4486, 2750, 4449, 4494, 4530, 2849, 4456, 4501, 4537, 4565,
    2937, 1947, 1957, 1966, 1974, 1981, 79, 1657, 3362, 3582, 1723, 3407, 6211, 3627, 3802, 3847,
    1789, 3443, 6247, 3663, 6367, 6695, 3883, 4022, 4067, 4103, 1855, 3471, 6275, 3691, 6395, 6723,
    3911, 6479, 6807, 7016, 4131, 4242, 4287, 4323, 4351, 1921, 3492, 6296, 3712, 6416, 6744, 3932,
    6500, 6828, 7037, 4152, 6556, 6884, 7093, 7218, 4372, 4462, 4507, 4543, 4571, 4592, 1987, 2530,
    4652, 2651, 4661, 4706, 2761, 4669, 4714, 4750, 2860, 4676, 4721, 4757, 4785, 2948, 4682, 4727,
    4763, 4791, 4812, 3025, 2013, 2023, 2032, 2040, 2047, 2053, 91, 1662, 3367, 3587, 1728, 3412,
    6216, 3632, 3807, 3852, 1794, 3448, 6252, 3668, 6372, 6700, 3888, 4027, 4072, 4108, 1860, 3476,
    6280, 3696, 6400, 6728, 3916, 6484, 6812, 7021, 4136, 4247, 4292, 4328, 4356, 1926, 3497, 6301,
    3717, 6421, 6749, 3937, 6505, 6833, 7042, 4157, 6561, 6889, 7098, 7223, 4377, 4467, 4512, 4548,
    4576, 4597, 1992, 3512, 6316, 3732, 6436, 6764, 3952, 6520, 6848, 7057, 4172, 6576, 6904, 7113,
    7238, 4392, 6611, 6939, 7148, 7273, 7342, 4612, 4687, 4732, 4768, 4796, 4817, 4832, 2058, 2541,
    4872, 2662, 4881, 4926, 2772, 4889, 4934, 4970, 2871, 4896, 4941, 4977, 5005, 2959, 4902, 4947,
    4983, 5011, 5032, 3036, 4907, 4952, 4988, 5016, 5037, 5052, 3102, 2079, 2089, 2098, 2106, 2113,
    2119, 2124, 103, 1666, 3371, 3591, 1732, 3416, 6220, 3636, 3811, 3856, 1798, 3452, 6256, 3672,
    6376, 6704, 3892, 4031, 4076, 4112, 1864, 3480, 6284, 3700, 6404, 6732, 3920, 6488, 6816, 7025,
    4140, 4251, 4296, 4332, 4360, 1930, 3501, 6305, 3721, 6425, 6753, 3941, 6509, 6837, 7046, 4161,
    6565, 6893, 7102, 7227, 4381, 4471, 4516, 4552, 4580, 4601, 1996, 3516, 6320, 3736, 6440, 6768,
    3956, 6524, 6852, 7061, 4176, 6580, 6908, 7117, 7242, 4396, 6615, 6943, 7152, 7277, 7346, 4616,
    4691, 4736, 4772, 4800, 4821, 4836, 2062, 3526, 6330, 3746, 6450, 6778, 3966, 6534, 6862, 7071,
    4186, 6590, 6918, 7127, 7252, 4406, 6625, 6953, 7162, 7287, 7356, 4626, 6645, 6973, 7182, 7307,
    7376, 1605, 4846, 4911, 4956, 4992, 5020, 5041, 5056, 5066, 2128, 2552, 5092, 2673, 5101, 5146,
    2783, 5109, 5154, 5190, 2882, 5116, 5161, 5197, 5225, 2970, 5122, 5167, 5203, 5231, 5252, 3047,
    5127, 5172, 5208, 5236, 5257, 5272, 3113, 5131, 5176, 5212, 5240, 5261, 5276, 5286, 3168, 2145,
    2155, 2164, 2172, 2179, 2185, 2190, 2194, 115, 175, 2556, 2677, 187, 2557, 5306, 2678, 2787,
    2788, 199, 2558, 5307, 2679, 5316, 5361, 2789, 2886, 2887, 2888, 211, 2559, 5308, 2680, 5317,
    5362, 2790, 5325, 5370, 5406, 2889, 2974, 2975, 2976, 2977, 223, 2560, 5309, 2681, 5318, 5363,
    2791, 5326, 5371, 5407, 2890, 5333, 5378, 5414, 5442, 2978, 3051, 3052, 3053, 3054, 3055, 235,
    2561, 5310, 2682, 5319, 5364, 2792, 5327, 5372, 5408, 2891, 5334, 5379, 5415, 5443, 2979, 5340,
    5385, 5421, 5449, 5470, 3056, 3117, 3118, 3119, 3120, 3121, 3122, 247, 2562, 5311, 2683, 5320,
    5365, 2793, 5328, 5373, 5409, 2892, 5335, 5380, 5416, 5444, 2980, 5341, 5386, 5422, 5450, 5471,
    3057, 5346, 5391, 5427, 5455, 5476, 5491, 3123, 3172, 3173, 3174, 3175, 3176, 3177, 3178, 259,
    2563, 5312, 2684, 5321, 5366, 2794, 5329, 5374, 5410, 2893, 5336, 5381, 5417, 5445, 2981, 5342,
    5387, 5423, 5451, 5472, 3058, 5347, 5392, 5428, 5456, 5477, 5492, 3124, 5351, 5396, 5432, 5460,
    5481, 5496, 5506, 3179, 3216, 3217, 3218, 3219, 3220, 3221, 3222, 3223, 271, 275, 2204, 276,
    2205, 2215, 277, 2206, 2216, 2225, 278, 2207, 2217, 2226, 2234, 279, 2208, 2218, 2227, 2235,
    2242, 280, 2209, 2219, 2228, 2236, 2243, 2249, 281, 2210, 2220, 2229, 2237, 2244, 2250, 2255,
    282, 2211, 2221, 2230, 2238, 2245, 2251, 2256, 2260, 283, 119, 120, 121, 122, 123, 124, 125,
    126, 127, 20, 1618, 2476, 1684, 32, 1628, 3333, 3553, 1694, 2487, 3773, 2608, 1750, 1760, 44,
    1637, 3342, 3562, 1703, 3387, 6191, 3607, 3782, 3827, 1769, 2498, 3993, 2619, 4002, 4047, 2729,
    1816, 1826, 1835, 56, 1645, 3350, 3570, 1711, 3395, 6199, 3615, 3790, 3835, 1777, 3431, 6235,
    3651, 6355, 6683, 3871, 4010, 4055, 4091, 1843, 2509, 4213, 2630, 4222, 4267, 2740, 4230, 4275,
    4311, 2839, 1882, 1892, 1901, 1909, 68, 1652, 3357, 3577, 1718, 3402, 6206, 3622, 3797, 3842,
    1784, 3438, 6242, 3658, 6362, 6690, 3878, 4017, 4062, 4098, 1850, 3466, 6270, 3686, 6390, 6718,
    3906, 6474, 6802, 7011, 4126, 4237, 4282, 4318, 4346, 1916, 2520, 4433, 2641, 4442, 4487, 2751,
    4450, 4495, 4531, 2850, 4457, 4502, 4538, 4566, 2938, 1948, 1958, 1967, 1975, 1982, 80, 1658,
    3363, 3583, 1724, 3408, 6212, 3628, 3803, 3848, 1790, 3444, 6248, 3664, 6368, 6696, 3884, 4023,
    4068, 4104, 1856, 3472, 6276, 3692, 6396, 6724, 3912, 6480, 6808, 7017, 4132, 4243, 4288, 4324,
    4352, 1922, 3493, 6297, 3713, 6417, 6745, 3933, 6501, 6829, 7038, 4153, 6557, 6885, 7094, 7219,
    4373, 4463, 4508, 4544, 4572, 4593, 1988, 2531, 4653, 2652, 4662, 4707, 2762, 4670, 4715, 4751,
    2861, 4677, 4722, 4758, 4786, 2949, 4683, 4728, 4764, 4792, 4813, 3026, 2014, 2024, 2033, 2041,
    2048, 2054, 92, 1663, 3368, 3588, 1729, 3413, 6217, 3633, 3808, 3853, 1795, 3449, 6253, 3669,
    6373, 6701, 3889, 4028, 4073, 4109, 1861, 3477, 6281, 3697, 6401, 6729, 3917, 6485, 6813, 7022,
    4137, 4248, 4293, 4329, 4357, 1927, 3498, 6302, 3718, 6422, 6750, 3938, 6506, 6834, 7043, 4158,
    6562, 6890, 7099, 7224, 4378, 4468, 4513, 4549, 4577, 4598, 1993, 3513, 6317, 3733, 6437, 6765,
    3953, 6521, 6849, 7058, 4173, 6577, 6905, 7114, 7239, 4393, 6612, 6940, 7149, 7274, 7343, 4613,
    4688, 4733, 4769, 4797, 4818, 4833, 2059, 2542, 4873, 2663, 4882, 4927, 2773, 4890, 4935, 4971,
    2872, 4897, 4942, 4978, 5006, 2960, 4903, 4948, 4984, 5012, 5033, 3037, 4908, 4953, 4989, 5017,
    5038, 5053, 3103, 2080, 2090, 2099, 2107, 2114, 2120, 2125, 104, 1667, 3372, 3592, 1733, 3417,
    6221, 3637, 3812, 3857, 1799, 3453, 6257, 3673, 6377, 6705, 3893, 4032, 4077, 4113, 1865, 3481,
    6285, 3701, 6405, 6733, 3921, 6489, 6817, 7026, 4141, 4252, 4297, 4333, 4361, 1931, 3502, 6306,
    3722, 6426, 6754, 3942, 6510, 6838, 7047, 4162, 6566, 6894, 7103, 7228, 4382, 4472, 4517, 4553,
    4581, 4602, 1997, 3517, 6321, 3737, 6441, 6769, 3957, 6525, 6853, 7062, 4177, 6581, 6909, 7118,
    7243, 4397, 6616, 6944, 7153, 7278, 7347, 4617, 4692, 4737, 4773, 4801, 4822, 4837, 2063, 3527,
    6331, 3747, 6451, 6779, 3967, 6535, 6863, 7072, 4187, 6591, 6919, 7128, 7253, 4407, 6626, 6954,
    7163, 7288, 7357, 4627, 6646, 6974, 7183, 7308, 7377, 7411, 4847, 4912, 4957, 4993, 5021, 5042,
    5057, 5067, 2129, 2553, 5093, 2674, 5102, 5147, 2784, 5110, 5155, 5191, 2883, 5117, 5162, 5198,
    5226, 2971, 5123, 5168, 5204, 5232, 5253, 3048, 5128, 5173, 5209, 5237, 5258, 5273, 3114, 5132,
    5177, 5213, 5241, 5262, 5277, 5287, 3169, 2146, 2156, 2165, 2173, 2180, 2186, 2191, 2195, 116,
    1609, 1609, 1609, 1736, 1609, 1609, 3640, 1609, 3860, 1802, 1609, 1609, 3676, 1609, 6708, 3896,
    1609, 4080, 4116, 1868, 1609, 1609, 3704, 1609, 6736, 3924, 1609, 6820, 7029, 4144, 1609, 4300,
    4336, 4364, 1934, 1609, 1609, 3725, 1609, 6757, 3945, 1609, 6841, 7050, 4165, 1609, 6897, 7106,
    7231, 4385, 1609, 4520, 4556, 4584, 4605, 2000, 1609, 1609, 3740, 1609, 6772, 3960, 1609, 6856,
    7065, 4180, 1609, 6912, 7121, 7246, 4400, 1609, 6947, 7156, 7281, 7350, 4620, 1609, 4740, 4776,
    4804, 4825, 4840, 2066, 1609, 1609, 3750, 1609, 6782, 3970, 1609, 6866, 7075, 4190, 1609, 6922,
    7131, 7256, 4410, 1609, 6957, 7166, 7291, 7360, 4630, 1609, 6977, 7186, 7311, 7380, 7414, 4850,
    1609, 4960, 4996, 5024, 5045, 5060, 5070, 2132, 1608, 1608, 1608, 1608, 1608, 1608, 1608, 1608,
    1608, 1608, 1608, 1608, 1608, 1608, 1608, 1608, 1608, 1608, 1608, 1608, 1608, 1608, 1608, 1608,
    1608, 1608, 1608, 1608, 1607, 1607, 1607, 1607, 1607, 1607, 1606, 1607, 1608, 1608, 1608, 1608,
    1608, 1608, 1608, 1607, 1608, 1609, 1609, 2685, 1609, 5367, 2795, 1609, 5375, 5411, 2894, 1609,
    5382, 5418, 5446, 2982, 1609, 5388, 5424, 5452, 5473, 3059, 1609, 5393, 5429, 5457, 5478, 5493,
    3125, 1609, 5397, 5433, 5461, 5482, 5497, 5507, 3180, 1608, 1608, 1608, 1608, 1608, 1608, 1608,
    1607, 1608, 1609, 2222, 2231, 2239, 2246, 2252, 2257, 2261, 1608, 128, 176, 2567, 2688, 188,
    2568, 5526, 2689, 2798, 2799, 200, 2569, 5527, 2690, 5536, 5581, 2800, 2897, 2898, 2899, 212,
    2570, 5528, 2691, 5537, 5582, 2801, 5545, 5590, 5626, 2900, 2985, 2986, 2987, 2988, 224, 2571,
    5529, 2692, 5538, 5583, 2802, 5546, 5591, 5627, 2901, 5553, 5598, 5634, 5662, 2989, 3062, 3063,
    3064, 3065, 3066, 236, 2572, 5530, 2693, 5539, 5584, 2803, 5547, 5592, 5628, 2902, 5554, 5599,
    5635, 5663, 2990, 5560, 5605, 5641, 5669, 5690, 3067, 3128, 3129, 3130, 3131, 3132, 3133, 248,
    2573, 5531, 2694, 5540, 5585, 2804, 5548, 5593, 5629, 2903, 5555, 5600, 5636, 5664, 2991, 5561,
    5606, 5642, 5670, 5691, 3068, 5566, 5611, 5647, 5675, 5696, 5711, 3134, 3183, 3184, 3185, 3186,
    3187, 3188, 3189, 260, 2574, 5532, 2695, 5541, 5586, 2805, 5549, 5594, 5630, 2904, 5556, 5601,
    5637, 5665, 2992, 5562, 5607, 5643, 5671, 5692, 3069, 5567, 5612, 5648, 5676, 5697, 5712, 3135,
    5571, 5616, 5652, 5680, 5701, 5716, 5726, 3190, 3227, 3228, 3229, 3230, 3231, 3232, 3233, 3234,
    272, 1609, 1609, 2696, 1609, 5587, 2806, 1609, 5595, 5631, 2905, 1609, 5602, 5638, 5666, 2993,
    1609, 5608, 5644, 5672, 5693, 3070, 1609, 5613, 5649, 5677, 5698, 5713, 3136, 1609, 5617, 5653,
    5681, 5702, 5717, 5727, 3191, 1608, 1608, 1608, 1608, 1608, 1608, 1608, 1607, 1608, 1609, 3261,
    3262, 3263, 3264, 3265, 3266, 3267, 1608, 284, 287, 2270, 288, 2271, 2281, 289, 2272, 2282,
    2291, 290, 2273, 2283, 2292, 2300, 291, 2274, 2284, 2293, 2301, 2308, 292, 2275, 2285, 2294,
    2302, 2309, 2315, 293, 2276, 2286, 2295, 2303, 2310, 2316, 2321, 294, 2277, 2287, 2296, 2304,
    2311, 2317, 2322, 2326, 295, 1609, 2288, 2297, 2305, 2312, 2318, 2323, 2327, 1608, 296, 131,
    132, 133, 134, 135, 136, 137, 138, 139, 140, 21, 177, 2477, 189, 33, 177, 2578, 2699, 189,
    2488, 2809, 2609, 201, 201, 45, 177, 2578, 2699, 189, 2579, 5746, 2700, 2809, 2810, 201, 2499,
    2908, 2620, 2908, 2909, 2730, 213, 213, 213, 57, 177, 2578, 2699, 189, 2579, 5746, 2700, 2809,
    2810, 201, 2580, 5747, 2701, 5756, 5801, 2811, 2908, 2909, 2910, 213, 2510, 2996, 2631, 2996,
    2997, 2741, 2996, 2997, 2998, 2840, 225, 225, 225, 225, 69, 177, 2578, 2699, 189, 2579, 5746,
    2700, 2809, 2810, 201, 2580, 5747, 2701, 5756, 5801, 2811, 2908, 2909, 2910, 213, 2581, 5748,
    2702, 5757, 5802, 2812, 5765, 5810, 5846, 2911, 2996, 2997, 2998, 2999, 225, 2521, 3073, 2642,
    3073, 3074, 2752, 3073, 3074, 3075, 2851, 3073, 3074, 3075, 3076, 2939, 237, 237, 237, 237,
    237, 81, 177, 2578, 2699, 189, 2579, 5746, 2700, 2809, 2810, 201, 2580, 5747, 2701, 5756, 5801,
    2811, 2908, 2909, 2910, 213, 2581, 5748, 2702, 5757, 5802, 2812, 5765, 5810, 5846, 2911, 2996,
    2997, 2998, 2999, 225, 2582, 5749, 2703, 5758, 5803, 2813, 5766, 5811, 5847, 2912, 5773, 5818,
    5854, 5882, 3000, 3073, 3074, 3075, 3076, 3077, 237, 2532, 3139, 2653, 3139, 3140, 2763, 3139,
    3140, 3141, 2862, 3139, 3140, 3141, 3142, 2950, 3139, 3140, 3141, 3142, 3143, 3027, 249, 249,
    249, 249, 249, 249, 93, 177, 2578, 2699, 189, 2579, 5746, 2700, 2809, 2810, 201, 2580, 5747,
    2701, 5756, 5801, 2811, 2908, 2909, 2910, 213, 2581, 5748, 2702, 5757, 5802, 2812, 5765, 5810,
    5846, 2911, 2996, 2997, 2998, 2999, 225, 2582, 5749, 2703, 5758, 5803, 2813, 5766, 5811, 5847,
    2912, 5773, 5818, 5854, 5882, 3000, 3073, 3074, 3075, 3076, 3077, 237, 2583, 5750, 2704, 5759,
    5804, 2814, 5767, 5812, 5848, 2913, 5774, 5819, 5855, 5883, 3001, 5780, 5825, 5861, 5889, 5910,
    3078, 3139, 3140, 3141, 3142, 3143, 3144, 249, 2543, 3194, 2664, 3194, 3195, 2774, 3194, 3195,
    3196, 2873, 3194, 3195, 3196, 3197, 2961, 3194, 3195, 3196, 3197, 3198, 3038, 3194, 3195, 3196,
    3197, 3198, 3199, 3104, 261, 261, 261, 261, 261, 261, 261, 105, 177, 2578, 2699, 189, 2579,
    5746, 2700, 2809, 2810, 201, 2580, 5747, 2701, 5756, 5801, 2811, 2908, 2909, 2910, 213, 2581,
    5748, 2702, 5757, 5802, 2812, 5765, 5810, 5846, 2911, 2996, 2997, 2998, 2999, 225, 2582, 5749,
    2703, 5758, 5803, 2813, 5766, 5811, 5847, 2912, 5773, 5818, 5854, 5882, 3000, 3073, 3074, 3075,
    3076, 3077, 237, 2583, 5750, 2704, 5759, 5804, 2814, 5767, 5812, 5848, 2913, 5774, 5819, 5855,
    5883, 3001, 5780, 5825, 5861, 5889, 5910, 3078, 3139, 3140, 3141, 3142, 3143, 3144, 249, 2584,
    5751, 2705, 5760, 5805, 2815, 5768, 5813, 5849, 2914, 5775, 5820, 5856, 5884, 3002, 5781, 5826,
    5862, 5890, 5911, 3079, 5786, 5831, 5867, 5895, 5916, 5931, 3145, 3194, 3195, 3196, 3197, 3198,
    3199, 3200, 261, 2554, 3238, 2675, 3238, 3239, 2785, 3238, 3239, 3240, 2884, 3238, 3239, 3240,
    3241, 2972, 3238, 3239, 3240, 3241, 3242, 3049, 3238, 3239, 3240, 3241, 3242, 3243, 3115, 3238,
    3239, 3240, 3241, 3242, 3243, 3244, 3170, 273, 273, 273, 273, 273, 273, 273, 273, 117, 177,
    2578, 2699, 189, 2579, 5746, 2700, 2809, 2810, 201, 2580, 5747, 2701, 5756, 5801, 2811, 2908,
    2909, 2910, 213, 2581, 5748, 2702, 5757, 5802, 2812, 5765, 5810, 5846, 2911, 2996, 2997, 2998,
    2999, 225, 2582, 5749, 2703, 5758, 5803, 2813, 5766, 5811, 5847, 2912, 5773, 5818, 5854, 5882,
    3000, 3073, 3074, 3075, 3076, 3077, 237, 2583, 5750, 2704, 5759, 5804, 2814, 5767, 5812, 5848,
    2913, 5774, 5819, 5855, 5883, 3001, 5780, 5825, 5861, 5889, 5910, 3078, 3139, 3140, 3141, 3142,
    3143, 3144, 249, 2584, 5751, 2705, 5760, 5805, 2815, 5768, 5813, 5849, 2914, 5775, 5820, 5856,
    5884, 3002, 5781, 5826, 5862, 5890, 5911, 3079, 5786, 5831, 5867, 5895, 5916, 5931, 3145, 3194,
    3195, 3196, 3197, 3198, 3199, 3200, 261, 2585, 5752, 2706, 5761, 5806, 2816, 5769, 5814, 5850,
    2915, 5776, 5821, 5857, 5885, 3003, 5782, 5827, 5863, 5891, 5912, 3080, 5787, 5832, 5868, 5896,
    5917, 5932, 3146, 5791, 5836, 5872, 5900, 5921, 5936, 5946, 3201, 3238, 3239, 3240, 3241, 3242,
    3243, 3244, 3245, 273, 2565, 3271, 2686, 3271, 3272, 2796, 3271, 3272, 3273, 2895, 3271, 3272,
    3273, 3274, 2983, 3271, 3272, 3273, 3274, 3275, 3060, 3271, 3272, 3273, 3274, 3275, 3276, 3126,
    3271, 3272, 3273, 3274, 3275, 3276, 3277, 3181, 3271, 3272, 3273, 3274, 3275, 3276, 3277, 3278,
    3225, 285, 285, 285, 285, 285, 285, 285, 285, 285, 129, 177, 2578, 2699, 189, 2579, 5746, 2700,
    2809, 2810, 201, 2580, 5747, 2701, 5756, 5801, 2811, 2908, 2909, 2910, 213, 2581, 5748, 2702,
    5757, 5802, 2812, 5765, 5810, 5846, 2911, 2996, 2997, 2998, 2999, 225, 2582, 5749, 2703, 5758,
    5803, 2813, 5766, 5811, 5847, 2912, 5773, 5818, 5854, 5882, 3000, 3073, 3074, 3075, 3076, 3077,
    237, 2583, 5750, 2704, 5759, 5804, 2814, 5767, 5812, 5848, 2913, 5774, 5819, 5855, 5883, 3001,
    5780, 5825, 5861, 5889, 5910, 3078, 3139, 3140, 3141, 3142, 3143, 3144, 249, 2584, 5751, 2705,
    5760, 5805, 2815, 5768, 5813, 5849, 2914, 5775, 5820, 5856, 5884, 3002, 5781, 5826, 5862, 5890,
    5911, 3079, 5786, 5831, 5867, 5895, 5916, 5931, 3145, 3194, 3195, 3196, 3197, 3198, 3199, 3200,
    261, 2585, 5752, 2706, 5761, 5806, 2816, 5769, 5814, 5850, 2915, 5776, 5821, 5857, 5885, 3003,
    5782, 5827, 5863, 5891, 5912, 3080, 5787, 5832, 5868, 5896, 5917, 5932, 3146, 5791, 5836, 5872,
    5900, 5921, 5936, 5946, 3201, 3238, 3239, 3240, 3241, 3242, 3243, 3244, 3245, 273, 1609, 1609,
    2707, 1609, 5807, 2817, 1609, 5815, 5851, 2916, 1609, 5822, 5858, 5886, 3004, 1609, 5828, 5864,
    5892, 5913, 3081, 1609, 5833, 5869, 5897, 5918, 5933, 3147, 1609, 5837, 5873, 5901, 5922, 5937,
    5947, 3202, 1608, 1608, 1608, 1608, 1608, 1608, 1608, 1607, 1608, 1609, 3272, 3273, 3274, 3275,
    3276, 3277, 3278, 1608, 285, 2576, 3293, 2697, 3293, 3294, 2807, 3293, 3294, 3295, 2906, 3293,
    3294, 3295, 3296, 2994, 3293, 3294, 3295, 3296, 3297, 3071, 3293, 3294, 3295, 3296, 3297, 3298,
    3137, 3293, 3294, 3295, 3296, 3297, 3298, 3299, 3192, 3293, 3294, 3295, 3296, 3297, 3298, 3299,
    3300, 3236, 1609, 3294, 3295, 3296, 3297, 3298, 3299, 3300, 1608, 3269, 297, 297, 297, 297,
    297, 297, 297, 297, 297, 297, 141, 177, 299, 300, 189, 299, 2336, 300, 301, 301, 201, 299,
    2336, 300, 2337, 2347, 301, 302, 302, 302, 213, 299, 2336, 300, 2337, 2347, 301, 2338, 2348,
    2357, 302, 303, 303, 303, 303, 225, 299, 2336, 300, 2337, 2347, 301, 2338, 2348, 2357, 302,
    2339, 2349, 2358, 2366, 303, 304, 304, 304, 304, 304, 237, 299, 2336, 300, 2337, 2347, 301,
    2338, 2348, 2357, 302, 2339, 2349, 2358, 2366, 303, 2340, 2350, 2359, 2367, 2374, 304, 305,
    305, 305, 305, 305, 305, 249, 299, 2336, 300, 2337, 2347, 301, 2338, 2348, 2357, 302, 2339,
    2349, 2358, 2366, 303, 2340, 2350, 2359, 2367, 2374, 304, 2341, 2351, 2360, 2368, 2375, 2381,
    305, 306, 306, 306, 306, 306, 306, 306, 261, 299, 2336, 300, 2337, 2347, 301, 2338, 2348, 2357,
    302, 2339, 2349, 2358, 2366, 303, 2340, 2350, 2359, 2367, 2374, 304, 2341, 2351, 2360, 2368,
    2375, 2381, 305, 2342, 2352, 2361, 2369, 2376, 2382, 2387, 306, 307, 307, 307, 307, 307, 307,
    307, 307, 273, 299, 2336, 300, 2337, 2347, 301, 2338, 2348, 2357, 302, 2339, 2349, 2358, 2366,
    303, 2340, 2350, 2359, 2367, 2374, 304, 2341, 2351, 2360, 2368, 2375, 2381, 305, 2342, 2352,
    2361, 2369, 2376, 2382, 2387, 306, 2343, 2353, 2362, 2370, 2377, 2383, 2388, 2392, 307, 308,
    308, 308, 308, 308, 308, 308, 308, 308, 285, 299, 2336, 300, 2337, 2347, 301, 2338, 2348, 2357,
    302, 2339, 2349, 2358, 2366, 303, 2340, 2350, 2359, 2367, 2374, 304, 2341, 2351, 2360, 2368,
    2375, 2381, 305, 2342, 2352, 2361, 2369, 2376, 2382, 2387, 306, 2343, 2353, 2362, 2370, 2377,
    2383, 2388, 2392, 307, 1609, 2354, 2363, 2371, 2378, 2384, 2389, 2393, 1608, 308, 309, 309,
    309, 309, 309, 309, 309, 309, 309, 309, 297, 143, 143, 144, 143, 144, 145, 143, 144, 145, 146,
    143, 144, 145, 146, 147, 143, 144, 145, 146, 147, 148, 143, 144, 145, 146, 147, 148, 149, 143,
    144, 145, 146, 147, 148, 149, 150, 143, 144, 145, 146, 147, 148, 149, 150, 151, 143, 144, 145,
    146, 147, 148, 149, 150, 151, 152, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 11,
    167, 179, 23, 12, 178, 2468, 190, 24, 168, 2479, 2600, 180, 191, 202, 192, 35, 36, 13, 178,
    2469, 190, 25, 178, 2589, 2710, 190, 2480, 2820, 2601, 202, 202, 37, 169, 2490, 2611, 181,
    2491, 2919, 2612, 2721, 2722, 193, 203, 214, 204, 214, 214, 205, 47, 48, 49, 14, 178, 2470,
    190, 26, 178, 2589, 2710, 190, 2481, 2820, 2602, 202, 202, 38, 178, 2589, 2710, 190, 2590,
    1600, 2711, 2820, 2821, 202, 2492, 2919, 2613, 2919, 2920, 2723, 214, 214, 214, 50, 170, 2501,
    2622, 182, 2502, 3007, 2623, 2732, 2733, 194, 2503, 3007, 2624, 3007, 3008, 2734, 2831, 2832,
    2833, 206, 215, 226, 216, 226, 226, 217, 226, 226, 226, 218, 59, 60, 61, 62, 15, 178, 2471,
    190, 27, 178, 2589, 2710, 190, 2482, 2820, 2603, 202, 202, 39, 178, 2589, 2710, 190, 2590,
    5966, 2711, 2820, 2821, 202, 2493, 2919, 2614, 2919, 2920, 2724, 214, 214, 214, 51, 178, 2589,
    2710, 190, 2590, 5966, 2711, 2820, 2821, 202, 2591, 5967, 2712, 5976, 1601, 2822, 2919, 2920,
    2921, 214, 2504, 3007, 2625, 3007, 3008, 2735, 3007, 3008, 3009, 2834, 226, 226, 226, 226, 63,
    171, 2512, 2633, 183, 2513, 3084, 2634, 2743, 2744, 195, 2514, 3084, 2635, 3084, 3085, 2745,
    2842, 2843, 2844, 207, 2515, 3084, 2636, 3084, 3085, 2746, 3084, 3085, 3086, 2845, 2930, 2931,
    2932, 2933, 219, 227, 238, 228, 238, 238, 229, 238, 238, 238, 230, 238, 238, 238, 238, 231, 71,
    72, 73, 74, 75, 16, 178, 2472, 190, 28, 178, 2589, 2710, 190, 2483, 2820, 2604, 202, 202, 40,
    178, 2589, 2710, 190, 2590, 5966, 2711, 2820, 2821, 202, 2494, 2919, 2615, 2919, 2920, 2725,
    214, 214, 214, 52, 178, 2589, 2710, 190, 2590, 5966, 2711, 2820, 2821, 202, 2591, 5967, 2712,
    5976, 6021, 2822, 2919, 2920, 2921, 214, 2505, 3007, 2626, 3007, 3008, 2736, 3007, 3008, 3009,
    2835, 226, 226, 226, 226, 64, 178, 2589, 2710, 190, 2590, 5966, 2711, 2820, 2821, 202, 2591,
    5967, 2712, 5976, 6021, 2822, 2919, 2920, 2921, 214, 2592, 5968, 2713, 5977, 6022, 2823, 5985,
    6030, 1602, 2922, 3007, 3008, 3009, 3010, 226, 2516, 3084, 2637, 3084, 3085, 2747, 3084, 3085,
    3086, 2846, 3084, 3085, 3086, 3087, 2934, 238, 238, 238, 238, 238, 76, 172, 2523, 2644, 184,
    2524, 3150, 2645, 2754, 2755, 196, 2525, 3150, 2646, 3150, 3151, 2756, 2853, 2854, 2855, 208,
    2526, 3150, 2647, 3150, 3151, 2757, 3150, 3151, 3152, 2856, 2941, 2942, 2943, 2944, 220, 2527,
    3150, 2648, 3150, 3151, 2758, 3150, 3151, 3152, 2857, 3150, 3151, 3152, 3153, 2945, 3018, 3019,
    3020, 3021, 3022, 232, 239, 250, 240, 250, 250, 241, 250, 250, 250, 242, 250, 250, 250, 250,
    243, 250, 250, 250, 250, 250, 244, 83, 84, 85, 86, 87, 88, 17, 178, 2473, 190, 29, 178, 2589,
    2710, 190, 2484, 2820, 2605, 202, 202, 41, 178, 2589, 2710, 190, 2590, 5966, 2711, 2820, 2821,
    202, 2495, 2919, 2616, 2919, 2920, 2726, 214, 214, 214, 53, 178, 2589, 2710, 190, 2590, 5966,
    2711, 2820, 2821, 202, 2591, 5967, 2712, 5976, 6021, 2822, 2919, 2920, 2921, 214, 2506, 3007,
    2627, 3007, 3008, 2737, 3007, 3008, 3009, 2836, 226, 226, 226, 226, 65, 178, 2589, 2710, 190,
    2590, 5966, 2711, 2820, 2821, 202, 2591, 5967, 2712, 5976, 6021, 2822, 2919, 2920, 2921, 214,
    2592, 5968, 2713, 5977, 6022, 2823, 5985, 6030, 6066, 2922, 3007, 3008, 3009, 3010, 226, 2517,
    3084, 2638, 3084, 3085, 2748, 3084, 3085, 3086, 2847, 3084, 3085, 3086, 3087, 2935, 238, 238,
    238, 238, 238, 77, 178, 2589, 2710, 190, 2590, 5966, 2711, 2820, 2821, 202, 2591, 5967, 2712,
    5976, 6021, 2822, 2919, 2920, 2921, 214, 2592, 5968, 2713, 5977, 6022, 2823, 5985, 6030, 6066,
    2922, 3007, 3008, 3009, 3010, 226, 2593, 5969, 2714, 5978, 6023, 2824, 5986, 6031, 6067, 2923,
    5993, 6038, 6074, 1603, 3011, 3084, 3085, 3086, 3087, 3088, 238, 2528, 3150, 2649, 3150, 3151,
    2759, 3150, 3151, 3152, 2858, 3150, 3151, 3152, 3153, 2946, 3150, 3151, 3152, 3153, 3154, 3023,
    250, 250, 250, 250, 250, 250, 89, 173, 2534, 2655, 185, 2535, 3205, 2656, 2765, 2766, 197,
    2536, 3205, 2657, 3205, 3206, 2767, 2864, 2865, 2866, 209, 2537, 3205, 2658, 3205, 3206, 2768,
    3205, 3206, 3207, 2867, 2952, 2953, 2954, 2955, 221, 2538, 3205, 2659, 3205, 3206, 2769, 3205,
    3206, 3207, 2868, 3205, 3206, 3207, 3208, 2956, 3029, 3030, 3031, 3032, 3033, 233, 2539, 3205,
    2660, 3205, 3206, 2770, 3205, 3206, 3207, 2869, 3205, 3206, 3207, 3208, 2957, 3205, 3206, 3207,
    3208, 3209, 3034, 3095, 3096, 3097, 3098, 3099, 3100, 245, 251, 262, 252, 262, 262, 253, 262,
    262, 262, 254, 262, 262, 262, 262, 255, 262, 262, 262, 262, 262, 256, 262, 262, 262, 262, 262,
    262, 257, 95, 96, 97, 98, 99, 100, 101, 18, 178, 2474, 190, 30, 178, 2589, 2710, 190, 2485,
    2820, 2606, 202, 202, 42, 178, 2589, 2710, 190, 2590, 5966, 2711, 2820, 2821, 202, 2496, 2919,
    2617, 2919, 2920, 2727, 214, 214, 214, 54, 178, 2589, 2710, 190, 2590, 5966, 2711, 2820, 2821,
    202, 2591, 5967, 2712, 5976, 6021, 2822, 2919, 2920, 2921, 214, 2507, 3007, 2628, 3007, 3008,
    2738, 3007, 3008, 3009, 2837, 226, 226, 226, 226, 66, 178, 2589, 2710, 190, 2590, 5966, 2711,
    2820, 2821, 202, 2591, 5967, 2712, 5976, 6021, 2822, 2919, 2920, 2921, 214, 2592, 5968, 2713,
    5977, 6022, 2823, 5985, 6030, 6066, 2922, 3007, 3008, 3009, 3010, 226, 2518, 3084, 2639, 3084,
    3085, 2749, 3084, 3085, 3086, 2848, 3084, 3085, 3086, 3087, 2936, 238, 238, 238, 238, 238, 78,
    178, 2589, 2710, 190, 2590, 5966, 2711, 2820, 2821, 202, 2591, 5967, 2712, 5976, 6021, 2822,
    2919, 2920, 2921, 214, 2592, 5968, 2713, 5977, 6022, 2823, 5985, 6030, 6066, 2922, 3007, 3008,
    3009, 3010, 226, 2593, 5969, 2714, 5978, 6023, 2824, 5986, 6031, 6067, 2923, 5993, 6038, 6074,
    6102, 3011, 3084, 3085, 3086, 3087, 3088, 238, 2529, 3150, 2650, 3150, 3151, 2760, 3150, 3151,
    3152, 2859, 3150, 3151, 3152, 3153, 2947, 3150, 3151, 3152, 3153, 3154, 3024, 250, 250, 250,
    250, 250, 250, 90, 178, 2589, 2710, 190, 2590, 5966, 2711, 2820, 2821, 202, 2591, 5967, 2712,
    5976, 6021, 2822, 2919, 2920, 2921, 214, 2592, 5968, 2713, 5977, 6022, 2823, 5985, 6030, 6066,
    2922, 3007, 3008, 3009, 3010, 226, 2593, 5969, 2714, 5978, 6023, 2824, 5986, 6031, 6067, 2923,
    5993, 6038, 6074, 6102, 3011, 3084, 3085, 3086, 3087, 3088, 238, 2594, 5970, 2715, 5979, 6024,
    2825, 5987, 6032, 6068, 2924, 5994, 6039, 6075, 6103, 3012, 6000, 6045, 6081, 6109, 1604, 3089,
    3150, 3151, 3152, 3153, 3154, 3155, 250, 2540, 3205, 2661, 3205, 3206, 2771, 3205, 3206, 3207,
    2870, 3205, 3206, 3207, 3208, 2958, 3205, 3206, 3207, 3208, 3209, 3035, 3205, 3206, 3207, 3208,
    3209, 3210, 3101, 262, 262, 262, 262, 262, 262, 262, 102, 174, 2545, 2666, 186, 2546, 3249,
    2667, 2776, 2777, 198, 2547, 3249, 2668, 3249, 3250, 2778, 2875, 2876, 2877, 210, 2548, 3249,
    2669, 3249, 3250, 2779, 3249, 3250, 3251, 2878, 2963, 2964, 2965, 2966, 222, 2549, 3249, 2670,
    3249, 3250, 2780, 3249, 3250, 3251, 2879, 3249, 3250, 3251, 3252, 2967, 3040, 3041, 3042, 3043,
    3044, 234, 2550, 3249, 2671, 3249, 3250, 2781, 3249, 3250, 3251, 2880, 3249, 3250, 3251, 3252,
    2968, 3249, 3250, 3251, 3252, 3253, 3045, 3106, 3107, 3108, 3109, 3110, 3111, 246, 2551, 3249,
    2672, 3249, 3250, 2782, 3249, 3250, 3251, 2881, 3249, 3250, 3251, 3252, 2969, 3249, 3250, 3251,
    3252, 3253, 3046, 3249, 3250, 3251, 3252, 3253, 3254, 3112, 3161, 3162, 3163, 3164, 3165, 3166,
    3167, 258, 263, 274, 264, 274, 274, 265, 274, 274, 274, 266, 274, 274, 274, 274, 267, 274, 274,
    274, 274, 274, 268, 274, 274, 274, 274, 274, 274, 269, 274, 274, 274, 274, 274, 274, 274, 270,
    107, 108, 109, 110, 111, 112, 113, 114, 19, 178, 2475, 190, 31, 178, 2589, 2710, 190, 2486,
    2820, 2607, 202, 202, 43, 178, 2589, 2710, 190, 2590, 5966, 2711, 2820, 2821, 202, 2497, 2919,
    2618, 2919, 2920, 2728, 214, 214, 214, 55, 178, 2589, 2710, 190, 2590, 5966, 2711, 2820, 2821,
    202, 2591, 5967, 2712, 5976, 6021, 2822, 2919, 2920, 2921, 214, 2508, 3007, 2629, 3007, 3008,
    2739, 3007, 3008, 3009, 2838, 226, 226, 226, 226, 67, 178, 2589, 2710, 190, 2590, 5966, 2711,
    2820, 2821, 202, 2591, 5967, 2712, 5976, 6021, 2822, 2919, 2920, 2921, 214, 2592, 5968, 2713,
    5977, 6022, 2823, 5985, 6030, 6066, 2922, 3007, 3008, 3009, 3010, 226, 2519, 3084, 2640, 3084,
    3085, 2750, 3084, 3085, 3086, 2849, 3084, 3085, 3086, 3087, 2937, 238, 238, 238, 238, 238, 79,
    178, 2589, 2710, 190, 2590, 5966, 2711, 2820, 2821, 202, 2591, 5967, 2712, 5976, 6021, 2822,
    2919, 2920, 2921, 214, 2592, 5968, 2713, 5977, 6022, 2823, 5985, 6030, 6066, 2922, 3007, 3008,
    3009, 3010, 226, 2593, 5969, 2714, 5978, 6023, 2824, 5986, 6031, 6067, 2923, 5993, 6038, 6074,
    6102, 3011, 3084, 3085, 3086, 3087, 3088, 238, 2530, 3150, 2651, 3150, 3151, 2761, 3150, 3151,
    3152, 2860, 3150, 3151, 3152, 3153, 2948, 3150, 3151, 3152, 3153, 3154, 3025, 250, 250, 250,
    250, 250, 250, 91, 178, 2589, 2710, 190, 2590, 5966, 2711, 2820, 2821, 202, 2591, 5967, 2712,
    5976, 6021, 2822, 2919, 2920, 2921, 214, 2592, 5968, 2713, 5977, 6022, 2823, 5985, 6030, 6066,
    2922, 3007, 3008, 3009, 3010, 226, 2593, 5969, 2714, 5978, 6023, 2824, 5986, 6031, 6067, 2923,
    5993, 6038, 6074, 6102, 3011, 3084, 3085, 3086, 3087, 3088, 238, 2594, 5970, 2715, 5979, 6024,
    2825, 5987, 6032, 6068, 2924, 5994, 6039, 6075, 6103, 3012, 6000, 6045, 6081, 6109, 6130, 3089,
    3150, 3151, 3152, 3153, 3154, 3155, 250, 2541, 3205, 2662, 3205, 3206, 2772, 3205, 3206, 3207,
    2871, 3205, 3206, 3207, 3208, 2959, 3205, 3206, 3207, 3208, 3209, 3036, 3205, 3206, 3207, 3208,
    3209, 3210, 3102, 262, 262, 262, 262, 262, 262, 262, 103, 178, 2589, 2710, 190, 2590, 5966,
    2711, 2820, 2821, 202, 2591, 5967, 2712, 5976, 6021, 2822, 2919, 2920, 2921, 214, 2592, 5968,
    2713, 5977, 6022, 2823, 5985, 6030, 6066, 2922, 3007, 3008, 3009, 3010, 226, 2593, 5969, 2714,
    5978, 6023, 2824, 5986, 6031, 6067, 2923, 5993, 6038, 6074, 6102, 3011, 3084, 3085, 3086, 3087,
    3088, 238, 2594, 5970, 2715, 5979, 6024, 2825, 5987, 6032, 6068, 2924, 5994, 6039, 6075, 6103,
    3012, 6000, 6045, 6081, 6109, 6130, 3089, 3150, 3151, 3152, 3153, 3154, 3155, 250, 2595, 5971,
    2716, 5980, 6025, 2826, 5988, 6033, 6069, 2925, 5995, 6040, 6076, 6104, 3013, 6001, 6046, 6082,
    6110, 6131, 3090, 6006, 6051, 6087, 6115, 6136, 1605, 3156, 3205, 3206, 3207, 3208, 3209, 3210,
    3211, 262, 2552, 3249, 2673, 3249, 3250, 2783, 3249, 3250, 3251, 2882, 3249, 3250, 3251, 3252,
    2970, 3249, 3250, 3251, 3252, 3253, 3047, 3249, 3250, 3251, 3252, 3253, 3254, 3113, 3249, 3250,
    3251, 3252, 3253, 3254, 3255, 3168, 274, 274, 274, 274, 274, 274, 274, 274, 115, 175, 2556,
    2677, 187, 2557, 3282, 2678, 2787, 2788, 199, 2558, 3282, 2679, 3282, 3283, 2789, 2886, 2887,
    2888, 211, 2559, 3282, 2680, 3282, 3283, 2790, 3282, 3283, 3284, 2889, 2974, 2975, 2976, 2977,
    223, 2560, 3282, 2681, 3282, 3283, 2791, 3282, 3283, 3284, 2890, 3282, 3283, 3284, 3285, 2978,
    3051, 3052, 3053, 3054, 3055, 235, 2561, 3282, 2682, 3282, 3283, 2792, 3282, 3283, 3284, 2891,
    3282, 3283, 3284, 3285, 2979, 3282, 3283, 3284, 3285, 3286, 3056, 3117, 3118, 3119, 3120, 3121,
    3122, 247, 2562, 3282, 2683, 3282, 3283, 2793, 3282, 3283, 3284, 2892, 3282, 3283, 3284, 3285,
    2980, 3282, 3283, 3284, 3285, 3286, 3057, 3282, 3283, 3284, 3285, 3286, 3287, 3123, 3172, 3173,
    3174, 3175, 3176, 3177, 3178, 259, 2563, 3282, 2684, 3282, 3283, 2794, 3282, 3283, 3284, 2893,
    3282, 3283, 3284, 3285, 2981, 3282, 3283, 3284, 3285, 3286, 3058, 3282, 3283, 3284, 3285, 3286,
    3287, 3124, 3282, 3283, 3284, 3285, 3286, 3287, 3288, 3179, 3216, 3217, 3218, 3219, 3220, 3221,
    3222, 3223, 271, 275, 286, 276, 286, 286, 277, 286, 286, 286, 278, 286, 286, 286, 286, 279,
    286, 286, 286, 286, 286, 280, 286, 286, 286, 286, 286, 286, 281, 286, 286, 286, 286, 286, 286,
    286, 282, 286, 286, 286, 286, 286, 286, 286, 286, 283, 119, 120, 121, 122, 123, 124, 125, 126,
    127, 20, 178, 2476, 190, 32, 178, 2589, 2710, 190, 2487, 2820, 2608, 202, 202, 44, 178, 2589,
    2710, 190, 2590, 5966, 2711, 2820, 2821, 202, 2498, 2919, 2619, 2919, 2920, 2729, 214, 214,
    214, 56, 178, 2589, 2710, 190, 2590, 5966, 2711, 2820, 2821, 202, 2591, 5967, 2712, 5976, 6021,
    2822, 2919, 2920, 2921, 214, 2509, 3007, 2630, 3007, 3008, 2740, 3007, 3008, 3009, 2839, 226,
    226, 226, 226, 68, 178, 2589, 2710, 190, 2590, 5966, 2711, 2820, 2821, 202, 2591, 5967, 2712,
    5976, 6021, 2822, 2919, 2920, 2921, 214, 2592, 5968, 2713, 5977, 6022, 2823, 5985, 6030, 6066,
    2922, 3007, 3008, 3009, 3010, 226, 2520, 3084, 2641, 3084, 3085, 2751, 3084, 3085, 3086, 2850,
    3084, 3085, 3086, 3087, 2938, 238, 238, 238, 238, 238, 80, 178, 2589, 2710, 190, 2590, 5966,
    2711, 2820, 2821, 202, 2591, 5967, 2712, 5976, 6021, 2822, 2919, 2920, 2921, 214, 2592, 5968,
    2713, 5977, 6022, 2823, 5985, 6030, 6066, 2922, 3007, 3008, 3009, 3010, 226, 2593, 5969, 2714,
    5978, 6023, 2824, 5986, 6031, 6067, 2923, 5993, 6038, 6074, 6102, 3011, 3084, 3085, 3086, 3087,
    3088, 238, 2531, 3150, 2652, 3150, 3151, 2762, 3150, 3151, 3152, 2861, 3150, 3151, 3152, 3153,
    2949, 3150, 3151, 3152, 3153, 3154, 3026, 250, 250, 250, 250, 250, 250, 92, 178, 2589, 2710,
    190, 2590, 5966, 2711, 2820, 2821, 202, 2591, 5967, 2712, 5976, 6021, 2822, 2919, 2920, 2921,
    214, 2592, 5968, 2713, 5977, 6022, 2823, 5985, 6030, 6066, 2922, 3007, 3008, 3009, 3010, 226,
    2593, 5969, 2714, 5978, 6023, 2824, 5986, 6031, 6067, 2923, 5993, 6038, 6074, 6102, 3011, 3084,
    3085, 3086, 3087, 3088, 238, 2594, 5970, 2715, 5979, 6024, 2825, 5987, 6032, 6068, 2924, 5994,
    6039, 6075, 6103, 3012, 6000, 6045, 6081, 6109, 6130, 3089, 3150, 3151, 3152, 3153, 3154, 3155,
    250, 2542, 3205, 2663, 3205, 3206, 2773, 3205, 3206, 3207, 2872, 3205, 3206, 3207, 3208, 2960,
    3205, 3206, 3207, 3208, 3209, 3037, 3205, 3206, 3207, 3208, 3209, 3210, 3103, 262, 262, 262,
    262, 262, 262, 262, 104, 178, 2589, 2710, 190, 2590, 5966, 2711, 2820, 2821, 202, 2591, 5967,
    2712, 5976, 6021, 2822, 2919, 2920, 2921, 214, 2592, 5968, 2713, 5977, 6022, 2823, 5985, 6030,
    6066, 2922, 3007, 3008, 3009, 3010, 226, 2593, 5969, 2714, 5978, 6023, 2824, 5986, 6031, 6067,
    2923, 5993, 6038, 6074, 6102, 3011, 3084, 3085, 3086, 3087, 3088, 238, 2594, 5970, 2715, 5979,
    6024, 2825, 5987, 6032, 6068, 2924, 5994, 6039, 6075, 6103, 3012, 6000, 6045, 6081, 6109, 6130,
    3089, 3150, 3151, 3152, 3153, 3154, 3155, 250, 2595, 5971, 2716, 5980, 6025, 2826, 5988, 6033,
    6069, 2925, 5995, 6040, 6076, 6104, 3013, 6001, 6046, 6082, 6110, 6131, 3090, 6006, 6051, 6087,
    6115, 6136, 6151, 3156, 3205, 3206, 3207, 3208, 3209, 3210, 3211, 262, 2553, 3249, 2674, 3249,
    3250, 2784, 3249, 3250, 3251, 2883, 3249, 3250, 3251, 3252, 2971, 3249, 3250, 3251, 3252, 3253,
    3048, 3249, 3250, 3251, 3252, 3253, 3254, 3114, 3249, 3250, 3251, 3252, 3253, 3254, 3255, 3169,
    274, 274, 274, 274, 274, 274, 274, 274, 116, 178, 2589, 2710, 190, 2590, 5966, 2711, 2820,
    2821, 202, 2591, 5967, 2712, 5976, 6021, 2822, 2919, 2920, 2921, 214, 2592, 5968, 2713, 5977,
    6022, 2823, 5985, 6030, 6066, 2922, 3007, 3008, 3009, 3010, 226, 2593, 5969, 2714, 5978, 6023,
    2824, 5986, 6031, 6067, 2923, 5993, 6038, 6074, 6102, 3011, 3084, 3085, 3086, 3087, 3088, 238,
    2594, 5970, 2715, 5979, 6024, 2825, 5987, 6032, 6068, 2924, 5994, 6039, 6075, 6103, 3012, 6000,
    6045, 6081, 6109, 6130, 3089, 3150, 3151, 3152, 3153, 3154, 3155, 250, 2595, 5971, 2716, 5980,
    6025, 2826, 5988, 6033, 6069, 2925, 5995, 6040, 6076, 6104, 3013, 6001, 6046, 6082, 6110, 6131,
    3090, 6006, 6051, 6087, 6115, 6136, 6151, 3156, 3205, 3206, 3207, 3208, 3209, 3210, 3211, 262,
    2596, 5972, 2717, 5981, 6026, 2827, 5989, 6034, 6070, 2926, 5996, 6041, 6077, 6105, 3014, 6002,
    6047, 6083, 6111, 6132, 3091, 6007, 6052, 6088, 6116, 6137, 6152, 3157, 6011, 6056, 6092, 6120,
    6141, 6156, 1606, 3212, 3249, 3250, 3251, 3252, 3253, 3254, 3255, 3256, 274, 2564, 3282, 2685,
    3282, 3283, 2795, 3282, 3283, 3284, 2894, 3282, 3283, 3284, 3285, 2982, 3282, 3283, 3284, 3285,
    3286, 3059, 3282, 3283, 3284, 3285, 3286, 3287, 3125, 3282, 3283, 3284, 3285, 3286, 3287, 3288,
    3180, 3282, 3283, 3284, 3285, 3286, 3287, 3288, 3289, 3224, 286, 286, 286, 286, 286, 286, 286,
    286, 286, 128, 176, 2567, 2688, 188, 2568, 3304, 2689, 2798, 2799, 200, 2569, 3304, 2690, 3304,
    3305, 2800, 2897, 2898, 2899, 212, 2570, 3304, 2691, 3304, 3305, 2801, 3304, 3305, 3306, 2900,
    2985, 2986, 2987, 2988, 224, 2571, 3304, 2692, 3304, 3305, 2802, 3304, 3305, 3306, 2901, 3304,
    3305, 3306, 3307, 2989, 3062, 3063, 3064, 3065, 3066, 236, 2572, 3304, 2693, 3304, 3305, 2803,
    3304, 3305, 3306, 2902, 3304, 3305, 3306, 3307, 2990, 3304, 3305, 3306, 3307, 3308, 3067, 3128,
    3129, 3130, 3131, 3132, 3133, 248, 2573, 3304, 2694, 3304, 3305, 2804, 3304, 3305, 3306, 2903,
    3304, 3305, 3306, 3307, 2991, 3304, 3305, 3306, 3307, 3308, 3068, 3304, 3305, 3306, 3307, 3308,
    3309, 3134, 3183, 3184, 3185, 3186, 3187, 3188, 3189, 260, 2574, 3304, 2695, 3304, 3305, 2805,
    3304, 3305, 3306, 2904, 3304, 3305, 3306, 3307, 2992, 3304, 3305, 3306, 3307, 3308, 3069, 3304,
    3305, 3306, 3307, 3308, 3309, 3135, 3304, 3305, 3306, 3307, 3308, 3309, 3310, 3190, 3227, 3228,
    3229, 3230, 3231, 3232, 3233, 3234, 272, 2575, 3304, 2696, 3304, 3305, 2806, 3304, 3305, 3306,
    2905, 3304, 3305, 3306, 3307, 2993, 3304, 3305, 3306, 3307, 3308, 3070, 3304, 3305, 3306, 3307,
    3308, 3309, 3136, 3304, 3305, 3306, 3307, 3308, 3309, 3310, 3191, 3304, 3305, 3306, 3307, 3308,
    3309, 3310, 3311, 3235, 3260, 3261, 3262, 3263, 3264, 3265, 3266, 3267, 3268, 284, 287, 298,
    288, 298, 298, 289, 298, 298, 298, 290, 298, 298, 298, 298, 291, 298, 298, 298, 298, 298, 292,
    298, 298, 298, 298, 298, 298, 293, 298, 298, 298, 298, 298, 298, 298, 294, 298, 298, 298, 298,
    298, 298, 298, 298, 295, 298, 298, 298, 298, 298, 298, 298, 298, 298, 296, 131, 132, 133, 134,
    135, 136, 137, 138, 139, 140, 21, 178, 2477, 190, 33, 178, 2589, 2710, 190, 2488, 2820, 2609,
    202, 202, 45, 178, 2589, 2710, 190, 2590, 5966, 2711, 2820, 2821, 202, 2499, 2919, 2620, 2919,
    2920, 2730, 214, 214, 214, 57, 178, 2589, 2710, 190, 2590, 5966, 2711, 2820, 2821, 202, 2591,
    5967, 2712, 5976, 6021, 2822, 2919, 2920, 2921, 214, 2510, 3007, 2631, 3007, 3008, 2741, 3007,
    3008, 3009, 2840, 226, 226, 226, 226, 69, 178, 2589, 2710, 190, 2590, 5966, 2711, 2820, 2821,
    202, 2591, 5967, 2712, 5976, 6021, 2822, 2919, 2920, 2921, 214, 2592, 5968, 2713, 5977, 6022,
    2823, 5985, 6030, 6066, 2922, 3007, 3008, 3009, 3010, 226, 2521, 3084, 2642, 3084, 3085, 2752,
    3084, 3085, 3086, 2851, 3084, 3085, 3086, 3087, 2939, 238, 238, 238, 238, 238, 81, 178, 2589,
    2710, 190, 2590, 5966, 2711, 2820, 2821, 202, 2591, 5967, 2712, 5976, 6021, 2822, 2919, 2920,
    2921, 214, 2592, 5968, 2713, 5977, 6022, 2823, 5985, 6030, 6066, 2922, 3007, 3008, 3009, 3010,
    226, 2593, 5969, 2714, 5978, 6023, 2824, 5986, 6031, 6067, 2923, 5993, 6038, 6074, 6102, 3011,
    3084, 3085, 3086, 3087, 3088, 238, 2532, 3150, 2653, 3150, 3151, 2763, 3150, 3151, 3152, 2862,
    3150, 3151, 3152, 3153, 2950, 3150, 3151, 3152, 3153, 3154, 3027, 250, 250, 250, 250, 250, 250,
    93, 178, 2589, 2710, 190, 2590, 5966, 2711, 2820, 2821, 202, 2591, 5967, 2712, 5976, 6021,
    2822, 2919, 2920, 2921, 214, 2592, 5968, 2713, 5977, 6022, 2823, 5985, 6030, 6066, 2922, 3007,
    3008, 3009, 3010, 226, 2593, 5969, 2714, 5978, 6023, 2824, 5986, 6031, 6067, 2923, 5993, 6038,
    6074, 6102, 3011, 3084, 3085, 3086, 3087, 3088, 238, 2594, 5970, 2715, 5979, 6024, 2825, 5987,
    6032, 6068, 2924, 5994, 6039, 6075, 6103, 3012, 6000, 6045, 6081, 6109, 6130, 3089, 3150, 3151,
    3152, 3153, 3154, 3155, 250, 2543, 3205, 2664, 3205, 3206, 2774, 3205, 3206, 3207, 2873, 3205,
    3206, 3207, 3208, 2961, 3205, 3206, 3207, 3208, 3209, 3038, 3205, 3206, 3207, 3208, 3209, 3210,
    3104, 262, 262, 262, 262, 262, 262, 262, 105, 178, 2589, 2710, 190, 2590, 5966, 2711, 2820,
    2821, 202, 2591, 5967, 2712, 5976, 6021, 2822, 2919, 2920, 2921, 214, 2592, 5968, 2713, 5977,
    6022, 2823, 5985, 6030, 6066, 2922, 3007, 3008, 3009, 3010, 226, 2593, 5969, 2714, 5978, 6023,
    2824, 5986, 6031, 6067, 2923, 5993, 6038, 6074, 6102, 3011, 3084, 3085, 3086, 3087, 3088, 238,
    2594, 5970, 2715, 5979, 6024, 2825, 5987, 6032, 6068, 2924, 5994, 6039, 6075, 6103, 3012, 6000,
    6045, 6081, 6109, 6130, 3089, 3150, 3151, 3152, 3153, 3154, 3155, 250, 2595, 5971, 2716, 5980,
    6025, 2826, 5988, 6033, 6069, 2925, 5995, 6040, 6076, 6104, 3013, 6001, 6046, 6082, 6110, 6131,
    3090, 6006, 6051, 6087, 6115, 6136, 6151, 3156, 3205, 3206, 3207, 3208, 3209, 3210, 3211, 262,
    2554, 3249, 2675, 3249, 3250, 2785, 3249, 3250, 3251, 2884, 3249, 3250, 3251, 3252, 2972, 3249,
    3250, 3251, 3252, 3253, 3049, 3249, 3250, 3251, 3252, 3253, 3254, 3115, 3249, 3250, 3251, 3252,
    3253, 3254, 3255, 3170, 274, 274, 274, 274, 274, 274, 274, 274, 117, 178, 2589, 2710, 190,
    2590, 5966, 2711, 2820, 2821, 202, 2591, 5967, 2712, 5976, 6021, 2822, 2919, 2920, 2921, 214,
    2592, 5968, 2713, 5977, 6022, 2823, 5985, 6030, 6066, 2922, 3007, 3008, 3009, 3010, 226, 2593,
    5969, 2714, 5978, 6023, 2824, 5986, 6031, 6067, 2923, 5993, 6038, 6074, 6102, 3011, 3084, 3085,
    3086, 3087, 3088, 238, 2594, 5970, 2715, 5979, 6024, 2825, 5987, 6032, 6068, 2924, 5994, 6039,
    6075, 6103, 3012, 6000, 6045, 6081, 6109, 6130, 3089, 3150, 3151, 3152, 3153, 3154, 3155, 250,
    2595, 5971, 2716, 5980, 6025, 2826, 5988, 6033, 6069, 2925, 5995, 6040, 6076, 6104, 3013, 6001,
    6046, 6082, 6110, 6131, 3090, 6006, 6051, 6087, 6115, 6136, 6151, 3156, 3205, 3206, 3207, 3208,
    3209, 3210, 3211, 262, 2596, 5972, 2717, 5981, 6026, 2827, 5989, 6034, 6070, 2926, 5996, 6041,
    6077, 6105, 3014, 6002, 6047, 6083, 6111, 6132, 3091, 6007, 6052, 6088, 6116, 6137, 6152, 3157,
    6011, 6056, 6092, 6120, 6141, 6156, 6166, 3212, 3249, 3250, 3251, 3252, 3253, 3254, 3255, 3256,
    274, 2565, 3282, 2686, 3282, 3283, 2796, 3282, 3283, 3284, 2895, 3282, 3283, 3284, 3285, 2983,
    3282, 3283, 3284, 3285, 3286, 3060, 3282, 3283, 3284, 3285, 3286, 3287, 3126, 3282, 3283, 3284,
    3285, 3286, 3287, 3288, 3181, 3282, 3283, 3284, 3285, 3286, 3287, 3288, 3289, 3225, 286, 286,
    286, 286, 286, 286, 286, 286, 286, 129, 178, 2589, 2710, 190, 2590, 5966, 2711, 2820, 2821,
    202, 2591, 5967, 2712, 5976, 6021, 2822, 2919, 2920, 2921, 214, 2592, 5968, 2713, 5977, 6022,
    2823, 5985, 6030, 6066, 2922, 3007, 3008, 3009, 3010, 226, 2593, 5969, 2714, 5978, 6023, 2824,
    5986, 6031, 6067, 2923, 5993, 6038, 6074, 6102, 3011, 3084, 3085, 3086, 3087, 3088, 238, 2594,
    5970, 2715, 5979, 6024, 2825, 5987, 6032, 6068, 2924, 5994, 6039, 6075, 6103, 3012, 6000, 6045,
    6081, 6109, 6130, 3089, 3150, 3151, 3152, 3153, 3154, 3155, 250, 2595, 5971, 2716, 5980, 6025,
    2826, 5988, 6033, 6069, 2925, 5995, 6040, 6076, 6104, 3013, 6001, 6046, 6082, 6110, 6131, 3090,
    6006, 6051, 6087, 6115, 6136, 6151, 3156, 3205, 3206, 3207, 3208, 3209, 3210, 3211, 262, 2596,
    5972, 2717, 5981, 6026, 2827, 5989, 6034, 6070, 2926, 5996, 6041, 6077, 6105, 3014, 6002, 6047,
    6083, 6111, 6132, 3091, 6007, 6052, 6088, 6116, 6137, 6152, 3157, 6011, 6056, 6092, 6120, 6141,
    6156, 6166, 3212, 3249, 3250, 3251, 3252, 3253, 3254, 3255, 3256, 274, 1609, 1609, 2718, 1609,
    6027, 2828, 1609, 6035, 6071, 2927, 1609, 6042, 6078, 6106, 3015, 1609, 6048, 6084, 6112, 6133,
    3092, 1609, 6053, 6089, 6117, 6138, 6153, 3158, 1609, 6057, 6093, 6121, 6142, 6157, 6167, 3213,
    1608, 1608, 1608, 1608, 1608, 1608, 1608, 1607, 1608, 1609, 3283, 3284, 3285, 3286, 3287, 3288,
    3289, 1608, 286, 2576, 3304, 2697, 3304, 3305, 2807, 3304, 3305, 3306, 2906, 3304, 3305, 3306,
    3307, 2994, 3304, 3305, 3306, 3307, 3308, 3071, 3304, 3305, 3306, 3307, 3308, 3309, 3137, 3304,
    3305, 3306, 3307, 3308, 3309, 3310, 3192, 3304, 3305, 3306, 3307, 3308, 3309, 3310, 3311, 3236,
    1609, 3305, 3306, 3307, 3308, 3309, 3310, 3311, 1608, 3269, 298, 298, 298, 298, 298, 298, 298,
    298, 298, 298, 141, 177, 2578, 2699, 189, 2579, 3315, 2700, 2809, 2810, 201, 2580, 3315, 2701,
    3315, 3316, 2811, 2908, 2909, 2910, 213, 2581, 3315, 2702, 3315, 3316, 2812, 3315, 3316, 3317,
    2911, 2996, 2997, 2998, 2999, 225, 2582, 3315, 2703, 3315, 3316, 2813, 3315, 3316, 3317, 2912,
    3315, 3316, 3317, 3318, 3000, 3073, 3074, 3075, 3076, 3077, 237, 2583, 3315, 2704, 3315, 3316,
    2814, 3315, 3316, 3317, 2913, 3315, 3316, 3317, 3318, 3001, 3315, 3316, 3317, 3318, 3319, 3078,
    3139, 3140, 3141, 3142, 3143, 3144, 249, 2584, 3315, 2705, 3315, 3316, 2815, 3315, 3316, 3317,
    2914, 3315, 3316, 3317, 3318, 3002, 3315, 3316, 3317, 3318, 3319, 3079, 3315, 3316, 3317, 3318,
    3319, 3320, 3145, 3194, 3195, 3196, 3197, 3198, 3199, 3200, 261, 2585, 3315, 2706, 3315, 3316,
    2816, 3315, 3316, 3317, 2915, 3315, 3316, 3317, 3318, 3003, 3315, 3316, 3317, 3318, 3319, 3080,
    3315, 3316, 3317, 3318, 3319, 3320, 3146, 3315, 3316, 3317, 3318, 3319, 3320, 3321, 3201, 3238,
    3239, 3240, 3241, 3242, 3243, 3244, 3245, 273, 2586, 3315, 2707, 3315, 3316, 2817, 3315, 3316,
    3317, 2916, 3315, 3316, 3317, 3318, 3004, 3315, 3316, 3317, 3318, 3319, 3081, 3315, 3316, 3317,
    3318, 3319, 3320, 3147, 3315, 3316, 3317, 3318, 3319, 3320, 3321, 3202, 3315, 3316, 3317, 3318,
    3319, 3320, 3321, 3322, 3246, 3271, 3272, 3273, 3274, 3275, 3276, 3277, 3278, 3279, 285, 2587,
    3315, 2708, 3315, 3316, 2818, 3315, 3316, 3317, 2917, 3315, 3316, 3317, 3318, 3005, 3315, 3316,
    3317, 3318, 3319, 3082, 3315, 3316, 3317, 3318, 3319, 3320, 3148, 3315, 3316, 3317, 3318, 3319,
    3320, 3321, 3203, 3315, 3316, 3317, 3318, 3319, 3320, 3321, 3322, 3247, 1609, 3316, 3317, 3318,
    3319, 3320, 3321, 3322, 1608, 3280, 3293, 3294, 3295, 3296, 3297, 3298, 3299, 3300, 3301, 3302,
    297, 299, 310, 300, 310, 310, 301, 310, 310, 310, 302, 310, 310, 310, 310, 303, 310, 310, 310,
    310, 310, 304, 310, 310, 310, 310, 310, 310, 305, 310, 310, 310, 310, 310, 310, 310, 306, 310,
    310, 310, 310, 310, 310, 310, 310, 307, 310, 310, 310, 310, 310, 310, 310, 310, 310, 308, 310,
    310, 310, 310, 310, 310, 310, 310, 310, 310, 309, 143, 144, 145, 146, 147, 148, 149, 150, 151,
    152, 153, 22, 178, 311, 190, 34, 178, 311, 312, 190, 311, 313, 312, 202, 202, 46, 178, 311,
    312, 190, 311, 2402, 312, 313, 313, 202, 311, 314, 312, 314, 314, 313, 214, 214, 214, 58, 178,
    311, 312, 190, 311, 2402, 312, 313, 313, 202, 311, 2402, 312, 2403, 2413, 313, 314, 314, 314,
    214, 311, 315, 312, 315, 315, 313, 315, 315, 315, 314, 226, 226, 226, 226, 70, 178, 311, 312,
    190, 311, 2402, 312, 313, 313, 202, 311, 2402, 312, 2403, 2413, 313, 314, 314, 314, 214, 311,
    2402, 312, 2403, 2413, 313, 2404, 2414, 2423, 314, 315, 315, 315, 315, 226, 311, 316, 312, 316,
    316, 313, 316, 316, 316, 314, 316, 316, 316, 316, 315, 238, 238, 238, 238, 238, 82, 178, 311,
    312, 190, 311, 2402, 312, 313, 313, 202, 311, 2402, 312, 2403, 2413, 313, 314, 314, 314, 214,
    311, 2402, 312, 2403, 2413, 313, 2404, 2414, 2423, 314, 315, 315, 315, 315, 226, 311, 2402,
    312, 2403, 2413, 313, 2404, 2414, 2423, 314, 2405, 2415, 2424, 2432, 315, 316, 316, 316, 316,
    316, 238, 311, 317, 312, 317, 317, 313, 317, 317, 317, 314, 317, 317, 317, 317, 315, 317, 317,
    317, 317, 317, 316, 250, 250, 250, 250, 250, 250, 94, 178, 311, 312, 190, 311, 2402, 312, 313,
    313, 202, 311, 2402, 312, 2403, 2413, 313, 314, 314, 314, 214, 311, 2402, 312, 2403, 2413, 313,
    2404, 2414, 2423, 314, 315, 315, 315, 315, 226, 311, 2402, 312, 2403, 2413, 313, 2404, 2414,
    2423, 314, 2405, 2415, 2424, 2432, 315, 316, 316, 316, 316, 316, 238, 311, 2402, 312, 2403,
    2413, 313, 2404, 2414, 2423, 314, 2405, 2415, 2424, 2432, 315, 2406, 2416, 2425, 2433, 2440,
    316, 317, 317, 317, 317, 317, 317, 250, 311, 318, 312, 318, 318, 313, 318, 318, 318, 314, 318,
    318, 318, 318, 315, 318, 318, 318, 318, 318, 316, 318, 318, 318, 318, 318, 318, 317, 262, 262,
    262, 262, 262, 262, 262, 106, 178, 311, 312, 190, 311, 2402, 312, 313, 313, 202, 311, 2402,
    312, 2403, 2413, 313, 314, 314, 314, 214, 311, 2402, 312, 2403, 2413, 313, 2404, 2414, 2423,
    314, 315, 315, 315, 315, 226, 311, 2402, 312, 2403, 2413, 313, 2404, 2414, 2423, 314, 2405,
    2415, 2424, 2432, 315, 316, 316, 316, 316, 316, 238, 311, 2402, 312, 2403, 2413, 313, 2404,
    2414, 2423, 314, 2405, 2415, 2424, 2432, 315, 2406, 2416, 2425, 2433, 2440, 316, 317, 317, 317,
    317, 317, 317, 250, 311, 2402, 312, 2403, 2413, 313, 2404, 2414, 2423, 314, 2405, 2415, 2424,
    2432, 315, 2406, 2416, 2425, 2433, 2440, 316, 2407, 2417, 2426, 2434, 2441, 2447, 317, 318,
    318, 318, 318, 318, 318, 318, 262, 311, 319, 312, 319, 319, 313, 319, 319, 319, 314, 319, 319,
    319, 319, 315, 319, 319, 319, 319, 319, 316, 319, 319, 319, 319, 319, 319, 317, 319, 319, 319,
    319, 319, 319, 319, 318, 274, 274, 274, 274, 274, 274, 274, 274, 118, 178, 311, 312, 190, 311,
    2402, 312, 313, 313, 202, 311, 2402, 312, 2403, 2413, 313, 314, 314, 314, 214, 311, 2402, 312,
    2403, 2413, 313, 2404, 2414, 2423, 314, 315, 315, 315, 315, 226, 311, 2402, 312, 2403, 2413,
    313, 2404, 2414, 2423, 314, 2405, 2415, 2424, 2432, 315, 316, 316, 316, 316, 316, 238, 311,
    2402, 312, 2403, 2413, 313, 2404, 2414, 2423, 314, 2405, 2415, 2424, 2432, 315, 2406, 2416,
    2425, 2433, 2440, 316, 317, 317, 317, 317, 317, 317, 250, 311, 2402, 312, 2403, 2413, 313,
    2404, 2414, 2423, 314, 2405, 2415, 2424, 2432, 315, 2406, 2416, 2425, 2433, 2440, 316, 2407,
    2417, 2426, 2434, 2441, 2447, 317, 318, 318, 318, 318, 318, 318, 318, 262, 311, 2402, 312,
    2403, 2413, 313, 2404, 2414, 2423, 314, 2405, 2415, 2424, 2432, 315, 2406, 2416, 2425, 2433,
    2440, 316, 2407, 2417, 2426, 2434, 2441, 2447, 317, 2408, 2418, 2427, 2435, 2442, 2448, 2453,
    318, 319, 319, 319, 319, 319, 319, 319, 319, 274, 311, 320, 312, 320, 320, 313, 320, 320, 320,
    314, 320, 320, 320, 320, 315, 320, 320, 320, 320, 320, 316, 320, 320, 320, 320, 320, 320, 317,
    320, 320, 320, 320, 320, 320, 320, 318, 320, 320, 320, 320, 320, 320, 320, 320, 319, 286, 286,
    286, 286, 286, 286, 286, 286, 286, 130, 178, 311, 312, 190, 311, 2402, 312, 313, 313, 202, 311,
    2402, 312, 2403, 2413, 313, 314, 314, 314, 214, 311, 2402, 312, 2403, 2413, 313, 2404, 2414,
    2423, 314, 315, 315, 315, 315, 226, 311, 2402, 312, 2403, 2413, 313, 2404, 2414, 2423, 314,
    2405, 2415, 2424, 2432, 315, 316, 316, 316, 316, 316, 238, 311, 2402, 312, 2403, 2413, 313,
    2404, 2414, 2423, 314, 2405, 2415, 2424, 2432, 315, 2406, 2416, 2425, 2433, 2440, 316, 317,
    317, 317, 317, 317, 317, 250, 311, 2402, 312, 2403, 2413, 313, 2404, 2414, 2423, 314, 2405,
    2415, 2424, 2432, 315, 2406, 2416, 2425, 2433, 2440, 316, 2407, 2417, 2426, 2434, 2441, 2447,
    317, 318, 318, 318, 318, 318, 318, 318, 262, 311, 2402, 312, 2403, 2413, 313, 2404, 2414, 2423,
    314, 2405, 2415, 2424, 2432, 315, 2406, 2416, 2425, 2433, 2440, 316, 2407, 2417, 2426, 2434,
    2441, 2447, 317, 2408, 2418, 2427, 2435, 2442, 2448, 2453, 318, 319, 319, 319, 319, 319, 319,
    319, 319, 274, 311, 2402, 312, 2403, 2413, 313, 2404, 2414, 2423, 314, 2405, 2415, 2424, 2432,
    315, 2406, 2416, 2425, 2433, 2440, 316, 2407, 2417, 2426, 2434, 2441, 2447, 317, 2408, 2418,
    2427, 2435, 2442, 2448, 2453, 318, 2409, 2419, 2428, 2436, 2443, 2449, 2454, 2458, 319, 320,
    320, 320, 320, 320, 320, 320, 320, 320, 286, 311, 321, 312, 321, 321, 313, 321, 321, 321, 314,
    321, 321, 321, 321, 315, 321, 321, 321, 321, 321, 316, 321, 321, 321, 321, 321, 321, 317, 321,
    321, 321, 321, 321, 321, 321, 318, 321, 321, 321, 321, 321, 321, 321, 321, 319, 321, 321, 321,
    321, 321, 321, 321, 321, 321, 320, 298, 298, 298, 298, 298, 298, 298, 298, 298, 298, 142, 178,
    311, 312, 190, 311, 2402, 312, 313, 313, 202, 311, 2402, 312, 2403, 2413, 313, 314, 314, 314,
    214, 311, 2402, 312, 2403, 2413, 313, 2404, 2414, 2423, 314, 315, 315, 315, 315, 226, 311,
    2402, 312, 2403, 2413, 313, 2404, 2414, 2423, 314, 2405, 2415, 2424, 2432, 315, 316, 316, 316,
    316, 316, 238, 311, 2402, 312, 2403, 2413, 313, 2404, 2414, 2423, 314, 2405, 2415, 2424, 2432,
    315, 2406, 2416, 2425, 2433, 2440, 316, 317, 317, 317, 317, 317, 317, 250, 311, 2402, 312,
    2403, 2413, 313, 2404, 2414, 2423, 314, 2405, 2415, 2424, 2432, 315, 2406, 2416, 2425, 2433,
    2440, 316, 2407, 2417, 2426, 2434, 2441, 2447, 317, 318, 318, 318, 318, 318, 318, 318, 262,
    311, 2402, 312, 2403, 2413, 313, 2404, 2414, 2423, 314, 2405, 2415, 2424, 2432, 315, 2406,
    2416, 2425, 2433, 2440, 316, 2407, 2417, 2426, 2434, 2441, 2447, 317, 2408, 2418, 2427, 2435,
    2442, 2448, 2453, 318, 319, 319, 319, 319, 319, 319, 319, 319, 274, 311, 2402, 312, 2403, 2413,
    313, 2404, 2414, 2423, 314, 2405, 2415, 2424, 2432, 315, 2406, 2416, 2425, 2433, 2440, 316,
    2407, 2417, 2426, 2434, 2441, 2447, 317, 2408, 2418, 2427, 2435, 2442, 2448, 2453, 318, 2409,
    2419, 2428, 2436, 2443, 2449, 2454, 2458, 319, 320, 320, 320, 320, 320, 320, 320, 320, 320,
    286, 311, 2402, 312, 2403, 2413, 313, 2404, 2414, 2423, 314, 2405, 2415, 2424, 2432, 315, 2406,
    2416, 2425, 2433, 2440, 316, 2407, 2417, 2426, 2434, 2441, 2447, 317, 2408, 2418, 2427, 2435,
    2442, 2448, 2453, 318, 2409, 2419, 2428, 2436, 2443, 2449, 2454, 2458, 319, 1609, 2420, 2429,
    2437, 2444, 2450, 2455, 2459, 1608, 320, 321, 321, 321, 321, 321, 321, 321, 321, 321, 321, 298,
    311, 322, 312, 322, 322, 313, 322, 322, 322, 314, 322, 322, 322, 322, 315, 322, 322, 322, 322,
    322, 316, 322, 322, 322, 322, 322, 322, 317, 322, 322, 322, 322, 322, 322, 322, 318, 322, 322,
    322, 322, 322, 322, 322, 322, 319, 322, 322, 322, 322, 322, 322, 322, 322, 322, 320, 322, 322,
    322, 322, 322, 322, 322, 322, 322, 322, 321, 310, 310, 310, 310, 310, 310, 310, 310, 310, 310,
    310, 154, 155, 155, 155, 156, 155, 155, 156, 155, 156, 157, 155, 155, 156, 155, 156, 157, 155,
    156, 157, 158, 155, 155, 156, 155, 156, 157, 155, 156, 157, 158, 155, 156, 157, 158, 159, 155,
    155, 156, 155, 156, 157, 155, 156, 157, 158, 155, 156, 157, 158, 159, 155, 156, 157, 158, 159,
    160, 155, 155, 156, 155, 156, 157, 155, 156, 157, 158, 155, 156, 157, 158, 159, 155, 156, 157,
    158, 159, 160, 155, 156, 157, 158, 159, 160, 161, 155, 155, 156, 155, 156, 157, 155, 156, 157,
    158, 155, 156, 157, 158, 159, 155, 156, 157, 158, 159, 160, 155, 156, 157, 158, 159, 160, 161,
    155, 156, 157, 158, 159, 160, 161, 162, 155, 155, 156, 155, 156, 157, 155, 156, 157, 158, 155,
    156, 157, 158, 159, 155, 156, 157, 158, 159, 160, 155, 156, 157, 158, 159, 160, 161, 155, 156,
    157, 158, 159, 160, 161, 162, 155, 156, 157, 158, 159, 160, 161, 162, 163, 155, 155, 156, 155,
    156, 157, 155, 156, 157, 158, 155, 156, 157, 158, 159, 155, 156, 157, 158, 159, 160, 155, 156,
    157, 158, 159, 160, 161, 155, 156, 157, 158, 159, 160, 161, 162, 155, 156, 157, 158, 159, 160,
    161, 162, 163, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 155, 155, 156, 155, 156, 157,
    155, 156, 157, 158, 155, 156, 157, 158, 159, 155, 156, 157, 158, 159, 160, 155, 156, 157, 158,
    159, 160, 161, 155, 156, 157, 158, 159, 160, 161, 162, 155, 156, 157, 158, 159, 160, 161, 162,
    163, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 155, 156, 157, 158, 159, 160, 161, 162,
    163, 164, 165, 155, 155, 156, 155, 156, 157, 155, 156, 157, 158, 155, 156, 157, 158, 159, 155,
    156, 157, 158, 159, 160, 155, 156, 157, 158, 159, 160, 161, 155, 156, 157, 158, 159, 160, 161,
    162, 155, 156, 157, 158, 159, 160, 161, 162, 163, 155, 156, 157, 158, 159, 160, 161, 162, 163,
    164, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 155, 156, 157, 158, 159, 160, 161,
    162, 163, 164, 165, 166,
];
