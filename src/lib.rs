use chess::{Board, BoardStatus, CacheTable, ChessMove, Color, MoveGen, Piece};
use std::cmp::{max, min};
use std::ffi::{c_ushort, CStr, CString};
use std::ops::BitAnd;
use std::os::raw::c_char;
use std::str::FromStr;

struct PieceValuePair {
    piece: Piece,
    value: i32,
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct SearchResult {
    value: i32,
    best_move: Option<ChessMove>,
}

const PIECE_VALUES: [PieceValuePair; 5] = [
    PieceValuePair {
        piece: Piece::Pawn,
        value: 10,
    },
    PieceValuePair {
        piece: Piece::Knight,
        value: 30,
    },
    PieceValuePair {
        piece: Piece::Bishop,
        value: 30,
    },
    PieceValuePair {
        piece: Piece::Rook,
        value: 50,
    },
    PieceValuePair {
        piece: Piece::Queen,
        value: 70,
    },
];

fn assess_board(board: &Board) -> i32 {
    let mut val: i32 = 0;
    for piece_val_pair in PIECE_VALUES {
        val += piece_val_pair.value
            * board
                .pieces(piece_val_pair.piece)
                .bitand(board.color_combined(Color::White))
                .popcnt() as i32;
        val -= piece_val_pair.value
            * board
                .pieces(piece_val_pair.piece)
                .bitand(board.color_combined(Color::Black))
                .popcnt() as i32;
    }
    let side_scalar = match board.side_to_move() {
        Color::White => 1,
        Color::Black => -1,
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
    if depth == 0 {
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
    let masks = [board.color_combined(!board.side_to_move()), &!chess::EMPTY];
    let mut moves = MoveGen::new_legal(&board);
    for mask in masks {
        moves.set_iterator_mask(*mask);
        for mov in &mut moves {
            let check = search(board.make_move_new(mov), depth - 1, alpha, beta, memo_table);
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
