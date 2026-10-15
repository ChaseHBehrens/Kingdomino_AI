use crate::core::{ GameState, Landscape, Ruleset, Turn, Board, Domino, DOMINOES };


pub fn eoh_trained<const N: usize, const S: usize, R: Ruleset>(
    state: &GameState<N, S, R>, 
    turns: &[Turn],
) -> usize { eoh(state, turns, [9.0, 0.1, 0.0, 0.0]) }


pub fn update_eoh_test(weights: [f32; 4]) { 
    unsafe { TEST_WEIGHTS = weights };
}

static mut TEST_WEIGHTS: [f32; 4] = [1.0, 0.0, 0.0, 0.0];
pub fn eoh_test<const N: usize, const S: usize, R: Ruleset>(
    state: &GameState<N, S, R>, 
    turns: &[Turn],
) -> usize { unsafe { eoh(state, turns, TEST_WEIGHTS) } }

pub fn eoh<const N: usize, const S: usize, R: Ruleset>(
    state: &GameState<N, S, R>, 
    turns: &[Turn],
    weights: [f32; 4],
) -> usize {
    let mut best_i: usize = 0;
    let mut best_score: f32 = 0.0;
    
    let deck_space = [
        (((state.deck & mask_first(Landscape::Farm)).count_ones()) 
            + ((state.deck & mask_second(Landscape::Farm)).count_ones())) as f32,
        (((state.deck & mask_first(Landscape::Field)).count_ones())
            + ((state.deck & mask_second(Landscape::Field)).count_ones())) as f32,
        (((state.deck & mask_first(Landscape::Forest)).count_ones()) 
            + ((state.deck & mask_second(Landscape::Forest)).count_ones())) as f32,
        (((state.deck & mask_first(Landscape::Mine)).count_ones()) 
            + ((state.deck & mask_second(Landscape::Mine)).count_ones())) as f32,
        (((state.deck & mask_first(Landscape::Lake)).count_ones()) 
            + ((state.deck & mask_second(Landscape::Lake)).count_ones())) as f32,
        (((state.deck & mask_first(Landscape::Swamp)).count_ones()) 
            + ((state.deck & mask_second(Landscape::Swamp)).count_ones())) as f32,
    ];
    let deck_crowns = [
        (((state.deck & mask_first(Landscape::Farm) & mask_crowns(1)).count_ones()) 
            + ((state.deck & mask_first(Landscape::Farm) & mask_crowns(2)).count_ones())) as f32,
        (((state.deck & mask_first(Landscape::Field) & mask_crowns(1)).count_ones()) 
            + ((state.deck & mask_first(Landscape::Field) & mask_crowns(2)).count_ones())) as f32,
        (((state.deck & mask_first(Landscape::Forest) & mask_crowns(1)).count_ones()) 
            + ((state.deck & mask_first(Landscape::Forest) & mask_crowns(2)).count_ones())) as f32,
        (((state.deck & mask_first(Landscape::Mine) & mask_crowns(1)).count_ones()) 
            + ((state.deck & mask_first(Landscape::Mine) & mask_crowns(2)).count_ones())
            + ((state.deck & mask_first(Landscape::Mine) & mask_crowns(3)).count_ones())) as f32,
        (((state.deck & mask_first(Landscape::Lake) & mask_crowns(1)).count_ones()) 
            + ((state.deck & mask_first(Landscape::Lake) & mask_crowns(2)).count_ones())) as f32,
        (((state.deck & mask_first(Landscape::Swamp) & mask_crowns(1)).count_ones()) 
            + ((state.deck & mask_first(Landscape::Swamp) & mask_crowns(2)).count_ones())) as f32,
    ];
    
    let mut opponent: [f32; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    for (i, board) in state.boards.iter().enumerate() {
        if i == state.player_order[0] as usize { continue; }
        for j in 0..6 {
            opponent[j] += ((board.landscape(j).count_ones() - 1) 
                * board.crowns_on_landscape(j)) as f32;
        }
    }
    
    for (i, turn) in turns.iter().enumerate() {
        let placements = [
            landscape_placements(Landscape::Farm, turn.board),
            landscape_placements(Landscape::Field, turn.board),
            landscape_placements(Landscape::Forest, turn.board),
            landscape_placements(Landscape::Mine, turn.board),
            landscape_placements(Landscape::Lake, turn.board),
            landscape_placements(Landscape::Swamp, turn.board),
        ];
        let space = [
            (turn.board.landscape(0).count_ones() - 1) as f32,
            (turn.board.landscape(1).count_ones() - 1) as f32,
            (turn.board.landscape(2).count_ones() - 1) as f32,
            (turn.board.landscape(3).count_ones() - 1) as f32,
            (turn.board.landscape(4).count_ones() - 1) as f32,
            (turn.board.landscape(5).count_ones() - 1) as f32,
        ];
        let crowns = [
            turn.board.crowns_on_landscape(0) as f32,
            turn.board.crowns_on_landscape(1) as f32,
            turn.board.crowns_on_landscape(2) as f32,
            turn.board.crowns_on_landscape(3) as f32,
            turn.board.crowns_on_landscape(4) as f32,
            turn.board.crowns_on_landscape(5) as f32,
        ];
        let score = (weights[0] * turn.board.score() as f32)
            + (0.5 * weights[1] * (
                placements[0] +
                placements[1] +
                placements[2] +
                placements[3] +
                placements[4] +
                placements[5]
            ))
            + (0.1 * weights[2] * (
                ((deck_space[0] + space[0]) * (deck_crowns[0] + crowns[0])) +
                ((deck_space[1] + space[1]) * (deck_crowns[1] + crowns[1])) +
                ((deck_space[2] + space[2]) * (deck_crowns[2] + crowns[2])) +
                ((deck_space[3] + space[3]) * (deck_crowns[3] + crowns[3])) +
                ((deck_space[4] + space[4]) * (deck_crowns[4] + crowns[4])) +
                ((deck_space[5] + space[5]) * (deck_crowns[5] + crowns[5]))
            ))
            + (-0.1 * weights[3] * (
                (opponent[0] - (space[0] * crowns[0])) +
                (opponent[1] - (space[1] * crowns[1])) +
                (opponent[2] - (space[2] * crowns[2])) +
                (opponent[3] - (space[3] * crowns[3])) +
                (opponent[4] - (space[4] * crowns[4])) +
                (opponent[5] - (space[5] * crowns[5]))
            ));
        if score > best_score {
            best_score = score;
            best_i = i;
        }
    }
    best_i
}

fn landscape_placements(landscape: Landscape, board: Board) -> f32 {
    let mut count: f32 = 0.0;
    board.placements(Domino(landscape, landscape, 0), |_| {
        count += 1.0;
    });
    count
}

const fn mask_first(landscape: Landscape) -> u64 {
    let mut mask = 0u64;
    let mut i = 0;
    while i < DOMINOES.len() {
        if discriminants_equal(DOMINOES[i].0, landscape) {
            mask |= 1u64 << i;
        }
        i += 1;
    }
    mask
}

const fn mask_second(landscape: Landscape) -> u64 {
    let mut mask = 0u64;
    let mut i = 0;
    while i < DOMINOES.len() {
        if discriminants_equal(DOMINOES[i].1, landscape) {
            mask |= 1u64 << i;
        }
        i += 1;
    }
    mask
}

const fn discriminants_equal(a: Landscape, b: Landscape) -> bool {
    matches!(
        (a, b),
        (Landscape::Farm, Landscape::Farm)
            | (Landscape::Forest, Landscape::Forest)
            | (Landscape::Lake, Landscape::Lake)
            | (Landscape::Field, Landscape::Field)
            | (Landscape::Swamp, Landscape::Swamp)
            | (Landscape::Mine, Landscape::Mine)
    ) && match (a, b) {
        (x, y) => x as u8 == y as u8, 
    }
}

const fn mask_crowns(crowns: u8) -> u64 {
    let mut mask = 0u64;
    let mut i = 0;
    while i < DOMINOES.len() {
        if DOMINOES[i].2 == crowns {
            mask |= 1u64 << i;
        }
        i += 1;
    }
    mask
}

