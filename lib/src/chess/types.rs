use crate::bitboards::Bitboard;
use std::ops::{Index, Not};

/// A trait for pieces independent of which side of the board the piece belongs to.
///
/// # Examples
///
/// This is how pieces are defined for the standard variant chess
///

pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy)]
pub enum Piece {
    WPawn,
    BPawn,
    WKnight,
    BKnight,
    WBishop,
    BBishop,
    WRook,
    BRook,
    WQueen,
    BQueen,
    WKing,
    BKing,
    None,
}

pub const PIECE_COUNT: usize = 14;

#[derive(Clone, Copy)]
pub enum ChessSide {
    White,
    Black,
}

pub const SIDE_COUNT: usize = 2;

impl Index<Piece> for [Bitboard; PIECE_COUNT] {
    type Output = Bitboard;

    fn index(&self, piece: Piece) -> &Self::Output {
        match piece {
            Piece::WPawn => &self[0],
            Piece::BPawn => &self[1],
            Piece::WKnight => &self[2],
            Piece::BKnight => &self[3],
            Piece::WBishop => &self[4],
            Piece::BBishop => &self[5],
            Piece::WRook => &self[6],
            Piece::BRook => &self[7],
            Piece::WQueen => &self[8],
            Piece::BQueen => &self[9],
            Piece::WKing => &self[10],
            Piece::BKing => &self[11],
            Piece::None => panic!("Attempted to index array with an empty piece")
        }
    }
}

impl Index<ChessSide> for [Bitboard; SIDE_COUNT] {
    type Output = Bitboard;

    fn index(&self, side: ChessSide) -> &Self::Output {
        match side {
            ChessSide::White => &self[0],
            ChessSide::Black => &self[1],
        }
    }
}

impl Not for ChessSide {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            ChessSide::White => ChessSide::Black,
            ChessSide::Black => ChessSide::White,
        }
    }
}

impl From<u8> for PieceType {
    fn from(n: u8) -> Self {
        match n {
            0 => PieceType::Pawn,
            1 => PieceType::Knight,
            2 => PieceType::Bishop,
            3 => PieceType::Rook,
            4 => PieceType::Queen,
            5 => PieceType::King,
            _ => panic!("Invalid PieceType number"),
        }
    }
}

impl From<u8> for Piece {
    fn from(n: u8) -> Piece {
        match n {
            0 => Piece::WPawn,
            1 => Piece::BPawn,
            2 => Piece::WKnight,
            3 => Piece::BKnight,
            4 => Piece::WBishop,
            5 => Piece::BBishop,
            6 => Piece::WRook,
            7 => Piece::BRook,
            8 => Piece::WQueen,
            9 => Piece::BQueen,
            10 => Piece::WKing,
            11 => Piece::BKing,
            _ => panic!("Invalid Piece number"),
        }
    }
}

impl From<u8> for ChessSide {
    fn from(n: u8) -> Self {
        match n {
            0 => ChessSide::White,
            1 => ChessSide::Black,
            _ => panic!("Invalid ChessSide number"),
        }
    }
}

impl From<(PieceType, ChessSide)> for Piece {
    fn from(p: (PieceType, ChessSide)) -> Self {
        ((p.0 as u8) * 2 + (p.1 as u8)).into()
    }
}

impl From<Piece> for PieceType {
    fn from(p: Piece) -> Self {
        match p {
            Piece::WPawn => PieceType::Pawn,
            Piece::BPawn => PieceType::Pawn,
            Piece::WKnight => PieceType::Knight,
            Piece::BKnight => PieceType::Knight,
            Piece::WBishop => PieceType::Bishop,
            Piece::BBishop => PieceType::Bishop,
            Piece::WRook => PieceType::Rook,
            Piece::BRook => PieceType::Rook,
            Piece::WQueen => PieceType::Queen,
            Piece::BQueen => PieceType::Queen,
            Piece::WKing => PieceType::King,
            Piece::BKing => PieceType::King,
            Piece::None => panic!("Attempted to obtain the piece type of None")
        }
    }
}

impl From<PieceType> for usize {
    fn from(p: PieceType) -> Self {
        match p {
            PieceType::Pawn => 0,
            PieceType::Knight => 1,
            PieceType::Bishop => 2,
            PieceType::Rook => 3,
            PieceType::Queen => 4,
            PieceType::King => 5,
        }
    }
}

impl From<Piece> for ChessSide {
    fn from(p: Piece) -> Self {
        match p {
            Piece::WPawn => ChessSide::White,
            Piece::WKnight => ChessSide::White,
            Piece::WBishop => ChessSide::White,
            Piece::WRook => ChessSide::White,
            Piece::WQueen => ChessSide::White,
            Piece::WKing => ChessSide::White,
            Piece::BPawn => ChessSide::Black,
            Piece::BKnight => ChessSide::Black,
            Piece::BBishop => ChessSide::Black,
            Piece::BRook => ChessSide::Black,
            Piece::BQueen => ChessSide::Black,
            Piece::BKing => ChessSide::Black,
            Piece::None => panic!("Attempted to obtain the chess side of None")
        }
    }
}

impl From<ChessSide> for usize {
    fn from(s: ChessSide) -> Self {
        match s {
            ChessSide::White => 0,
            ChessSide::Black => 1,
        }
    }
}
