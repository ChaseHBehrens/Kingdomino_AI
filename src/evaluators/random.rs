use crate::core::{ GameState, Ruleset, Turn };
use crate::rng::rng;

pub fn random<const N: usize, const S: usize, R: Ruleset>(
    _state: &GameState<N, S, R>, 
    turns: &[Turn]
) -> usize {
    rng(turns.len() as u32) as usize
}

