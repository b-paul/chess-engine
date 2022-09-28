use super::{movegen::*, types::*};
use crate::{bitboards::*, Board};

// (File, Rank)
#[derive(Copy, Clone)]
pub struct Square(u8, u8);

impl Square {
    pub fn index(&self) -> u8 {
        self.0 + 8 * self.1
    }

    pub fn from_index(idx: u8) -> Self {
        Square(idx & 0b111, (idx >> 3) & 0b111)
    }

    pub fn rank(&self) -> u8 {
        self.1
    }
}

pub enum CastlingRight {
    WhiteKing,
    WhiteQueen,
    BlackKing,
    BlackQueen,
}

pub struct CastlingRights(u8);

impl Default for CastlingRights {
    fn default() -> Self {
        CastlingRights(0)
    }
}

impl CastlingRights {
    pub fn set_right(&mut self, right: CastlingRight) {
        self.0 |= 1
            << match right {
                CastlingRight::WhiteKing => 0,
                CastlingRight::WhiteQueen => 1,
                CastlingRight::BlackKing => 2,
                CastlingRight::BlackQueen => 3,
            }
    }
    pub fn unset_right(&mut self, right: CastlingRight) {
        self.0 &= !(1
            << match right {
                CastlingRight::WhiteKing => 0,
                CastlingRight::WhiteQueen => 1,
                CastlingRight::BlackKing => 2,
                CastlingRight::BlackQueen => 3,
            })
    }
    pub fn has_right(&self, right: CastlingRight) -> bool {
        self.0
            & (1 << match right {
                CastlingRight::WhiteKing => 0,
                CastlingRight::WhiteQueen => 1,
                CastlingRight::BlackKing => 2,
                CastlingRight::BlackQueen => 3,
            })
            != 0
    }
}

pub struct ChessBoard {
    // Array which stores each piece
    pub grid: [Piece; 64],
    // List of bitboards per piece
    // Making it for piece_type might be better for cache efficiency or something
    pub piece_bb: [Bitboard; PIECE_COUNT],
    // Also have two bitboards for all of each player's pieces
    pub side_bb: [Bitboard; SIDE_COUNT],

    // Who's turn it currently is
    pub turn: ChessSide,
    // Castling rights
    pub castling_rights: CastlingRights,
    // En passant
    pub en_passant: Option<Square>,
    // 50 move rule counter
    // Full move count
}

impl Board for ChessBoard {
    type Move = ChessMove;

    fn make_move(&mut self, _mv: ChessMove) {
        unimplemented!()
    }

    fn from_fen(fen: String) -> ChessBoard {
        // TODO switch this to a proper io reader thing maybe ? I feel like there is a better way
        // to do this fen stuff
        let tokens: Vec<&str> = fen.split(' ').collect();

        let mut board = ChessBoard {
            grid: [Piece::None; 64],
            piece_bb: [Bitboard::empty(); PIECE_COUNT],
            side_bb: [Bitboard::empty(); SIDE_COUNT],
            turn: ChessSide::White,
            castling_rights: CastlingRights::default(),
            en_passant: None,
        };

        let mut row = 7;
        for row_str in tokens[0].split('/') {
            let mut file = 0;
            for c in row_str.chars() {
                if file >= 8 {
                    panic!(
                        "Invalid fen! Row {} had more than 8 squares as input!\n{}",
                        row, fen
                    );
                }
                match c {
                    '1' => file += 0,
                    '2' => file += 1,
                    '3' => file += 2,
                    '4' => file += 3,
                    '5' => file += 4,
                    '6' => file += 5,
                    '7' => file += 6,
                    '8' => file += 7,

                    'P' => board.place_piece(Piece::WPawn, row, file),
                    'p' => board.place_piece(Piece::BPawn, row, file),
                    'N' => board.place_piece(Piece::WKnight, row, file),
                    'n' => board.place_piece(Piece::BKnight, row, file),
                    'B' => board.place_piece(Piece::WBishop, row, file),
                    'b' => board.place_piece(Piece::BBishop, row, file),
                    'R' => board.place_piece(Piece::WRook, row, file),
                    'r' => board.place_piece(Piece::BRook, row, file),
                    'Q' => board.place_piece(Piece::WQueen, row, file),
                    'q' => board.place_piece(Piece::BQueen, row, file),
                    'K' => board.place_piece(Piece::WKing, row, file),
                    'k' => board.place_piece(Piece::BKing, row, file),

                    _ => panic!("Invalid character {}\n{}", c, fen),
                }
                file += 1;
            }
            if row > 0 {
                row -= 1;
            }
        }

        match tokens[1] {
            "w" => board.turn = ChessSide::White,
            "b" => board.turn = ChessSide::Black,
            _ => panic!("Invalid side to move {}\n{}", tokens[1], fen),
        }

        // Castling rights
        if tokens[2] != "-" {
            for c in tokens[2].chars() {
                match c {
                    'K' => board.castling_rights.set_right(CastlingRight::WhiteKing),
                    'Q' => board.castling_rights.set_right(CastlingRight::WhiteQueen),
                    'k' => board.castling_rights.set_right(CastlingRight::BlackKing),
                    'q' => board.castling_rights.set_right(CastlingRight::BlackQueen),
                    _ => panic!(
                        "Invalid castling right {} for the castling rights {}",
                        c, tokens[2]
                    ),
                }
            }
        }

        // En passant
        if tokens[3] != "-" {
            assert!(
                tokens[3].len() == 2,
                "Invalid en passant square {}",
                tokens[3]
            );
            let mut chars = tokens[3].chars();
            // TODO Probably should make a function to construct a square from a string
            board.en_passant = Some(Square(
                match chars.next() {
                    Some('a') => 0,
                    Some('b') => 1,
                    Some('c') => 2,
                    Some('d') => 3,
                    Some('e') => 4,
                    Some('f') => 5,
                    Some('g') => 6,
                    Some('h') => 7,
                    _ => panic!("Invalid file for the en passant square {}", tokens[3]),
                },
                match chars.next() {
                    Some('0') => 0,
                    Some('1') => 1,
                    Some('2') => 2,
                    Some('3') => 3,
                    Some('4') => 4,
                    Some('5') => 5,
                    Some('6') => 6,
                    Some('7') => 7,
                    _ => panic!("Invalid rank for the en passant square {}", tokens[3]),
                },
            ));
        }

        // Halfmove clock
        // Full move number

        board
    }
}

impl ChessBoard {
    fn place_piece(&mut self, piece: Piece, row: u8, file: u8) {
        self.grid[sq(row, file) as usize] = piece;
        self.piece_bb[piece as usize].set_sq(row, file);
        self.side_bb[ChessSide::from(piece) as usize].set_sq(row, file);
    }

    pub fn occ(&self) -> Bitboard {
        self.side_bb[0] | self.side_bb[1]
    }
}

fn print_piece(p: Piece) {
    print!(
        "{}",
        match p {
            Piece::WPawn => "P",
            Piece::BPawn => "p",
            Piece::WKnight => "K",
            Piece::BKnight => "k",
            Piece::WBishop => "B",
            Piece::BBishop => "b",
            Piece::WRook => "R",
            Piece::BRook => "r",
            Piece::WQueen => "Q",
            Piece::BQueen => "q",
            Piece::WKing => "K",
            Piece::BKing => "k",
            _ => ".",
        }
    )
}

impl ChessBoard {
    #[allow(dead_code)]
    pub fn print_board(&self) {
        println!("┌─┬─┬─┬─┬─┬─┬─┬─┐");
        for col in 0..8 {
            print!("│");
            print_piece(self.grid[sq(7, col) as usize]);
        }
        println!("│");
        for row in (0..7).rev() {
            println!("├─┼─┼─┼─┼─┼─┼─┼─┤");
            for col in 0..8 {
                print!("│");
                print_piece(self.grid[sq(row, col) as usize]);
            }
            println!("│");
        }
        println!("└─┴─┴─┴─┴─┴─┴─┴─┘");
    }
}
