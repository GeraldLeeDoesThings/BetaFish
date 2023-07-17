use chess::{BitBoard, Board, BoardStatus, CacheTable, ChessMove, Color, MoveGen, Piece};
use std::cmp::{max, min};
use std::ffi::{c_ushort, CStr, CString};
use std::os::raw::c_char;
use std::str::FromStr;

struct PieceValuePair {
    piece: Piece,
    value: i32,
    forward_scale: i32,
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct SearchResult {
    value: i32,
    best_move: Option<ChessMove>,
}

const PIECE_VALUES: [PieceValuePair; 5] = [
    PieceValuePair {
        piece: Piece::Pawn,
        value: 100,
        forward_scale: 3,
    },
    PieceValuePair {
        piece: Piece::Knight,
        value: 300,
        forward_scale: 2,
    },
    PieceValuePair {
        piece: Piece::Bishop,
        value: 300,
        forward_scale: 2,
    },
    PieceValuePair {
        piece: Piece::Rook,
        value: 500,
        forward_scale: 3,
    },
    PieceValuePair {
        piece: Piece::Queen,
        value: 700,
        forward_scale: 4,
    },
];
const RANK_BITBOARDS: [BitBoard; 8] = [
    BitBoard(0xFF),
    BitBoard(0xFF << 8),
    BitBoard(0xFF << 16),
    BitBoard(0xFF << 24),
    BitBoard(0xFF << 32),
    BitBoard(0xFF << 40),
    BitBoard(0xFF << 48),
    BitBoard(0xFF << 56),
];
const MAX_DEPTH_INCREASE: u16 = 0;
const SIDE_SCALAR: i32 = 10;

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
    val += side_scalar * MoveGen::new_legal(board).len() as i32;
    if let Some(flipped) = board.null_move() {
        val -= side_scalar * MoveGen::new_legal(&flipped).len() as i32
    }
    val
}

/// # Safety
/// raw_fen_ptr must point to a valid null terminated string
#[no_mangle]
pub unsafe extern "C" fn start_search(
    raw_fen_ptr: *const c_char,
    raw_depth: *const c_ushort,
) -> *mut c_char {
    let mut memo_table: CacheTable<SearchResult> = CacheTable::new(
        2 << 26,
        SearchResult {
            best_move: None,
            value: 0,
        },
    );
    let fen = CStr::from_ptr(raw_fen_ptr).to_str().unwrap();
    let depth = *raw_depth.as_ref().unwrap();
    let best = search(
        Board::from_str(fen).unwrap(),
        depth,
        depth + MAX_DEPTH_INCREASE,
        i32::MIN,
        i32::MAX,
        &mut memo_table,
    );
    match best.best_move {
        Some(best_move) => CString::new(best_move.to_string()).unwrap().into_raw(),
        None => CString::new("0000").unwrap().into_raw(),
    }
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

#[no_mangle]
pub extern "C" fn main() {
    println!("Hello, world!");
}
