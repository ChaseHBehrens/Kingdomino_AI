use kingdomino_ai::rng;
use kingdomino_ai::run_game;
use kingdomino_ai::simulation::simulated_deal;
use kingdomino_ai::evaluators::{ eoh_test, update_eoh_test, greedy };
use kingdomino_ai::user_interface::{ loading_bar, loading_finished };

fn main() {
    let mut best_weights: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
    let mut best_scores: u32 = 0;
    for n in 0..14641 {
        rng::seed(5);
        loading_bar(n, 14641);
        let i = n / 1331;
        let j = (n / 121) % 11;
        let k = (n / 11) % 11;
        let l = n % 11;
        if i + j + k + l != 10 { continue; }
        let test_weights = [
            i as f32 * 0.1,
            j as f32 * 0.1,
            k as f32 * 0.1,
            l as f32 * 0.1,
        ];
        update_eoh_test(test_weights);

        let mut test_scores: u32 = 0;
        for _ in 0..100 {
            let game = run_game!(
                dealer: simulated_deal, 
                players: [eoh_test, greedy, greedy, greedy],
                additional_rules: [MIDDLE_KINGDOM, HARMONY],
            ).map(|(_, score)| score);
            test_scores += game[0];
            
            let game = run_game!(
                dealer: simulated_deal, 
                players: [greedy, eoh_test, greedy, greedy],
                additional_rules: [MIDDLE_KINGDOM, HARMONY],
            ).map(|(_, score)| score);
            test_scores += game[1];

            let game = run_game!(
                dealer: simulated_deal, 
                players: [greedy, greedy, eoh_test, greedy],
                additional_rules: [MIDDLE_KINGDOM, HARMONY],
            ).map(|(_, score)| score);
            test_scores += game[2];
            
            let game = run_game!(
                dealer: simulated_deal, 
                players: [greedy, greedy, greedy, eoh_test],
                additional_rules: [MIDDLE_KINGDOM, HARMONY],
            ).map(|(_, score)| score);
            test_scores += game[3];
        }
        if test_scores > best_scores {
            best_scores = test_scores;
            best_weights = test_weights;
        }
    }
    loading_finished(14641);
    println!("Best Weights: {:?}", best_weights);
}
