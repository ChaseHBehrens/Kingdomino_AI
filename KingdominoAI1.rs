mod dominos;
use dominos::{DOMINOS};
use std::io;

/// * 'position' - the current board position.
/// * 'available_spots' - where tiles can be placed indexed by type.
#[derive(Clone)]
#[derive(PartialEq)]
struct Board {
    biomes: [[usize; 9]; 9],
    crowns: [[usize; 9]; 9],
    available_spots: [[[bool; 9]; 9]; 6],
}
impl Board {
    fn new() -> Self {
        let mut available_spots: [[[bool; 9]; 9]; 6] = [[[false; 9]; 9]; 6];
        for i in 0..6 {
            available_spots[i][3][4] = true;
            available_spots[i][4][3] = true;
            available_spots[i][5][4] = true;
            available_spots[i][4][5] = true;
        }
        let mut biomes: [[usize; 9]; 9] = [[0; 9]; 9];
        biomes[4][4] = dominos::CASTLE as usize;
        return Self {
            biomes: biomes, 
            crowns: [[0; 9]; 9],
            available_spots: available_spots,
        };
    }
}

/// gets an intiger input from the user within a range.
/// # Arguments
/// * 'min' - The minimum input allowed.
/// * 'max' - The maximum input allowed.
/// # Returns
/// * The users input. 
fn get_int_input(min: usize, max: usize,) -> usize {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");
        match input.trim().parse::<usize>() {
            Ok(number) if (min..=max).contains(&number) => {
                return number;
            }
            Ok(_) => {
                println!("Out of range. Enter a number between {} and {}:", min, max);
                continue;
            }
            Err(_) => {
                println!("Invalid input. Enter an intiger:");
                continue;
            }
        }
    }
}

/// gets available dominos to choose from.
/// * 'number_of_players' the number of players in the game.
/// # Returns
/// * a vector of the tile index of all available choises.
fn get_available_dominos(mut number_of_players: usize,) -> Vec<usize> {
    let mut available_dominos: Vec<usize> = Vec::new();
    println!("Enter available domino numbers, or 0 if no more available:");
    let mut first: bool = true;
    loop {
        let input_domino: usize = get_int_input(0, 48);
        if input_domino == 0 {
            if first {
                println!("There must be at least one available domino.");
                continue;
            } else {
                return available_dominos;
            }
        }
        available_dominos.push(input_domino - 1);
        first = false;
        number_of_players -= 1;
        if number_of_players == 0 {return available_dominos;}
    }
}

/// places a domino onto board.
/// * 'board' the board being updated.
/// * 'domino' the domino being placed.
/// * 'positions' the positions to place the dominos.
/// # Returns
/// * a new board with the added domino.
fn palce_domino(board: &Board, domino: usize, positions: [[usize; 2]; 2],) -> Board {
    let mut board = board.clone();
    for i in 0..2 {
        board.biomes[positions[i][0]][positions[i][1]] = dominos::DOMINOS[domino][i].biome;
        board.crowns[positions[i][0]][positions[i][1]] = dominos::DOMINOS[domino][i].crowns;
        for j in 0..6 {
            board.available_spots[j][positions[i][0]][positions[i][1]] = false;
        }
        let biome: usize = dominos::DOMINOS[domino][i].biome;
        if positions[i][0] > 0 && board.biomes[positions[i][0] - 1][positions[i][1]] == 0 {
            board.available_spots[biome - 1][positions[i][0] - 1][positions[i][1]] = true;
        }
        if positions[i][0] < 8 && board.biomes[positions[i][0] + 1][positions[i][1]] == 0 {
            board.available_spots[biome - 1][positions[i][0] + 1][positions[i][1]] = true;
        }
        if positions[i][1] > 0 && board.biomes[positions[i][0]][positions[i][1] - 1] == 0 {
            board.available_spots[biome - 1][positions[i][0]][positions[i][1] - 1] = true;
        }
        if positions[i][1] < 8 && board.biomes[positions[i][0]][positions[i][1] + 1] == 0 {
            board.available_spots[biome - 1][positions[i][0]][positions[i][1] + 1] = true;
        }
        if positions[i][0] > 4 {
            for k in 0..9 {
                for j in 0..6 {
                    board.available_spots[j][positions[i][0] - 5][k] = false;
                }
                board.biomes[positions[i][0] - 5][k] = 8;
            }
        }
        if positions[i][0] < 4 {
            for k in 0..9 {
                for j in 0..6 {
                    board.available_spots[j][positions[i][0] + 5][k] = false;
                }
                board.biomes[positions[i][0] + 5][k] = 8;
            }
        }
        if positions[i][1] > 4 {
            for k in 0..9 {
                for j in 0..6 {
                    board.available_spots[j][k][positions[i][1] - 5] = false;
                }
                board.biomes[k][positions[i][1] - 5] = 8;
            }
        }
        if positions[i][1] < 4 {
            for k in 0..9 {
                for j in 0..6 {
                    board.available_spots[j][k][positions[i][1] + 5] = false;
                }
                board.biomes[k][positions[i][1] + 5] = 8;
            }
        }
    }
    return board;
}

/// finds all possible placements of a domino.
/// * 'board' the board the domino is being placed onto.
/// # Return
/// * a list of all possible boards.
fn possible_domino_placements(board: &Board, dominos: &Vec<usize>,) -> Vec<(Board, usize)> {
    let mut possible_placements: Vec<(Board, usize)> = Vec::new();
    for domino in dominos {
        for i in 0..2 {
            for x in 0..9 {
                for y in 0..9 {
                    if board.available_spots[dominos::DOMINOS[*domino][i].biome - 1][x][y] {
                        if x > 0 && board.biomes[x - 1][y] == 0 {
                            let new_board: Board;
                            if i == 0 {
                                new_board = palce_domino(&board, *domino, [[x, y], [x - 1, y]]);
                            } else {
                                new_board = palce_domino(&board, *domino, [[x - 1, y], [x, y]]);
                            }
                            possible_placements.push((new_board, *domino));
                        }
                        if x < 8 && board.biomes[x + 1][y] == 0 {
                            let new_board: Board;
                            if i == 0 {
                                new_board = palce_domino(&board, *domino, [[x, y], [x + 1, y]]);
                            } else {
                                new_board = palce_domino(&board, *domino, [[x + 1, y], [x, y]]);
                            }
                            possible_placements.push((new_board, *domino));
                        }
                        if y > 0 && board.biomes[x][y - 1] == 0 {
                            let new_board: Board;
                            if i == 0 {
                                new_board = palce_domino(&board, *domino, [[x, y], [x, y - 1]]);
                            } else {
                                new_board = palce_domino(&board, *domino, [[x, y - 1], [x, y]]);
                            }
                            possible_placements.push((new_board, *domino));
                        }
                        if y < 8 && board.biomes[x][y + 1] == 0 {
                            let new_board: Board;
                            if i == 0 {
                                new_board = palce_domino(&board, *domino, [[x, y], [x, y + 1]]);
                            } else {
                                new_board = palce_domino(&board, *domino, [[x, y + 1], [x, y]]);
                            }
                            possible_placements.push((new_board, *domino));
                        }
                    }
                }
            }
        }
    }
    return possible_placements;
}

/// displays the board state to the user.
/// * 'board' the board being displayed.
fn display_board(board: &Board) {
    for y in 0..9 {
        let mut new_line: u8 = 0;
        for x in 0..9 {
            match board.biomes[x][y] {
                dominos::GRASS => print!("\x1B[38;5;82mG{} \x1B[0m",board.crowns[x][y]),
                dominos::FARM => print!("\x1B[33mF{} \x1B[0m",board.crowns[x][y]),
                dominos::LAKE => print!("\x1B[34mL{} \x1B[0m",board.crowns[x][y]),
                dominos::FOREST => print!("\x1B[32mT{} \x1B[0m",board.crowns[x][y]),
                dominos::SWAMP => print!("\x1B[38;5;94mS{} \x1B[0m",board.crowns[x][y]),
                dominos::MINE => print!("\x1B[90mM{} \x1B[0m",board.crowns[x][y]),
                dominos::CASTLE => print!("C  "),
                8 => new_line += 1,
                _ => print!(".  "),
            }
        }
        if new_line != 9 {print!("\n");}
    }
}

/// Scores a board.
/// * 'board' the board being scored.
/// # Returns
/// * the score of the board.
fn evaluate_board(board: &Board) -> usize {
    struct Group {
        total_squares: usize,
        total_crowns: usize,
    }
    impl Group {
        fn score(&self) -> usize {
            return self.total_squares * self.total_crowns;
        }
    }
    let mut groups: Vec<Group> = Vec::new();
    let mut grouped_squares: [[bool; 9]; 9] = [[false; 9]; 9];
    for i in 0..9 {
        for j in 0..9 {
            if grouped_squares[i][j] {continue;}
            if board.biomes[i][j] == 0 || board.biomes[i][j] >= 7 {
                grouped_squares[i][j] = true;
            } else {
                let mut group = Group{total_squares: 0, total_crowns: 0,};
                let mut que: Vec<[usize; 2]> = vec![[i, j]];
                grouped_squares[i][j] = true;
                while !que.is_empty() {
                    let x = que[0][0];
                    let y = que[0][1];
                    group.total_squares += 1;
                    group.total_crowns += board.crowns[x][y];
                    if x > 0 && !grouped_squares[x - 1][y] && board.biomes[x][y] == board.biomes[x - 1][y] {
                        que.push([x - 1, y]);
                        grouped_squares[x - 1][y] = true;
                    }
                    if x < 8 && !grouped_squares[x + 1][y] && board.biomes[x][y] == board.biomes[x + 1][y] {
                        que.push([x + 1, y]);
                        grouped_squares[x + 1][y] = true;
                    }
                    if y > 0 && !grouped_squares[x][y - 1] && board.biomes[x][y] == board.biomes[x][y - 1] {
                        que.push([x, y - 1]);
                        grouped_squares[x][y - 1] = true;
                    }
                    if y < 8 && !grouped_squares[x][y + 1] && board.biomes[x][y] == board.biomes[x][y + 1] {
                        que.push([x, y + 1]);
                        grouped_squares[x][y + 1] = true;
                    }
                    que.remove(0);
                }
                groups.push(group);
            }
        }
    }
    return groups.iter().map(|group|group.score()).sum();
}

/// Gets the number of players.
/// Then gets available dominos from user and plays the best available option until the end of the game.
fn main() {
    println!("Enter the number of players:");
    let number_of_players: usize = get_int_input(2, 4);
    let mut turn: u8 = 0;
    let mut board: Board = Board::new();
    while turn < 12 {
        let mut available_dominos: Vec<usize> = get_available_dominos(number_of_players);
        available_dominos.sort_by(|a, b|b.cmp(a));
        let possible_moves: Vec<(Board, usize)> = possible_domino_placements(&board, &available_dominos);
        let best_move: Option<&(Board, usize)> = possible_moves.iter().max_by_key(|x|evaluate_board(&x.0));
        match best_move {
            Some((new_board, choise)) => {
                board = new_board.clone();
                println!("Selected domino number {}.", choise + 1);
            },
            None => {
                let mut choise = 0;
                let mut number_of_crowns = 0;
                for domino in available_dominos {
                    if dominos::DOMINOS[domino][0].crowns + dominos::DOMINOS[domino][1].crowns >= number_of_crowns {
                        choise = domino;
                        number_of_crowns = dominos::DOMINOS[domino][0].crowns + dominos::DOMINOS[domino][1].crowns;
                    }
                }
                println!("Selected domino number {}.", choise + 1);
                println!("No avaiable placement.");    
            },
        }
        display_board(&board);
        println!("Score: {}", evaluate_board(&board));
        turn += 1;
    }
}