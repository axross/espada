# ♠️ espada

Texas Hold'em poker odds evaluator.

![https://crates.io/crates/espada](https://img.shields.io/crates/v/espada) ![Recent Downloads](https://img.shields.io/crates/dr/espada) ![License](https://img.shields.io/crates/l/espada/0.1.0)

- [API Docs](https://docs.rs/espada/latest/espada/)
- [Examples](/examples)

## Quickstart

Add this to Cargo.toml:

```toml
[dependencies]
espada = "0.3.1"
```

In your `main.rs` or `lib.rs`, you can use this as:

```rust
use espada::evaluator::FlopExhaustiveEvaluator;
use espada::hand_range::HandRange;

fn main() {
    let board = [
        Some(Card::new(Rank::Queen, Suit::Spade)),
        Some(Card::new(Rank::Eight, Suit::Diamond)),
        Some(Card::new(Rank::Deuce, Suit::Heart)),
        None,
        None
    ];

    let players = vec!["JJ+".parse().unwrap(), "A2s+".parse().unwrap()];

    let evaluator = FlopExhaustiveEvaluator::new(&board, &players);

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

## License

[Here](/LICENSE.txt)
