use super::Suit;

// NOTE:
// ideally Rank struct should implement Step trait and achieve the same job but
// as of March 2024, Step trait is still marked as unstable. So this is only the
// decent workaround to do the equivalent thing for the time being.
// ref. https://github.com/rust-lang/rust/issues/42168

pub struct SuitRange {
    start: usize,
    end: usize,
    inclusive: bool,
}

impl SuitRange {
    pub fn new(start: Suit, end: Suit) -> Self {
        Self {
            start: u8::from(start).into(),
            end: u8::from(end).into(),
            inclusive: false,
        }
    }

    pub fn inclusive(start: Suit, end: Suit) -> Self {
        Self {
            start: u8::from(start).into(),
            end: u8::from(end).into(),
            inclusive: true,
        }
    }

    pub fn all() -> Self {
        Self {
            start: 0,
            end: SUITS.len(),
            inclusive: false,
        }
    }
}

const SUITS: [Suit; 4] = [Suit::Spade, Suit::Heart, Suit::Diamond, Suit::Club];

impl IntoIterator for SuitRange {
    type Item = Suit;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let vec: Vec<Suit> = match self.inclusive {
            true => SUITS[self.start..=self.end].into(),
            false => SUITS[self.start..self.end].into(),
        };

        vec.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_iterable() {
        assert_eq!(
            format!("{:?}", SuitRange::new(Suit::Spade, Suit::Club).into_iter()),
            "IntoIter([Spade, Heart, Diamond])"
        );
        assert_eq!(
            format!(
                "{:?}",
                SuitRange::new(Suit::Heart, Suit::Diamond).into_iter()
            ),
            "IntoIter([Heart])"
        );
        assert_eq!(
            format!(
                "{:?}",
                SuitRange::new(Suit::Diamond, Suit::Diamond).into_iter()
            ),
            "IntoIter([])"
        );
        assert_eq!(
            format!(
                "{:?}",
                SuitRange::inclusive(Suit::Spade, Suit::Club).into_iter()
            ),
            "IntoIter([Spade, Heart, Diamond, Club])"
        );
        assert_eq!(
            format!(
                "{:?}",
                SuitRange::inclusive(Suit::Heart, Suit::Diamond).into_iter()
            ),
            "IntoIter([Heart, Diamond])"
        );
        assert_eq!(
            format!(
                "{:?}",
                SuitRange::inclusive(Suit::Diamond, Suit::Diamond).into_iter()
            ),
            "IntoIter([Diamond])"
        );
        assert_eq!(
            format!("{:?}", SuitRange::all().into_iter()),
            "IntoIter([Spade, Heart, Diamond, Club])"
        );
    }
}
