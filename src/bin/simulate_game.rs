use kingdomino_ai::rng;
use kingdomino_ai::run_game;
use kingdomino_ai::user_interface::display_board;
use kingdomino_ai::simulation::simulated_deal;
use kingdomino_ai::evaluators::{ 
    mce_score, 
    mce_heuristic, 
    mce_race,
    eoh_trained,
};

fn main() {
    rng::init();
    //let mut score_sum: u32 = 0;
    //for _ in 0..1000 {
    //    let game = run_game!(
    //        dealer: simulated_deal, 
    //        players: [midas, midas, midas, midas],
    //        additional_rules: [MIDDLE_KINGDOM, HARMONY],
    //    );
    //    score_sum += game.into_iter().map(|b| b.score()).sum::<u32>();
    //}
    //let average = score_sum as f32 / 4000.0;
    //println!("{average}")
    
    let game = run_game!(
        dealer: simulated_deal, 
        players: [mce_heuristic, eoh_trained, mce_race, mce_score],
        additional_rules: [MIDDLE_KINGDOM, HARMONY],
    );
    for (i, (board, score)) in game.iter().enumerate() {
        println!("player {} scored {} points", i + 1, score);
        display_board(board);
    }
}
