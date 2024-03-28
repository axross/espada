use super::dp_table::{dp_ref, AS_FLUSH, AS_RAINBOW};
use crate::card::{Card, Rank, Suit};

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
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.power_index().partial_cmp(&other.power_index())
    }
}

impl From<[Card; 7]> for MadeHand {
    fn from(cards: [Card; 7]) -> Self {
        let flash_suit = find_flush_suit(&cards);

        match flash_suit {
            Some(suit) => MadeHand(AS_FLUSH[hash_for_flush(&cards, &suit) as usize]),
            _ => MadeHand(AS_RAINBOW[hash_for_rainbow(&cards) as usize]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod power_index {
        use super::*;

        #[macro_export]
        macro_rules! card_array {
        ( $( $x:expr ),* ) => {
            {
                [
                    $(
                        $x.parse().unwrap(),
                    )*
                ]
            }
        };
    }

        #[test]
        fn it_returns_4c8hkhqc4s6hjd_power_index_5581() {
            let made_hand: MadeHand = card_array!["4c", "8h", "Kh", "Qc", "4s", "6h", "Jd"].into();

            assert_eq!(made_hand.power_index(), 5581);
        }

        #[test]
        fn it_returns_2d5sjc6s3s3dqh_power_index_5850() {
            let made_hand: MadeHand = card_array!["2d", "5s", "Jc", "6s", "3s", "3d", "Qh"].into();

            assert_eq!(made_hand.power_index(), 5850);
        }

        #[test]
        fn it_returns_7h9c5h5d4d7s4s_power_index_3177() {
            let made_hand: MadeHand = card_array!["7h", "9c", "5h", "5d", "4d", "7s", "4s"].into();

            assert_eq!(made_hand.power_index(), 3177);
        }

        #[test]
        fn it_returns_5h7d7ctd8dad2c_power_index_4894() {
            let made_hand: MadeHand = card_array!["5h", "7d", "7c", "Td", "8d", "Ad", "2c"].into();

            assert_eq!(made_hand.power_index(), 4894);
        }

        #[test]
        fn it_returns_5d2h7c3d5h7skh_power_index_3173() {
            let made_hand: MadeHand = card_array!["5d", "2h", "7c", "3d", "5h", "7s", "Kh"].into();

            assert_eq!(made_hand.power_index(), 3173);
        }

        #[test]
        fn it_returns_js8s5s4sas7dqs_power_index_504() {
            let made_hand: MadeHand = card_array!["Js", "8s", "5s", "4s", "As", "7d", "Qs"].into();

            assert_eq!(made_hand.power_index(), 504);
        }

        #[test]
        fn it_returns_9htc3sad2s6cah_power_index_3464() {
            let made_hand: MadeHand = card_array!["9h", "Tc", "3s", "Ad", "2s", "6c", "Ah"].into();

            assert_eq!(made_hand.power_index(), 3464);
        }

        #[test]
        fn it_returns_6h4skh7h8d3had_power_index_6315() {
            let made_hand: MadeHand = card_array!["6h", "4s", "Kh", "7h", "8d", "3h", "Ad"].into();

            assert_eq!(made_hand.power_index(), 6315);
        }

        #[test]
        fn it_returns_td9s5ctsjc6dah_power_index_4225() {
            let made_hand: MadeHand = card_array!["Td", "9s", "5c", "Ts", "Jc", "6d", "Ah"].into();

            assert_eq!(made_hand.power_index(), 4225);
        }

        #[test]
        fn it_returns_6s2d8c6d6h4dth_power_index_2177() {
            let made_hand: MadeHand = card_array!["6s", "2d", "8c", "6d", "6h", "4d", "Th"].into();

            assert_eq!(made_hand.power_index(), 2177);
        }

        #[test]
        fn it_returns_asjsjh5s9d7d9h_power_index_2842() {
            let made_hand: MadeHand = card_array!["As", "Js", "Jh", "5s", "9d", "7d", "9h"].into();

            assert_eq!(made_hand.power_index(), 2842);
        }

        #[test]
        fn it_returns_jd4d5d5s9d7h4c_power_index_3263() {
            let made_hand: MadeHand = card_array!["Jd", "4d", "5d", "5s", "9d", "7h", "4c"].into();

            assert_eq!(made_hand.power_index(), 3263);
        }

        #[test]
        fn it_returns_kcks2h6hac9c7d_power_index_3574() {
            let made_hand: MadeHand = card_array!["Kc", "Ks", "2h", "6h", "Ac", "9c", "7d"].into();

            assert_eq!(made_hand.power_index(), 3574);
        }

        #[test]
        fn it_returns_2s8c3skdqs8d7d_power_index_4704() {
            let made_hand: MadeHand = card_array!["2s", "8c", "3s", "Kd", "Qs", "8d", "7d"].into();

            assert_eq!(made_hand.power_index(), 4704);
        }

        #[test]
        fn it_returns_7c9s6c2hadqd3c_power_index_6420() {
            let made_hand: MadeHand = card_array!["7c", "9s", "6c", "2h", "Ad", "Qd", "3c"].into();

            assert_eq!(made_hand.power_index(), 6420);
        }

        #[test]
        fn it_returns_8d7s5cacks7h9d_power_index_4869() {
            let made_hand: MadeHand = card_array!["8d", "7s", "5c", "Ac", "Ks", "7h", "9d"].into();

            assert_eq!(made_hand.power_index(), 4869);
        }

        #[test]
        fn it_returns_8c8s3s9c8h3c9h_power_index_244() {
            let made_hand: MadeHand = card_array!["8c", "8s", "3s", "9c", "8h", "3c", "9h"].into();

            assert_eq!(made_hand.power_index(), 244);
        }

        #[test]
        fn it_returns_2hthjh8sts7djd_power_index_2835() {
            let made_hand: MadeHand = card_array!["2h", "Th", "Jh", "8s", "Ts", "7d", "Jd"].into();

            assert_eq!(made_hand.power_index(), 2835);
        }

        #[test]
        fn it_returns_askcahjsthac4d_power_index_1611() {
            let made_hand: MadeHand = card_array!["As", "Kc", "Ah", "Js", "Th", "Ac", "4d"].into();

            assert_eq!(made_hand.power_index(), 1611);
        }

        #[test]
        fn it_returns_4sqh3h7s9s7hqd_power_index_2769() {
            let made_hand: MadeHand = card_array!["4s", "Qh", "3h", "7s", "9s", "7h", "Qd"].into();

            assert_eq!(made_hand.power_index(), 2769);
        }

        #[test]
        fn it_returns_4h5d6dah3d5sth_power_index_5336() {
            let made_hand: MadeHand = card_array!["4h", "5d", "6d", "Ah", "3d", "5s", "Th"].into();

            assert_eq!(made_hand.power_index(), 5336);
        }

        #[test]
        fn it_returns_4dtsqd8c6s3c7s_power_index_7112() {
            let made_hand: MadeHand = card_array!["4d", "Ts", "Qd", "8c", "6s", "3c", "7s"].into();

            assert_eq!(made_hand.power_index(), 7112);
        }

        #[test]
        fn it_returns_4d3s2c7d7s2d2s_power_index_318() {
            let made_hand: MadeHand = card_array!["4d", "3s", "2c", "7d", "7s", "2d", "2s"].into();

            assert_eq!(made_hand.power_index(), 318);
        }

        #[test]
        fn it_returns_8casqskd2s7hth_power_index_6195() {
            let made_hand: MadeHand = card_array!["8c", "As", "Qs", "Kd", "2s", "7h", "Th"].into();

            assert_eq!(made_hand.power_index(), 6195);
        }

        #[test]
        fn it_returns_7s6hkc2hqstsjc_power_index_6680() {
            let made_hand: MadeHand = card_array!["7s", "6h", "Kc", "2h", "Qs", "Ts", "Jc"].into();

            assert_eq!(made_hand.power_index(), 6680);
        }

        #[test]
        fn it_returns_ah7hjs5h4cts6d_power_index_6483() {
            let made_hand: MadeHand = card_array!["Ah", "7h", "Js", "5h", "4c", "Ts", "6d"].into();

            assert_eq!(made_hand.power_index(), 6483);
        }

        #[test]
        fn it_returns_kc4h6hqs3h7cth_power_index_6727() {
            let made_hand: MadeHand = card_array!["Kc", "4h", "6h", "Qs", "3h", "7c", "Th"].into();

            assert_eq!(made_hand.power_index(), 6727);
        }

        #[test]
        fn it_returns_thks9d4s2s8d3s_power_index_6885() {
            let made_hand: MadeHand = card_array!["Th", "Ks", "9d", "4s", "2s", "8d", "3s"].into();

            assert_eq!(made_hand.power_index(), 6885);
        }

        #[test]
        fn it_returns_kd3hth4c5h8h2s_power_index_6912() {
            let made_hand: MadeHand = card_array!["Kd", "3h", "Th", "4c", "5h", "8h", "2s"].into();

            assert_eq!(made_hand.power_index(), 6912);
        }

        #[test]
        fn it_returns_3s6s8h9cjc2dqs_power_index_7036() {
            let made_hand: MadeHand = card_array!["3s", "6s", "8h", "9c", "Jc", "2d", "Qs"].into();

            assert_eq!(made_hand.power_index(), 7036);
        }

        #[test]
        fn it_returns_4d2c3cad3h4s8h_power_index_3293() {
            let made_hand: MadeHand = card_array!["4d", "2c", "3c", "Ad", "3h", "4s", "8h"].into();

            assert_eq!(made_hand.power_index(), 3293);
        }

        #[test]
        fn it_returns_8d2sts5c8s9cac_power_index_4673() {
            let made_hand: MadeHand = card_array!["8d", "2s", "Ts", "5c", "8s", "9c", "Ac"].into();

            assert_eq!(made_hand.power_index(), 4673);
        }

        #[test]
        fn it_returns_9c6d4cks7c6sqs_power_index_5143() {
            let made_hand: MadeHand = card_array!["9c", "6d", "4c", "Ks", "7c", "6s", "Qs"].into();

            assert_eq!(made_hand.power_index(), 5143);
        }

        #[test]
        fn it_returns_4c3hkc5cqdqctd_power_index_3834() {
            let made_hand: MadeHand = card_array!["4c", "3h", "Kc", "5c", "Qd", "Qc", "Td"].into();

            assert_eq!(made_hand.power_index(), 3834);
        }

        #[test]
        fn it_returns_3d9h6dtc7s9c5s_power_index_4596() {
            let made_hand: MadeHand = card_array!["3d", "9h", "6d", "Tc", "7s", "9c", "5s"].into();

            assert_eq!(made_hand.power_index(), 4596);
        }

        #[test]
        fn it_returns_7c4ctcks3d2s6c_power_index_6919() {
            let made_hand: MadeHand = card_array!["7c", "4c", "Tc", "Ks", "3d", "2s", "6c"].into();

            assert_eq!(made_hand.power_index(), 6919);
        }

        #[test]
        fn it_returns_6cjhtsqdtc2sjd_power_index_2833() {
            let made_hand: MadeHand = card_array!["6c", "Jh", "Ts", "Qd", "Tc", "2s", "Jd"].into();

            assert_eq!(made_hand.power_index(), 2833);
        }

        #[test]
        fn it_returns_qd4ckh2h2s3cah_power_index_5966() {
            let made_hand: MadeHand = card_array!["Qd", "4c", "Kh", "2h", "2s", "3c", "Ah"].into();

            assert_eq!(made_hand.power_index(), 5966);
        }

        #[test]
        fn it_returns_6d2dkd3h8sas9c_power_index_6295() {
            let made_hand: MadeHand = card_array!["6d", "2d", "Kd", "3h", "8s", "As", "9c"].into();

            assert_eq!(made_hand.power_index(), 6295);
        }

        #[test]
        fn it_returns_4s3hqc2c5hkhts_power_index_6736() {
            let made_hand: MadeHand = card_array!["4s", "3h", "Qc", "2c", "5h", "Kh", "Ts"].into();

            assert_eq!(made_hand.power_index(), 6736);
        }

        #[test]
        fn it_returns_4s6hqh4d2s7hqc_power_index_2804() {
            let made_hand: MadeHand = card_array!["4s", "6h", "Qh", "4d", "2s", "7h", "Qc"].into();

            assert_eq!(made_hand.power_index(), 2804);
        }

        #[test]
        fn it_returns_3d6d3h3s4skh9h_power_index_2350() {
            let made_hand: MadeHand = card_array!["3d", "6d", "3h", "3s", "4s", "Kh", "9h"].into();

            assert_eq!(made_hand.power_index(), 2350);
        }

        #[test]
        fn it_returns_ksqdkc4d5d3cts_power_index_3614() {
            let made_hand: MadeHand = card_array!["Ks", "Qd", "Kc", "4d", "5d", "3c", "Ts"].into();

            assert_eq!(made_hand.power_index(), 3614);
        }

        #[test]
        fn it_returns_jh6s5s5d4sqh2h_power_index_5410() {
            let made_hand: MadeHand = card_array!["Jh", "6s", "5s", "5d", "4s", "Qh", "2h"].into();

            assert_eq!(made_hand.power_index(), 5410);
        }

        #[test]
        fn it_returns_th5d5c9c2sjsqs_power_index_5406() {
            let made_hand: MadeHand = card_array!["Th", "5d", "5c", "9c", "2s", "Js", "Qs"].into();

            assert_eq!(made_hand.power_index(), 5406);
        }

        #[test]
        fn it_returns_th9h6cks5hkh4s_power_index_3684() {
            let made_hand: MadeHand = card_array!["Th", "9h", "6c", "Ks", "5h", "Kh", "4s"].into();

            assert_eq!(made_hand.power_index(), 3684);
        }

        #[test]
        fn it_returns_7djs6h5d2s2dtd_power_index_6104() {
            let made_hand: MadeHand = card_array!["7d", "Js", "6h", "5d", "2s", "2d", "Td"].into();

            assert_eq!(made_hand.power_index(), 6104);
        }

        #[test]
        fn it_returns_as9d8c6dqd6sjh_power_index_5096() {
            let made_hand: MadeHand = card_array!["As", "9d", "8c", "6d", "Qd", "6s", "Jh"].into();

            assert_eq!(made_hand.power_index(), 5096);
        }

        #[test]
        fn it_returns_jd4d2c6sad4s5c_power_index_5549() {
            let made_hand: MadeHand = card_array!["Jd", "4d", "2c", "6s", "Ad", "4s", "5c"].into();

            assert_eq!(made_hand.power_index(), 5549);
        }

        #[test]
        fn it_returns_2d7c6skd6ckskh_power_index_186() {
            let made_hand: MadeHand = card_array!["2d", "7c", "6s", "Kd", "6c", "Ks", "Kh"].into();

            assert_eq!(made_hand.power_index(), 186);
        }

        #[test]
        fn it_returns_7dkdjc4ctcksas_power_index_3556() {
            let made_hand: MadeHand = card_array!["7d", "Kd", "Jc", "4c", "Tc", "Ks", "As"].into();

            assert_eq!(made_hand.power_index(), 3556);
        }

        #[test]
        fn it_returns_qd2d5cjd7c6cac_power_index_6371() {
            let made_hand: MadeHand = card_array!["Qd", "2d", "5c", "Jd", "7c", "6c", "Ac"].into();

            assert_eq!(made_hand.power_index(), 6371);
        }

        #[test]
        fn it_returns_jhad3d3cts4d5h_power_index_5765() {
            let made_hand: MadeHand = card_array!["Jh", "Ad", "3d", "3c", "Ts", "4d", "5h"].into();

            assert_eq!(made_hand.power_index(), 5765);
        }

        #[test]
        fn it_returns_8dkd7ckstc6sqd_power_index_3611() {
            let made_hand: MadeHand = card_array!["8d", "Kd", "7c", "Ks", "Tc", "6s", "Qd"].into();

            assert_eq!(made_hand.power_index(), 3611);
        }

        #[test]
        fn it_returns_6h3s7d8h7c9h3d_power_index_3199() {
            let made_hand: MadeHand = card_array!["6h", "3s", "7d", "8h", "7c", "9h", "3d"].into();

            assert_eq!(made_hand.power_index(), 3199);
        }

        #[test]
        fn it_returns_tc2s2c4c8d9h6d_power_index_6130() {
            let made_hand: MadeHand = card_array!["Tc", "2s", "2c", "4c", "8d", "9h", "6d"].into();

            assert_eq!(made_hand.power_index(), 6130);
        }

        #[test]
        fn it_returns_5hjd4s2h3d7d9h_power_index_7291() {
            let made_hand: MadeHand = card_array!["5h", "Jd", "4s", "2h", "3d", "7d", "9h"].into();

            assert_eq!(made_hand.power_index(), 7291);
        }

        #[test]
        fn it_returns_7c5ststcqcacjd_power_index_4216() {
            let made_hand: MadeHand = card_array!["7c", "5s", "Ts", "Tc", "Qc", "Ac", "Jd"].into();

            assert_eq!(made_hand.power_index(), 4216);
        }

        #[test]
        fn it_returns_jd7dacqd7c2cjs_power_index_2864() {
            let made_hand: MadeHand = card_array!["Jd", "7d", "Ac", "Qd", "7c", "2c", "Js"].into();

            assert_eq!(made_hand.power_index(), 2864);
        }

        #[test]
        fn it_returns_4ckc6h9cac9sad_power_index_2512() {
            let made_hand: MadeHand = card_array!["4c", "Kc", "6h", "9c", "Ac", "9s", "Ad"].into();

            assert_eq!(made_hand.power_index(), 2512);
        }

        #[test]
        fn it_returns_ksadjc7dqs2h7c_power_index_4866() {
            let made_hand: MadeHand = card_array!["Ks", "Ad", "Jc", "7d", "Qs", "2h", "7c"].into();

            assert_eq!(made_hand.power_index(), 4866);
        }

        #[test]
        fn it_returns_2djc8h9cjd7c9d_power_index_2846() {
            let made_hand: MadeHand = card_array!["2d", "Jc", "8h", "9c", "Jd", "7c", "9d"].into();

            assert_eq!(made_hand.power_index(), 2846);
        }

        #[test]
        fn it_returns_2hac8cjdtd5d6h_power_index_6478() {
            let made_hand: MadeHand = card_array!["2h", "Ac", "8c", "Jd", "Td", "5d", "6h"].into();

            assert_eq!(made_hand.power_index(), 6478);
        }

        #[test]
        fn it_returns_qh6s5c8d7d5s9c_power_index_1605() {
            let made_hand: MadeHand = card_array!["Qh", "6s", "5c", "8d", "7d", "5s", "9c"].into();

            assert_eq!(made_hand.power_index(), 1605);
        }

        #[test]
        fn it_returns_8dkdah2cqd7cqh_power_index_3769() {
            let made_hand: MadeHand = card_array!["8d", "Kd", "Ah", "2c", "Qd", "7c", "Qh"].into();

            assert_eq!(made_hand.power_index(), 3769);
        }

        #[test]
        fn it_returns_2h3h8djc7h3dqs_power_index_5848() {
            let made_hand: MadeHand = card_array!["2h", "3h", "8d", "Jc", "7h", "3d", "Qs"].into();

            assert_eq!(made_hand.power_index(), 5848);
        }

        #[test]
        fn it_returns_9s8cks5c2d9c3s_power_index_4507() {
            let made_hand: MadeHand = card_array!["9s", "8c", "Ks", "5c", "2d", "9c", "3s"].into();

            assert_eq!(made_hand.power_index(), 4507);
        }

        #[test]
        fn it_returns_5std5c6hqs8d3h_power_index_5415() {
            let made_hand: MadeHand = card_array!["5s", "Td", "5c", "6h", "Qs", "8d", "3h"].into();

            assert_eq!(made_hand.power_index(), 5415);
        }

        #[test]
        fn it_returns_3h6htsadjc3d4d_power_index_5765() {
            let made_hand: MadeHand = card_array!["3h", "6h", "Ts", "Ad", "Jc", "3d", "4d"].into();

            assert_eq!(made_hand.power_index(), 5765);
        }

        #[test]
        fn it_returns_thas2dks3s8had_power_index_3346() {
            let made_hand: MadeHand = card_array!["Th", "As", "2d", "Ks", "3s", "8h", "Ad"].into();

            assert_eq!(made_hand.power_index(), 3346);
        }

        #[test]
        fn it_returns_6h8hts8c6c4h2s_power_index_3110() {
            let made_hand: MadeHand = card_array!["6h", "8h", "Ts", "8c", "6c", "4h", "2s"].into();

            assert_eq!(made_hand.power_index(), 3110);
        }

        #[test]
        fn it_returns_3htsjcqhqd2s9h_power_index_3866() {
            let made_hand: MadeHand = card_array!["3h", "Ts", "Jc", "Qh", "Qd", "2s", "9h"].into();

            assert_eq!(made_hand.power_index(), 3866);
        }

        #[test]
        fn it_returns_jcas6c8c9d5h2s_power_index_6499() {
            let made_hand: MadeHand = card_array!["Jc", "As", "6c", "8c", "9d", "5h", "2s"].into();

            assert_eq!(made_hand.power_index(), 6499);
        }

        #[test]
        fn it_returns_actsqs9sjs4d3h_power_index_6350() {
            let made_hand: MadeHand = card_array!["Ac", "Ts", "Qs", "9s", "Js", "4d", "3h"].into();

            assert_eq!(made_hand.power_index(), 6350);
        }

        #[test]
        fn it_returns_5dkh9sqs2c7has_power_index_6203() {
            let made_hand: MadeHand = card_array!["5d", "Kh", "9s", "Qs", "2c", "7h", "As"].into();

            assert_eq!(made_hand.power_index(), 6203);
        }

        #[test]
        fn it_returns_js2cks5sahtsqh_power_index_1600() {
            let made_hand: MadeHand = card_array!["Js", "2c", "Ks", "5s", "Ah", "Ts", "Qh"].into();

            assert_eq!(made_hand.power_index(), 1600);
        }

        #[test]
        fn it_returns_2c4hks6c8s6s5d_power_index_5172() {
            let made_hand: MadeHand = card_array!["2c", "4h", "Ks", "6c", "8s", "6s", "5d"].into();

            assert_eq!(made_hand.power_index(), 5172);
        }

        #[test]
        fn it_returns_2d3sthasjhjd8d_power_index_4006() {
            let made_hand: MadeHand = card_array!["2d", "3s", "Th", "As", "Jh", "Jd", "8d"].into();

            assert_eq!(made_hand.power_index(), 4006);
        }

        #[test]
        fn it_returns_tsjs9c8d9h4cjh_power_index_2845() {
            let made_hand: MadeHand = card_array!["Ts", "Js", "9c", "8d", "9h", "4c", "Jh"].into();

            assert_eq!(made_hand.power_index(), 2845);
        }

        #[test]
        fn it_returns_2cahad8h6c4s4h_power_index_2572() {
            let made_hand: MadeHand = card_array!["2c", "Ah", "Ad", "8h", "6c", "4s", "4h"].into();

            assert_eq!(made_hand.power_index(), 2572);
        }

        #[test]
        fn it_returns_qc8d7cqdkh4s3c_power_index_3845() {
            let made_hand: MadeHand = card_array!["Qc", "8d", "7c", "Qd", "Kh", "4s", "3c"].into();

            assert_eq!(made_hand.power_index(), 3845);
        }

        #[test]
        fn it_returns_jd8skhth4hjsqs_power_index_4041() {
            let made_hand: MadeHand = card_array!["Jd", "8s", "Kh", "Th", "4h", "Js", "Qs"].into();

            assert_eq!(made_hand.power_index(), 4041);
        }

        #[test]
        fn it_returns_qctc6hqd6c8s7h_power_index_2779() {
            let made_hand: MadeHand = card_array!["Qc", "Tc", "6h", "Qd", "6c", "8s", "7h"].into();

            assert_eq!(made_hand.power_index(), 2779);
        }

        #[test]
        fn it_returns_qd3dkdjd8h6sjc_power_index_4043() {
            let made_hand: MadeHand = card_array!["Qd", "3d", "Kd", "Jd", "8h", "6s", "Jc"].into();

            assert_eq!(made_hand.power_index(), 4043);
        }

        #[test]
        fn it_returns_5cts4s9s4d7sac_power_index_5553() {
            let made_hand: MadeHand = card_array!["5c", "Ts", "4s", "9s", "4d", "7s", "Ac"].into();

            assert_eq!(made_hand.power_index(), 5553);
        }

        #[test]
        fn it_returns_qd5s2s4sadth9h_power_index_6389() {
            let made_hand: MadeHand = card_array!["Qd", "5s", "2s", "4s", "Ad", "Th", "9h"].into();

            assert_eq!(made_hand.power_index(), 6389);
        }

        #[test]
        fn it_returns_5c2h9dqc2s3d4h_power_index_6084() {
            let made_hand: MadeHand = card_array!["5c", "2h", "9d", "Qc", "2s", "3d", "4h"].into();

            assert_eq!(made_hand.power_index(), 6084);
        }

        #[test]
        fn it_returns_6dkh7htd3s7d2s_power_index_4940() {
            let made_hand: MadeHand = card_array!["6d", "Kh", "7h", "Td", "3s", "7d", "2s"].into();

            assert_eq!(made_hand.power_index(), 4940);
        }

        #[test]
        fn it_returns_9h2dkhtd9c5h4h_power_index_4501() {
            let made_hand: MadeHand = card_array!["9h", "2d", "Kh", "Td", "9c", "5h", "4h"].into();

            assert_eq!(made_hand.power_index(), 4501);
        }

        #[test]
        fn it_returns_5s8d2d7ckh3h5d_power_index_5391() {
            let made_hand: MadeHand = card_array!["5s", "8d", "2d", "7c", "Kh", "3h", "5d"].into();

            assert_eq!(made_hand.power_index(), 5391);
        }

        #[test]
        fn it_returns_8hth9s7cqc4std_power_index_4314() {
            let made_hand: MadeHand = card_array!["8h", "Th", "9s", "7c", "Qc", "4s", "Td"].into();

            assert_eq!(made_hand.power_index(), 4314);
        }

        #[test]
        fn it_returns_jdkdjh8c5h8h3c_power_index_2854() {
            let made_hand: MadeHand = card_array!["Jd", "Kd", "Jh", "8c", "5h", "8h", "3c"].into();

            assert_eq!(made_hand.power_index(), 2854);
        }

        #[test]
        fn it_returns_4s3s4c6hahadqc_power_index_2568() {
            let made_hand: MadeHand = card_array!["4s", "3s", "4c", "6h", "Ah", "Ad", "Qc"].into();

            assert_eq!(made_hand.power_index(), 2568);
        }

        #[test]
        fn it_returns_jh7d6casac3dks_power_index_3339() {
            let made_hand: MadeHand = card_array!["Jh", "7d", "6c", "As", "Ac", "3d", "Ks"].into();

            assert_eq!(made_hand.power_index(), 3339);
        }

        #[test]
        fn it_returns_3sqc8h3hth9sac_power_index_5757() {
            let made_hand: MadeHand = card_array!["3s", "Qc", "8h", "3h", "Th", "9s", "Ac"].into();

            assert_eq!(made_hand.power_index(), 5757);
        }

        #[test]
        fn it_returns_7d4h4s7h9s8d6s_power_index_3188() {
            let made_hand: MadeHand = card_array!["7d", "4h", "4s", "7h", "9s", "8d", "6s"].into();

            assert_eq!(made_hand.power_index(), 3188);
        }

        #[test]
        fn it_returns_5s4s7skc3dqs9c_power_index_6749() {
            let made_hand: MadeHand = card_array!["5s", "4s", "7s", "Kc", "3d", "Qs", "9c"].into();

            assert_eq!(made_hand.power_index(), 6749);
        }

        #[test]
        fn it_returns_4s6c9d8cqcac4h_power_index_5538() {
            let made_hand: MadeHand = card_array!["4s", "6c", "9d", "8c", "Qc", "Ac", "4h"].into();

            assert_eq!(made_hand.power_index(), 5538);
        }

        #[test]
        fn it_returns_6hjcjd9d3c9h2c_power_index_2848() {
            let made_hand: MadeHand = card_array!["6h", "Jc", "Jd", "9d", "3c", "9h", "2c"].into();

            assert_eq!(made_hand.power_index(), 2848);
        }

        #[test]
        fn it_returns_6c7dqd2castdjc_power_index_6352() {
            let made_hand: MadeHand = card_array!["6c", "7d", "Qd", "2c", "As", "Td", "Jc"].into();

            assert_eq!(made_hand.power_index(), 6352);
        }

        #[test]
        fn it_returns_8s6d2c8c7c2skd_power_index_3151() {
            let made_hand: MadeHand = card_array!["8s", "6d", "2c", "8c", "7c", "2s", "Kd"].into();

            assert_eq!(made_hand.power_index(), 3151);
        }

        #[test]
        fn it_returns_jdtctsjh5das7c_power_index_2831() {
            let made_hand: MadeHand = card_array!["Jd", "Tc", "Ts", "Jh", "5d", "As", "7c"].into();

            assert_eq!(made_hand.power_index(), 2831);
        }

        #[test]
        fn it_returns_th4h3c9d9s4dac_power_index_3062() {
            let made_hand: MadeHand = card_array!["Th", "4h", "3c", "9d", "9s", "4d", "Ac"].into();

            assert_eq!(made_hand.power_index(), 3062);
        }

        #[test]
        fn it_returns_4hahts9h8d9c2c_power_index_4453() {
            let made_hand: MadeHand = card_array!["4h", "Ah", "Ts", "9h", "8d", "9c", "2c"].into();

            assert_eq!(made_hand.power_index(), 4453);
        }

        #[test]
        fn it_returns_3sac3dqctdks4c_power_index_5746() {
            let made_hand: MadeHand = card_array!["3s", "Ac", "3d", "Qc", "Td", "Ks", "4c"].into();

            assert_eq!(made_hand.power_index(), 5746);
        }

        #[test]
        fn it_returns_2c2sqcqd4has9s_power_index_2820() {
            let made_hand: MadeHand = card_array!["2c", "2s", "Qc", "Qd", "4h", "As", "9s"].into();

            assert_eq!(made_hand.power_index(), 2820);
        }

        #[test]
        fn it_returns_9d8d3cadtdjhas_power_index_3426() {
            let made_hand: MadeHand = card_array!["9d", "8d", "3c", "Ad", "Td", "Jh", "As"].into();

            assert_eq!(made_hand.power_index(), 3426);
        }

        #[test]
        fn it_returns_3d4stc4d2hah3h_power_index_3293() {
            let made_hand: MadeHand = card_array!["3d", "4s", "Tc", "4d", "2h", "Ah", "3h"].into();

            assert_eq!(made_hand.power_index(), 3293);
        }

        #[test]
        fn it_returns_ks9d6hjs4h8sqh_power_index_6686() {
            let made_hand: MadeHand = card_array!["Ks", "9d", "6h", "Js", "4h", "8s", "Qh"].into();

            assert_eq!(made_hand.power_index(), 6686);
        }

        #[test]
        fn it_returns_2cadkh6d8hah4s_power_index_3361() {
            let made_hand: MadeHand = card_array!["2c", "Ad", "Kh", "6d", "8h", "Ah", "4s"].into();

            assert_eq!(made_hand.power_index(), 3361);
        }

        #[test]
        fn it_returns_jhjd7s8s8c2s4s_power_index_2858() {
            let made_hand: MadeHand = card_array!["Jh", "Jd", "7s", "8s", "8c", "2s", "4s"].into();

            assert_eq!(made_hand.power_index(), 2858);
        }

        #[test]
        fn it_returns_js9d5dah7dqhkc_power_index_6186() {
            let made_hand: MadeHand = card_array!["Js", "9d", "5d", "Ah", "7d", "Qh", "Kc"].into();

            assert_eq!(made_hand.power_index(), 6186);
        }

        #[test]
        fn it_returns_qcjh6sqs8hts4c_power_index_3867() {
            let made_hand: MadeHand = card_array!["Qc", "Jh", "6s", "Qs", "8h", "Ts", "4c"].into();

            assert_eq!(made_hand.power_index(), 3867);
        }

        #[test]
        fn it_returns_ksahjdkckhjc7h_power_index_181() {
            let made_hand: MadeHand = card_array!["Ks", "Ah", "Jd", "Kc", "Kh", "Jc", "7h"].into();

            assert_eq!(made_hand.power_index(), 181);
        }

        #[test]
        fn it_returns_9d6c2ctdjc5h5d_power_index_5442() {
            let made_hand: MadeHand = card_array!["9d", "6c", "2c", "Td", "Jc", "5h", "5d"].into();

            assert_eq!(made_hand.power_index(), 5442);
        }

        #[test]
        fn it_returns_4s9c5s6d2dtc8d_power_index_7346() {
            let made_hand: MadeHand = card_array!["4s", "9c", "5s", "6d", "2d", "Tc", "8d"].into();

            assert_eq!(made_hand.power_index(), 7346);
        }

        #[test]
        fn it_returns_7ckh4sad6hts8d_power_index_6273() {
            let made_hand: MadeHand = card_array!["7c", "Kh", "4s", "Ad", "6h", "Ts", "8d"].into();

            assert_eq!(made_hand.power_index(), 6273);
        }

        #[test]
        fn it_returns_tdtsjdtc6hkc4s_power_index_1886() {
            let made_hand: MadeHand = card_array!["Td", "Ts", "Jd", "Tc", "6h", "Kc", "4s"].into();

            assert_eq!(made_hand.power_index(), 1886);
        }

        #[test]
        fn it_returns_7s4h2d5sks5h4d_power_index_3261() {
            let made_hand: MadeHand = card_array!["7s", "4h", "2d", "5s", "Ks", "5h", "4d"].into();

            assert_eq!(made_hand.power_index(), 3261);
        }

        #[test]
        fn it_returns_6s8c3d8dkcqhah_power_index_4646() {
            let made_hand: MadeHand = card_array!["6s", "8c", "3d", "8d", "Kc", "Qh", "Ah"].into();

            assert_eq!(made_hand.power_index(), 4646);
        }

        #[test]
        fn it_returns_7s9dkhkd6hthqd_power_index_3610() {
            let made_hand: MadeHand = card_array!["7s", "9d", "Kh", "Kd", "6h", "Th", "Qd"].into();

            assert_eq!(made_hand.power_index(), 3610);
        }

        #[test]
        fn it_returns_7h2c5h4s3sad6c_power_index_1607() {
            let made_hand: MadeHand = card_array!["7h", "2c", "5h", "4s", "3s", "Ad", "6c"].into();

            assert_eq!(made_hand.power_index(), 1607);
        }

        #[test]
        fn it_returns_8c2h3ckc9h7c3h_power_index_5825() {
            let made_hand: MadeHand = card_array!["8c", "2h", "3c", "Kc", "9h", "7c", "3h"].into();

            assert_eq!(made_hand.power_index(), 5825);
        }

        #[test]
        fn it_returns_kh5s6s2h7hts9c_power_index_6888() {
            let made_hand: MadeHand = card_array!["Kh", "5s", "6s", "2h", "7h", "Ts", "9c"].into();

            assert_eq!(made_hand.power_index(), 6888);
        }

        #[test]
        fn it_returns_qs9d9hah4h4c3s_power_index_3062() {
            let made_hand: MadeHand = card_array!["Qs", "9d", "9h", "Ah", "4h", "4c", "3s"].into();

            assert_eq!(made_hand.power_index(), 3062);
        }

        #[test]
        fn it_returns_9c9hjs5s3c6sjd_power_index_2848() {
            let made_hand: MadeHand = card_array!["9c", "9h", "Js", "5s", "3c", "6s", "Jd"].into();

            assert_eq!(made_hand.power_index(), 2848);
        }

        #[test]
        fn it_returns_kc8cjhtd7c9d2c_power_index_1603() {
            let made_hand: MadeHand = card_array!["Kc", "8c", "Jh", "Td", "7c", "9d", "2c"].into();

            assert_eq!(made_hand.power_index(), 1603);
        }

        #[test]
        fn it_returns_9d2d6s9hts3std_power_index_2936() {
            let made_hand: MadeHand = card_array!["9d", "2d", "6s", "9h", "Ts", "3s", "Td"].into();

            assert_eq!(made_hand.power_index(), 2936);
        }

        #[test]
        fn it_returns_3htsah5c8h4d6c_power_index_6580() {
            let made_hand: MadeHand = card_array!["3h", "Ts", "Ah", "5c", "8h", "4d", "6c"].into();

            assert_eq!(made_hand.power_index(), 6580);
        }

        #[test]
        fn it_returns_4d3sqh2s3c7c9s_power_index_5862() {
            let made_hand: MadeHand = card_array!["4d", "3s", "Qh", "2s", "3c", "7c", "9s"].into();

            assert_eq!(made_hand.power_index(), 5862);
        }

        #[test]
        fn it_returns_5c4std6dtc2sjd_power_index_4360() {
            let made_hand: MadeHand = card_array!["5c", "4s", "Td", "6d", "Tc", "2s", "Jd"].into();

            assert_eq!(made_hand.power_index(), 4360);
        }

        #[test]
        fn it_returns_6d6h8cts8s5s2h_power_index_3110() {
            let made_hand: MadeHand = card_array!["6d", "6h", "8c", "Ts", "8s", "5s", "2h"].into();

            assert_eq!(made_hand.power_index(), 3110);
        }

        #[test]
        fn it_returns_3dtdjsqhjhqs4h_power_index_2723() {
            let made_hand: MadeHand = card_array!["3d", "Td", "Js", "Qh", "Jh", "Qs", "4h"].into();

            assert_eq!(made_hand.power_index(), 2723);
        }

        #[test]
        fn it_returns_6dkh5d6ckc2c2s_power_index_2673() {
            let made_hand: MadeHand = card_array!["6d", "Kh", "5d", "6c", "Kc", "2c", "2s"].into();

            assert_eq!(made_hand.power_index(), 2673);
        }

        #[test]
        fn it_returns_9c4s6d6s7h3s8h_power_index_5271() {
            let made_hand: MadeHand = card_array!["9c", "4s", "6d", "6s", "7h", "3s", "8h"].into();

            assert_eq!(made_hand.power_index(), 5271);
        }

        #[test]
        fn it_returns_2dtd5s8dkhadkd_power_index_415() {
            let made_hand: MadeHand = card_array!["2d", "Td", "5s", "8d", "Kh", "Ad", "Kd"].into();

            assert_eq!(made_hand.power_index(), 415);
        }

        #[test]
        fn it_returns_jsahjd4c7htckc_power_index_3987() {
            let made_hand: MadeHand = card_array!["Js", "Ah", "Jd", "4c", "7h", "Tc", "Kc"].into();

            assert_eq!(made_hand.power_index(), 3987);
        }

        #[test]
        fn it_returns_8cqh5hjs8sjc2d_power_index_2855() {
            let made_hand: MadeHand = card_array!["8c", "Qh", "5h", "Js", "8s", "Jc", "2d"].into();

            assert_eq!(made_hand.power_index(), 2855);
        }

        #[test]
        fn it_returns_6djd7d8s2htc4d_power_index_7237() {
            let made_hand: MadeHand = card_array!["6d", "Jd", "7d", "8s", "2h", "Tc", "4d"].into();

            assert_eq!(made_hand.power_index(), 7237);
        }

        #[test]
        fn it_returns_ad6c7sqs4d9d2s_power_index_6420() {
            let made_hand: MadeHand = card_array!["Ad", "6c", "7s", "Qs", "4d", "9d", "2s"].into();

            assert_eq!(made_hand.power_index(), 6420);
        }

        #[test]
        fn it_returns_qsqc3hjd3c9dqh_power_index_201() {
            let made_hand: MadeHand = card_array!["Qs", "Qc", "3h", "Jd", "3c", "9d", "Qh"].into();

            assert_eq!(made_hand.power_index(), 201);
        }

        #[test]
        fn it_returns_qc9h6d4d3dkc5c_power_index_6753() {
            let made_hand: MadeHand = card_array!["Qc", "9h", "6d", "4d", "3d", "Kc", "5c"].into();

            assert_eq!(made_hand.power_index(), 6753);
        }

        #[test]
        fn it_returns_td8cjd7c9s9h8s_power_index_1603() {
            let made_hand: MadeHand = card_array!["Td", "8c", "Jd", "7c", "9s", "9h", "8s"].into();

            assert_eq!(made_hand.power_index(), 1603);
        }

        #[test]
        fn it_returns_9s4cacthjc7cad_power_index_3426() {
            let made_hand: MadeHand = card_array!["9s", "4c", "Ac", "Th", "Jc", "7c", "Ad"].into();

            assert_eq!(made_hand.power_index(), 3426);
        }

        #[test]
        fn it_returns_7sah8skd3c6s2s_power_index_6315() {
            let made_hand: MadeHand = card_array!["7s", "Ah", "8s", "Kd", "3c", "6s", "2s"].into();

            assert_eq!(made_hand.power_index(), 6315);
        }

        #[test]
        fn it_returns_3das3cah4c9dqh_power_index_2579() {
            let made_hand: MadeHand = card_array!["3d", "As", "3c", "Ah", "4c", "9d", "Qh"].into();

            assert_eq!(made_hand.power_index(), 2579);
        }

        #[test]
        fn it_returns_5h4h8s2d7cqcas_power_index_6436() {
            let made_hand: MadeHand = card_array!["5h", "4h", "8s", "2d", "7c", "Qc", "As"].into();

            assert_eq!(made_hand.power_index(), 6436);
        }

        #[test]
        fn it_returns_3h9h5d6d6cqh4h_power_index_5203() {
            let made_hand: MadeHand = card_array!["3h", "9h", "5d", "6d", "6c", "Qh", "4h"].into();

            assert_eq!(made_hand.power_index(), 5203);
        }

        #[test]
        fn it_returns_3h9h7s9dqc8d2s_power_index_4541() {
            let made_hand: MadeHand = card_array!["3h", "9h", "7s", "9d", "Qc", "8d", "2s"].into();

            assert_eq!(made_hand.power_index(), 4541);
        }

        #[test]
        fn it_returns_6sthac5c6djd4h_power_index_5105() {
            let made_hand: MadeHand = card_array!["6s", "Th", "Ac", "5c", "6d", "Jd", "4h"].into();

            assert_eq!(made_hand.power_index(), 5105);
        }

        #[test]
        fn it_returns_jc5d5c7djs6s3d_power_index_2892() {
            let made_hand: MadeHand = card_array!["Jc", "5d", "5c", "7d", "Js", "6s", "3d"].into();

            assert_eq!(made_hand.power_index(), 2892);
        }

        #[test]
        fn it_returns_td4sjstsac7d5d_power_index_4227() {
            let made_hand: MadeHand = card_array!["Td", "4s", "Js", "Ts", "Ac", "7d", "5d"].into();

            assert_eq!(made_hand.power_index(), 4227);
        }

        #[test]
        fn it_returns_qdksqh7h2c6s4h_power_index_3851() {
            let made_hand: MadeHand = card_array!["Qd", "Ks", "Qh", "7h", "2c", "6s", "4h"].into();

            assert_eq!(made_hand.power_index(), 3851);
        }

        #[test]
        fn it_returns_ah2h9s9dqh2d7c_power_index_3084() {
            let made_hand: MadeHand = card_array!["Ah", "2h", "9s", "9d", "Qh", "2d", "7c"].into();

            assert_eq!(made_hand.power_index(), 3084);
        }

        #[test]
        fn it_returns_7hqc3c9cjc5sad_power_index_6359() {
            let made_hand: MadeHand = card_array!["7h", "Qc", "3c", "9c", "Jc", "5s", "Ad"].into();

            assert_eq!(made_hand.power_index(), 6359);
        }

        #[test]
        fn it_returns_5stdah8dqcqs2h_power_index_3786() {
            let made_hand: MadeHand = card_array!["5s", "Td", "Ah", "8d", "Qc", "Qs", "2h"].into();

            assert_eq!(made_hand.power_index(), 3786);
        }

        #[test]
        fn it_returns_qd8c4s6dqc6s9c_power_index_2780() {
            let made_hand: MadeHand = card_array!["Qd", "8c", "4s", "6d", "Qc", "6s", "9c"].into();

            assert_eq!(made_hand.power_index(), 2780);
        }

        #[test]
        fn it_returns_3c5dqh2c9c8hkd_power_index_6744() {
            let made_hand: MadeHand = card_array!["3c", "5d", "Qh", "2c", "9c", "8h", "Kd"].into();

            assert_eq!(made_hand.power_index(), 6744);
        }

        #[test]
        fn it_returns_4htsjc7h8c6c5c_power_index_1606() {
            let made_hand: MadeHand = card_array!["4h", "Ts", "Jc", "7h", "8c", "6c", "5c"].into();

            assert_eq!(made_hand.power_index(), 1606);
        }

        #[test]
        fn it_returns_ts4c5c7ctcjc6c_power_index_1389() {
            let made_hand: MadeHand = card_array!["Ts", "4c", "5c", "7c", "Tc", "Jc", "6c"].into();

            assert_eq!(made_hand.power_index(), 1389);
        }

        #[test]
        fn it_returns_th6d9s5s7s2sqd_power_index_7097() {
            let made_hand: MadeHand = card_array!["Th", "6d", "9s", "5s", "7s", "2s", "Qd"].into();

            assert_eq!(made_hand.power_index(), 7097);
        }

        #[test]
        fn it_returns_5c5dac2c9hadjd_power_index_2558() {
            let made_hand: MadeHand = card_array!["5c", "5d", "Ac", "2c", "9h", "Ad", "Jd"].into();

            assert_eq!(made_hand.power_index(), 2558);
        }

        #[test]
        fn it_returns_5s5d3stskd3h7c_power_index_3272() {
            let made_hand: MadeHand = card_array!["5s", "5d", "3s", "Ts", "Kd", "3h", "7c"].into();

            assert_eq!(made_hand.power_index(), 3272);
        }

        #[test]
        fn it_returns_2c3h5c3s5s6d4c_power_index_1608() {
            let made_hand: MadeHand = card_array!["2c", "3h", "5c", "3s", "5s", "6d", "4c"].into();

            assert_eq!(made_hand.power_index(), 1608);
        }

        #[test]
        fn it_returns_ahkc3c8h3s3hjh_power_index_2336() {
            let made_hand: MadeHand = card_array!["Ah", "Kc", "3c", "8h", "3s", "3h", "Jh"].into();

            assert_eq!(made_hand.power_index(), 2336);
        }

        #[test]
        fn it_returns_jhtdjd2skhth6d_power_index_2832() {
            let made_hand: MadeHand = card_array!["Jh", "Td", "Jd", "2s", "Kh", "Th", "6d"].into();

            assert_eq!(made_hand.power_index(), 2832);
        }

        #[test]
        fn it_returns_qctd4hjc8s2s4c_power_index_5626() {
            let made_hand: MadeHand = card_array!["Qc", "Td", "4h", "Jc", "8s", "2s", "4c"].into();

            assert_eq!(made_hand.power_index(), 5626);
        }

        #[test]
        fn it_returns_kc9c2dks8s7cad_power_index_3573() {
            let made_hand: MadeHand = card_array!["Kc", "9c", "2d", "Ks", "8s", "7c", "Ad"].into();

            assert_eq!(made_hand.power_index(), 3573);
        }

        #[test]
        fn it_returns_3d5c9c8c2s2h3h_power_index_3320() {
            let made_hand: MadeHand = card_array!["3d", "5c", "9c", "8c", "2s", "2h", "3h"].into();

            assert_eq!(made_hand.power_index(), 3320);
        }

        #[test]
        fn it_returns_adas5hqd6dac7h_power_index_1625() {
            let made_hand: MadeHand = card_array!["Ad", "As", "5h", "Qd", "6d", "Ac", "7h"].into();

            assert_eq!(made_hand.power_index(), 1625);
        }

        #[test]
        fn it_returns_4hjd8c7d7c4d8h_power_index_3098() {
            let made_hand: MadeHand = card_array!["4h", "Jd", "8c", "7d", "7c", "4d", "8h"].into();

            assert_eq!(made_hand.power_index(), 3098);
        }

        #[test]
        fn it_returns_4dts4casks8c5s_power_index_5528() {
            let made_hand: MadeHand = card_array!["4d", "Ts", "4c", "As", "Ks", "8c", "5s"].into();

            assert_eq!(made_hand.power_index(), 5528);
        }

        #[test]
        fn it_returns_qs4d5h7s2cac9s_power_index_6421() {
            let made_hand: MadeHand = card_array!["Qs", "4d", "5h", "7s", "2c", "Ac", "9s"].into();

            assert_eq!(made_hand.power_index(), 6421);
        }

        #[test]
        fn it_returns_tdac6c8hts3h6h_power_index_2963() {
            let made_hand: MadeHand = card_array!["Td", "Ac", "6c", "8h", "Ts", "3h", "6h"].into();

            assert_eq!(made_hand.power_index(), 2963);
        }

        #[test]
        fn it_returns_5h8cacjsah6s8h_power_index_2525() {
            let made_hand: MadeHand = card_array!["5h", "8c", "Ac", "Js", "Ah", "6s", "8h"].into();

            assert_eq!(made_hand.power_index(), 2525);
        }

        #[test]
        fn it_returns_9s2d3c8h7s5h9h_power_index_4612() {
            let made_hand: MadeHand = card_array!["9s", "2d", "3c", "8h", "7s", "5h", "9h"].into();

            assert_eq!(made_hand.power_index(), 4612);
        }

        #[test]
        fn it_returns_th9d2c8s6c8htc_power_index_2945() {
            let made_hand: MadeHand = card_array!["Th", "9d", "2c", "8s", "6c", "8h", "Tc"].into();

            assert_eq!(made_hand.power_index(), 2945);
        }

        #[test]
        fn it_returns_5cqdjh7h6s2ckd_power_index_6699() {
            let made_hand: MadeHand = card_array!["5c", "Qd", "Jh", "7h", "6s", "2c", "Kd"].into();

            assert_eq!(made_hand.power_index(), 6699);
        }

        #[test]
        fn it_returns_9d3c6c7skcks5h_power_index_3716() {
            let made_hand: MadeHand = card_array!["9d", "3c", "6c", "7s", "Kc", "Ks", "5h"].into();

            assert_eq!(made_hand.power_index(), 3716);
        }

        #[test]
        fn it_returns_ksjc7sjdkc7hqs_power_index_2612() {
            let made_hand: MadeHand = card_array!["Ks", "Jc", "7s", "Jd", "Kc", "7h", "Qs"].into();

            assert_eq!(made_hand.power_index(), 2612);
        }

        #[test]
        fn it_returns_tc8h2sac5c4s8s_power_index_4676() {
            let made_hand: MadeHand = card_array!["Tc", "8h", "2s", "Ac", "5c", "4s", "8s"].into();

            assert_eq!(made_hand.power_index(), 4676);
        }

        #[test]
        fn it_returns_ac5h9hqs6s5s9s_power_index_3051() {
            let made_hand: MadeHand = card_array!["Ac", "5h", "9h", "Qs", "6s", "5s", "9s"].into();

            assert_eq!(made_hand.power_index(), 3051);
        }

        #[test]
        fn it_returns_qs9cqhjh8c6sqc_power_index_1764() {
            let made_hand: MadeHand = card_array!["Qs", "9c", "Qh", "Jh", "8c", "6s", "Qc"].into();

            assert_eq!(made_hand.power_index(), 1764);
        }

        #[test]
        fn it_returns_7h5hks6htd6s2d_power_index_5160() {
            let made_hand: MadeHand = card_array!["7h", "5h", "Ks", "6h", "Td", "6s", "2d"].into();

            assert_eq!(made_hand.power_index(), 5160);
        }

        #[test]
        fn it_returns_th4s4had3s9sjs_power_index_5545() {
            let made_hand: MadeHand = card_array!["Th", "4s", "4h", "Ad", "3s", "9s", "Js"].into();

            assert_eq!(made_hand.power_index(), 5545);
        }

        #[test]
        fn it_returns_tsjhqskdqh3d2d_power_index_3821() {
            let made_hand: MadeHand = card_array!["Ts", "Jh", "Qs", "Kd", "Qh", "3d", "2d"].into();

            assert_eq!(made_hand.power_index(), 3821);
        }

        #[test]
        fn it_returns_qd6h9hjdaskd6d_power_index_5086() {
            let made_hand: MadeHand = card_array!["Qd", "6h", "9h", "Jd", "As", "Kd", "6d"].into();

            assert_eq!(made_hand.power_index(), 5086);
        }

        #[test]
        fn it_returns_kdqc8c3h7s6c4h_power_index_6763() {
            let made_hand: MadeHand = card_array!["Kd", "Qc", "8c", "3h", "7s", "6c", "4h"].into();

            assert_eq!(made_hand.power_index(), 6763);
        }

        #[test]
        fn it_returns_6sjcts9s6d8c9d_power_index_3043() {
            let made_hand: MadeHand = card_array!["6s", "Jc", "Ts", "9s", "6d", "8c", "9d"].into();

            assert_eq!(made_hand.power_index(), 3043);
        }

        #[test]
        fn it_returns_8cqc2h2d3dtcks_power_index_6022() {
            let made_hand: MadeHand = card_array!["8c", "Qc", "2h", "2d", "3d", "Tc", "Ks"].into();

            assert_eq!(made_hand.power_index(), 6022);
        }

        #[test]
        fn it_returns_qd6c8d5d9d7d5c_power_index_1285() {
            let made_hand: MadeHand = card_array!["Qd", "6c", "8d", "5d", "9d", "7d", "5c"].into();

            assert_eq!(made_hand.power_index(), 1285);
        }

        #[test]
        fn it_returns_5hqd7d8h9sts9h_power_index_4534() {
            let made_hand: MadeHand = card_array!["5h", "Qd", "7d", "8h", "9s", "Ts", "9h"].into();

            assert_eq!(made_hand.power_index(), 4534);
        }

        #[test]
        fn it_returns_qhth6cqcqd6d4c_power_index_198() {
            let made_hand: MadeHand = card_array!["Qh", "Th", "6c", "Qc", "Qd", "6d", "4c"].into();

            assert_eq!(made_hand.power_index(), 198);
        }

        #[test]
        fn it_returns_6s6c8c2dasjdkc_power_index_5087() {
            let made_hand: MadeHand = card_array!["6s", "6c", "8c", "2d", "As", "Jd", "Kc"].into();

            assert_eq!(made_hand.power_index(), 5087);
        }

        #[test]
        fn it_returns_7h4dad2hqs6d6s_power_index_5100() {
            let made_hand: MadeHand = card_array!["7h", "4d", "Ad", "2h", "Qs", "6d", "6s"].into();

            assert_eq!(made_hand.power_index(), 5100);
        }

        #[test]
        fn it_returns_kdqhjd4d3hahqd_power_index_3766() {
            let made_hand: MadeHand = card_array!["Kd", "Qh", "Jd", "4d", "3h", "Ah", "Qd"].into();

            assert_eq!(made_hand.power_index(), 3766);
        }

        #[test]
        fn it_returns_ts5c8d7sah9ctd_power_index_4233() {
            let made_hand: MadeHand = card_array!["Ts", "5c", "8d", "7s", "Ah", "9c", "Td"].into();

            assert_eq!(made_hand.power_index(), 4233);
        }

        #[test]
        fn it_returns_4dackc3d2d2hqd_power_index_5966() {
            let made_hand: MadeHand = card_array!["4d", "Ac", "Kc", "3d", "2d", "2h", "Qd"].into();

            assert_eq!(made_hand.power_index(), 5966);
        }

        #[test]
        fn it_returns_9h7sjd3c5c7dac_power_index_4886() {
            let made_hand: MadeHand = card_array!["9h", "7s", "Jd", "3c", "5c", "7d", "Ac"].into();

            assert_eq!(made_hand.power_index(), 4886);
        }

        #[test]
        fn it_returns_8c3s5hjh7s4h3h_power_index_5895() {
            let made_hand: MadeHand = card_array!["8c", "3s", "5h", "Jh", "7s", "4h", "3h"].into();

            assert_eq!(made_hand.power_index(), 5895);
        }

        #[test]
        fn it_returns_qh8c3dqc9d5sac_power_index_3793() {
            let made_hand: MadeHand = card_array!["Qh", "8c", "3d", "Qc", "9d", "5s", "Ac"].into();

            assert_eq!(made_hand.power_index(), 3793);
        }

        #[test]
        fn it_returns_ah8c9hjh7cqs2s_power_index_6358() {
            let made_hand: MadeHand = card_array!["Ah", "8c", "9h", "Jh", "7c", "Qs", "2s"].into();

            assert_eq!(made_hand.power_index(), 6358);
        }

        #[test]
        fn it_returns_7c6das3hkdtd4c_power_index_6279() {
            let made_hand: MadeHand = card_array!["7c", "6d", "As", "3h", "Kd", "Td", "4c"].into();

            assert_eq!(made_hand.power_index(), 6279);
        }

        #[test]
        fn it_returns_kd3cth8cqc4d5s_power_index_6723() {
            let made_hand: MadeHand = card_array!["Kd", "3c", "Th", "8c", "Qc", "4d", "5s"].into();

            assert_eq!(made_hand.power_index(), 6723);
        }

        #[test]
        fn it_returns_ts9d5d6sjd2s6h_power_index_5222() {
            let made_hand: MadeHand = card_array!["Ts", "9d", "5d", "6s", "Jd", "2s", "6h"].into();

            assert_eq!(made_hand.power_index(), 5222);
        }

        #[test]
        fn it_returns_as2h6dtdth9dqc_power_index_4217() {
            let made_hand: MadeHand = card_array!["As", "2h", "6d", "Td", "Th", "9d", "Qc"].into();

            assert_eq!(made_hand.power_index(), 4217);
        }

        #[test]
        fn it_returns_7c8c4s4c3sth2d_power_index_5696() {
            let made_hand: MadeHand = card_array!["7c", "8c", "4s", "4c", "3s", "Th", "2d"].into();

            assert_eq!(made_hand.power_index(), 5696);
        }

        #[test]
        fn it_returns_4djc7d4s3htcac_power_index_5545() {
            let made_hand: MadeHand = card_array!["4d", "Jc", "7d", "4s", "3h", "Tc", "Ac"].into();

            assert_eq!(made_hand.power_index(), 5545);
        }

        #[test]
        fn it_returns_9cad8c7dac6cqd_power_index_3398() {
            let made_hand: MadeHand = card_array!["9c", "Ad", "8c", "7d", "Ac", "6c", "Qd"].into();

            assert_eq!(made_hand.power_index(), 3398);
        }

        #[test]
        fn it_returns_6d9s2hjdts3cqd_power_index_7009() {
            let made_hand: MadeHand = card_array!["6d", "9s", "2h", "Jd", "Ts", "3c", "Qd"].into();

            assert_eq!(made_hand.power_index(), 7009);
        }

        #[test]
        fn it_returns_2s5c2hkskh7s4c_power_index_2716() {
            let made_hand: MadeHand = card_array!["2s", "5c", "2h", "Ks", "Kh", "7s", "4c"].into();

            assert_eq!(made_hand.power_index(), 2716);
        }

        #[test]
        fn it_returns_3c9sas6sac7d4h_power_index_3496() {
            let made_hand: MadeHand = card_array!["3c", "9s", "As", "6s", "Ac", "7d", "4h"].into();

            assert_eq!(made_hand.power_index(), 3496);
        }

        #[test]
        fn it_returns_qd4h3s9c6dtdjc_power_index_7009() {
            let made_hand: MadeHand = card_array!["Qd", "4h", "3s", "9c", "6d", "Td", "Jc"].into();

            assert_eq!(made_hand.power_index(), 7009);
        }

        #[test]
        fn it_returns_ah2d5dqhqc4has_power_index_2486() {
            let made_hand: MadeHand = card_array!["Ah", "2d", "5d", "Qh", "Qc", "4h", "As"].into();

            assert_eq!(made_hand.power_index(), 2486);
        }

        #[test]
        fn it_returns_4h3sjc7c6hth2s_power_index_7253() {
            let made_hand: MadeHand = card_array!["4h", "3s", "Jc", "7c", "6h", "Th", "2s"].into();

            assert_eq!(made_hand.power_index(), 7253);
        }

        #[test]
        fn it_returns_6h5h9c8h7s2h7d_power_index_1605() {
            let made_hand: MadeHand = card_array!["6h", "5h", "9c", "8h", "7s", "2h", "7d"].into();

            assert_eq!(made_hand.power_index(), 1605);
        }

        #[test]
        fn it_returns_8d4d3stc8s8c8h_power_index_87() {
            let made_hand: MadeHand = card_array!["8d", "4d", "3s", "Tc", "8s", "8c", "8h"].into();

            assert_eq!(made_hand.power_index(), 87);
        }

        #[test]
        fn it_returns_7h8c4djc4c6c2c_power_index_1458() {
            let made_hand: MadeHand = card_array!["7h", "8c", "4d", "Jc", "4c", "6c", "2c"].into();

            assert_eq!(made_hand.power_index(), 1458);
        }

        #[test]
        fn it_returns_6djs4cjh2d3d9s_power_index_4162() {
            let made_hand: MadeHand = card_array!["6d", "Js", "4c", "Jh", "2d", "3d", "9s"].into();

            assert_eq!(made_hand.power_index(), 4162);
        }

        #[test]
        fn it_returns_9h7had7c4dkd8d_power_index_4869() {
            let made_hand: MadeHand = card_array!["9h", "7h", "Ad", "7c", "4d", "Kd", "8d"].into();

            assert_eq!(made_hand.power_index(), 4869);
        }

        #[test]
        fn it_returns_jc5h2c2h9stc8s_power_index_6102() {
            let made_hand: MadeHand = card_array!["Jc", "5h", "2c", "2h", "9s", "Tc", "8s"].into();

            assert_eq!(made_hand.power_index(), 6102);
        }

        #[test]
        fn it_returns_6d5c8h9sqckd9h_power_index_4483() {
            let made_hand: MadeHand = card_array!["6d", "5c", "8h", "9s", "Qc", "Kd", "9h"].into();

            assert_eq!(made_hand.power_index(), 4483);
        }

        #[test]
        fn it_returns_6d6had9dqs3d2s_power_index_5098() {
            let made_hand: MadeHand = card_array!["6d", "6h", "Ad", "9d", "Qs", "3d", "2s"].into();

            assert_eq!(made_hand.power_index(), 5098);
        }

        #[test]
        fn it_returns_4cqckc6s9h3dqs_power_index_3840() {
            let made_hand: MadeHand = card_array!["4c", "Qc", "Kc", "6s", "9h", "3d", "Qs"].into();

            assert_eq!(made_hand.power_index(), 3840);
        }

        #[test]
        fn it_returns_4ctdth2c6dah5d_power_index_4251() {
            let made_hand: MadeHand = card_array!["4c", "Td", "Th", "2c", "6d", "Ah", "5d"].into();

            assert_eq!(made_hand.power_index(), 4251);
        }

        #[test]
        fn it_returns_7d2d2h2ctsjdkc_power_index_2414() {
            let made_hand: MadeHand = card_array!["7d", "2d", "2h", "2c", "Ts", "Jd", "Kc"].into();

            assert_eq!(made_hand.power_index(), 2414);
        }

        #[test]
        fn it_returns_5h2c5c9dtcjs6c_power_index_5442() {
            let made_hand: MadeHand = card_array!["5h", "2c", "5c", "9d", "Tc", "Js", "6c"].into();

            assert_eq!(made_hand.power_index(), 5442);
        }

        #[test]
        fn it_returns_5h5c6d2dkh8hjh_power_index_5372() {
            let made_hand: MadeHand = card_array!["5h", "5c", "6d", "2d", "Kh", "8h", "Jh"].into();

            assert_eq!(made_hand.power_index(), 5372);
        }

        #[test]
        fn it_returns_2d2s9h4dqh6d5s_power_index_6083() {
            let made_hand: MadeHand = card_array!["2d", "2s", "9h", "4d", "Qh", "6d", "5s"].into();

            assert_eq!(made_hand.power_index(), 6083);
        }

        #[test]
        fn it_returns_4dad5stdkdqc2d_power_index_429() {
            let made_hand: MadeHand = card_array!["4d", "Ad", "5s", "Td", "Kd", "Qc", "2d"].into();

            assert_eq!(made_hand.power_index(), 429);
        }

        #[test]
        fn it_returns_8cthas7s9ctc6c_power_index_1604() {
            let made_hand: MadeHand = card_array!["8c", "Th", "As", "7s", "9c", "Tc", "6c"].into();

            assert_eq!(made_hand.power_index(), 1604);
        }

        #[test]
        fn it_returns_3hah7c5dtcts5c_power_index_2974() {
            let made_hand: MadeHand = card_array!["3h", "Ah", "7c", "5d", "Tc", "Ts", "5c"].into();

            assert_eq!(made_hand.power_index(), 2974);
        }

        #[test]
        fn it_returns_2h7d3ckc6sac8d_power_index_6315() {
            let made_hand: MadeHand = card_array!["2h", "7d", "3c", "Kc", "6s", "Ac", "8d"].into();

            assert_eq!(made_hand.power_index(), 6315);
        }

        #[test]
        fn it_returns_2c6sqdth4htd7d_power_index_4327() {
            let made_hand: MadeHand = card_array!["2c", "6s", "Qd", "Th", "4h", "Td", "7d"].into();

            assert_eq!(made_hand.power_index(), 4327);
        }

        #[test]
        fn it_returns_5d7d6c9s2d4sas_power_index_6625() {
            let made_hand: MadeHand = card_array!["5d", "7d", "6c", "9s", "2d", "4s", "As"].into();

            assert_eq!(made_hand.power_index(), 6625);
        }

        #[test]
        fn it_returns_jsqh6s5s9c6d2d_power_index_5187() {
            let made_hand: MadeHand = card_array!["Js", "Qh", "6s", "5s", "9c", "6d", "2d"].into();

            assert_eq!(made_hand.power_index(), 5187);
        }

        #[test]
        fn it_returns_kc8c2c6sqs4cth_power_index_6722() {
            let made_hand: MadeHand = card_array!["Kc", "8c", "2c", "6s", "Qs", "4c", "Th"].into();

            assert_eq!(made_hand.power_index(), 6722);
        }

        #[test]
        fn it_returns_kd2sth2djdtskc_power_index_2624() {
            let made_hand: MadeHand = card_array!["Kd", "2s", "Th", "2d", "Jd", "Ts", "Kc"].into();

            assert_eq!(made_hand.power_index(), 2624);
        }

        #[test]
        fn it_returns_jc4d8dqs2h2c4h_power_index_3306() {
            let made_hand: MadeHand = card_array!["Jc", "4d", "8d", "Qs", "2h", "2c", "4h"].into();

            assert_eq!(made_hand.power_index(), 3306);
        }

        #[test]
        fn it_returns_7d5d9sqd7htc2d_power_index_4974() {
            let made_hand: MadeHand = card_array!["7d", "5d", "9s", "Qd", "7h", "Tc", "2d"].into();

            assert_eq!(made_hand.power_index(), 4974);
        }

        #[test]
        fn it_returns_9c3s9skd6sas6c_power_index_3040() {
            let made_hand: MadeHand = card_array!["9c", "3s", "9s", "Kd", "6s", "As", "6c"].into();

            assert_eq!(made_hand.power_index(), 3040);
        }

        #[test]
        fn it_returns_qd4c8dkc5djcqc_power_index_3823() {
            let made_hand: MadeHand = card_array!["Qd", "4c", "8d", "Kc", "5d", "Jc", "Qc"].into();

            assert_eq!(made_hand.power_index(), 3823);
        }

        #[test]
        fn it_returns_2h2c3c9d8c9cas_power_index_3084() {
            let made_hand: MadeHand = card_array!["2h", "2c", "3c", "9d", "8c", "9c", "As"].into();

            assert_eq!(made_hand.power_index(), 3084);
        }

        #[test]
        fn it_returns_ackh5c9s9h7h7c_power_index_3029() {
            let made_hand: MadeHand = card_array!["Ac", "Kh", "5c", "9s", "9h", "7h", "7c"].into();

            assert_eq!(made_hand.power_index(), 3029);
        }

        #[test]
        fn it_returns_3hjctdtcahkd6s_power_index_4207() {
            let made_hand: MadeHand = card_array!["3h", "Jc", "Td", "Tc", "Ah", "Kd", "6s"].into();

            assert_eq!(made_hand.power_index(), 4207);
        }

        #[test]
        fn it_returns_7c6s9cjcahqs3h_power_index_6359() {
            let made_hand: MadeHand = card_array!["7c", "6s", "9c", "Jc", "Ah", "Qs", "3h"].into();

            assert_eq!(made_hand.power_index(), 6359);
        }

        #[test]
        fn it_returns_qd3hjc2s4hjd3d_power_index_2910() {
            let made_hand: MadeHand = card_array!["Qd", "3h", "Jc", "2s", "4h", "Jd", "3d"].into();

            assert_eq!(made_hand.power_index(), 2910);
        }

        #[test]
        fn it_returns_qsas3hjhjc2cth_power_index_3996() {
            let made_hand: MadeHand = card_array!["Qs", "As", "3h", "Jh", "Jc", "2c", "Th"].into();

            assert_eq!(made_hand.power_index(), 3996);
        }

        #[test]
        fn it_returns_jhkc9h6stc4c9d_power_index_4490() {
            let made_hand: MadeHand = card_array!["Jh", "Kc", "9h", "6s", "Tc", "4c", "9d"].into();

            assert_eq!(made_hand.power_index(), 4490);
        }

        #[test]
        fn it_returns_6sah9c9dqh2h6h_power_index_3040() {
            let made_hand: MadeHand = card_array!["6s", "Ah", "9c", "9d", "Qh", "2h", "6h"].into();

            assert_eq!(made_hand.power_index(), 3040);
        }

        #[test]
        fn it_returns_8s8dqd2d8h4sqc_power_index_241() {
            let made_hand: MadeHand = card_array!["8s", "8d", "Qd", "2d", "8h", "4s", "Qc"].into();

            assert_eq!(made_hand.power_index(), 241);
        }

        #[test]
        fn it_returns_kdtc2h7stsks2c_power_index_2627() {
            let made_hand: MadeHand = card_array!["Kd", "Tc", "2h", "7s", "Ts", "Ks", "2c"].into();

            assert_eq!(made_hand.power_index(), 2627);
        }

        #[test]
        fn it_returns_khksqd9d9s2h8s_power_index_2634() {
            let made_hand: MadeHand = card_array!["Kh", "Ks", "Qd", "9d", "9s", "2h", "8s"].into();

            assert_eq!(made_hand.power_index(), 2634);
        }

        #[test]
        fn it_returns_3stsjh5dqhqd9h_power_index_3866() {
            let made_hand: MadeHand = card_array!["3s", "Ts", "Jh", "5d", "Qh", "Qd", "9h"].into();

            assert_eq!(made_hand.power_index(), 3866);
        }

        #[test]
        fn it_returns_jh6d9s2hks3d2c_power_index_6031() {
            let made_hand: MadeHand = card_array!["Jh", "6d", "9s", "2h", "Ks", "3d", "2c"].into();

            assert_eq!(made_hand.power_index(), 6031);
        }

        #[test]
        fn it_returns_9sac3ctd4s9h6s_power_index_4455() {
            let made_hand: MadeHand = card_array!["9s", "Ac", "3c", "Td", "4s", "9h", "6s"].into();

            assert_eq!(made_hand.power_index(), 4455);
        }

        #[test]
        fn it_returns_kd5dad7c7dkcjs_power_index_2655() {
            let made_hand: MadeHand = card_array!["Kd", "5d", "Ad", "7c", "7d", "Kc", "Js"].into();

            assert_eq!(made_hand.power_index(), 2655);
        }

        #[test]
        fn it_returns_5c4d3skdah9ckc_power_index_3576() {
            let made_hand: MadeHand = card_array!["5c", "4d", "3s", "Kd", "Ah", "9c", "Kc"].into();

            assert_eq!(made_hand.power_index(), 3576);
        }

        #[test]
        fn it_returns_qc3ckhqhjd9s5d_power_index_3822() {
            let made_hand: MadeHand = card_array!["Qc", "3c", "Kh", "Qh", "Jd", "9s", "5d"].into();

            assert_eq!(made_hand.power_index(), 3822);
        }

        #[test]
        fn it_returns_8haskh7sjh7hth_power_index_942() {
            let made_hand: MadeHand = card_array!["8h", "As", "Kh", "7s", "Jh", "7h", "Th"].into();

            assert_eq!(made_hand.power_index(), 942);
        }

        #[test]
        fn it_returns_5d4dkd2h2dqhts_power_index_6022() {
            let made_hand: MadeHand = card_array!["5d", "4d", "Kd", "2h", "2d", "Qh", "Ts"].into();

            assert_eq!(made_hand.power_index(), 6022);
        }

        #[test]
        fn it_returns_tcahqs2has4h6c_power_index_3393() {
            let made_hand: MadeHand = card_array!["Tc", "Ah", "Qs", "2h", "As", "4h", "6c"].into();

            assert_eq!(made_hand.power_index(), 3393);
        }

        #[test]
        fn it_returns_9c4s6d3dahjcjh_power_index_4015() {
            let made_hand: MadeHand = card_array!["9c", "4s", "6d", "3d", "Ah", "Jc", "Jh"].into();

            assert_eq!(made_hand.power_index(), 4015);
        }

        #[test]
        fn it_returns_ks7cts2sjc9dtd_power_index_4270() {
            let made_hand: MadeHand = card_array!["Ks", "7c", "Ts", "2s", "Jc", "9d", "Td"].into();

            assert_eq!(made_hand.power_index(), 4270);
        }

        #[test]
        fn it_returns_qdkc3h4c5c5h2h_power_index_5367() {
            let made_hand: MadeHand = card_array!["Qd", "Kc", "3h", "4c", "5c", "5h", "2h"].into();

            assert_eq!(made_hand.power_index(), 5367);
        }

        #[test]
        fn it_returns_6d8h2c5d5skd8s_power_index_3118() {
            let made_hand: MadeHand = card_array!["6d", "8h", "2c", "5d", "5s", "Kd", "8s"].into();

            assert_eq!(made_hand.power_index(), 3118);
        }

        #[test]
        fn it_returns_5hjh3s2s9sqdqh_power_index_3877() {
            let made_hand: MadeHand = card_array!["5h", "Jh", "3s", "2s", "9s", "Qd", "Qh"].into();

            assert_eq!(made_hand.power_index(), 3877);
        }

        #[test]
        fn it_returns_asjsqcad5skc8h_power_index_3326() {
            let made_hand: MadeHand = card_array!["As", "Js", "Qc", "Ad", "5s", "Kc", "8h"].into();

            assert_eq!(made_hand.power_index(), 3326);
        }

        #[test]
        fn it_returns_6sjd3h3djskc9s_power_index_2909() {
            let made_hand: MadeHand = card_array!["6s", "Jd", "3h", "3d", "Js", "Kc", "9s"].into();

            assert_eq!(made_hand.power_index(), 2909);
        }

        #[test]
        fn it_returns_3h3s5s2s7das7s_power_index_810() {
            let made_hand: MadeHand = card_array!["3h", "3s", "5s", "2s", "7d", "As", "7s"].into();

            assert_eq!(made_hand.power_index(), 810);
        }

        #[test]
        fn it_returns_qd6h4d6dth8c9s_power_index_5194() {
            let made_hand: MadeHand = card_array!["Qd", "6h", "4d", "6d", "Th", "8c", "9s"].into();

            assert_eq!(made_hand.power_index(), 5194);
        }

        #[test]
        fn it_returns_7c9sjc2dqc7h8h_power_index_4967() {
            let made_hand: MadeHand = card_array!["7c", "9s", "Jc", "2d", "Qc", "7h", "8h"].into();

            assert_eq!(made_hand.power_index(), 4967);
        }

        #[test]
        fn it_returns_7dqc5d6hkhtcqs_power_index_3832() {
            let made_hand: MadeHand = card_array!["7d", "Qc", "5d", "6h", "Kh", "Tc", "Qs"].into();

            assert_eq!(made_hand.power_index(), 3832);
        }

        #[test]
        fn it_returns_4s5dtsqh2s3hqd_power_index_3924() {
            let made_hand: MadeHand = card_array!["4s", "5d", "Ts", "Qh", "2s", "3h", "Qd"].into();

            assert_eq!(made_hand.power_index(), 3924);
        }

        #[test]
        fn it_returns_4djc3d4h8c2s2d_power_index_3307() {
            let made_hand: MadeHand = card_array!["4d", "Jc", "3d", "4h", "8c", "2s", "2d"].into();

            assert_eq!(made_hand.power_index(), 3307);
        }

        #[test]
        fn it_returns_4d9sas6ckdtdqd_power_index_6194() {
            let made_hand: MadeHand = card_array!["4d", "9s", "As", "6c", "Kd", "Td", "Qd"].into();

            assert_eq!(made_hand.power_index(), 6194);
        }

        #[test]
        fn it_returns_asthqdjc6dtd5d_power_index_4216() {
            let made_hand: MadeHand = card_array!["As", "Th", "Qd", "Jc", "6d", "Td", "5d"].into();

            assert_eq!(made_hand.power_index(), 4216);
        }

        #[test]
        fn it_returns_td3cjc4s7dkh9h_power_index_6799() {
            let made_hand: MadeHand = card_array!["Td", "3c", "Jc", "4s", "7d", "Kh", "9h"].into();

            assert_eq!(made_hand.power_index(), 6799);
        }

        #[test]
        fn it_returns_5ststh3s9htcqs_power_index_1896() {
            let made_hand: MadeHand = card_array!["5s", "Ts", "Th", "3s", "9h", "Tc", "Qs"].into();

            assert_eq!(made_hand.power_index(), 1896);
        }

        #[test]
        fn it_returns_9d3sac5c4ckcth_power_index_6269() {
            let made_hand: MadeHand = card_array!["9d", "3s", "Ac", "5c", "4c", "Kc", "Th"].into();

            assert_eq!(made_hand.power_index(), 6269);
        }

        #[test]
        fn it_returns_kh7c6c2d5d4d5s_power_index_5396() {
            let made_hand: MadeHand = card_array!["Kh", "7c", "6c", "2d", "5d", "4d", "5s"].into();

            assert_eq!(made_hand.power_index(), 5396);
        }

        #[test]
        fn it_returns_5dkcjctcqd7d4c_power_index_6680() {
            let made_hand: MadeHand = card_array!["5d", "Kc", "Jc", "Tc", "Qd", "7d", "4c"].into();

            assert_eq!(made_hand.power_index(), 6680);
        }

        #[test]
        fn it_returns_kc4hjdacjc3djh_power_index_1808() {
            let made_hand: MadeHand = card_array!["Kc", "4h", "Jd", "Ac", "Jc", "3d", "Jh"].into();

            assert_eq!(made_hand.power_index(), 1808);
        }

        #[test]
        fn it_returns_9c5c3c2h2sjd5d_power_index_3285() {
            let made_hand: MadeHand = card_array!["9c", "5c", "3c", "2h", "2s", "Jd", "5d"].into();

            assert_eq!(made_hand.power_index(), 3285);
        }

        #[test]
        fn it_returns_acqh7h6hks3s5h_power_index_6215() {
            let made_hand: MadeHand = card_array!["Ac", "Qh", "7h", "6h", "Ks", "3s", "5h"].into();

            assert_eq!(made_hand.power_index(), 6215);
        }

        #[test]
        fn it_returns_kcad3h2dastc4c_power_index_3350() {
            let made_hand: MadeHand = card_array!["Kc", "Ad", "3h", "2d", "As", "Tc", "4c"].into();

            assert_eq!(made_hand.power_index(), 3350);
        }

        #[test]
        fn it_returns_4c5dac6c8c6sks_power_index_5090() {
            let made_hand: MadeHand = card_array!["4c", "5d", "Ac", "6c", "8c", "6s", "Ks"].into();

            assert_eq!(made_hand.power_index(), 5090);
        }

        #[test]
        fn it_returns_6d2d4d6s9sqd8d_power_index_1333() {
            let made_hand: MadeHand = card_array!["6d", "2d", "4d", "6s", "9s", "Qd", "8d"].into();

            assert_eq!(made_hand.power_index(), 1333);
        }

        #[test]
        fn it_returns_qh5h4d8h6c3cjd_power_index_7061() {
            let made_hand: MadeHand = card_array!["Qh", "5h", "4d", "8h", "6c", "3c", "Jd"].into();

            assert_eq!(made_hand.power_index(), 7061);
        }

        #[test]
        fn it_returns_acas6dahjd5c5d_power_index_175() {
            let made_hand: MadeHand = card_array!["Ac", "As", "6d", "Ah", "Jd", "5c", "5d"].into();

            assert_eq!(made_hand.power_index(), 175);
        }

        #[test]
        fn it_returns_jc8dth7s9s2d6s_power_index_1603() {
            let made_hand: MadeHand = card_array!["Jc", "8d", "Th", "7s", "9s", "2d", "6s"].into();

            assert_eq!(made_hand.power_index(), 1603);
        }

        #[test]
        fn it_returns_8djs6d5dkh9sad_power_index_6238() {
            let made_hand: MadeHand = card_array!["8d", "Js", "6d", "5d", "Kh", "9s", "Ad"].into();

            assert_eq!(made_hand.power_index(), 6238);
        }

        #[test]
        fn it_returns_9s9cjh8hqc2dkd_power_index_4481() {
            let made_hand: MadeHand = card_array!["9s", "9c", "Jh", "8h", "Qc", "2d", "Kd"].into();

            assert_eq!(made_hand.power_index(), 4481);
        }

        #[test]
        fn it_returns_adjh6skc5d6h8s_power_index_5087() {
            let made_hand: MadeHand = card_array!["Ad", "Jh", "6s", "Kc", "5d", "6h", "8s"].into();

            assert_eq!(made_hand.power_index(), 5087);
        }

        #[test]
        fn it_returns_7c4c8hks2h7s9s_power_index_4945() {
            let made_hand: MadeHand = card_array!["7c", "4c", "8h", "Ks", "2h", "7s", "9s"].into();

            assert_eq!(made_hand.power_index(), 4945);
        }

        #[test]
        fn it_returns_jh4s8d3s9h6dac_power_index_6499() {
            let made_hand: MadeHand = card_array!["Jh", "4s", "8d", "3s", "9h", "6d", "Ac"].into();

            assert_eq!(made_hand.power_index(), 6499);
        }

        #[test]
        fn it_returns_2cqdqc4c7h4s5h_power_index_2804() {
            let made_hand: MadeHand = card_array!["2c", "Qd", "Qc", "4c", "7h", "4s", "5h"].into();

            assert_eq!(made_hand.power_index(), 2804);
        }

        #[test]
        fn it_returns_6c3h6d8c7h2d2s_power_index_3255() {
            let made_hand: MadeHand = card_array!["6c", "3h", "6d", "8c", "7h", "2d", "2s"].into();

            assert_eq!(made_hand.power_index(), 3255);
        }

        #[test]
        fn it_returns_jc7s6ckhkc2d5s_power_index_3667() {
            let made_hand: MadeHand = card_array!["Jc", "7s", "6c", "Kh", "Kc", "2d", "5s"].into();

            assert_eq!(made_hand.power_index(), 3667);
        }

        #[test]
        fn it_returns_kc7s8d6h8ctcad_power_index_4648() {
            let made_hand: MadeHand = card_array!["Kc", "7s", "8d", "6h", "8c", "Tc", "Ad"].into();

            assert_eq!(made_hand.power_index(), 4648);
        }

        #[test]
        fn it_returns_2d7h2hqh3dqckd_power_index_2821() {
            let made_hand: MadeHand = card_array!["2d", "7h", "2h", "Qh", "3d", "Qc", "Kd"].into();

            assert_eq!(made_hand.power_index(), 2821);
        }

        #[test]
        fn it_returns_ah3c8s8hqc4stc_power_index_4657() {
            let made_hand: MadeHand = card_array!["Ah", "3c", "8s", "8h", "Qc", "4s", "Tc"].into();

            assert_eq!(made_hand.power_index(), 4657);
        }

        #[test]
        fn it_returns_8sad4c9s8h2c6c_power_index_4681() {
            let made_hand: MadeHand = card_array!["8s", "Ad", "4c", "9s", "8h", "2c", "6c"].into();

            assert_eq!(made_hand.power_index(), 4681);
        }

        #[test]
        fn it_returns_qsqc6sthas9s6h_power_index_2776() {
            let made_hand: MadeHand = card_array!["Qs", "Qc", "6s", "Th", "As", "9s", "6h"].into();

            assert_eq!(made_hand.power_index(), 2776);
        }

        #[test]
        fn it_returns_6c4s9h9dacqhts_power_index_4437() {
            let made_hand: MadeHand = card_array!["6c", "4s", "9h", "9d", "Ac", "Qh", "Ts"].into();

            assert_eq!(made_hand.power_index(), 4437);
        }

        #[test]
        fn it_returns_3d4has2s2h4d4s_power_index_298() {
            let made_hand: MadeHand = card_array!["3d", "4h", "As", "2s", "2h", "4d", "4s"].into();

            assert_eq!(made_hand.power_index(), 298);
        }

        #[test]
        fn it_returns_jd9d4htc9s8c7c_power_index_1603() {
            let made_hand: MadeHand = card_array!["Jd", "9d", "4h", "Tc", "9s", "8c", "7c"].into();

            assert_eq!(made_hand.power_index(), 1603);
        }

        #[test]
        fn it_returns_qd2das3s4skd5c_power_index_1609() {
            let made_hand: MadeHand = card_array!["Qd", "2d", "As", "3s", "4s", "Kd", "5c"].into();

            assert_eq!(made_hand.power_index(), 1609);
        }

        #[test]
        fn it_returns_ksjd6d9c8c7ctd_power_index_1603() {
            let made_hand: MadeHand = card_array!["Ks", "Jd", "6d", "9c", "8c", "7c", "Td"].into();

            assert_eq!(made_hand.power_index(), 1603);
        }

        #[test]
        fn it_returns_qsts6d6c8h2c9s_power_index_5194() {
            let made_hand: MadeHand = card_array!["Qs", "Ts", "6d", "6c", "8h", "2c", "9s"].into();

            assert_eq!(made_hand.power_index(), 5194);
        }

        #[test]
        fn it_returns_8hah8s7s2c2sac_power_index_2528() {
            let made_hand: MadeHand = card_array!["8h", "Ah", "8s", "7s", "2c", "2s", "Ac"].into();

            assert_eq!(made_hand.power_index(), 2528);
        }

        #[test]
        fn it_returns_6s6has8dtckd2h_power_index_5088() {
            let made_hand: MadeHand = card_array!["6s", "6h", "As", "8d", "Tc", "Kd", "2h"].into();

            assert_eq!(made_hand.power_index(), 5088);
        }

        #[test]
        fn it_returns_7c4sas7h9d9h3s_power_index_3029() {
            let made_hand: MadeHand = card_array!["7c", "4s", "As", "7h", "9d", "9h", "3s"].into();

            assert_eq!(made_hand.power_index(), 3029);
        }

        #[test]
        fn it_returns_jc3s3h7h7cts5s_power_index_3197() {
            let made_hand: MadeHand = card_array!["Jc", "3s", "3h", "7h", "7c", "Ts", "5s"].into();

            assert_eq!(made_hand.power_index(), 3197);
        }

        #[test]
        fn it_returns_kc4c9c7c2djs4h_power_index_5591() {
            let made_hand: MadeHand = card_array!["Kc", "4c", "9c", "7c", "2d", "Js", "4h"].into();

            assert_eq!(made_hand.power_index(), 5591);
        }

        #[test]
        fn it_returns_kh9d2s8d2h7d2d_power_index_2416() {
            let made_hand: MadeHand = card_array!["Kh", "9d", "2s", "8d", "2h", "7d", "2d"].into();

            assert_eq!(made_hand.power_index(), 2416);
        }

        #[test]
        fn it_returns_5djhkc6c8c3h3c_power_index_5812() {
            let made_hand: MadeHand = card_array!["5d", "Jh", "Kc", "6c", "8c", "3h", "3c"].into();

            assert_eq!(made_hand.power_index(), 5812);
        }

        #[test]
        fn it_returns_6skc6c7s4d5dkd_power_index_2672() {
            let made_hand: MadeHand = card_array!["6s", "Kc", "6c", "7s", "4d", "5d", "Kd"].into();

            assert_eq!(made_hand.power_index(), 2672);
        }

        #[test]
        fn it_returns_4h2sjsks3s3cqc_power_index_5801() {
            let made_hand: MadeHand = card_array!["4h", "2s", "Js", "Ks", "3s", "3c", "Qc"].into();

            assert_eq!(made_hand.power_index(), 5801);
        }

        #[test]
        fn it_returns_8c4d2skdqd3h6d_power_index_6769() {
            let made_hand: MadeHand = card_array!["8c", "4d", "2s", "Kd", "Qd", "3h", "6d"].into();

            assert_eq!(made_hand.power_index(), 6769);
        }

        #[test]
        fn it_returns_7d6skhts7c3d9c_power_index_4938() {
            let made_hand: MadeHand = card_array!["7d", "6s", "Kh", "Ts", "7c", "3d", "9c"].into();

            assert_eq!(made_hand.power_index(), 4938);
        }

        #[test]
        fn it_returns_kdac7s2s4d5s7h_power_index_4872() {
            let made_hand: MadeHand = card_array!["Kd", "Ac", "7s", "2s", "4d", "5s", "7h"].into();

            assert_eq!(made_hand.power_index(), 4872);
        }

        #[test]
        fn it_returns_qsac5d7c9s6d8s_power_index_1605() {
            let made_hand: MadeHand = card_array!["Qs", "Ac", "5d", "7c", "9s", "6d", "8s"].into();

            assert_eq!(made_hand.power_index(), 1605);
        }

        #[test]
        fn it_returns_ts4d2c8s6h3hks_power_index_6909() {
            let made_hand: MadeHand = card_array!["Ts", "4d", "2c", "8s", "6h", "3h", "Ks"].into();

            assert_eq!(made_hand.power_index(), 6909);
        }

        #[test]
        fn it_returns_3d2c4ctdts7d4s_power_index_2991() {
            let made_hand: MadeHand = card_array!["3d", "2c", "4c", "Td", "Ts", "7d", "4s"].into();

            assert_eq!(made_hand.power_index(), 2991);
        }

        #[test]
        fn it_returns_thjs3c4s9d2c8c_power_index_7219() {
            let made_hand: MadeHand = card_array!["Th", "Js", "3c", "4s", "9d", "2c", "8c"].into();

            assert_eq!(made_hand.power_index(), 7219);
        }

        #[test]
        fn it_returns_9h9s7hksqskc5h_power_index_2634() {
            let made_hand: MadeHand = card_array!["9h", "9s", "7h", "Ks", "Qs", "Kc", "5h"].into();

            assert_eq!(made_hand.power_index(), 2634);
        }

        #[test]
        fn it_returns_qdah3c5s3s8sjs_power_index_5756() {
            let made_hand: MadeHand = card_array!["Qd", "Ah", "3c", "5s", "3s", "8s", "Js"].into();

            assert_eq!(made_hand.power_index(), 5756);
        }

        #[test]
        fn it_returns_as2d4h9cjh6c5h_power_index_6509() {
            let made_hand: MadeHand = card_array!["As", "2d", "4h", "9c", "Jh", "6c", "5h"].into();

            assert_eq!(made_hand.power_index(), 6509);
        }

        #[test]
        fn it_returns_5skcks7d8d5hjc_power_index_2679() {
            let made_hand: MadeHand = card_array!["5s", "Kc", "Ks", "7d", "8d", "5h", "Jc"].into();

            assert_eq!(made_hand.power_index(), 2679);
        }

        #[test]
        fn it_returns_4s6c7d2sqc6h9h_power_index_5202() {
            let made_hand: MadeHand = card_array!["4s", "6c", "7d", "2s", "Qc", "6h", "9h"].into();

            assert_eq!(made_hand.power_index(), 5202);
        }

        #[test]
        fn it_returns_ks8h6h2d5dad5h_power_index_5310() {
            let made_hand: MadeHand = card_array!["Ks", "8h", "6h", "2d", "5d", "Ad", "5h"].into();

            assert_eq!(made_hand.power_index(), 5310);
        }

        #[test]
        fn it_returns_5d6s7s5h8hqhjc_power_index_5408() {
            let made_hand: MadeHand = card_array!["5d", "6s", "7s", "5h", "8h", "Qh", "Jc"].into();

            assert_eq!(made_hand.power_index(), 5408);
        }

        #[test]
        fn it_returns_3hqdjd9h6h5c6d_power_index_5187() {
            let made_hand: MadeHand = card_array!["3h", "Qd", "Jd", "9h", "6h", "5c", "6d"].into();

            assert_eq!(made_hand.power_index(), 5187);
        }

        #[test]
        fn it_returns_9das4d4s5htc9s_power_index_3062() {
            let made_hand: MadeHand = card_array!["9d", "As", "4d", "4s", "5h", "Tc", "9s"].into();

            assert_eq!(made_hand.power_index(), 3062);
        }

        #[test]
        fn it_returns_ts7s8c2dah4c9c_power_index_6554() {
            let made_hand: MadeHand = card_array!["Ts", "7s", "8c", "2d", "Ah", "4c", "9c"].into();

            assert_eq!(made_hand.power_index(), 6554);
        }

        #[test]
        fn it_returns_7htc6hjdqh3d4h_power_index_7020() {
            let made_hand: MadeHand = card_array!["7h", "Tc", "6h", "Jd", "Qh", "3d", "4h"].into();

            assert_eq!(made_hand.power_index(), 7020);
        }

        #[test]
        fn it_returns_9d4std2hqsks8d_power_index_6714() {
            let made_hand: MadeHand = card_array!["9d", "4s", "Td", "2h", "Qs", "Ks", "8d"].into();

            assert_eq!(made_hand.power_index(), 6714);
        }

        #[test]
        fn it_returns_6s2s8sks5c6h2d_power_index_3250() {
            let made_hand: MadeHand = card_array!["6s", "2s", "8s", "Ks", "5c", "6h", "2d"].into();

            assert_eq!(made_hand.power_index(), 3250);
        }

        #[test]
        fn it_returns_9h8d8cks9d6h4c_power_index_3019() {
            let made_hand: MadeHand = card_array!["9h", "8d", "8c", "Ks", "9d", "6h", "4c"].into();

            assert_eq!(made_hand.power_index(), 3019);
        }

        #[test]
        fn it_returns_js3s6h9hqd4sqs_power_index_3876() {
            let made_hand: MadeHand = card_array!["Js", "3s", "6h", "9h", "Qd", "4s", "Qs"].into();

            assert_eq!(made_hand.power_index(), 3876);
        }

        #[test]
        fn it_returns_2hqh4dtd2c3d7c_power_index_6076() {
            let made_hand: MadeHand = card_array!["2h", "Qh", "4d", "Td", "2c", "3d", "7c"].into();

            assert_eq!(made_hand.power_index(), 6076);
        }

        #[test]
        fn it_returns_qh7sac4s6dqd9s_power_index_3794() {
            let made_hand: MadeHand = card_array!["Qh", "7s", "Ac", "4s", "6d", "Qd", "9s"].into();

            assert_eq!(made_hand.power_index(), 3794);
        }

        #[test]
        fn it_returns_qs6s6c7sah6hqd_power_index_265() {
            let made_hand: MadeHand = card_array!["Qs", "6s", "6c", "7s", "Ah", "6h", "Qd"].into();

            assert_eq!(made_hand.power_index(), 265);
        }

        #[test]
        fn it_returns_9sjhqd6c7s3had_power_index_6359() {
            let made_hand: MadeHand = card_array!["9s", "Jh", "Qd", "6c", "7s", "3h", "Ad"].into();

            assert_eq!(made_hand.power_index(), 6359);
        }

        #[test]
        fn it_returns_ahqs8dkh3c4cjd_power_index_6187() {
            let made_hand: MadeHand = card_array!["Ah", "Qs", "8d", "Kh", "3c", "4c", "Jd"].into();

            assert_eq!(made_hand.power_index(), 6187);
        }

        #[test]
        fn it_returns_2d6d9d2hth3s7d_power_index_6131() {
            let made_hand: MadeHand = card_array!["2d", "6d", "9d", "2h", "Th", "3s", "7d"].into();

            assert_eq!(made_hand.power_index(), 6131);
        }

        #[test]
        fn it_returns_8dqcas5cjd6sjh_power_index_3998() {
            let made_hand: MadeHand = card_array!["8d", "Qc", "As", "5c", "Jd", "6s", "Jh"].into();

            assert_eq!(made_hand.power_index(), 3998);
        }

        #[test]
        fn it_returns_4skd2cjc4c2dqd_power_index_3305() {
            let made_hand: MadeHand = card_array!["4s", "Kd", "2c", "Jc", "4c", "2d", "Qd"].into();

            assert_eq!(made_hand.power_index(), 3305);
        }

        #[test]
        fn it_returns_6std7h7c8hkd4d_power_index_4939() {
            let made_hand: MadeHand = card_array!["6s", "Td", "7h", "7c", "8h", "Kd", "4d"].into();

            assert_eq!(made_hand.power_index(), 4939);
        }

        #[test]
        fn it_returns_3h6sjhadqd2skd_power_index_6189() {
            let made_hand: MadeHand = card_array!["3h", "6s", "Jh", "Ad", "Qd", "2s", "Kd"].into();

            assert_eq!(made_hand.power_index(), 6189);
        }

        #[test]
        fn it_returns_8d3c2h6hthks5s_power_index_6908() {
            let made_hand: MadeHand = card_array!["8d", "3c", "2h", "6h", "Th", "Ks", "5s"].into();

            assert_eq!(made_hand.power_index(), 6908);
        }

        #[test]
        fn it_returns_ah4sjh5h6d9dtc_power_index_6472() {
            let made_hand: MadeHand = card_array!["Ah", "4s", "Jh", "5h", "6d", "9d", "Tc"].into();

            assert_eq!(made_hand.power_index(), 6472);
        }

        #[test]
        fn it_returns_6c7h5d6s2sah8c_power_index_5126() {
            let made_hand: MadeHand = card_array!["6c", "7h", "5d", "6s", "2s", "Ah", "8c"].into();

            assert_eq!(made_hand.power_index(), 5126);
        }

        #[test]
        fn it_returns_jckh7h4d4s4c4h_power_index_132() {
            let made_hand: MadeHand = card_array!["Jc", "Kh", "7h", "4d", "4s", "4c", "4h"].into();

            assert_eq!(made_hand.power_index(), 132);
        }

        #[test]
        fn it_returns_tckc2c8d5h9hth_power_index_4278() {
            let made_hand: MadeHand = card_array!["Tc", "Kc", "2c", "8d", "5h", "9h", "Th"].into();

            assert_eq!(made_hand.power_index(), 4278);
        }

        #[test]
        fn it_returns_8cjh7h5skcadqd_power_index_6187() {
            let made_hand: MadeHand = card_array!["8c", "Jh", "7h", "5s", "Kc", "Ad", "Qd"].into();

            assert_eq!(made_hand.power_index(), 6187);
        }

        #[test]
        fn it_returns_8s4h9sqc3dac6c_power_index_6415() {
            let made_hand: MadeHand = card_array!["8s", "4h", "9s", "Qc", "3d", "Ac", "6c"].into();

            assert_eq!(made_hand.power_index(), 6415);
        }

        #[test]
        fn it_returns_js3c9c4c2h9sas_power_index_4450() {
            let made_hand: MadeHand = card_array!["Js", "3c", "9c", "4c", "2h", "9s", "As"].into();

            assert_eq!(made_hand.power_index(), 4450);
        }

        #[test]
        fn it_returns_7dqsad9h7s4c3d_power_index_4878() {
            let made_hand: MadeHand = card_array!["7d", "Qs", "Ad", "9h", "7s", "4c", "3d"].into();

            assert_eq!(made_hand.power_index(), 4878);
        }

        #[test]
        fn it_returns_kdtcqhah9d6h5c_power_index_6194() {
            let made_hand: MadeHand = card_array!["Kd", "Tc", "Qh", "Ah", "9d", "6h", "5c"].into();

            assert_eq!(made_hand.power_index(), 6194);
        }

        #[test]
        fn it_returns_khas2hqs2sjs3h_power_index_5966() {
            let made_hand: MadeHand = card_array!["Kh", "As", "2h", "Qs", "2s", "Js", "3h"].into();

            assert_eq!(made_hand.power_index(), 5966);
        }

        #[test]
        fn it_returns_9d3d3sqs8hjh2s_power_index_5847() {
            let made_hand: MadeHand = card_array!["9d", "3d", "3s", "Qs", "8h", "Jh", "2s"].into();

            assert_eq!(made_hand.power_index(), 5847);
        }

        #[test]
        fn it_returns_jc4h2c7h5h2s3h_power_index_6121() {
            let made_hand: MadeHand = card_array!["Jc", "4h", "2c", "7h", "5h", "2s", "3h"].into();

            assert_eq!(made_hand.power_index(), 6121);
        }

        #[test]
        fn it_returns_6s3h3sjh7hth9c_power_index_5882() {
            let made_hand: MadeHand = card_array!["6s", "3h", "3s", "Jh", "7h", "Th", "9c"].into();

            assert_eq!(made_hand.power_index(), 5882);
        }

        #[test]
        fn it_returns_6h9dahth9cactc_power_index_2504() {
            let made_hand: MadeHand = card_array!["6h", "9d", "Ah", "Th", "9c", "Ac", "Tc"].into();

            assert_eq!(made_hand.power_index(), 2504);
        }

        #[test]
        fn it_returns_3c8h7d6sjcqh2d_power_index_7056() {
            let made_hand: MadeHand = card_array!["3c", "8h", "7d", "6s", "Jc", "Qh", "2d"].into();

            assert_eq!(made_hand.power_index(), 7056);
        }

        #[test]
        fn it_returns_5d8c5h2sts9dkh_power_index_5378() {
            let made_hand: MadeHand = card_array!["5d", "8c", "5h", "2s", "Ts", "9d", "Kh"].into();

            assert_eq!(made_hand.power_index(), 5378);
        }

        #[test]
        fn it_returns_8ckd2s2cqcjc3s_power_index_6021() {
            let made_hand: MadeHand = card_array!["8c", "Kd", "2s", "2c", "Qc", "Jc", "3s"].into();

            assert_eq!(made_hand.power_index(), 6021);
        }

        #[test]
        fn it_returns_9das5s4djd3c7s_power_index_6505() {
            let made_hand: MadeHand = card_array!["9d", "As", "5s", "4d", "Jd", "3c", "7s"].into();

            assert_eq!(made_hand.power_index(), 6505);
        }

        #[test]
        fn it_returns_2c8hahkd4s8s3d_power_index_4653() {
            let made_hand: MadeHand = card_array!["2c", "8h", "Ah", "Kd", "4s", "8s", "3d"].into();

            assert_eq!(made_hand.power_index(), 4653);
        }

        #[test]
        fn it_returns_qd6sas6d5s7h8h_power_index_5099() {
            let made_hand: MadeHand = card_array!["Qd", "6s", "As", "6d", "5s", "7h", "8h"].into();

            assert_eq!(made_hand.power_index(), 5099);
        }

        #[test]
        fn it_returns_7h4ctcqd5cks8s_power_index_6721() {
            let made_hand: MadeHand = card_array!["7h", "4c", "Tc", "Qd", "5c", "Ks", "8s"].into();

            assert_eq!(made_hand.power_index(), 6721);
        }

        #[test]
        fn it_returns_ks9d2dkcah6d2c_power_index_2710() {
            let made_hand: MadeHand = card_array!["Ks", "9d", "2d", "Kc", "Ah", "6d", "2c"].into();

            assert_eq!(made_hand.power_index(), 2710);
        }

        #[test]
        fn it_returns_9stc3hkcad2h8c_power_index_6266() {
            let made_hand: MadeHand = card_array!["9s", "Tc", "3h", "Kc", "Ad", "2h", "8c"].into();

            assert_eq!(made_hand.power_index(), 6266);
        }

        #[test]
        fn it_returns_8hqctc2h5c2d3s_power_index_6075() {
            let made_hand: MadeHand = card_array!["8h", "Qc", "Tc", "2h", "5c", "2d", "3s"].into();

            assert_eq!(made_hand.power_index(), 6075);
        }

        #[test]
        fn it_returns_7d4h9dkc2sas5s_power_index_6301() {
            let made_hand: MadeHand = card_array!["7d", "4h", "9d", "Kc", "2s", "As", "5s"].into();

            assert_eq!(made_hand.power_index(), 6301);
        }

        #[test]
        fn it_returns_ts2hkd5d2ckctd_power_index_2629() {
            let made_hand: MadeHand = card_array!["Ts", "2h", "Kd", "5d", "2c", "Kc", "Td"].into();

            assert_eq!(made_hand.power_index(), 2629);
        }

        #[test]
        fn it_returns_4s6s9hkhadkc3s_power_index_3575() {
            let made_hand: MadeHand = card_array!["4s", "6s", "9h", "Kh", "Ad", "Kc", "3s"].into();

            assert_eq!(made_hand.power_index(), 3575);
        }

        #[test]
        fn it_returns_js8sad3c3dacth_power_index_2580() {
            let made_hand: MadeHand = card_array!["Js", "8s", "Ad", "3c", "3d", "Ac", "Th"].into();

            assert_eq!(made_hand.power_index(), 2580);
        }

        #[test]
        fn it_returns_9d8c4h5d6h9hah_power_index_4461() {
            let made_hand: MadeHand = card_array!["9d", "8c", "4h", "5d", "6h", "9h", "Ah"].into();

            assert_eq!(made_hand.power_index(), 4461);
        }

        #[test]
        fn it_returns_tc2s5djs4cjcas_power_index_4009() {
            let made_hand: MadeHand = card_array!["Tc", "2s", "5d", "Js", "4c", "Jc", "As"].into();

            assert_eq!(made_hand.power_index(), 4009);
        }

        #[test]
        fn it_returns_6d6ckstd9dah5h_power_index_5088() {
            let made_hand: MadeHand = card_array!["6d", "6c", "Ks", "Td", "9d", "Ah", "5h"].into();

            assert_eq!(made_hand.power_index(), 5088);
        }

        #[test]
        fn it_returns_3hqc5d8dqh6ckd_power_index_3846() {
            let made_hand: MadeHand = card_array!["3h", "Qc", "5d", "8d", "Qh", "6c", "Kd"].into();

            assert_eq!(made_hand.power_index(), 3846);
        }

        #[test]
        fn it_returns_jstcth4cad3c8s_power_index_4226() {
            let made_hand: MadeHand = card_array!["Js", "Tc", "Th", "4c", "Ad", "3c", "8s"].into();

            assert_eq!(made_hand.power_index(), 4226);
        }

        #[test]
        fn it_returns_6hah2stc2djh6c_power_index_3249() {
            let made_hand: MadeHand = card_array!["6h", "Ah", "2s", "Tc", "2d", "Jh", "6c"].into();

            assert_eq!(made_hand.power_index(), 3249);
        }

        #[test]
        fn it_returns_7d3c7hqhas5hjs_power_index_4876() {
            let made_hand: MadeHand = card_array!["7d", "3c", "7h", "Qh", "As", "5h", "Js"].into();

            assert_eq!(made_hand.power_index(), 4876);
        }

        #[test]
        fn it_returns_4s8s6c4c8hjh9h_power_index_3131() {
            let made_hand: MadeHand = card_array!["4s", "8s", "6c", "4c", "8h", "Jh", "9h"].into();

            assert_eq!(made_hand.power_index(), 3131);
        }

        #[test]
        fn it_returns_kh6c7h8htcts4s_power_index_4285() {
            let made_hand: MadeHand = card_array!["Kh", "6c", "7h", "8h", "Tc", "Ts", "4s"].into();

            assert_eq!(made_hand.power_index(), 4285);
        }

        #[test]
        fn it_returns_8s9h3h3c7dks9d_power_index_3074() {
            let made_hand: MadeHand = card_array!["8s", "9h", "3h", "3c", "7d", "Ks", "9d"].into();

            assert_eq!(made_hand.power_index(), 3074);
        }

        #[test]
        fn it_returns_8sac5sqc6dah6h_power_index_2546() {
            let made_hand: MadeHand = card_array!["8s", "Ac", "5s", "Qc", "6d", "Ah", "6h"].into();

            assert_eq!(made_hand.power_index(), 2546);
        }

        #[test]
        fn it_returns_qsad7d4ctd6h9d_power_index_6387() {
            let made_hand: MadeHand = card_array!["Qs", "Ad", "7d", "4c", "Td", "6h", "9d"].into();

            assert_eq!(made_hand.power_index(), 6387);
        }

        #[test]
        fn it_returns_js7hac9d5h2hkd_power_index_6239() {
            let made_hand: MadeHand = card_array!["Js", "7h", "Ac", "9d", "5h", "2h", "Kd"].into();

            assert_eq!(made_hand.power_index(), 6239);
        }

        #[test]
        fn it_returns_3c3d5cjd7s8hqd_power_index_5848() {
            let made_hand: MadeHand = card_array!["3c", "3d", "5c", "Jd", "7s", "8h", "Qd"].into();

            assert_eq!(made_hand.power_index(), 5848);
        }

        #[test]
        fn it_returns_2dacjckh3cqcah_power_index_3326() {
            let made_hand: MadeHand = card_array!["2d", "Ac", "Jc", "Kh", "3c", "Qc", "Ah"].into();

            assert_eq!(made_hand.power_index(), 3326);
        }

        #[test]
        fn it_returns_jh4s6dtd8c3sqh_power_index_7015() {
            let made_hand: MadeHand = card_array!["Jh", "4s", "6d", "Td", "8c", "3s", "Qh"].into();

            assert_eq!(made_hand.power_index(), 7015);
        }

        #[test]
        fn it_returns_2d6had3hks4c8d_power_index_6321() {
            let made_hand: MadeHand = card_array!["2d", "6h", "Ad", "3h", "Ks", "4c", "8d"].into();

            assert_eq!(made_hand.power_index(), 6321);
        }

        #[test]
        fn it_returns_jc8sac8hth4sjs_power_index_2853() {
            let made_hand: MadeHand = card_array!["Jc", "8s", "Ac", "8h", "Th", "4s", "Js"].into();

            assert_eq!(made_hand.power_index(), 2853);
        }

        #[test]
        fn it_returns_qd8h4h9s6c8cac_power_index_4658() {
            let made_hand: MadeHand = card_array!["Qd", "8h", "4h", "9s", "6c", "8c", "Ac"].into();

            assert_eq!(made_hand.power_index(), 4658);
        }

        #[test]
        fn it_returns_qcah5hadac2s7c_power_index_1625() {
            let made_hand: MadeHand = card_array!["Qc", "Ah", "5h", "Ad", "Ac", "2s", "7c"].into();

            assert_eq!(made_hand.power_index(), 1625);
        }

        #[test]
        fn it_returns_7h9h4dtd5c6c7d_power_index_5031() {
            let made_hand: MadeHand = card_array!["7h", "9h", "4d", "Td", "5c", "6c", "7d"].into();

            assert_eq!(made_hand.power_index(), 5031);
        }

        #[test]
        fn it_returns_9h6c2hjdts8d5h_power_index_7217() {
            let made_hand: MadeHand = card_array!["9h", "6c", "2h", "Jd", "Ts", "8d", "5h"].into();

            assert_eq!(made_hand.power_index(), 7217);
        }

        #[test]
        fn it_returns_7h3h9d9sjh3d5h_power_index_3076() {
            let made_hand: MadeHand = card_array!["7h", "3h", "9d", "9s", "Jh", "3d", "5h"].into();

            assert_eq!(made_hand.power_index(), 3076);
        }

        #[test]
        fn it_returns_acjh2s7d7s6c2h_power_index_3205() {
            let made_hand: MadeHand = card_array!["Ac", "Jh", "2s", "7d", "7s", "6c", "2h"].into();

            assert_eq!(made_hand.power_index(), 3205);
        }

        #[test]
        fn it_returns_5c2skstd6s9cts_power_index_4280() {
            let made_hand: MadeHand = card_array!["5c", "2s", "Ks", "Td", "6s", "9c", "Ts"].into();

            assert_eq!(made_hand.power_index(), 4280);
        }

        #[test]
        fn it_returns_6s2c8s2sts5h3s_power_index_1528() {
            let made_hand: MadeHand = card_array!["6s", "2c", "8s", "2s", "Ts", "5h", "3s"].into();

            assert_eq!(made_hand.power_index(), 1528);
        }

        #[test]
        fn it_returns_9s5s3s7h4dtstd_power_index_4377() {
            let made_hand: MadeHand = card_array!["9s", "5s", "3s", "7h", "4d", "Ts", "Td"].into();

            assert_eq!(made_hand.power_index(), 4377);
        }

        #[test]
        fn it_returns_qcqd2s2cas9hts_power_index_2820() {
            let made_hand: MadeHand = card_array!["Qc", "Qd", "2s", "2c", "As", "9h", "Ts"].into();

            assert_eq!(made_hand.power_index(), 2820);
        }

        #[test]
        fn it_returns_9hkd4skc8dtc7s_power_index_3682() {
            let made_hand: MadeHand = card_array!["9h", "Kd", "4s", "Kc", "8d", "Tc", "7s"].into();

            assert_eq!(made_hand.power_index(), 3682);
        }

        #[test]
        fn it_returns_9c6dkd3d7h9d5d_power_index_1101() {
            let made_hand: MadeHand = card_array!["9c", "6d", "Kd", "3d", "7h", "9d", "5d"].into();

            assert_eq!(made_hand.power_index(), 1101);
        }

        #[test]
        fn it_returns_tdkh3h4c8h3sks_power_index_2702() {
            let made_hand: MadeHand = card_array!["Td", "Kh", "3h", "4c", "8h", "3s", "Ks"].into();

            assert_eq!(made_hand.power_index(), 2702);
        }

        #[test]
        fn it_returns_4s9dtc2saskd7h_power_index_6267() {
            let made_hand: MadeHand = card_array!["4s", "9d", "Tc", "2s", "As", "Kd", "7h"].into();

            assert_eq!(made_hand.power_index(), 6267);
        }

        #[test]
        fn it_returns_kcad7hth4hqs8h_power_index_6195() {
            let made_hand: MadeHand = card_array!["Kc", "Ad", "7h", "Th", "4h", "Qs", "8h"].into();

            assert_eq!(made_hand.power_index(), 6195);
        }

        #[test]
        fn it_returns_qs7sjhqdahtd9c_power_index_3776() {
            let made_hand: MadeHand = card_array!["Qs", "7s", "Jh", "Qd", "Ah", "Td", "9c"].into();

            assert_eq!(made_hand.power_index(), 3776);
        }

        #[test]
        fn it_returns_th3s9h2ctd5c6c_power_index_4381() {
            let made_hand: MadeHand = card_array!["Th", "3s", "9h", "2c", "Td", "5c", "6c"].into();

            assert_eq!(made_hand.power_index(), 4381);
        }

        #[test]
        fn it_returns_6hqd8hkc8sadqs_power_index_2754() {
            let made_hand: MadeHand = card_array!["6h", "Qd", "8h", "Kc", "8s", "Ad", "Qs"].into();

            assert_eq!(made_hand.power_index(), 2754);
        }

        #[test]
        fn it_returns_js9h8c7ckhqh9d_power_index_4481() {
            let made_hand: MadeHand = card_array!["Js", "9h", "8c", "7c", "Kh", "Qh", "9d"].into();

            assert_eq!(made_hand.power_index(), 4481);
        }

        #[test]
        fn it_returns_6h2stdad6dks2c_power_index_3249() {
            let made_hand: MadeHand = card_array!["6h", "2s", "Td", "Ad", "6d", "Ks", "2c"].into();

            assert_eq!(made_hand.power_index(), 3249);
        }

        #[test]
        fn it_returns_ahqh5cjc7d7s6s_power_index_4876() {
            let made_hand: MadeHand = card_array!["Ah", "Qh", "5c", "Jc", "7d", "7s", "6s"].into();

            assert_eq!(made_hand.power_index(), 4876);
        }

        #[test]
        fn it_returns_kdtdts2cad4c6h_power_index_4211() {
            let made_hand: MadeHand = card_array!["Kd", "Td", "Ts", "2c", "Ad", "4c", "6h"].into();

            assert_eq!(made_hand.power_index(), 4211);
        }

        #[test]
        fn it_returns_kh3h7s8cjh8dah_power_index_4647() {
            let made_hand: MadeHand = card_array!["Kh", "3h", "7s", "8c", "Jh", "8d", "Ah"].into();

            assert_eq!(made_hand.power_index(), 4647);
        }

        #[test]
        fn it_returns_8s3cqs9d8d6hqh_power_index_2758() {
            let made_hand: MadeHand = card_array!["8s", "3c", "Qs", "9d", "8d", "6h", "Qh"].into();

            assert_eq!(made_hand.power_index(), 2758);
        }

        #[test]
        fn it_returns_7cad3c9h6dqc8h_power_index_6414() {
            let made_hand: MadeHand = card_array!["7c", "Ad", "3c", "9h", "6d", "Qc", "8h"].into();

            assert_eq!(made_hand.power_index(), 6414);
        }

        #[test]
        fn it_returns_5s6c4c7s5h9s3d_power_index_1607() {
            let made_hand: MadeHand = card_array!["5s", "6c", "4c", "7s", "5h", "9s", "3d"].into();

            assert_eq!(made_hand.power_index(), 1607);
        }

        #[test]
        fn it_returns_3hjdah3s8hth4d_power_index_5765() {
            let made_hand: MadeHand = card_array!["3h", "Jd", "Ah", "3s", "8h", "Th", "4d"].into();

            assert_eq!(made_hand.power_index(), 5765);
        }

        #[test]
        fn it_returns_3sjdts4h5h2h8c_power_index_7246() {
            let made_hand: MadeHand = card_array!["3s", "Jd", "Ts", "4h", "5h", "2h", "8c"].into();

            assert_eq!(made_hand.power_index(), 7246);
        }

        #[test]
        fn it_returns_td5d3hjsqh4stc_power_index_4310() {
            let made_hand: MadeHand = card_array!["Td", "5d", "3h", "Js", "Qh", "4s", "Tc"].into();

            assert_eq!(made_hand.power_index(), 4310);
        }

        #[test]
        fn it_returns_js8d2cad3sjd7s_power_index_4020() {
            let made_hand: MadeHand = card_array!["Js", "8d", "2c", "Ad", "3s", "Jd", "7s"].into();

            assert_eq!(made_hand.power_index(), 4020);
        }

        #[test]
        fn it_returns_8hjc3h3ctd2sac_power_index_5765() {
            let made_hand: MadeHand = card_array!["8h", "Jc", "3h", "3c", "Td", "2s", "Ac"].into();

            assert_eq!(made_hand.power_index(), 5765);
        }

        #[test]
        fn it_returns_qdahjckdtc2hjs_power_index_1600() {
            let made_hand: MadeHand = card_array!["Qd", "Ah", "Jc", "Kd", "Tc", "2h", "Js"].into();

            assert_eq!(made_hand.power_index(), 1600);
        }

        #[test]
        fn it_returns_qs5h2dqd8hkdkh_power_index_2604() {
            let made_hand: MadeHand = card_array!["Qs", "5h", "2d", "Qd", "8h", "Kd", "Kh"].into();

            assert_eq!(made_hand.power_index(), 2604);
        }

        #[test]
        fn it_returns_qhqdjc9c2d8hjh_power_index_2724() {
            let made_hand: MadeHand = card_array!["Qh", "Qd", "Jc", "9c", "2d", "8h", "Jh"].into();

            assert_eq!(made_hand.power_index(), 2724);
        }

        #[test]
        fn it_returns_9c4djc2d2h7d9d_power_index_3087() {
            let made_hand: MadeHand = card_array!["9c", "4d", "Jc", "2d", "2h", "7d", "9d"].into();

            assert_eq!(made_hand.power_index(), 3087);
        }

        #[test]
        fn it_returns_ac8ctcthjs8h7s_power_index_2941() {
            let made_hand: MadeHand = card_array!["Ac", "8c", "Tc", "Th", "Js", "8h", "7s"].into();

            assert_eq!(made_hand.power_index(), 2941);
        }

        #[test]
        fn it_returns_9d5c8d4hkstdjd_power_index_6798() {
            let made_hand: MadeHand = card_array!["9d", "5c", "8d", "4h", "Ks", "Td", "Jd"].into();

            assert_eq!(made_hand.power_index(), 6798);
        }

        #[test]
        fn it_returns_2hks4ctd6cjskd_power_index_3649() {
            let made_hand: MadeHand = card_array!["2h", "Ks", "4c", "Td", "6c", "Js", "Kd"].into();

            assert_eq!(made_hand.power_index(), 3649);
        }

        #[test]
        fn it_returns_6hqc8s9c9d4sjh_power_index_4527() {
            let made_hand: MadeHand = card_array!["6h", "Qc", "8s", "9c", "9d", "4s", "Jh"].into();

            assert_eq!(made_hand.power_index(), 4527);
        }

        #[test]
        fn it_returns_9ckcjdactd7s6s_power_index_6230() {
            let made_hand: MadeHand = card_array!["9c", "Kc", "Jd", "Ac", "Td", "7s", "6s"].into();

            assert_eq!(made_hand.power_index(), 6230);
        }

        #[test]
        fn it_returns_khtcts2d6htd2c_power_index_226() {
            let made_hand: MadeHand = card_array!["Kh", "Tc", "Ts", "2d", "6h", "Td", "2c"].into();

            assert_eq!(made_hand.power_index(), 226);
        }

        #[test]
        fn it_returns_qh2c3d5c8sqd9s_power_index_3932() {
            let made_hand: MadeHand = card_array!["Qh", "2c", "3d", "5c", "8s", "Qd", "9s"].into();

            assert_eq!(made_hand.power_index(), 3932);
        }

        #[test]
        fn it_returns_6ckd2d4hac8hjh_power_index_6246() {
            let made_hand: MadeHand = card_array!["6c", "Kd", "2d", "4h", "Ac", "8h", "Jh"].into();

            assert_eq!(made_hand.power_index(), 6246);
        }

        #[test]
        fn it_returns_7c2s6sjdas6h2d_power_index_3249() {
            let made_hand: MadeHand = card_array!["7c", "2s", "6s", "Jd", "As", "6h", "2d"].into();

            assert_eq!(made_hand.power_index(), 3249);
        }

        #[test]
        fn it_returns_ad6c5cqc4c5hth_power_index_5317() {
            let made_hand: MadeHand = card_array!["Ad", "6c", "5c", "Qc", "4c", "5h", "Th"].into();

            assert_eq!(made_hand.power_index(), 5317);
        }

        #[test]
        fn it_returns_8djc2d3s9hjd8s_power_index_2857() {
            let made_hand: MadeHand = card_array!["8d", "Jc", "2d", "3s", "9h", "Jd", "8s"].into();

            assert_eq!(made_hand.power_index(), 2857);
        }

        #[test]
        fn it_returns_jdqc9c3s7d2h5h_power_index_7042() {
            let made_hand: MadeHand = card_array!["Jd", "Qc", "9c", "3s", "7d", "2h", "5h"].into();

            assert_eq!(made_hand.power_index(), 7042);
        }

        #[test]
        fn it_returns_7djhkdasjs7s5d_power_index_2864() {
            let made_hand: MadeHand = card_array!["7d", "Jh", "Kd", "As", "Js", "7s", "5d"].into();

            assert_eq!(made_hand.power_index(), 2864);
        }

        #[test]
        fn it_returns_7c5s4ckc7sts8d_power_index_4939() {
            let made_hand: MadeHand = card_array!["7c", "5s", "4c", "Kc", "7s", "Ts", "8d"].into();

            assert_eq!(made_hand.power_index(), 4939);
        }

        #[test]
        fn it_returns_5h6d4s4h2d2sqd_power_index_3306() {
            let made_hand: MadeHand = card_array!["5h", "6d", "4s", "4h", "2d", "2s", "Qd"].into();

            assert_eq!(made_hand.power_index(), 3306);
        }

        #[test]
        fn it_returns_kh8dkstc3dqs7d_power_index_3611() {
            let made_hand: MadeHand = card_array!["Kh", "8d", "Ks", "Tc", "3d", "Qs", "7d"].into();

            assert_eq!(made_hand.power_index(), 3611);
        }

        #[test]
        fn it_returns_9sjdkh4c7c5cjc_power_index_4059() {
            let made_hand: MadeHand = card_array!["9s", "Jd", "Kh", "4c", "7c", "5c", "Jc"].into();

            assert_eq!(made_hand.power_index(), 4059);
        }

        #[test]
        fn it_returns_qd6sjs8dkd3c3h_power_index_5801() {
            let made_hand: MadeHand = card_array!["Qd", "6s", "Js", "8d", "Kd", "3c", "3h"].into();

            assert_eq!(made_hand.power_index(), 5801);
        }

        #[test]
        fn it_returns_ad2d2cjcac7d3h_power_index_2591() {
            let made_hand: MadeHand = card_array!["Ad", "2d", "2c", "Jc", "Ac", "7d", "3h"].into();

            assert_eq!(made_hand.power_index(), 2591);
        }

        #[test]
        fn it_returns_5s8dqh5h2d3d4h_power_index_5429() {
            let made_hand: MadeHand = card_array!["5s", "8d", "Qh", "5h", "2d", "3d", "4h"].into();

            assert_eq!(made_hand.power_index(), 5429);
        }

        #[test]
        fn it_returns_ksqc7s5c6h4skd_power_index_3631() {
            let made_hand: MadeHand = card_array!["Ks", "Qc", "7s", "5c", "6h", "4s", "Kd"].into();

            assert_eq!(made_hand.power_index(), 3631);
        }

        #[test]
        fn it_returns_3h7s9std3ctc5c_power_index_3000() {
            let made_hand: MadeHand = card_array!["3h", "7s", "9s", "Td", "3c", "Tc", "5c"].into();

            assert_eq!(made_hand.power_index(), 3000);
        }

        #[test]
        fn it_returns_5hkhjdahks6h2s_power_index_3560() {
            let made_hand: MadeHand = card_array!["5h", "Kh", "Jd", "Ah", "Ks", "6h", "2s"].into();

            assert_eq!(made_hand.power_index(), 3560);
        }

        #[test]
        fn it_returns_4ctd2sjhqc3s7c_power_index_7022() {
            let made_hand: MadeHand = card_array!["4c", "Td", "2s", "Jh", "Qc", "3s", "7c"].into();

            assert_eq!(made_hand.power_index(), 7022);
        }

        #[test]
        fn it_returns_6s8sthjhjdjs2d_power_index_1839() {
            let made_hand: MadeHand = card_array!["6s", "8s", "Th", "Jh", "Jd", "Js", "2d"].into();

            assert_eq!(made_hand.power_index(), 1839);
        }

        #[test]
        fn it_returns_9hqdjd6dth5c2c_power_index_7009() {
            let made_hand: MadeHand = card_array!["9h", "Qd", "Jd", "6d", "Th", "5c", "2c"].into();

            assert_eq!(made_hand.power_index(), 7009);
        }

        #[test]
        fn it_returns_4dthjs9s9c6d7s_power_index_4563() {
            let made_hand: MadeHand = card_array!["4d", "Th", "Js", "9s", "9c", "6d", "7s"].into();

            assert_eq!(made_hand.power_index(), 4563);
        }

        #[test]
        fn it_returns_6s8h8d5skdqskc_power_index_2645() {
            let made_hand: MadeHand = card_array!["6s", "8h", "8d", "5s", "Kd", "Qs", "Kc"].into();

            assert_eq!(made_hand.power_index(), 2645);
        }

        #[test]
        fn it_returns_2stskdth7c4ckh_power_index_2627() {
            let made_hand: MadeHand = card_array!["2s", "Ts", "Kd", "Th", "7c", "4c", "Kh"].into();

            assert_eq!(made_hand.power_index(), 2627);
        }

        #[test]
        fn it_returns_askdqc5h9c9htd_power_index_4426() {
            let made_hand: MadeHand = card_array!["As", "Kd", "Qc", "5h", "9c", "9h", "Td"].into();

            assert_eq!(made_hand.power_index(), 4426);
        }

        #[test]
        fn it_returns_ad8h5d4cjhthks_power_index_6231() {
            let made_hand: MadeHand = card_array!["Ad", "8h", "5d", "4c", "Jh", "Th", "Ks"].into();

            assert_eq!(made_hand.power_index(), 6231);
        }

        #[test]
        fn it_returns_8ctdjd4s5d4d7h_power_index_5663() {
            let made_hand: MadeHand = card_array!["8c", "Td", "Jd", "4s", "5d", "4d", "7h"].into();

            assert_eq!(made_hand.power_index(), 5663);
        }

        #[test]
        fn it_returns_5s8sqc8dtd4h7d_power_index_4755() {
            let made_hand: MadeHand = card_array!["5s", "8s", "Qc", "8d", "Td", "4h", "7d"].into();

            assert_eq!(made_hand.power_index(), 4755);
        }

        #[test]
        fn it_returns_2dad4c2hqckc7h_power_index_5966() {
            let made_hand: MadeHand = card_array!["2d", "Ad", "4c", "2h", "Qc", "Kc", "7h"].into();

            assert_eq!(made_hand.power_index(), 5966);
        }

        #[test]
        fn it_returns_7sks3dkc5d7d6h_power_index_2661() {
            let made_hand: MadeHand = card_array!["7s", "Ks", "3d", "Kc", "5d", "7d", "6h"].into();

            assert_eq!(made_hand.power_index(), 2661);
        }

        #[test]
        fn it_returns_7s5d6h9djc5s6d_power_index_3219() {
            let made_hand: MadeHand = card_array!["7s", "5d", "6h", "9d", "Jc", "5s", "6d"].into();

            assert_eq!(made_hand.power_index(), 3219);
        }

        #[test]
        fn it_returns_7htc9d2s8h5skc_power_index_6882() {
            let made_hand: MadeHand = card_array!["7h", "Tc", "9d", "2s", "8h", "5s", "Kc"].into();

            assert_eq!(made_hand.power_index(), 6882);
        }

        #[test]
        fn it_returns_kd2dqhks6h4hjs_power_index_3605() {
            let made_hand: MadeHand = card_array!["Kd", "2d", "Qh", "Ks", "6h", "4h", "Js"].into();

            assert_eq!(made_hand.power_index(), 3605);
        }

        #[test]
        fn it_returns_8s2c7c5c4dkc4s_power_index_5611() {
            let made_hand: MadeHand = card_array!["8s", "2c", "7c", "5c", "4d", "Kc", "4s"].into();

            assert_eq!(made_hand.power_index(), 5611);
        }

        #[test]
        fn it_returns_ks7h9sac5hjdad_power_index_3337() {
            let made_hand: MadeHand = card_array!["Ks", "7h", "9s", "Ac", "5h", "Jd", "Ad"].into();

            assert_eq!(made_hand.power_index(), 3337);
        }

        #[test]
        fn it_returns_9cas7sactc4c8d_power_index_3462() {
            let made_hand: MadeHand = card_array!["9c", "As", "7s", "Ac", "Tc", "4c", "8d"].into();

            assert_eq!(made_hand.power_index(), 3462);
        }

        #[test]
        fn it_returns_7s4cthts3djdqh_power_index_4308() {
            let made_hand: MadeHand = card_array!["7s", "4c", "Th", "Ts", "3d", "Jd", "Qh"].into();

            assert_eq!(made_hand.power_index(), 4308);
        }

        #[test]
        fn it_returns_6d5h2hthts3s3c_power_index_3003() {
            let made_hand: MadeHand = card_array!["6d", "5h", "2h", "Th", "Ts", "3s", "3c"].into();

            assert_eq!(made_hand.power_index(), 3003);
        }

        #[test]
        fn it_returns_5stsjhkh9s8hks_power_index_3646() {
            let made_hand: MadeHand = card_array!["5s", "Ts", "Jh", "Kh", "9s", "8h", "Ks"].into();

            assert_eq!(made_hand.power_index(), 3646);
        }

        #[test]
        fn it_returns_6c9ckh2s3c5skc_power_index_3721() {
            let made_hand: MadeHand = card_array!["6c", "9c", "Kh", "2s", "3c", "5s", "Kc"].into();

            assert_eq!(made_hand.power_index(), 3721);
        }

        #[test]
        fn it_returns_9skckh7sqckd8h_power_index_1689() {
            let made_hand: MadeHand = card_array!["9s", "Kc", "Kh", "7s", "Qc", "Kd", "8h"].into();

            assert_eq!(made_hand.power_index(), 1689);
        }

        #[test]
        fn it_returns_3hkc7cac6s5c9d_power_index_6300() {
            let made_hand: MadeHand = card_array!["3h", "Kc", "7c", "Ac", "6s", "5c", "9d"].into();

            assert_eq!(made_hand.power_index(), 6300);
        }

        #[test]
        fn it_returns_3s8c5s4d7h9d7d_power_index_5052() {
            let made_hand: MadeHand = card_array!["3s", "8c", "5s", "4d", "7h", "9d", "7d"].into();

            assert_eq!(made_hand.power_index(), 5052);
        }

        #[test]
        fn it_returns_5had5dtd4d9c6h_power_index_5333() {
            let made_hand: MadeHand = card_array!["5h", "Ad", "5d", "Td", "4d", "9c", "6h"].into();

            assert_eq!(made_hand.power_index(), 5333);
        }

        #[test]
        fn it_returns_ksaskh3djh6c7c_power_index_3559() {
            let made_hand: MadeHand = card_array!["Ks", "As", "Kh", "3d", "Jh", "6c", "7c"].into();

            assert_eq!(made_hand.power_index(), 3559);
        }

        #[test]
        fn it_returns_kh8sqd2h6s6djs_power_index_5141() {
            let made_hand: MadeHand = card_array!["Kh", "8s", "Qd", "2h", "6s", "6d", "Js"].into();

            assert_eq!(made_hand.power_index(), 5141);
        }

        #[test]
        fn it_returns_jd2ckh3skdqh2s_power_index_2711() {
            let made_hand: MadeHand = card_array!["Jd", "2c", "Kh", "3s", "Kd", "Qh", "2s"].into();

            assert_eq!(made_hand.power_index(), 2711);
        }

        #[test]
        fn it_returns_askc9d4c5dkh9s_power_index_2633() {
            let made_hand: MadeHand = card_array!["As", "Kc", "9d", "4c", "5d", "Kh", "9s"].into();

            assert_eq!(made_hand.power_index(), 2633);
        }

        #[test]
        fn it_returns_kdth5had9dqs4c_power_index_6194() {
            let made_hand: MadeHand = card_array!["Kd", "Th", "5h", "Ad", "9d", "Qs", "4c"].into();

            assert_eq!(made_hand.power_index(), 6194);
        }

        #[test]
        fn it_returns_ks5hjdjh5d9d6d_power_index_2887() {
            let made_hand: MadeHand = card_array!["Ks", "5h", "Jd", "Jh", "5d", "9d", "6d"].into();

            assert_eq!(made_hand.power_index(), 2887);
        }

        #[test]
        fn it_returns_5c8hahks9sas3h_power_index_3353() {
            let made_hand: MadeHand = card_array!["5c", "8h", "Ah", "Ks", "9s", "As", "3h"].into();

            assert_eq!(made_hand.power_index(), 3353);
        }

        #[test]
        fn it_returns_4dqs8cksqcackh_power_index_2600() {
            let made_hand: MadeHand = card_array!["4d", "Qs", "8c", "Ks", "Qc", "Ac", "Kh"].into();

            assert_eq!(made_hand.power_index(), 2600);
        }

        #[test]
        fn it_returns_9htcjc5hac9c4d_power_index_4445() {
            let made_hand: MadeHand = card_array!["9h", "Tc", "Jc", "5h", "Ac", "9c", "4d"].into();

            assert_eq!(made_hand.power_index(), 4445);
        }

        #[test]
        fn it_returns_thah6c4h4c4s3c_power_index_2273() {
            let made_hand: MadeHand = card_array!["Th", "Ah", "6c", "4h", "4c", "4s", "3c"].into();

            assert_eq!(made_hand.power_index(), 2273);
        }

        #[test]
        fn it_returns_6h8c9d3d4cadah_power_index_3491() {
            let made_hand: MadeHand = card_array!["6h", "8c", "9d", "3d", "4c", "Ad", "Ah"].into();

            assert_eq!(made_hand.power_index(), 3491);
        }

        #[test]
        fn it_returns_7s8d5hkh2hth4c_power_index_6904() {
            let made_hand: MadeHand = card_array!["7s", "8d", "5h", "Kh", "2h", "Th", "4c"].into();

            assert_eq!(made_hand.power_index(), 6904);
        }

        #[test]
        fn it_returns_ac8c9ctd4das5d_power_index_3462() {
            let made_hand: MadeHand = card_array!["Ac", "8c", "9c", "Td", "4d", "As", "5d"].into();

            assert_eq!(made_hand.power_index(), 3462);
        }

        #[test]
        fn it_returns_khjd8h5sacad4h_power_index_3338() {
            let made_hand: MadeHand = card_array!["Kh", "Jd", "8h", "5s", "Ac", "Ad", "4h"].into();

            assert_eq!(made_hand.power_index(), 3338);
        }

        #[test]
        fn it_returns_2c6d5d7d2s4s7c_power_index_3212() {
            let made_hand: MadeHand = card_array!["2c", "6d", "5d", "7d", "2s", "4s", "7c"].into();

            assert_eq!(made_hand.power_index(), 3212);
        }

        #[test]
        fn it_returns_kh6h7c3h6c4h5h_power_index_1140() {
            let made_hand: MadeHand = card_array!["Kh", "6h", "7c", "3h", "6c", "4h", "5h"].into();

            assert_eq!(made_hand.power_index(), 1140);
        }

        #[test]
        fn it_returns_8d7c8s8has5hth_power_index_2009() {
            let made_hand: MadeHand = card_array!["8d", "7c", "8s", "8h", "As", "5h", "Th"].into();

            assert_eq!(made_hand.power_index(), 2009);
        }

        #[test]
        fn it_returns_3h8s5s2ctcqh4c_power_index_7121() {
            let made_hand: MadeHand = card_array!["3h", "8s", "5s", "2c", "Tc", "Qh", "4c"].into();

            assert_eq!(made_hand.power_index(), 7121);
        }

        #[test]
        fn it_returns_qcjc6d7hth2c4s_power_index_7020() {
            let made_hand: MadeHand = card_array!["Qc", "Jc", "6d", "7h", "Th", "2c", "4s"].into();

            assert_eq!(made_hand.power_index(), 7020);
        }

        #[test]
        fn it_returns_tc5hth2cqh4c8c_power_index_4323() {
            let made_hand: MadeHand = card_array!["Tc", "5h", "Th", "2c", "Qh", "4c", "8c"].into();

            assert_eq!(made_hand.power_index(), 4323);
        }

        #[test]
        fn it_returns_3s7sqc9ctd8sqh_power_index_3902() {
            let made_hand: MadeHand = card_array!["3s", "7s", "Qc", "9c", "Td", "8s", "Qh"].into();

            assert_eq!(made_hand.power_index(), 3902);
        }

        #[test]
        fn it_returns_ts4djd7ckc3c9h_power_index_6799() {
            let made_hand: MadeHand = card_array!["Ts", "4d", "Jd", "7c", "Kc", "3c", "9h"].into();

            assert_eq!(made_hand.power_index(), 6799);
        }

        #[test]
        fn it_returns_6hkhjd8skd6dks_power_index_186() {
            let made_hand: MadeHand = card_array!["6h", "Kh", "Jd", "8s", "Kd", "6d", "Ks"].into();

            assert_eq!(made_hand.power_index(), 186);
        }

        #[test]
        fn it_returns_7c3dtc4cqhtd6s_power_index_4327() {
            let made_hand: MadeHand = card_array!["7c", "3d", "Tc", "4c", "Qh", "Td", "6s"].into();

            assert_eq!(made_hand.power_index(), 4327);
        }

        #[test]
        fn it_returns_5d8d4hjd6htd9d_power_index_1355() {
            let made_hand: MadeHand = card_array!["5d", "8d", "4h", "Jd", "6h", "Td", "9d"].into();

            assert_eq!(made_hand.power_index(), 1355);
        }

        #[test]
        fn it_returns_7c8hadqh9c6h8s_power_index_4658() {
            let made_hand: MadeHand = card_array!["7c", "8h", "Ad", "Qh", "9c", "6h", "8s"].into();

            assert_eq!(made_hand.power_index(), 4658);
        }

        #[test]
        fn it_returns_2d3ctcjs7hjdjh_power_index_1840() {
            let made_hand: MadeHand = card_array!["2d", "3c", "Tc", "Js", "7h", "Jd", "Jh"].into();

            assert_eq!(made_hand.power_index(), 1840);
        }

        #[test]
        fn it_returns_5cqsasqdkh3d6s_power_index_3771() {
            let made_hand: MadeHand = card_array!["5c", "Qs", "As", "Qd", "Kh", "3d", "6s"].into();

            assert_eq!(made_hand.power_index(), 3771);
        }

        #[test]
        fn it_returns_6sjsjh3cjcahkc_power_index_1808() {
            let made_hand: MadeHand = card_array!["6s", "Js", "Jh", "3c", "Jc", "Ah", "Kc"].into();

            assert_eq!(made_hand.power_index(), 1808);
        }

        #[test]
        fn it_returns_tc8h5s2sacjhjs_power_index_4006() {
            let made_hand: MadeHand = card_array!["Tc", "8h", "5s", "2s", "Ac", "Jh", "Js"].into();

            assert_eq!(made_hand.power_index(), 4006);
        }

        #[test]
        fn it_returns_8s5d2d4d6d5hjd_power_index_1475() {
            let made_hand: MadeHand = card_array!["8s", "5d", "2d", "4d", "6d", "5h", "Jd"].into();

            assert_eq!(made_hand.power_index(), 1475);
        }

        #[test]
        fn it_returns_8c5d9sqd2h2s5c_power_index_3284() {
            let made_hand: MadeHand = card_array!["8c", "5d", "9s", "Qd", "2h", "2s", "5c"].into();

            assert_eq!(made_hand.power_index(), 3284);
        }

        #[test]
        fn it_returns_4dthqh9h4ckd2s_power_index_5582() {
            let made_hand: MadeHand = card_array!["4d", "Th", "Qh", "9h", "4c", "Kd", "2s"].into();

            assert_eq!(made_hand.power_index(), 5582);
        }

        #[test]
        fn it_returns_5h5s2hah9htc5d_power_index_2207() {
            let made_hand: MadeHand = card_array!["5h", "5s", "2h", "Ah", "9h", "Tc", "5d"].into();

            assert_eq!(made_hand.power_index(), 2207);
        }

        #[test]
        fn it_returns_tctsjc9c7c4d9s_power_index_2933() {
            let made_hand: MadeHand = card_array!["Tc", "Ts", "Jc", "9c", "7c", "4d", "9s"].into();

            assert_eq!(made_hand.power_index(), 2933);
        }

        #[test]
        fn it_returns_jh9sjd6c5d7c9h_power_index_2847() {
            let made_hand: MadeHand = card_array!["Jh", "9s", "Jd", "6c", "5d", "7c", "9h"].into();

            assert_eq!(made_hand.power_index(), 2847);
        }

        #[test]
        fn it_returns_5htckd2sthasad_power_index_2501() {
            let made_hand: MadeHand = card_array!["5h", "Tc", "Kd", "2s", "Th", "As", "Ad"].into();

            assert_eq!(made_hand.power_index(), 2501);
        }

        #[test]
        fn it_returns_6d9dqsqcjh7d4d_power_index_3875() {
            let made_hand: MadeHand = card_array!["6d", "9d", "Qs", "Qc", "Jh", "7d", "4d"].into();

            assert_eq!(made_hand.power_index(), 3875);
        }

        #[test]
        fn it_returns_9cqs3hqdqckd9d_power_index_195() {
            let made_hand: MadeHand = card_array!["9c", "Qs", "3h", "Qd", "Qc", "Kd", "9d"].into();

            assert_eq!(made_hand.power_index(), 195);
        }

        #[test]
        fn it_returns_jh5djcth7hqs2c_power_index_4088() {
            let made_hand: MadeHand = card_array!["Jh", "5d", "Jc", "Th", "7h", "Qs", "2c"].into();

            assert_eq!(made_hand.power_index(), 4088);
        }

        #[test]
        fn it_returns_9h5hac5d7sqs2d_power_index_5318() {
            let made_hand: MadeHand = card_array!["9h", "5h", "Ac", "5d", "7s", "Qs", "2d"].into();

            assert_eq!(made_hand.power_index(), 5318);
        }

        #[test]
        fn it_returns_2htc6d3cth2c4c_power_index_3014() {
            let made_hand: MadeHand = card_array!["2h", "Tc", "6d", "3c", "Th", "2c", "4c"].into();

            assert_eq!(made_hand.power_index(), 3014);
        }

        #[test]
        fn it_returns_9d8s8hqdkdjh3c_power_index_4701() {
            let made_hand: MadeHand = card_array!["9d", "8s", "8h", "Qd", "Kd", "Jh", "3c"].into();

            assert_eq!(made_hand.power_index(), 4701);
        }

        #[test]
        fn it_returns_2c8s6djdjh4dqh_power_index_4102() {
            let made_hand: MadeHand = card_array!["2c", "8s", "6d", "Jd", "Jh", "4d", "Qh"].into();

            assert_eq!(made_hand.power_index(), 4102);
        }

        #[test]
        fn it_returns_3s6h3dtskhqh5s_power_index_5802() {
            let made_hand: MadeHand = card_array!["3s", "6h", "3d", "Ts", "Kh", "Qh", "5s"].into();

            assert_eq!(made_hand.power_index(), 5802);
        }

        #[test]
        fn it_returns_qs3d5djctsqc9c_power_index_3866() {
            let made_hand: MadeHand = card_array!["Qs", "3d", "5d", "Jc", "Ts", "Qc", "9c"].into();

            assert_eq!(made_hand.power_index(), 3866);
        }

        #[test]
        fn it_returns_9hjh3h9d6c4c8h_power_index_4570() {
            let made_hand: MadeHand = card_array!["9h", "Jh", "3h", "9d", "6c", "4c", "8h"].into();

            assert_eq!(made_hand.power_index(), 4570);
        }

        #[test]
        fn it_returns_jd7c9d3dqh3h2d_power_index_5847() {
            let made_hand: MadeHand = card_array!["Jd", "7c", "9d", "3d", "Qh", "3h", "2d"].into();

            assert_eq!(made_hand.power_index(), 5847);
        }

        #[test]
        fn it_returns_qh5c6d8d9hjs7c_power_index_1605() {
            let made_hand: MadeHand = card_array!["Qh", "5c", "6d", "8d", "9h", "Js", "7c"].into();

            assert_eq!(made_hand.power_index(), 1605);
        }

        #[test]
        fn it_returns_5cjs2d6sjctd4d_power_index_4140() {
            let made_hand: MadeHand = card_array!["5c", "Js", "2d", "6s", "Jc", "Td", "4d"].into();

            assert_eq!(made_hand.power_index(), 4140);
        }

        #[test]
        fn it_returns_5dtc7sqc9s5h6s_power_index_5414() {
            let made_hand: MadeHand = card_array!["5d", "Tc", "7s", "Qc", "9s", "5h", "6s"].into();

            assert_eq!(made_hand.power_index(), 5414);
        }

        #[test]
        fn it_returns_ad6s2d7cjs7d4c_power_index_4888() {
            let made_hand: MadeHand = card_array!["Ad", "6s", "2d", "7c", "Js", "7d", "4c"].into();

            assert_eq!(made_hand.power_index(), 4888);
        }

        #[test]
        fn it_returns_7s6sqc4h2h8c8s_power_index_4767() {
            let made_hand: MadeHand = card_array!["7s", "6s", "Qc", "4h", "2h", "8c", "8s"].into();

            assert_eq!(made_hand.power_index(), 4767);
        }

        #[test]
        fn it_returns_js9c5c9sksth8d_power_index_4490() {
            let made_hand: MadeHand = card_array!["Js", "9c", "5c", "9s", "Ks", "Th", "8d"].into();

            assert_eq!(made_hand.power_index(), 4490);
        }

        #[test]
        fn it_returns_7dtsqdkdkc5dad_power_index_353() {
            let made_hand: MadeHand = card_array!["7d", "Ts", "Qd", "Kd", "Kc", "5d", "Ad"].into();

            assert_eq!(made_hand.power_index(), 353);
        }

        #[test]
        fn it_returns_6hjdkcad5h7d4h_power_index_6251() {
            let made_hand: MadeHand = card_array!["6h", "Jd", "Kc", "Ad", "5h", "7d", "4h"].into();

            assert_eq!(made_hand.power_index(), 6251);
        }

        #[test]
        fn it_returns_jc5h9sqh5d9hjs_power_index_2844() {
            let made_hand: MadeHand = card_array!["Jc", "5h", "9s", "Qh", "5d", "9h", "Js"].into();

            assert_eq!(made_hand.power_index(), 2844);
        }

        #[test]
        fn it_returns_8sadac3s9dqdks_power_index_3328() {
            let made_hand: MadeHand = card_array!["8s", "Ad", "Ac", "3s", "9d", "Qd", "Ks"].into();

            assert_eq!(made_hand.power_index(), 3328);
        }

        #[test]
        fn it_returns_th4h2h4d7sts8d_power_index_2990() {
            let made_hand: MadeHand = card_array!["Th", "4h", "2h", "4d", "7s", "Ts", "8d"].into();

            assert_eq!(made_hand.power_index(), 2990);
        }

        #[test]
        fn it_returns_qdqhth5sad3skh_power_index_3767() {
            let made_hand: MadeHand = card_array!["Qd", "Qh", "Th", "5s", "Ad", "3s", "Kh"].into();

            assert_eq!(made_hand.power_index(), 3767);
        }

        #[test]
        fn it_returns_2djsqs7h5hts9d_power_index_7008() {
            let made_hand: MadeHand = card_array!["2d", "Js", "Qs", "7h", "5h", "Ts", "9d"].into();

            assert_eq!(made_hand.power_index(), 7008);
        }

        #[test]
        fn it_returns_khkd2cqd3d9h3c_power_index_2700() {
            let made_hand: MadeHand = card_array!["Kh", "Kd", "2c", "Qd", "3d", "9h", "3c"].into();

            assert_eq!(made_hand.power_index(), 2700);
        }

        #[test]
        fn it_returns_4ckc6sjh5h6c9c_power_index_5151() {
            let made_hand: MadeHand = card_array!["4c", "Kc", "6s", "Jh", "5h", "6c", "9c"].into();

            assert_eq!(made_hand.power_index(), 5151);
        }

        #[test]
        fn it_returns_9cjh5h2s4hjs7h_power_index_4157() {
            let made_hand: MadeHand = card_array!["9c", "Jh", "5h", "2s", "4h", "Js", "7h"].into();

            assert_eq!(made_hand.power_index(), 4157);
        }

        #[test]
        fn it_returns_as5hqc2s7c8s4c_power_index_6436() {
            let made_hand: MadeHand = card_array!["As", "5h", "Qc", "2s", "7c", "8s", "4c"].into();

            assert_eq!(made_hand.power_index(), 6436);
        }

        #[test]
        fn it_returns_5sksacjc3stdth_power_index_4207() {
            let made_hand: MadeHand = card_array!["5s", "Ks", "Ac", "Jc", "3s", "Td", "Th"].into();

            assert_eq!(made_hand.power_index(), 4207);
        }

        #[test]
        fn it_returns_qcjh5dah7dac9d_power_index_3382() {
            let made_hand: MadeHand = card_array!["Qc", "Jh", "5d", "Ah", "7d", "Ac", "9d"].into();

            assert_eq!(made_hand.power_index(), 3382);
        }

        #[test]
        fn it_returns_qdahkhjcjd7s4c_power_index_3986() {
            let made_hand: MadeHand = card_array!["Qd", "Ah", "Kh", "Jc", "Jd", "7s", "4c"].into();

            assert_eq!(made_hand.power_index(), 3986);
        }

        #[test]
        fn it_returns_4sah4cth7ctsqd_power_index_2985() {
            let made_hand: MadeHand = card_array!["4s", "Ah", "4c", "Th", "7c", "Ts", "Qd"].into();

            assert_eq!(made_hand.power_index(), 2985);
        }

        #[test]
        fn it_returns_jhth6h3c8s5ctc_power_index_4350() {
            let made_hand: MadeHand = card_array!["Jh", "Th", "6h", "3c", "8s", "5c", "Tc"].into();

            assert_eq!(made_hand.power_index(), 4350);
        }

        #[test]
        fn it_returns_9c3s4dkctd8hqh_power_index_6714() {
            let made_hand: MadeHand = card_array!["9c", "3s", "4d", "Kc", "Td", "8h", "Qh"].into();

            assert_eq!(made_hand.power_index(), 6714);
        }

        #[test]
        fn it_returns_jc3h9s4h3sqc8d_power_index_5847() {
            let made_hand: MadeHand = card_array!["Jc", "3h", "9s", "4h", "3s", "Qc", "8d"].into();

            assert_eq!(made_hand.power_index(), 5847);
        }

        #[test]
        fn it_returns_6cjh2c8c7cad7s_power_index_4887() {
            let made_hand: MadeHand = card_array!["6c", "Jh", "2c", "8c", "7c", "Ad", "7s"].into();

            assert_eq!(made_hand.power_index(), 4887);
        }

        #[test]
        fn it_returns_3cqs6d4c7h9h9s_power_index_4547() {
            let made_hand: MadeHand = card_array!["3c", "Qs", "6d", "4c", "7h", "9h", "9s"].into();

            assert_eq!(made_hand.power_index(), 4547);
        }

        #[test]
        fn it_returns_2h3h8dkc8s5h6s_power_index_4736() {
            let made_hand: MadeHand = card_array!["2h", "3h", "8d", "Kc", "8s", "5h", "6s"].into();

            assert_eq!(made_hand.power_index(), 4736);
        }

        #[test]
        fn it_returns_7skh8s6h4s7hks_power_index_2660() {
            let made_hand: MadeHand = card_array!["7s", "Kh", "8s", "6h", "4s", "7h", "Ks"].into();

            assert_eq!(made_hand.power_index(), 2660);
        }

        #[test]
        fn it_returns_6h4h3sth3d4ckc_power_index_3294() {
            let made_hand: MadeHand = card_array!["6h", "4h", "3s", "Th", "3d", "4c", "Kc"].into();

            assert_eq!(made_hand.power_index(), 3294);
        }

        #[test]
        fn it_returns_thqsasjh3d9h4s_power_index_6350() {
            let made_hand: MadeHand = card_array!["Th", "Qs", "As", "Jh", "3d", "9h", "4s"].into();

            assert_eq!(made_hand.power_index(), 6350);
        }

        #[test]
        fn it_returns_ksjd9s3d3c4d6s_power_index_5811() {
            let made_hand: MadeHand = card_array!["Ks", "Jd", "9s", "3d", "3c", "4d", "6s"].into();

            assert_eq!(made_hand.power_index(), 5811);
        }

        #[test]
        fn it_returns_9s4cad7c6sackc_power_index_3354() {
            let made_hand: MadeHand = card_array!["9s", "4c", "Ad", "7c", "6s", "Ac", "Kc"].into();

            assert_eq!(made_hand.power_index(), 3354);
        }

        #[test]
        fn it_returns_ahjs7h4hac9s5s_power_index_3435() {
            let made_hand: MadeHand = card_array!["Ah", "Js", "7h", "4h", "Ac", "9s", "5s"].into();

            assert_eq!(made_hand.power_index(), 3435);
        }

        #[test]
        fn it_returns_5dah9d9cjh8c8h_power_index_3018() {
            let made_hand: MadeHand = card_array!["5d", "Ah", "9d", "9c", "Jh", "8c", "8h"].into();

            assert_eq!(made_hand.power_index(), 3018);
        }

        #[test]
        fn it_returns_6c3s6hkcas5s6s_power_index_2138() {
            let made_hand: MadeHand = card_array!["6c", "3s", "6h", "Kc", "As", "5s", "6s"].into();

            assert_eq!(made_hand.power_index(), 2138);
        }

        #[test]
        fn it_returns_3h7d4c7s7ctc8h_power_index_2111() {
            let made_hand: MadeHand = card_array!["3h", "7d", "4c", "7s", "7c", "Tc", "8h"].into();

            assert_eq!(made_hand.power_index(), 2111);
        }

        #[test]
        fn it_returns_qsac3djc2d9htc_power_index_6350() {
            let made_hand: MadeHand = card_array!["Qs", "Ac", "3d", "Jc", "2d", "9h", "Tc"].into();

            assert_eq!(made_hand.power_index(), 6350);
        }

        #[test]
        fn it_returns_3s8s7s6sas3cth_power_index_784() {
            let made_hand: MadeHand = card_array!["3s", "8s", "7s", "6s", "As", "3c", "Th"].into();

            assert_eq!(made_hand.power_index(), 784);
        }

        #[test]
        fn it_returns_4s2hah8h3d7c8d_power_index_4688() {
            let made_hand: MadeHand = card_array!["4s", "2h", "Ah", "8h", "3d", "7c", "8d"].into();

            assert_eq!(made_hand.power_index(), 4688);
        }

        #[test]
        fn it_returns_9hjs8sqcqs5hts_power_index_1602() {
            let made_hand: MadeHand = card_array!["9h", "Js", "8s", "Qc", "Qs", "5h", "Ts"].into();

            assert_eq!(made_hand.power_index(), 1602);
        }

        #[test]
        fn it_returns_kd7cqd5c3h6dkh_power_index_3631() {
            let made_hand: MadeHand = card_array!["Kd", "7c", "Qd", "5c", "3h", "6d", "Kh"].into();

            assert_eq!(made_hand.power_index(), 3631);
        }

        #[test]
        fn it_returns_5c6h8c3c2hjd4c_power_index_1608() {
            let made_hand: MadeHand = card_array!["5c", "6h", "8c", "3c", "2h", "Jd", "4c"].into();

            assert_eq!(made_hand.power_index(), 1608);
        }

        #[test]
        fn it_returns_6ctcjhahqd2cjd_power_index_3996() {
            let made_hand: MadeHand = card_array!["6c", "Tc", "Jh", "Ah", "Qd", "2c", "Jd"].into();

            assert_eq!(made_hand.power_index(), 3996);
        }

        #[test]
        fn it_returns_kcad4c6c4htc7h_power_index_5528() {
            let made_hand: MadeHand = card_array!["Kc", "Ad", "4c", "6c", "4h", "Tc", "7h"].into();

            assert_eq!(made_hand.power_index(), 5528);
        }

        #[test]
        fn it_returns_3sas3d6skh7c5d_power_index_5751() {
            let made_hand: MadeHand = card_array!["3s", "As", "3d", "6s", "Kh", "7c", "5d"].into();

            assert_eq!(made_hand.power_index(), 5751);
        }

        #[test]
        fn it_returns_7h8c9sadqdjdjs_power_index_3997() {
            let made_hand: MadeHand = card_array!["7h", "8c", "9s", "Ad", "Qd", "Jd", "Js"].into();

            assert_eq!(made_hand.power_index(), 3997);
        }

        #[test]
        fn it_returns_9h9s7sac7dts5c_power_index_3029() {
            let made_hand: MadeHand = card_array!["9h", "9s", "7s", "Ac", "7d", "Ts", "5c"].into();

            assert_eq!(made_hand.power_index(), 3029);
        }

        #[test]
        fn it_returns_4sjsas2d2h4c6c_power_index_3304() {
            let made_hand: MadeHand = card_array!["4s", "Js", "As", "2d", "2h", "4c", "6c"].into();

            assert_eq!(made_hand.power_index(), 3304);
        }

        #[test]
        fn it_returns_6s6d8s7hqs8hqd_power_index_2759() {
            let made_hand: MadeHand = card_array!["6s", "6d", "8s", "7h", "Qs", "8h", "Qd"].into();

            assert_eq!(made_hand.power_index(), 2759);
        }

        #[test]
        fn it_returns_4s7d7hqc4d2d4c_power_index_294() {
            let made_hand: MadeHand = card_array!["4s", "7d", "7h", "Qc", "4d", "2d", "4c"].into();

            assert_eq!(made_hand.power_index(), 294);
        }

        #[test]
        fn it_returns_7c6hjc9d5d9skc_power_index_4492() {
            let made_hand: MadeHand = card_array!["7c", "6h", "Jc", "9d", "5d", "9s", "Kc"].into();

            assert_eq!(made_hand.power_index(), 4492);
        }

        #[test]
        fn it_returns_qskh8d3s8c4s6c_power_index_4705() {
            let made_hand: MadeHand = card_array!["Qs", "Kh", "8d", "3s", "8c", "4s", "6c"].into();

            assert_eq!(made_hand.power_index(), 4705);
        }

        #[test]
        fn it_returns_tsqs6h9cqh3s9d_power_index_2746() {
            let made_hand: MadeHand = card_array!["Ts", "Qs", "6h", "9c", "Qh", "3s", "9d"].into();

            assert_eq!(made_hand.power_index(), 2746);
        }

        #[test]
        fn it_returns_7cks6sjd4h6dah_power_index_5087() {
            let made_hand: MadeHand = card_array!["7c", "Ks", "6s", "Jd", "4h", "6d", "Ah"].into();

            assert_eq!(made_hand.power_index(), 5087);
        }

        #[test]
        fn it_returns_6c9ctsth5sjd8s_power_index_4342() {
            let made_hand: MadeHand = card_array!["6c", "9c", "Ts", "Th", "5s", "Jd", "8s"].into();

            assert_eq!(made_hand.power_index(), 4342);
        }

        #[test]
        fn it_returns_5h3ctc7s5d9sjc_power_index_5442() {
            let made_hand: MadeHand = card_array!["5h", "3c", "Tc", "7s", "5d", "9s", "Jc"].into();

            assert_eq!(made_hand.power_index(), 5442);
        }

        #[test]
        fn it_returns_6s7d5skh9d9sqd_power_index_4484() {
            let made_hand: MadeHand = card_array!["6s", "7d", "5s", "Kh", "9d", "9s", "Qd"].into();

            assert_eq!(made_hand.power_index(), 4484);
        }

        #[test]
        fn it_returns_ks2hkd3cqh8h8c_power_index_2645() {
            let made_hand: MadeHand = card_array!["Ks", "2h", "Kd", "3c", "Qh", "8h", "8c"].into();

            assert_eq!(made_hand.power_index(), 2645);
        }

        #[test]
        fn it_returns_7djcad8c2sjh6c_power_index_4020() {
            let made_hand: MadeHand = card_array!["7d", "Jc", "Ad", "8c", "2s", "Jh", "6c"].into();

            assert_eq!(made_hand.power_index(), 4020);
        }

        #[test]
        fn it_returns_2h3sah7c8dth9h_power_index_6554() {
            let made_hand: MadeHand = card_array!["2h", "3s", "Ah", "7c", "8d", "Th", "9h"].into();

            assert_eq!(made_hand.power_index(), 6554);
        }

        #[test]
        fn it_returns_4htcqhad4sjh3h_power_index_5536() {
            let made_hand: MadeHand = card_array!["4h", "Tc", "Qh", "Ad", "4s", "Jh", "3h"].into();

            assert_eq!(made_hand.power_index(), 5536);
        }

        #[test]
        fn it_returns_7d8cks8hah5c2h_power_index_4650() {
            let made_hand: MadeHand = card_array!["7d", "8c", "Ks", "8h", "Ah", "5c", "2h"].into();

            assert_eq!(made_hand.power_index(), 4650);
        }

        #[test]
        fn it_returns_9dtsjdas5s4s5h_power_index_5325() {
            let made_hand: MadeHand = card_array!["9d", "Ts", "Jd", "As", "5s", "4s", "5h"].into();

            assert_eq!(made_hand.power_index(), 5325);
        }

        #[test]
        fn it_returns_3cqdqsjc7c3s2s_power_index_2811() {
            let made_hand: MadeHand = card_array!["3c", "Qd", "Qs", "Jc", "7c", "3s", "2s"].into();

            assert_eq!(made_hand.power_index(), 2811);
        }

        #[test]
        fn it_returns_asts6c7h4h2djh_power_index_6483() {
            let made_hand: MadeHand = card_array!["As", "Ts", "6c", "7h", "4h", "2d", "Jh"].into();

            assert_eq!(made_hand.power_index(), 6483);
        }

        #[test]
        fn it_returns_kc7sks9h2cts3s_power_index_3683() {
            let made_hand: MadeHand = card_array!["Kc", "7s", "Ks", "9h", "2c", "Ts", "3s"].into();

            assert_eq!(made_hand.power_index(), 3683);
        }

        #[test]
        fn it_returns_6h8cts3d6d4sjc_power_index_5223() {
            let made_hand: MadeHand = card_array!["6h", "8c", "Ts", "3d", "6d", "4s", "Jc"].into();

            assert_eq!(made_hand.power_index(), 5223);
        }

        #[test]
        fn it_returns_2hkh8dac4std9d_power_index_6266() {
            let made_hand: MadeHand = card_array!["2h", "Kh", "8d", "Ac", "4s", "Td", "9d"].into();

            assert_eq!(made_hand.power_index(), 6266);
        }

        #[test]
        fn it_returns_4d3sks7s2s9cad_power_index_6302() {
            let made_hand: MadeHand = card_array!["4d", "3s", "Ks", "7s", "2s", "9c", "Ad"].into();

            assert_eq!(made_hand.power_index(), 6302);
        }

        #[test]
        fn it_returns_3d6dqh7h6h4d2d_power_index_5213() {
            let made_hand: MadeHand = card_array!["3d", "6d", "Qh", "7h", "6h", "4d", "2d"].into();

            assert_eq!(made_hand.power_index(), 5213);
        }

        #[test]
        fn it_returns_2d8h2h9hahjcqd_power_index_5976() {
            let made_hand: MadeHand = card_array!["2d", "8h", "2h", "9h", "Ah", "Jc", "Qd"].into();

            assert_eq!(made_hand.power_index(), 5976);
        }

        #[test]
        fn it_returns_9sjcts7cas5djh_power_index_4005() {
            let made_hand: MadeHand = card_array!["9s", "Jc", "Ts", "7c", "As", "5d", "Jh"].into();

            assert_eq!(made_hand.power_index(), 4005);
        }

        #[test]
        fn it_returns_9dks8h4sqs4hkh_power_index_2689() {
            let made_hand: MadeHand = card_array!["9d", "Ks", "8h", "4s", "Qs", "4h", "Kh"].into();

            assert_eq!(made_hand.power_index(), 2689);
        }

        #[test]
        fn it_returns_4d7d4s4h3s9s5c_power_index_2316() {
            let made_hand: MadeHand = card_array!["4d", "7d", "4s", "4h", "3s", "9s", "5c"].into();

            assert_eq!(made_hand.power_index(), 2316);
        }

        #[test]
        fn it_returns_jd2dqdac7d8d8h_power_index_1197() {
            let made_hand: MadeHand = card_array!["Jd", "2d", "Qd", "Ac", "7d", "8d", "8h"].into();

            assert_eq!(made_hand.power_index(), 1197);
        }

        #[test]
        fn it_returns_6dth7s8cjdtd4c_power_index_4349() {
            let made_hand: MadeHand = card_array!["6d", "Th", "7s", "8c", "Jd", "Td", "4c"].into();

            assert_eq!(made_hand.power_index(), 4349);
        }

        #[test]
        fn it_returns_asacqs6d2c7hjc_power_index_3384() {
            let made_hand: MadeHand = card_array!["As", "Ac", "Qs", "6d", "2c", "7h", "Jc"].into();

            assert_eq!(made_hand.power_index(), 3384);
        }

        #[test]
        fn it_returns_2d9hqc8d6sqd4c_power_index_3931() {
            let made_hand: MadeHand = card_array!["2d", "9h", "Qc", "8d", "6s", "Qd", "4c"].into();

            assert_eq!(made_hand.power_index(), 3931);
        }

        #[test]
        fn it_returns_3sts6cqs7c4cah_power_index_6399() {
            let made_hand: MadeHand = card_array!["3s", "Ts", "6c", "Qs", "7c", "4c", "Ah"].into();

            assert_eq!(made_hand.power_index(), 6399);
        }

        #[test]
        fn it_returns_qd5h9d6c5s8cad_power_index_5318() {
            let made_hand: MadeHand = card_array!["Qd", "5h", "9d", "6c", "5s", "8c", "Ad"].into();

            assert_eq!(made_hand.power_index(), 5318);
        }

        #[test]
        fn it_returns_jh9d6c8h5d7sqc_power_index_1605() {
            let made_hand: MadeHand = card_array!["Jh", "9d", "6c", "8h", "5d", "7s", "Qc"].into();

            assert_eq!(made_hand.power_index(), 1605);
        }

        #[test]
        fn it_returns_3d4h8s4sjc7cqs_power_index_5628() {
            let made_hand: MadeHand = card_array!["3d", "4h", "8s", "4s", "Jc", "7c", "Qs"].into();

            assert_eq!(made_hand.power_index(), 5628);
        }

        #[test]
        fn it_returns_jc9d9c5cjsad6d_power_index_2842() {
            let made_hand: MadeHand = card_array!["Jc", "9d", "9c", "5c", "Js", "Ad", "6d"].into();

            assert_eq!(made_hand.power_index(), 2842);
        }

        #[test]
        fn it_returns_qhjsjcjh8d4c5c_power_index_1831() {
            let made_hand: MadeHand = card_array!["Qh", "Js", "Jc", "Jh", "8d", "4c", "5c"].into();

            assert_eq!(made_hand.power_index(), 1831);
        }

        #[test]
        fn it_returns_3h7s9d5hactd5s_power_index_5333() {
            let made_hand: MadeHand = card_array!["3h", "7s", "9d", "5h", "Ac", "Td", "5s"].into();

            assert_eq!(made_hand.power_index(), 5333);
        }

        #[test]
        fn it_returns_td4c2d4s6cqdqs_power_index_2801() {
            let made_hand: MadeHand = card_array!["Td", "4c", "2d", "4s", "6c", "Qd", "Qs"].into();

            assert_eq!(made_hand.power_index(), 2801);
        }

        #[test]
        fn it_returns_9s4s2s8sqstd2d_power_index_1297() {
            let made_hand: MadeHand = card_array!["9s", "4s", "2s", "8s", "Qs", "Td", "2d"].into();

            assert_eq!(made_hand.power_index(), 1297);
        }

        #[test]
        fn it_returns_6c4h7sjd2hqsqc_power_index_3887() {
            let made_hand: MadeHand = card_array!["6c", "4h", "7s", "Jd", "2h", "Qs", "Qc"].into();

            assert_eq!(made_hand.power_index(), 3887);
        }

        #[test]
        fn it_returns_8skstd7d9sth8d_power_index_2942() {
            let made_hand: MadeHand = card_array!["8s", "Ks", "Td", "7d", "9s", "Th", "8d"].into();

            assert_eq!(made_hand.power_index(), 2942);
        }

        #[test]
        fn it_returns_7c9d8s2s3s9sjh_power_index_4569() {
            let made_hand: MadeHand = card_array!["7c", "9d", "8s", "2s", "3s", "9s", "Jh"].into();

            assert_eq!(made_hand.power_index(), 4569);
        }

        #[test]
        fn it_returns_jd6sqd7d9s5hkc_power_index_6687() {
            let made_hand: MadeHand = card_array!["Jd", "6s", "Qd", "7d", "9s", "5h", "Kc"].into();

            assert_eq!(made_hand.power_index(), 6687);
        }

        #[test]
        fn it_returns_jcac6d2s8h9h6s_power_index_5106() {
            let made_hand: MadeHand = card_array!["Jc", "Ac", "6d", "2s", "8h", "9h", "6s"].into();

            assert_eq!(made_hand.power_index(), 5106);
        }

        #[test]
        fn it_returns_8d6c4h8c6dqc9d_power_index_3108() {
            let made_hand: MadeHand = card_array!["8d", "6c", "4h", "8c", "6d", "Qc", "9d"].into();

            assert_eq!(made_hand.power_index(), 3108);
        }

        #[test]
        fn it_returns_5d2dtsjdas9c3c_power_index_6473() {
            let made_hand: MadeHand = card_array!["5d", "2d", "Ts", "Jd", "As", "9c", "3c"].into();

            assert_eq!(made_hand.power_index(), 6473);
        }

        #[test]
        fn it_returns_qc3cjh8c3dkd9d_power_index_5801() {
            let made_hand: MadeHand = card_array!["Qc", "3c", "Jh", "8c", "3d", "Kd", "9d"].into();

            assert_eq!(made_hand.power_index(), 5801);
        }

        #[test]
        fn it_returns_6s8d5s9c4s3cks_power_index_6943() {
            let made_hand: MadeHand = card_array!["6s", "8d", "5s", "9c", "4s", "3c", "Ks"].into();

            assert_eq!(made_hand.power_index(), 6943);
        }

        #[test]
        fn it_returns_3s8ckdkhjdts9s_power_index_3646() {
            let made_hand: MadeHand = card_array!["3s", "8c", "Kd", "Kh", "Jd", "Ts", "9s"].into();

            assert_eq!(made_hand.power_index(), 3646);
        }

        #[test]
        fn it_returns_7d8h3c7hjd5h6d_power_index_5015() {
            let made_hand: MadeHand = card_array!["7d", "8h", "3c", "7h", "Jd", "5h", "6d"].into();

            assert_eq!(made_hand.power_index(), 5015);
        }

        #[test]
        fn it_returns_3d3h9sqs7s9c8h_power_index_3075() {
            let made_hand: MadeHand = card_array!["3d", "3h", "9s", "Qs", "7s", "9c", "8h"].into();

            assert_eq!(made_hand.power_index(), 3075);
        }

        #[test]
        fn it_returns_tdqs3s3c6c8d4d_power_index_5855() {
            let made_hand: MadeHand = card_array!["Td", "Qs", "3s", "3c", "6c", "8d", "4d"].into();

            assert_eq!(made_hand.power_index(), 5855);
        }

        #[test]
        fn it_returns_3h9s7c7d2s9h4h_power_index_3037() {
            let made_hand: MadeHand = card_array!["3h", "9s", "7c", "7d", "2s", "9h", "4h"].into();

            assert_eq!(made_hand.power_index(), 3037);
        }

        #[test]
        fn it_returns_4sjh7dqh9d5c8c_power_index_7035() {
            let made_hand: MadeHand = card_array!["4s", "Jh", "7d", "Qh", "9d", "5c", "8c"].into();

            assert_eq!(made_hand.power_index(), 7035);
        }

        #[test]
        fn it_returns_4c6s6d2dad9c8s_power_index_5120() {
            let made_hand: MadeHand = card_array!["4c", "6s", "6d", "2d", "Ad", "9c", "8s"].into();

            assert_eq!(made_hand.power_index(), 5120);
        }

        #[test]
        fn it_returns_jh7s2s5cjsqh9h_power_index_4095() {
            let made_hand: MadeHand = card_array!["Jh", "7s", "2s", "5c", "Js", "Qh", "9h"].into();

            assert_eq!(made_hand.power_index(), 4095);
        }

        #[test]
        fn it_returns_ks7sjhahkdqc6h_power_index_3546() {
            let made_hand: MadeHand = card_array!["Ks", "7s", "Jh", "Ah", "Kd", "Qc", "6h"].into();

            assert_eq!(made_hand.power_index(), 3546);
        }

        #[test]
        fn it_returns_4cjhqs8hkc2std_power_index_6679() {
            let made_hand: MadeHand = card_array!["4c", "Jh", "Qs", "8h", "Kc", "2s", "Td"].into();

            assert_eq!(made_hand.power_index(), 6679);
        }

        #[test]
        fn it_returns_tcjc9s2d5c9dqc_power_index_4526() {
            let made_hand: MadeHand = card_array!["Tc", "Jc", "9s", "2d", "5c", "9d", "Qc"].into();

            assert_eq!(made_hand.power_index(), 4526);
        }

        #[test]
        fn it_returns_8d5djd7d8h3c6c_power_index_4795() {
            let made_hand: MadeHand = card_array!["8d", "5d", "Jd", "7d", "8h", "3c", "6c"].into();

            assert_eq!(made_hand.power_index(), 4795);
        }

        #[test]
        fn it_returns_5s7hjdkd3c4dqd_power_index_6700() {
            let made_hand: MadeHand = card_array!["5s", "7h", "Jd", "Kd", "3c", "4d", "Qd"].into();

            assert_eq!(made_hand.power_index(), 6700);
        }

        #[test]
        fn it_returns_kh6h5h8dkcqd6s_power_index_2667() {
            let made_hand: MadeHand = card_array!["Kh", "6h", "5h", "8d", "Kc", "Qd", "6s"].into();

            assert_eq!(made_hand.power_index(), 2667);
        }

        #[test]
        fn it_returns_tstdqs4h2cjs4d_power_index_2987() {
            let made_hand: MadeHand = card_array!["Ts", "Td", "Qs", "4h", "2c", "Js", "4d"].into();

            assert_eq!(made_hand.power_index(), 2987);
        }

        #[test]
        fn it_returns_5c3h9djdkc2s9h_power_index_4494() {
            let made_hand: MadeHand = card_array!["5c", "3h", "9d", "Jd", "Kc", "2s", "9h"].into();

            assert_eq!(made_hand.power_index(), 4494);
        }

        #[test]
        fn it_returns_8sqsts4h5sqhac_power_index_3786() {
            let made_hand: MadeHand = card_array!["8s", "Qs", "Ts", "4h", "5s", "Qh", "Ac"].into();

            assert_eq!(made_hand.power_index(), 3786);
        }

        #[test]
        fn it_returns_4c2stc6h7cjh9s_power_index_7222() {
            let made_hand: MadeHand = card_array!["4c", "2s", "Tc", "6h", "7c", "Jh", "9s"].into();

            assert_eq!(made_hand.power_index(), 7222);
        }

        #[test]
        fn it_returns_qs2hks4sjh9sqc_power_index_3822() {
            let made_hand: MadeHand = card_array!["Qs", "2h", "Ks", "4s", "Jh", "9s", "Qc"].into();

            assert_eq!(made_hand.power_index(), 3822);
        }

        #[test]
        fn it_returns_jc2s9h6sqc7ctc_power_index_7008() {
            let made_hand: MadeHand = card_array!["Jc", "2s", "9h", "6s", "Qc", "7c", "Tc"].into();

            assert_eq!(made_hand.power_index(), 7008);
        }

        #[test]
        fn it_returns_8dkh8c9d9sjd7d_power_index_3019() {
            let made_hand: MadeHand = card_array!["8d", "Kh", "8c", "9d", "9s", "Jd", "7d"].into();

            assert_eq!(made_hand.power_index(), 3019);
        }

        #[test]
        fn it_returns_actc8d6d2saskc_power_index_3346() {
            let made_hand: MadeHand = card_array!["Ac", "Tc", "8d", "6d", "2s", "As", "Kc"].into();

            assert_eq!(made_hand.power_index(), 3346);
        }

        #[test]
        fn it_returns_8dthjh2dts4sac_power_index_4226() {
            let made_hand: MadeHand = card_array!["8d", "Th", "Jh", "2d", "Ts", "4s", "Ac"].into();

            assert_eq!(made_hand.power_index(), 4226);
        }

        #[test]
        fn it_returns_7djc2h8s4c9hjs_power_index_4150() {
            let made_hand: MadeHand = card_array!["7d", "Jc", "2h", "8s", "4c", "9h", "Js"].into();

            assert_eq!(made_hand.power_index(), 4150);
        }

        #[test]
        fn it_returns_qsjhkdtckhasjd_power_index_1600() {
            let made_hand: MadeHand = card_array!["Qs", "Jh", "Kd", "Tc", "Kh", "As", "Jd"].into();

            assert_eq!(made_hand.power_index(), 1600);
        }

        #[test]
        fn it_returns_7s7cjdas6s2ckd_power_index_4867() {
            let made_hand: MadeHand = card_array!["7s", "7c", "Jd", "As", "6s", "2c", "Kd"].into();

            assert_eq!(made_hand.power_index(), 4867);
        }

        #[test]
        fn it_returns_kd2c6sac2hjc7h_power_index_5967() {
            let made_hand: MadeHand = card_array!["Kd", "2c", "6s", "Ac", "2h", "Jc", "7h"].into();

            assert_eq!(made_hand.power_index(), 5967);
        }

        #[test]
        fn it_returns_qs2d4d9d7s6h3c_power_index_7163() {
            let made_hand: MadeHand = card_array!["Qs", "2d", "4d", "9d", "7s", "6h", "3c"].into();

            assert_eq!(made_hand.power_index(), 7163);
        }

        #[test]
        fn it_returns_as4cks4sqsjsqc_power_index_328() {
            let made_hand: MadeHand = card_array!["As", "4c", "Ks", "4s", "Qs", "Js", "Qc"].into();

            assert_eq!(made_hand.power_index(), 328);
        }

        #[test]
        fn it_returns_2sah5s8h4h7c9s_power_index_6611() {
            let made_hand: MadeHand = card_array!["2s", "Ah", "5s", "8h", "4h", "7c", "9s"].into();

            assert_eq!(made_hand.power_index(), 6611);
        }

        #[test]
        fn it_returns_7skd4cthqh6d5d_power_index_6727() {
            let made_hand: MadeHand = card_array!["7s", "Kd", "4c", "Th", "Qh", "6d", "5d"].into();

            assert_eq!(made_hand.power_index(), 6727);
        }

        #[test]
        fn it_returns_ac8d7cjh5s9h8h_power_index_4666() {
            let made_hand: MadeHand = card_array!["Ac", "8d", "7c", "Jh", "5s", "9h", "8h"].into();

            assert_eq!(made_hand.power_index(), 4666);
        }

        #[test]
        fn it_returns_3c4sjd4d8d7hkd_power_index_5592() {
            let made_hand: MadeHand = card_array!["3c", "4s", "Jd", "4d", "8d", "7h", "Kd"].into();

            assert_eq!(made_hand.power_index(), 5592);
        }

        #[test]
        fn it_returns_2s9cjc8c9hjs5d_power_index_2846() {
            let made_hand: MadeHand = card_array!["2s", "9c", "Jc", "8c", "9h", "Js", "5d"].into();

            assert_eq!(made_hand.power_index(), 2846);
        }

        #[test]
        fn it_returns_jstskc4h8sks6c_power_index_3647() {
            let made_hand: MadeHand = card_array!["Js", "Ts", "Kc", "4h", "8s", "Ks", "6c"].into();

            assert_eq!(made_hand.power_index(), 3647);
        }

        #[test]
        fn it_returns_thad2s6htdkh3s_power_index_4211() {
            let made_hand: MadeHand = card_array!["Th", "Ad", "2s", "6h", "Td", "Kh", "3s"].into();

            assert_eq!(made_hand.power_index(), 4211);
        }

        #[test]
        fn it_returns_2s8h8c3s9djdjh_power_index_2857() {
            let made_hand: MadeHand = card_array!["2s", "8h", "8c", "3s", "9d", "Jd", "Jh"].into();

            assert_eq!(made_hand.power_index(), 2857);
        }

        #[test]
        fn it_returns_6cqh2h7c8djh6h_power_index_5188() {
            let made_hand: MadeHand = card_array!["6c", "Qh", "2h", "7c", "8d", "Jh", "6h"].into();

            assert_eq!(made_hand.power_index(), 5188);
        }

        #[test]
        fn it_returns_7c8s8hjs5h7d8c_power_index_245() {
            let made_hand: MadeHand = card_array!["7c", "8s", "8h", "Js", "5h", "7d", "8c"].into();

            assert_eq!(made_hand.power_index(), 245);
        }

        #[test]
        fn it_returns_8h6ckd7dqs5h2c_power_index_6763() {
            let made_hand: MadeHand = card_array!["8h", "6c", "Kd", "7d", "Qs", "5h", "2c"].into();

            assert_eq!(made_hand.power_index(), 6763);
        }

        #[test]
        fn it_returns_qc2c9c9djd4s2s_power_index_3086() {
            let made_hand: MadeHand = card_array!["Qc", "2c", "9c", "9d", "Jd", "4s", "2s"].into();

            assert_eq!(made_hand.power_index(), 3086);
        }

        #[test]
        fn it_returns_2d4sjs6ststcqc_power_index_4309() {
            let made_hand: MadeHand = card_array!["2d", "4s", "Js", "6s", "Ts", "Tc", "Qc"].into();

            assert_eq!(made_hand.power_index(), 4309);
        }

        #[test]
        fn it_returns_3c4ckhjh3hackc_power_index_2699() {
            let made_hand: MadeHand = card_array!["3c", "4c", "Kh", "Jh", "3h", "Ac", "Kc"].into();

            assert_eq!(made_hand.power_index(), 2699);
        }

        #[test]
        fn it_returns_js8d7dtdacqsts_power_index_4216() {
            let made_hand: MadeHand = card_array!["Js", "8d", "7d", "Td", "Ac", "Qs", "Ts"].into();

            assert_eq!(made_hand.power_index(), 4216);
        }

        #[test]
        fn it_returns_5d3ckh6d9cqc4c_power_index_6753() {
            let made_hand: MadeHand = card_array!["5d", "3c", "Kh", "6d", "9c", "Qc", "4c"].into();

            assert_eq!(made_hand.power_index(), 6753);
        }

        #[test]
        fn it_returns_as6c6s9c3hjd8c_power_index_5106() {
            let made_hand: MadeHand = card_array!["As", "6c", "6s", "9c", "3h", "Jd", "8c"].into();

            assert_eq!(made_hand.power_index(), 5106);
        }

        #[test]
        fn it_returns_7cqc9cjc3hadtc_power_index_1145() {
            let made_hand: MadeHand = card_array!["7c", "Qc", "9c", "Jc", "3h", "Ad", "Tc"].into();

            assert_eq!(made_hand.power_index(), 1145);
        }

        #[test]
        fn it_returns_3c6ctd9d7s8dtc_power_index_1604() {
            let made_hand: MadeHand = card_array!["3c", "6c", "Td", "9d", "7s", "8d", "Tc"].into();

            assert_eq!(made_hand.power_index(), 1604);
        }

        #[test]
        fn it_returns_5sks7h6c4skd7c_power_index_2661() {
            let made_hand: MadeHand = card_array!["5s", "Ks", "7h", "6c", "4s", "Kd", "7c"].into();

            assert_eq!(made_hand.power_index(), 2661);
        }

        #[test]
        fn it_returns_3c4sjc7d4d2d6s_power_index_5680() {
            let made_hand: MadeHand = card_array!["3c", "4s", "Jc", "7d", "4d", "2d", "6s"].into();

            assert_eq!(made_hand.power_index(), 5680);
        }

        #[test]
        fn it_returns_th5d7s7h9s6s9c_power_index_3033() {
            let made_hand: MadeHand = card_array!["Th", "5d", "7s", "7h", "9s", "6s", "9c"].into();

            assert_eq!(made_hand.power_index(), 3033);
        }

        #[test]
        fn it_returns_tsth4hkc6d3sad_power_index_4211() {
            let made_hand: MadeHand = card_array!["Ts", "Th", "4h", "Kc", "6d", "3s", "Ad"].into();

            assert_eq!(made_hand.power_index(), 4211);
        }

        #[test]
        fn it_returns_5hac9s5sad4c6d_power_index_2560() {
            let made_hand: MadeHand = card_array!["5h", "Ac", "9s", "5s", "Ad", "4c", "6d"].into();

            assert_eq!(made_hand.power_index(), 2560);
        }

        #[test]
        fn it_returns_4cad7d2hkcac9d_power_index_3354() {
            let made_hand: MadeHand = card_array!["4c", "Ad", "7d", "2h", "Kc", "Ac", "9d"].into();

            assert_eq!(made_hand.power_index(), 3354);
        }

        #[test]
        fn it_returns_ks4s2d4hjs3sjc_power_index_2898() {
            let made_hand: MadeHand = card_array!["Ks", "4s", "2d", "4h", "Js", "3s", "Jc"].into();

            assert_eq!(made_hand.power_index(), 2898);
        }

        #[test]
        fn it_returns_jh4d6d8dkdjs7h_power_index_4065() {
            let made_hand: MadeHand = card_array!["Jh", "4d", "6d", "8d", "Kd", "Js", "7h"].into();

            assert_eq!(made_hand.power_index(), 4065);
        }

        #[test]
        fn it_returns_2h2c2d4ctdthqd_power_index_315() {
            let made_hand: MadeHand = card_array!["2h", "2c", "2d", "4c", "Td", "Th", "Qd"].into();

            assert_eq!(made_hand.power_index(), 315);
        }

        #[test]
        fn it_returns_asjhtd7d9dqcjd_power_index_3996() {
            let made_hand: MadeHand = card_array!["As", "Jh", "Td", "7d", "9d", "Qc", "Jd"].into();

            assert_eq!(made_hand.power_index(), 3996);
        }

        #[test]
        fn it_returns_qh7d3sjs3c4dac_power_index_5756() {
            let made_hand: MadeHand = card_array!["Qh", "7d", "3s", "Js", "3c", "4d", "Ac"].into();

            assert_eq!(made_hand.power_index(), 5756);
        }

        #[test]
        fn it_returns_4c7d5dkstdqd7s_power_index_4922() {
            let made_hand: MadeHand = card_array!["4c", "7d", "5d", "Ks", "Td", "Qd", "7s"].into();

            assert_eq!(made_hand.power_index(), 4922);
        }

        #[test]
        fn it_returns_kctcjs7ckh6s4c_power_index_3648() {
            let made_hand: MadeHand = card_array!["Kc", "Tc", "Js", "7c", "Kh", "6s", "4c"].into();

            assert_eq!(made_hand.power_index(), 3648);
        }

        #[test]
        fn it_returns_khahqhtc5dkc7s_power_index_3547() {
            let made_hand: MadeHand = card_array!["Kh", "Ah", "Qh", "Tc", "5d", "Kc", "7s"].into();

            assert_eq!(made_hand.power_index(), 3547);
        }

        #[test]
        fn it_returns_5c8skc2d4h5d2s_power_index_3283() {
            let made_hand: MadeHand = card_array!["5c", "8s", "Kc", "2d", "4h", "5d", "2s"].into();

            assert_eq!(made_hand.power_index(), 3283);
        }

        #[test]
        fn it_returns_6s6dkh4h4c4d2c_power_index_295() {
            let made_hand: MadeHand = card_array!["6s", "6d", "Kh", "4h", "4c", "4d", "2c"].into();

            assert_eq!(made_hand.power_index(), 295);
        }

        #[test]
        fn it_returns_9sth2c3dts4hjs_power_index_4346() {
            let made_hand: MadeHand = card_array!["9s", "Th", "2c", "3d", "Ts", "4h", "Js"].into();

            assert_eq!(made_hand.power_index(), 4346);
        }

        #[test]
        fn it_returns_qd3dks9skdad5d_power_index_362() {
            let made_hand: MadeHand = card_array!["Qd", "3d", "Ks", "9s", "Kd", "Ad", "5d"].into();

            assert_eq!(made_hand.power_index(), 362);
        }

        #[test]
        fn it_returns_5h7d8had5cas2s_power_index_2561() {
            let made_hand: MadeHand = card_array!["5h", "7d", "8h", "Ad", "5c", "As", "2s"].into();

            assert_eq!(made_hand.power_index(), 2561);
        }

        #[test]
        fn it_returns_3hacqh8s8c6cth_power_index_4657() {
            let made_hand: MadeHand = card_array!["3h", "Ac", "Qh", "8s", "8c", "6c", "Th"].into();

            assert_eq!(made_hand.power_index(), 4657);
        }

        #[test]
        fn it_returns_9dqsqd2c5h7s2d_power_index_2824() {
            let made_hand: MadeHand = card_array!["9d", "Qs", "Qd", "2c", "5h", "7s", "2d"].into();

            assert_eq!(made_hand.power_index(), 2824);
        }

        #[test]
        fn it_returns_ks8cjd9d8h5c6s_power_index_4711() {
            let made_hand: MadeHand = card_array!["Ks", "8c", "Jd", "9d", "8h", "5c", "6s"].into();

            assert_eq!(made_hand.power_index(), 4711);
        }

        #[test]
        fn it_returns_9d8s5s7sjs4h4s_power_index_1448() {
            let made_hand: MadeHand = card_array!["9d", "8s", "5s", "7s", "Js", "4h", "4s"].into();

            assert_eq!(made_hand.power_index(), 1448);
        }

        #[test]
        fn it_returns_9d8hahas5h4h3d_power_index_3492() {
            let made_hand: MadeHand = card_array!["9d", "8h", "Ah", "As", "5h", "4h", "3d"].into();

            assert_eq!(made_hand.power_index(), 3492);
        }

        #[test]
        fn it_returns_6c2cqcjd7s4h9s_power_index_7041() {
            let made_hand: MadeHand = card_array!["6c", "2c", "Qc", "Jd", "7s", "4h", "9s"].into();

            assert_eq!(made_hand.power_index(), 7041);
        }

        #[test]
        fn it_returns_tckd4sas9std4d_power_index_2985() {
            let made_hand: MadeHand = card_array!["Tc", "Kd", "4s", "As", "9s", "Td", "4d"].into();

            assert_eq!(made_hand.power_index(), 2985);
        }

        #[test]
        fn it_returns_js3cqh4h8s7dqd_power_index_3881() {
            let made_hand: MadeHand = card_array!["Js", "3c", "Qh", "4h", "8s", "7d", "Qd"].into();

            assert_eq!(made_hand.power_index(), 3881);
        }

        #[test]
        fn it_returns_kcjdtsjcac7hkd_power_index_2611() {
            let made_hand: MadeHand = card_array!["Kc", "Jd", "Ts", "Jc", "Ac", "7h", "Kd"].into();

            assert_eq!(made_hand.power_index(), 2611);
        }

        #[test]
        fn it_returns_td2sqcqd2ckd3h_power_index_2821() {
            let made_hand: MadeHand = card_array!["Td", "2s", "Qc", "Qd", "2c", "Kd", "3h"].into();

            assert_eq!(made_hand.power_index(), 2821);
        }

        #[test]
        fn it_returns_8s7h6htc7d2s3c_power_index_5036() {
            let made_hand: MadeHand = card_array!["8s", "7h", "6h", "Tc", "7d", "2s", "3c"].into();

            assert_eq!(made_hand.power_index(), 5036);
        }

        #[test]
        fn it_returns_8djd6c4hqd8h2h_power_index_4749() {
            let made_hand: MadeHand = card_array!["8d", "Jd", "6c", "4h", "Qd", "8h", "2h"].into();

            assert_eq!(made_hand.power_index(), 4749);
        }

        #[test]
        fn it_returns_3hqs2hjckckh5d_power_index_3606() {
            let made_hand: MadeHand = card_array!["3h", "Qs", "2h", "Jc", "Kc", "Kh", "5d"].into();

            assert_eq!(made_hand.power_index(), 3606);
        }

        #[test]
        fn it_returns_9c9h8c6s3c7s2s_power_index_4611() {
            let made_hand: MadeHand = card_array!["9c", "9h", "8c", "6s", "3c", "7s", "2s"].into();

            assert_eq!(made_hand.power_index(), 4611);
        }

        #[test]
        fn it_returns_qckhks9dth3d2h_power_index_3610() {
            let made_hand: MadeHand = card_array!["Qc", "Kh", "Ks", "9d", "Th", "3d", "2h"].into();

            assert_eq!(made_hand.power_index(), 3610);
        }

        #[test]
        fn it_returns_3s4cqs4htd3d5d_power_index_3295() {
            let made_hand: MadeHand = card_array!["3s", "4c", "Qs", "4h", "Td", "3d", "5d"].into();

            assert_eq!(made_hand.power_index(), 3295);
        }

        #[test]
        fn it_returns_5s6d6h8d5c9sqs_power_index_3218() {
            let made_hand: MadeHand = card_array!["5s", "6d", "6h", "8d", "5c", "9s", "Qs"].into();

            assert_eq!(made_hand.power_index(), 3218);
        }

        #[test]
        fn it_returns_acksjdqd4sjs9s_power_index_3986() {
            let made_hand: MadeHand = card_array!["Ac", "Ks", "Jd", "Qd", "4s", "Js", "9s"].into();

            assert_eq!(made_hand.power_index(), 3986);
        }

        #[test]
        fn it_returns_6cas3sqh8h3h9c_power_index_5758() {
            let made_hand: MadeHand = card_array!["6c", "As", "3s", "Qh", "8h", "3h", "9c"].into();

            assert_eq!(made_hand.power_index(), 5758);
        }

        #[test]
        fn it_returns_jcasth3ctc8s7h_power_index_4226() {
            let made_hand: MadeHand = card_array!["Jc", "As", "Th", "3c", "Tc", "8s", "7h"].into();

            assert_eq!(made_hand.power_index(), 4226);
        }

        #[test]
        fn it_returns_5djd7c5ckc9sjh_power_index_2887() {
            let made_hand: MadeHand = card_array!["5d", "Jd", "7c", "5c", "Kc", "9s", "Jh"].into();

            assert_eq!(made_hand.power_index(), 2887);
        }

        #[test]
        fn it_returns_7dkhjc5htd3s5d_power_index_5370() {
            let made_hand: MadeHand = card_array!["7d", "Kh", "Jc", "5h", "Td", "3s", "5d"].into();

            assert_eq!(made_hand.power_index(), 5370);
        }

        #[test]
        fn it_returns_3djs2c9hkc9d4c_power_index_4495() {
            let made_hand: MadeHand = card_array!["3d", "Js", "2c", "9h", "Kc", "9d", "4c"].into();

            assert_eq!(made_hand.power_index(), 4495);
        }

        #[test]
        fn it_returns_tc8sqh5h3sad9c_power_index_6386() {
            let made_hand: MadeHand = card_array!["Tc", "8s", "Qh", "5h", "3s", "Ad", "9c"].into();

            assert_eq!(made_hand.power_index(), 6386);
        }

        #[test]
        fn it_returns_jskc2d8std8cjd_power_index_2854() {
            let made_hand: MadeHand = card_array!["Js", "Kc", "2d", "8s", "Td", "8c", "Jd"].into();

            assert_eq!(made_hand.power_index(), 2854);
        }

        #[test]
        fn it_returns_6s8sad9h5sas6d_power_index_2549() {
            let made_hand: MadeHand = card_array!["6s", "8s", "Ad", "9h", "5s", "As", "6d"].into();

            assert_eq!(made_hand.power_index(), 2549);
        }

        #[test]
        fn it_returns_9d8d2c8h4hkc3s_power_index_4728() {
            let made_hand: MadeHand = card_array!["9d", "8d", "2c", "8h", "4h", "Kc", "3s"].into();

            assert_eq!(made_hand.power_index(), 4728);
        }

        #[test]
        fn it_returns_adjhqc7c4d5std_power_index_6352() {
            let made_hand: MadeHand = card_array!["Ad", "Jh", "Qc", "7c", "4d", "5s", "Td"].into();

            assert_eq!(made_hand.power_index(), 6352);
        }

        #[test]
        fn it_returns_qs9d8cjstdkd8s_power_index_1601() {
            let made_hand: MadeHand = card_array!["Qs", "9d", "8c", "Js", "Td", "Kd", "8s"].into();

            assert_eq!(made_hand.power_index(), 1601);
        }

        #[test]
        fn it_returns_qsjs8c9skdah8h_power_index_4646() {
            let made_hand: MadeHand = card_array!["Qs", "Js", "8c", "9s", "Kd", "Ah", "8h"].into();

            assert_eq!(made_hand.power_index(), 4646);
        }

        #[test]
        fn it_returns_9djd6skdthad3s_power_index_6230() {
            let made_hand: MadeHand = card_array!["9d", "Jd", "6s", "Kd", "Th", "Ad", "3s"].into();

            assert_eq!(made_hand.power_index(), 6230);
        }

        #[test]
        fn it_returns_kh3c6h2h7dqh9h_power_index_893() {
            let made_hand: MadeHand = card_array!["Kh", "3c", "6h", "2h", "7d", "Qh", "9h"].into();

            assert_eq!(made_hand.power_index(), 893);
        }

        #[test]
        fn it_returns_7sjd6dtd9d4c4h_power_index_5662() {
            let made_hand: MadeHand = card_array!["7s", "Jd", "6d", "Td", "9d", "4c", "4h"].into();

            assert_eq!(made_hand.power_index(), 5662);
        }

        #[test]
        fn it_returns_2d5stc4d8sad6c_power_index_6580() {
            let made_hand: MadeHand = card_array!["2d", "5s", "Tc", "4d", "8s", "Ad", "6c"].into();

            assert_eq!(made_hand.power_index(), 6580);
        }

        #[test]
        fn it_returns_th8h5cksjh7dtc_power_index_4271() {
            let made_hand: MadeHand = card_array!["Th", "8h", "5c", "Ks", "Jh", "7d", "Tc"].into();

            assert_eq!(made_hand.power_index(), 4271);
        }

        #[test]
        fn it_returns_ksjs6cjh7s4dts_power_index_4052() {
            let made_hand: MadeHand = card_array!["Ks", "Js", "6c", "Jh", "7s", "4d", "Ts"].into();

            assert_eq!(made_hand.power_index(), 4052);
        }

        #[test]
        fn it_returns_jsksjdad4s2c2h_power_index_2919() {
            let made_hand: MadeHand = card_array!["Js", "Ks", "Jd", "Ad", "4s", "2c", "2h"].into();

            assert_eq!(made_hand.power_index(), 2919);
        }

        #[test]
        fn it_returns_5h5sqs8d6d7s3d_power_index_5427() {
            let made_hand: MadeHand = card_array!["5h", "5s", "Qs", "8d", "6d", "7s", "3d"].into();

            assert_eq!(made_hand.power_index(), 5427);
        }

        #[test]
        fn it_returns_ksqhqc8d2d9c2s_power_index_2821() {
            let made_hand: MadeHand = card_array!["Ks", "Qh", "Qc", "8d", "2d", "9c", "2s"].into();

            assert_eq!(made_hand.power_index(), 2821);
        }

        #[test]
        fn it_returns_acjs9c6hthtsqh_power_index_4216() {
            let made_hand: MadeHand = card_array!["Ac", "Js", "9c", "6h", "Th", "Ts", "Qh"].into();

            assert_eq!(made_hand.power_index(), 4216);
        }

        #[test]
        fn it_returns_2h6s4c3c4dkh9d_power_index_5607() {
            let made_hand: MadeHand = card_array!["2h", "6s", "4c", "3c", "4d", "Kh", "9d"].into();

            assert_eq!(made_hand.power_index(), 5607);
        }

        #[test]
        fn it_returns_6c8c2ckhkctc7h_power_index_1048() {
            let made_hand: MadeHand = card_array!["6c", "8c", "2c", "Kh", "Kc", "Tc", "7h"].into();

            assert_eq!(made_hand.power_index(), 1048);
        }

        #[test]
        fn it_returns_td3d6d4dqdqc8d_power_index_1255() {
            let made_hand: MadeHand = card_array!["Td", "3d", "6d", "4d", "Qd", "Qc", "8d"].into();

            assert_eq!(made_hand.power_index(), 1255);
        }

        #[test]
        fn it_returns_2cts7d7h3s6c4c_power_index_5042() {
            let made_hand: MadeHand = card_array!["2c", "Ts", "7d", "7h", "3s", "6c", "4c"].into();

            assert_eq!(made_hand.power_index(), 5042);
        }

        #[test]
        fn it_returns_7h8dahqc3cjs4d_power_index_6365() {
            let made_hand: MadeHand = card_array!["7h", "8d", "Ah", "Qc", "3c", "Js", "4d"].into();

            assert_eq!(made_hand.power_index(), 6365);
        }

        #[test]
        fn it_returns_ac4h8h5s9sqdks_power_index_6202() {
            let made_hand: MadeHand = card_array!["Ac", "4h", "8h", "5s", "9s", "Qd", "Ks"].into();

            assert_eq!(made_hand.power_index(), 6202);
        }

        #[test]
        fn it_returns_ahjhkc8dts4skh_power_index_3556() {
            let made_hand: MadeHand = card_array!["Ah", "Jh", "Kc", "8d", "Ts", "4s", "Kh"].into();

            assert_eq!(made_hand.power_index(), 3556);
        }

        #[test]
        fn it_returns_8sks2d7d3c8h7c_power_index_3096() {
            let made_hand: MadeHand = card_array!["8s", "Ks", "2d", "7d", "3c", "8h", "7c"].into();

            assert_eq!(made_hand.power_index(), 3096);
        }

        #[test]
        fn it_returns_tc4dtsqh4c7has_power_index_2985() {
            let made_hand: MadeHand = card_array!["Tc", "4d", "Ts", "Qh", "4c", "7h", "As"].into();

            assert_eq!(made_hand.power_index(), 2985);
        }

        #[test]
        fn it_returns_qs9h6skc7c4h2s_power_index_6748() {
            let made_hand: MadeHand = card_array!["Qs", "9h", "6s", "Kc", "7c", "4h", "2s"].into();

            assert_eq!(made_hand.power_index(), 6748);
        }

        #[test]
        fn it_returns_jh8c3cqd5d9had_power_index_6358() {
            let made_hand: MadeHand = card_array!["Jh", "8c", "3c", "Qd", "5d", "9h", "Ad"].into();

            assert_eq!(made_hand.power_index(), 6358);
        }

        #[test]
        fn it_returns_2htd7hah7dackd_power_index_2534() {
            let made_hand: MadeHand = card_array!["2h", "Td", "7h", "Ah", "7d", "Ac", "Kd"].into();

            assert_eq!(made_hand.power_index(), 2534);
        }

        #[test]
        fn it_returns_jckc5h9cqhjdjh_power_index_1819() {
            let made_hand: MadeHand = card_array!["Jc", "Kc", "5h", "9c", "Qh", "Jd", "Jh"].into();

            assert_eq!(made_hand.power_index(), 1819);
        }

        #[test]
        fn it_returns_6d9s9das3d8s5d_power_index_4461() {
            let made_hand: MadeHand = card_array!["6d", "9s", "9d", "As", "3d", "8s", "5d"].into();

            assert_eq!(made_hand.power_index(), 4461);
        }

        #[test]
        fn it_returns_8h5s9s7skh2d3d_power_index_6939() {
            let made_hand: MadeHand = card_array!["8h", "5s", "9s", "7s", "Kh", "2d", "3d"].into();

            assert_eq!(made_hand.power_index(), 6939);
        }

        #[test]
        fn it_returns_2h3c8s9d9c8cjs_power_index_3021() {
            let made_hand: MadeHand = card_array!["2h", "3c", "8s", "9d", "9c", "8c", "Js"].into();

            assert_eq!(made_hand.power_index(), 3021);
        }

        #[test]
        fn it_returns_9c2cqd4s3ctc2d_power_index_6074() {
            let made_hand: MadeHand = card_array!["9c", "2c", "Qd", "4s", "3c", "Tc", "2d"].into();

            assert_eq!(made_hand.power_index(), 6074);
        }

        #[test]
        fn it_returns_qd6cjd5cks2c9h_power_index_6688() {
            let made_hand: MadeHand = card_array!["Qd", "6c", "Jd", "5c", "Ks", "2c", "9h"].into();

            assert_eq!(made_hand.power_index(), 6688);
        }

        #[test]
        fn it_returns_6d2d6s7h3h9s8c_power_index_5271() {
            let made_hand: MadeHand = card_array!["6d", "2d", "6s", "7h", "3h", "9s", "8c"].into();

            assert_eq!(made_hand.power_index(), 5271);
        }

        #[test]
        fn it_returns_6c8h5h9h2h6d8s_power_index_3111() {
            let made_hand: MadeHand = card_array!["6c", "8h", "5h", "9h", "2h", "6d", "8s"].into();

            assert_eq!(made_hand.power_index(), 3111);
        }

        #[test]
        fn it_returns_3d6c8h2h3h6s4d_power_index_3244() {
            let made_hand: MadeHand = card_array!["3d", "6c", "8h", "2h", "3h", "6s", "4d"].into();

            assert_eq!(made_hand.power_index(), 3244);
        }

        #[test]
        fn it_returns_tc6s5h5dqc2c8c_power_index_5415() {
            let made_hand: MadeHand = card_array!["Tc", "6s", "5h", "5d", "Qc", "2c", "8c"].into();

            assert_eq!(made_hand.power_index(), 5415);
        }

        #[test]
        fn it_returns_th9d2hqcks3s4s_power_index_6718() {
            let made_hand: MadeHand = card_array!["Th", "9d", "2h", "Qc", "Ks", "3s", "4s"].into();

            assert_eq!(made_hand.power_index(), 6718);
        }

        #[test]
        fn it_returns_th5h8cackd5dtd_power_index_2974() {
            let made_hand: MadeHand = card_array!["Th", "5h", "8c", "Ac", "Kd", "5d", "Td"].into();

            assert_eq!(made_hand.power_index(), 2974);
        }

        #[test]
        fn it_returns_4sasjd9s8s7h7s_power_index_749() {
            let made_hand: MadeHand = card_array!["4s", "As", "Jd", "9s", "8s", "7h", "7s"].into();

            assert_eq!(made_hand.power_index(), 749);
        }

        #[test]
        fn it_returns_3c8s5cksas8h7c_power_index_4650() {
            let made_hand: MadeHand = card_array!["3c", "8s", "5c", "Ks", "As", "8h", "7c"].into();

            assert_eq!(made_hand.power_index(), 4650);
        }

        #[test]
        fn it_returns_7s8s7c3c2s7dqs_power_index_2096() {
            let made_hand: MadeHand = card_array!["7s", "8s", "7c", "3c", "2s", "7d", "Qs"].into();

            assert_eq!(made_hand.power_index(), 2096);
        }

        #[test]
        fn it_returns_8ckh5s7d4d3sah_power_index_6316() {
            let made_hand: MadeHand = card_array!["8c", "Kh", "5s", "7d", "4d", "3s", "Ah"].into();

            assert_eq!(made_hand.power_index(), 6316);
        }

        #[test]
        fn it_returns_4c8h5s7cjh4dqc_power_index_5628() {
            let made_hand: MadeHand = card_array!["4c", "8h", "5s", "7c", "Jh", "4d", "Qc"].into();

            assert_eq!(made_hand.power_index(), 5628);
        }

        #[test]
        fn it_returns_3cjsqhtsad5cqd_power_index_3776() {
            let made_hand: MadeHand = card_array!["3c", "Js", "Qh", "Ts", "Ad", "5c", "Qd"].into();

            assert_eq!(made_hand.power_index(), 3776);
        }

        #[test]
        fn it_returns_jh4h9sqh7s8dqc_power_index_3874() {
            let made_hand: MadeHand = card_array!["Jh", "4h", "9s", "Qh", "7s", "8d", "Qc"].into();

            assert_eq!(made_hand.power_index(), 3874);
        }

        #[test]
        fn it_returns_ah7c2hts6c9d9s_power_index_4454() {
            let made_hand: MadeHand = card_array!["Ah", "7c", "2h", "Ts", "6c", "9d", "9s"].into();

            assert_eq!(made_hand.power_index(), 4454);
        }

        #[test]
        fn it_returns_kcjs5h3c9s9dad_power_index_4427() {
            let made_hand: MadeHand = card_array!["Kc", "Js", "5h", "3c", "9s", "9d", "Ad"].into();

            assert_eq!(made_hand.power_index(), 4427);
        }

        #[test]
        fn it_returns_7h2h5c8s7c6c3h_power_index_5066() {
            let made_hand: MadeHand = card_array!["7h", "2h", "5c", "8s", "7c", "6c", "3h"].into();

            assert_eq!(made_hand.power_index(), 5066);
        }

        #[test]
        fn it_returns_asqh3hkdahjhqs_power_index_2479() {
            let made_hand: MadeHand = card_array!["As", "Qh", "3h", "Kd", "Ah", "Jh", "Qs"].into();

            assert_eq!(made_hand.power_index(), 2479);
        }

        #[test]
        fn it_returns_3d5h9sad2h4s8s_power_index_1609() {
            let made_hand: MadeHand = card_array!["3d", "5h", "9s", "Ad", "2h", "4s", "8s"].into();

            assert_eq!(made_hand.power_index(), 1609);
        }

        #[test]
        fn it_returns_ackh4htcqhqcts_power_index_2732() {
            let made_hand: MadeHand = card_array!["Ac", "Kh", "4h", "Tc", "Qh", "Qc", "Ts"].into();

            assert_eq!(made_hand.power_index(), 2732);
        }

        #[test]
        fn it_returns_5h7sjs4c5ckd7h_power_index_3173() {
            let made_hand: MadeHand = card_array!["5h", "7s", "Js", "4c", "5c", "Kd", "7h"].into();

            assert_eq!(made_hand.power_index(), 3173);
        }

        #[test]
        fn it_returns_kc6c8h5c8s2c5d_power_index_3118() {
            let made_hand: MadeHand = card_array!["Kc", "6c", "8h", "5c", "8s", "2c", "5d"].into();

            assert_eq!(made_hand.power_index(), 3118);
        }

        #[test]
        fn it_returns_7d7h9h6d3htdth_power_index_2956() {
            let made_hand: MadeHand = card_array!["7d", "7h", "9h", "6d", "3h", "Td", "Th"].into();

            assert_eq!(made_hand.power_index(), 2956);
        }

        #[test]
        fn it_returns_5sjsqsts5dkhkc_power_index_2678() {
            let made_hand: MadeHand = card_array!["5s", "Js", "Qs", "Ts", "5d", "Kh", "Kc"].into();

            assert_eq!(made_hand.power_index(), 2678);
        }

        #[test]
        fn it_returns_kc2s9hqh6dkhtd_power_index_3610() {
            let made_hand: MadeHand = card_array!["Kc", "2s", "9h", "Qh", "6d", "Kh", "Td"].into();

            assert_eq!(made_hand.power_index(), 3610);
        }

        #[test]
        fn it_returns_6h7c9c5c9sqs4h_power_index_4547() {
            let made_hand: MadeHand = card_array!["6h", "7c", "9c", "5c", "9s", "Qs", "4h"].into();

            assert_eq!(made_hand.power_index(), 4547);
        }

        #[test]
        fn it_returns_8hkc8s5c4s8c3s_power_index_2023() {
            let made_hand: MadeHand = card_array!["8h", "Kc", "8s", "5c", "4s", "8c", "3s"].into();

            assert_eq!(made_hand.power_index(), 2023);
        }

        #[test]
        fn it_returns_5d6cacjcjh4s4h_power_index_2897() {
            let made_hand: MadeHand = card_array!["5d", "6c", "Ac", "Jc", "Jh", "4s", "4h"].into();

            assert_eq!(made_hand.power_index(), 2897);
        }

        #[test]
        fn it_returns_ac6dqdqs2d5skd_power_index_3771() {
            let made_hand: MadeHand = card_array!["Ac", "6d", "Qd", "Qs", "2d", "5s", "Kd"].into();

            assert_eq!(made_hand.power_index(), 3771);
        }

        #[test]
        fn it_returns_kcac5s3s6s4s3d_power_index_5752() {
            let made_hand: MadeHand = card_array!["Kc", "Ac", "5s", "3s", "6s", "4s", "3d"].into();

            assert_eq!(made_hand.power_index(), 5752);
        }

        #[test]
        fn it_returns_ts3djc3sjh4dkc_power_index_2909() {
            let made_hand: MadeHand = card_array!["Ts", "3d", "Jc", "3s", "Jh", "4d", "Kc"].into();

            assert_eq!(made_hand.power_index(), 2909);
        }

        #[test]
        fn it_returns_ks6cjd9c2d7hjs_power_index_4059() {
            let made_hand: MadeHand = card_array!["Ks", "6c", "Jd", "9c", "2d", "7h", "Js"].into();

            assert_eq!(made_hand.power_index(), 4059);
        }

        #[test]
        fn it_returns_tdjdah6h2s9sts_power_index_4225() {
            let made_hand: MadeHand = card_array!["Td", "Jd", "Ah", "6h", "2s", "9s", "Ts"].into();

            assert_eq!(made_hand.power_index(), 4225);
        }

        #[test]
        fn it_returns_5h2cjc7h2h6dtc_power_index_6104() {
            let made_hand: MadeHand = card_array!["5h", "2c", "Jc", "7h", "2h", "6d", "Tc"].into();

            assert_eq!(made_hand.power_index(), 6104);
        }

        #[test]
        fn it_returns_4d2h3cth4sqc2s_power_index_3306() {
            let made_hand: MadeHand = card_array!["4d", "2h", "3c", "Th", "4s", "Qc", "2s"].into();

            assert_eq!(made_hand.power_index(), 3306);
        }

        #[test]
        fn it_returns_ad9d4s8std2c8c_power_index_4673() {
            let made_hand: MadeHand = card_array!["Ad", "9d", "4s", "8s", "Td", "2c", "8c"].into();

            assert_eq!(made_hand.power_index(), 4673);
        }

        #[test]
        fn it_returns_9stsjhjd9c4s3c_power_index_2845() {
            let made_hand: MadeHand = card_array!["9s", "Ts", "Jh", "Jd", "9c", "4s", "3c"].into();

            assert_eq!(made_hand.power_index(), 2845);
        }

        #[test]
        fn it_returns_8htcqd7s7hjdad_power_index_4876() {
            let made_hand: MadeHand = card_array!["8h", "Tc", "Qd", "7s", "7h", "Jd", "Ad"].into();

            assert_eq!(made_hand.power_index(), 4876);
        }

        #[test]
        fn it_returns_9h9c8h4sjckc9s_power_index_1952() {
            let made_hand: MadeHand = card_array!["9h", "9c", "8h", "4s", "Jc", "Kc", "9s"].into();

            assert_eq!(made_hand.power_index(), 1952);
        }

        #[test]
        fn it_returns_tc8sqskh5s9c6d_power_index_6714() {
            let made_hand: MadeHand = card_array!["Tc", "8s", "Qs", "Kh", "5s", "9c", "6d"].into();

            assert_eq!(made_hand.power_index(), 6714);
        }

        #[test]
        fn it_returns_3das4d9h8stsjc_power_index_6470() {
            let made_hand: MadeHand = card_array!["3d", "As", "4d", "9h", "8s", "Ts", "Jc"].into();

            assert_eq!(made_hand.power_index(), 6470);
        }

        #[test]
        fn it_returns_4h7d2s3dacks9c_power_index_6302() {
            let made_hand: MadeHand = card_array!["4h", "7d", "2s", "3d", "Ac", "Ks", "9c"].into();

            assert_eq!(made_hand.power_index(), 6302);
        }

        #[test]
        fn it_returns_8h9h9d4h5sah6h_power_index_753() {
            let made_hand: MadeHand = card_array!["8h", "9h", "9d", "4h", "5s", "Ah", "6h"].into();

            assert_eq!(made_hand.power_index(), 753);
        }

        #[test]
        fn it_returns_8hahqcth6sqh7d_power_index_3786() {
            let made_hand: MadeHand = card_array!["8h", "Ah", "Qc", "Th", "6s", "Qh", "7d"].into();

            assert_eq!(made_hand.power_index(), 3786);
        }

        #[test]
        fn it_returns_5c3hqcjs5djc7s_power_index_2888() {
            let made_hand: MadeHand = card_array!["5c", "3h", "Qc", "Js", "5d", "Jc", "7s"].into();

            assert_eq!(made_hand.power_index(), 2888);
        }

        #[test]
        fn it_returns_9sthksac6c5djc_power_index_6230() {
            let made_hand: MadeHand = card_array!["9s", "Th", "Ks", "Ac", "6c", "5d", "Jc"].into();

            assert_eq!(made_hand.power_index(), 6230);
        }

        #[test]
        fn it_returns_6c4d9s6d7d3sjd_power_index_5230() {
            let made_hand: MadeHand = card_array!["6c", "4d", "9s", "6d", "7d", "3s", "Jd"].into();

            assert_eq!(made_hand.power_index(), 5230);
        }

        #[test]
        fn it_returns_ad3dac2dqs3s4s_power_index_2579() {
            let made_hand: MadeHand = card_array!["Ad", "3d", "Ac", "2d", "Qs", "3s", "4s"].into();

            assert_eq!(made_hand.power_index(), 2579);
        }

        #[test]
        fn it_returns_3d3ctdtcjc4cjd_power_index_2839() {
            let made_hand: MadeHand = card_array!["3d", "3c", "Td", "Tc", "Jc", "4c", "Jd"].into();

            assert_eq!(made_hand.power_index(), 2839);
        }

        #[test]
        fn it_returns_7s8hjstsas2s2d_power_index_624() {
            let made_hand: MadeHand = card_array!["7s", "8h", "Js", "Ts", "As", "2s", "2d"].into();

            assert_eq!(made_hand.power_index(), 624);
        }

        #[test]
        fn it_returns_6hjhjd3s6d5d2c_power_index_2882() {
            let made_hand: MadeHand = card_array!["6h", "Jh", "Jd", "3s", "6d", "5d", "2c"].into();

            assert_eq!(made_hand.power_index(), 2882);
        }

        #[test]
        fn it_returns_4sjc8stc6cqcks_power_index_6679() {
            let made_hand: MadeHand = card_array!["4s", "Jc", "8s", "Tc", "6c", "Qc", "Ks"].into();

            assert_eq!(made_hand.power_index(), 6679);
        }

        #[test]
        fn it_returns_khtd7sqhtsjh5h_power_index_4261() {
            let made_hand: MadeHand = card_array!["Kh", "Td", "7s", "Qh", "Ts", "Jh", "5h"].into();

            assert_eq!(made_hand.power_index(), 4261);
        }

        #[test]
        fn it_returns_9sjc2hksad9d6h_power_index_4427() {
            let made_hand: MadeHand = card_array!["9s", "Jc", "2h", "Ks", "Ad", "9d", "6h"].into();

            assert_eq!(made_hand.power_index(), 4427);
        }

        #[test]
        fn it_returns_3h2std7d5c5h9c_power_index_5471() {
            let made_hand: MadeHand = card_array!["3h", "2s", "Td", "7d", "5c", "5h", "9c"].into();

            assert_eq!(made_hand.power_index(), 5471);
        }

        #[test]
        fn it_returns_7ckh2d6c3cahkc_power_index_3586() {
            let made_hand: MadeHand = card_array!["7c", "Kh", "2d", "6c", "3c", "Ah", "Kc"].into();

            assert_eq!(made_hand.power_index(), 3586);
        }

        #[test]
        fn it_returns_4s6djd8cahqd2d_power_index_6366() {
            let made_hand: MadeHand = card_array!["4s", "6d", "Jd", "8c", "Ah", "Qd", "2d"].into();

            assert_eq!(made_hand.power_index(), 6366);
        }

        #[test]
        fn it_returns_7dkhthas4hacks_power_index_2470() {
            let made_hand: MadeHand = card_array!["7d", "Kh", "Th", "As", "4h", "Ac", "Ks"].into();

            assert_eq!(made_hand.power_index(), 2470);
        }

        #[test]
        fn it_returns_9ctc2sadqh6c5h_power_index_6388() {
            let made_hand: MadeHand = card_array!["9c", "Tc", "2s", "Ad", "Qh", "6c", "5h"].into();

            assert_eq!(made_hand.power_index(), 6388);
        }

        #[test]
        fn it_returns_jcjhadqd4h7cjd_power_index_1809() {
            let made_hand: MadeHand = card_array!["Jc", "Jh", "Ad", "Qd", "4h", "7c", "Jd"].into();

            assert_eq!(made_hand.power_index(), 1809);
        }

        #[test]
        fn it_returns_9s7stsjs6c5sth_power_index_1360() {
            let made_hand: MadeHand = card_array!["9s", "7s", "Ts", "Js", "6c", "5s", "Th"].into();

            assert_eq!(made_hand.power_index(), 1360);
        }

        #[test]
        fn it_returns_4d3htcacah9s5c_power_index_3465() {
            let made_hand: MadeHand = card_array!["4d", "3h", "Tc", "Ac", "Ah", "9s", "5c"].into();

            assert_eq!(made_hand.power_index(), 3465);
        }

        #[test]
        fn it_returns_tc3s8d2dtskskc_power_index_2626() {
            let made_hand: MadeHand = card_array!["Tc", "3s", "8d", "2d", "Ts", "Ks", "Kc"].into();

            assert_eq!(made_hand.power_index(), 2626);
        }

        #[test]
        fn it_returns_ah7s4cqh6had2h_power_index_3411() {
            let made_hand: MadeHand = card_array!["Ah", "7s", "4c", "Qh", "6h", "Ad", "2h"].into();

            assert_eq!(made_hand.power_index(), 3411);
        }

        #[test]
        fn it_returns_khtd7s5stc9cjd_power_index_4270() {
            let made_hand: MadeHand = card_array!["Kh", "Td", "7s", "5s", "Tc", "9c", "Jd"].into();

            assert_eq!(made_hand.power_index(), 4270);
        }

        #[test]
        fn it_returns_3sjs7c4h2h7hts_power_index_5006() {
            let made_hand: MadeHand = card_array!["3s", "Js", "7c", "4h", "2h", "7h", "Ts"].into();

            assert_eq!(made_hand.power_index(), 5006);
        }

        #[test]
        fn it_returns_9d9cqcqhtd9hkd_power_index_229() {
            let made_hand: MadeHand = card_array!["9d", "9c", "Qc", "Qh", "Td", "9h", "Kd"].into();

            assert_eq!(made_hand.power_index(), 229);
        }

        #[test]
        fn it_returns_5c5djd7h5sahts_power_index_2206() {
            let made_hand: MadeHand = card_array!["5c", "5d", "Jd", "7h", "5s", "Ah", "Ts"].into();

            assert_eq!(made_hand.power_index(), 2206);
        }

        #[test]
        fn it_returns_qhqd4sadtd4c3c_power_index_2798() {
            let made_hand: MadeHand = card_array!["Qh", "Qd", "4s", "Ad", "Td", "4c", "3c"].into();

            assert_eq!(made_hand.power_index(), 2798);
        }

        #[test]
        fn it_returns_7h3c3d5cjckhts_power_index_5810() {
            let made_hand: MadeHand = card_array!["7h", "3c", "3d", "5c", "Jc", "Kh", "Ts"].into();

            assert_eq!(made_hand.power_index(), 5810);
        }

        #[test]
        fn it_returns_4c4dkhqs9h6ckc_power_index_2689() {
            let made_hand: MadeHand = card_array!["4c", "4d", "Kh", "Qs", "9h", "6c", "Kc"].into();

            assert_eq!(made_hand.power_index(), 2689);
        }

        #[test]
        fn it_returns_td4d8sjdks2sqs_power_index_6679() {
            let made_hand: MadeHand = card_array!["Td", "4d", "8s", "Jd", "Ks", "2s", "Qs"].into();

            assert_eq!(made_hand.power_index(), 6679);
        }

        #[test]
        fn it_returns_2djc5cjh9hacqh_power_index_3997() {
            let made_hand: MadeHand = card_array!["2d", "Jc", "5c", "Jh", "9h", "Ac", "Qh"].into();

            assert_eq!(made_hand.power_index(), 3997);
        }

        #[test]
        fn it_returns_6hqd9d2dks3dkh_power_index_3620() {
            let made_hand: MadeHand = card_array!["6h", "Qd", "9d", "2d", "Ks", "3d", "Kh"].into();

            assert_eq!(made_hand.power_index(), 3620);
        }

        #[test]
        fn it_returns_9s3s6sjcjsadac_power_index_2493() {
            let made_hand: MadeHand = card_array!["9s", "3s", "6s", "Jc", "Js", "Ad", "Ac"].into();

            assert_eq!(made_hand.power_index(), 2493);
        }

        #[test]
        fn it_returns_qstc8d5h6s6dqd_power_index_2779() {
            let made_hand: MadeHand = card_array!["Qs", "Tc", "8d", "5h", "6s", "6d", "Qd"].into();

            assert_eq!(made_hand.power_index(), 2779);
        }

        #[test]
        fn it_returns_jd6c7c7hjcjskh_power_index_209() {
            let made_hand: MadeHand = card_array!["Jd", "6c", "7c", "7h", "Jc", "Js", "Kh"].into();

            assert_eq!(made_hand.power_index(), 209);
        }

        #[test]
        fn it_returns_kcjhqs4sadjsqd_power_index_2721() {
            let made_hand: MadeHand = card_array!["Kc", "Jh", "Qs", "4s", "Ad", "Js", "Qd"].into();

            assert_eq!(made_hand.power_index(), 2721);
        }

        #[test]
        fn it_returns_asqc9h8d9c2ctd_power_index_4437() {
            let made_hand: MadeHand = card_array!["As", "Qc", "9h", "8d", "9c", "2c", "Td"].into();

            assert_eq!(made_hand.power_index(), 4437);
        }

        #[test]
        fn it_returns_8hkd3hts4d6hjd_power_index_6806() {
            let made_hand: MadeHand = card_array!["8h", "Kd", "3h", "Ts", "4d", "6h", "Jd"].into();

            assert_eq!(made_hand.power_index(), 6806);
        }

        #[test]
        fn it_returns_jdtd4c7dkcah2h_power_index_6232() {
            let made_hand: MadeHand = card_array!["Jd", "Td", "4c", "7d", "Kc", "Ah", "2h"].into();

            assert_eq!(made_hand.power_index(), 6232);
        }

        #[test]
        fn it_returns_2h6hthkc6s8sqd_power_index_5142() {
            let made_hand: MadeHand = card_array!["2h", "6h", "Th", "Kc", "6s", "8s", "Qd"].into();

            assert_eq!(made_hand.power_index(), 5142);
        }

        #[test]
        fn it_returns_5s8d8s7h9sqctd_power_index_4754() {
            let made_hand: MadeHand = card_array!["5s", "8d", "8s", "7h", "9s", "Qc", "Td"].into();

            assert_eq!(made_hand.power_index(), 4754);
        }

        #[test]
        fn it_returns_7c6h4hqc2dtctd_power_index_4327() {
            let made_hand: MadeHand = card_array!["7c", "6h", "4h", "Qc", "2d", "Tc", "Td"].into();

            assert_eq!(made_hand.power_index(), 4327);
        }

        #[test]
        fn it_returns_qhkh6h2c3dqs6s_power_index_2777() {
            let made_hand: MadeHand = card_array!["Qh", "Kh", "6h", "2c", "3d", "Qs", "6s"].into();

            assert_eq!(made_hand.power_index(), 2777);
        }

        #[test]
        fn it_returns_8ctdjhkd3s2cas_power_index_6231() {
            let made_hand: MadeHand = card_array!["8c", "Td", "Jh", "Kd", "3s", "2c", "As"].into();

            assert_eq!(made_hand.power_index(), 6231);
        }

        #[test]
        fn it_returns_th6h2sks2hqsjc_power_index_6021() {
            let made_hand: MadeHand = card_array!["Th", "6h", "2s", "Ks", "2h", "Qs", "Jc"].into();

            assert_eq!(made_hand.power_index(), 6021);
        }

        #[test]
        fn it_returns_8h6s9dkhjh2h5d_power_index_6827() {
            let made_hand: MadeHand = card_array!["8h", "6s", "9d", "Kh", "Jh", "2h", "5d"].into();

            assert_eq!(made_hand.power_index(), 6827);
        }

        #[test]
        fn it_returns_4h7dqcjh4s8s6s_power_index_5628() {
            let made_hand: MadeHand = card_array!["4h", "7d", "Qc", "Jh", "4s", "8s", "6s"].into();

            assert_eq!(made_hand.power_index(), 5628);
        }

        #[test]
        fn it_returns_3cad9hkh5h4h4c_power_index_5529() {
            let made_hand: MadeHand = card_array!["3c", "Ad", "9h", "Kh", "5h", "4h", "4c"].into();

            assert_eq!(made_hand.power_index(), 5529);
        }

        #[test]
        fn it_returns_jd8h8c3c6d8s5s_power_index_2039() {
            let made_hand: MadeHand = card_array!["Jd", "8h", "8c", "3c", "6d", "8s", "5s"].into();

            assert_eq!(made_hand.power_index(), 2039);
        }

        #[test]
        fn it_returns_6d4c8d8h4d2djd_power_index_1458() {
            let made_hand: MadeHand = card_array!["6d", "4c", "8d", "8h", "4d", "2d", "Jd"].into();

            assert_eq!(made_hand.power_index(), 1458);
        }

        #[test]
        fn it_returns_9c6c9saskh8hth_power_index_4428() {
            let made_hand: MadeHand = card_array!["9c", "6c", "9s", "As", "Kh", "8h", "Th"].into();

            assert_eq!(made_hand.power_index(), 4428);
        }

        #[test]
        fn it_returns_4c6d4s8s5d4hqh_power_index_2294() {
            let made_hand: MadeHand = card_array!["4c", "6d", "4s", "8s", "5d", "4h", "Qh"].into();

            assert_eq!(made_hand.power_index(), 2294);
        }

        #[test]
        fn it_returns_jh4s4h6hah4cth_power_index_626() {
            let made_hand: MadeHand = card_array!["Jh", "4s", "4h", "6h", "Ah", "4c", "Th"].into();

            assert_eq!(made_hand.power_index(), 626);
        }

        #[test]
        fn it_returns_thks8h5d3c3s8c_power_index_3140() {
            let made_hand: MadeHand = card_array!["Th", "Ks", "8h", "5d", "3c", "3s", "8c"].into();

            assert_eq!(made_hand.power_index(), 3140);
        }

        #[test]
        fn it_returns_ks7djs2dtcjc4d_power_index_4052() {
            let made_hand: MadeHand = card_array!["Ks", "7d", "Js", "2d", "Tc", "Jc", "4d"].into();

            assert_eq!(made_hand.power_index(), 4052);
        }

        #[test]
        fn it_returns_2h7s2c9c6d8hjh_power_index_6109() {
            let made_hand: MadeHand = card_array!["2h", "7s", "2c", "9c", "6d", "8h", "Jh"].into();

            assert_eq!(made_hand.power_index(), 6109);
        }

        #[test]
        fn it_returns_5s5d8c6ckd8skh_power_index_2650() {
            let made_hand: MadeHand = card_array!["5s", "5d", "8c", "6c", "Kd", "8s", "Kh"].into();

            assert_eq!(made_hand.power_index(), 2650);
        }

        #[test]
        fn it_returns_8s3cksadas4dac_power_index_1614() {
            let made_hand: MadeHand = card_array!["8s", "3c", "Ks", "Ad", "As", "4d", "Ac"].into();

            assert_eq!(made_hand.power_index(), 1614);
        }

        #[test]
        fn it_returns_3sqd7d9d3h4cqh_power_index_2813() {
            let made_hand: MadeHand = card_array!["3s", "Qd", "7d", "9d", "3h", "4c", "Qh"].into();

            assert_eq!(made_hand.power_index(), 2813);
        }

        #[test]
        fn it_returns_tdah5s5d6s4d7d_power_index_5335() {
            let made_hand: MadeHand = card_array!["Td", "Ah", "5s", "5d", "6s", "4d", "7d"].into();

            assert_eq!(made_hand.power_index(), 5335);
        }

        #[test]
        fn it_returns_9c4d4hts9h6cqh_power_index_3064() {
            let made_hand: MadeHand = card_array!["9c", "4d", "4h", "Ts", "9h", "6c", "Qh"].into();

            assert_eq!(made_hand.power_index(), 3064);
        }

        #[test]
        fn it_returns_8h8c4h6dqd3c9d_power_index_4762() {
            let made_hand: MadeHand = card_array!["8h", "8c", "4h", "6d", "Qd", "3c", "9d"].into();

            assert_eq!(made_hand.power_index(), 4762);
        }

        #[test]
        fn it_returns_6das7s8cjhjcth_power_index_4006() {
            let made_hand: MadeHand = card_array!["6d", "As", "7s", "8c", "Jh", "Jc", "Th"].into();

            assert_eq!(made_hand.power_index(), 4006);
        }

        #[test]
        fn it_returns_8sjh6s7sasjd2h_power_index_4020() {
            let made_hand: MadeHand = card_array!["8s", "Jh", "6s", "7s", "As", "Jd", "2h"].into();

            assert_eq!(made_hand.power_index(), 4020);
        }

        #[test]
        fn it_returns_qctc3s7d8s3cah_power_index_5757() {
            let made_hand: MadeHand = card_array!["Qc", "Tc", "3s", "7d", "8s", "3c", "Ah"].into();

            assert_eq!(made_hand.power_index(), 5757);
        }

        #[test]
        fn it_returns_as8d9dac2ckc9h_power_index_2512() {
            let made_hand: MadeHand = card_array!["As", "8d", "9d", "Ac", "2c", "Kc", "9h"].into();

            assert_eq!(made_hand.power_index(), 2512);
        }

        #[test]
        fn it_returns_9ctsthjhkh7d5c_power_index_4270() {
            let made_hand: MadeHand = card_array!["9c", "Ts", "Th", "Jh", "Kh", "7d", "5c"].into();

            assert_eq!(made_hand.power_index(), 4270);
        }

        #[test]
        fn it_returns_8ckh9hkcjhjsjc_power_index_204() {
            let made_hand: MadeHand = card_array!["8c", "Kh", "9h", "Kc", "Jh", "Js", "Jc"].into();

            assert_eq!(made_hand.power_index(), 204);
        }

        #[test]
        fn it_returns_qc7hah2d4cjd5s_power_index_6372() {
            let made_hand: MadeHand = card_array!["Qc", "7h", "Ah", "2d", "4c", "Jd", "5s"].into();

            assert_eq!(made_hand.power_index(), 6372);
        }

        #[test]
        fn it_returns_5s2c6s2das7dkh_power_index_5971() {
            let made_hand: MadeHand = card_array!["5s", "2c", "6s", "2d", "As", "7d", "Kh"].into();

            assert_eq!(made_hand.power_index(), 5971);
        }

        #[test]
        fn it_returns_kh7d9skd3ckc5h_power_index_1715() {
            let made_hand: MadeHand = card_array!["Kh", "7d", "9s", "Kd", "3c", "Kc", "5h"].into();

            assert_eq!(made_hand.power_index(), 1715);
        }

        #[test]
        fn it_returns_tdadjc7s6d4dkh_power_index_6232() {
            let made_hand: MadeHand = card_array!["Td", "Ad", "Jc", "7s", "6d", "4d", "Kh"].into();

            assert_eq!(made_hand.power_index(), 6232);
        }

        #[test]
        fn it_returns_8dkh7c2h9s9had_power_index_4429() {
            let made_hand: MadeHand = card_array!["8d", "Kh", "7c", "2h", "9s", "9h", "Ad"].into();

            assert_eq!(made_hand.power_index(), 4429);
        }

        #[test]
        fn it_returns_tsqd3s4sjdkhkc_power_index_3601() {
            let made_hand: MadeHand = card_array!["Ts", "Qd", "3s", "4s", "Jd", "Kh", "Kc"].into();

            assert_eq!(made_hand.power_index(), 3601);
        }

        #[test]
        fn it_returns_6s3das5hqd9dkc_power_index_6204() {
            let made_hand: MadeHand = card_array!["6s", "3d", "As", "5h", "Qd", "9d", "Kc"].into();

            assert_eq!(made_hand.power_index(), 6204);
        }

        #[test]
        fn it_returns_asqs3d2cjh7dac_power_index_3384() {
            let made_hand: MadeHand = card_array!["As", "Qs", "3d", "2c", "Jh", "7d", "Ac"].into();

            assert_eq!(made_hand.power_index(), 3384);
        }

        #[test]
        fn it_returns_2skcqs2d9stc5h_power_index_6022() {
            let made_hand: MadeHand = card_array!["2s", "Kc", "Qs", "2d", "9s", "Tc", "5h"].into();

            assert_eq!(made_hand.power_index(), 6022);
        }

        #[test]
        fn it_returns_9s2d2s9dqh9hjh_power_index_238() {
            let made_hand: MadeHand = card_array!["9s", "2d", "2s", "9d", "Qh", "9h", "Jh"].into();

            assert_eq!(made_hand.power_index(), 238);
        }

        #[test]
        fn it_returns_2dac8d9dkd3c9s_power_index_4429() {
            let made_hand: MadeHand = card_array!["2d", "Ac", "8d", "9d", "Kd", "3c", "9s"].into();

            assert_eq!(made_hand.power_index(), 4429);
        }

        #[test]
        fn it_returns_jcjd9h4sqdjh5h_power_index_1830() {
            let made_hand: MadeHand = card_array!["Jc", "Jd", "9h", "4s", "Qd", "Jh", "5h"].into();

            assert_eq!(made_hand.power_index(), 1830);
        }

        #[test]
        fn it_returns_jh2s6s9dad5cqd_power_index_6360() {
            let made_hand: MadeHand = card_array!["Jh", "2s", "6s", "9d", "Ad", "5c", "Qd"].into();

            assert_eq!(made_hand.power_index(), 6360);
        }

        #[test]
        fn it_returns_td5h6c4hkd7h6d_power_index_5160() {
            let made_hand: MadeHand = card_array!["Td", "5h", "6c", "4h", "Kd", "7h", "6d"].into();

            assert_eq!(made_hand.power_index(), 5160);
        }

        #[test]
        fn it_returns_8dtc8c6d8h5h6s_power_index_246() {
            let made_hand: MadeHand = card_array!["8d", "Tc", "8c", "6d", "8h", "5h", "6s"].into();

            assert_eq!(made_hand.power_index(), 246);
        }

        #[test]
        fn it_returns_9hkc6cqd5htd8c_power_index_6714() {
            let made_hand: MadeHand = card_array!["9h", "Kc", "6c", "Qd", "5h", "Td", "8c"].into();

            assert_eq!(made_hand.power_index(), 6714);
        }

        #[test]
        fn it_returns_6cadah7hqd9dkh_power_index_3328() {
            let made_hand: MadeHand = card_array!["6c", "Ad", "Ah", "7h", "Qd", "9d", "Kh"].into();

            assert_eq!(made_hand.power_index(), 3328);
        }

        #[test]
        fn it_returns_js8h2d8cts2c9s_power_index_3153() {
            let made_hand: MadeHand = card_array!["Js", "8h", "2d", "8c", "Ts", "2c", "9s"].into();

            assert_eq!(made_hand.power_index(), 3153);
        }

        #[test]
        fn it_returns_2h5c3sks9dad4d_power_index_1609() {
            let made_hand: MadeHand = card_array!["2h", "5c", "3s", "Ks", "9d", "Ad", "4d"].into();

            assert_eq!(made_hand.power_index(), 1609);
        }

        #[test]
        fn it_returns_2hkc5c4stdqc3d_power_index_6736() {
            let made_hand: MadeHand = card_array!["2h", "Kc", "5c", "4s", "Td", "Qc", "3d"].into();

            assert_eq!(made_hand.power_index(), 6736);
        }

        #[test]
        fn it_returns_3c5h8d2c5s2dah_power_index_3282() {
            let made_hand: MadeHand = card_array!["3c", "5h", "8d", "2c", "5s", "2d", "Ah"].into();

            assert_eq!(made_hand.power_index(), 3282);
        }

        #[test]
        fn it_returns_tsad7d8s9d7h7c_power_index_2075() {
            let made_hand: MadeHand = card_array!["Ts", "Ad", "7d", "8s", "9d", "7h", "7c"].into();

            assert_eq!(made_hand.power_index(), 2075);
        }

        #[test]
        fn it_returns_7std9cts2dkhkd_power_index_2625() {
            let made_hand: MadeHand = card_array!["7s", "Td", "9c", "Ts", "2d", "Kh", "Kd"].into();

            assert_eq!(made_hand.power_index(), 2625);
        }

        #[test]
        fn it_returns_4c4hqhackd8hks_power_index_2688() {
            let made_hand: MadeHand = card_array!["4c", "4h", "Qh", "Ac", "Kd", "8h", "Ks"].into();

            assert_eq!(made_hand.power_index(), 2688);
        }

        #[test]
        fn it_returns_2c9dtcts4skd3s_power_index_4282() {
            let made_hand: MadeHand = card_array!["2c", "9d", "Tc", "Ts", "4s", "Kd", "3s"].into();

            assert_eq!(made_hand.power_index(), 4282);
        }

        #[test]
        fn it_returns_9sjsad4h4d3h3c_power_index_3293() {
            let made_hand: MadeHand = card_array!["9s", "Js", "Ad", "4h", "4d", "3h", "3c"].into();

            assert_eq!(made_hand.power_index(), 3293);
        }

        #[test]
        fn it_returns_asahqd3sqs2s5s_power_index_605() {
            let made_hand: MadeHand = card_array!["As", "Ah", "Qd", "3s", "Qs", "2s", "5s"].into();

            assert_eq!(made_hand.power_index(), 605);
        }

        #[test]
        fn it_returns_7d6c5cqhts7s2d_power_index_4976() {
            let made_hand: MadeHand = card_array!["7d", "6c", "5c", "Qh", "Ts", "7s", "2d"].into();

            assert_eq!(made_hand.power_index(), 4976);
        }

        #[test]
        fn it_returns_8d6djhac7d6h4d_power_index_5107() {
            let made_hand: MadeHand = card_array!["8d", "6d", "Jh", "Ac", "7d", "6h", "4d"].into();

            assert_eq!(made_hand.power_index(), 5107);
        }

        #[test]
        fn it_returns_5s8ctcjc6c2cts_power_index_1382() {
            let made_hand: MadeHand = card_array!["5s", "8c", "Tc", "Jc", "6c", "2c", "Ts"].into();

            assert_eq!(made_hand.power_index(), 1382);
        }

        #[test]
        fn it_returns_ah7d5dadqh2d4d_power_index_809() {
            let made_hand: MadeHand = card_array!["Ah", "7d", "5d", "Ad", "Qh", "2d", "4d"].into();

            assert_eq!(made_hand.power_index(), 809);
        }

        #[test]
        fn it_returns_qd4htd5c4ckd8c_power_index_5582() {
            let made_hand: MadeHand = card_array!["Qd", "4h", "Td", "5c", "4c", "Kd", "8c"].into();

            assert_eq!(made_hand.power_index(), 5582);
        }

        #[test]
        fn it_returns_kd7d3ckc8cjd4h_power_index_3661() {
            let made_hand: MadeHand = card_array!["Kd", "7d", "3c", "Kc", "8c", "Jd", "4h"].into();

            assert_eq!(made_hand.power_index(), 3661);
        }

        #[test]
        fn it_returns_8hjsjhqdkh3c6h_power_index_4043() {
            let made_hand: MadeHand = card_array!["8h", "Js", "Jh", "Qd", "Kh", "3c", "6h"].into();

            assert_eq!(made_hand.power_index(), 4043);
        }

        #[test]
        fn it_returns_6c6dkd3h2sjhks_power_index_2668() {
            let made_hand: MadeHand = card_array!["6c", "6d", "Kd", "3h", "2s", "Jh", "Ks"].into();

            assert_eq!(made_hand.power_index(), 2668);
        }

        #[test]
        fn it_returns_2h4c7s6sjdqckd_power_index_6699() {
            let made_hand: MadeHand = card_array!["2h", "4c", "7s", "6s", "Jd", "Qc", "Kd"].into();

            assert_eq!(made_hand.power_index(), 6699);
        }

        #[test]
        fn it_returns_8hqd4s2h8c3cjs_power_index_4751() {
            let made_hand: MadeHand = card_array!["8h", "Qd", "4s", "2h", "8c", "3c", "Js"].into();

            assert_eq!(made_hand.power_index(), 4751);
        }

        #[test]
        fn it_returns_jh5c7d6s5hts7h_power_index_3175() {
            let made_hand: MadeHand = card_array!["Jh", "5c", "7d", "6s", "5h", "Ts", "7h"].into();

            assert_eq!(made_hand.power_index(), 3175);
        }

        #[test]
        fn it_returns_acadkh5c8hqhkc_power_index_2468() {
            let made_hand: MadeHand = card_array!["Ac", "Ad", "Kh", "5c", "8h", "Qh", "Kc"].into();

            assert_eq!(made_hand.power_index(), 2468);
        }

        #[test]
        fn it_returns_7h8hts9skd2s7d_power_index_4938() {
            let made_hand: MadeHand = card_array!["7h", "8h", "Ts", "9s", "Kd", "2s", "7d"].into();

            assert_eq!(made_hand.power_index(), 4938);
        }

        #[test]
        fn it_returns_kc6s4sqd8h3d9d_power_index_6743() {
            let made_hand: MadeHand = card_array!["Kc", "6s", "4s", "Qd", "8h", "3d", "9d"].into();

            assert_eq!(made_hand.power_index(), 6743);
        }

        #[test]
        fn it_returns_4d2c3s7d4c7s7h_power_index_260() {
            let made_hand: MadeHand = card_array!["4d", "2c", "3s", "7d", "4c", "7s", "7h"].into();

            assert_eq!(made_hand.power_index(), 260);
        }

        #[test]
        fn it_returns_acjhts5s9c4has_power_index_3426() {
            let made_hand: MadeHand = card_array!["Ac", "Jh", "Ts", "5s", "9c", "4h", "As"].into();

            assert_eq!(made_hand.power_index(), 3426);
        }

        #[test]
        fn it_returns_asjcjh9s8h2ctd_power_index_4005() {
            let made_hand: MadeHand = card_array!["As", "Jc", "Jh", "9s", "8h", "2c", "Td"].into();

            assert_eq!(made_hand.power_index(), 4005);
        }

        #[test]
        fn it_returns_6dqd9s3cjh2sts_power_index_7009() {
            let made_hand: MadeHand = card_array!["6d", "Qd", "9s", "3c", "Jh", "2s", "Ts"].into();

            assert_eq!(made_hand.power_index(), 7009);
        }

        #[test]
        fn it_returns_4dkc9sqs3c7stc_power_index_6715() {
            let made_hand: MadeHand = card_array!["4d", "Kc", "9s", "Qs", "3c", "7s", "Tc"].into();

            assert_eq!(made_hand.power_index(), 6715);
        }

        #[test]
        fn it_returns_kdqcjd9dah9s4c_power_index_4426() {
            let made_hand: MadeHand = card_array!["Kd", "Qc", "Jd", "9d", "Ah", "9s", "4c"].into();

            assert_eq!(made_hand.power_index(), 4426);
        }

        #[test]
        fn it_returns_kc4dahtd6c7h3c_power_index_6279() {
            let made_hand: MadeHand = card_array!["Kc", "4d", "Ah", "Td", "6c", "7h", "3c"].into();

            assert_eq!(made_hand.power_index(), 6279);
        }

        #[test]
        fn it_returns_jdac3hkc9sjh2h_power_index_3988() {
            let made_hand: MadeHand = card_array!["Jd", "Ac", "3h", "Kc", "9s", "Jh", "2h"].into();

            assert_eq!(made_hand.power_index(), 3988);
        }

        #[test]
        fn it_returns_7h4d6c2h8h7d8d_power_index_3101() {
            let made_hand: MadeHand = card_array!["7h", "4d", "6c", "2h", "8h", "7d", "8d"].into();

            assert_eq!(made_hand.power_index(), 3101);
        }

        #[test]
        fn it_returns_td6sas8htckcqc_power_index_4206() {
            let made_hand: MadeHand = card_array!["Td", "6s", "As", "8h", "Tc", "Kc", "Qc"].into();

            assert_eq!(made_hand.power_index(), 4206);
        }

        #[test]
        fn it_returns_7h7c3dqdqs4djd_power_index_2767() {
            let made_hand: MadeHand = card_array!["7h", "7c", "3d", "Qd", "Qs", "4d", "Jd"].into();

            assert_eq!(made_hand.power_index(), 2767);
        }

        #[test]
        fn it_returns_ahks7h8c5h4std_power_index_6273() {
            let made_hand: MadeHand = card_array!["Ah", "Ks", "7h", "8c", "5h", "4s", "Td"].into();

            assert_eq!(made_hand.power_index(), 6273);
        }

        #[test]
        fn it_returns_8c9dqh6c7h3d9h_power_index_4541() {
            let made_hand: MadeHand = card_array!["8c", "9d", "Qh", "6c", "7h", "3d", "9h"].into();

            assert_eq!(made_hand.power_index(), 4541);
        }

        #[test]
        fn it_returns_7s3cac7cjhkc9s_power_index_4867() {
            let made_hand: MadeHand = card_array!["7s", "3c", "Ac", "7c", "Jh", "Kc", "9s"].into();

            assert_eq!(made_hand.power_index(), 4867);
        }

        #[test]
        fn it_returns_4dthtc5has8s9c_power_index_4233() {
            let made_hand: MadeHand = card_array!["4d", "Th", "Tc", "5h", "As", "8s", "9c"].into();

            assert_eq!(made_hand.power_index(), 4233);
        }

        #[test]
        fn it_returns_8c2dqhks7h3s6c_power_index_6763() {
            let made_hand: MadeHand = card_array!["8c", "2d", "Qh", "Ks", "7h", "3s", "6c"].into();

            assert_eq!(made_hand.power_index(), 6763);
        }

        #[test]
        fn it_returns_adqs2dac3ctcts_power_index_2502() {
            let made_hand: MadeHand = card_array!["Ad", "Qs", "2d", "Ac", "3c", "Tc", "Ts"].into();

            assert_eq!(made_hand.power_index(), 2502);
        }

        #[test]
        fn it_returns_9d5sacah2d6c6s_power_index_2549() {
            let made_hand: MadeHand = card_array!["9d", "5s", "Ac", "Ah", "2d", "6c", "6s"].into();

            assert_eq!(made_hand.power_index(), 2549);
        }

        #[test]
        fn it_returns_kd9c4h8c7c7h6s_power_index_4945() {
            let made_hand: MadeHand = card_array!["Kd", "9c", "4h", "8c", "7c", "7h", "6s"].into();

            assert_eq!(made_hand.power_index(), 4945);
        }

        #[test]
        fn it_returns_jhqc5skc7c3dkd_power_index_3604() {
            let made_hand: MadeHand = card_array!["Jh", "Qc", "5s", "Kc", "7c", "3d", "Kd"].into();

            assert_eq!(made_hand.power_index(), 3604);
        }

        #[test]
        fn it_returns_qh3djs8skd3cth_power_index_5801() {
            let made_hand: MadeHand = card_array!["Qh", "3d", "Js", "8s", "Kd", "3c", "Th"].into();

            assert_eq!(made_hand.power_index(), 5801);
        }

        #[test]
        fn it_returns_2h7c8c8dad4djd_power_index_4667() {
            let made_hand: MadeHand = card_array!["2h", "7c", "8c", "8d", "Ad", "4d", "Jd"].into();

            assert_eq!(made_hand.power_index(), 4667);
        }

        #[test]
        fn it_returns_qd7had9s9d6skh_power_index_4426() {
            let made_hand: MadeHand = card_array!["Qd", "7h", "Ad", "9s", "9d", "6s", "Kh"].into();

            assert_eq!(made_hand.power_index(), 4426);
        }

        #[test]
        fn it_returns_9c8c3d8hts6ckd_power_index_4718() {
            let made_hand: MadeHand = card_array!["9c", "8c", "3d", "8h", "Ts", "6c", "Kd"].into();

            assert_eq!(made_hand.power_index(), 4718);
        }

        #[test]
        fn it_returns_3c5hthksah6cqs_power_index_6197() {
            let made_hand: MadeHand = card_array!["3c", "5h", "Th", "Ks", "Ah", "6c", "Qs"].into();

            assert_eq!(made_hand.power_index(), 6197);
        }

        #[test]
        fn it_returns_8d7s9dqh5c4cks_power_index_6742() {
            let made_hand: MadeHand = card_array!["8d", "7s", "9d", "Qh", "5c", "4c", "Ks"].into();

            assert_eq!(made_hand.power_index(), 6742);
        }

        #[test]
        fn it_returns_3sah6hjh7hjd2s_power_index_4026() {
            let made_hand: MadeHand = card_array!["3s", "Ah", "6h", "Jh", "7h", "Jd", "2s"].into();

            assert_eq!(made_hand.power_index(), 4026);
        }

        #[test]
        fn it_returns_9hjd8s8c8h4d2h_power_index_2037() {
            let made_hand: MadeHand = card_array!["9h", "Jd", "8s", "8c", "8h", "4d", "2h"].into();

            assert_eq!(made_hand.power_index(), 2037);
        }

        #[test]
        fn it_returns_7c3skh6s8dtcjc_power_index_6805() {
            let made_hand: MadeHand = card_array!["7c", "3s", "Kh", "6s", "8d", "Tc", "Jc"].into();

            assert_eq!(made_hand.power_index(), 6805);
        }

        #[test]
        fn it_returns_ac8sas3ckd5cth_power_index_3346() {
            let made_hand: MadeHand = card_array!["Ac", "8s", "As", "3c", "Kd", "5c", "Th"].into();

            assert_eq!(made_hand.power_index(), 3346);
        }

        #[test]
        fn it_returns_3c3s4h7dasqskc_power_index_5746() {
            let made_hand: MadeHand = card_array!["3c", "3s", "4h", "7d", "As", "Qs", "Kc"].into();

            assert_eq!(made_hand.power_index(), 5746);
        }

        #[test]
        fn it_returns_6d5cqcjcjdjs5s_power_index_211() {
            let made_hand: MadeHand = card_array!["6d", "5c", "Qc", "Jc", "Jd", "Js", "5s"].into();

            assert_eq!(made_hand.power_index(), 211);
        }

        #[test]
        fn it_returns_5c7hqckdah2d9d_power_index_6203() {
            let made_hand: MadeHand = card_array!["5c", "7h", "Qc", "Kd", "Ah", "2d", "9d"].into();

            assert_eq!(made_hand.power_index(), 6203);
        }

        #[test]
        fn it_returns_qcksth2h5hjs3c_power_index_6682() {
            let made_hand: MadeHand = card_array!["Qc", "Ks", "Th", "2h", "5h", "Js", "3c"].into();

            assert_eq!(made_hand.power_index(), 6682);
        }

        #[test]
        fn it_returns_ac7hjh5dthas3h_power_index_3428() {
            let made_hand: MadeHand = card_array!["Ac", "7h", "Jh", "5d", "Th", "As", "3h"].into();

            assert_eq!(made_hand.power_index(), 3428);
        }

        #[test]
        fn it_returns_9sjh9c7has2sqc_power_index_4436() {
            let made_hand: MadeHand = card_array!["9s", "Jh", "9c", "7h", "As", "2s", "Qc"].into();

            assert_eq!(made_hand.power_index(), 4436);
        }

        #[test]
        fn it_returns_qd5d7c4s9dkh2h_power_index_6749() {
            let made_hand: MadeHand = card_array!["Qd", "5d", "7c", "4s", "9d", "Kh", "2h"].into();

            assert_eq!(made_hand.power_index(), 6749);
        }

        #[test]
        fn it_returns_adtdqsac8dkcth_power_index_2501() {
            let made_hand: MadeHand = card_array!["Ad", "Td", "Qs", "Ac", "8d", "Kc", "Th"].into();

            assert_eq!(made_hand.power_index(), 2501);
        }

        #[test]
        fn it_returns_4hthjs5cad8d5h_power_index_5325() {
            let made_hand: MadeHand = card_array!["4h", "Th", "Js", "5c", "Ad", "8d", "5h"].into();

            assert_eq!(made_hand.power_index(), 5325);
        }

        #[test]
        fn it_returns_7sqcqdkd9d5d7c_power_index_2766() {
            let made_hand: MadeHand = card_array!["7s", "Qc", "Qd", "Kd", "9d", "5d", "7c"].into();

            assert_eq!(made_hand.power_index(), 2766);
        }

        #[test]
        fn it_returns_9cac3h3sjc4sjd_power_index_2908() {
            let made_hand: MadeHand = card_array!["9c", "Ac", "3h", "3s", "Jc", "4s", "Jd"].into();

            assert_eq!(made_hand.power_index(), 2908);
        }

        #[test]
        fn it_returns_6c7d6sjdqh9std_power_index_5186() {
            let made_hand: MadeHand = card_array!["6c", "7d", "6s", "Jd", "Qh", "9s", "Td"].into();

            assert_eq!(made_hand.power_index(), 5186);
        }

        #[test]
        fn it_returns_6hac5h4h6dasqc_power_index_2546() {
            let made_hand: MadeHand = card_array!["6h", "Ac", "5h", "4h", "6d", "As", "Qc"].into();

            assert_eq!(made_hand.power_index(), 2546);
        }

        #[test]
        fn it_returns_ksah2h5s9h7s4c_power_index_6301() {
            let made_hand: MadeHand = card_array!["Ks", "Ah", "2h", "5s", "9h", "7s", "4c"].into();

            assert_eq!(made_hand.power_index(), 6301);
        }

        #[test]
        fn it_returns_9dad8sahqc6h7h_power_index_3398() {
            let made_hand: MadeHand = card_array!["9d", "Ad", "8s", "Ah", "Qc", "6h", "7h"].into();

            assert_eq!(made_hand.power_index(), 3398);
        }

        #[test]
        fn it_returns_qd5ckh9hqc6ckc_power_index_2603() {
            let made_hand: MadeHand = card_array!["Qd", "5c", "Kh", "9h", "Qc", "6c", "Kc"].into();

            assert_eq!(made_hand.power_index(), 2603);
        }

        #[test]
        fn it_returns_2d2hkh6s4c2s5c_power_index_2419() {
            let made_hand: MadeHand = card_array!["2d", "2h", "Kh", "6s", "4c", "2s", "5c"].into();

            assert_eq!(made_hand.power_index(), 2419);
        }

        #[test]
        fn it_returns_js9cth8hacts7s_power_index_1603() {
            let made_hand: MadeHand = card_array!["Js", "9c", "Th", "8h", "Ac", "Ts", "7s"].into();

            assert_eq!(made_hand.power_index(), 1603);
        }

        #[test]
        fn it_returns_9hqh4djc2d4h6d_power_index_5627() {
            let made_hand: MadeHand = card_array!["9h", "Qh", "4d", "Jc", "2d", "4h", "6d"].into();

            assert_eq!(made_hand.power_index(), 5627);
        }

        #[test]
        fn it_returns_2c5h8sahqd2h4d_power_index_5979() {
            let made_hand: MadeHand = card_array!["2c", "5h", "8s", "Ah", "Qd", "2h", "4d"].into();

            assert_eq!(made_hand.power_index(), 5979);
        }

        #[test]
        fn it_returns_kd4c6dts7h4h6c_power_index_3228() {
            let made_hand: MadeHand = card_array!["Kd", "4c", "6d", "Ts", "7h", "4h", "6c"].into();

            assert_eq!(made_hand.power_index(), 3228);
        }

        #[test]
        fn it_returns_jh8h3s9d2sqh6d_power_index_7036() {
            let made_hand: MadeHand = card_array!["Jh", "8h", "3s", "9d", "2s", "Qh", "6d"].into();

            assert_eq!(made_hand.power_index(), 7036);
        }

        #[test]
        fn it_returns_khksqc3cqs3d9s_power_index_2603() {
            let made_hand: MadeHand = card_array!["Kh", "Ks", "Qc", "3c", "Qs", "3d", "9s"].into();

            assert_eq!(made_hand.power_index(), 2603);
        }

        #[test]
        fn it_returns_js3s8h5s7htd2h_power_index_7238() {
            let made_hand: MadeHand = card_array!["Js", "3s", "8h", "5s", "7h", "Td", "2h"].into();

            assert_eq!(made_hand.power_index(), 7238);
        }

        #[test]
        fn it_returns_6d8cad5s6skc5h_power_index_3216() {
            let made_hand: MadeHand = card_array!["6d", "8c", "Ad", "5s", "6s", "Kc", "5h"].into();

            assert_eq!(made_hand.power_index(), 3216);
        }

        #[test]
        fn it_returns_6h3dks8s9d3s4h_power_index_5825() {
            let made_hand: MadeHand = card_array!["6h", "3d", "Ks", "8s", "9d", "3s", "4h"].into();

            assert_eq!(made_hand.power_index(), 5825);
        }

        #[test]
        fn it_returns_3hjh2s7c8s3s9s_power_index_5889() {
            let made_hand: MadeHand = card_array!["3h", "Jh", "2s", "7c", "8s", "3s", "9s"].into();

            assert_eq!(made_hand.power_index(), 5889);
        }

        #[test]
        fn it_returns_9h2sjhqs6d3s8h_power_index_7036() {
            let made_hand: MadeHand = card_array!["9h", "2s", "Jh", "Qs", "6d", "3s", "8h"].into();

            assert_eq!(made_hand.power_index(), 7036);
        }

        #[test]
        fn it_returns_4c2s7hkcth5sas_power_index_6280() {
            let made_hand: MadeHand = card_array!["4c", "2s", "7h", "Kc", "Th", "5s", "As"].into();

            assert_eq!(made_hand.power_index(), 6280);
        }

        #[test]
        fn it_returns_2dqh4d9d2h8dkh_power_index_6023() {
            let made_hand: MadeHand = card_array!["2d", "Qh", "4d", "9d", "2h", "8d", "Kh"].into();

            assert_eq!(made_hand.power_index(), 6023);
        }

        #[test]
        fn it_returns_ac5h9dts3hkc4h_power_index_6269() {
            let made_hand: MadeHand = card_array!["Ac", "5h", "9d", "Ts", "3h", "Kc", "4h"].into();

            assert_eq!(made_hand.power_index(), 6269);
        }

        #[test]
        fn it_returns_4h9s7djd6cqh2d_power_index_7041() {
            let made_hand: MadeHand = card_array!["4h", "9s", "7d", "Jd", "6c", "Qh", "2d"].into();

            assert_eq!(made_hand.power_index(), 7041);
        }

        #[test]
        fn it_returns_2hkh7hac6hqcah_power_index_470() {
            let made_hand: MadeHand = card_array!["2h", "Kh", "7h", "Ac", "6h", "Qc", "Ah"].into();

            assert_eq!(made_hand.power_index(), 470);
        }

        #[test]
        fn it_returns_3dqc3s4c9h4has_power_index_3293() {
            let made_hand: MadeHand = card_array!["3d", "Qc", "3s", "4c", "9h", "4h", "As"].into();

            assert_eq!(made_hand.power_index(), 3293);
        }

        #[test]
        fn it_returns_2dqhth2s9d9hqd_power_index_2746() {
            let made_hand: MadeHand = card_array!["2d", "Qh", "Th", "2s", "9d", "9h", "Qd"].into();

            assert_eq!(made_hand.power_index(), 2746);
        }

        #[test]
        fn it_returns_3c4s6s6d7c9c6h_power_index_2184() {
            let made_hand: MadeHand = card_array!["3c", "4s", "6s", "6d", "7c", "9c", "6h"].into();

            assert_eq!(made_hand.power_index(), 2184);
        }

        #[test]
        fn it_returns_2c6h8skc2h5s7h_power_index_6051() {
            let made_hand: MadeHand = card_array!["2c", "6h", "8s", "Kc", "2h", "5s", "7h"].into();

            assert_eq!(made_hand.power_index(), 6051);
        }

        #[test]
        fn it_returns_5d2d8sah5cac4s_power_index_2561() {
            let made_hand: MadeHand = card_array!["5d", "2d", "8s", "Ah", "5c", "Ac", "4s"].into();

            assert_eq!(made_hand.power_index(), 2561);
        }

        #[test]
        fn it_returns_jh2sqd9s7s6hjs_power_index_4095() {
            let made_hand: MadeHand = card_array!["Jh", "2s", "Qd", "9s", "7s", "6h", "Js"].into();

            assert_eq!(made_hand.power_index(), 4095);
        }

        #[test]
        fn it_returns_jd3s7h2ckc9s2s_power_index_6031() {
            let made_hand: MadeHand = card_array!["Jd", "3s", "7h", "2c", "Kc", "9s", "2s"].into();

            assert_eq!(made_hand.power_index(), 6031);
        }

        #[test]
        fn it_returns_2sksjc7dkhtd8d_power_index_3647() {
            let made_hand: MadeHand = card_array!["2s", "Ks", "Jc", "7d", "Kh", "Td", "8d"].into();

            assert_eq!(made_hand.power_index(), 3647);
        }

        #[test]
        fn it_returns_th4s9c3c6h2s6s_power_index_5253() {
            let made_hand: MadeHand = card_array!["Th", "4s", "9c", "3c", "6h", "2s", "6s"].into();

            assert_eq!(made_hand.power_index(), 5253);
        }

        #[test]
        fn it_returns_8sts2ctd7dqs3h_power_index_4321() {
            let made_hand: MadeHand = card_array!["8s", "Ts", "2c", "Td", "7d", "Qs", "3h"].into();

            assert_eq!(made_hand.power_index(), 4321);
        }

        #[test]
        fn it_returns_6sth6has6d2c2h_power_index_274() {
            let made_hand: MadeHand = card_array!["6s", "Th", "6h", "As", "6d", "2c", "2h"].into();

            assert_eq!(made_hand.power_index(), 274);
        }

        #[test]
        fn it_returns_qd6d8sac8d4cqs_power_index_2754() {
            let made_hand: MadeHand = card_array!["Qd", "6d", "8s", "Ac", "8d", "4c", "Qs"].into();

            assert_eq!(made_hand.power_index(), 2754);
        }

        #[test]
        fn it_returns_3dadkstdqc2d4h_power_index_6199() {
            let made_hand: MadeHand = card_array!["3d", "Ad", "Ks", "Td", "Qc", "2d", "4h"].into();

            assert_eq!(made_hand.power_index(), 6199);
        }

        #[test]
        fn it_returns_9d4c8h5dac2c5c_power_index_5340() {
            let made_hand: MadeHand = card_array!["9d", "4c", "8h", "5d", "Ac", "2c", "5c"].into();

            assert_eq!(made_hand.power_index(), 5340);
        }

        #[test]
        fn it_returns_kc4c9cas8s4sqs_power_index_5526() {
            let made_hand: MadeHand = card_array!["Kc", "4c", "9c", "As", "8s", "4s", "Qs"].into();

            assert_eq!(made_hand.power_index(), 5526);
        }

        #[test]
        fn it_returns_qs8hqh9s7d4d2h_power_index_3930() {
            let made_hand: MadeHand = card_array!["Qs", "8h", "Qh", "9s", "7d", "4d", "2h"].into();

            assert_eq!(made_hand.power_index(), 3930);
        }

        #[test]
        fn it_returns_9s2s6dacts3dkh_power_index_6268() {
            let made_hand: MadeHand = card_array!["9s", "2s", "6d", "Ac", "Ts", "3d", "Kh"].into();

            assert_eq!(made_hand.power_index(), 6268);
        }

        #[test]
        fn it_returns_3d6c7hqctstd5c_power_index_4327() {
            let made_hand: MadeHand = card_array!["3d", "6c", "7h", "Qc", "Ts", "Td", "5c"].into();

            assert_eq!(made_hand.power_index(), 4327);
        }

        #[test]
        fn it_returns_kh5d6s6d2c5cjc_power_index_3217() {
            let made_hand: MadeHand = card_array!["Kh", "5d", "6s", "6d", "2c", "5c", "Jc"].into();

            assert_eq!(made_hand.power_index(), 3217);
        }

        #[test]
        fn it_returns_9dqh4s2s2cahtc_power_index_5977() {
            let made_hand: MadeHand = card_array!["9d", "Qh", "4s", "2s", "2c", "Ah", "Tc"].into();

            assert_eq!(made_hand.power_index(), 5977);
        }

        #[test]
        fn it_returns_3has5hqs8hah3c_power_index_2579() {
            let made_hand: MadeHand = card_array!["3h", "As", "5h", "Qs", "8h", "Ah", "3c"].into();

            assert_eq!(made_hand.power_index(), 2579);
        }

        #[test]
        fn it_returns_2s8s8h6djc7sjh_power_index_2858() {
            let made_hand: MadeHand = card_array!["2s", "8s", "8h", "6d", "Jc", "7s", "Jh"].into();

            assert_eq!(made_hand.power_index(), 2858);
        }

        #[test]
        fn it_returns_3s7h5h7s5dac9c_power_index_3172() {
            let made_hand: MadeHand = card_array!["3s", "7h", "5h", "7s", "5d", "Ac", "9c"].into();

            assert_eq!(made_hand.power_index(), 3172);
        }

        #[test]
        fn it_returns_3h9s8c7d3c2d6c_power_index_5931() {
            let made_hand: MadeHand = card_array!["3h", "9s", "8c", "7d", "3c", "2d", "6c"].into();

            assert_eq!(made_hand.power_index(), 5931);
        }

        #[test]
        fn it_returns_3d9s2c2d7sah8s_power_index_6000() {
            let made_hand: MadeHand = card_array!["3d", "9s", "2c", "2d", "7s", "Ah", "8s"].into();

            assert_eq!(made_hand.power_index(), 6000);
        }

        #[test]
        fn it_returns_8ctc5s2c7d5c8s_power_index_3121() {
            let made_hand: MadeHand = card_array!["8c", "Tc", "5s", "2c", "7d", "5c", "8s"].into();

            assert_eq!(made_hand.power_index(), 3121);
        }

        #[test]
        fn it_returns_8s7s8h6sad5s9s_power_index_6() {
            let made_hand: MadeHand = card_array!["8s", "7s", "8h", "6s", "Ad", "5s", "9s"].into();

            assert_eq!(made_hand.power_index(), 6);
        }

        #[test]
        fn it_returns_8c7s8h8dadqh7h_power_index_245() {
            let made_hand: MadeHand = card_array!["8c", "7s", "8h", "8d", "Ad", "Qh", "7h"].into();

            assert_eq!(made_hand.power_index(), 245);
        }

        #[test]
        fn it_returns_tsjc6s3htdah6d_power_index_2963() {
            let made_hand: MadeHand = card_array!["Ts", "Jc", "6s", "3h", "Td", "Ah", "6d"].into();

            assert_eq!(made_hand.power_index(), 2963);
        }

        #[test]
        fn it_returns_3h9h7s6c4hjhkc_power_index_6832() {
            let made_hand: MadeHand = card_array!["3h", "9h", "7s", "6c", "4h", "Jh", "Kc"].into();

            assert_eq!(made_hand.power_index(), 6832);
        }

        #[test]
        fn it_returns_jd7h4d2stc9c9s_power_index_4563() {
            let made_hand: MadeHand = card_array!["Jd", "7h", "4d", "2s", "Tc", "9c", "9s"].into();

            assert_eq!(made_hand.power_index(), 4563);
        }

        #[test]
        fn it_returns_8dthkd6s8sah3s_power_index_4648() {
            let made_hand: MadeHand = card_array!["8d", "Th", "Kd", "6s", "8s", "Ah", "3s"].into();

            assert_eq!(made_hand.power_index(), 4648);
        }

        #[test]
        fn it_returns_th7sjs3ckhks4d_power_index_3648() {
            let made_hand: MadeHand = card_array!["Th", "7s", "Js", "3c", "Kh", "Ks", "4d"].into();

            assert_eq!(made_hand.power_index(), 3648);
        }

        #[test]
        fn it_returns_ts6d7d2sjh7c5c_power_index_5004() {
            let made_hand: MadeHand = card_array!["Ts", "6d", "7d", "2s", "Jh", "7c", "5c"].into();

            assert_eq!(made_hand.power_index(), 5004);
        }

        #[test]
        fn it_returns_8d7cjdjhqd9s9c_power_index_2844() {
            let made_hand: MadeHand = card_array!["8d", "7c", "Jd", "Jh", "Qd", "9s", "9c"].into();

            assert_eq!(made_hand.power_index(), 2844);
        }

        #[test]
        fn it_returns_ks4c7d2d4s8c3s_power_index_5611() {
            let made_hand: MadeHand = card_array!["Ks", "4c", "7d", "2d", "4s", "8c", "3s"].into();

            assert_eq!(made_hand.power_index(), 5611);
        }

        #[test]
        fn it_returns_3d7c5d6c3c2d8c_power_index_5946() {
            let made_hand: MadeHand = card_array!["3d", "7c", "5d", "6c", "3c", "2d", "8c"].into();

            assert_eq!(made_hand.power_index(), 5946);
        }
    }
}

fn find_flush_suit<'c>(cards: &[Card; 7]) -> Option<Suit> {
    let mut suit_counts = [0; 4];

    for card in cards {
        let suit = card.suit();
        let suit_index = u8::from(suit) as usize;

        suit_counts[suit_index] += 1;

        if suit_counts[suit_index] >= 5 {
            return Some(*suit);
        }
    }

    None
}

fn hash_for_flush<'c>(cards: &[Card; 7], suit: &Suit) -> u16 {
    let mut hash: u16 = 0;

    for card in cards.iter() {
        if card.suit() == suit {
            hash += match card.rank() {
                Rank::Ace => 0b1000000000000,
                Rank::King => 0b100000000000,
                Rank::Queen => 0b10000000000,
                Rank::Jack => 0b1000000000,
                Rank::Ten => 0b100000000,
                Rank::Nine => 0b10000000,
                Rank::Eight => 0b1000000,
                Rank::Seven => 0b100000,
                Rank::Six => 0b10000,
                Rank::Five => 0b1000,
                Rank::Four => 0b100,
                Rank::Trey => 0b10,
                Rank::Deuce => 0b1,
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

fn hash_for_rainbow<'c>(cards: &[Card; 7]) -> u16 {
    let mut card_len_each_rank: [u8; 13] = [0; 13];
    let mut remaining_card_len: u8 = 0;

    for card in cards.iter() {
        let card_i = u8::from(card.rank()) as usize;

        card_len_each_rank[card_i] += 1;
        remaining_card_len += 1;
    }

    let mut hash: u16 = 0;

    for rank in RANKS {
        let rank_i = u8::from(rank) as usize;
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
