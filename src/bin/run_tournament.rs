//! File Generated Partly With Claude Opus 5.5
use std:: {
    cell::Cell,
    cmp::Ordering,
    env,
    fs,
    io::ErrorKind,
};
use kingdomino_ai::{
    rng,
    run_game,
    evaluators::evaluator,
    simulation::simulated_deal,
    user_interface::loading_bar,
};

const EVALUATORS: &[&str] = &[
    "random",
    "greedy",
    "mce score",
    "mce place",
    "mce lead",
    "mce race",
    "mce halving",
    "mce eoh",
    "nn score",
    "eoh",
    "mce heuristic",
    "claude heuristic",
];

const DEFAULT_MU: f64 = 25.0;
const DEFAULT_SIGMA: f64 = 25.0 / 3.0;
const BETA: f64 = 25.0 / 6.0;
const KAPPA: f64 = 0.0001;

struct Player {
    name: String,
    mu: Cell<f64>,
    sigma: Cell<f64>,
}

impl Player {
    fn ordinal(&self) -> f64 {
        self.mu.get() - 3.0 * self.sigma.get()
    }
}

fn parse_line(line: &str) -> Option<(&str, f64, f64)> {
    let mut it = line.trim().rsplitn(3, ',');
    let sigma = it.next()?.trim().parse().ok()?;
    let mu = it.next()?.trim().parse().ok()?;
    let name = it.next()?.trim();
    if name.is_empty() {
        return None;
    }
    Some((name, mu, sigma))
}

fn load_players(path: &str, names: &[&str]) -> Vec<Player> {
    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) if e.kind() == ErrorKind::NotFound => String::new(),
        Err(e) => panic!("Failed to read {path}: {e}"),
    };
    let existing: Vec<(&str, f64, f64)> = contents.lines().filter_map(parse_line).collect();

    names
        .iter()
        .map(|&name| {
            let (mu, sigma) = existing
                .iter()
                .find(|(n, _, _)| *n == name)
                .map(|&(_, mu, sigma)| (mu, sigma))
                .unwrap_or((DEFAULT_MU, DEFAULT_SIGMA));
            Player {
                name: name.to_string(),
                mu: Cell::new(mu),
                sigma: Cell::new(sigma),
            }
        })
        .collect()
}

fn save_players(path: &str, players: &[Player]) {
    let mut entries: Vec<(String, f64, f64)> = match fs::read_to_string(path) {
        Ok(contents) => contents
            .lines()
            .filter_map(parse_line)
            .map(|(n, mu, sigma)| (n.to_string(), mu, sigma))
            .collect(),
        Err(e) if e.kind() == ErrorKind::NotFound => Vec::new(),
        Err(e) => panic!("Failed to read {path}: {e}"),
    };

    for p in players {
        match entries.iter_mut().find(|(n, _, _)| *n == p.name) {
            Some(entry) => {
                entry.1 = p.mu.get();
                entry.2 = p.sigma.get();
            }
            None => entries.push((p.name.clone(), p.mu.get(), p.sigma.get())),
        }
    }

    let out: String = entries
        .iter()
        .map(|(n, mu, sigma)| format!("{n},{mu},{sigma}\n"))
        .collect();

    let tmp = format!("{path}.tmp");
    fs::write(&tmp, out).unwrap_or_else(|e| panic!("Failed to write {tmp}: {e}"));
    fs::rename(&tmp, path).unwrap_or_else(|e| panic!("Failed to replace {path}: {e}"));
}

fn rate(players: &[Player], seats: [usize; 4], scores: [u32; 4]) {
    let mut omega = vec![0.0; players.len()];
    let mut delta = vec![0.0; players.len()];

    for a in 0..4 {
        for b in 0..4 {
            if seats[a] == seats[b] { continue; }

            let pa = &players[seats[a]];
            let pb = &players[seats[b]];
            let (mu_a, mu_b) = (pa.mu.get(), pb.mu.get());
            let (sig_a, sig_b) = (pa.sigma.get(), pb.sigma.get());

            let c = (sig_a * sig_a + sig_b * sig_b + 2.0 * BETA * BETA).sqrt();
            let p = 1.0 / (1.0 + ((mu_b - mu_a) / c).exp());
            let s = match scores[a].cmp(&scores[b]) {
                Ordering::Greater => 1.0,
                Ordering::Equal => 0.5,
                Ordering::Less => 0.0,
            };

            omega[seats[a]] += sig_a * sig_a / c * (s - p);
            delta[seats[a]] += (sig_a / c) * sig_a * sig_a / (c * c) * p * (1.0 - p);
        }
    }

    for (i, p) in players.iter().enumerate() {
        p.mu.set(p.mu.get() + omega[i]);
        p.sigma.set(p.sigma.get() * (1.0 - delta[i]).max(KAPPA).sqrt());
    }
}

fn play_game(players: &[Player]) {
    let seats: [usize; 4] = std::array::from_fn(|_| {
        rng::rng(players.len() as u32) as usize
    });
    let scores: [u32; 4] = run_game!(
        dealer: simulated_deal,
        players: [
            evaluator(&players[seats[0]].name),
            evaluator(&players[seats[1]].name),
            evaluator(&players[seats[2]].name),
            evaluator(&players[seats[3]].name),
        ],
        additional_rules: [MIDDLE_KINGDOM, HARMONY],
    ).map(|(_board, score)| score);
    rate(players, seats, scores);
}

fn print_leaderboard(players: &[Player]) {
    let mut sorted: Vec<&Player> = players.iter().collect();
    sorted.sort_by(|a, b| b.ordinal().total_cmp(&a.ordinal()));
    println!("{:<10} | {:>8} | {:>8} | {:>8}", "name", "ordinal", "mu", "sigma");
    for p in sorted {
        println!(
            "{:<10} | {:>8.2} | {:>8.2} | {:>8.2}",
            p.name,
            p.ordinal(),
            p.mu.get(),
            p.sigma.get()
        );
    }
}

fn main() {
    rng::init();
    let path = "ratings.csv";

    let args: Vec<String> = env::args().collect();
    let game_count: usize = if args.len() > 1 {
        args[1].parse().unwrap_or_else(|_| {
            eprintln!("Invalid number provided, defaulting to 10 games.");
            10
        })
    } else {
        10
    };

    let players = load_players(path, EVALUATORS);
    for i in 0..game_count {
        loading_bar(i, game_count);
        play_game(&players);
    }
    loading_bar(game_count, game_count);
    println!();
    print_leaderboard(&players);
    save_players(path, &players);
}
