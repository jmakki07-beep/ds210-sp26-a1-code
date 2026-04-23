use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player;
 
pub struct SolutionAgent {}
 
// 中心控制：越靠近中心的位置价值越高
fn center_control_score(board: &Board) -> i32 {
    let cells = board.get_cells();
    let rows = cells.len();
    let cols = cells[0].len();
    let center_r = rows / 2;
    let center_c = cols / 2;
    let mut score = 0;
 
    for r in 0..rows {
        for c in 0..cols {
            let distance = ((r as i32 - center_r as i32).abs() + (c as i32 - center_c as i32).abs()) as i32;
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
 
// detection for 3 in a row
fn three_in_row_score(board: &Board) -> i32 {
    let cells = board.get_cells();
    let rows = cells.len();
    let cols = cells[0].len();
    let mut score = 0;
 
    for r in 0..rows {
        for c in 0..cols {
            // 水平三连
            if c + 2 < cols {
                match (&cells[r][c], &cells[r][c+1], &cells[r][c+2]) {
                    (Cell::X, Cell::X, Cell::X) => score += 20,
                    (Cell::O, Cell::O, Cell::O) => score -= 20,
                    _ => {}
                }
            }
            // 垂直三连
            if r + 2 < rows {
                match (&cells[r][c], &cells[r+1][c], &cells[r+2][c]) {
                    (Cell::X, Cell::X, Cell::X) => score += 20,
                    (Cell::O, Cell::O, Cell::O) => score -= 20,
                    _ => {}
                }
            }
            // 对角线三连（\方向）
            if r + 2 < rows && c + 2 < cols {
                match (&cells[r][c], &cells[r+1][c+1], &cells[r+2][c+2]) {
                    (Cell::X, Cell::X, Cell::X) => score += 20,
                    (Cell::O, Cell::O, Cell::O) => score -= 20,
                    _ => {}
                }
            }
            // 对角线三连（/方向）
            if r + 2 < rows && c >= 2 {
                match (&cells[r][c], &cells[r+1][c-1], &cells[r+2][c-2]) {
                    (Cell::X, Cell::X, Cell::X) => score += 20,
                    (Cell::O, Cell::O, Cell::O) => score -= 20,
                    _ => {}
                }
            }
        }
    }
    score
}
 
// 3. 阻止对手连子
fn block_opponent_score(board: &Board) -> i32 {
    let cells = board.get_cells();
    let rows = cells.len();
    let cols = cells[0].len();
    let mut score = 0;
 
    for r in 0..rows {
        for c in 0..cols {
            // 水平方向
            if c + 2 < cols {
                match (&cells[r][c], &cells[r][c+1], &cells[r][c+2]) {
                    // X 有威胁 → 对 X 有利（正分）
                    (Cell::Empty, Cell::X, Cell::X) => score += 8,
                    (Cell::X, Cell::X, Cell::Empty) => score += 8,
                    // O 有威胁 → 对 X 不利（负分）
                    (Cell::Empty, Cell::O, Cell::O) => score -= 8,
                    (Cell::O, Cell::O, Cell::Empty) => score -= 8,
                    _ => {}
                }
            }
            // 垂直方向
            if r + 2 < rows {
                match (&cells[r][c], &cells[r+1][c], &cells[r+2][c]) {
                    (Cell::Empty, Cell::X, Cell::X) => score += 8,
                    (Cell::X, Cell::X, Cell::Empty) => score += 8,
                    (Cell::Empty, Cell::O, Cell::O) => score -= 8,
                    (Cell::O, Cell::O, Cell::Empty) => score -= 8,
                    _ => {}
                }
            }
        }
    }
    score
}
 
// 4. 角落和对角线价值
fn corner_diagonal_score(board: &Board) -> i32 {
    let cells = board.get_cells();
    let rows = cells.len();
    let cols = cells[0].len();
    let mut score = 0;
 
    // 四个角落
    let corners = [(0, 0), (0, cols-1), (rows-1, 0), (rows-1, cols-1)];
    for (r, c) in corners {
        match cells[r][c] {
            Cell::X => score += 2,
            Cell::O => score -= 2,
            _ => {}
        }
    }
 
    // 主对角线
    for i in 0..rows.min(cols) {
        match cells[i][i] {
            Cell::X => score += 2,
            Cell::O => score -= 2,
            _ => {}
        }
    }
 
    // 副对角线
    for i in 0..rows.min(cols) {
        if cols > i {
            match cells[i][cols - 1 - i] {
                Cell::X => score += 2,
                Cell::O => score -= 2,
                _ => {}
            }
        }
    }
    score
}
 
//  墙壁惩罚
fn wall_penalty_score(board: &Board) -> i32 {
    let cells = board.get_cells();
    let rows = cells.len();
    let cols = cells[0].len();
    let mut score = 0;
 
    for r in 0..rows {
        for c in 0..cols {
            let my_cell = &cells[r][c];
            if !matches!(my_cell, Cell::X | Cell::O) {
                continue;
            }
 
            let mut wall_count = 0;
 
            if c > 0 && matches!(cells[r][c-1], Cell::Wall) {
                wall_count += 1;
            }
            if c + 1 < cols && matches!(cells[r][c+1], Cell::Wall) {
                wall_count += 1;
            }
            if r > 0 && matches!(cells[r-1][c], Cell::Wall) {
                wall_count += 1;
            }
            if r + 1 < rows && matches!(cells[r+1][c], Cell::Wall) {
                wall_count += 1;
            }
 
            match my_cell {
                Cell::X => score -= wall_count * 2,
                Cell::O => score += wall_count * 2,
                _ => {}
            }
        }
    }
    score
}
 
// 6. 四连奖励
fn four_in_row_score(board: &Board) -> i32 {
    let cells = board.get_cells();
    let rows = cells.len();
    let cols = cells[0].len();
    let mut score = 0;
 
    for r in 0..rows {
        for c in 0..cols {
            // 水平四连
            if c + 3 < cols {
                match (&cells[r][c], &cells[r][c+1], &cells[r][c+2], &cells[r][c+3]) {
                    (Cell::X, Cell::X, Cell::X, Cell::X) => score += 50,
                    (Cell::O, Cell::O, Cell::O, Cell::O) => score -= 50,
                    _ => {}
                }
            }
            // 垂直四连
            if r + 3 < rows {
                match (&cells[r][c], &cells[r+1][c], &cells[r+2][c], &cells[r+3][c]) {
                    (Cell::X, Cell::X, Cell::X, Cell::X) => score += 50,
                    (Cell::O, Cell::O, Cell::O, Cell::O) => score -= 50,
                    _ => {}
                }
            }
            // 对角线四连 \
            if r + 3 < rows && c + 3 < cols {
                match (&cells[r][c], &cells[r+1][c+1], &cells[r+2][c+2], &cells[r+3][c+3]) {
                    (Cell::X, Cell::X, Cell::X, Cell::X) => score += 50,
                    (Cell::O, Cell::O, Cell::O, Cell::O) => score -= 50,
                    _ => {}
                }
            }
            // 对角线四连 /
            if r + 3 < rows && c >= 3 {
                match (&cells[r][c], &cells[r+1][c-1], &cells[r+2][c-2], &cells[r+3][c-3]) {
                    (Cell::X, Cell::X, Cell::X, Cell::X) => score += 50,
                    (Cell::O, Cell::O, Cell::O, Cell::O) => score -= 50,
                    _ => {}
                }
            }
        }
    }
    score
}
 

fn heuristic(board: &Board) -> i32 {
    let base = board.score() * 100;           // 当前得分权重最高
    let center = center_control_score(board);  // 中心控制
    let threes = three_in_row_score(board);    // 三连子
    let block = block_opponent_score(board);   // 阻挡对手
    let corner = corner_diagonal_score(board); // 对角线
    let wall = wall_penalty_score(board);      // 墙壁惩罚
    let fours = four_in_row_score(board);      // 四连奖励
 
    base + center + threes + block + corner + wall + fours
}
 
fn minimax(board: &mut Board, player: Player, current_depth: i32, max_depth: i32) -> (i32, usize, usize) {
 
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
 
    for i in avbmoves {
        board.apply_move(i, player);
        let (score, _, _) = minimax(board, opponent, current_depth + 1, max_depth);
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
 
impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let avbmoves = board.moves();
 
        if avbmoves.is_empty() {
            return (board.score(), 0, 0);
        }
 
        let remaining_moves = avbmoves.len();
 
        // 分层深度选择：随着可走步数减少，搜索更深
        // 这样可以平滑过渡，而不是像以前那样在 9 步的阈值突然跳变
        let max_depth = if remaining_moves <= 5 {
            remaining_moves as i32   // 剩余步数少，直接全搜索
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
