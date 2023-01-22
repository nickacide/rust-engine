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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
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
#[derive(Debug)]
pub struct Move {
    from: usize,
    to: usize,
    promoted_piece: Option<PieceType>,
    piece_color: Color,
    // castling_square: Option<usize>,
}
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
}
#[derive(Clone, Copy)]
pub struct PinMask {
    h: u64,
    v: u64,
    d1: u64,
    d2: u64,
}
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
    // fn all_moves(self, color: Color) {
    //     match color {
    //         Color::White => {
    //             for idx in self.w_idx {
    //                 let piece = self.piece_types[idx].expect("Expected piece but none found");
    //                 match piece {
    //                     PieceType::Knight => {
    //                         let moves = self.knight_moves[idx] & !self.white_pieces;
    //                     }
    //                     _ => todo!(),
    //                 }
    //             }
    //         }
    //         Color::Black => {}
    //     }
    // }
    // fn pseudo_legal_moves(self) {}
    // fn apply_move(self) {
    //     //does not check if move is legal since the legal moves have already been generated
    // }
}
struct GameState {
    pieces: Pieces,
    empty: u64,
    white_castling: (bool, bool), //Queenside, Kingside
    black_castling: (bool, bool),
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
    // slide_lookup: HashMap<u64, u64>,
    // horizontal_lookup: Vec<u64>,
    // vertical_lookup: Vec<u64>,
    // a1h8_lookup: Vec<u64>,
    // h1a8_lookup: Vec<u64>,
    // we_lookup: Vec<u64>,
    // nw_lookup: Vec<u64>,
    // no_lookup: Vec<u64>,
    // ne_lookup: Vec<u64>,
    // ea_lookup: Vec<u64>,
    // se_lookup: Vec<u64>,
    // so_lookup: Vec<u64>,
    // sw_lookup: Vec<u64>,
    white_pawn_lookup: Vec<u64>, // captures
    black_pawn_lookup: Vec<u64>,
}

//Notes:
/*
any move must have flags to indicate additional information such as the type of piece during promotion.
*/
impl GameState {
    fn new(fen: String) -> GameState {
        let parsed: Vec<&str> = fen.split(" ").collect();
        let position = parsed[0].to_owned();
        let active_color = match parsed[1].to_lowercase().as_str() {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Invalid color"),
        };
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
                let rank = chars.next().expect("Invalid en passant square") as usize - 48; // '1'
                Some(file + 64 - 8 * rank)
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
        let mut white_pinmask = 0u64;
        let mut black_pinmask = 0u64;

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

        // println!("White pins: {}, Black pins: {}", white_pinned, black_pinned);
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
            | (king_lookup[w_king_idx] & pieces.b_king);
        black_checkers |= (knight_lookup[b_king_idx] & pieces.w_knight)
            | (king_lookup[b_king_idx] & pieces.w_king);

        white_checkmask |= knight_lookup[w_king_idx] & pieces.b_knight;
        black_checkmask |= knight_lookup[b_king_idx] & pieces.w_knight;
        white_checkmask &= !pieces.w_king;
        black_checkmask &= !pieces.b_king;

        println!(
            "Checkers: White: {}, Black: {}",
            white_checkers, black_checkers
        );

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

        // white_pinmask |=
        //     vision(pieces.w_king, empty | white_pinned) & !pieces.white_pieces & !white_checkmask;
        // black_pinmask |=
        //     vision(pieces.b_king, empty | black_pinned) & !pieces.black_pieces & !black_checkmask;

        if white_checkmask == 0 {
            white_checkmask = ALL_BITS;
        }
        if black_checkmask == 0 {
            black_checkmask = ALL_BITS;
        }

        white_checkmask &= !pieces.white_pieces;
        black_checkmask &= !pieces.black_pieces;

        // if white_pinmask == 0 {
        //     white_pinmask = ALL_BITS;
        // }
        // if black_pinmask == 0 {
        //     black_pinmask = ALL_BITS;
        // }

        // println!(
        //     "White pinmask: {}, Black pinmask: {}",
        //     white_pinmask, black_pinmask
        // );
        println!(
            "Checkmask: White: {}, Black: {}",
            white_checkmask, black_checkmask
        );
        // println!("{}", pieces.w_rook | pieces.w_queen);
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

        white_space |= king_lookup[w_king_idx];
        black_space |= king_lookup[b_king_idx];

        let mut pawns_copy = pieces.w_pawn;
        while pawns_copy > 0 {
            white_space |= white_pawn_lookup[pawns_copy.trailing_zeros() as usize];
            pawns_copy &= pawns_copy - 1;
        }
        pawns_copy = pieces.b_pawn;
        while pawns_copy > 0 {
            black_space |= black_pawn_lookup[pawns_copy.trailing_zeros() as usize];
            pawns_copy &= pawns_copy - 1;
        }

        let mut knights_copy = pieces.w_knight;
        while knights_copy > 0 {
            white_space |= knight_lookup[knights_copy.trailing_zeros() as usize];
            knights_copy &= knights_copy - 1;
        }
        knights_copy = pieces.b_knight;
        while knights_copy > 0 {
            black_space |= knight_lookup[knights_copy.trailing_zeros() as usize];
            knights_copy &= knights_copy - 1;
        }

        println!("White space: {}", white_space);

        GameState {
            pieces,
            empty,
            white_castling,
            black_castling,
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
            },
        }
    }
    fn default() -> GameState {
        GameState::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_owned())
    }
    fn king_moves(&self, color: Color) -> u64 {
        match color {
            Color::White => {
                self.king_lookup[self.w_king_idx]
                    & !self.masks.black_space
                    & !self.pieces.white_pieces
            }
            Color::Black => {
                self.king_lookup[self.b_king_idx]
                    & !self.masks.white_space
                    & !self.pieces.black_pieces
            }
        }
    }
    fn knight_moves(&self, color: Color) -> u64 {
        let mut moves: u64 = 0;
        let our_pieces;
        match color {
            Color::White => {
                let mut knights = self.pieces.w_knight;
                our_pieces = self.pieces.white_pieces;
                while knights > 0 {
                    if self.masks.white_pinned & 1u64 << knights.trailing_zeros() > 0 {
                        knights &= knights - 1;
                        continue;
                    }
                    moves |= self.knight_lookup[knights.trailing_zeros() as usize]
                        & self.masks.white_checkmask;
                    knights &= knights - 1;
                }
            }
            Color::Black => {
                let mut knights = self.pieces.b_knight;
                our_pieces = self.pieces.black_pieces;
                while knights > 0 {
                    if self.masks.black_pinned & 1u64 << knights.trailing_zeros() > 0 {
                        knights &= knights - 1;
                        continue;
                    }
                    moves |= self.knight_lookup[knights.trailing_zeros() as usize]
                        & self.masks.black_checkmask;
                    knights &= knights - 1;
                }
            }
        }
        moves & !our_pieces
    }
    //fn does not take double checks into account; that's the job of the main moves fn
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
                    // our_movemask = our_pinmask.d1;
                } else if king_file - file == rank - king_rank {
                    // our_movemask = our_pinmask.d2;
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
                    // our_movemask = our_pinmask.h;
                    bishop &= bishop - 1;
                    continue;
                } else if king_file == file {
                    // our_movemask = our_pinmask.v;
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
            println!("Move mask: {}", our_movemask);
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
                    panic!("pin?")
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
    fn moves(self, color: Color) -> Vec<Move> {
        let move_list: Vec<Move> = vec![];
        match color {
            Color::White => {
                if self.masks.white_checkers.count_ones() > 1 {
                    //generate only king moves
                } else {
                }
            }
            Color::Black => {}
        }
        todo!()
    }
}
// fn all_moves(self) -> Self {
//     todo!()
// }
// fn in_check(self, color: Color) -> bool {
//     match color {
//         Color::White => {
//             let king_index = self.pieces.w_king.trailing_zeros() as usize;
//             ((self.king_lookup[king_index] & self.pieces.b_king)
//                 | (self.queen_lookup[king_index] & self.pieces.b_queen)
//                 | (self.rook_lookup[king_index] & self.pieces.b_rook)
//                 | (self.bishop_lookup[king_index] & self.pieces.b_bishop)
//                 | (self.knight_lookup[king_index] & self.pieces.b_knight)
//                 | (self.white_pawn_lookup[king_index] & self.pieces.b_pawn))
//                 > 0
//         }
//         Color::Black => {
//             let king_index = self.pieces.b_king.trailing_zeros() as usize;
//             ((self.king_lookup[king_index] & self.pieces.w_king)
//                 | (self.queen_lookup[king_index] & self.pieces.w_queen)
//                 | (self.rook_lookup[king_index] & self.pieces.w_rook)
//                 | (self.bishop_lookup[king_index] & self.pieces.w_bishop)
//                 | (self.knight_lookup[king_index] & self.pieces.w_knight)
//                 | (self.black_pawn_lookup[king_index] & self.pieces.w_pawn))
//                 > 0
//         }
//     }
// }
// fn pseudo_legal_moves(
//     &self,
//     piece_type: PieceType,
//     piece_index: usize,
//     piece_color: Color,
// ) -> u64 {
//     let our_pieces: u64;
//     let their_pieces: u64;
//     let pawn_coefficient: i32;
//     let pawn_lookup: u64;
//     let starting_pawn_rank: usize;
//     // let our_pieces = match piece_color {
//     //     Color::White => self.pieces.white_pieces,
//     //     Color::Black => self.pieces.black_pieces,
//     // };
//     // let their_pieces = match piece_color {
//     //     Color::White => self.pieces.black_pieces,
//     //     Color::Black => self.pieces.white_pieces,
//     // };
//     match piece_color {
//         Color::White => {
//             our_pieces = self.pieces.white_pieces;
//             their_pieces = self.pieces.black_pieces;
//             pawn_coefficient = -8;
//             pawn_lookup = self.white_pawn_lookup[piece_index];
//             starting_pawn_rank = 6;
//         }
//         Color::Black => {
//             our_pieces = self.pieces.black_pieces;
//             their_pieces = self.pieces.white_pieces;
//             pawn_coefficient = 8;
//             pawn_lookup = self.black_pawn_lookup[piece_index];
//             starting_pawn_rank = 1;
//         }
//     }
//     let moves = match piece_type {
//         //implement castling
//         PieceType::King => self.king_lookup[piece_index] & !our_pieces,
//         PieceType::Queen => {
//             let mut queen_moves = 0u64;
//             let mut blockers = self.queen_lookup[piece_index] & (their_pieces | our_pieces);
//             let lesser: u64;
//             let mut greater = 0u64;
//             while blockers > 1 << piece_index {
//                 let blocker = 63 - blockers.leading_zeros() as usize;
//                 greater |= 1 << blocker;
//                 set_bit(&mut blockers, blocker, false);
//             }
//             lesser = blockers;
//             let north: u64;
//             let northeast: u64;
//             let east: u64;
//             let southeast: u64;
//             let south: u64;
//             let southwest: u64;
//             let west: u64;
//             let northwest: u64;

//             if self.we_lookup[piece_index] & lesser > 0 {
//                 west = 1 << 63 - (self.we_lookup[piece_index] & lesser).leading_zeros();
//             } else {
//                 queen_moves |= self.we_lookup[piece_index];
//                 west = 0;
//             }
//             if self.nw_lookup[piece_index] & lesser > 0 {
//                 northwest = 1 << 63 - (self.nw_lookup[piece_index] & lesser).leading_zeros();
//             } else {
//                 queen_moves |= self.nw_lookup[piece_index];
//                 northwest = 0;
//             }
//             if self.no_lookup[piece_index] & lesser > 0 {
//                 north = 1 << 63 - (self.no_lookup[piece_index] & lesser).leading_zeros();
//             } else {
//                 queen_moves |= self.no_lookup[piece_index];
//                 north = 0;
//             }
//             if self.ne_lookup[piece_index] & lesser > 0 {
//                 northeast = 1 << 63 - (self.ne_lookup[piece_index] & lesser).leading_zeros();
//             } else {
//                 queen_moves |= self.ne_lookup[piece_index];
//                 northeast = 0;
//             }

//             if self.ea_lookup[piece_index] & greater > 0 {
//                 east = 1 << (self.ea_lookup[piece_index] & greater).trailing_zeros();
//             } else {
//                 queen_moves |= self.ea_lookup[piece_index];
//                 east = 0;
//             }
//             if self.se_lookup[piece_index] & greater > 0 {
//                 southeast = 1 << (self.se_lookup[piece_index] & greater).trailing_zeros();
//             } else {
//                 queen_moves |= self.se_lookup[piece_index];
//                 southeast = 0;
//             }
//             if self.so_lookup[piece_index] & greater > 0 {
//                 south = 1 << (self.so_lookup[piece_index] & greater).trailing_zeros();
//             } else {
//                 queen_moves |= self.so_lookup[piece_index];
//                 south = 0;
//             }
//             if self.sw_lookup[piece_index] & greater > 0 {
//                 southwest = 1 << (self.sw_lookup[piece_index] & greater).trailing_zeros();
//             } else {
//                 queen_moves |= self.sw_lookup[piece_index];
//                 southwest = 0;
//             }
//             let mut parsed_blockers: u64 =
//                 west | northwest | north | northeast | east | southeast | south | southwest;
//             while parsed_blockers > 0 {
//                 let blocker_index = parsed_blockers.trailing_zeros() as usize;
//                 let key: u64 = 1 << piece_index | 1 << blocker_index;
//                 queen_moves |= self.slide_lookup.get(&key).unwrap();
//                 parsed_blockers &= parsed_blockers - 1;
//             }
//             queen_moves & !(our_pieces) & !(1 << piece_index)
//         }
//         PieceType::Rook => {
//             let mut rook_moves = 0u64;
//             let mut blockers = self.rook_lookup[piece_index] & (their_pieces | our_pieces);
//             let lesser: u64;
//             let mut greater = 0u64;
//             while blockers > 1 << piece_index {
//                 let blocker = 63 - blockers.leading_zeros() as usize;
//                 greater |= 1 << blocker;
//                 set_bit(&mut blockers, blocker, false);
//             }
//             lesser = blockers;
//             let north: u64;
//             let east: u64;
//             let south: u64;
//             let west: u64;

//             if self.we_lookup[piece_index] & lesser > 0 {
//                 west = 1 << 63 - (self.we_lookup[piece_index] & lesser).leading_zeros();
//             } else {
//                 rook_moves |= self.we_lookup[piece_index];
//                 west = 0;
//             }
//             if self.no_lookup[piece_index] & lesser > 0 {
//                 north = 1 << 63 - (self.no_lookup[piece_index] & lesser).leading_zeros();
//             } else {
//                 rook_moves |= self.no_lookup[piece_index];
//                 north = 0;
//             }
//             if self.ea_lookup[piece_index] & greater > 0 {
//                 east = 1 << (self.ea_lookup[piece_index] & greater).trailing_zeros();
//             } else {
//                 rook_moves |= self.ea_lookup[piece_index];
//                 east = 0;
//             }
//             if self.so_lookup[piece_index] & greater > 0 {
//                 south = 1 << (self.so_lookup[piece_index] & greater).trailing_zeros();
//             } else {
//                 rook_moves |= self.so_lookup[piece_index];
//                 south = 0;
//             }
//             let mut parsed_blockers: u64 = north | east | south | west;
//             while parsed_blockers > 0 {
//                 let blocker_index = parsed_blockers.trailing_zeros() as usize;
//                 let key: u64 = 1 << piece_index | 1 << blocker_index;
//                 rook_moves |= self.slide_lookup.get(&key).unwrap();
//                 parsed_blockers &= parsed_blockers - 1;
//             }
//             rook_moves & !(our_pieces) & !(1 << piece_index)
//         }
//         PieceType::Bishop => {
//             let mut bishop_moves = 0u64;
//             let mut blockers = self.bishop_lookup[piece_index] & (their_pieces | our_pieces);
//             let lesser: u64;
//             let mut greater = 0u64;
//             while blockers > 1 << piece_index {
//                 let blocker = 63 - blockers.leading_zeros() as usize;
//                 greater |= 1 << blocker;
//                 set_bit(&mut blockers, blocker, false);
//             }
//             lesser = blockers;
//             let northeast: u64;
//             let southeast: u64;
//             let southwest: u64;
//             let northwest: u64;

//             if self.nw_lookup[piece_index] & lesser > 0 {
//                 northwest = 1 << 63 - (self.nw_lookup[piece_index] & lesser).leading_zeros();
//             } else {
//                 bishop_moves |= self.nw_lookup[piece_index];
//                 northwest = 0;
//             }
//             if self.ne_lookup[piece_index] & lesser > 0 {
//                 northeast = 1 << 63 - (self.ne_lookup[piece_index] & lesser).leading_zeros();
//             } else {
//                 bishop_moves |= self.ne_lookup[piece_index];
//                 northeast = 0;
//             }
//             if self.se_lookup[piece_index] & greater > 0 {
//                 southeast = 1 << (self.se_lookup[piece_index] & greater).trailing_zeros();
//             } else {
//                 bishop_moves |= self.se_lookup[piece_index];
//                 southeast = 0;
//             }
//             if self.sw_lookup[piece_index] & greater > 0 {
//                 southwest = 1 << (self.sw_lookup[piece_index] & greater).trailing_zeros();
//             } else {
//                 bishop_moves |= self.sw_lookup[piece_index];
//                 southwest = 0;
//             }
//             let mut parsed_blockers: u64 = northwest | northeast | southeast | southwest;
//             while parsed_blockers > 0 {
//                 let blocker_index = parsed_blockers.trailing_zeros() as usize;
//                 let key: u64 = 1 << piece_index | 1 << blocker_index;
//                 bishop_moves |= self.slide_lookup.get(&key).unwrap();
//                 parsed_blockers &= parsed_blockers - 1;
//             }
//             bishop_moves & !(our_pieces) & !(1 << piece_index)
//         }
//         PieceType::Knight => self.knight_lookup[piece_index] & !our_pieces,
//         PieceType::Pawn => {
//             let mut pawn_captures: u64 = 0;
//             let mut pawn_pushes: u64 = 0;
//             // let mut blockers: u64 = (1 << (piece_index as i32 + pawn_coefficient)
//             //     | 1 << (piece_index as i32 + 2 * pawn_coefficient))
//             //     & (our_pieces | their_pieces);
//             let single_blocker: u64 =
//                 1 << (piece_index as i32 + 1 * pawn_coefficient) & (our_pieces | their_pieces);
//             let double_blocker: u64 =
//                 1 << (piece_index as i32 + 2 * pawn_coefficient) & (our_pieces | their_pieces);

//             pawn_captures |= pawn_lookup & their_pieces;
//             // println!("s {} d {}", single_blocker, double_blocker);
//             // println!("{}", pawn_captures);
//             if let Some(square) = self.en_passant {
//                 pawn_captures |= 1 << square;
//             }
//             // println!("{}", pawn_captures);
//             pawn_pushes |= !single_blocker & (1 << piece_index as i32 + 1 * pawn_coefficient);
//             if starting_pawn_rank == piece_index / 8 && single_blocker == 0 {
//                 pawn_pushes |=
//                     !double_blocker & (1 << piece_index as i32 + 2 * pawn_coefficient);
//             }
//             pawn_captures | pawn_pushes & !(1 << piece_index)
//         }
//     };
//     moves
// }
//generates the moves for a given piece
// fn all_moves(
//     &self,
//     piece_index: usize,
//     piece_type: PieceType,
//     piece_color: Color,
// ) -> Vec<Move> {
//     let mut king_danger_squares = 0u64;
//     let their_color = match piece_color {
//         Color::White => Color::Black,
//         Color::Black => Color::White,
//     };
//     for square in 0..64 {
//         if let Some(piece_type) = self.pieces.piece_type_lookup[square] {
//             if Some(their_color) == self.pieces.color_lookup[square] {
//                 king_danger_squares |= self.pseudo_legal_moves(piece_type, square, their_color);
//             }
//         }
//     }
//     println!("{}", king_danger_squares);
//     return vec![Move {
//         from: 0,
//         to: 0,
//         castling_square: None,
//         piece_color: Color::White,
//         promoted_piece: None,
//     }];
//     // todo!()
// }
// fn piece_moves(&self, piece_type: PieceType, piece_index: usize, piece_color: Color) -> u64 {

//     match piece_type {
//         PieceType::King => self.king_lookup[index],
//         PieceType::Rook => self.rook_lookup[index],
//         PieceType::Bishop => self.bishop_lookup[index],
//         PieceType::Knight => self.knight_lookup[index],
//         _ => 0u64,
//     }
// }
// fn every_piece(&self, color: Color) {
//     // let mut pieces = match color {
//     //     Color::White => self.pieces.white_pieces,
//     //     Color::Black => self.pieces.black_pieces,
//     // };
//     // while pieces != 0 {
//     //     let idx = 63 - pieces.trailing_zeros() as usize;
//     //     let piece_type = self.pieces.piece_types[idx].expect("Invalid piece");
//     //     let moves = self.piece_moves(piece_type, idx) & !pieces;
//     //     // println!("Moves for {:?}: {:?}", piece_type, moves);
//     //     pieces &= pieces - 1;
//     // }
// }
// }
//index must be either the biggest or smallest
// pub fn closest(index: usize, a: usize, b: usize) -> usize {
//     if index < min(a, b) {
//         return min(a, b);
//     } else if index > std::cmp::max(a, b) {
//         return std::cmp::max(a, b);
//     }
//     panic!("index must either be biggest or smallest")
// }
// pub fn direction(start: usize, end: usize) -> Direction {
//     let start_zeros = start.trailing_zeros() as i32;
//     let end_zeros = end.trailing_zeros() as i32;
//     let rank1 = start_zeros / 8;
//     let file1 = start_zeros % 8;
//     let rank2 = end_zeros / 8;
//     let file2 = end_zeros % 8;
//     if rank1 - rank2 == file2 - file1 {
//         if start > end {
//             Direction::NorthEast
//         } else {
//             Direction::SouthWest
//         }
//     } else if rank1 - rank2 == file1 - file2 {
//         if start > end {
//             Direction::NorthWest
//         } else {
//             Direction::SouthEast
//         }
//     } else if rank1 == rank2 {
//         if start > end {
//             Direction::West
//         } else {
//             Direction::East
//         }
//     } else if file1 == file2 {
//         if start > end {
//             Direction::South
//         } else {
//             Direction::North
//         }
//     } else {
//         panic!("invalid direction")
//     }
// }
//for lookup
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
// pub fn line_loookup(line_index: usize, line_type: LineType) -> u64 {
//     let mut bitboard = 0u64;
//     match line_type {
//         LineType::Horizontal => {
//             'line: for line_move in [-1, 1] {
//                 for multiplier in 1..8 {
//                     let new = to_12x10(line_index as isize) + line_move * multiplier;
//                     if verify_index(new) && to_8x8(new) < 64 {
//                         bitboard |= 1 << to_8x8(new);
//                     } else {
//                         continue 'line;
//                     }
//                 }
//             }
//         }
//         LineType::Vertical => {
//             'line: for line_move in [-10, 10] {
//                 for multiplier in 1..8 {
//                     let new = to_12x10(line_index as isize) + line_move * multiplier;
//                     if verify_index(new) && to_8x8(new) < 64 {
//                         bitboard |= 1 << to_8x8(new);
//                     } else {
//                         continue 'line;
//                     }
//                 }
//             }
//         }
//         LineType::A1h8 => {
//             'line: for line_move in [-9, 9] {
//                 for multiplier in 1..8 {
//                     let new = to_12x10(line_index as isize) + line_move * multiplier;
//                     if verify_index(new) && to_8x8(new) < 64 {
//                         bitboard |= 1 << to_8x8(new);
//                     } else {
//                         continue 'line;
//                     }
//                 }
//             }
//         }
//         LineType::H1a8 => {
//             'line: for line_move in [-11, 11] {
//                 for multiplier in 1..8 {
//                     let new = to_12x10(line_index as isize) + line_move * multiplier;
//                     if verify_index(new) && to_8x8(new) < 64 {
//                         bitboard |= 1 << to_8x8(new);
//                     } else {
//                         continue 'line;
//                     }
//                 }
//             }
//         }
//     }
//     bitboard
// }
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
// pub fn directional_lookup(index: usize, direction: Direction) -> u64 {
//     let direction_offset = match direction {
//         Direction::West => -1,
//         Direction::NorthWest => -11,
//         Direction::North => -10,
//         Direction::NorthEast => -9,
//         Direction::East => 1,
//         Direction::SouthEast => 11,
//         Direction::South => 10,
//         Direction::SouthWest => 9,
//     };
//     let mut bitboard = 0u64;
//     for multiplier in 1..8 {
//         let new = to_12x10(index as isize) + direction_offset * multiplier;
//         if verify_index(new) && to_8x8(new) < 64 {
//             bitboard |= 1 << to_8x8(new);
//         } else {
//             break;
//         }
//     }
//     bitboard
// }
fn main() {
    let fen = "rn2k1n1/pppppppp/4r3/b7/7q/2P5/PP3Q1P/1N2KB2 w q - 0 1".to_owned();
    let game: GameState = GameState::new(fen);
    let now = SystemTime::now();
    let moves = game.queen_moves(Color::White);
    println!("Moves: {:?}", moves);
    println!("White pieces: {}", game.pieces.white_pieces);
    let since = now.elapsed().expect("wtf").as_micros();
    println!("Time taken: {:?}μs", since);
}
//notes
//undefended_pieces = white_pieces - (white_space & white_pieces)
//award +-0.5 for the bishop pair
