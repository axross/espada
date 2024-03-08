use crate::card_set::CardSet;
use crate::hand_range::HandRange;
use crate::showdown::Showdown;

pub struct PostflopExhaustiveEvaluator<'b, 'p> {
    board: &'b CardSet,
    players: &'p Vec<HandRange>,
    turn_from: u8,
    river_from: u8,
    turn_to: u8,
    river_to: u8,
}

impl<'b, 'p> PostflopExhaustiveEvaluator<'b, 'p> {
    pub fn new(board: &'b CardSet, players: &'p Vec<HandRange>) -> Self {
        Self {
            board,
            players,
            turn_from: 0,
            river_from: 1,
            turn_to: 48,
            river_to: 49,
        }
    }

    pub fn scoped(
        board: &'b CardSet,
        players: &'p Vec<HandRange>,
        turn_from: u8,
        river_from: u8,
        turn_to: u8,
        river_to: u8,
    ) -> Self {
        debug_assert!(turn_from <= turn_to);
        debug_assert!(turn_from < river_from);
        debug_assert!(turn_to < river_to);

        Self {
            board,
            players,
            turn_from,
            river_from,
            turn_to,
            river_to,
        }
    }
}

impl<'b, 'p> IntoIterator for PostflopExhaustiveEvaluator<'b, 'p> {
    type Item = Showdown;
    type IntoIter = PostflopExhaustiveEvaluatorIterator;

    fn into_iter(self) -> Self::IntoIter {
        PostflopExhaustiveEvaluatorIterator::new(&self)
    }
}

pub struct PostflopExhaustiveEvaluatorIterator {
    turn_to: u8,
    river_to: u8,
    player_entries: Vec<Vec<(CardSet, f32)>>,
    current_deck: CardSet,
    current_board: CardSet,
    current_turn_index: u8,
    current_river_index: u8,
    current_player_indexes: Vec<u8>,
}

impl PostflopExhaustiveEvaluatorIterator {
    fn new(evaluator: &PostflopExhaustiveEvaluator) -> Self {
        let mut player_entries = vec![vec![]; evaluator.players.len()];

        for (player_index, player) in evaluator.players.iter().enumerate() {
            for (card_pair, probability) in player.card_pairs() {
                player_entries[player_index].push((*card_pair, *probability));
            }
        }

        let mut current_deck = CardSet::full();
        current_deck.remove_all(evaluator.board);

        Self {
            turn_to: evaluator.turn_to,
            river_to: evaluator.river_to,
            player_entries,
            current_deck,
            current_board: evaluator.board.clone(),
            current_turn_index: evaluator.turn_from,
            current_river_index: evaluator.river_from,
            current_player_indexes: vec![0; evaluator.players.len()],
        }
    }
}

impl Iterator for PostflopExhaustiveEvaluatorIterator {
    type Item = Showdown;

    fn next(&mut self) -> Option<Showdown> {
        if self.current_turn_index >= self.turn_to && self.current_river_index >= self.river_to {
            return None;
        }

        let turn = self
            .current_deck
            .into_iter()
            .nth(self.current_turn_index as usize)
            .unwrap();

        let river: crate::card::Card = self
            .current_deck
            .into_iter()
            .nth(self.current_river_index as usize)
            .unwrap();

        self.current_board.insert(&turn);
        self.current_board.insert(&river);

        self.current_deck.remove(&turn);
        self.current_deck.remove(&river);

        let mut player_card_pairs = vec![];
        let mut probability: f32 = 1.0;

        for (player_index, player_entry) in self.player_entries.iter().enumerate() {
            let entry = player_entry[self.current_player_indexes[player_index] as usize];

            player_card_pairs.push(entry.0);
            probability *= entry.1;
        }

        let showdown = Showdown::new(player_card_pairs, self.current_board, probability);

        let mut player_index_to_increment = None;

        for i in 0..self.current_player_indexes.len() {
            let ri = self.current_player_indexes.len() - i - 1;

            if self.current_player_indexes[ri] < self.player_entries[ri].len() as u8 - 1 {
                player_index_to_increment = Some(ri);

                break;
            }
        }

        self.current_board.remove(&river);
        self.current_board.remove(&turn);

        self.current_deck.insert(&river);
        self.current_deck.insert(&turn);

        if let Some(player_index_to_increment) = player_index_to_increment {
            self.current_player_indexes[player_index_to_increment] += 1;

            for i in (player_index_to_increment + 1)..self.current_player_indexes.len() {
                self.current_player_indexes[i] = 0;
            }

            return showdown.or_else(|| self.next());
        }

        if self.current_river_index < (49 - 1) {
            self.current_river_index += 1;
            self.current_player_indexes.fill(0);

            return showdown.or_else(|| self.next());
        }

        self.current_turn_index += 1;
        self.current_river_index = self.current_turn_index + 1;
        self.current_player_indexes.fill(0);

        showdown.or_else(|| self.next())
    }
}

#[cfg(test)]
mod tests_iterator {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn it_iterates_scoped_from_0_1_to_2_25() {
        let board: CardSet = CardSet::from_str("2h2d2c").unwrap();
        let players = vec![
            HandRange::from_str("4s3h:1").unwrap(),
            HandRange::from_str("4d3c:1").unwrap(),
        ];

        let evaluator = PostflopExhaustiveEvaluator::scoped(&board, &players, 0, 1, 2, 25);
        let iterator = evaluator.into_iter();

        let mut times = 0;

        for _ in iterator {
            times += 1;
        }

        assert_eq!(times, (48 - 4) + (47 - 4) + 22);
    }

    #[test]
    fn it_iterates_scoped_from_10_43_to_14_18() {
        let board: CardSet = CardSet::from_str("Jh9d3c").unwrap();
        let players = vec![
            HandRange::from_str("As4h:1").unwrap(),
            HandRange::from_str("Td8c:1").unwrap(),
        ];

        let evaluator = PostflopExhaustiveEvaluator::scoped(&board, &players, 10, 43, 14, 18);
        let iterator = evaluator.into_iter();

        let mut times = 0;

        for _ in iterator {
            times += 1;
        }

        assert_eq!(times, 5 + (37 - 3) + (36 - 3) + (35 - 3) + 3);
    }

    #[test]
    fn it_iterates_scoped_from_32_48_to_47_49() {
        let board: CardSet = CardSet::from_str("KhKdKc").unwrap();
        let players = vec![
            HandRange::from_str("As2s:1").unwrap(),
            HandRange::from_str("JdJc:1").unwrap(),
        ];

        let evaluator = PostflopExhaustiveEvaluator::scoped(&board, &players, 32, 48, 47, 49);
        let iterator = evaluator.into_iter();

        let mut times = 0;

        for _ in iterator {
            times += 1;
        }

        assert_eq!(
            times,
            1 + (15 - 1)
                + (14 - 1)
                + (13 - 1)
                + (12 - 1)
                + (11 - 1)
                + (10 - 1)
                + (9 - 1)
                + (8 - 1)
                + (7 - 1)
                + (6 - 1)
                + (5 - 1)
                + 3
                + 2
                + 1
        );
    }
}
