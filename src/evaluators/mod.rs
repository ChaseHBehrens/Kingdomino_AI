pub mod random; pub use random::random;
pub mod greedy; pub use greedy::greedy;
pub mod mce; pub use mce::*;
pub mod nn; pub use nn::*;
pub mod eoh; pub use eoh::*;
pub mod claude_heuristic; pub use claude_heuristic::claude;
pub mod mce_heuristic; pub use mce_heuristic::mce_heuristic;

use crate::core::{ GameState, Ruleset, Turn };

/// Returns the corresponding evaluator for the given name
/// Panics if an invalid name is given
pub fn evaluator<const N: usize, const S: usize, R: Ruleset>(name: &str) -> 
fn(&GameState<N, S, R>, &[Turn]) -> usize {
    match name {
        "random" => random,
        "greedy" => greedy,
        "mce score" => mce_score,
        "mce place" => mce_place,
        "mce lead" => mce_lead,
        "mce race" => mce_race,
        "mce halving" => mce_halving,
        "nn score" => nn_score,
        "mce nn" => mce_race_nn_score,
        "eoh" => eoh_trained,
        "mce eoh" => mce_race_eoh,
        "mce heuristic" => mce_heuristic,
        "claude heuristic" => claude,
        _ => panic!("No bot with name {}!", name),
    }
}

