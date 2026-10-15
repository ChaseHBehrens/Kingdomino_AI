//! Terminal evaluator that lets a person choose the moves.
//!
//! One screen shows every kingdom, the draw pile and our kingdom as it would
//! look after the selected move. Left and right step through the placements
//! of the selected domino, running on into the next domino at either end.
//! Up and down switch domino, jumping to its first placement.
use std::io::{ self, Write };
use std::sync::Mutex;
use crossterm::{ event::{ self, Event, KeyCode, KeyEventKind, KeyModifiers }, terminal };
use crate::core::{ Board, GameState, Landscape, Ruleset, Turn, DOMINOES };

const FULL: u32 = 0x1FFFFFF;
const TAKEN: u8 = u8::MAX;

/// Marks squares that were just placed and the selected domino
const HIGHLIGHT: &str = "\x1b[7m";
const RESET: &str = "\x1b[0m";
const CLEAR: &str = "\x1b[2J\x1b[H";

/// Width of a board column; boards are 15 columns wide
const COLUMN: usize = 20;

// Nerd Font icons (https://www.nerdfonts.com), mostly Material Design
const FARM: char = '\u{F0073}';
const FIELD: char = '\u{EEF1}';
const FOREST: char = '\u{F0405}';
const MINE: char = '\u{F08B7}';
const LAKE: char = '\u{F078D}';
const SWAMP: char = '\u{EDF8}';
const CASTLE: char = '\u{F011A}';

/// Names shown for each seat
static NAMES: Mutex<Vec<String>> = Mutex::new(Vec::new());
/// For each seat, the boards as they were at the end of its last turn
static PREVIOUS: Mutex<Vec<Vec<Board>>> = Mutex::new(Vec::new());

/// Sets the names shown for each seat
pub fn set_player_names(names: Vec<String>) {
    *NAMES.lock().unwrap() = names;
}

fn player_name(p: usize) -> String {
    NAMES.lock().unwrap().get(p).cloned().unwrap_or_else(|| format!("Player {}", p + 1))
}

/// Colour of each landscape, from the 256 colour palette
fn color(l: Landscape) -> &'static str {
    match l {
        Landscape::Farm => "\x1b[38;5;178m",
        Landscape::Field => "\x1b[38;5;112m",
        Landscape::Forest => "\x1b[38;5;28m",
        Landscape::Mine => "\x1b[38;5;160m",
        Landscape::Lake => "\x1b[38;5;32m",
        Landscape::Swamp => "\x1b[38;5;133m",
        Landscape::Castle => "",
        Landscape::None => "",
    }
}

/// `text` in the colour of a landscape
fn paint(l: Landscape, text: &str) -> String {
    format!("{}{text}{RESET}", color(l))
}

fn icon(l: Landscape) -> char {
    match l {
        Landscape::Farm => FARM,
        Landscape::Field => FIELD,
        Landscape::Forest => FOREST,
        Landscape::Mine => MINE,
        Landscape::Lake => LAKE,
        Landscape::Swamp => SWAMP,
        Landscape::Castle => CASTLE,
        Landscape::None => '·',
    }
}

/// Squares and crowns of each landscape on the dominoes still to come: those
/// left in the deck and those not yet taken from the draw pile
fn remaining<const N: usize, const S: usize, R: Ruleset>(state: &GameState<N, S, R>) -> String {
    const LANDSCAPES: [(Landscape, &str); 6] = [
        (Landscape::Farm, "Farm"),
        (Landscape::Field, "Field"),
        (Landscape::Forest, "Forest"),
        (Landscape::Mine, "Mine"),
        (Landscape::Lake, "Lake"),
        (Landscape::Swamp, "Swamp"),
    ];
    let mut squares = [0; 6];
    let mut crowns = [0; 6];
    let pile = state.draw_pile.iter().filter(|&&d| d != TAKEN).map(|&d| d as usize);
    let deck = (0..48).filter(|&d| (state.deck >> d) & 1 == 1);
    for d in deck.chain(pile) {
        let dom = DOMINOES[d];
        squares[dom.0 as usize] += 1;
        squares[dom.1 as usize] += 1;
        crowns[dom.0 as usize] += dom.2 as u32;
    }
    let counts: Vec<String> = LANDSCAPES.iter()
        .map(|&(l, name)| paint(l, &format!("{} {name} {}/{}", icon(l), squares[l as usize], crowns[l as usize])))
        .collect();
    format!("Still to come (squares/crowns):  {}", counts.join("   "))
}

/// A square, three columns wide: its crowns, its icon, then a space that
/// Nerd Font icons wider than one column can spill into. The space shares
/// the colour so a highlighted square is one solid block.
pub(super) fn cell(l: Landscape, crowns: u8) -> String {
    let crowns = if crowns > 0 { char::from(b'0' + crowns) } else { ' ' };
    paint(l, &format!("{crowns}{} ", icon(l)))
}

fn highlight(s: &str) -> String {
    // Colours end with a reset, which would also end the highlight
    format!("{HIGHLIGHT}{}{RESET}", s.replace(RESET, &format!("{RESET}{HIGHLIGHT}")))
}

/// Castle position in board coordinates (row, column)
fn castle(board: &Board) -> (i32, i32) {
    let c = (0..6).fold(FULL, |a, l| a & board.landscape(l));
    let sq = c.trailing_zeros() as i32;
    (sq / 5, sq % 5)
}

/// Whether the square at (rows, columns) from the castle is occupied
fn occupied_at(board: &Board, dr: i32, dc: i32) -> bool {
    let (cr, cc) = castle(board);
    let (r, c) = (cr + dr, cc + dc);
    (0..5).contains(&r) && (0..5).contains(&c)
        && board.square_landscape((r * 5 + c) as u8) != Landscape::None
}

/// Squares occupied in `board` but not in `before` as (row, column) from the
/// castle. Boards shift as kingdoms grow, so squares are matched this way.
fn added(before: &Board, board: &Board) -> Vec<(i32, i32)> {
    let (cr, cc) = castle(board);
    (0..25)
        .filter(|&sq| board.square_landscape(sq as u8) != Landscape::None)
        .map(|sq| (sq / 5 - cr, sq % 5 - cc))
        .filter(|&(dr, dc)| !occupied_at(before, dr, dc))
        .collect()
}

/// The board's 5x5 window, highlighting squares that are empty in `before`
fn board_lines(board: &Board, before: Option<&Board>) -> Vec<String> {
    let new = before.map(|b| added(b, board)).unwrap_or_default();
    let (cr, cc) = castle(board);
    (0..5).rev().map(|r| {
        (0..5).map(|c| {
            let sq = (r * 5 + c) as u8;
            let s = cell(board.square_landscape(sq), board.square_crowns(sq));
            if new.contains(&(r - cr, c - cc)) { highlight(&s) } else { s }
        }).collect()
    }).collect()
}

/// Boards side by side under their titles
fn side_by_side(titles: &[String], boards: &[Vec<String>]) -> Vec<String> {
    let mut lines = vec![titles.iter().map(|t| format!("{t:<COLUMN$}")).collect()];
    for row in 0..5 {
        lines.push(boards.iter().map(|b| format!("{}{}", b[row], " ".repeat(COLUMN - 15))).collect());
    }
    lines
}

/// Prints the final boards and scores, best first
pub fn show_results(results: &[(Board, u32)]) {
    println!("\nFinal kingdoms\n");
    let mut order: Vec<usize> = (0..results.len()).collect();
    order.sort_by_key(|&p| std::cmp::Reverse(results[p].1));
    let titles: Vec<String> = order.iter().map(|&p| format!("{} {}", player_name(p), results[p].1)).collect();
    let boards: Vec<Vec<String>> = order.iter().map(|&p| board_lines(&results[p].0, None)).collect();
    for line in side_by_side(&titles, &boards) { println!("{line}"); }
    let best = results.iter().map(|r| r.1).max().unwrap();
    let winners: Vec<String> = (0..results.len()).filter(|&p| results[p].1 == best).map(player_name).collect();
    println!("\nWinner: {} with {} points", winners.join(" and "), best);
}

/// Placements of one domino in the draw pile
struct Choice {
    position: usize,
    /// Turn indices, ordered by where the domino goes
    turns: Vec<usize>,
    /// The domino fits nowhere, so taking it discards it
    discard: bool,
}

fn quit() -> ! {
    let _ = terminal::disable_raw_mode();
    println!("\r\nGame abandoned.");
    std::process::exit(0)
}

/// Evaluator that asks the person at the terminal for a move
pub fn human<const N: usize, const S: usize, R: Ruleset>(
    state: &GameState<N, S, R>,
    turns: &[Turn],
) -> usize {
    let me = state.player_order[0] as usize;
    let board = state.boards[me];
    let previous = PREVIOUS.lock().unwrap().get(me).cloned().unwrap_or_default();

    // Group the turns by domino, ordering placements top left to bottom right
    let mut choices: Vec<Choice> = Vec::new();
    for (i, turn) in turns.iter().enumerate() {
        match choices.iter_mut().find(|c| c.position == turn.position) {
            Some(c) => c.turns.push(i),
            None => choices.push(Choice { position: turn.position, turns: vec![i], discard: false }),
        }
    }
    for c in choices.iter_mut() {
        c.discard = c.turns.iter().all(|&i| turns[i].board.occupied() == board.occupied());
        let key = |i: &usize| {
            let mut squares = added(&board, &turns[*i].board);
            squares.sort_by_key(|&(dr, dc)| (-dr, dc));
            squares.iter().map(|&(dr, dc)| (-dr, dc)).collect::<Vec<_>>()
        };
        c.turns.sort_by_cached_key(key);
    }

    // Every kingdom with its current score, in the order the players move
    let mut top = vec![
        format!("{}'s turn", player_name(me)),
        remaining(state),
        String::new(),
    ];
    let mut order: Vec<usize> = Vec::new();
    for &p in &state.player_order {
        if !order.contains(&(p as usize)) { order.push(p as usize); }
    }
    let titles: Vec<String> = order.iter().map(|&p| format!("{} {}", player_name(p), state.score(p))).collect();
    let boards: Vec<Vec<String>> = order.iter().map(|&p| board_lines(&state.boards[p], previous.get(p))).collect();
    top.extend(side_by_side(&titles, &boards));
    if !previous.is_empty() { top.push("Highlighted squares were placed since your last turn.".into()); }
    top.push(String::new());

    let (mut d, mut k) = (0, 0);
    let _ = terminal::enable_raw_mode();
    loop {
        let choice = &choices[d];
        let turn = &turns[choice.turns[k]];

        // The draw pile beside our kingdom after the selected move
        let pile: Vec<String> = state.draw_pile.iter().enumerate().map(|(pos, &dom)| {
            if dom == TAKEN { return format!("  {:<6}", "--"); }
            let dom = DOMINOES[dom as usize];
            let s = format!("{}{}", cell(dom.0, dom.2), cell(dom.1, 0));
            if pos == choice.position { format!("> {}", highlight(&s)) } else { format!("  {s}") }
        }).collect();
        let after = board_lines(&turn.board, Some(&board));
        let mut lines = top.clone();
        lines.push(format!("{:<COLUMN$}{}", "Draw pile", "Your kingdom after this move"));
        for row in 0..5 {
            // Pile entries are 8 columns wide
            let left = pile.get(row).map(|p| format!("{p}{}", " ".repeat(COLUMN - 8))).unwrap_or(" ".repeat(COLUMN));
            lines.push(format!("{left}{}", after[row]));
        }
        lines.push(String::new());
        lines.push(if choice.discard {
            "This domino fits nowhere: taking it discards it.".into()
        } else {
            format!("Placement {} of {}", k + 1, choice.turns.len())
        });
        lines.push("←/→ placements   ↑/↓ dominoes   Enter play   Ctrl+C quit".into());
        // Raw mode needs explicit carriage returns
        print!("{CLEAR}{}", lines.join("\r\n"));
        let _ = io::stdout().flush();

        let Ok(Event::Key(key)) = event::read() else { continue };
        if key.kind != KeyEventKind::Press { continue; }
        let n = choices.len();
        match key.code {
            KeyCode::Right => {
                k += 1;
                if k == choices[d].turns.len() { d = (d + 1) % n; k = 0; }
            }
            KeyCode::Left => {
                if k == 0 { d = (d + n - 1) % n; k = choices[d].turns.len() - 1; } else { k -= 1; }
            }
            KeyCode::Down => { d = (d + 1) % n; k = 0; }
            KeyCode::Up => { d = (d + n - 1) % n; k = 0; }
            KeyCode::Enter => break,
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => quit(),
            _ => {}
        }
    }
    let _ = terminal::disable_raw_mode();

    let chosen = choices[d].turns[k];
    let mut boards = state.boards.to_vec();
    boards[me] = turns[chosen].board;
    let mut previous = PREVIOUS.lock().unwrap();
    if previous.len() < N { previous.resize(N, Vec::new()); }
    previous[me] = boards;
    println!("\r\n\r\nWaiting for the next player...");
    chosen
}
