//! Claude MCE evaluator: Monte Carlo racing over heuristically pruned moves,
//! using the claude evaluator as the rollout policy.
//!
//! Opponents in rollouts pick moves at random, weighted towards the ones the
//! claude heuristic values most, which matched logged human games far better
//! than assuming they always play claude's best move. In the final round they
//! play for placement instead: they block a player only when it changes where
//! they finish. Each rollout is scored by the opponents beaten plus a small
//! share of the margin over the best one, so the search plays to win but
//! still prefers bigger wins and closer losses.
use crate::core::{ Board, GameState, Ruleset, Turn };
use crate::rng::rng;
use crate::simulation::simulated_deal;
use super::claude_heuristic::{ claude, claude_values };

/// Points of heuristic value that make a move e times less likely for an
/// opponent in a rollout
const OPPONENT_SPREAD: f32 = 2.0;
/// Points one place in the final standings is worth to an opponent in the
/// final round
const PLACE: i32 = 30;
/// Points of margin over the best opponent worth as much as beating one
const MARGIN_PER_WIN: f64 = 60.0;

/// Score of a board at the end of the game
fn final_score<R: Ruleset>(board: &Board) -> i32 {
    board.score() as i32
        + if R::MIDDLE_KINGDOM && board.middle_kingdom() { 10 } else { 0 }
        + if R::HARMONY && board.harmony() { 5 } else { 0 }
}

/// Values of final round turns, in units of 1/16 of a point: the final score
/// plus PLACE points for each opponent finished above, assuming everyone
/// picking later takes their highest scoring move
fn final_values<const N: usize, const S: usize, R: Ruleset>(
    state: &GameState<N, S, R>,
    turns: &[Turn],
) -> Vec<i32> {
    let active = state.player_order[0] as usize;
    let mut later = Vec::with_capacity(64);
    turns.iter().map(|&turn| {
        let mut s = *state;
        s.play_turn(turn);
        while s.draw_pile.iter().any(|&d| d != u8::MAX) {
            s.possible_turns_into(&mut later);
            let best = (0..later.len()).max_by_key(|&i| final_score::<R>(&later[i].board)).unwrap();
            s.play_turn(later[best]);
        }
        let mine = final_score::<R>(&s.boards[active]);
        let halves: i32 = (0..N).filter(|&p| p != active).map(|p| {
            let theirs = final_score::<R>(&s.boards[p]);
            if mine > theirs { 2 } else if mine == theirs { 1 } else { 0 }
        }).sum();
        16 * mine + 8 * PLACE * halves
    }).collect()
}

/// Rollout policy for opponents: a move picked at random with probability
/// proportional to exp(value / OPPONENT_SPREAD)
pub fn opponent<const N: usize, const S: usize, R: Ruleset>(
    state: &GameState<N, S, R>,
    turns: &[Turn],
) -> usize {
    if turns.len() <= 1 { return 0; }
    let values: Vec<i32> = if state.deck == 0 {
        final_values(state, turns)
    } else {
        claude_values(state, turns).collect()
    };
    let max = *values.iter().max().unwrap();
    let weight = |v: i32| ((v - max) as f32 / (16.0 * OPPONENT_SPREAD)).exp();
    let total: f32 = values.iter().map(|&v| weight(v)).sum();
    let mut r = rng(1 << 24) as f32 / (1 << 24) as f32 * total;
    for (i, &v) in values.iter().enumerate() {
        r -= weight(v);
        if r <= 0.0 { return i; }
    }
    values.len() - 1
}

pub fn mce_heuristic<const N: usize, const S: usize, R: Ruleset>(
    state: &GameState<N, S, R>,
    turns: &[Turn],
) -> usize {
    /// Moves kept after pruning by the heuristic
    const MAX_ARMS: usize = 16;
    /// Rollouts per decision; 4000 tested no stronger, 1000 slightly weaker
    const MAX_SIMULATIONS: usize = 2000;
    /// Rounds of rollouts before any move can be dropped
    const MIN_ROUNDS: u32 = 4;
    /// Standard errors a move may trail the best by before it is dropped
    const CONFIDENCE: f64 = 2.5;

    if turns.len() <= 1 { return 0; }
    let active = state.player_order[0] as usize;

    // Keep only the most promising moves according to the heuristic
    let mut ranked: Vec<(usize, i32)> = claude_values(state, turns).enumerate().collect();
    ranked.sort_unstable_by_key(|&(_, v)| std::cmp::Reverse(v));
    ranked.truncate(MAX_ARMS);

    // (turn index, state after it, sum of rollout values, sum of their squares)
    let mut arms: Vec<_> = ranked
        .iter()
        .map(|&(i, _)| {
            let mut s = *state;
            s.play_turn(turns[i]);
            (i, s, 0.0f64, 0.0f64)
        })
        .collect();
    let policies: [fn(&GameState<N, S, R>, &[Turn]) -> usize; N] =
        std::array::from_fn(|p| if p == active { claude } else { opponent::<N, S, R> });
    let mut n = 0u32;
    let mut used = 0;

    while arms.len() > 1 && used + arms.len() <= MAX_SIMULATIONS {
        n += 1;
        used += arms.len();
        for (_, start, sum, squares) in arms.iter_mut() {
            let mut s = *start;
            s.play_out(simulated_deal, policies);
            let mine = s.score(active);
            let mut value = 0.0;
            let mut best = 0;
            for p in (0..N).filter(|&p| p != active) {
                let theirs = s.score(p);
                value += if theirs < mine { 1.0 } else if theirs == mine { 0.5 } else { 0.0 };
                best = best.max(theirs);
            }
            value += (mine as f64 - best as f64) / MARGIN_PER_WIN;
            *sum += value;
            *squares += value * value;
        }
        if n < MIN_ROUNDS { continue; }

        // Drop moves whose mean trails the best by more than a few standard errors
        let rounds = n as f64;
        let variance = arms.iter()
            .map(|a| (a.3 / rounds - (a.2 / rounds).powi(2)).max(0.0))
            .sum::<f64>() / arms.len() as f64;
        let limit = CONFIDENCE * (2.0 * variance / rounds).sqrt();
        let best = arms.iter().map(|a| a.2 / rounds).fold(f64::MIN, f64::max);
        arms.retain(|a| best - a.2 / rounds <= limit);
    }
    arms.iter().max_by(|a, b| a.2.total_cmp(&b.2)).unwrap().0
}
