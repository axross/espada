use std::fmt::{Display, Formatter};
use std::str::FromStr;

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

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let c: char = self.into();

        c.to_string().fmt(f)
    }
}

impl TryFrom<&char> for Rank {
    type Error = ();

    fn try_from(c: &char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Rank::Ace),
            'K' => Ok(Rank::King),
            'Q' => Ok(Rank::Queen),
            'J' => Ok(Rank::Jack),
            'T' => Ok(Rank::Ten),
            '9' => Ok(Rank::Nine),
            '8' => Ok(Rank::Eight),
            '7' => Ok(Rank::Seven),
            '6' => Ok(Rank::Six),
            '5' => Ok(Rank::Five),
            '4' => Ok(Rank::Four),
            '3' => Ok(Rank::Trey),
            '2' => Ok(Rank::Deuce),
            _ => Err(()),
        }
    }
}

impl TryFrom<char> for Rank {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        (&c).try_into()
    }
}

impl FromStr for Rank {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Some(c) = value.chars().nth(0) {
            c.try_into()
        } else {
            Err(())
        }
    }
}

impl From<&Rank> for u8 {
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

impl From<Rank> for u8 {
    fn from(rank: Rank) -> Self {
        u8::from(&rank)
    }
}

impl From<&Rank> for char {
    fn from(value: &Rank) -> Self {
        match value {
            Rank::Ace => 'A',
            Rank::King => 'K',
            Rank::Queen => 'Q',
            Rank::Jack => 'J',
            Rank::Ten => 'T',
            Rank::Nine => '9',
            Rank::Eight => '8',
            Rank::Seven => '7',
            Rank::Six => '6',
            Rank::Five => '5',
            Rank::Four => '4',
            Rank::Trey => '3',
            Rank::Deuce => '2',
        }
    }
}

impl From<Rank> for char {
    fn from(value: Rank) -> Self {
        char::from(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod display {
        use super::*;

        #[test]
        fn it_parses_into_a() {
            assert_eq!(Rank::Ace.to_string(), "A");
        }

        #[test]
        fn it_parses_into_k() {
            assert_eq!(Rank::King.to_string(), "K");
        }

        #[test]
        fn it_parses_into_q() {
            assert_eq!(Rank::Queen.to_string(), "Q");
        }

        #[test]
        fn it_parses_into_j() {
            assert_eq!(Rank::Jack.to_string(), "J");
        }

        #[test]
        fn it_parses_into_t() {
            assert_eq!(Rank::Ten.to_string(), "T");
        }

        #[test]
        fn it_parses_into_9() {
            assert_eq!(Rank::Nine.to_string(), "9");
        }

        #[test]
        fn it_parses_into_8() {
            assert_eq!(Rank::Eight.to_string(), "8");
        }

        #[test]
        fn it_parses_into_7() {
            assert_eq!(Rank::Seven.to_string(), "7");
        }

        #[test]
        fn it_parses_into_6() {
            assert_eq!(Rank::Six.to_string(), "6");
        }

        #[test]
        fn it_parses_into_5() {
            assert_eq!(Rank::Five.to_string(), "5");
        }

        #[test]
        fn it_parses_into_4() {
            assert_eq!(Rank::Four.to_string(), "4");
        }

        #[test]
        fn it_parses_into_3() {
            assert_eq!(Rank::Trey.to_string(), "3");
        }

        #[test]
        fn it_parses_into_2() {
            assert_eq!(Rank::Deuce.to_string(), "2");
        }
    }

    mod try_from_char {
        use super::*;

        #[test]
        fn it_parses_into_ace() {
            let result: Result<Rank, ()> = 'A'.try_into();

            assert_eq!(result.unwrap(), Rank::Ace);
        }

        #[test]
        fn it_parses_into_king() {
            let result: Result<Rank, ()> = 'K'.try_into();

            assert_eq!(result.unwrap(), Rank::King);
        }

        #[test]
        fn it_parses_into_queen() {
            let result: Result<Rank, ()> = 'Q'.try_into();

            assert_eq!(result.unwrap(), Rank::Queen);
        }

        #[test]
        fn it_parses_into_jack() {
            let result: Result<Rank, ()> = 'J'.try_into();

            assert_eq!(result.unwrap(), Rank::Jack);
        }

        #[test]
        fn it_parses_into_ten() {
            let result: Result<Rank, ()> = 'T'.try_into();

            assert_eq!(result.unwrap(), Rank::Ten);
        }

        #[test]
        fn it_parses_into_nine() {
            let result: Result<Rank, ()> = '9'.try_into();

            assert_eq!(result.unwrap(), Rank::Nine);
        }

        #[test]
        fn it_parses_into_eight() {
            let result: Result<Rank, ()> = '8'.try_into();

            assert_eq!(result.unwrap(), Rank::Eight);
        }

        #[test]
        fn it_parses_into_seven() {
            let result: Result<Rank, ()> = '7'.try_into();

            assert_eq!(result.unwrap(), Rank::Seven);
        }

        #[test]
        fn it_parses_into_six() {
            let result: Result<Rank, ()> = '6'.try_into();

            assert_eq!(result.unwrap(), Rank::Six);
        }

        #[test]
        fn it_parses_into_five() {
            let result: Result<Rank, ()> = '5'.try_into();

            assert_eq!(result.unwrap(), Rank::Five);
        }

        #[test]
        fn it_parses_into_four() {
            let result: Result<Rank, ()> = '4'.try_into();

            assert_eq!(result.unwrap(), Rank::Four);
        }

        #[test]
        fn it_parses_into_three() {
            let result: Result<Rank, ()> = '3'.try_into();

            assert_eq!(result.unwrap(), Rank::Trey);
        }

        #[test]
        fn it_parses_into_deuce() {
            let result: Result<Rank, ()> = '2'.try_into();

            assert_eq!(result.unwrap(), Rank::Deuce);
        }

        #[test]
        fn it_failes_parsing() {
            let result: Result<Rank, ()> = 'X'.try_into();

            assert_eq!(result.is_err(), true);
        }
    }

    mod from_str {
        use super::*;

        #[test]
        fn it_parses_into_ace() {
            assert_eq!("A".parse::<Rank>().unwrap(), Rank::Ace);
        }

        #[test]
        fn it_parses_into_king() {
            assert_eq!("K".parse::<Rank>().unwrap(), Rank::King);
        }

        #[test]
        fn it_parses_into_queen() {
            assert_eq!("Q".parse::<Rank>().unwrap(), Rank::Queen);
        }

        #[test]
        fn it_parses_into_jack() {
            assert_eq!("J".parse::<Rank>().unwrap(), Rank::Jack);
        }

        #[test]
        fn it_parses_into_ten() {
            assert_eq!("T".parse::<Rank>().unwrap(), Rank::Ten);
        }

        #[test]
        fn it_parses_into_nine() {
            assert_eq!("9".parse::<Rank>().unwrap(), Rank::Nine);
        }

        #[test]
        fn it_parses_into_eight() {
            assert_eq!("8".parse::<Rank>().unwrap(), Rank::Eight);
        }

        #[test]
        fn it_parses_into_seven() {
            assert_eq!("7".parse::<Rank>().unwrap(), Rank::Seven);
        }

        #[test]
        fn it_parses_into_six() {
            assert_eq!("6".parse::<Rank>().unwrap(), Rank::Six);
        }

        #[test]
        fn it_parses_into_five() {
            assert_eq!("5".parse::<Rank>().unwrap(), Rank::Five);
        }

        #[test]
        fn it_parses_into_four() {
            assert_eq!("4".parse::<Rank>().unwrap(), Rank::Four);
        }

        #[test]
        fn it_parses_into_three() {
            assert_eq!("3".parse::<Rank>().unwrap(), Rank::Trey);
        }

        #[test]
        fn it_parses_into_deuce() {
            assert_eq!("2".parse::<Rank>().unwrap(), Rank::Deuce);
        }

        #[test]
        fn it_failes_parsing() {
            assert_eq!("X".parse::<Rank>().is_err(), true);
        }
    }

    mod u8_from_rank {
        use super::*;

        #[test]
        fn it_parses_into_0() {
            let num: u8 = Rank::Ace.into();

            assert_eq!(num, 0);
        }

        #[test]
        fn it_parses_into_1() {
            let num: u8 = Rank::King.into();

            assert_eq!(num, 1);
        }

        #[test]
        fn it_parses_into_2() {
            let num: u8 = Rank::Queen.into();

            assert_eq!(num, 2);
        }

        #[test]
        fn it_parses_into_3() {
            let num: u8 = Rank::Jack.into();

            assert_eq!(num, 3);
        }

        #[test]
        fn it_parses_into_4() {
            let num: u8 = Rank::Ten.into();

            assert_eq!(num, 4);
        }

        #[test]
        fn it_parses_into_5() {
            let num: u8 = Rank::Nine.into();

            assert_eq!(num, 5);
        }

        #[test]
        fn it_parses_into_6() {
            let num: u8 = Rank::Eight.into();

            assert_eq!(num, 6);
        }

        #[test]
        fn it_parses_into_7() {
            let num: u8 = Rank::Seven.into();

            assert_eq!(num, 7);
        }

        #[test]
        fn it_parses_into_8() {
            let num: u8 = Rank::Six.into();

            assert_eq!(num, 8);
        }

        #[test]
        fn it_parses_into_9() {
            let num: u8 = Rank::Five.into();

            assert_eq!(num, 9);
        }

        #[test]
        fn it_parses_into_10() {
            let num: u8 = Rank::Four.into();

            assert_eq!(num, 10);
        }

        #[test]
        fn it_parses_into_11() {
            let num: u8 = Rank::Trey.into();

            assert_eq!(num, 11);
        }

        #[test]
        fn it_parses_into_12() {
            let num: u8 = Rank::Deuce.into();

            assert_eq!(num, 12);
        }
    }

    mod char_from_rank {
        use super::*;

        #[test]
        fn it_parses_into_a() {
            let c: char = Rank::Ace.into();

            assert_eq!(c, 'A');
        }

        #[test]
        fn it_parses_into_k() {
            let c: char = Rank::King.into();

            assert_eq!(c, 'K');
        }

        #[test]
        fn it_parses_into_q() {
            let c: char = Rank::Queen.into();

            assert_eq!(c, 'Q');
        }

        #[test]
        fn it_parses_into_j() {
            let c: char = Rank::Jack.into();

            assert_eq!(c, 'J');
        }

        #[test]
        fn it_parses_into_t() {
            let c: char = Rank::Ten.into();

            assert_eq!(c, 'T');
        }

        #[test]
        fn it_parses_into_9() {
            let c: char = Rank::Nine.into();

            assert_eq!(c, '9');
        }

        #[test]
        fn it_parses_into_8() {
            let c: char = Rank::Eight.into();

            assert_eq!(c, '8');
        }

        #[test]
        fn it_parses_into_7() {
            let c: char = Rank::Seven.into();

            assert_eq!(c, '7');
        }

        #[test]
        fn it_parses_into_6() {
            let c: char = Rank::Six.into();

            assert_eq!(c, '6');
        }

        #[test]
        fn it_parses_into_5() {
            let c: char = Rank::Five.into();

            assert_eq!(c, '5');
        }

        #[test]
        fn it_parses_into_4() {
            let c: char = Rank::Four.into();

            assert_eq!(c, '4');
        }

        #[test]
        fn it_parses_into_3() {
            let c: char = Rank::Trey.into();

            assert_eq!(c, '3');
        }

        #[test]
        fn it_parses_into_2() {
            let c: char = Rank::Deuce.into();

            assert_eq!(c, '2');
        }
    }
}
