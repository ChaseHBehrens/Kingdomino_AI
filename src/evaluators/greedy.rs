use crate::core::{ GameState, Ruleset, Turn };

pub fn greedy<const N: usize, const S: usize, R: Ruleset>(
    _state: &GameState<N, S, R>, 
    turns: &[Turn]
) -> usize {
    let mut best_i = 0;
    let mut best_score = 0;
    for (i, turn) in turns.iter().enumerate() {
        let score = turn.board.score();
        if score > best_score {
            best_score = score;
            best_i = i;
        }
    }
    best_i
}

