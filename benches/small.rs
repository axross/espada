use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;

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

    let evaluator = PostflopExhaustiveEvaluator::new(&board, &players);
    let mut player_results = vec![HashMap::new(); players.len()];

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
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("small", |b| b.iter(|| evaluate()));
}

criterion_group! {
    name = small;
    config = Criterion::default()
        .sample_size(50)
        .warm_up_time(Duration::from_secs(5))
        .measurement_time(Duration::from_secs(60));
    targets = criterion_benchmark
}

criterion_main!(small);