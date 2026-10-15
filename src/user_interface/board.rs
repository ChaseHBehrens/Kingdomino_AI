use crate::core::{ Board, Landscape };

pub fn display_board(board: &Board) {    
    for r in (0..25).step_by(5).rev() {
        for c in 0..5 {
            match board.square_landscape(r + c) {
                Landscape::Farm => print!("W"),
                Landscape::Field => print!("F"),
                Landscape::Forest => print!("T"),
                Landscape::Mine => print!("M"),
                Landscape::Lake => print!("L"),
                Landscape::Swamp => print!("S"),
                Landscape::Castle => print!("C"),
                Landscape::None => print!("."),
            }
            match board.square_crowns(r + c) {
                0 => print!("  "),
                1 => print!("1 "),
                2 => print!("2 "),
                3 => print!("3 "),
                _ => unreachable!("invalid number of crowns"),
            }
        }
        println!();
    }
}

