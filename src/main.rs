mod constants;

use crate::constants::*;
use chess::{Board, BoardStatus, CacheTable, ChessMove, Color, MoveGen, Piece};
use std::cmp::{max, min};
use std::io::stdin;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct SearchResult {
    value: i32,
    best_move: Option<ChessMove>,
}

#[inline]
fn get_attack_weight(board: &Board) -> usize {
    let mut current_moves = MoveGen::new_legal(board);
    let mut attack_weight: usize = 0;
    current_moves.set_iterator_mask(match board.side_to_move() {
        Color::White => {
            WHITE_KING_DANGER_SQUARE_MAP[(board.pieces(Piece::King)
                & board.color_combined(Color::Black))
            .to_square()
            .to_index()]
        }
        Color::Black => {
            BLACK_KING_DANGER_SQUARE_MAP[(board.pieces(Piece::King)
                & board.color_combined(Color::White))
            .to_square()
            .to_index()]
        }
    });
    for current_move in current_moves {
        attack_weight += PIECE_VALUES[board
            .piece_on(current_move.get_source())
            .unwrap()
            .to_index()]
        .attack_weight as usize;
    }
    attack_weight
}

fn assess_board(board: &Board) -> i32 {
    let mut val: i32 = 0;
    for piece_val_pair in PIECE_VALUES {
        let piece_bits = board.pieces(piece_val_pair.piece);
        let white_pieces = board.color_combined(Color::White) & piece_bits;
        let black_pieces = board.color_combined(Color::Black) & piece_bits;
        for (rank, rank_bits) in RANK_BITBOARDS.iter().enumerate() {
            let num_pieces = (white_pieces & rank_bits).popcnt() as i32;
            val += (piece_val_pair.value + piece_val_pair.forward_scale * rank as i32) * num_pieces;
        }
        for (rank, rank_bits) in RANK_BITBOARDS.iter().enumerate() {
            let num_pieces = (black_pieces & rank_bits).popcnt() as i32;
            val -= (piece_val_pair.value + piece_val_pair.forward_scale * (7 - rank) as i32)
                * num_pieces;
        }
    }
    let side_scalar = match board.side_to_move() {
        Color::White => SIDE_SCALAR,
        Color::Black => -SIDE_SCALAR,
    };
    val += side_scalar
        * (MoveGen::new_legal(board).len() as i32 + ATTACK_WEIGHT_MAP[get_attack_weight(board)]);
    if let Some(flipped) = board.null_move() {
        val -= side_scalar
            * (MoveGen::new_legal(&flipped).len() as i32
                + ATTACK_WEIGHT_MAP[get_attack_weight(&flipped)])
    }
    val
}

/// # Safety
/// raw_fen_ptr must point to a valid null terminated string
fn start_search(fen: &str, depth: u16) -> SearchResult {
    let mut memo_table: CacheTable<SearchResult> = CacheTable::new(
        2 << 26,
        SearchResult {
            best_move: None,
            value: 0,
        },
    );
    search(
        Board::from_str(fen).unwrap(),
        depth,
        depth + MAX_DEPTH_INCREASE,
        i32::MIN,
        i32::MAX,
        &mut memo_table,
    )
}

fn search(
    board: Board,
    depth: u16,
    depth_limit: u16,
    mut alpha: i32,
    mut beta: i32,
    memo_table: &mut CacheTable<SearchResult>,
) -> SearchResult {
    match board.status() {
        BoardStatus::Ongoing => {}
        BoardStatus::Stalemate => {
            return SearchResult {
                best_move: None,
                value: 0,
            }
        }
        BoardStatus::Checkmate => {
            return match board.side_to_move() {
                Color::White => SearchResult {
                    best_move: None,
                    value: i32::MIN,
                },
                Color::Black => SearchResult {
                    best_move: None,
                    value: i32::MAX,
                },
            }
        }
    }
    if depth == 0 || depth_limit == 0 {
        return SearchResult {
            best_move: None,
            value: assess_board(&board),
        };
    }
    if let Some(cached_result) = memo_table.get(board.get_hash()) {
        return cached_result;
    }
    let mut result = SearchResult {
        best_move: None,
        value: 0,
    };
    match board.side_to_move() {
        Color::White => result.value = i32::MIN,
        Color::Black => result.value = i32::MAX,
    }
    let masks = [
        board.color_combined(!board.side_to_move()) & !board.pieces(Piece::Pawn),
        !chess::EMPTY,
    ];
    let mut moves = MoveGen::new_legal(&board);
    for (processed, mask) in masks.into_iter().enumerate() {
        moves.set_iterator_mask(mask);
        for mov in &mut moves {
            let new_board = board.make_move_new(mov);
            let new_depth = if (processed < masks.len() - 1
                && board.piece_on(mov.get_source()).unwrap_or(Piece::King) == Piece::Pawn)
                || new_board.checkers().0 > 0
            {
                depth
            } else {
                depth - 1
            };
            let check = search(
                new_board,
                new_depth,
                depth_limit - 1,
                alpha,
                beta,
                memo_table,
            );
            match board.side_to_move() {
                Color::White => {
                    alpha = max(alpha, check.value);
                    if check.value > result.value || result.best_move.is_none() {
                        result = check;
                        result.best_move = Some(mov);
                    }
                    if result.value >= beta {
                        break;
                    }
                }
                Color::Black => {
                    beta = min(beta, check.value);
                    if check.value < result.value || result.best_move.is_none() {
                        result = check;
                        result.best_move = Some(mov);
                    }
                    if result.value <= alpha {
                        break;
                    }
                }
            }
        }
    }
    memo_table.add(board.get_hash(), result);
    result
}

fn main() {
    let mut line_in = String::new();
    let mut fen: String = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
    let mut depth: u16 = 7;
    loop {
        match stdin().read_line(&mut line_in) {
            Ok(_n) => {
                if line_in.starts_with("fen") {
                    fen = line_in[4..].to_string();
                }
                if line_in.starts_with("depth") {
                    match line_in[6..].trim().parse::<u16>() {
                        Ok(val) => depth = val,
                        Err(error) => println!("DEPTH ERROR: {} | {}", line_in[6..].trim(), error),
                    }
                }
                if line_in.starts_with("eval") {
                    match start_search(fen.as_str(), depth).best_move {
                        Some(good_move) => println!("{}", good_move),
                        None => println!("0000"),
                    }
                }
                if line_in.starts_with("quit") {
                    return;
                }
            }
            Err(error) => println!("ERROR: {}", error),
        }
        line_in.clear();
    }
}
