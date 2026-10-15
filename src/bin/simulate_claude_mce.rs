use std::fs::File;
use std::io::{ self, Write };
use kingdomino_ai::rng;
use kingdomino_ai::run_game;
use kingdomino_ai::evaluators::claude_mce;
use kingdomino_ai::simulation::simulated_deal;


fn main() -> io::Result<()> {
    rng::init();
    
    // 4 player matches
    let mut file = File::create("claude_mce_scores/four_player.csv")?;
    writeln!(file, "Player1,Player2,Player3,Player4")?;
    for _ in 0..100 {
        let game: Vec<u32> = run_game!(
            dealer: simulated_deal, 
            players: [claude_mce, claude_mce, claude_mce, claude_mce],
            additional_rules: [MIDDLE_KINGDOM, HARMONY],
        ).into_iter().map(|b| b.1).collect();
        writeln!(file, "{},{},{},{}", game[0], game[1], game[2], game[3])?;
    }

    // 3 player matches
    let mut file = File::create("claude_mce_scores/three_player.csv")?;
    writeln!(file, "Player1,Player2,Player3")?;
    for _ in 0..100 {
        let game: Vec<u32> = run_game!(
            dealer: simulated_deal, 
            players: [claude_mce, claude_mce, claude_mce],
            additional_rules: [MIDDLE_KINGDOM, HARMONY],
        ).into_iter().map(|b| b.1).collect();
        writeln!(file, "{},{},{}", game[0], game[1], game[2])?;
    }

    // 2 player matches
    let mut file = File::create("claude_mce_scores/two_player.csv")?;
    writeln!(file, "Player1,Player2")?;
    for _ in 0..100 {
        let game: Vec<u32> = run_game!(
            dealer: simulated_deal, 
            players: [claude_mce, claude_mce],
            additional_rules: [MIDDLE_KINGDOM, HARMONY],
        ).into_iter().map(|b| b.1).collect();
        writeln!(file, "{},{}", game[0], game[1])?;
    }

    Ok(())
}
