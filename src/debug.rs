
pub fn print_bitboard(b: u32) {
    for r in (0..25).step_by(5).rev() {
        for c in 0..5 {
            if ((1 << (r + c)) & b) != 0 { print!(" X") }
            else { print!(" .") }
        }
        println!();
    }
}

