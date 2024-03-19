mod scope;

use crate::scope::calculate_scopes;
use espada::card::Card;
use espada::evaluator::FlopExhaustiveEvaluator;
use espada::hand_range::HandRange;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut board = [None; 5];

    for i in 0..args[1].len() / 2 {
        let card: Card = args[1][i * 2..i * 2 + 2].parse().unwrap();

        board[i] = Some(card);
    }

    let mut players = vec![];

    for p in &args[2..] {
        let hand_range: HandRange = p.parse().unwrap();

        players.push(hand_range);
    }

    println!("board: {:?}", board);

    let mut space: u64 = 1176;

    for (player_index, player) in players.iter().enumerate() {
        println!("player[{}]: {}", player_index, player);

        space *= player.card_pairs().len() as u64;
    }

    println!("space: {} patterns", space);

    let scopes = calculate_scopes(num_cpus::get() as u32 - 1);
    let board_arc = std::sync::Arc::new(board);
    let players_arc = std::sync::Arc::new(players);

    let instant = std::time::Instant::now();

    let mut handles = vec![];

    for scope in scopes {
        let board = board_arc.clone();
        let players = players_arc.clone();

        let handle = std::thread::spawn(move || {
            let mut player_results = vec![HashMap::new(); players.len()];
            let mut materialized: u64 = 0;

            for (player_index, player) in players.iter().enumerate() {
                for (card_pair, _) in player {
                    player_results[player_index].insert(*card_pair, (0.0, 0));
                }
            }

            let mut evaluator = FlopExhaustiveEvaluator::new(&board, &players);
            evaluator.scope(
                scope.turn_from,
                scope.river_from,
                scope.turn_to,
                scope.river_to,
            );

            for showdown in evaluator {
                for (player_index, player) in showdown.players().into_iter().enumerate() {
                    player_results[player_index]
                        .get_mut(&player.hole_cards())
                        .unwrap()
                        .1 += 1;

                    if player.is_winner() {
                        player_results[player_index]
                            .get_mut(&player.hole_cards())
                            .unwrap()
                            .0 +=
                            1.0 / showdown.winner_len() as f64 * showdown.probability() as f64;
                    }
                }

                materialized += 1;
            }

            (player_results, materialized)
        });

        handles.push(handle);
    }

    let mut player_results = vec![HashMap::new(); players_arc.len()];
    let mut materialized: u64 = 0;

    for (player_index, player) in players_arc.iter().enumerate() {
        for (card_pair, _) in player {
            player_results[player_index].insert(*card_pair, (0.0, 0));
        }
    }

    for handle in handles {
        if let Ok((cluster_player_results, cluster_materialized)) = handle.join() {
            for (player_index, player_result) in cluster_player_results.into_iter().enumerate() {
                for (cards, (wins, materialized)) in player_result {
                    let result_entry = player_results[player_index].get_mut(&cards).unwrap();

                    result_entry.0 += wins;
                    result_entry.1 += materialized;
                }
            }

            materialized += cluster_materialized;
        }
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
