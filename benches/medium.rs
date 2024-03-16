use criterion::{criterion_group, criterion_main, Criterion};
use espada::card::Card;
use espada::evaluator::FlopExhaustiveEvaluator;
use std::collections::HashMap;
use std::time::Duration;

fn evaluate() {
    let board: Vec<Card> = ["Js", "8h", "2d"]
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();
    let players = vec![
        "JJ+,TT:0.5,99:0.25,AQs+,AJs:0.5,A7s-A6s:0.25,A5s:0.75,A4s-A2s:0.25,AKo,AQo:0.5,AJo-A9o:0.25,KQs:0.75,KJs-KTs:0.5,K9s-K7s:0.25,K6s:0.5,K5s-K3s:0.25,KQo:0.25,KJo:0.5,KTo:0.25,QTs+:0.5,Q9s-Q8s:0.25,Q6s-Q4s:0.25,QTo+:0.25,JTs,J9s-J7s:0.25,T9s:0.25,87s:0.25,76s:0.25,65s:0.5,54s:0.25".parse().unwrap(),
        "QQ:0.25,JJ-99:0.5,88-22:0.25,AQs:0.75,AJs:0.25,ATs:0.5,A5s-A4s:0.25,KQs:0.25,KJs:0.5,KTs:0.25,QJs:0.25,JTs:0.25,T9s:0.25,98s:0.25,87s:0.25,76s:0.25,65s:0.25,54s:0.25+".parse().unwrap(),
    ];

    let evaluator = FlopExhaustiveEvaluator::new(board, &players);
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
    c.bench_function("small", |b| b.iter(|| evaluate()));
}

criterion_group! {
    name = medium;
    config = Criterion::default()
        .sample_size(20)
        .warm_up_time(Duration::from_secs(10))
        .measurement_time(Duration::from_secs(120));
    targets = criterion_benchmark
}

criterion_main!(medium);
