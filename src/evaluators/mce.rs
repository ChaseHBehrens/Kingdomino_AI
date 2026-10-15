//! Monte Carlo Evaluation Evaluators
use std::array;
use crate::core::{ GameState, Ruleset, Turn };
use crate::simulation::simulated_deal;
use super::{ greedy, nn_score, eoh_trained };

pub fn mce_score<const N: usize, const S: usize, R: Ruleset>(
    state: &GameState<N, S, R>, 
    turns: &[Turn]
) -> usize {
    let mut best_i = 0;
    let mut best_score = 0;
    let active = state.player_order[0];
    for (i, turn) in turns.iter().enumerate() {
        let mut score_sum: u32 = 0;
        const SIMULATIONS: u32 = 40;
        let mut turn_state = *state;
        turn_state.play_turn(*turn);
        for _ in 0..SIMULATIONS {
            let mut s = turn_state;
            s.play_out(simulated_deal, [greedy; N]);
            score_sum += s.score(active as usize);
        }
        let score = score_sum / SIMULATIONS;
        if score > best_score {
            best_score = score;
            best_i = i;
        }
    }
    best_i
}


pub fn mce_place<const N: usize, const S: usize, R: Ruleset>(
    state: &GameState<N, S, R>, 
    turns: &[Turn]
) -> usize {
    let mut best_i: usize = 0;
    let mut best_score: f32 = 0.0;
    let active = state.player_order[0];
    for (i, turn) in turns.iter().enumerate() {
        let mut score_sum: f32 = 0.0;
        const SIMULATIONS: u32 = 40;
        let mut turn_state = *state;
        turn_state.play_turn(*turn);
        for _ in 0..SIMULATIONS {
            let mut s = turn_state;
            s.play_out(simulated_deal, [greedy; N]);
            let scores: [u32; N] = array::from_fn(|i| s.score(i));
            let score = scores[active as usize];
            score_sum += scores.iter().filter(|&&x| x < score).count() as f32;
        }
        let score = score_sum / SIMULATIONS as f32;
        if score > best_score {
            best_score = score;
            best_i = i;
        }
    }
    best_i
}


pub fn mce_lead<const N: usize, const S: usize, R: Ruleset>(
    state: &GameState<N, S, R>, 
    turns: &[Turn]
) -> usize {
    let mut best_i: usize = 0;
    let mut best_score: f32 = 0.0;
    let active = state.player_order[0];
    for (i, turn) in turns.iter().enumerate() {
        let mut score_sum: f32 = 0.0;
        const SIMULATIONS: u32 = 40;
        let mut turn_state = *state;
        turn_state.play_turn(*turn);
        for _ in 0..SIMULATIONS {
            let mut s = turn_state;
            s.play_out(simulated_deal, [greedy; N]);
            let scores: [u32; N] = array::from_fn(|i| s.score(i));
            let score = scores[active as usize];
            score_sum += scores.iter().fold(0.0, |a, &opponent_score| {
                a + (score as f32 - opponent_score as f32)
            });
        }
        let score = score_sum / SIMULATIONS as f32;
        if score > best_score {
            best_score = score;
            best_i = i;
        }
    }
    best_i
}

pub fn mce_race<const N: usize, const S: usize, R: Ruleset>(
    state: &GameState<N, S, R>,
    turns: &[Turn],
) -> usize {
    const MAX_SIMULATIONS: usize = 5000;
    const CONFIDENCE: f32 = 4.0;

    let active = state.player_order[0] as usize;
    let mut arms: Vec<_> = turns
        .iter()
        .enumerate()
        .map(|(i, &t)| {
            let mut s = *state;
            s.play_turn(t);
            (i, s, 0u32)
        })
        .collect();
    let mut n = 0u32;
    let mut used = 0;

    while arms.len() > 1 && used + arms.len() <= MAX_SIMULATIONS {
        n += 1;
        used += arms.len();
        for (_, start, wins) in arms.iter_mut() {
            let mut s = *start;
            s.play_out(simulated_deal, [greedy; N]);
            let mine = s.score(active);
            *wins += (0..N).filter(|&i| s.score(i) < mine).count() as u32;
        }
        let best = arms.iter().map(|a| a.2).max().unwrap();
        let limit = 2.0 * CONFIDENCE * n as f32;
        arms.retain(|a| {
            let d = (best - a.2) as f32;
            d * d <= limit
        });
    }
    arms.iter().max_by_key(|a| a.2).unwrap().0
}


pub fn mce_halving<const N: usize, const S: usize, R: Ruleset>(
    state: &GameState<N, S, R>,
    turns: &[Turn],
) -> usize {
    const PER_ROUND: u32 = 12;
    let active = state.player_order[0] as usize;
    let mut arms: Vec<(usize, u32)> = (0..turns.len()).map(|i| (i, 0)).collect();

    while arms.len() > 1 {
        for (i, wins) in arms.iter_mut() {
            for _ in 0..PER_ROUND {
                let mut s = *state;
                s.play_turn(turns[*i]);
                s.play_out(simulated_deal, [greedy; N]);
                let mine = s.score(active);
                *wins += (0..N).filter(|&p| s.score(p) < mine).count() as u32;
            }
        }
        arms.sort_unstable_by_key(|a| std::cmp::Reverse(a.1));
        arms.truncate((arms.len() + 1) / 2);
    }
    arms[0].0
}


pub fn mce_race_nn_score<const N: usize, const S: usize, R: Ruleset>(
    state: &GameState<N, S, R>,
    turns: &[Turn],
) -> usize {
    const MAX_SIMULATIONS: usize = 200;
    const CONFIDENCE: f32 = 4.0;

    let active = state.player_order[0] as usize;
    let mut arms: Vec<_> = turns
        .iter()
        .enumerate()
        .map(|(i, &t)| {
            let mut s = *state;
            s.play_turn(t);
            (i, s, 0u32)
        })
        .collect();
    let mut n = 0u32;
    let mut used = 0;

    while arms.len() > 1 && used + arms.len() <= MAX_SIMULATIONS {
        n += 1;
        used += arms.len();
        for (_, start, wins) in arms.iter_mut() {
            let mut s = *start;
            s.play_out(simulated_deal, [nn_score; N]);
            let mine = s.score(active);
            *wins += (0..N).filter(|&i| s.score(i) < mine).count() as u32;
        }
        let best = arms.iter().map(|a| a.2).max().unwrap();
        let limit = 2.0 * CONFIDENCE * n as f32;
        arms.retain(|a| {
            let d = (best - a.2) as f32;
            d * d <= limit
        });
    }
    arms.iter().max_by_key(|a| a.2).unwrap().0
}


pub fn mce_race_eoh<const N: usize, const S: usize, R: Ruleset>(
    state: &GameState<N, S, R>,
    turns: &[Turn],
) -> usize {
    const MAX_SIMULATIONS: usize = 200;
    const CONFIDENCE: f32 = 4.0;

    let active = state.player_order[0] as usize;
    let mut arms: Vec<_> = turns
        .iter()
        .enumerate()
        .map(|(i, &t)| {
            let mut s = *state;
            s.play_turn(t);
            (i, s, 0u32)
        })
        .collect();
    let mut n = 0u32;
    let mut used = 0;

    while arms.len() > 1 && used + arms.len() <= MAX_SIMULATIONS {
        n += 1;
        used += arms.len();
        for (_, start, wins) in arms.iter_mut() {
            let mut s = *start;
            s.play_out(simulated_deal, [eoh_trained; N]);
            let mine = s.score(active);
            *wins += (0..N).filter(|&i| s.score(i) < mine).count() as u32;
        }
        let best = arms.iter().map(|a| a.2).max().unwrap();
        let limit = 2.0 * CONFIDENCE * n as f32;
        arms.retain(|a| {
            let d = (best - a.2) as f32;
            d * d <= limit
        });
    }
    arms.iter().max_by_key(|a| a.2).unwrap().0
}


