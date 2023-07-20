use chess::{BitBoard, Piece};


pub struct PieceValuePair {
    pub(crate) piece: Piece,
    pub(crate) value: i32,
    pub(crate) forward_scale: i32,
}

pub const PIECE_VALUES: [PieceValuePair; 5] = [
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
pub const RANK_BITBOARDS: [BitBoard; 8] = [
    BitBoard(0xFF),
    BitBoard(0xFF << 8),
    BitBoard(0xFF << 16),
    BitBoard(0xFF << 24),
    BitBoard(0xFF << 32),
    BitBoard(0xFF << 40),
    BitBoard(0xFF << 48),
    BitBoard(0xFF << 56),
];
pub const MAX_DEPTH_INCREASE: u16 = 0;
pub const SIDE_SCALAR: i32 = 10;
