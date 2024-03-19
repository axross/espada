use criterion::{criterion_group, criterion_main, Criterion};
use espada::card::{Card, Rank, Suit};
use espada::evaluator::FlopExhaustiveEvaluator;
use std::collections::HashMap;

fn evaluate() {
    let board = [
        Some(Card::new(Rank::King, Suit::Spade)),
        Some(Card::new(Rank::Eight, Suit::Diamond)),
        Some(Card::new(Rank::Deuce, Suit::Heart)),
        None,
        None,
    ];
    let players = vec!["TT+".parse().unwrap(), "A8s+".parse().unwrap()];

    let evaluator = FlopExhaustiveEvaluator::new(&board, &players);
    let mut player_results = vec![HashMap::new(); players.len()];

    for (player_index, player) in players.iter().enumerate() {
        for (card_pair, _) in player {
            player_results[player_index].insert(*card_pair, (0.0, 0));
        }
    }

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
                    .0 += 1.0 / showdown.winner_len() as f64;
            }
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("base", |b| b.iter(|| evaluate()));
}

criterion_group! {
    name = base;
    config = Criterion::default();
    targets = criterion_benchmark
}

criterion_main!(base);
