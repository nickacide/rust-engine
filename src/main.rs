use std::{collections::HashMap, vec};
// mod test;
mod sliding_pieces;
mod util;

// #[warn(unused, dead_code)]
use std::time::SystemTime;

use crate::sliding_pieces::{
    east_attacks, noea_attacks, nort_attacks, nowe_attacks, soea_attacks, sout_attacks,
    sowe_attacks, west_attacks,
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}
//first time using trait! pretty useful tbh
trait Invert {
    fn invert(&self) -> Color;
}
impl Invert for Color {
    fn invert(&self) -> Color {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

struct Evaluation {
    best_move: Move,
    score: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}
#[derive(Debug, Clone, Copy)]
pub enum PromotionType {
    Queen,
    Rook,
    Bishop,
    Knight,
}
pub enum LineType {
    Horizontal,
    Vertical,
    A1h8,
    H1a8,
}
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

// const not_h: u64 = 0x7f7f7f7f7f7f7f7f;
// const not_a: u64 = 0xfefefefefefefefe;
pub const HORIZONTAL_LOOKUP: [u64; 8] = [
    0xff,
    0xff00,
    0xff0000,
    0xff000000,
    0xff00000000,
    0xff0000000000,
    0xff000000000000,
    0xff00000000000000,
];

const WHITE_QUEENSIDE: u64 = 0xc;
const WHITE_KINGSIDE: u64 = 0x60;
const BLACK_QUEENSIDE: u64 = 0xc00000000000000;
const BLACK_KINGSIDE: u64 = 0x6000000000000000;
// pub enum MoveType {
//     Promotion,
//     EnPassant,
//     Check,
//     Capture,
//     Normal,
// }
// struct Move {
//     promoted_piece: Option<>
// }
// pub struct KingMask {
//     check_mask: u64,
//     pin_mask: u64,
//     checkers: u64,
//     valid_moves: u64,
// }
#[derive(Debug, Clone, Copy)]
pub struct Move {
    from: usize,
    to: usize,
    promoted_piece: Option<PromotionType>,
    piece_color: Color,
    // castling_square: Option<usize>,
}
#[derive(Clone)]
pub struct Masks {
    white_checkmask: u64,
    black_checkmask: u64,
    white_space: u64,
    black_space: u64,
    white_checkers: u64,
    black_checkers: u64,
    white_pinmask: PinMask,
    black_pinmask: PinMask,
    white_pinned: u64,
    black_pinned: u64,
    white_king_danger: u64,
    black_king_danger: u64,
}
#[derive(Clone, Copy)]
pub struct PinMask {
    h: u64,
    v: u64,
    d1: u64,
    d2: u64,
}
#[derive(Clone)]
pub struct Pieces {
    w_king: u64,
    w_queen: u64,
    w_rook: u64,
    w_bishop: u64,
    w_knight: u64,
    w_pawn: u64,
    b_king: u64,
    b_queen: u64,
    b_rook: u64,
    b_bishop: u64,
    b_knight: u64,
    b_pawn: u64,
    white_pieces: u64,
    black_pieces: u64,
    color_lookup: [Option<Color>; 64],
    piece_type_lookup: [Option<PieceType>; 64],
}

static ALL_BITS: u64 = 0xffffffffffffffff;

pub fn vision(pieces: u64, empty: u64) -> u64 {
    nort_attacks(pieces, empty)
        | noea_attacks(pieces, empty)
        | east_attacks(pieces, empty)
        | soea_attacks(pieces, empty)
        | sout_attacks(pieces, empty)
        | sowe_attacks(pieces, empty)
        | west_attacks(pieces, empty)
        | nowe_attacks(pieces, empty)
}

pub fn piece_type(piece: char) -> Option<PieceType> {
    let lowercase_piece = piece.to_lowercase().to_owned().to_string();
    match &*lowercase_piece {
        "k" => Some(PieceType::King),
        "q" => Some(PieceType::Queen),
        "r" => Some(PieceType::Rook),
        "b" => Some(PieceType::Bishop),
        "n" => Some(PieceType::Knight),
        "p" => Some(PieceType::Pawn),
        _ => None,
    }
}

pub fn piece_color(piece: char) -> Option<Color> {
    if piece.is_numeric() {
        return None;
    }
    if piece.is_uppercase() {
        return Some(Color::White);
    }
    Some(Color::Black)
}

pub fn set_bit(bitboard: &mut u64, index: usize, state: bool) {
    let board = *bitboard;
    let new_bitboard: u64 = match state {
        true => board | 1 << index,
        false => board & !(1 << index),
    };
    *bitboard = new_bitboard;
}

impl Pieces {
    fn new(position: String) -> Pieces {
        let mut w_king = 0u64;
        let mut w_queen = 0u64;
        let mut w_rook = 0u64;
        let mut w_bishop = 0u64;
        let mut w_knight = 0u64;
        let mut w_pawn = 0u64;
        let mut b_king = 0u64;
        let mut b_queen = 0u64;
        let mut b_rook = 0u64;
        let mut b_bishop = 0u64;
        let mut b_knight = 0u64;
        let mut b_pawn = 0u64;
        let mut color_lookup: [Option<Color>; 64] = [None; 64];
        let mut piece_type_lookup: [Option<PieceType>; 64] = [None; 64];

        let parsed: Vec<&str> = position.split("/").collect();
        let mut rank_count = 0;
        let mut offset = 0;
        for rank in parsed {
            if rank_count > 7 {
                panic!("Invalid FEN position")
            }
            for sq in 0..rank.len() {
                let square = rank.as_bytes()[sq] as char;
                let idx = (7 - rank_count) * 8 + sq + offset;
                // println!("{} {}", idx, offset);
                // let piece_type = piece_type(square);
                // let color = piece_color(square);
                // print!("{}", square);
                // let mut bitboard = 0;
                // if !square.is_numeric() {
                //     bitboard = match square {
                //         'K' => w_king,
                //         'Q' => w_queen,
                //         'R' => w_rook,
                //         'B' => w_bishop,
                //         'N' => w_knight,
                //         'P' => w_pawn,
                //         'k' => b_king,
                //         'q' => b_queen,
                //         'r' => b_rook,
                //         'b' => b_bishop,
                //         'n' => b_knight,
                //         'p' => b_pawn,
                //         _ => panic!("How did we get here?"),
                //     };
                // } else {
                //     num_offset += square as usize;
                // }
                match square {
                    'K' => {
                        set_bit(&mut w_king, idx, true);
                        color_lookup[idx] = Some(Color::White);
                        piece_type_lookup[idx] = Some(PieceType::King);
                    }
                    'Q' => {
                        set_bit(&mut w_queen, idx, true);
                        color_lookup[idx] = Some(Color::White);
                        piece_type_lookup[idx] = Some(PieceType::Queen);
                    }
                    'R' => {
                        set_bit(&mut w_rook, idx, true);
                        color_lookup[idx] = Some(Color::White);
                        piece_type_lookup[idx] = Some(PieceType::Rook);
                    }
                    'B' => {
                        set_bit(&mut w_bishop, idx, true);
                        color_lookup[idx] = Some(Color::White);
                        piece_type_lookup[idx] = Some(PieceType::Bishop);
                    }
                    'N' => {
                        set_bit(&mut w_knight, idx, true);
                        color_lookup[idx] = Some(Color::White);
                        piece_type_lookup[idx] = Some(PieceType::Knight);
                    }
                    'P' => {
                        set_bit(&mut w_pawn, idx, true);
                        color_lookup[idx] = Some(Color::White);
                        piece_type_lookup[idx] = Some(PieceType::Pawn);
                    }
                    'k' => {
                        set_bit(&mut b_king, idx, true);
                        color_lookup[idx] = Some(Color::Black);
                        piece_type_lookup[idx] = Some(PieceType::King);
                    }
                    'q' => {
                        set_bit(&mut b_queen, idx, true);
                        color_lookup[idx] = Some(Color::Black);
                        piece_type_lookup[idx] = Some(PieceType::Queen);
                    }
                    'r' => {
                        set_bit(&mut b_rook, idx, true);
                        color_lookup[idx] = Some(Color::Black);
                        piece_type_lookup[idx] = Some(PieceType::Rook);
                    }
                    'b' => {
                        set_bit(&mut b_bishop, idx, true);
                        color_lookup[idx] = Some(Color::Black);
                        piece_type_lookup[idx] = Some(PieceType::Bishop);
                    }
                    'n' => {
                        set_bit(&mut b_knight, idx, true);
                        color_lookup[idx] = Some(Color::Black);
                        piece_type_lookup[idx] = Some(PieceType::Knight);
                    }
                    'p' => {
                        set_bit(&mut b_pawn, idx, true);
                        color_lookup[idx] = Some(Color::Black);
                        piece_type_lookup[idx] = Some(PieceType::Pawn);
                    }
                    int => offset += int as usize - 49, // 1
                }
            }
            rank_count += 1;
            offset = 0;
        }
        let white_pieces = w_king | w_queen | w_rook | w_bishop | w_knight | w_pawn;
        let black_pieces = b_king | b_queen | b_rook | b_bishop | b_knight | b_pawn;
        // println!(
        //     "White Pieces: {}; Black Pieces: {}",
        //     white_pieces, black_pieces
        // );
        Pieces {
            w_king,
            w_queen,
            w_rook,
            w_bishop,
            w_knight,
            w_pawn,
            b_king,
            b_queen,
            b_rook,
            b_bishop,
            b_knight,
            b_pawn,
            white_pieces,
            black_pieces,
            color_lookup,
            piece_type_lookup,
        }
    }
}
#[derive(Clone)]
struct GameState {
    pieces: Pieces,
    empty: u64,
    white_castling: (bool, bool), //Queenside, Kingside (FEN)
    black_castling: (bool, bool),
    legal_castling: (bool, bool, bool, bool), // Evaluated castling (after analysis)
    active_color: Color,
    halfmoves: usize,
    fullmoves: usize,
    en_passant: Option<usize>,

    king_lookup: Vec<u64>,
    queen_lookup: Vec<u64>,
    rook_lookup: Vec<u64>,
    bishop_lookup: Vec<u64>,
    knight_lookup: Vec<u64>,

    masks: Masks,
    w_king_idx: usize,
    b_king_idx: usize,
    white_pawn_lookup: Vec<u64>, // captures
    black_pawn_lookup: Vec<u64>,
}

impl GameState {
    fn new(fen: String) -> GameState {
        let parsed: Vec<&str> = fen.split(" ").collect();
        let position = parsed[0].to_owned();
        let active_color = match parsed[1].to_lowercase().as_str() {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Invalid color"),
        };
        //Castling verified later on
        let castling = parsed[2].to_owned();
        let mut white_castling = (false, false);
        let mut black_castling = (false, false);
        if castling.contains("Q") {
            white_castling.0 = true;
        }
        if castling.contains("K") {
            white_castling.1 = true;
        }
        if castling.contains("q") {
            black_castling.0 = true;
        }
        if castling.contains("k") {
            black_castling.1 = true;
        }
        let en_passant = match parsed[3] {
            "-" => None,
            str => {
                if str.len() != 2 {
                    panic!("invalid en passant square (too long)");
                }
                let mut chars = str.chars();
                let file = chars.next().expect("Invalid en passant square") as usize - 97; // 'a'
                let rank = chars.next().expect("Invalid en passant square") as usize - 49; // '1'
                Some(file + 8 * rank)
            }
        };
        let halfmoves = parsed[4].chars().next().expect("Invalid halfmove clock") as usize - 48; // '0'
        let fullmoves = parsed[5].chars().next().expect("Invalid fullmove clock") as usize - 48;
        let mut king_lookup: Vec<u64> = vec![];
        let mut queen_lookup: Vec<u64> = vec![];
        let mut rook_lookup: Vec<u64> = vec![];
        let mut bishop_lookup: Vec<u64> = vec![];
        let mut knight_lookup: Vec<u64> = vec![];
        let mut white_pawn_lookup: Vec<u64> = vec![];
        let mut black_pawn_lookup: Vec<u64> = vec![];

        let mut slide_lookup: HashMap<u64, u64> = HashMap::new();
        for idx in 0..64 {
            //generate lookup tables
            king_lookup.push(piece_lookup(idx, PieceType::King, None));
            queen_lookup.push(piece_lookup(idx, PieceType::Queen, None));
            rook_lookup.push(piece_lookup(idx, PieceType::Rook, None));
            bishop_lookup.push(piece_lookup(idx, PieceType::Bishop, None));
            knight_lookup.push(piece_lookup(idx, PieceType::Knight, None));
            black_pawn_lookup.push(piece_lookup(idx, PieceType::Pawn, Some(Color::Black)));
            white_pawn_lookup.push(piece_lookup(idx, PieceType::Pawn, Some(Color::White)));
            let mut slide_squares = queen_lookup[idx];
            while slide_squares > 0 {
                let slide_square = slide_squares.trailing_zeros() as u64;
                let key = (1u64 << idx as u64) | (1u64 << slide_square);
                slide_lookup.insert(key, generate_slide_lookup(key));
                slide_squares &= slide_squares - 1;
            }
        }
        let pieces = Pieces::new(position);

        let w_king_idx = pieces.w_king.trailing_zeros() as usize;
        let b_king_idx = pieces.b_king.trailing_zeros() as usize;

        let potential_white_checkers = (queen_lookup[w_king_idx] & pieces.b_queen)
            | (rook_lookup[w_king_idx] & pieces.b_rook)
            | (bishop_lookup[w_king_idx] & pieces.b_bishop);
        let potential_black_checkers = (queen_lookup[b_king_idx] & pieces.w_queen)
            | (rook_lookup[b_king_idx] & pieces.w_rook)
            | (bishop_lookup[b_king_idx] & pieces.w_bishop);

        let mut white_checkmask = 0u64;
        let mut black_checkmask = 0u64;
        let white_pinmask = 0u64;
        let black_pinmask = 0u64;

        white_checkmask |= (knight_lookup[w_king_idx] & pieces.b_knight)
            | (king_lookup[w_king_idx] & pieces.b_king);
        black_checkmask |= (knight_lookup[b_king_idx] & pieces.w_knight)
            | (king_lookup[b_king_idx] & pieces.w_king);

        let empty = !(pieces.white_pieces | pieces.black_pieces);
        let white_king_vision = vision(pieces.w_king, empty);
        let black_king_vision = vision(pieces.b_king, empty);
        let mut white_checkers = white_king_vision & potential_white_checkers;
        let mut black_checkers = black_king_vision & potential_black_checkers;

        let white_pinned = ((nort_attacks(pieces.w_king, empty)
            & sout_attacks(pieces.b_rook | pieces.b_queen, empty))
            | (noea_attacks(pieces.w_king, empty)
                & sowe_attacks(pieces.b_bishop | pieces.b_queen, empty))
            | (east_attacks(pieces.w_king, empty)
                & west_attacks(pieces.b_rook | pieces.b_queen, empty))
            | (soea_attacks(pieces.w_king, empty)
                & nowe_attacks(pieces.b_bishop | pieces.b_queen, empty))
            | (sout_attacks(pieces.w_king, empty)
                & nort_attacks(pieces.b_rook | pieces.b_queen, empty))
            | (sowe_attacks(pieces.w_king, empty)
                & noea_attacks(pieces.b_bishop | pieces.b_queen, empty))
            | (west_attacks(pieces.w_king, empty)
                & east_attacks(pieces.b_rook | pieces.b_queen, empty))
            | (nowe_attacks(pieces.w_king, empty)
                & soea_attacks(pieces.b_bishop | pieces.b_queen, empty)))
            & pieces.white_pieces;

        let black_pinned = ((nort_attacks(pieces.b_king, empty)
            & sout_attacks(pieces.w_rook | pieces.w_queen, empty))
            | (noea_attacks(pieces.b_king, empty)
                & sowe_attacks(pieces.w_bishop | pieces.w_queen, empty))
            | (east_attacks(pieces.b_king, empty)
                & west_attacks(pieces.w_rook | pieces.w_queen, empty))
            | (soea_attacks(pieces.b_king, empty)
                & nowe_attacks(pieces.w_bishop | pieces.w_queen, empty))
            | (sout_attacks(pieces.b_king, empty)
                & nort_attacks(pieces.w_rook | pieces.w_queen, empty))
            | (sowe_attacks(pieces.b_king, empty)
                & noea_attacks(pieces.w_bishop | pieces.w_queen, empty))
            | (west_attacks(pieces.b_king, empty)
                & east_attacks(pieces.w_rook | pieces.w_queen, empty))
            | (nowe_attacks(pieces.b_king, empty)
                & soea_attacks(pieces.w_bishop | pieces.w_queen, empty)))
            & pieces.black_pieces;

        let mut white_checkers_copy = white_checkers;
        let mut black_checkers_copy = black_checkers;

        while white_checkers_copy > 0 {
            white_checkmask |= slide_lookup
                .get(&(pieces.w_king | (1u64 << white_checkers_copy.trailing_zeros())))
                .unwrap();
            white_checkers_copy &= white_checkers_copy - 1;
        }
        while black_checkers_copy > 0 {
            black_checkmask |= slide_lookup
                .get(&(pieces.b_king | (1u64 << black_checkers_copy.trailing_zeros())))
                .unwrap();
            black_checkers_copy &= black_checkers_copy - 1;
        }

        white_checkers |= (knight_lookup[w_king_idx] & pieces.b_knight)
            | (king_lookup[w_king_idx] & pieces.b_king)
            | (white_pawn_lookup[w_king_idx] & pieces.b_pawn);
        black_checkers |= (knight_lookup[b_king_idx] & pieces.w_knight)
            | (king_lookup[b_king_idx] & pieces.w_king)
            | (black_pawn_lookup[b_king_idx] & pieces.w_pawn);

        white_checkmask |= white_pawn_lookup[w_king_idx] & pieces.b_pawn;
        black_checkmask |= black_pawn_lookup[b_king_idx] & pieces.w_pawn;

        white_checkmask |= knight_lookup[w_king_idx] & pieces.b_knight;
        black_checkmask |= knight_lookup[b_king_idx] & pieces.w_knight;
        white_checkmask &= !pieces.w_king;
        black_checkmask &= !pieces.b_king;

        let white_pinmask = PinMask {
            h: (east_attacks(pieces.w_king, empty | white_pinned)
                | west_attacks(pieces.w_king, empty | white_pinned))
                & !white_checkmask,
            v: (nort_attacks(pieces.w_king, empty | white_pinned)
                | sout_attacks(pieces.w_king, empty | white_pinned))
                & !white_checkmask,
            d1: (noea_attacks(pieces.w_king, empty | white_pinned)
                | sowe_attacks(pieces.w_king, empty | white_pinned))
                & !white_checkmask,
            d2: (nowe_attacks(pieces.w_king, empty | white_pinned)
                | soea_attacks(pieces.w_king, empty | white_pinned))
                & !white_checkmask,
        };
        let black_pinmask = PinMask {
            h: (east_attacks(pieces.b_king, empty | black_pinned)
                | west_attacks(pieces.b_king, empty | black_pinned))
                & !black_checkmask,
            v: (nort_attacks(pieces.b_king, empty | black_pinned)
                | sout_attacks(pieces.b_king, empty | black_pinned))
                & !black_checkmask,
            d1: (noea_attacks(pieces.b_king, empty | black_pinned)
                | sowe_attacks(pieces.b_king, empty | black_pinned))
                & !black_checkmask,
            d2: (nowe_attacks(pieces.b_king, empty | black_pinned)
                | soea_attacks(pieces.b_king, empty | black_pinned))
                & !black_checkmask,
        };

        if white_checkmask == 0 {
            white_checkmask = ALL_BITS;
        }
        if black_checkmask == 0 {
            black_checkmask = ALL_BITS;
        }

        white_checkmask &= !pieces.white_pieces;
        black_checkmask &= !pieces.black_pieces;

        let mut white_space = nort_attacks(pieces.w_rook | pieces.w_queen, empty)
            | noea_attacks(pieces.w_bishop | pieces.w_queen, empty)
            | east_attacks(pieces.w_rook | pieces.w_queen, empty)
            | soea_attacks(pieces.w_bishop | pieces.w_queen, empty)
            | sout_attacks(pieces.w_rook | pieces.w_queen, empty)
            | sowe_attacks(pieces.w_bishop | pieces.w_queen, empty)
            | west_attacks(pieces.w_rook | pieces.w_queen, empty)
            | nowe_attacks(pieces.w_bishop | pieces.w_queen, empty);
        let mut black_space = nort_attacks(pieces.b_rook | pieces.b_queen, empty)
            | noea_attacks(pieces.b_bishop | pieces.b_queen, empty)
            | east_attacks(pieces.b_rook | pieces.b_queen, empty)
            | soea_attacks(pieces.b_bishop | pieces.b_queen, empty)
            | sout_attacks(pieces.b_rook | pieces.b_queen, empty)
            | sowe_attacks(pieces.b_bishop | pieces.b_queen, empty)
            | west_attacks(pieces.b_rook | pieces.b_queen, empty)
            | nowe_attacks(pieces.b_bishop | pieces.b_queen, empty);

        let white_king_empty = empty | pieces.w_king;
        let black_king_empty = empty | pieces.b_king;
        let mut white_king_danger = nort_attacks(pieces.b_rook | pieces.b_queen, white_king_empty)
            | noea_attacks(pieces.b_bishop | pieces.b_queen, white_king_empty)
            | east_attacks(pieces.b_rook | pieces.b_queen, white_king_empty)
            | soea_attacks(pieces.b_bishop | pieces.b_queen, white_king_empty)
            | sout_attacks(pieces.b_rook | pieces.b_queen, white_king_empty)
            | sowe_attacks(pieces.b_bishop | pieces.b_queen, white_king_empty)
            | west_attacks(pieces.b_rook | pieces.b_queen, white_king_empty)
            | nowe_attacks(pieces.b_bishop | pieces.b_queen, white_king_empty);
        let mut black_king_danger = nort_attacks(pieces.w_rook | pieces.w_queen, black_king_empty)
            | noea_attacks(pieces.w_bishop | pieces.w_queen, black_king_empty)
            | east_attacks(pieces.w_rook | pieces.w_queen, black_king_empty)
            | soea_attacks(pieces.w_bishop | pieces.w_queen, black_king_empty)
            | sout_attacks(pieces.w_rook | pieces.w_queen, black_king_empty)
            | sowe_attacks(pieces.w_bishop | pieces.w_queen, black_king_empty)
            | west_attacks(pieces.w_rook | pieces.w_queen, black_king_empty)
            | nowe_attacks(pieces.w_bishop | pieces.w_queen, black_king_empty);

        white_space |= king_lookup[w_king_idx];
        black_space |= king_lookup[b_king_idx];

        let mut pawns_copy = pieces.w_pawn;
        while pawns_copy > 0 {
            white_space |= white_pawn_lookup[pawns_copy.trailing_zeros() as usize];
            black_king_danger |= white_pawn_lookup[pawns_copy.trailing_zeros() as usize];
            pawns_copy &= pawns_copy - 1;
        }
        pawns_copy = pieces.b_pawn;
        while pawns_copy > 0 {
            black_space |= black_pawn_lookup[pawns_copy.trailing_zeros() as usize];
            white_king_danger |= black_pawn_lookup[pawns_copy.trailing_zeros() as usize];
            pawns_copy &= pawns_copy - 1;
        }

        let mut knights_copy = pieces.w_knight;
        while knights_copy > 0 {
            white_space |= knight_lookup[knights_copy.trailing_zeros() as usize];
            black_king_danger |= knight_lookup[knights_copy.trailing_zeros() as usize];
            knights_copy &= knights_copy - 1;
        }
        white_king_danger |= king_lookup[b_king_idx];
        black_king_danger |= king_lookup[w_king_idx];
        knights_copy = pieces.b_knight;
        while knights_copy > 0 {
            black_space |= knight_lookup[knights_copy.trailing_zeros() as usize];
            white_king_danger |= knight_lookup[knights_copy.trailing_zeros() as usize];
            knights_copy &= knights_copy - 1;
        }
        let mut legal_castling = (false, false, false, false);
        if w_king_idx == 4 {
            if white_castling.0
                && pieces.w_rook & 1 == 1
                && empty & 0xe == 0xe
                && black_space & 0xc == 0
            {
                legal_castling.0 = true;
            }
            if white_castling.1
                && pieces.w_rook & 0x80 == 0x80
                && empty & 0x60 == 0x60
                && black_space & 0x60 == 0
            {
                legal_castling.1 = true;
            }
        };
        if b_king_idx == 60 {
            if black_castling.0
                && pieces.b_rook & 1 << 56 == 1 << 56
                && empty & 0xe00000000000000 == 0xe00000000000000
                && white_space & 0xc00000000000000 == 0
            {
                legal_castling.2 = true;
            }
            if black_castling.1
                && pieces.b_rook & 1 << 63 == 1 << 63
                && empty & 0x6000000000000000 == 0x6000000000000000
                && white_space & 0x6000000000000000 == 0
            {
                legal_castling.3 = true;
            }
        }
        GameState {
            pieces,
            empty,
            white_castling,
            black_castling,
            legal_castling,
            active_color,
            halfmoves,
            fullmoves,
            en_passant,
            king_lookup,
            queen_lookup,
            rook_lookup,
            bishop_lookup,
            knight_lookup,
            white_pawn_lookup,
            black_pawn_lookup,
            w_king_idx,
            b_king_idx,
            masks: Masks {
                white_checkmask,
                black_checkmask,
                white_space,
                black_space,
                white_checkers,
                black_checkers,
                white_pinmask,
                black_pinmask,
                white_pinned,
                black_pinned,
                white_king_danger,
                black_king_danger,
            },
        }
    }
    //function does not check for legality, that is the job of the movegen
    fn apply_move(&self, piece_move: Move) -> GameState {
        let mut new_gamestate = self.clone();
        let piece = self.pieces.piece_type_lookup[piece_move.from];
        match piece {
            Some(PieceType::King) => {
                if piece_move.piece_color == Color::White {
                    new_gamestate.legal_castling.0 = false;
                    new_gamestate.legal_castling.1 = false;
                    new_gamestate.pieces.w_king = 1u64 << piece_move.to;
                } else {
                    new_gamestate.legal_castling.2 = false;
                    new_gamestate.legal_castling.3 = false;
                    new_gamestate.pieces.b_king = 1u64 << piece_move.to;
                }
                new_gamestate.pieces.piece_type_lookup[piece_move.from] = None;
                new_gamestate.pieces.piece_type_lookup[piece_move.to] = Some(PieceType::King);
            }
            Some(PieceType::Queen) => {
                match piece_move.piece_color {
                    Color::White => {
                        new_gamestate.pieces.w_queen &= !(1u64 << piece_move.from);
                        new_gamestate.pieces.w_queen |= 1u64 << piece_move.to;
                    }
                    Color::Black => {
                        new_gamestate.pieces.b_queen &= !(1u64 << piece_move.from);
                        new_gamestate.pieces.b_queen |= 1u64 << piece_move.to;
                    }
                };
                new_gamestate.pieces.piece_type_lookup[piece_move.from] = None;
                new_gamestate.pieces.piece_type_lookup[piece_move.to] = Some(PieceType::Queen);
            }
            Some(PieceType::Rook) => {
                match piece_move.from {
                    00 => new_gamestate.legal_castling.0 = false,
                    07 => new_gamestate.legal_castling.1 = false,
                    56 => new_gamestate.legal_castling.2 = false,
                    63 => new_gamestate.legal_castling.3 = false,
                    _ => {}
                }
                match piece_move.piece_color {
                    Color::White => {
                        new_gamestate.pieces.w_rook &= !(1u64 << piece_move.from);
                        new_gamestate.pieces.w_rook |= 1u64 << piece_move.to;
                    }
                    Color::Black => {
                        new_gamestate.pieces.b_rook &= !(1u64 << piece_move.from);
                        new_gamestate.pieces.b_rook |= 1u64 << piece_move.to;
                    }
                }
                new_gamestate.pieces.piece_type_lookup[piece_move.from] = None;
                new_gamestate.pieces.piece_type_lookup[piece_move.to] = Some(PieceType::Rook);
            }
            Some(PieceType::Bishop) => {
                match piece_move.piece_color {
                    Color::White => {
                        new_gamestate.pieces.w_bishop &= !(1u64 << piece_move.from);
                        new_gamestate.pieces.w_bishop |= 1u64 << piece_move.to;
                    }
                    Color::Black => {
                        new_gamestate.pieces.b_bishop &= !(1u64 << piece_move.from);
                        new_gamestate.pieces.b_bishop |= 1u64 << piece_move.to;
                    }
                };
                new_gamestate.pieces.piece_type_lookup[piece_move.from] = None;
                new_gamestate.pieces.piece_type_lookup[piece_move.to] = Some(PieceType::Bishop);
            }
            Some(PieceType::Knight) => {
                match piece_move.piece_color {
                    Color::White => {
                        new_gamestate.pieces.w_knight &= !(1u64 << piece_move.from);
                        new_gamestate.pieces.w_knight |= 1u64 << piece_move.to;
                    }
                    Color::Black => {
                        new_gamestate.pieces.b_knight &= !(1u64 << piece_move.from);
                        new_gamestate.pieces.b_knight |= 1u64 << piece_move.to;
                    }
                };
                new_gamestate.pieces.piece_type_lookup[piece_move.from] = None;
                new_gamestate.pieces.piece_type_lookup[piece_move.to] = Some(PieceType::Knight);
            }
            Some(PieceType::Pawn) => {
                match piece_move.piece_color {
                    Color::White => {
                        new_gamestate.pieces.w_queen &= !(1u64 << piece_move.from);
                        new_gamestate.pieces.w_queen |= 1u64 << piece_move.to;
                    }
                    Color::Black => {
                        new_gamestate.pieces.b_queen &= !(1u64 << piece_move.from);
                        new_gamestate.pieces.b_queen |= 1u64 << piece_move.to;
                    }
                };
                new_gamestate.pieces.piece_type_lookup[piece_move.from] = None;
                new_gamestate.pieces.piece_type_lookup[piece_move.to] = Some(PieceType::Queen);
                if piece_move.from.abs_diff(piece_move.to) == 16 {
                    match piece_move.piece_color {
                        Color::White => {
                            if (1u64 << piece_move.to << 1 | 1u64 << piece_move.to >> 1)
                                & self.pieces.b_pawn
                                > 0
                            {
                                new_gamestate.en_passant = Some(piece_move.to >> 8);
                            }
                        }
                        Color::Black => {
                            if (1u64 << piece_move.to << 1 | 1u64 << piece_move.to >> 1)
                                & self.pieces.w_pawn
                                > 0
                            {
                                new_gamestate.en_passant = Some(piece_move.to << 8);
                            }
                        }
                    }
                }
            }
            None => {}
        }
        // new_gamestate.active_color = new_gamestate.active_color.invert();
        new_gamestate
    }
    fn perft(&self, depth: usize) -> u64 {
        let mut nodes = 0;
        if depth == 0 {
            return 1;
        }
        for piece_move in self.moves(self.active_color) {
            let new_gamestate = self.apply_move(piece_move);
            nodes += new_gamestate.perft(depth - 1);
        }
        return nodes;
    }
    fn default() -> GameState {
        GameState::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_owned())
    }
    fn king_moves(&self, color: Color) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        let king_idx = match color {
            Color::White => self.w_king_idx,
            Color::Black => self.b_king_idx,
        };
        let mut bb;
        match color {
            Color::White => {
                bb = self.king_lookup[self.w_king_idx]
                    & !self.masks.white_king_danger
                    & !self.pieces.white_pieces;
                if self.legal_castling.0 {
                    bb |= 0x4
                }
                if self.legal_castling.1 {
                    bb |= 0x40
                }
            }
            Color::Black => {
                bb = self.king_lookup[self.b_king_idx]
                    & !self.masks.black_king_danger
                    & !self.pieces.black_pieces;
                if self.legal_castling.2 {
                    bb |= 400000000000000
                }
                if self.legal_castling.3 {
                    bb |= 4000000000000000
                }
            }
        };
        while bb > 0 {
            let king_move = bb.trailing_zeros() as usize;
            moves.push(Move {
                from: king_idx,
                to: king_move,
                piece_color: color,
                promoted_piece: None,
            });
            bb &= bb - 1;
        }
        moves
    }
    fn knight_moves(&self, color: Color) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        let our_pieces;
        let mut our_knights;
        let us_pinned;
        match color {
            Color::White => {
                our_knights = self.pieces.w_knight;
                our_pieces = self.pieces.white_pieces;
                us_pinned = self.masks.white_pinned;
            }
            Color::Black => {
                our_knights = self.pieces.b_knight;
                our_pieces = self.pieces.black_pieces;
                us_pinned = self.masks.black_pinned;
            }
        }
        while our_knights > 0 {
            if us_pinned & 1u64 << our_knights.trailing_zeros() > 0 {
                our_knights &= our_knights - 1;
                continue;
            }
            let mut bb_moves =
                self.knight_lookup[our_knights.trailing_zeros() as usize] & !our_pieces;
            while bb_moves > 0 {
                moves.push(Move {
                    from: our_knights.trailing_zeros() as usize,
                    to: bb_moves.trailing_zeros() as usize,
                    piece_color: color,
                    promoted_piece: None,
                });
                bb_moves &= bb_moves - 1;
            }
            our_knights &= our_knights - 1;
        }
        moves
    }
    fn rook_moves(&self, color: Color) -> Vec<Move> {
        let our_rooks;
        let our_pieces;
        let mut our_movemask;
        let our_pinmask;
        let us_pinned;
        let our_king;
        match color {
            Color::White => {
                our_rooks = self.pieces.w_rook;
                our_pieces = self.pieces.white_pieces;
                our_movemask = self.masks.white_checkmask;
                our_pinmask = self.masks.white_pinmask;
                us_pinned = self.masks.white_pinned;
                our_king = self.pieces.w_king;
            }
            Color::Black => {
                our_rooks = self.pieces.b_rook;
                our_pieces = self.pieces.black_pieces;
                our_movemask = self.masks.black_checkmask;
                our_pinmask = self.masks.black_pinmask;
                us_pinned = self.masks.black_pinned;
                our_king = self.pieces.b_king;
            }
        }
        let mut moves: Vec<Move> = vec![];
        let mut rook = our_rooks;
        while rook > 0 {
            let current_piece = rook.trailing_zeros() as u64;
            if us_pinned & 1u64 << current_piece > 0 {
                let king_rank = (our_king.trailing_zeros() / 8) as i8;
                let king_file = (our_king.trailing_zeros() % 8) as i8;
                let rank = (current_piece / 8) as i8;
                let file = (current_piece % 8) as i8;
                if king_rank == rank {
                    our_movemask &= our_pinmask.h;
                } else if king_file == file {
                    our_movemask &= our_pinmask.v;
                } else if king_file - file == king_rank - rank {
                    rook &= rook - 1;
                    continue;
                } else if king_file - file == rank - king_rank {
                    rook &= rook - 1;
                    continue;
                } else {
                    panic!("pin?")
                }
            }
            let mut bb_moves = (nort_attacks(1u64 << current_piece, self.empty)
                | east_attacks(1u64 << current_piece, self.empty)
                | sout_attacks(1u64 << current_piece, self.empty)
                | west_attacks(1u64 << current_piece, self.empty))
                & !our_pieces
                & our_movemask;
            while bb_moves > 0 {
                let bb_move = bb_moves.trailing_zeros();
                moves.push(Move {
                    from: current_piece as usize,
                    to: bb_move as usize,
                    piece_color: color,
                    promoted_piece: None,
                });
                bb_moves &= bb_moves - 1;
            }
            rook &= rook - 1;
        }
        moves
    }
    fn bishop_moves(&self, color: Color) -> Vec<Move> {
        let our_bishops;
        let our_pieces;
        let mut our_movemask;
        let our_pinmask;
        let us_pinned;
        let our_king;
        match color {
            Color::White => {
                our_bishops = self.pieces.w_bishop;
                our_pieces = self.pieces.white_pieces;
                our_movemask = self.masks.white_checkmask;
                our_pinmask = self.masks.white_pinmask;
                us_pinned = self.masks.white_pinned;
                our_king = self.pieces.w_king;
            }
            Color::Black => {
                our_bishops = self.pieces.b_bishop;
                our_pieces = self.pieces.black_pieces;
                our_movemask = self.masks.black_checkmask;
                our_pinmask = self.masks.black_pinmask;
                us_pinned = self.masks.black_pinned;
                our_king = self.pieces.b_king;
            }
        }

        let mut moves: Vec<Move> = vec![];
        let mut bishop = our_bishops;
        while bishop > 0 {
            let current_piece = bishop.trailing_zeros() as u64;
            if us_pinned & 1u64 << current_piece > 0 {
                let king_rank = (our_king.trailing_zeros() / 8) as i8;
                let king_file = (our_king.trailing_zeros() % 8) as i8;
                let rank = (current_piece / 8) as i8;
                let file = (current_piece % 8) as i8;
                if king_rank == rank {
                    bishop &= bishop - 1;
                    continue;
                } else if king_file == file {
                    bishop &= bishop - 1;
                    continue;
                } else if king_file - file == king_rank - rank {
                    our_movemask &= our_pinmask.d1;
                } else if king_file - file == rank - king_rank {
                    our_movemask &= our_pinmask.d2;
                } else {
                    panic!("pin?")
                }
            }
            let mut bb_moves = (noea_attacks(1u64 << current_piece, self.empty)
                | soea_attacks(1u64 << current_piece, self.empty)
                | sowe_attacks(1u64 << current_piece, self.empty)
                | nowe_attacks(1u64 << current_piece, self.empty))
                & !our_pieces
                & our_movemask;
            while bb_moves > 0 {
                let bb_move = bb_moves.trailing_zeros();
                moves.push(Move {
                    from: current_piece as usize,
                    to: bb_move as usize,
                    piece_color: color,
                    promoted_piece: None,
                });
                bb_moves &= bb_moves - 1;
            }
            bishop &= bishop - 1;
        }
        moves
    }
    fn queen_moves(&self, color: Color) -> Vec<Move> {
        let our_queens;
        let our_pieces;
        let mut our_movemask;
        let our_pinmask;
        let us_pinned;
        let our_king;
        match color {
            Color::White => {
                our_queens = self.pieces.w_queen;
                our_pieces = self.pieces.white_pieces;
                our_movemask = self.masks.white_checkmask;
                our_pinmask = self.masks.white_pinmask;
                us_pinned = self.masks.white_pinned;
                our_king = self.pieces.w_king;
            }
            Color::Black => {
                our_queens = self.pieces.b_queen;
                our_pieces = self.pieces.black_pieces;
                our_movemask = self.masks.black_checkmask;
                our_pinmask = self.masks.black_pinmask;
                us_pinned = self.masks.black_pinned;
                our_king = self.pieces.b_king;
            }
        }
        let mut moves: Vec<Move> = vec![];
        let mut queen = our_queens;
        while queen > 0 {
            let current_piece = queen.trailing_zeros() as u64;
            if us_pinned & 1u64 << current_piece > 0 {
                let king_rank = (our_king.trailing_zeros() / 8) as i8;
                let king_file = (our_king.trailing_zeros() % 8) as i8;
                let rank = (current_piece / 8) as i8;
                let file = (current_piece % 8) as i8;
                if king_rank == rank {
                    our_movemask &= our_pinmask.h;
                } else if king_file == file {
                    our_movemask &= our_pinmask.v;
                } else if king_file - file == king_rank - rank {
                    our_movemask &= our_pinmask.d1;
                } else if king_file - file == rank - king_rank {
                    our_movemask &= our_pinmask.d2;
                } else {
                    panic!(
                        "pin? KR {} KF {} R {} F {}",
                        king_rank, king_file, rank, file
                    );
                }
            }
            let mut bb_moves = (nort_attacks(1u64 << current_piece, self.empty)
                | noea_attacks(1u64 << current_piece, self.empty)
                | east_attacks(1u64 << current_piece, self.empty)
                | soea_attacks(1u64 << current_piece, self.empty)
                | sout_attacks(1u64 << current_piece, self.empty)
                | sowe_attacks(1u64 << current_piece, self.empty)
                | west_attacks(1u64 << current_piece, self.empty)
                | nowe_attacks(1u64 << current_piece, self.empty))
                & !our_pieces
                & our_movemask;
            while bb_moves > 0 {
                let bb_move = bb_moves.trailing_zeros();
                moves.push(Move {
                    from: current_piece as usize,
                    to: bb_move as usize,
                    piece_color: color,
                    promoted_piece: None,
                });
                bb_moves &= bb_moves - 1;
            }
            queen &= queen - 1;
        }
        moves
    }
    fn pawn_moves(&self, color: Color) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        match color {
            Color::White => {
                let mut pawn = self.pieces.w_pawn;
                while pawn > 0 {
                    let mut movemask = self.masks.white_checkmask;
                    let current_piece = pawn.trailing_zeros() as u64;
                    if self.masks.white_pinned & (1u64 << current_piece) > 0 {
                        let king_rank = (self.pieces.w_king.trailing_zeros() / 8) as i8;
                        let king_file = (self.pieces.w_king.trailing_zeros() % 8) as i8;
                        let rank = (current_piece / 8) as i8;
                        let file = (current_piece % 8) as i8;
                        // println!("KR {} KF {} R {} F {}", king_rank, king_file, rank, file);
                        if king_rank == rank {
                            movemask &= self.masks.white_pinmask.h;
                        } else if king_file == file {
                            movemask &= self.masks.white_pinmask.v;
                        } else if king_rank - rank == king_file - file {
                            movemask &= self.masks.white_pinmask.d1;
                        } else if rank - king_rank == king_file - file {
                            movemask &= self.masks.white_pinmask.d2;
                        } else {
                            panic!("pin?")
                        }
                    }
                    let mut bb_moves =
                        self.white_pawn_lookup[current_piece as usize] & self.pieces.black_pieces;
                    if (1u64 << current_piece) << 8 & self.empty > 0 {
                        bb_moves |= (1u64 << current_piece) << 8;
                        if (1u64 << current_piece) << 16 & self.empty > 0 && current_piece / 8 == 1
                        {
                            bb_moves |= (1u64 << current_piece) << 16;
                        }
                    }
                    if let Some(sq) = self.en_passant {
                        if self.white_pawn_lookup[current_piece as usize] & (1u64 << sq) > 0 {
                            let potential_white_checkers =
                                HORIZONTAL_LOOKUP[4] & (self.pieces.b_queen | self.pieces.b_rook);
                            let empty = self.empty | 1u64 << current_piece | 1u64 << (sq - 8);
                            let white_checkers = (east_attacks(self.pieces.w_king, empty)
                                | west_attacks(self.pieces.w_king, empty))
                                & potential_white_checkers;
                            if white_checkers == 0 {
                                bb_moves |= 1u64 << sq
                            }
                        }
                        if (self.masks.white_checkers & self.pieces.b_pawn).count_ones() == 1
                            && ((self.masks.white_checkers & self.pieces.b_pawn).trailing_zeros()
                                + 8) as usize
                                == sq
                        {
                            bb_moves |= 1u64 << sq;
                            movemask |= 1u64 << sq;
                        }
                    }
                    bb_moves &= movemask;
                    while bb_moves > 0 {
                        let bb_move = bb_moves.trailing_zeros();
                        if bb_move / 8 == 7 {
                            moves.push(Move {
                                from: current_piece as usize,
                                to: bb_move as usize,
                                piece_color: color,
                                promoted_piece: Some(PromotionType::Queen),
                            });
                            moves.push(Move {
                                from: current_piece as usize,
                                to: bb_move as usize,
                                piece_color: color,
                                promoted_piece: Some(PromotionType::Rook),
                            });
                            moves.push(Move {
                                from: current_piece as usize,
                                to: bb_move as usize,
                                piece_color: color,
                                promoted_piece: Some(PromotionType::Bishop),
                            });
                            moves.push(Move {
                                from: current_piece as usize,
                                to: bb_move as usize,
                                piece_color: color,
                                promoted_piece: Some(PromotionType::Knight),
                            });
                        } else {
                            moves.push(Move {
                                from: current_piece as usize,
                                to: bb_move as usize,
                                piece_color: color,
                                promoted_piece: None,
                            });
                        }
                        bb_moves &= bb_moves - 1;
                    }
                    pawn &= pawn - 1;
                }
            }
            Color::Black => {
                let mut pawn = self.pieces.b_pawn;
                while pawn > 0 {
                    let mut movemask = self.masks.black_checkmask;
                    let current_piece = pawn.trailing_zeros() as u64;
                    if self.masks.black_pinned & (1u64 << current_piece) > 0 {
                        let king_rank = (self.pieces.b_king.trailing_zeros() / 8) as i8;
                        let king_file = (self.pieces.b_king.trailing_zeros() % 8) as i8;
                        let rank = (current_piece / 8) as i8;
                        let file = (current_piece % 8) as i8;
                        if king_rank == rank {
                            movemask &= self.masks.black_pinmask.h;
                        } else if king_file == file {
                            movemask &= self.masks.black_pinmask.v;
                        } else if king_rank - rank == king_file - file {
                            movemask &= self.masks.black_pinmask.d1;
                        } else if rank - king_rank == king_file - file {
                            movemask &= self.masks.black_pinmask.d2;
                        } else {
                            panic!("pin?")
                        }
                    }
                    let mut bb_moves =
                        self.black_pawn_lookup[current_piece as usize] & self.pieces.white_pieces;
                    if (1u64 << current_piece) >> 8 & self.empty > 0 {
                        bb_moves |= (1u64 << current_piece) >> 8;
                        if (1u64 << current_piece) >> 16 & self.empty > 0 && current_piece / 8 == 6
                        {
                            bb_moves |= (1u64 << current_piece) >> 16;
                        }
                    }
                    if let Some(sq) = self.en_passant {
                        if self.black_pawn_lookup[current_piece as usize] & (1u64 << sq) > 0 {
                            let potential_black_checkers =
                                HORIZONTAL_LOOKUP[3] & (self.pieces.w_queen | self.pieces.w_rook);
                            let empty = self.empty | 1u64 << current_piece | 1u64 << (sq + 8);
                            let black_checkers = (east_attacks(self.pieces.b_king, empty)
                                | west_attacks(self.pieces.b_king, empty))
                                & potential_black_checkers;
                            if black_checkers == 0 {
                                bb_moves |= 1u64 << sq
                            }
                        }
                        if (self.masks.black_checkers & self.pieces.w_pawn).count_ones() == 1
                            && ((self.masks.black_checkers & self.pieces.w_pawn).trailing_zeros()
                                - 8) as usize
                                == sq
                        {
                            bb_moves |= 1u64 << sq;
                            movemask |= 1u64 << sq;
                        }
                    }
                    bb_moves &= movemask;
                    while bb_moves > 0 {
                        let bb_move = bb_moves.trailing_zeros();
                        if bb_move / 8 == 1 {
                            moves.push(Move {
                                from: current_piece as usize,
                                to: bb_move as usize,
                                piece_color: color,
                                promoted_piece: Some(PromotionType::Queen),
                            });
                            moves.push(Move {
                                from: current_piece as usize,
                                to: bb_move as usize,
                                piece_color: color,
                                promoted_piece: Some(PromotionType::Rook),
                            });
                            moves.push(Move {
                                from: current_piece as usize,
                                to: bb_move as usize,
                                piece_color: color,
                                promoted_piece: Some(PromotionType::Bishop),
                            });
                            moves.push(Move {
                                from: current_piece as usize,
                                to: bb_move as usize,
                                piece_color: color,
                                promoted_piece: Some(PromotionType::Knight),
                            });
                        } else {
                            moves.push(Move {
                                from: current_piece as usize,
                                to: bb_move as usize,
                                piece_color: color,
                                promoted_piece: None,
                            });
                        }
                        bb_moves &= bb_moves - 1;
                    }
                    pawn &= pawn - 1;
                }
            }
        }
        moves
    }
    fn moves(&self, color: Color) -> Vec<Move> {
        let move_list: Vec<Move>;
        let checkers = match color {
            Color::White => self.masks.white_checkers,
            Color::Black => self.masks.black_checkers,
        };
        if checkers.count_ones() > 1 {
            move_list = self.king_moves(color);
        } else {
            let mut king_moves = self.king_moves(color);
            let mut queen_moves = self.queen_moves(color);
            let mut rook_moves = self.rook_moves(color);
            let mut bishop_moves = self.bishop_moves(color);
            let mut knight_moves = self.knight_moves(color);
            let mut pawn_moves = self.pawn_moves(color);
            king_moves.append(&mut queen_moves);
            king_moves.append(&mut rook_moves);
            king_moves.append(&mut bishop_moves);
            king_moves.append(&mut knight_moves);
            king_moves.append(&mut pawn_moves);
            move_list = king_moves;
        }
        move_list
    }
    // fn fen(&self) -> String {
    //     let mut fen_string = String::new();
    //     for rank in 0..8 {
    //         let mut open_file_count = 0;
    //         for file in 0..8 {
    //             let piece = self.pieces.piece_type_lookup[rank * 8 + file];
    //             let color = self.pieces.color_lookup[rank * 8 + file];
    //             if piece == None {
    //                 open_file_count += 1;
    //             } else {
    //                 let mut piece_str = match piece.unwrap() {
    //                     PieceType::King => "k",
    //                     PieceType::Queen => "q",
    //                     PieceType::Rook => "r",
    //                     PieceType::Bishop => "b",
    //                     PieceType::Knight => "n",
    //                     PieceType::Pawn => "p",
    //                 };
    //                 if color == Some(Color::White) {
    //                     piece_str = piece_str.to_uppercase().as_str();
    //                 }
    //                 fen_string.push_str(piece_str);
    //             }
    //         }
    //         if rank != 7 {
    //             fen_string.push_str("/");
    //             open_file_count = 0;
    //         }
    //     }
    //     // for index in 0..64 {
    //     //     let piece = self.pieces.piece_type_lookup[index];
    //     //     let color = self.pieces.color_lookup[index];

    //     // }
    //     todo!()
    // }
    fn negamax(&self, depth: usize, color: Color) -> (Move, f32) {
        let mut best_score = f32::NEG_INFINITY;
        let mut best_move = self.moves(color)[0];
        if depth == 0 {
            return (best_move, self.static_eval());
        }
        for piece_move in self.moves(color) {
            let new_gamestate = self.apply_move(piece_move);
            let (_, score) = new_gamestate.negamax(depth - 1, new_gamestate.active_color.invert());
            if -score > best_score {
                best_score = -score;
                best_move = piece_move;
            }
        }
        (best_move, best_score)
    }
    fn static_eval(&self) -> f32 {
        let score: f32;
        let king_diff = (self.pieces.w_king.count_ones() - self.pieces.b_king.count_ones()) as f32;
        let queen_diff =
            (self.pieces.w_queen.count_ones() - self.pieces.b_queen.count_ones()) as f32;
        let rook_diff = (self.pieces.w_rook.count_ones() - self.pieces.b_rook.count_ones()) as f32;
        let bishop_diff =
            (self.pieces.w_bishop.count_ones() - self.pieces.b_bishop.count_ones()) as f32;
        let knight_diff =
            (self.pieces.w_knight.count_ones() - self.pieces.b_knight.count_ones()) as f32;
        let pawn_diff = (self.pieces.w_pawn.count_ones() - self.pieces.b_pawn.count_ones()) as f32;
        let side = match self.active_color {
            Color::White => 1.0,
            Color::Black => -1.0,
        };
        score = 69420.0 * king_diff
            + 9.0 * queen_diff
            + 5.0 * rook_diff
            + 3.0 * bishop_diff
            + 3.0 * knight_diff
            + pawn_diff;
        score * side
    }
}

pub fn to_12x10(index: isize) -> isize {
    index + 21 + 2 * (index / 8)
}
pub fn to_8x8(index: isize) -> isize {
    (index - 21) - 2 * ((index - 21) / 10)
}
pub fn verify_index(index: isize) -> bool {
    if index < 21 || index > 119 {
        return false;
    }
    (index - 21) % 10 < 8
}
pub fn piece_lookup(piece_index: usize, piece_type: PieceType, piece_color: Option<Color>) -> u64 {
    let mut bitboard = 0u64;
    match piece_type {
        PieceType::King => {
            for piece_move in [-11, -10, -9, -1, 1, 9, 10, 11] {
                let new = to_12x10(piece_index as isize) + piece_move;
                if verify_index(new) && to_8x8(new) < 64 {
                    bitboard |= 1 << to_8x8(new);
                }
            }
        }
        PieceType::Queen => {
            'queen: for piece_move in [-11, -10, -9, -1, 1, 9, 10, 11] {
                for multiplier in 1..8 {
                    let new = to_12x10(piece_index as isize) + piece_move * multiplier;
                    if verify_index(new) && to_8x8(new) < 64 {
                        bitboard |= 1 << to_8x8(new);
                    } else {
                        continue 'queen;
                    }
                }
            }
        }
        PieceType::Rook => {
            'rook: for piece_move in [-10, -1, 1, 10] {
                for multiplier in 1..8 {
                    let new = to_12x10(piece_index as isize) + piece_move * multiplier;
                    if verify_index(new) && to_8x8(new) < 64 {
                        bitboard |= 1 << to_8x8(new);
                    } else {
                        continue 'rook;
                    }
                }
            }
        }
        PieceType::Bishop => {
            'bishop: for piece_move in [-11, -9, 9, 11] {
                for multiplier in 1..8 {
                    let new = to_12x10(piece_index as isize) + piece_move * multiplier;
                    if verify_index(new) && to_8x8(new) < 64 {
                        bitboard |= 1 << to_8x8(new);
                    } else {
                        continue 'bishop;
                    }
                }
            }
        }
        PieceType::Knight => {
            for piece_move in [-21, -19, -12, -8, 8, 12, 19, 21] {
                let new = to_12x10(piece_index as isize) + piece_move;
                if verify_index(new) && to_8x8(new) < 64 {
                    bitboard |= 1 << to_8x8(new);
                }
            }
        }
        PieceType::Pawn => match piece_color {
            Some(Color::White) => {
                for piece_move in [11, 9] {
                    let new = to_12x10(piece_index as isize) + piece_move;
                    if verify_index(new) && to_8x8(new) < 64 {
                        bitboard |= 1 << to_8x8(new);
                    }
                }
            }
            Some(Color::Black) => {
                for piece_move in [-11, -9] {
                    let new = to_12x10(piece_index as isize) + piece_move;
                    if verify_index(new) && to_8x8(new) < 64 {
                        bitboard |= 1 << to_8x8(new);
                    }
                }
            }
            None => panic!("must provide pawn color"),
        },
    };
    return bitboard;
}
pub fn line_attacks(occ: u8, sldr: u8) -> u8 {
    (occ - 2 * sldr) ^ (occ.reverse_bits() - 2 * sldr.reverse_bits()).reverse_bits()
}
pub fn generate_slide_lookup(key: u64) -> u64 {
    let mut bitboard = key;
    let index1 = key.trailing_zeros() as i32;
    let rank1 = index1 / 8;
    let file1 = index1 % 8;
    let key2 = key & key - 1;
    let index2 = key2.trailing_zeros() as i32;
    let rank2 = index2 / 8;
    let file2 = index2 % 8;
    if rank1 - rank2 == file2 - file1 {
        // println!("a1h8");
        //file1 > file2
        for file in file2..file1 {
            // bitboard |= 1 << 4;
            // let rank = 8 - (file - file2 + rank1);
            bitboard |= 1 << (index1 + (file - file2) * 7);
            // let index = 8 * file + file;
            // println!("{}", index);
        }
    } else if rank1 - rank2 == file1 - file2 {
        // println!("h1a8");
        for file in file1..file2 {
            bitboard |= 1 << (index1 + (file - file1) * 9);
        }
    } else if rank1 == rank2 {
        //Horizontal
        // println!("horizontal");
        for file in file1..file2 {
            // println!("{}", index1 + (file - file1));
            bitboard |= 1 << (index1 + file - file1);
        }
    } else if file1 == file2 {
        //Vertical
        // println!("vertical");
        for rank in rank1..rank2 {
            // println!("{}", index1 + (rank - rank1) * 8);
            bitboard |= 1 << (index1 + (rank - rank1) * 8);
        }
    } else {
        panic!("invalid direction");
    }
    bitboard
}
fn main() {
    let fen = "8/8/8/2k1q1Q1/8/8/2K5/8 w - - 0 1".to_owned();
    let game: GameState = GameState::new(fen);
    let now = SystemTime::now();
    // let moves = game.moves(game.active_color);
    // let color = game.active_color;
    // let king_moves = game.king_moves(color);
    // let new_game = game.apply_move(king_moves[0]);
    // println!(
    //     "KING {} QUEEN {} ROOK {} BISHOP {} KNIGHT {} PAWN {:?}",
    //     game.king_moves(color).len(),
    //     game.queen_moves(color).len(),
    //     game.rook_moves(color).len(),
    //     game.bishop_moves(color).len(),
    //     game.knight_moves(color).len(),
    //     game.pawn_moves(color).len()
    // );
    // let nodes = game.perft(4);
    // println!("Node Count: {}", nodes);
    let (best_move, score) = game.negamax(3, game.active_color);
    println!("Score: {}, Best Move: {:?}", score, best_move);
    let since = now.elapsed().expect("wtf").as_millis() as u64;
    println!("Time taken: {:?}ms", since);
    // println!("Nodes per Second: {}", nodes / since * 1000);
}
//notes
//undefended_pieces = white_pieces - (white_space & white_pieces)
//award +-0.5 for the bishop pair

//todo
//fix movegen bugs
//fix negamax not working
//regenerate masks  each time a move is made
