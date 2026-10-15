use std::{ marker::PhantomData, array };
use super::{ Board, DOMINOES };
const TAKEN: u8 = u8::MAX;


/// Represents a turn of the game
#[derive(Clone, Copy)]
pub struct Turn {
    pub board: Board,
    pub position: usize,
}


/// Represents the optional bonus rules
pub trait Ruleset: Copy + Clone {
    const MIDDLE_KINGDOM: bool;
    const HARMONY: bool;
}

#[derive(Clone, Copy)]
pub struct Rules<const MK: bool, const H: bool>;

impl<const MK: bool, const H: bool> Ruleset for Rules<MK, H> {
    const MIDDLE_KINGDOM: bool = MK;
    const HARMONY: bool = H;
}

pub struct GameOver;

#[derive(Clone, Copy)]
pub struct GameState<const N: usize, const S: usize, R: Ruleset> {
    pub deck: u64,
    pub draw_pile: [u8; S],
    pub player_order: [u8; S],
    pub boards: [Board; N],
    _ruleset: PhantomData<R>,
}

impl<const N: usize, const S: usize, R: Ruleset> GameState<N, S, R> {
    pub fn new(dealer: fn(u64) -> u8) -> Self {
        let mut deck: u64 = 0xFFFFFFFFFFFF;
        for _ in 0..(48 - (12 * N)) { deck ^= 1 << dealer(deck); }
        GameState {
            deck,
            draw_pile: [TAKEN; S],
            player_order: array::from_fn(|i| (i % N) as u8),
            boards: [Board::new(); N],
            _ruleset: PhantomData,
        }
    }

    pub fn possible_turns_into(&self, turns: &mut Vec<Turn>) {
        turns.clear();
        let board = &self.boards[self.player_order[0] as usize]; 
        let available = || { 
            self.draw_pile
                .iter()
                .enumerate()
                .filter(|&(_, &draw)| draw != TAKEN)
        };
        for (position, &draw) in available() {
            board.placements(DOMINOES[draw as usize], |b| {
                turns.push(Turn { board: b, position });
            });
        }
        if turns.is_empty() {
            for (position, _) in available() {
                turns.push(Turn { board: *board, position });
            }
        }
    }
    
    pub fn possible_turns(&self) -> Vec<Turn> {
        let mut turns = Vec::with_capacity(256);
        self.possible_turns_into(&mut turns);
        turns
    }

    pub fn play_turn(&mut self, turn: Turn) {
        let active = self.player_order[0] as usize;
        self.boards[active] = turn.board;
        self.draw_pile[turn.position] = TAKEN;
        let taken_after = self.draw_pile[turn.position..]
            .iter()
            .filter(|&&x| x == TAKEN)
            .count();
        let i = S - taken_after;
        self.player_order[0..=i].rotate_left(1);
        self.player_order[i] = active as u8;
    }

    pub fn play_out(
        &mut self,
        dealer: fn(u64) -> u8,
        evaluators: [fn(&GameState<N, S, R>, &[Turn]) -> usize; N],
    ) {
        let mut turns = Vec::<Turn>::with_capacity(256);
        loop {
            if self.draw_pile == [TAKEN; S] {
                if self.deck == 0 { return; }
                self.draw_pile = array::from_fn(|_| {
                    let domino = dealer(self.deck);
                    self.deck ^= 1 << domino;
                    domino
                });
                self.draw_pile.sort_unstable();
            }
            let active = self.player_order[0] as usize;
            self.possible_turns_into(&mut turns);
            let i = evaluators[active](self, &turns);
            self.play_turn(turns.swap_remove(i));
        }
    }

    pub fn score(&self, player: usize) -> u32 {
        let board = self.boards[player];
        let mut score = board.score();
        let end = (self.deck == 0) && (self.draw_pile == [TAKEN; S]);
        score += 10 * (R::MIDDLE_KINGDOM && end && board.middle_kingdom()) as u32;
        score += 5 * (R::HARMONY && end && board.harmony()) as u32; 
        score
    }
}

pub fn run<const N: usize, const S: usize, R: Ruleset>(
    dealer: fn(u64) -> u8,
    evaluators: [fn(&GameState<N, S, R>, &[Turn]) -> usize; N],
) -> [(Board, u32); N] {
    let mut state = GameState::new(dealer);
    state.play_out(dealer, evaluators);
    array::from_fn(|i| (state.boards[i], state.score(i)))
}


#[macro_export]
macro_rules! ruleset_type {
    (@build [$mk:tt, $h:tt];) => {
        $crate::core::game::Rules<$mk, $h>
    };
    (@build [$mk:tt, $h:tt]; MIDDLE_KINGDOM $(, $rest:ident)*) => {
        $crate::ruleset_type!(@build [true, $h]; $($rest),*)
    };
    (@build [$mk:tt, $h:tt]; HARMONY $(, $rest:ident)*) => {
        $crate::ruleset_type!(@build [$mk, true]; $($rest),*)
    };
    ($($flag:ident),* $(,)?) => {
        $crate::ruleset_type!(@build [false, false]; $($flag),*)
    };
    (@build [$mk:tt, $h:tt]; $bad:ident $(, $rest:ident)*) => {
        compile_error!(concat!("unknown ruleset flag: ", stringify!($bad)))
    };
}


/// Runs a game and returns the final board states
#[macro_export]
macro_rules! run_game {
    (dealer: $dealer:expr, 
     players: [$e1:expr, $e2:expr $(,)?], 
     additional_rules: [$($flag:ident),* $(,)?] $(,)?) => {
        $crate::core::game::run::<2, 4, $crate::ruleset_type!($($flag),*)>($dealer, [$e1, $e2])
    };
    (dealer: $dealer:expr, 
     players: [$e1:expr, $e2:expr, $e3:expr $(,)?], 
     additional_rules: [$($flag:ident),* $(,)?] $(,)?) => {
        $crate::core::game::run::<3, 3, $crate::ruleset_type!($($flag),*)>($dealer, [$e1, $e2, $e3])
    };
    (dealer: $dealer:expr, 
     players: [$e1:expr, $e2:expr, $e3:expr, $e4:expr $(,)?], 
     additional_rules: [$($flag:ident),* $(,)?] $(,)?) => {
        $crate::core::game::run::<4, 4, $crate::ruleset_type!($($flag),*)>($dealer, [$e1, $e2, $e3, $e4])
    };
    (dealer: $dealer:expr, 
     players: [$e1:expr, $e2:expr $(,)?] $(,)?) => {
        $crate::core::game::run::<2, 4, $crate::ruleset_type!()>($dealer, [$e1, $e2])
    };
    (dealer: $dealer:expr, 
     players: [$e1:expr, $e2:expr, $e3:expr $(,)?] $(,)?) => {
        $crate::core::game::run::<3, 3, $crate::ruleset_type!()>($dealer, [$e1, $e2, $e3])
    };
    (dealer: $dealer:expr, 
     players: [$e1:expr, $e2:expr, $e3:expr, $e4:expr $(,)?] $(,)?) => {
        $crate::core::game::run::<4, 4, $crate::ruleset_type!()>($dealer, [$e1, $e2, $e3, $e4])
    };
    ($($rest:tt)*) => {
        compile_error!("A game must have a dealer and between 2 and 4 players")
    };
}
