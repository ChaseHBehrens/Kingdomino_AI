pub mod random; pub use random::random;
pub mod greedy; pub use greedy::greedy;
pub mod mce; pub use mce::*;
pub mod nn; pub use nn::*;
pub mod eoh; pub use eoh::*;
pub mod claude; pub use claude::claude;
pub mod claude_mce; pub use claude_mce::claude_mce;

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
        "claude mce" => claude_mce,
        "claude" => claude,
        _ => panic!("No bot with name {}!", name),
    }
}

