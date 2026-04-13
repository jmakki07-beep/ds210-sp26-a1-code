use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        // Step 1: base case
        if board.game_over() {
            return (board.score(), 0, 0);
        }

        // Step 2: get available moves
        let avbmoves = board.moves();

        // Track best score and move
        let mut best_score = match player {
            Player::X => i32::MIN,
            Player::O => i32::MAX,
        };
        let mut best_x = avbmoves[0].0;
        let mut best_y = avbmoves[0].1;

        // Opponent
        let opponent = match player {
            Player::X => Player::O,
            Player::O => Player::X,
        };

        // Step 3: loop over moves
        for i in avbmoves {
            board.apply_move(i, player);
            let (score, _, _) = SolutionAgent::solve(board, opponent, _time_limit);
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

        return (best_score, best_x, best_y);
    }
}