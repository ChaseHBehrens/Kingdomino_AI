use super::{ Landscape, Domino };


/// Bit masks for board files
pub mod file {
    pub const A: u32 = 0x108421;
    pub const B: u32 = 0x210842;
    pub const C: u32 = 0x421084;
    pub const D: u32 = 0x842108;
    pub const E: u32 = 0x1084210;
}


/// Bit masks for board ranks
pub mod rank {
    pub const ONE: u32 = 0x1F;
    pub const TWO: u32 = 0x3E0;
    pub const THREE: u32 = 0x7C00;
    pub const FOUR: u32 = 0xF8000;
    pub const FIVE: u32 = 0x1F00000;
}


/// Represents a set of placed dominoes
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Board {
    crowns: [u32; 2],
    landscape: [u32; 6],
}

impl Board {
    pub fn new() -> Self {
        Self {
            crowns: [0; 2],
            landscape: [1 << 12; 6],
        }
    }
    
    pub fn landscape(&self, landscape: usize) -> u32 {
        self.landscape[landscape]
    }

    pub fn crowns_on_landscape(&self, landscape: usize) -> u32 { 
        (self.crowns[0] & self.landscape[landscape]).count_ones()
            + 2 * (self.crowns[1] & self.landscape[landscape]).count_ones()
    }

    pub fn square_landscape(&self, square: u8) -> Landscape {
        match self.landscape.map(|b| (b >> square) & 1) {
            [1, 0, 0, 0, 0, 0] => Landscape::Farm,
            [0, 1, 0, 0, 0, 0] => Landscape::Field,
            [0, 0, 1, 0, 0, 0] => Landscape::Forest,
            [0, 0, 0, 1, 0, 0] => Landscape::Mine,
            [0, 0, 0, 0, 1, 0] => Landscape::Lake,
            [0, 0, 0, 0, 0, 1] => Landscape::Swamp,
            [1, 1, 1, 1, 1, 1] => Landscape::Castle,
            _ => Landscape::None,
        }
    }

    pub fn square_crowns(&self, square: u8) -> u8 {
        let low = (self.crowns[0] >> square) & 1;
        let high = (self.crowns[1] >> square) & 1;
        (low | (high << 1)) as u8
    }
    
    #[inline(always)]
    fn castle(&self) -> u32 {
        self.landscape
            .iter()
            .copied()
            .reduce(|a, x| a & x)
            .expect("There always must be a castle!")
    }

    #[inline(always)]
    pub fn occupied(&self) -> u32 {
        self.landscape[Landscape::Farm as usize] |
        self.landscape[Landscape::Field as usize] |
        self.landscape[Landscape::Forest as usize] |
        self.landscape[Landscape::Mine as usize] |
        self.landscape[Landscape::Lake as usize] |
        self.landscape[Landscape::Swamp as usize]
    }
    
    #[inline(always)]
    fn shift_up(&mut self, occupied: u32) -> u32 {
        let shift = 5 * ((occupied & rank::FIVE) == 0) as u32;
        for l in self.crowns.iter_mut() { *l <<= shift; }
        for l in self.landscape.iter_mut() { *l <<= shift; }
        occupied << shift
    }
    
    #[inline(always)]
    fn shift_down(&mut self, occupied: u32) -> u32 {
        let shift = 5 * ((occupied & rank::ONE) == 0) as u32;
        for l in self.crowns.iter_mut() { *l >>= shift; }
        for l in self.landscape.iter_mut() { *l >>= shift; }
        occupied >> shift
    }

    #[inline(always)]
    fn shift_left(&mut self, occupied: u32) -> u32 {
        let shift = ((occupied & file::A) == 0) as u32;
        for l in self.crowns.iter_mut() { *l >>= shift; }
        for l in self.landscape.iter_mut() { *l >>= shift; }
        occupied >> shift
    }

    #[inline(always)]
    fn shift_right(&mut self, occupied: u32) -> u32 {
        let shift = ((occupied & file::E) == 0) as u32;
        for l in self.crowns.iter_mut() { *l <<= shift; }
        for l in self.landscape.iter_mut() { *l <<= shift; }
        occupied << shift
    }

    #[inline(always)]
    fn set_crowns(&mut self, square: u32, crowns: u8) {
        self.crowns[0] |= (crowns as u32 & 1) << square;
        self.crowns[1] |= (crowns as u32 >> 1) << square;
    }

    /// Returns a list of possible board states after the domino is placed
    pub fn placements(&self, domino: Domino, mut f: impl FnMut(Board)) {
        const UP: i8 = 5;
        const DOWN: i8 = -5;
        const LEFT: i8 = -1;
        const RIGHT: i8 = 1;

        let occupied = self.occupied();
        let mut d_board = *self;
        let d_occupied = d_board.shift_down(occupied);
        let d_occupied = d_board.shift_down(d_occupied);
        let mut dl_board = d_board;
        let dl_occupied = dl_board.shift_left(d_occupied);
        let dl_occupied = dl_board.shift_left(dl_occupied);
        let dl_free = !dl_occupied;
        let mut dr_board = d_board;
        let dr_occupied = dr_board.shift_right(d_occupied);
        let dr_occupied = dr_board.shift_right(dr_occupied);
        let dr_free = !dr_occupied;
        let mut u_board = *self;
        let u_occupied = u_board.shift_up(occupied);
        let u_occupied = u_board.shift_up(u_occupied);
        let mut ul_board = u_board;
        let ul_occupied = ul_board.shift_left(u_occupied);
        let ul_occupied = ul_board.shift_left(ul_occupied);
        let ul_free = !ul_occupied;
        let mut ur_board = u_board;
        let ur_occupied = ur_board.shift_right(u_occupied);
        let ur_occupied = ur_board.shift_right(ur_occupied);
        let ur_free = !ur_occupied;
        
        let l0 = dl_board.landscape[domino.0 as usize] &! (rank::FOUR | rank::FIVE);
        let mut dl_u0 = ((l0 << 5) & dl_free) & (((l0 << 10) & dl_free) >> 5);
        let l0 = dl_board.landscape[domino.0 as usize] &! (file::E | rank::FIVE);
        dl_u0 |= ((l0 << 1) & dl_free) & (((l0 << 6) & dl_free) >> 5);
        let dl_r0 = ((l0 << 5) & dl_free) & (((l0 << 6) & dl_free) >> 1);
        let l0 = dr_board.landscape[domino.0 as usize] &! (file::B | file::A);
        let mut dr_l0 = ((l0 >> 1) & dr_free) & (((l0 >> 2) & dr_free) << 1);
        let l0 = dr_board.landscape[domino.0 as usize] &! (file::A | rank::FIVE);
        dr_l0 |= ((l0 << 5) & dr_free) & (((l0 << 4) & dr_free) << 1);
        let dr_u0 = ((l0 >> 1) & dr_free) & (((l0 << 4) & dr_free) >> 5);
        let l0 = ul_board.landscape[domino.0 as usize] &! (file::D | file::E);
        let mut ul_r0 = ((l0 << 1) & ul_free) & (((l0 << 2) & ul_free) >> 1);
        let l0 = ul_board.landscape[domino.0 as usize] &! (file::E | rank::ONE);
        ul_r0 |= ((l0 >> 5) & ul_free) & (((l0 >> 4) & ul_free) >> 1);
        let ul_d0 = ((l0 << 1) & ul_free) & (((l0 >> 4) & ul_free) << 5);
        let l0 = ur_board.landscape[domino.0 as usize] &! (rank::TWO | rank::ONE);
        let mut ur_d0 = ((l0 >> 5) & ur_free) & (((l0 >> 10) & ur_free) << 5);
        let l0 = ur_board.landscape[domino.0 as usize] &! (file::A | rank::ONE);
        ur_d0 |= ((l0 >> 1) & ur_free) & (((l0 >> 6) & ur_free) << 5);
        let ur_l0 = ((l0 >> 5) & ur_free) & (((l0 >> 6) & ur_free) << 1);

        place0::<UP>(dl_u0, dl_board, domino, &mut f);
        place0::<UP>(dr_u0, dr_board, domino, &mut f);
        place0::<DOWN>(ul_d0, ul_board, domino, &mut f);
        place0::<DOWN>(ur_d0, ur_board, domino, &mut f);
        place0::<LEFT>(dr_l0, dr_board, domino, &mut f);
        place0::<LEFT>(ur_l0, ur_board, domino, &mut f);
        place0::<RIGHT>(dl_r0, dl_board, domino, &mut f);
        place0::<RIGHT>(ul_r0, ul_board, domino, &mut f);
        if (domino.0 == domino.1) && (domino.2 == 0) { return }

        let l1 = dl_board.landscape[domino.1 as usize] &! (rank::FOUR | rank::FIVE);
        let mut dl_u1 = ((l1 << 5) & dl_free) & (((l1 << 10) & dl_free) >> 5);
        let l1 = dl_board.landscape[domino.1 as usize] &! (file::E | rank::FIVE);
        dl_u1 |= ((l1 << 1) & dl_free) & (((l1 << 6) & dl_free) >> 5);
        let dl_r1 = ((l1 << 5) & dl_free) & (((l1 << 6) & dl_free) >> 1);
        let l1 = dr_board.landscape[domino.1 as usize] &! (file::B | file::A);
        let mut dr_l1 = ((l1 >> 1) & dr_free) & (((l1 >> 2) & dr_free) << 1);
        let l1 = dr_board.landscape[domino.1 as usize] &! (file::A | rank::FIVE);
        dr_l1 |= ((l1 << 5) & dr_free) & (((l1 << 4) & dr_free) << 1);
        let dr_u1 = ((l1 >> 1) & dr_free) & (((l1 << 4) & dr_free) >> 5);
        let l1 = ul_board.landscape[domino.1 as usize] &! (file::D | file::E);
        let mut ul_r1 = ((l1 << 1) & ul_free) & (((l1 << 2) & ul_free) >> 1);
        let l1 = ul_board.landscape[domino.1 as usize] &! (file::E | rank::ONE);
        ul_r1 |= ((l1 >> 5) & ul_free) & (((l1 >> 4) & ul_free) >> 1);
        let ul_d1 = ((l1 << 1) & ul_free) & (((l1 >> 4) & ul_free) << 5);
        let l1 = ur_board.landscape[domino.1 as usize] &! (rank::TWO | rank::ONE);
        let mut ur_d1 = ((l1 >> 5) & ur_free) & (((l1 >> 10) & ur_free) << 5);
        let l1 = ur_board.landscape[domino.1 as usize] &! (file::A | rank::ONE);
        ur_d1 |= ((l1 >> 1) & ur_free) & (((l1 >> 6) & ur_free) << 5);
        let ur_l1 = ((l1 >> 5) & ur_free) & (((l1 >> 6) & ur_free) << 1);

        place1::<UP>(dl_u1, dl_board, domino, &mut f);
        place1::<UP>(dr_u1, dr_board, domino, &mut f);
        place1::<DOWN>(ul_d1, ul_board, domino, &mut f);
        place1::<DOWN>(ur_d1, ur_board, domino, &mut f);
        place1::<LEFT>(dr_l1, dr_board, domino, &mut f);
        place1::<LEFT>(ur_l1, ur_board, domino, &mut f);
        place1::<RIGHT>(dl_r1, dl_board, domino, &mut f);
        place1::<RIGHT>(ul_r1, ul_board, domino, &mut f);
    }

    /// Returns the score of the board
    pub fn score(&self) -> u32 {
        let castle = self.castle();
        let crowned = self.crowns[0] | self.crowns[1];
        let not_a = !file::A;
        let not_e = !file::E;
        let mut score = 0;
        for l in self.landscape {
            let squares = l & !castle;
            let mut seeds = squares & crowned;
            while seeds != 0 {
                let mut cluster = 1 << seeds.trailing_zeros();
                loop {
                    let h = (cluster
                        | ((cluster << 1) & not_a)
                        | ((cluster >> 1) & not_e))
                        & squares;
                    let grown = (h | (h << 5) | (h >> 5)) & squares;
                    if grown == cluster { break; }
                    cluster = grown;
                }
                let crowns = (self.crowns[0] & cluster).count_ones()
                    + 2 * (self.crowns[1] & cluster).count_ones();
                score += cluster.count_ones() * crowns;
                seeds &= !cluster;
            }
        }
        score
    }
    
    /// Returns true if the middle kingdom bonus applies
    #[inline(always)]
    pub fn middle_kingdom(&self) -> bool {
        self.castle() == (file::C & rank::THREE)
    }

    /// Returns true if the harmony bonus applies
    #[inline(always)]
    pub fn harmony(&self) -> bool {
        self.occupied() == 0x1FFFFFF
    }
}


#[inline(always)]
fn place0<const OFFSET: i8>(
    mut squares: u32,
    board: Board,
    domino: Domino,
    out: &mut impl FnMut(Board),
) {
    while squares != 0 {
        let square0 = squares.trailing_zeros();
        let square1 = (square0 as i8 + OFFSET) as u32;
        let mut b = board;
        b.set_crowns(square0, domino.2);
        unsafe {
            *b.landscape.get_unchecked_mut(domino.0 as usize) |= 1 << square0;
            *b.landscape.get_unchecked_mut(domino.1 as usize) |= 1 << square1;
        }
        out(b);
        squares &= squares - 1;
    }
}

#[inline(always)]
fn place1<const OFFSET: i8>(
    mut squares: u32,
    board: Board,
    domino: Domino,
    out: &mut impl FnMut(Board),
) {
    while squares != 0 {
        let square0 = squares.trailing_zeros();
        let square1 = (square0 as i8 + OFFSET) as u32;
        let mut b = board;
        b.set_crowns(square1, domino.2);
        unsafe {
            *b.landscape.get_unchecked_mut(domino.1 as usize) |= 1 << square0;
            *b.landscape.get_unchecked_mut(domino.0 as usize) |= 1 << square1;
        }
        out(b);
        squares &= squares - 1;
    }
}

