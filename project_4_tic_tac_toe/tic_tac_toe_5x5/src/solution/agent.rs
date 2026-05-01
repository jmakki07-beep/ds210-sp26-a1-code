use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player;

pub struct SolutionAgent {}

// Gives more value to pieces closer to the center of the board.
fn center_control_score(board: &Board) -> i32 {
    let cells = board.get_cells();
    let rows = cells.len();
    let cols = cells[0].len();
    let center_r = rows / 2;
    let center_c = cols / 2;
    let mut score = 0;

    for r in 0..rows {
        for c in 0..cols {
            let distance =
                ((r as i32 - center_r as i32).abs() + (c as i32 - center_c as i32).abs()) as i32;
            let value = (rows as i32 + cols as i32) - distance;

            match cells[r][c] {
                Cell::X => score += value,
                Cell::O => score -= value,
                _ => {}
            }
        }
    }

    score
}

// Rewards three pieces in a row.
fn three_in_row_score(board: &Board) -> i32 {
    let cells = board.get_cells();
    let rows = cells.len();
    let cols = cells[0].len();
    let mut score = 0;

    for r in 0..rows {
        for c in 0..cols {
            // Horizontal
            if c + 2 < cols {
                match (&cells[r][c], &cells[r][c + 1], &cells[r][c + 2]) {
                    (Cell::X, Cell::X, Cell::X) => score += 20,
                    (Cell::O, Cell::O, Cell::O) => score -= 20,
                    _ => {}
                }
            }

            // Vertical
            if r + 2 < rows {
                match (&cells[r][c], &cells[r + 1][c], &cells[r + 2][c]) {
                    (Cell::X, Cell::X, Cell::X) => score += 20,
                    (Cell::O, Cell::O, Cell::O) => score -= 20,
                    _ => {}
                }
            }

            // Diagonal down-right
            if r + 2 < rows && c + 2 < cols {
                match (&cells[r][c], &cells[r + 1][c + 1], &cells[r + 2][c + 2]) {
                    (Cell::X, Cell::X, Cell::X) => score += 20,
                    (Cell::O, Cell::O, Cell::O) => score -= 20,
                    _ => {}
                }
            }

            // Diagonal down-left
            if r + 2 < rows && c >= 2 {
                match (&cells[r][c], &cells[r + 1][c - 1], &cells[r + 2][c - 2]) {
                    (Cell::X, Cell::X, Cell::X) => score += 20,
                    (Cell::O, Cell::O, Cell::O) => score -= 20,
                    _ => {}
                }
            }
        }
    }

    score
}

// Rewards threats and punishes opponent threats.
fn block_opponent_score(board: &Board) -> i32 {
    let cells = board.get_cells();
    let rows = cells.len();
    let cols = cells[0].len();
    let mut score = 0;

    for r in 0..rows {
        for c in 0..cols {
            // Horizontal threat
            if c + 2 < cols {
                match (&cells[r][c], &cells[r][c + 1], &cells[r][c + 2]) {
                    (Cell::Empty, Cell::X, Cell::X) => score += 8,
                    (Cell::X, Cell::X, Cell::Empty) => score += 8,
                    (Cell::X, Cell::Empty, Cell::X) => score += 8,
                    (Cell::Empty, Cell::O, Cell::O) => score -= 8,
                    (Cell::O, Cell::O, Cell::Empty) => score -= 8,
                    (Cell::O, Cell::Empty, Cell::O) => score -= 8,
                    _ => {}
                }
            }

            // Vertical threat
            if r + 2 < rows {
                match (&cells[r][c], &cells[r + 1][c], &cells[r + 2][c]) {
                    (Cell::Empty, Cell::X, Cell::X) => score += 8,
                    (Cell::X, Cell::X, Cell::Empty) => score += 8,
                    (Cell::X, Cell::Empty, Cell::X) => score += 8,
                    (Cell::Empty, Cell::O, Cell::O) => score -= 8,
                    (Cell::O, Cell::O, Cell::Empty) => score -= 8,
                    (Cell::O, Cell::Empty, Cell::O) => score -= 8,
                    _ => {}
                }
            }
        }
    }

    score
}

// Rewards four pieces in a row.
fn four_in_row_score(board: &Board) -> i32 {
    let cells = board.get_cells();
    let rows = cells.len();
    let cols = cells[0].len();
    let mut score = 0;

    for r in 0..rows {
        for c in 0..cols {
            // Horizontal
            if c + 3 < cols {
                match (&cells[r][c], &cells[r][c + 1], &cells[r][c + 2], &cells[r][c + 3]) {
                    (Cell::X, Cell::X, Cell::X, Cell::X) => score += 50,
                    (Cell::O, Cell::O, Cell::O, Cell::O) => score -= 50,
                    _ => {}
                }
            }

            // Vertical
            if r + 3 < rows {
                match (&cells[r][c], &cells[r + 1][c], &cells[r + 2][c], &cells[r + 3][c]) {
                    (Cell::X, Cell::X, Cell::X, Cell::X) => score += 50,
                    (Cell::O, Cell::O, Cell::O, Cell::O) => score -= 50,
                    _ => {}
                }
            }

            // Diagonal down-right
            if r + 3 < rows && c + 3 < cols {
                match (&cells[r][c], &cells[r + 1][c + 1], &cells[r + 2][c + 2], &cells[r + 3][c + 3]) {
                    (Cell::X, Cell::X, Cell::X, Cell::X) => score += 50,
                    (Cell::O, Cell::O, Cell::O, Cell::O) => score -= 50,
                    _ => {}
                }
            }

            // Diagonal down-left
            if r + 3 < rows && c >= 3 {
                match (&cells[r][c], &cells[r + 1][c - 1], &cells[r + 2][c - 2], &cells[r + 3][c - 3]) {
                    (Cell::X, Cell::X, Cell::X, Cell::X) => score += 50,
                    (Cell::O, Cell::O, Cell::O, Cell::O) => score -= 50,
                    _ => {}
                }
            }
        }
    }

    score
}

// Combines the real board score with helper scores.
fn heuristic(board: &Board) -> i32 {
    let base = board.score() * 100;
    let center = center_control_score(board);
    let threes = three_in_row_score(board);
    let block = block_opponent_score(board);
    let fours = four_in_row_score(board);

    base + center + threes + block + fours
}

fn minimax(
    board: &mut Board,
    player: Player,
    current_depth: i32,
    max_depth: i32,
) -> (i32, usize, usize) {
    if board.game_over() {
        return (board.score() * 100, 0, 0);
    }

    let avbmoves = board.moves();

    if avbmoves.is_empty() {
        return (heuristic(board), 0, 0);
    }

    if current_depth == max_depth {
        return (heuristic(board), avbmoves[0].0, avbmoves[0].1);
    }

    let mut best_score = match player {
        Player::X => i32::MIN,
        Player::O => i32::MAX,
    };

    let mut best_x = avbmoves[0].0;
    let mut best_y = avbmoves[0].1;

    let opponent = match player {
        Player::X => Player::O,
        Player::O => Player::X,
    };

    for mv in avbmoves {
        board.apply_move(mv, player);
        let (score, _, _) = minimax(board, opponent, current_depth + 1, max_depth);
        board.undo_move(mv, player);

        match player {
            Player::X => {
                if score > best_score {
                    best_score = score;
                    best_x = mv.0;
                    best_y = mv.1;
                }
            }
            Player::O => {
                if score < best_score {
                    best_score = score;
                    best_x = mv.0;
                    best_y = mv.1;
                }
            }
        }
    }

    (best_score, best_x, best_y)
}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let avbmoves = board.moves();

        if avbmoves.is_empty() {
            return (board.score(), 0, 0);
        }

        let remaining_moves = avbmoves.len();

        // Search deeper when fewer moves are left.
        let max_depth = if remaining_moves <= 5 {
            remaining_moves as i32
        } else if remaining_moves <= 12 {
            4
        } else if remaining_moves <= 16 {
            3
        } else if remaining_moves <= 25 {
            2
        } else {
            4
        };

        minimax(board, player, 0, max_depth)
    }
}