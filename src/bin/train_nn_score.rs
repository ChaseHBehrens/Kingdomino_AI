use std::cell::RefCell;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::process::Command;

use kingdomino_ai::core::{GameState, Ruleset, Turn};
use kingdomino_ai::evaluators::{ greedy, nn_score, vectorize_board, vectorize_deck, nn::load_weights };
use kingdomino_ai::rng;
use kingdomino_ai::run_game;
use kingdomino_ai::simulation::simulated_deal;

const DATA_DIR: &str = "data";
const WEIGHTS: &str = "src/evaluators/weights.bin";
const TRAIN_SCRIPT: &str = "python/score_prediction.py";
const GAMES: usize = 100;
const KEEP_FILES: usize = 20;

thread_local! {
    static STATES: RefCell<[Vec<(i32, [f32; 194])>; 4]> = RefCell::new(Default::default());
}

fn log<const N: usize, const S: usize, R: Ruleset>(
    state: &GameState<N, S, R>,
    turns: &[Turn],
    player: usize,
) -> usize {
    let choice = nn_score(state, turns);
    let turn = &turns[choice];
    let mut features: [f32; 194] = [0.0; 194];
    features[..19].copy_from_slice(&vectorize_deck(state.deck));
    features[19..].copy_from_slice(&vectorize_board(turn.board));
    STATES.with(|s| s.borrow_mut()[player].push((turn.board.score() as i32, features)));
    choice
}

macro_rules! player {
    ($name:ident, $idx:expr) => {
        fn $name<const N: usize, const S: usize, R: Ruleset>(
            state: &GameState<N, S, R>,
            turns: &[Turn],
        ) -> usize {
            log(state, turns, $idx)
        }
    };
}
player!(player0, 0);
player!(player1, 1);
player!(player2, 2);
player!(player3, 3);

fn main() -> std::io::Result<()> {
    rng::init();
    fs::create_dir_all(DATA_DIR)?;
    load_weights(WEIGHTS);

    for generation in 0usize.. {
        let path = format!("{DATA_DIR}/gen_{:03}.csv", generation % KEEP_FILES);
        let mut out = BufWriter::new(File::create(&path)?);
        let mut total: u32 = 0;

        for _ in 0..GAMES {
            let scores: [u32; 4] = run_game!(
                dealer: simulated_deal,
                players: [player0, player1, player2, player3],
            )
            .map(|(_, score)| score);
            total += scores.iter().sum::<u32>();

            let states = STATES.with(|s| std::mem::take(&mut *s.borrow_mut()));
            for (player_states, score) in states.iter().zip(scores) {
                for (current, board) in player_states {
                    let future = score as i32 - current;
                    write!(out, "{future}")?;
                    for v in board {
                        write!(out, ",{v}")?;
                    }
                    writeln!(out)?;
                }
            }
        }
        out.flush()?;
        drop(out);

        println!(
            "generation {generation}: mean score {:.2}",
            total as f32 / (GAMES * 4) as f32
        );

        let status = Command::new("python3").arg(TRAIN_SCRIPT).status()?;
        if !status.success() {
            eprintln!("training script failed ({status}); stopping");
            break;
        }

        load_weights(WEIGHTS);
    }
    Ok(())
}
