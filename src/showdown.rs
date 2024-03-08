use crate::card_set::CardSet;
use crate::made_hand::MadeHand;

#[derive(Debug)]
pub struct Showdown {
    board: CardSet,
    players: Vec<ShowdownPlayer>,
    probability: f32,
}

impl Showdown {
    pub fn new(players: Vec<CardSet>, board: CardSet, probability: f32) -> Option<Showdown> {
        debug_assert!(board.len() == 5);

        let mut showdown_players = vec![];
        let mut strongest_index = u16::MAX;
        let mut winner_indexes = vec![];

        for (i, player) in players.into_iter().enumerate() {
            debug_assert!(player.len() == 2);

            let mut player: CardSet = player.clone();

            if board.include_any(&player) {
                return None;
            }

            player.insert_all(&board);

            if player.len() != 7 {
                return None;
            }

            debug_assert!(player.len() == 7);

            let made_hand: MadeHand = player.into();
            let power_index = made_hand.power_index();

            player.remove_all(&board);

            let showdown_player = ShowdownPlayer {
                cards: player,
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

    pub fn board(&self) -> &CardSet {
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
    cards: CardSet,
    hand: MadeHand,
    win: bool,
}

impl ShowdownPlayer {
    pub fn cards(&self) -> CardSet {
        self.cards
    }

    pub fn hand(&self) -> MadeHand {
        self.hand
    }

    pub fn is_winner(&self) -> bool {
        self.win
    }
}
