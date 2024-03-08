use std::{collections::HashMap, str::FromStr};

use criterion::{criterion_group, criterion_main, Criterion};
use espada::card_set::CardSet;
use espada::evaluator::postflop_exhaustive::PostflopExhaustiveEvaluator;
use espada::hand_range::HandRange;

fn evaluate() {
    let board = CardSet::from_str("Ks8d2h").unwrap();
    let players = vec![
        HandRange::from_str("TT+").unwrap(),
        HandRange::from_str("A8s+").unwrap(),
    ];

    println!("{} vs {} on {}", players[0], players[1], board);

    let evaluator = PostflopExhaustiveEvaluator::new(&board, &players);
    let mut player_results = vec![HashMap::new(); players.len()];
    let mut materialized: u64 = 0;

    for (player_index, player) in players.iter().enumerate() {
        for (card_pair, _) in player {
            player_results[player_index].insert(*card_pair, (0.0, 0));
        }
    }

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
                    .0 += 1.0 / showdown.winner_len() as f64;
            }
        }

        materialized += 1;
    }

    println!("materialized: {}", materialized);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("small", |b| b.iter(|| evaluate()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
