use crate::constants::{PIECE_VALUES, PLAYER_SCALAR_MAP, RANK_BITBOARDS};
use chess::{Board, Color, Piece, Rank, Square};

#[inline]
pub fn eval_piece(piece: Piece, position: Square, player: Color) -> i32 {
    PLAYER_SCALAR_MAP[player.to_index()] * PIECE_VALUES[piece.to_index()].value
        + eval_piece_position(piece, position, player)
}

#[inline]
pub fn eval_piece_position(piece: Piece, position: Square, player: Color) -> i32 {
    PIECE_VALUES[piece.to_index()].forward_scale
        * ((player == Color::White) as i32 * position.get_rank().to_index() as i32
            - (player == Color::Black) as i32
                * (Rank::Eighth.to_index() as i32 - position.get_rank().to_index() as i32))
}

#[allow(unused)]
pub fn eval_all_pieces_positional(board: Board) -> i32 {
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
    val
}
