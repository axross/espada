use core::str::FromStr;
use espada::card_set::CardSet;
use espada::evaluator::postflop_exhaustive::PostflopExhaustiveEvaluator;
use espada::hand_range::HandRange;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let board = CardSet::from_str(&args[1]).unwrap();
    let mut players = vec![];

    for p in &args[2..] {
        players.push(HandRange::from_str(p).unwrap())
    }

    println!("board: {}", board);

    let mut player_results = vec![HashMap::new(); players.len()];
    let mut materialized: u64 = 0;

    let mut space: u64 = 1176;

    for (player_index, player) in players.iter().enumerate() {
        println!("player[{}]: {}", player_index, player);

        for (card_pair, _) in player {
            player_results[player_index].insert(*card_pair, (0.0, 0));
        }

        space *= player.card_pairs().len() as u64;
    }

    println!("space: {} patterns", space);

    let evaluator = PostflopExhaustiveEvaluator::new(&board, &players);

    let instant = std::time::Instant::now();

    for showdown in evaluator {
        for (player_index, player) in showdown.players().into_iter().enumerate() {
            player_results[player_index]
                .get_mut(&player.cards())
                .unwrap()
                .1 += 1;

            if player.is_winner() {
                player_results[player_index]
                    .get_mut(&player.cards())
                    .unwrap()
                    .0 += 1.0 / showdown.winner_len() as f64 * showdown.probability() as f64;
            }
        }

        materialized += 1;
    }

    let finished = instant.elapsed();

    println!("elapsed: {:03} ms", finished.as_millis(),);

    println!("materialized: {} partterns", materialized);

    for player_result in player_results {
        for (cards, (wins, materialized)) in player_result {
            if materialized == 0 {
                continue;
            }

            println!("{}: {:.3}%", cards, wins / materialized as f64 * 100.0);
        }
    }
}
