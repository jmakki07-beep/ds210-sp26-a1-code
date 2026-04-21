use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player;

// New improved solution 
pub struct SolutionAgent {}

impl SolutionAgent {
    fn heuristic(board: &Board) -> i32 {
        let mut score = board.score() * 20;
        let cells = board.get_cells();

        for i in 0..cells.len() {
            for j in 0..cells.len() {
                // Row check
                if j + 2 < cells.len() {
                    let a = &cells[i][j];
                    let b = &cells[i][j + 1];
                    let c = &cells[i][j + 2];
                    match (a, b, c) {
                        (Cell::X, Cell::X, Cell::Empty) => score += 1,
                        (Cell::Empty, Cell::X, Cell::X) => score += 1,
                        (Cell::X, Cell::Empty, Cell::X) => score += 1,
                        (Cell::O, Cell::O, Cell::Empty) => score -= 1,
                        (Cell::Empty, Cell::O, Cell::O) => score -= 1,
                        (Cell::O, Cell::Empty, Cell::O) => score -= 1,
                        _ => {}
                    }
                }
                // Column check
                if i + 2 < cells.len() {
                    let a = &cells[i][j];
                    let b = &cells[i + 1][j];
                    let c = &cells[i + 2][j];
                    match (a, b, c) {
                        (Cell::X, Cell::X, Cell::Empty) => score += 1,
                        (Cell::Empty, Cell::X, Cell::X) => score += 1,
                        (Cell::X, Cell::Empty, Cell::X) => score += 1,
                        (Cell::O, Cell::O, Cell::Empty) => score -= 1,
                        (Cell::Empty, Cell::O, Cell::O) => score -= 1,
                        (Cell::O, Cell::Empty, Cell::O) => score -= 1,
                        _ => {}
                    }
                }
                // Diagonal 1
                if i + 2 < cells.len() && j + 2 < cells.len() {
                    let a = &cells[i][j];
                    let b = &cells[i + 1][j + 1];
                    let c = &cells[i + 2][j + 2];
                    match (a, b, c) {
                        (Cell::X, Cell::X, Cell::Empty) => score += 1,
                        (Cell::Empty, Cell::X, Cell::X) => score += 1,
                        (Cell::X, Cell::Empty, Cell::X) => score += 1,
                        (Cell::O, Cell::O, Cell::Empty) => score -= 1,
                        (Cell::Empty, Cell::O, Cell::O) => score -= 1,
                        (Cell::O, Cell::Empty, Cell::O) => score -= 1,
                        _ => {}
                    }
                }
                // Diagonal 2
                if i + 2 < cells.len() && j >= 2 {
                    let a = &cells[i][j];
                    let b = &cells[i + 1][j - 1];
                    let c = &cells[i + 2][j - 2];
                    match (a, b, c) {
                        (Cell::X, Cell::X, Cell::Empty) => score += 1,
                        (Cell::Empty, Cell::X, Cell::X) => score += 1,
                        (Cell::X, Cell::Empty, Cell::X) => score += 1,
                        (Cell::O, Cell::O, Cell::Empty) => score -= 1,
                        (Cell::Empty, Cell::O, Cell::O) => score -= 1,
                        (Cell::O, Cell::Empty, Cell::O) => score -= 1,
                        _ => {}
                    }
                }
            }
        }
        score
    }

    fn minimax(board: &mut Board, player: Player, depth: u32, max_depth: u32) -> (i32, usize, usize) {
        if board.game_over() || depth == max_depth {
            return (SolutionAgent::heuristic(board), 0, 0);
        }

        let avbmoves = board.moves();
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

        for i in avbmoves {
            board.apply_move(i, player);
            let (score, _, _) = SolutionAgent::minimax(board, opponent, depth + 1, max_depth);
            board.undo_move(i, player);

            match player {
                Player::X => if score > best_score {
                    best_score = score;
                    best_x = i.0;
                    best_y = i.1;
                },
                Player::O => if score < best_score {
                    best_score = score;
                    best_x = i.0;
                    best_y = i.1;
                },
            }
        }

        (best_score, best_x, best_y)
    }
}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let cells = board.get_cells();
        let max_depth = if cells.len() <= 3 { board.moves().len() as u32 } else { 2 };
        SolutionAgent::minimax(board, player, 0, max_depth)
    }
}