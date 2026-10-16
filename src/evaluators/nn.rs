use crate::core::{ GameState, Ruleset, Turn, Board, Landscape, DOMINOES };
use std::cell::RefCell;

const NN_WEIGHT: f32 = 1.0;
const N_FEATURES: usize = 194;
const DIMS: [usize; 5] = [194, 256, 128, 64, 1];
const SCALE: f32 = 10.0;
const DEFAULT_WEIGHTS: &str = "src/evaluators/weights.bin";

struct Layer {
    w: Vec<f32>,
    b: Vec<f32>,
    inp: usize,
    out: usize,
}

thread_local! {
    static NET: RefCell<Vec<Layer>> = RefCell::new(Vec::new());
}

pub fn has_weights() -> bool {
    NET.with(|n| !n.borrow().is_empty())
}

pub fn load_weights(path: &str) -> bool {
    let Ok(bytes) = std::fs::read(path) else {
        return false;
    };
    let floats: Vec<f32> = bytes
        .chunks_exact(4)
        .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect();

    let expected: usize = (0..4).map(|k| DIMS[k] * DIMS[k + 1] + DIMS[k + 1]).sum();
    if floats.len() != expected {
        eprintln!(
            "{path}: expected {expected} floats, found {}; does DIMS match the Python model?",
            floats.len()
        );
        return false;
    }

    let mut pos = 0;
    let mut layers = Vec::new();
    for k in 0..4 {
        let (inp, out) = (DIMS[k], DIMS[k + 1]);
        let w = floats[pos..pos + inp * out].to_vec();
        pos += inp * out;
        let b = floats[pos..pos + out].to_vec();
        pos += out;
        layers.push(Layer { w, b, inp, out });
    }
    NET.with(|n| *n.borrow_mut() = layers);
    true
}

fn dot(a: &[f32], b: &[f32]) -> f32 {
    let mut acc = [0.0f32; 8];
    let mut ac = a.chunks_exact(8);
    let mut bc = b.chunks_exact(8);
    for (x, y) in ac.by_ref().zip(bc.by_ref()) {
        for l in 0..8 {
            acc[l] += x[l] * y[l];
        }
    }
    let mut s: f32 = acc.iter().sum();
    for (x, y) in ac.remainder().iter().zip(bc.remainder()) {
        s += x * y;
    }
    s
}

fn forward(layers: &[Layer], x: &[f32; N_FEATURES]) -> f32 {
    let mut cur: Vec<f32> = x.to_vec();
    for (k, l) in layers.iter().enumerate() {
        let mut next = Vec::with_capacity(l.out);
        for o in 0..l.out {
            let s = l.b[o] + dot(&l.w[o * l.inp..(o + 1) * l.inp], &cur);
            next.push(if k < 3 { s.max(0.0) } else { s });
        }
        cur = next;
    }
    cur[0]
}

pub fn nn_score<const N: usize, const S: usize, R: Ruleset>(
    state: &GameState<N, S, R>,
    turns: &[Turn],
) -> usize {
    if !has_weights() { load_weights(DEFAULT_WEIGHTS); }
    NET.with(|n| {
        let layers = n.borrow();
        let mut best_i = 0;
        let mut best_value = f32::NEG_INFINITY;
        let deck_features = vectorize_deck(state.deck);
        for (i, turn) in turns.iter().enumerate() {
            let mut features: [f32; 194] = [0.0; 194];
            features[..19].copy_from_slice(&deck_features);
            features[19..].copy_from_slice(&vectorize_board(turn.board));
            let value = turn.board.score() as f32
                + NN_WEIGHT * SCALE * forward(&layers, &features);
            if value > best_value {
                best_value = value;
                best_i = i;
            }
        }
        best_i
    })
}

pub fn vectorize_board(board: Board) -> [f32; 175] {
    let mut out = [0.0f32; 175];
    for square in 0..25 {
        let start = square * 7;
        let landscape_slice: [f32; 6] = match board.square_landscape(square as u8) {
            Landscape::Farm   => [1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            Landscape::Field  => [0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
            Landscape::Forest => [0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
            Landscape::Mine   => [0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
            Landscape::Lake   => [0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
            Landscape::Swamp  => [0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
            Landscape::Castle => [1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            Landscape::None   => [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        };
        out[start..start + 6].copy_from_slice(&landscape_slice);
        out[start + 6] = board.square_crowns(square as u8) as f32;
    }
    out
}


pub fn vectorize_deck(deck: u64) -> [f32; 19] {
    [
        ((deck & mask_first(Landscape::Farm) & mask_crowns(0)).count_ones() 
            + (deck & mask_second(Landscape::Farm)).count_ones()) as f32,
        (deck & mask_first(Landscape::Farm) & mask_crowns(1)).count_ones() as f32,
        (deck & mask_first(Landscape::Farm) & mask_crowns(2)).count_ones() as f32,
        ((deck & mask_first(Landscape::Field) & mask_crowns(0)).count_ones() 
            + (deck & mask_second(Landscape::Field)).count_ones()) as f32,
        (deck & mask_first(Landscape::Field) & mask_crowns(1)).count_ones() as f32,
        (deck & mask_first(Landscape::Field) & mask_crowns(2)).count_ones() as f32,
        ((deck & mask_first(Landscape::Forest) & mask_crowns(0)).count_ones() 
            + (deck & mask_second(Landscape::Forest)).count_ones()) as f32,
        (deck & mask_first(Landscape::Forest) & mask_crowns(1)).count_ones() as f32,
        (deck & mask_first(Landscape::Forest) & mask_crowns(2)).count_ones() as f32,
        ((deck & mask_first(Landscape::Mine) & mask_crowns(0)).count_ones() 
            + (deck & mask_second(Landscape::Mine)).count_ones()) as f32,
        (deck & mask_first(Landscape::Mine) & mask_crowns(1)).count_ones() as f32,
        (deck & mask_first(Landscape::Mine) & mask_crowns(2)).count_ones() as f32,
        (deck & mask_first(Landscape::Mine) & mask_crowns(3)).count_ones() as f32,
        ((deck & mask_first(Landscape::Lake) & mask_crowns(0)).count_ones() 
            + (deck & mask_second(Landscape::Lake)).count_ones()) as f32,
        (deck & mask_first(Landscape::Lake) & mask_crowns(1)).count_ones() as f32,
        (deck & mask_first(Landscape::Lake) & mask_crowns(2)).count_ones() as f32,
        ((deck & mask_first(Landscape::Swamp) & mask_crowns(0)).count_ones() 
            + (deck & mask_second(Landscape::Swamp)).count_ones()) as f32,
        (deck & mask_first(Landscape::Swamp) & mask_crowns(1)).count_ones() as f32,
        (deck & mask_first(Landscape::Swamp) & mask_crowns(2)).count_ones() as f32,
    ]
}

pub const fn mask_first(landscape: Landscape) -> u64 {
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

pub const fn mask_second(landscape: Landscape) -> u64 {
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

pub const fn mask_crowns(crowns: u8) -> u64 {
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


