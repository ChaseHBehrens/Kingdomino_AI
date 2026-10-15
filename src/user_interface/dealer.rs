//! Dealer that asks the person at the terminal which domino was drawn, for
//! playing with a real set of tiles. It works like `simulated_deal`, except
//! the person picks each domino from those still in the deck.
//!
//! Dominoes are listed by the number printed on the back of the tile. Typing
//! filters the list by number or by landscape name.
//!
//! In 2 and 3 player games some dominoes are set aside before the game. The
//! game has to know which ones, so the person either enters them or lets the
//! dealer choose them and takes those tiles out of the box.
use std::fmt;
use std::sync::Mutex;
use inquire::{ InquireError, Select };
use crate::core::{ Landscape, DOMINOES };
use crate::simulation::simulated_deal;
use super::human::cell;

/// Landscape names that filter the list
const NAMES: [(Landscape, &str); 6] = [
    (Landscape::Farm, "farm"),
    (Landscape::Field, "field"),
    (Landscape::Forest, "forest"),
    (Landscape::Mine, "mine"),
    (Landscape::Lake, "lake"),
    (Landscape::Swamp, "swamp"),
];

/// Whether the dealer chooses the dominoes set aside before the game
static CHOOSE_SET_ASIDE: Mutex<bool> = Mutex::new(false);
/// Dominoes set aside so far, when the dealer chooses them
static SET_ASIDE: Mutex<Vec<u8>> = Mutex::new(Vec::new());
/// Dominoes entered so far for the draw pile being dealt
static PILE: Mutex<Vec<u8>> = Mutex::new(Vec::new());

/// A domino, shown as its tile number and its two squares
#[derive(Clone, Copy)]
struct Tile(u8);

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dom = DOMINOES[self.0 as usize];
        write!(f, "{:>2}  {}{}", self.0 + 1, cell(dom.0, dom.2), cell(dom.1, 0))
    }
}

/// Keeps tiles whose number starts with the input or that have a landscape
/// whose name does, in tile order with an exact number first
fn score(input: &str, tile: &Tile, _: &str, _: usize) -> Option<i64> {
    let input = input.trim().to_lowercase();
    let number = (tile.0 + 1).to_string();
    let dom = DOMINOES[tile.0 as usize];
    let landscape = NAMES.iter()
        .any(|&(l, name)| (dom.0 == l || dom.1 == l) && name.starts_with(input.as_str()));
    if number == input { Some(0) }
    else if number.starts_with(input.as_str()) || landscape { Some(-(number.len() as i64 * 100) - tile.0 as i64) }
    else { None }
}

/// Asks which of the dominoes in `contents` was drawn
fn ask(contents: u64, message: &str) -> u8 {
    loop {
        let tiles: Vec<Tile> = (0..48).filter(|&d| (contents >> d) & 1 == 1).map(Tile).collect();
        let answer = Select::new(message, tiles)
            .with_scorer(&score)
            .with_page_size(12)
            .with_help_message("type a tile number or landscape to filter, enter to select")
            .prompt();
        match answer {
            Ok(tile) => return tile.0,
            // Escape asks again rather than ending the game
            Err(InquireError::OperationCanceled) => continue,
            Err(_) => {
                println!("Game abandoned.");
                std::process::exit(0)
            }
        }
    }
}

/// Dealer for a game of N players with S dominoes in each draw pile that
/// asks the person at the terminal for each domino
pub fn user_deal<const N: usize, const S: usize>(contents: u64) -> u8 {
    let left = contents.count_ones() as usize;
    let used = 12 * N;

    // Before the game, setting dominoes aside
    if left > used {
        let (count, total) = (48 - left, 48 - used);
        if count == 0 {
            let options = vec!["Choose them for me", "I'll enter them"];
            let message = format!("{total} dominoes are set aside in a {N} player game. Which ones?");
            let choose = match Select::new(&message, options).raw_prompt() {
                Ok(option) => option.index == 0,
                Err(_) => std::process::exit(0),
            };
            *CHOOSE_SET_ASIDE.lock().unwrap() = choose;
            SET_ASIDE.lock().unwrap().clear();
        }
        if !*CHOOSE_SET_ASIDE.lock().unwrap() {
            return ask(contents, &format!("Set-aside domino {} of {total}?", count + 1));
        }
        let domino = simulated_deal(contents);
        let mut set_aside = SET_ASIDE.lock().unwrap();
        set_aside.push(domino);
        if set_aside.len() == total {
            set_aside.sort_unstable();
            println!("Take these dominoes out of the game:");
            for row in set_aside.chunks(6) {
                let tiles: Vec<String> = row.iter().map(|&d| Tile(d).to_string()).collect();
                println!("  {}", tiles.join("   "));
            }
            println!();
        }
        return domino;
    }

    // During the game, filling the draw pile
    let drawn = used - left;
    let (round, slot) = (drawn / S + 1, drawn % S + 1);
    let mut pile = PILE.lock().unwrap();
    if slot == 1 { pile.clear(); }
    let mut message = format!("Round {round} of {}: domino {slot} of {S} for the draw pile?", used / S);
    if !pile.is_empty() {
        let so_far: Vec<String> = pile.iter().map(|&d| Tile(d).to_string()).collect();
        message += &format!("  (so far: {})", so_far.join("   "));
    }
    let domino = ask(contents, &message);
    pile.push(domino);
    domino
}
