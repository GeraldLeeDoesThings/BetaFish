use chess::{BitBoard, Color, Piece, Square};

pub struct PieceValuePair {
    pub(crate) piece: Piece,
    pub(crate) value: i32,
    pub(crate) forward_scale: i32,
    pub(crate) attack_weight: i32,
}

pub const PIECE_VALUES: [PieceValuePair; 6] = [
    PieceValuePair {
        piece: Piece::Pawn,
        value: 100,
        forward_scale: 7,
        attack_weight: 2,
    },
    PieceValuePair {
        piece: Piece::Knight,
        value: 300,
        forward_scale: 2,
        attack_weight: 2,
    },
    PieceValuePair {
        piece: Piece::Bishop,
        value: 300,
        forward_scale: 2,
        attack_weight: 2,
    },
    PieceValuePair {
        piece: Piece::Rook,
        value: 500,
        forward_scale: 3,
        attack_weight: 3,
    },
    PieceValuePair {
        piece: Piece::Queen,
        value: 700,
        forward_scale: 4,
        attack_weight: 5,
    },
    PieceValuePair {
        piece: Piece::King,
        value: 0,
        forward_scale: -10,
        attack_weight: 0,
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
pub const WHITE_KING_DANGER_SQUARE_MAP: [BitBoard; 64] = [
    BitBoard(0x0000000000030303),
    BitBoard(0x0000000000070707),
    BitBoard(0x00000000000E0E0E),
    BitBoard(0x00000000001C1C1C),
    BitBoard(0x0000000000383838),
    BitBoard(0x0000000000707070),
    BitBoard(0x0000000000E0E0E0),
    BitBoard(0x0000000000C0C0C0),
    BitBoard(0x0000000003030303),
    BitBoard(0x0000000007070707),
    BitBoard(0x000000000E0E0E0E),
    BitBoard(0x000000001C1C1C1C),
    BitBoard(0x0000000038383838),
    BitBoard(0x0000000070707070),
    BitBoard(0x00000000E0E0E0E0),
    BitBoard(0x00000000C0C0C0C0),
    BitBoard(0x0000000303030300),
    BitBoard(0x0000000707070700),
    BitBoard(0x0000000E0E0E0E00),
    BitBoard(0x0000001C1C1C1C00),
    BitBoard(0x0000003838383800),
    BitBoard(0x0000007070707000),
    BitBoard(0x000000E0E0E0E000),
    BitBoard(0x000000C0C0C0C000),
    BitBoard(0x0000030303030000),
    BitBoard(0x0000070707070000),
    BitBoard(0x00000E0E0E0E0000),
    BitBoard(0x00001C1C1C1C0000),
    BitBoard(0x0000383838380000),
    BitBoard(0x0000707070700000),
    BitBoard(0x0000E0E0E0E00000),
    BitBoard(0x0000C0C0C0C00000),
    BitBoard(0x0003030303000000),
    BitBoard(0x0007070707000000),
    BitBoard(0x000E0E0E0E000000),
    BitBoard(0x001C1C1C1C000000),
    BitBoard(0x0038383838000000),
    BitBoard(0x0070707070000000),
    BitBoard(0x00E0E0E0E0000000),
    BitBoard(0x00C0C0C0C0000000),
    BitBoard(0x0303030300000000),
    BitBoard(0x0707070700000000),
    BitBoard(0x0E0E0E0E00000000),
    BitBoard(0x1C1C1C1C00000000),
    BitBoard(0x3838383800000000),
    BitBoard(0x7070707000000000),
    BitBoard(0xE0E0E0E000000000),
    BitBoard(0xC0C0C0C000000000),
    BitBoard(0x0303030000000000),
    BitBoard(0x0707070000000000),
    BitBoard(0x0E0E0E0000000000),
    BitBoard(0x1C1C1C0000000000),
    BitBoard(0x3838380000000000),
    BitBoard(0x7070700000000000),
    BitBoard(0xE0E0E00000000000),
    BitBoard(0xC0C0C00000000000),
    BitBoard(0x0303000000000000),
    BitBoard(0x0707000000000000),
    BitBoard(0x0E0E000000000000),
    BitBoard(0x1C1C000000000000),
    BitBoard(0x3838000000000000),
    BitBoard(0x7070000000000000),
    BitBoard(0xE0E0000000000000),
    BitBoard(0xC0C0000000000000),
];
pub const BLACK_KING_DANGER_SQUARE_MAP: [BitBoard; 64] = [
    BitBoard(0x0000000000000303),
    BitBoard(0x0000000000000707),
    BitBoard(0x0000000000000E0E),
    BitBoard(0x0000000000001C1C),
    BitBoard(0x0000000000003838),
    BitBoard(0x0000000000007070),
    BitBoard(0x000000000000E0E0),
    BitBoard(0x000000000000C0C0),
    BitBoard(0x0000000000030303),
    BitBoard(0x0000000000070707),
    BitBoard(0x00000000000E0E0E),
    BitBoard(0x00000000001C1C1C),
    BitBoard(0x0000000000383838),
    BitBoard(0x0000000000707070),
    BitBoard(0x0000000000E0E0E0),
    BitBoard(0x0000000000C0C0C0),
    BitBoard(0x0000000003030303),
    BitBoard(0x0000000007070707),
    BitBoard(0x000000000E0E0E0E),
    BitBoard(0x000000001C1C1C1C),
    BitBoard(0x0000000038383838),
    BitBoard(0x0000000070707070),
    BitBoard(0x00000000E0E0E0E0),
    BitBoard(0x00000000C0C0C0C0),
    BitBoard(0x0000000303030300),
    BitBoard(0x0000000707070700),
    BitBoard(0x0000000E0E0E0E00),
    BitBoard(0x0000001C1C1C1C00),
    BitBoard(0x0000003838383800),
    BitBoard(0x0000007070707000),
    BitBoard(0x000000E0E0E0E000),
    BitBoard(0x000000C0C0C0C000),
    BitBoard(0x0000030303030000),
    BitBoard(0x0000070707070000),
    BitBoard(0x00000E0E0E0E0000),
    BitBoard(0x00001C1C1C1C0000),
    BitBoard(0x0000383838380000),
    BitBoard(0x0000707070700000),
    BitBoard(0x0000E0E0E0E00000),
    BitBoard(0x0000C0C0C0C00000),
    BitBoard(0x0003030303000000),
    BitBoard(0x0007070707000000),
    BitBoard(0x000E0E0E0E000000),
    BitBoard(0x001C1C1C1C000000),
    BitBoard(0x0038383838000000),
    BitBoard(0x0070707070000000),
    BitBoard(0x00E0E0E0E0000000),
    BitBoard(0x00C0C0C0C0000000),
    BitBoard(0x0303030300000000),
    BitBoard(0x0707070700000000),
    BitBoard(0x0E0E0E0E00000000),
    BitBoard(0x1C1C1C1C00000000),
    BitBoard(0x3838383800000000),
    BitBoard(0x7070707000000000),
    BitBoard(0xE0E0E0E000000000),
    BitBoard(0xC0C0C0C000000000),
    BitBoard(0x0303030000000000),
    BitBoard(0x0707070000000000),
    BitBoard(0x0E0E0E0000000000),
    BitBoard(0x1C1C1C0000000000),
    BitBoard(0x3838380000000000),
    BitBoard(0x7070700000000000),
    BitBoard(0xE0E0E00000000000),
    BitBoard(0xC0C0C00000000000),
];
pub const ATTACK_WEIGHT_MAP: [i32; 100] = [
    0, 0, 1, 2, 3, 5, 7, 9, 12, 15, 18, 22, 26, 30, 35, 39, 44, 50, 56, 62, 68, 75, 82, 85, 89, 97,
    105, 113, 122, 131, 140, 150, 169, 180, 191, 202, 213, 225, 237, 248, 260, 272, 283, 295, 307,
    319, 330, 342, 354, 366, 377, 389, 401, 412, 424, 436, 448, 459, 471, 483, 494, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
];
pub const PLAYER_SCALAR_MAP: [i32; 2] = [1, -1];

#[allow(unused)]
#[inline]
fn expand_square(square: Square) -> [Option<Square>; 9] {
    [
        Some(square),
        square.left(),
        if let Some(left) = square.left() {
            left.up()
        } else {
            None
        },
        if let Some(left) = square.left() {
            left.down()
        } else {
            None
        },
        square.right(),
        if let Some(right) = square.right() {
            right.up()
        } else {
            None
        },
        if let Some(right) = square.right() {
            right.down()
        } else {
            None
        },
        square.up(),
        square.down(),
    ]
}

fn init_king_mask(index: u8, player: Color) -> BitBoard {
    let king_square = unsafe { Square::new(index) };
    let mut bitboard = BitBoard::from_square(king_square);
    for square in expand_square(king_square).iter().flatten() {
        bitboard |= BitBoard::from_square(*square);
    }
    if let Some(above) = match player {
        Color::White => king_square.up(),
        Color::Black => king_square.down(),
    } {
        for square in expand_square(above).iter().flatten() {
            bitboard |= BitBoard::from_square(*square);
        }
    }
    bitboard
}

#[allow(unused)]
pub fn get_white_king_attack_squares() -> [BitBoard; 64] {
    (0..64)
        .map(|v| init_king_mask(v, Color::White))
        .collect::<Vec<BitBoard>>()
        .try_into()
        .unwrap()
}

#[allow(unused)]
pub fn get_black_king_attack_squares() -> [BitBoard; 64] {
    (0..64)
        .map(|v| init_king_mask(v, Color::Black))
        .collect::<Vec<BitBoard>>()
        .try_into()
        .unwrap()
}
