//! Times every evaluator on the same positions in 4, 3 and 2 player games
//! with Middle Kingdom and Harmony.
//!   cargo run --bin time_evaluators --release [games per player count]
//!
//! The positions are every decision of games played by claude bots, dealt
//! from a fixed seed so each run times the same positions. Evaluators run one
//! at a time on a single thread; fast ones are repeated on each position
//! until at least a millisecond has passed, for a steadier reading.
use std::{
    env,
    hint::black_box,
    time::{ Duration, Instant },
};
use kingdomino_ai::{
    rng,
    core::{ GameState, Turn, game::Rules },
    evaluators::{ claude, evaluator },
    simulation::simulated_deal,
};

const EVALUATORS: &[&str] = &[
    "random",
    "greedy",
    "mce score",
    "mce place",
    "mce lead",
    "mce race",
    "mce halving",
    "nn score",
    "mce nn",
    "eoh",
    "mce eoh",
    "claude",
    "claude mce",
];

/// Both bonus rules: Middle Kingdom and Harmony
type Bonuses = Rules<true, true>;
/// Seed the positions are dealt from
const SEED: u64 = 0x4B494E47;
/// Decisions each player makes in a game, for any player count
const DECISIONS_PER_GAME: f64 = 12.0;

/// Every decision, with its possible turns, from games played by claude bots
fn positions<const N: usize, const S: usize>(games: usize) -> Vec<(GameState<N, S, Bonuses>, Vec<Turn>)> {
    let mut positions = Vec::new();
    for _ in 0..games {
        let mut state = GameState::<N, S, Bonuses>::new(simulated_deal);
        loop {
            if state.draw_pile.iter().all(|&d| d == u8::MAX) {
                if state.deck == 0 { break; }
                state.draw_pile = std::array::from_fn(|_| {
                    let domino = simulated_deal(state.deck);
                    state.deck ^= 1 << domino;
                    domino
                });
                state.draw_pile.sort_unstable();
            }
            let turns = state.possible_turns();
            let choice = claude(&state, &turns);
            positions.push((state, turns.clone()));
            state.play_turn(turns[choice]);
        }
    }
    positions
}

/// Readable duration, given in seconds
fn format(seconds: f64) -> String {
    if seconds < 1e-6 {
        format!("{:.1} ns", seconds * 1e9)
    } else if seconds < 1e-3 {
        format!("{:.1} µs", seconds * 1e6)
    } else if seconds < 1.0 {
        format!("{:.1} ms", seconds * 1e3)
    } else {
        format!("{:.2} s", seconds)
    }
}

fn time<const N: usize, const S: usize>(games: usize) {
    rng::seed(SEED);
    let positions = positions::<N, S>(games);
    rng::init();
    println!("{N} players, Middle Kingdom + Harmony, {} positions", positions.len());
    println!("{:<12} | {:>12} | {:>12} | {:>12}", "evaluator", "per decision", "slowest", "per game");

    for &name in EVALUATORS {
        let evaluate = evaluator::<N, S, Bonuses>(name);
        // Warm up untimed, which also loads any weights
        evaluate(&positions[0].0, &positions[0].1);

        let mut total = 0.0;
        let mut slowest: f64 = 0.0;
        for (state, turns) in &positions {
            let mut repeats = 1;
            let seconds = loop {
                let start = Instant::now();
                for _ in 0..repeats {
                    black_box(evaluate(black_box(state), black_box(turns)));
                }
                let elapsed = start.elapsed();
                if elapsed >= Duration::from_millis(1) || repeats >= 1 << 20 {
                    break elapsed.as_secs_f64() / repeats as f64;
                }
                repeats *= 10;
            };
            total += seconds;
            slowest = slowest.max(seconds);
        }
        let mean = total / positions.len() as f64;
        println!(
            "{:<12} | {:>12} | {:>12} | {:>12}",
            name, format(mean), format(slowest), format(mean * DECISIONS_PER_GAME),
        );
    }
    println!("per game: time one seat spends deciding over a whole game");
    println!();
}

fn main() {
    let games: usize = match env::args().nth(1) {
        Some(arg) => arg.parse().unwrap_or_else(|_| {
            eprintln!("Invalid number provided, defaulting to 1 game.");
            1
        }),
        None => 1,
    };
    time::<4, 4>(games);
    time::<3, 3>(games);
    time::<2, 4>(games);
}
