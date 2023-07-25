use crate::constants::{PIECE_VALUES, PLAYER_SCALAR_MAP, RANK_BITBOARDS};
use chess::Color::{Black, White};
use chess::File::H;
use chess::Piece::Pawn;
use chess::{Board, Color, Piece, Rank, Square};
use std::cmp::min;

#[inline]
pub fn eval_piece(piece: Piece, position: Square, player: Color) -> i32 {
    PLAYER_SCALAR_MAP[player.to_index()] * PIECE_VALUES[piece.to_index()].value
        + eval_piece_position(piece, position, player)
}

#[inline]
pub fn eval_piece_position(piece: Piece, position: Square, player: Color) -> i32 {
    let rank = position.get_rank() as i32;
    let file = position.get_file() as i32;
    PIECE_VALUES[piece.to_index()].forward_scale
        * ((player == White) as i32 * rank
            - (player == Black) as i32 * (Rank::Eighth.to_index() as i32 - rank))
        + PIECE_VALUES[piece.to_index()].center_scale * min(file, H.to_index() as i32 - file)
}

#[allow(unused)]
pub fn eval_all_pieces_positional(board: &Board) -> i32 {
    let mut val: i32 = 0;
    for piece_val_pair in PIECE_VALUES {
        let piece_bits = board.pieces(piece_val_pair.piece);
        let white_pieces = board.color_combined(White) & piece_bits;
        let black_pieces = board.color_combined(Black) & piece_bits;
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
    val
}

fn eval_pawn_extra(pos: Square, board: &Board) -> i32 {
    let mut bonus: i32 = 0;
    if let Some(below) = pos.down() {
        if let Some(west_defender) = below.left() {
            if board
                .piece_on(west_defender)
                .is_some_and(|piece| piece == Pawn)
            {
                bonus += 5;
            }
        }
        if let Some(east_defender) = below.right() {
            if board
                .piece_on(east_defender)
                .is_some_and(|piece| piece == Pawn)
            {
                bonus += 5;
            }
        }
    }
    bonus
}

pub fn eval_overall_pawn_bonus(board: &Board) -> i32 {
    let mut overall: i32 = 0;
    let white_pawns = board.pieces(Pawn) & board.color_combined(White);
    let black_pawns = board.pieces(Pawn) & board.color_combined(Black);
    for pawn_loc in white_pawns {
        overall += eval_pawn_extra(pawn_loc, board);
    }
    for pawn_loc in black_pawns {
        overall -= eval_pawn_extra(pawn_loc, board);
    }
    overall
}
