use super::Rank;

// NOTE:
// ideally Rank struct should implement Step trait and achieve the same job but
// as of March 2024, Step trait is still marked as unstable. So this is only the
// decent workaround to do the equivalent thing for the time being.
// ref. https://github.com/rust-lang/rust/issues/42168

pub struct RankRange {
    start: usize,
    end: usize,
    inclusive: bool,
}

impl RankRange {
    pub fn new(start: Rank, end: Rank) -> RankRange {
        Self {
            start: start.into(),
            end: end.into(),
            inclusive: false,
        }
    }

    pub fn inclusive(start: Rank, end: Rank) -> RankRange {
        Self {
            start: start.into(),
            end: end.into(),
            inclusive: true,
        }
    }

    pub fn all() -> Self {
        Self {
            start: 0,
            end: RANKS.len(),
            inclusive: false,
        }
    }
}

const RANKS: [Rank; 13] = [
    Rank::Ace,
    Rank::King,
    Rank::Queen,
    Rank::Jack,
    Rank::Ten,
    Rank::Nine,
    Rank::Eight,
    Rank::Seven,
    Rank::Six,
    Rank::Five,
    Rank::Four,
    Rank::Trey,
    Rank::Deuce,
];

impl IntoIterator for RankRange {
    type Item = Rank;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let vec: Vec<Rank> = match self.inclusive {
            true => RANKS[self.start..=self.end].into(),
            false => RANKS[self.start..self.end].into(),
        };

        vec.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_iterable_from_ace_until_trey() {
        assert_eq!(
            RankRange::new(Rank::Ace, Rank::Deuce)
                .into_iter()
                .collect::<Vec<_>>(),
            vec![
                Rank::Ace,
                Rank::King,
                Rank::Queen,
                Rank::Jack,
                Rank::Ten,
                Rank::Nine,
                Rank::Eight,
                Rank::Seven,
                Rank::Six,
                Rank::Five,
                Rank::Four,
                Rank::Trey,
            ]
        );
    }

    #[test]
    fn it_is_iterable_from_jack_until_six() {
        assert_eq!(
            RankRange::new(Rank::Jack, Rank::Six)
                .into_iter()
                .collect::<Vec<_>>(),
            vec![Rank::Jack, Rank::Ten, Rank::Nine, Rank::Eight, Rank::Seven,]
        );
    }

    #[test]
    fn it_is_iterable_from_ace_to_two() {
        assert_eq!(
            RankRange::inclusive(Rank::Ace, Rank::Deuce)
                .into_iter()
                .collect::<Vec<_>>(),
            vec![
                Rank::Ace,
                Rank::King,
                Rank::Queen,
                Rank::Jack,
                Rank::Ten,
                Rank::Nine,
                Rank::Eight,
                Rank::Seven,
                Rank::Six,
                Rank::Five,
                Rank::Four,
                Rank::Trey,
                Rank::Deuce,
            ]
        );
    }

    #[test]
    fn it_is_iterable_from_jack_to_six() {
        assert_eq!(
            RankRange::inclusive(Rank::Jack, Rank::Six)
                .into_iter()
                .collect::<Vec<_>>(),
            vec![
                Rank::Jack,
                Rank::Ten,
                Rank::Nine,
                Rank::Eight,
                Rank::Seven,
                Rank::Six,
            ]
        );
    }

    #[test]
    fn it_is_iterable_through_all_ranks() {
        assert_eq!(
            RankRange::all().into_iter().collect::<Vec<_>>(),
            vec![
                Rank::Ace,
                Rank::King,
                Rank::Queen,
                Rank::Jack,
                Rank::Ten,
                Rank::Nine,
                Rank::Eight,
                Rank::Seven,
                Rank::Six,
                Rank::Five,
                Rank::Four,
                Rank::Trey,
                Rank::Deuce,
            ]
        );
    }
}
