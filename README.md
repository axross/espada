# ♠️ espada

Texas Hold'em poker odds evaluator.

## Quickstart

Add this to Cargo.toml:

```toml
[dependencies]
espada = "0.1"
```

In your `main.rs` or `lib.rs`, you can use this as:

```rust
use std::str::FromStr;

use espada::card_set::CardSet;
use espada::evaluator::postflop_exhaustive::PostflopExhaustiveEvaluator;
use espada::hand_range::HandRange;

fn main() {
    let board = CardSet::from_str("Qs8d2h").unwrap();

    let players = vec![
        HandRange::from_str("JJ+").unwrap(),
        HandRange::from_str("A2s+").unwrap(),
    ];

    let evaluator = PostflopExhaustiveEvaluator::new(&board, &players);

    let mut materialized = 0_u32;
    let mut wins = vec![0_f32; players.len()];

    for showdown in evaluator {
        for (player_index, player) in showdown.players().into_iter().enumerate() {
            if player.is_winner() {
                wins[player_index] += 1.0 / showdown.winner_len() as f32
            }
        }

        materialized += 1;
    }

    for (player_index, _) in players.into_iter().enumerate() {
        println!(
            "player {}: {:.2}% eq",
            player_index,
            wins[player_index] / materialized as f32 * 100.0
        );
    }
}
```