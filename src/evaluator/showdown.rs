use super::MadeHand;
use crate::card::Card;
use crate::hand_range::CardPair;

#[derive(Debug)]
pub struct Showdown {
    board: [Card; 5],
    players: Vec<ShowdownPlayer>,
    probability: f32,
}

impl Showdown {
    pub fn new(players: Vec<CardPair>, board: [Card; 5], probability: f32) -> Option<Showdown> {
        debug_assert!(board.len() == 5);

        let mut showdown_players = vec![];
        let mut strongest_index = u16::MAX;
        let mut winner_indexes = vec![];

        for (i, player) in players.into_iter().enumerate() {
            if board.contains(&player[0]) || board.contains(&player[1]) {
                return None;
            }

            let made_hand: MadeHand = [
                player[0], player[1], board[0], board[1], board[2], board[3], board[4],
            ]
            .into();
            let power_index = made_hand.power_index();

            let showdown_player = ShowdownPlayer {
                hole_cards: player,
                board: [board[0], board[1], board[2], board[3], board[4]],
                hand: made_hand,
                win: false,
            };

            if power_index <= strongest_index {
                if power_index < strongest_index {
                    strongest_index = power_index;
                    winner_indexes.clear();
                }

                winner_indexes.push(i);
            }

            showdown_players.push(showdown_player);
        }

        for (i, player) in showdown_players.iter_mut().enumerate() {
            if winner_indexes.contains(&i) {
                player.win = true;
            }
        }

        Some(Showdown {
            players: showdown_players,
            board,
            probability,
        })
    }

    pub fn board(&self) -> &[Card; 5] {
        &self.board
    }

    pub fn players(&self) -> &Vec<ShowdownPlayer> {
        &self.players
    }

    pub fn probability(&self) -> f32 {
        self.probability
    }

    pub fn winner_len(&self) -> u8 {
        let mut len = 0;

        for player in &self.players {
            if player.win {
                len += 1;
            }
        }

        len
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ShowdownPlayer {
    hole_cards: CardPair,
    board: [Card; 5],
    hand: MadeHand,
    win: bool,
}

impl ShowdownPlayer {
    pub fn hole_cards(&self) -> CardPair {
        self.hole_cards
    }

    pub fn board(&self) -> [Card; 5] {
        self.board
    }

    pub fn cards(&self) -> [Card; 7] {
        [
            self.board[0],
            self.board[1],
            self.board[2],
            self.board[3],
            self.board[4],
            self.hole_cards[0],
            self.hole_cards[1],
        ]
    }

    pub fn hand(&self) -> MadeHand {
        self.hand
    }

    pub fn is_winner(&self) -> bool {
        self.win
    }
}
