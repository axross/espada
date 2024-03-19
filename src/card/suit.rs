use core::fmt::{Display, Formatter};
use core::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        let c: char = self.into();

        c.to_string().fmt(f)
    }
}

impl TryFrom<&char> for Suit {
    type Error = ();

    fn try_from(c: &char) -> Result<Self, Self::Error> {
        match c {
            's' => Ok(Suit::Spade),
            'h' => Ok(Suit::Heart),
            'd' => Ok(Suit::Diamond),
            'c' => Ok(Suit::Club),
            _ => Err(()),
        }
    }
}

impl TryFrom<char> for Suit {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        (&c).try_into()
    }
}

impl FromStr for Suit {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Some(c) = value.chars().nth(0) {
            c.try_into()
        } else {
            Err(())
        }
    }
}

impl From<&Suit> for u8 {
    fn from(suit: &Suit) -> Self {
        match suit {
            Suit::Spade => 0,
            Suit::Heart => 1,
            Suit::Diamond => 2,
            Suit::Club => 3,
        }
    }
}

impl From<Suit> for u8 {
    fn from(suit: Suit) -> Self {
        u8::from(&suit)
    }
}

impl From<&Suit> for char {
    fn from(value: &Suit) -> Self {
        match value {
            Suit::Spade => 's',
            Suit::Heart => 'h',
            Suit::Diamond => 'd',
            Suit::Club => 'c',
        }
    }
}

impl From<Suit> for char {
    fn from(value: Suit) -> Self {
        char::from(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod display {
        use super::*;

        #[test]
        fn it_parses_into_s() {
            assert_eq!(Suit::Spade.to_string(), "s");
        }

        #[test]
        fn it_parses_into_k() {
            assert_eq!(Suit::Heart.to_string(), "h");
        }

        #[test]
        fn it_parses_into_q() {
            assert_eq!(Suit::Diamond.to_string(), "d");
        }

        #[test]
        fn it_parses_into_j() {
            assert_eq!(Suit::Club.to_string(), "c");
        }
    }

    mod try_from_char {
        use super::*;

        #[test]
        fn it_parses_into_spade() {
            let result: Result<Suit, ()> = 's'.try_into();

            assert_eq!(result.unwrap(), Suit::Spade);
        }

        #[test]
        fn it_parses_into_heart() {
            let result: Result<Suit, ()> = 'h'.try_into();

            assert_eq!(result.unwrap(), Suit::Heart);
        }

        #[test]
        fn it_parses_into_diamond() {
            let result: Result<Suit, ()> = 'd'.try_into();

            assert_eq!(result.unwrap(), Suit::Diamond);
        }

        #[test]
        fn it_parses_into_club() {
            let result: Result<Suit, ()> = 'c'.try_into();

            assert_eq!(result.unwrap(), Suit::Club);
        }

        #[test]
        fn it_failes_parsing() {
            let result: Result<Suit, ()> = 'X'.try_into();

            assert_eq!(result.is_err(), true);
        }
    }

    mod from_str {
        use super::*;

        #[test]
        fn it_parses_into_spade() {
            assert_eq!("s".parse::<Suit>().unwrap(), Suit::Spade);
        }

        #[test]
        fn it_parses_into_heart() {
            assert_eq!("h".parse::<Suit>().unwrap(), Suit::Heart);
        }

        #[test]
        fn it_parses_into_diamond() {
            assert_eq!("d".parse::<Suit>().unwrap(), Suit::Diamond);
        }

        #[test]
        fn it_parses_into_club() {
            assert_eq!("c".parse::<Suit>().unwrap(), Suit::Club);
        }

        #[test]
        fn it_failes_parsing() {
            assert_eq!("X".parse::<Suit>().is_err(), true);
        }
    }

    mod u8_from_rank {
        use super::*;

        #[test]
        fn it_parses_into_0() {
            let num: u8 = Suit::Spade.into();

            assert_eq!(num, 0);
        }

        #[test]
        fn it_parses_into_1() {
            let num: u8 = Suit::Heart.into();

            assert_eq!(num, 1);
        }

        #[test]
        fn it_parses_into_2() {
            let num: u8 = Suit::Diamond.into();

            assert_eq!(num, 2);
        }

        #[test]
        fn it_parses_into_3() {
            let num: u8 = Suit::Club.into();

            assert_eq!(num, 3);
        }
    }

    #[cfg(test)]
    mod char_from_rank {
        use super::*;

        #[test]
        fn it_parses_into_s() {
            let c: char = Suit::Spade.into();

            assert_eq!(c, 's');
        }

        #[test]
        fn it_parses_into_h() {
            let c: char = Suit::Heart.into();

            assert_eq!(c, 'h');
        }

        #[test]
        fn it_parses_into_d() {
            let c: char = Suit::Diamond.into();

            assert_eq!(c, 'd');
        }

        #[test]
        fn it_parses_into_c() {
            let c: char = Suit::Club.into();

            assert_eq!(c, 'c');
        }
    }
}
