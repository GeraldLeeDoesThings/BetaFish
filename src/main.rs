#![feature(binary_heap_into_iter_sorted)]

mod constants;
mod eval;

use crate::constants::*;
use crate::eval::*;
use crate::NodeType::{All, Cut, PV};
use chess::{BitBoard, Board, BoardStatus, CacheTable, ChessMove, Color, MoveGen, Piece};
use std::cmp::{max, min};
use std::io::stdin;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum NodeType {
    PV,
    Cut,
    All,
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct SearchResult {
    value: i32,
    lazy_value: i32,
    best_move: Option<ChessMove>,
    depth: u16,
    node_type: NodeType,
}

impl SearchResult {
    fn new(value: i32, lazy_value: i32, best_move: Option<ChessMove>, depth: u16) -> SearchResult {
        SearchResult {
            value,
            lazy_value,
            best_move,
            depth,
            node_type: PV,
        }
    }
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

fn lazy_assess_board(board: &Board) -> i32 {
    let mut val: i32 = 0;
    let side_scalar = PLAYER_SCALAR_MAP[board.side_to_move().to_index()];
    val += side_scalar * (eval_mobility(board) + ATTACK_WEIGHT_MAP[get_attack_weight(board)]);
    if let Some(flipped) = board.null_move() {
        val -=
            side_scalar * (eval_mobility(&flipped) + ATTACK_WEIGHT_MAP[get_attack_weight(&flipped)])
    }
    val += eval_overall_pawn_bonus(board);
    val
}

fn assess_incremental(board: &Board, chess_move: ChessMove) -> i32 {
    let mut val: i32 = 0;
    let moving_piece = board.piece_on(chess_move.get_source()).unwrap();
    let moving_val = &PIECE_VALUES[moving_piece.to_index()];
    let result_piece = chess_move.get_promotion().unwrap_or(moving_piece);
    let result_val = &PIECE_VALUES[result_piece.to_index()];
    let captured_piece = board.piece_on(chess_move.get_dest());
    let side_scalar = PLAYER_SCALAR_MAP[board.side_to_move().to_index()];
    let current_player = board.side_to_move();
    // Eval diff for moving forward (accounts for promotion)
    val += eval_piece_position(result_piece, chess_move.get_dest(), current_player)
        - eval_piece_position(moving_piece, chess_move.get_source(), current_player);
    // Eval pure value diff for promoting
    val += side_scalar * (result_val.value - moving_val.value);
    // Eval diff for captures
    if let Some(captured) = captured_piece {
        val -= eval_piece(captured, chess_move.get_dest(), !board.side_to_move());
    } else if let Some(en_passant_square) = board.en_passant() {
        if moving_piece == Piece::Pawn
            && chess_move.get_source().get_file() != chess_move.get_dest().get_file()
        {
            val -= eval_piece(Piece::Pawn, en_passant_square, !board.side_to_move());
        }
    }
    val
}

/// # Safety
/// raw_fen_ptr must point to a valid null terminated string
fn start_search(fen: &str, depth: u16, memo_table: &mut CacheTable<SearchResult>) -> SearchResult {
    let board = Board::from_str(fen).unwrap();
    search(
        board,
        MAX_DEPTH_INCREASE,
        0,
        depth + MAX_DEPTH_INCREASE,
        eval_all_pieces_positional(&board),
        i32::MIN,
        i32::MAX,
        memo_table,
    )
}

#[inline]
fn advantaged_capture(chess_move: &ChessMove, board: &Board) -> bool {
    let attacker = board.piece_on(chess_move.get_source()).unwrap();
    if let Some(defender) = board.piece_on(chess_move.get_dest()) {
        PIECE_VALUES[attacker.to_index()].value <= PIECE_VALUES[defender.to_index()].value
    } else {
        false
    }
}

// TODO: reduce number of args by packaging
#[allow(clippy::too_many_arguments)]
fn search(
    board: Board,
    logical_depth: u16,
    true_depth: u16,
    depth_limit: u16,
    lazy_eval: i32,
    mut alpha: i32,
    mut beta: i32,
    memo_table: &mut CacheTable<SearchResult>,
) -> SearchResult {
    match board.status() {
        BoardStatus::Ongoing => {}
        BoardStatus::Stalemate => {
            return SearchResult::new(0, 0, None, 0);
        }
        BoardStatus::Checkmate => {
            return match board.side_to_move() {
                Color::White => SearchResult::new(i32::MIN, i32::MIN, None, 0),
                Color::Black => SearchResult::new(i32::MAX, i32::MAX, None, 0),
            }
        }
    }
    let cached_result = memo_table.get(board.get_hash());
    if let Some(result) = cached_result {
        if result.depth <= true_depth {
            match result.node_type {
                PV => return result,
                Cut => alpha = max(alpha, result.value),
                All => beta = min(beta, result.value),
            }
            if alpha >= beta {
                return result;
            }
        }
    }
    if logical_depth >= depth_limit || true_depth >= depth_limit {
        return SearchResult::new(
            SIDE_SCALAR * lazy_eval + lazy_assess_board(&board),
            lazy_eval,
            None,
            true_depth,
        );
    }
    let mut result = SearchResult::new(0, lazy_eval, None, true_depth);
    match board.side_to_move() {
        Color::White => result.value = i32::MIN,
        Color::Black => result.value = i32::MAX,
    }
    let mut masks = vec![
        !chess::EMPTY,
        board.color_combined(!board.side_to_move()) & !board.pieces(Piece::Pawn),
    ];
    if let Some(old_best) = cached_result {
        if let Some(old_best_move) = old_best.best_move {
            masks.push(BitBoard::from_square(old_best_move.get_dest()))
        }
    }
    masks.reverse();
    let mut moves = MoveGen::new_legal(&board);
    'mask_loop: for mask in masks.into_iter() {
        moves.set_iterator_mask(mask);
        for mov in &mut moves {
            let new_board = board.make_move_new(mov);
            let new_depth = if advantaged_capture(&mov, &board) || new_board.checkers().0 > 0 {
                logical_depth
            } else {
                logical_depth + 1
            };
            let check = search(
                new_board,
                new_depth,
                true_depth + 1,
                depth_limit,
                lazy_eval + assess_incremental(&board, mov),
                alpha,
                beta,
                memo_table,
            );
            match board.side_to_move() {
                Color::White => {
                    alpha = max(alpha, check.value);
                    if check.value > result.value || result.best_move.is_none() {
                        result.value = check.value;
                        result.best_move = Some(mov);
                    }
                    if result.value >= beta {
                        // Beta cutoff
                        result.node_type = Cut;
                        break 'mask_loop;
                    }
                }
                Color::Black => {
                    beta = min(beta, check.value);
                    if check.value < result.value || result.best_move.is_none() {
                        result.value = check.value;
                        result.best_move = Some(mov);
                    }
                    if result.value <= alpha {
                        // Alpha cutoff
                        result.node_type = All;
                        break 'mask_loop;
                    }
                }
            }
        }
    }
    memo_table.replace_if(board.get_hash(), result, |old| old.depth > result.depth);
    result
}

fn main() {
    let mut line_in = String::new();
    let mut fen: String = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
    let mut depth: u16 = 6;
    let mut memo_table: CacheTable<SearchResult> =
        CacheTable::new(2 << 26, SearchResult::new(0, 0, None, u16::MAX));
    let mut board = Board::from_str(fen.as_str()).unwrap();
    loop {
        match stdin().read_line(&mut line_in) {
            Ok(_n) => {
                if line_in.starts_with("fen") {
                    fen = line_in[4..].to_string();
                    board = Board::from_str(fen.as_str()).unwrap();
                }
                if line_in.starts_with("depth") {
                    match line_in[6..].trim().parse::<u16>() {
                        Ok(val) => depth = val,
                        Err(error) => println!("DEPTH ERROR: {} | {}", line_in[6..].trim(), error),
                    }
                }
                if line_in.starts_with("eval") {
                    match start_search(fen.as_str(), depth, &mut memo_table).best_move {
                        Some(good_move) => println!("{}", good_move),
                        None => println!("0000"),
                    }
                }
                if line_in.starts_with("query") {
                    println!(
                        "{}",
                        start_search(board.to_string().as_str(), depth, &mut memo_table).value
                    );
                }
                if line_in.starts_with("move") {
                    if let Ok(chess_move) = ChessMove::from_san(&board, &line_in[5..]) {
                        board = board.make_move_new(chess_move);
                    }
                }
                if line_in.starts_with("reset") {
                    board = Board::from_str(fen.as_str()).unwrap();
                }
                if line_in.starts_with("cget") {
                    if let Some(cached) = memo_table.get(board.get_hash()) {
                        println!(
                            "{} | {}",
                            cached.value,
                            cached.best_move.unwrap_or(ChessMove::default())
                        );
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
