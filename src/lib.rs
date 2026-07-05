// Tic-Tac-Toe in Rust, compiled to WebAssembly.
//
// The entire game — board state, win detection, and an unbeatable
// minimax AI — lives here in Rust. The browser only handles clicks and
// drawing, calling into this compiled WASM module for every decision.

use wasm_bindgen::prelude::*;

const EMPTY: u8 = 0;
const X: u8 = 1; // human player
const O: u8 = 2; // computer player

const LINES: [[usize; 3]; 8] = [
    [0, 1, 2], [3, 4, 5], [6, 7, 8], // rows
    [0, 3, 6], [1, 4, 7], [2, 5, 8], // columns
    [0, 4, 8], [2, 4, 6],            // diagonals
];

/// Returns the winning player (X or O) if there is one, otherwise EMPTY.
fn winner(board: &[u8; 9]) -> u8 {
    for line in LINES.iter() {
        let a = board[line[0]];
        if a != EMPTY && a == board[line[1]] && a == board[line[2]] {
            return a;
        }
    }
    EMPTY
}

fn is_full(board: &[u8; 9]) -> bool {
    board.iter().all(|&c| c != EMPTY)
}

/// Minimax: O (the computer) maximizes, X (the human) minimizes.
/// `depth` makes the AI prefer quicker wins and slower losses.
fn minimax(board: &mut [u8; 9], o_to_move: bool, depth: i32) -> i32 {
    match winner(board) {
        O => return 10 - depth,
        X => return depth - 10,
        _ => {}
    }
    if is_full(board) {
        return 0;
    }

    if o_to_move {
        let mut best = i32::MIN;
        for i in 0..9 {
            if board[i] == EMPTY {
                board[i] = O;
                best = best.max(minimax(board, false, depth + 1));
                board[i] = EMPTY;
            }
        }
        best
    } else {
        let mut best = i32::MAX;
        for i in 0..9 {
            if board[i] == EMPTY {
                board[i] = X;
                best = best.min(minimax(board, true, depth + 1));
                board[i] = EMPTY;
            }
        }
        best
    }
}

/// The best move for O (computer): the empty cell with the highest minimax score.
fn best_move(board: &[u8; 9]) -> Option<usize> {
    let mut b = *board;
    let mut best_score = i32::MIN;
    let mut best_idx = None;
    for i in 0..9 {
        if b[i] == EMPTY {
            b[i] = O;
            let score = minimax(&mut b, false, 1);
            b[i] = EMPTY;
            if score > best_score {
                best_score = score;
                best_idx = Some(i);
            }
        }
    }
    best_idx
}

/// The game, exposed to JavaScript.
#[wasm_bindgen]
pub struct Game {
    board: [u8; 9],
    over: bool,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        Game { board: [EMPTY; 9], over: false }
    }

    /// Human plays X at `idx`. If the move is legal, the computer replies.
    /// Returns true if the human's move was accepted.
    pub fn human_move(&mut self, idx: usize) -> bool {
        if self.over || idx >= 9 || self.board[idx] != EMPTY {
            return false;
        }
        self.board[idx] = X;

        if winner(&self.board) != EMPTY || is_full(&self.board) {
            self.over = true;
            return true;
        }

        // Computer responds.
        if let Some(mv) = best_move(&self.board) {
            self.board[mv] = O;
        }
        if winner(&self.board) != EMPTY || is_full(&self.board) {
            self.over = true;
        }
        true
    }

    pub fn reset(&mut self) {
        self.board = [EMPTY; 9];
        self.over = false;
    }

    /// Cell contents at `idx`: 0 = empty, 1 = X (you), 2 = O (computer).
    pub fn cell(&self, idx: usize) -> u8 {
        if idx < 9 { self.board[idx] } else { EMPTY }
    }

    /// Current status: "playing", "x" (you win), "o" (computer wins), or "draw".
    pub fn status(&self) -> String {
        match winner(&self.board) {
            X => "x".into(),
            O => "o".into(),
            _ if is_full(&self.board) => "draw".into(),
            _ => "playing".into(),
        }
    }
}

// ---- Native tests (run with `cargo test`; not included in the WASM build) ----
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_wins() {
        assert_eq!(winner(&[X, X, X, 0, 0, 0, 0, 0, 0]), X);
        assert_eq!(winner(&[O, 0, 0, O, 0, 0, O, 0, 0]), O);
        assert_eq!(winner(&[X, 0, 0, 0, X, 0, 0, 0, X]), X);
        assert_eq!(winner(&[0; 9]), EMPTY);
    }

    #[test]
    fn ai_takes_win_and_blocks() {
        assert_eq!(best_move(&[O, O, EMPTY, X, X, EMPTY, EMPTY, EMPTY, EMPTY]), Some(2));
        assert_eq!(best_move(&[X, X, EMPTY, EMPTY, O, EMPTY, EMPTY, EMPTY, EMPTY]), Some(2));
    }

    // Exhaustively verify the human can never win against the AI.
    fn human_can_win(board: [u8; 9], human_turn: bool) -> bool {
        match winner(&board) {
            X => return true,
            O => return false,
            _ => {}
        }
        if is_full(&board) {
            return false;
        }
        if human_turn {
            (0..9).any(|i| {
                board[i] == EMPTY && {
                    let mut nb = board;
                    nb[i] = X;
                    human_can_win(nb, false)
                }
            })
        } else {
            let mut nb = board;
            if let Some(mv) = best_move(&board) {
                nb[mv] = O;
            }
            human_can_win(nb, true)
        }
    }

    #[test]
    fn ai_is_unbeatable() {
        assert!(!human_can_win([EMPTY; 9], true));  // human first
        assert!(!human_can_win([EMPTY; 9], false)); // computer first
    }
}
