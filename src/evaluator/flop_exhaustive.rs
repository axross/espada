use std::collections::HashSet;

use super::showdown::Showdown;
use crate::card::{Card, RankRange, SuitRange};
use crate::hand_range::{CardPair, HandRange};

pub struct FlopExhaustiveEvaluator<'p> {
    board: Vec<Card>,
    players: &'p Vec<HandRange>,
    turn_from: u8,
    river_from: u8,
    turn_to: u8,
    river_to: u8,
}

impl<'p> FlopExhaustiveEvaluator<'p> {
    pub fn new(
        original_board: impl IntoIterator<Item = Card>,
        players: &'p Vec<HandRange>,
    ) -> Self {
        let mut board: Vec<Card> = Vec::with_capacity(7);
        let mut len = 0;

        for card in original_board {
            len += 1;

            if len > 5 {
                panic!("board length must be less than or equal to 5.");
            }

            board.push(card);
        }

        Self {
            board,
            players,
            turn_from: 0,
            river_from: 1,
            turn_to: 48,
            river_to: 49,
        }
    }

    pub fn scope(&mut self, turn_from: u8, river_from: u8, turn_to: u8, river_to: u8) {
        debug_assert!(turn_from <= turn_to);
        debug_assert!(turn_from < river_from);
        debug_assert!(turn_to < river_to);

        self.turn_from = turn_from;
        self.river_from = river_from;
        self.turn_to = turn_to;
        self.river_to = river_to;
    }
}

impl<'p> IntoIterator for FlopExhaustiveEvaluator<'p> {
    type Item = Showdown;
    type IntoIter = FlopExhaustiveEvaluatorIterator;

    fn into_iter(self) -> Self::IntoIter {
        FlopExhaustiveEvaluatorIterator::new(&self)
    }
}

pub struct FlopExhaustiveEvaluatorIterator {
    turn_to: u8,
    river_to: u8,
    player_entries: Vec<Vec<(CardPair, f32)>>,
    current_deck: [Card; 49],
    current_board: [Option<Card>; 5],
    current_used_cards: HashSet<Card>,
    current_turn_index: u8,
    current_river_index: u8,
    current_player_indexes: Vec<u8>,
}

impl FlopExhaustiveEvaluatorIterator {
    fn new(evaluator: &FlopExhaustiveEvaluator) -> Self {
        let mut player_entries = vec![vec![]; evaluator.players.len()];

        for (player_index, player) in evaluator.players.iter().enumerate() {
            for (card_pair, probability) in player.card_pairs() {
                player_entries[player_index].push((*card_pair, *probability));
            }
        }

        let mut current_deck = Vec::with_capacity(52);

        for rank in RankRange::all() {
            for suit in SuitRange::all() {
                let card = Card::new(rank, suit);

                if evaluator.board.iter().all(|c| *c != card) {
                    current_deck.push(card);
                }
            }
        }

        let mut current_board = [None; 5];

        for (i, card) in evaluator.board.iter().enumerate() {
            current_board[i] = Some(*card);
        }

        Self {
            turn_to: evaluator.turn_to,
            river_to: evaluator.river_to,
            player_entries,
            current_deck: current_deck.try_into().unwrap(),
            current_board,
            current_used_cards: HashSet::with_capacity(2 + evaluator.players.len() * 2),
            current_turn_index: evaluator.turn_from,
            current_river_index: evaluator.river_from,
            current_player_indexes: vec![0; evaluator.players.len()],
        }
    }
}

impl Iterator for FlopExhaustiveEvaluatorIterator {
    type Item = Showdown;

    fn next(&mut self) -> Option<Showdown> {
        if self.current_turn_index >= self.turn_to && self.current_river_index >= self.river_to {
            return None;
        }

        let turn = self.current_deck[self.current_turn_index as usize];
        let river = self.current_deck[self.current_river_index as usize];

        self.current_board[3] = Some(turn);
        self.current_board[4] = Some(river);

        self.current_used_cards.insert(turn);
        self.current_used_cards.insert(river);

        // self.current_deck.remove(self.current_river_index as usize);
        // self.current_deck.remove(self.current_turn_index as usize);

        let mut player_card_pairs = vec![];
        let mut probability: f32 = 1.0;

        let mut is_materialized = true;

        for (player_index, player_entry) in self.player_entries.iter().enumerate() {
            let entry = player_entry[self.current_player_indexes[player_index] as usize];

            if self.current_used_cards.contains(&entry.0[0])
                || self.current_used_cards.contains(&entry.0[1])
            {
                is_materialized = false;
            }

            player_card_pairs.push(entry.0);
            probability *= entry.1;
        }

        let mut showdown = None;

        if is_materialized {
            showdown = Showdown::new(
                player_card_pairs,
                [
                    self.current_board[0].unwrap(),
                    self.current_board[1].unwrap(),
                    self.current_board[2].unwrap(),
                    self.current_board[3].unwrap(),
                    self.current_board[4].unwrap(),
                ],
                probability,
            );
        }

        let mut player_index_to_increment = None;

        for i in 0..self.current_player_indexes.len() {
            let ri = self.current_player_indexes.len() - i - 1;

            if self.current_player_indexes[ri] < self.player_entries[ri].len() as u8 - 1 {
                player_index_to_increment = Some(ri);

                break;
            }
        }

        self.current_board[3] = None;
        self.current_board[4] = None;

        self.current_used_cards.clear();

        if let Some(player_index_to_increment) = player_index_to_increment {
            self.current_player_indexes[player_index_to_increment] += 1;

            for i in (player_index_to_increment + 1)..self.current_player_indexes.len() {
                self.current_player_indexes[i] = 0;
            }

            return showdown.or_else(|| self.next());
        }

        if self.current_river_index < 48 {
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
mod tests {
    use super::*;

    mod iterator {
        use insta::assert_debug_snapshot;

        use super::*;
        use std::str::FromStr;

        #[test]
        fn it_iterates_scoped_from_0_1_to_2_25() {
            let board = ["2h", "2d", "2c"]
                .into_iter()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<Card>>();
            let players = vec![
                HandRange::from_str("4s3h:1").unwrap(),
                HandRange::from_str("4d3c:1").unwrap(),
            ];

            let mut evaluator = FlopExhaustiveEvaluator::new(board, &players);
            evaluator.scope(0, 1, 2, 25);

            let result: Vec<Showdown> = evaluator.into_iter().collect();

            assert_eq!(result.len(), (48 - 4) + (47 - 4) + 22);
            assert_debug_snapshot!(result);
        }

        #[test]
        fn it_iterates_scoped_from_10_43_to_14_18() {
            let board = ["Jh", "9d", "3c"]
                .into_iter()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<Card>>();
            let players = vec![
                HandRange::from_str("As4h:1").unwrap(),
                HandRange::from_str("Td8c:1").unwrap(),
            ];

            let mut evaluator = FlopExhaustiveEvaluator::new(board, &players);
            evaluator.scope(10, 43, 14, 18);

            let result: Vec<Showdown> = evaluator.into_iter().collect();

            assert_eq!(result.len(), 5 + (37 - 3) + (36 - 3) + (35 - 3) + 3);
            assert_debug_snapshot!(result);
        }

        #[test]
        fn it_iterates_scoped_from_32_48_to_47_49() {
            let board = ["Kh", "Kd", "Kc"]
                .into_iter()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<Card>>();
            let players = vec![
                HandRange::from_str("As2s:1").unwrap(),
                HandRange::from_str("JdJc:1").unwrap(),
            ];

            let mut evaluator = FlopExhaustiveEvaluator::new(board, &players);
            evaluator.scope(32, 48, 47, 49);

            let result: Vec<Showdown> = evaluator.into_iter().collect();

            assert_eq!(
                result.len(),
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
            assert_debug_snapshot!(result);
        }
    }
}
