use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

impl SolutionAgent {
    fn minimax(board: &mut Board, player: Player, depth: u32, max_depth: u32) -> (i32, usize, usize) {
        if board.game_over() || depth == max_depth {
            return (board.score(), 0, 0);
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
    let moves = board.moves();
    let max_depth = if moves.len() <= 9 { moves.len() as u32 } else { 3 };
    SolutionAgent::minimax(board, player, 0, max_depth)
}
}