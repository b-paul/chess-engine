use super::attacks::*;
use super::{board::ChessBoard, types::*};
use crate::bitboards::Bitboard;
use crate::chess::board::{CastlingRight, Square};
use crate::types::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ChessMove(u16);

impl ChessMove {
    // A chess move contains a from, to, promotion and en passant
    // 12 bits for from and to, 3 for promotion
    //  16th bit (counting from 1) is the en passant flag
    pub fn new(from: u16, to: u16, promotion: u16, en_pas: u16) -> Self {
        ChessMove(from | to << 6 | promotion << 12 | en_pas << 15)
    }
}

pub enum GenType {
    Quiet,
    Noisy,
}
impl MoveGen for ChessBoard {
    type Move = ChessMove;

    fn gen_quiet(&self) -> Vec<ChessMove> {
        let mut mv_list = Vec::new();

        self.gen_pawn_moves(&mut mv_list, GenType::Quiet);
        self.gen_knight_moves(&mut mv_list, GenType::Quiet);
        self.gen_king_moves(&mut mv_list, GenType::Quiet);
        self.gen_slider_moves(&mut mv_list, GenType::Quiet);
        self.gen_castle_moves(&mut mv_list, GenType::Quiet);

        mv_list
    }

    fn gen_noisy(&self) -> Vec<ChessMove> {
        let mut mv_list = Vec::new();

        self.gen_pawn_moves(&mut mv_list, GenType::Noisy);
        self.gen_knight_moves(&mut mv_list, GenType::Noisy);
        self.gen_king_moves(&mut mv_list, GenType::Noisy);
        self.gen_slider_moves(&mut mv_list, GenType::Noisy);
        self.gen_castle_moves(&mut mv_list, GenType::Noisy);

        mv_list
    }
}

fn make_pawn_move(from: u16, to: u16, mv_list: &mut Vec<ChessMove>) {
    let to_rank = Square::from_index(to as u8).rank();
    if to_rank == 0 || to_rank == 7 {
        mv_list.push(ChessMove::new(from, to, 1, 0));
        mv_list.push(ChessMove::new(from, to, 2, 0));
        mv_list.push(ChessMove::new(from, to, 3, 0));
        mv_list.push(ChessMove::new(from, to, 4, 0));
    } else {
        mv_list.push(ChessMove::new(from, to, 0, 0));
    }
}

impl ChessBoard {
    #[inline]
    pub fn gen_pawn_moves(&self, mv_list: &mut Vec<ChessMove>, _gen_type: GenType) {
        // There are a bunch types of pawn moves
        // Singular pushes
        // Double pushes
        // Diagonal captures (in both directions)
        // En passant
        // Promotions for all of those except en passant

        let our_pawns = self.piece_bb[Piece::from((PieceType::Pawn, self.turn))];

        // En passant first, since every other type of move can have promotion
        if let Some(enpas_sq) = &self.en_passant {
            // The en passant square is the square that a pawn would move to when doing an en
            // passant.

            // This code is quite ugly! These comments are also quite ugly!

            let enpas_sq = enpas_sq.index();

            // These shifts represent the bitwise shift from the en passant square to a square
            // where a pawn that could do en passant would be placed.
            let shifts = match self.turn {
                ChessSide::White => [-7, -9],
                ChessSide::Black => [7, 9],
            };
            for shift in shifts {
                let enpas_sq_bb = Bitboard::square(enpas_sq);
                let pawn_sq_bb = enpas_sq_bb.shift1(shift);
                if !(pawn_sq_bb & our_pawns).is_empty() {
                    // :((((( FIX THIS!!! as casts are ugly!!
                    mv_list.push(ChessMove::new(
                        (enpas_sq as i8 + shift) as u16,
                        enpas_sq as u16,
                        0,
                        1,
                    ));
                }
            }
        }

        // Single pushes
        let push_shift = match self.turn {
            ChessSide::White => 8,
            ChessSide::Black => -8,
        };
        let single_pushes = our_pawns.shift1(push_shift) & !self.side_bb[!self.turn];
        let mut single_pushes_iter = single_pushes;
        while !single_pushes_iter.is_empty() {
            let to = single_pushes_iter.poplsb();
            make_pawn_move((to as i8 - push_shift) as u16, to as u16, mv_list);
        }
        // Double pushes
        let third_rank = match self.turn {
            ChessSide::White => 0xFF0000,
            ChessSide::Black => 0xFF00000000,
        };
        let mut double_pushes_iter = (single_pushes & third_rank).shift1(push_shift);
        while !double_pushes_iter.is_empty() {
            let to = double_pushes_iter.poplsb();
            make_pawn_move((to as i8 - 2 * push_shift) as u16, to as u16, mv_list);
        }
        // Captures
        let capture_shifts = match self.turn {
            ChessSide::White => [7, 9],
            ChessSide::Black => [-7, -9],
        };
        for shift in capture_shifts {
            let mut captures_iter = our_pawns.shift1(shift) & self.side_bb[!self.turn];
            while !captures_iter.is_empty() {
                let to = captures_iter.poplsb();
                make_pawn_move((to as i8 - shift) as u16, to as u16, mv_list);
            }
        }
    }

    #[inline]
    pub fn gen_knight_moves(&self, mv_list: &mut Vec<ChessMove>, gen_type: GenType) {
        let target_squares = match gen_type {
            GenType::Quiet => !self.side_bb[self.turn],
            GenType::Noisy => self.side_bb[!self.turn],
        };
        let mut knights = self.piece_bb[Piece::from((PieceType::Knight, self.turn))];

        while !knights.is_empty() {
            let from = knights.poplsb() as u8;

            let mut attacks = target_squares & get_knight_attacks(from);

            while !attacks.is_empty() {
                let to = attacks.poplsb();
                mv_list.push(ChessMove::new(from as u16, to as u16, 0, 0));
            }
        }
    }

    #[inline]
    pub fn gen_king_moves(&self, mv_list: &mut Vec<ChessMove>, gen_type: GenType) {
        let target_squares = match gen_type {
            GenType::Quiet => !self.side_bb[self.turn] ^ self.side_bb[!self.turn],
            GenType::Noisy => self.side_bb[!self.turn],
        };
        let from = self.piece_bb[Piece::from((PieceType::King, self.turn))].lsb() as u8;

        let mut attacks = target_squares & get_king_attacks(from);
        while !attacks.is_empty() {
            let to = attacks.poplsb();
            mv_list.push(ChessMove::new(from as u16, to as u16, 0, 0));
        }
    }

    #[inline]
    pub fn gen_slider_moves(&self, mv_list: &mut Vec<ChessMove>, gen_type: GenType) {
        let target_squares = match gen_type {
            GenType::Quiet => !self.side_bb[self.turn] ^ self.side_bb[!self.turn],
            GenType::Noisy => self.side_bb[!self.turn],
        };
        let occupied_squares = self.side_bb[self.turn] | self.side_bb[!self.turn];

        // abstraction
        let mut bishops = self.piece_bb[Piece::from((PieceType::Bishop, self.turn))];

        while !bishops.is_empty() {
            let from = bishops.poplsb() as u8;

            let mut attacks = target_squares & get_bishop_attacks(from, occupied_squares);
            while !attacks.is_empty() {
                let to = attacks.poplsb();
                mv_list.push(ChessMove::new(from as u16, to as u16, 0, 0));
            }
        }

        let mut rooks = self.piece_bb[Piece::from((PieceType::Rook, self.turn))];
        while !rooks.is_empty() {
            let from = rooks.poplsb() as u8;

            let mut attacks = target_squares & get_rook_attacks(from, occupied_squares);
            while !attacks.is_empty() {
                let to = attacks.poplsb();
                mv_list.push(ChessMove::new(from as u16, to as u16, 0, 0));
            }
        }

        let mut queens = self.piece_bb[Piece::from((PieceType::Queen, self.turn))];
        while !queens.is_empty() {
            let from = queens.poplsb() as u8;

            let mut attacks = target_squares & get_queen_attacks(from, occupied_squares);
            while !attacks.is_empty() {
                let to = attacks.poplsb();
                mv_list.push(ChessMove::new(from as u16, to as u16, 0, 0));
            }
        }
    }

    #[inline]
    pub fn gen_castle_moves(&self, mv_list: &mut Vec<ChessMove>, _gen_type: GenType) {
        let occ = self.occ();

        // Reminder that this is a pseudolegal move generator
        match self.turn {
            ChessSide::White => {
                if self.castling_rights.has_right(CastlingRight::WhiteKing)
                    && (occ & 0b00000110).is_empty()
                {
                    mv_list.push(ChessMove::new(4, 6, 0, 0));
                }
                if self.castling_rights.has_right(CastlingRight::WhiteQueen)
                    && (occ & 0b01110000).is_empty()
                {
                    mv_list.push(ChessMove::new(4, 2, 0, 0));
                }
            }
            ChessSide::Black => {
                if self.castling_rights.has_right(CastlingRight::BlackKing)
                    && (occ & (0b00000110 << 56)).is_empty()
                {
                    mv_list.push(ChessMove::new(60, 62, 0, 0));
                }
                if self.castling_rights.has_right(CastlingRight::BlackQueen)
                    && (occ & (0b01110000 << 56)).is_empty()
                {
                    mv_list.push(ChessMove::new(60, 58, 0, 0));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn knight_moves() {
        let board = ChessBoard::from_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );

        board.print_board();

        let mut move_list = Vec::new();
        board.gen_knight_moves(&mut move_list, GenType::Quiet);
        println!("{:?}", move_list);

        assert_eq!(move_list.len(), 4);
    }

    #[test]
    fn king_moves() {
        let board = ChessBoard::from_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );

        board.print_board();

        let mut move_list = Vec::new();
        board.gen_king_moves(&mut move_list, GenType::Quiet);
        println!("{:?}", move_list);

        assert_eq!(move_list.len(), 0);
    }
}
