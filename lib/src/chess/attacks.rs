use crate::bitboards::Bitboard;

#[inline]
pub fn get_knight_attacks(sq: u8) -> Bitboard {
    KNIGHT_ATTACKS[sq as usize].into()
}

#[inline]
pub fn get_king_attacks(sq: u8) -> Bitboard {
    KING_ATTACKS[sq as usize].into()
}

#[inline]
pub fn get_bishop_attacks(sq: u8, occ: Bitboard) -> Bitboard {
    gen_sliding_attack(sq, occ, [-9, -7, 7, 9])
}

#[inline]
pub fn get_rook_attacks(sq: u8, occ: Bitboard) -> Bitboard {
    gen_sliding_attack(sq, occ, [-8, -1, 1, 8])
}

#[inline]
pub fn get_queen_attacks(sq: u8, occ: Bitboard) -> Bitboard {
    get_bishop_attacks(sq, occ) | get_rook_attacks(sq, occ)
}

const KNIGHT_ATTACKS: [u64; 64] = gen_knight_attack_table();
const KING_ATTACKS: [u64; 64] = gen_king_attack_table();

const fn gen_knight_attack_table() -> [u64; 64] {
    let mut attacks = [0; 64];

    let mut sq = 0;
    loop {
        if sq >= 64 {
            break;
        }
        let attack = 1 << sq;

        attacks[sq] |= (attack & !0x0101010101010101) >> 17;
        attacks[sq] |= (attack & !0x8080808080808080) >> 15;
        attacks[sq] |= (attack & !0x0303030303030303) >> 10;
        attacks[sq] |= (attack & !0xC0C0C0C0C0C0C0C0) >> 6;
        attacks[sq] |= (attack & !0x0303030303030303) << 6;
        attacks[sq] |= (attack & !0xC0C0C0C0C0C0C0C0) << 10;
        attacks[sq] |= (attack & !0x0101010101010101) << 15;
        attacks[sq] |= (attack & !0x8080808080808080) << 17;

        sq += 1;
    }

    attacks
}

const fn gen_king_attack_table() -> [u64; 64] {
    let mut attacks = [0; 64];

    let mut sq = 0;
    loop {
        if sq >= 64 {
            break;
        }
        let attack = 1 << sq;

        attacks[sq] |= (attack & !0x0101010101010101) >> 9;
        attacks[sq] |= (attack & !0x8080808080808080) >> 7;
        attacks[sq] |= (attack & !0x0101010101010101) >> 1;
        attacks[sq] |= (attack & !0x8080808080808080) << 1;
        attacks[sq] |= (attack & !0x0101010101010101) << 7;
        attacks[sq] |= (attack & !0x8080808080808080) << 9;
        attacks[sq] |= attack >> 8;
        attacks[sq] |= attack << 8;

        sq += 1;
    }

    attacks
}

fn gen_sliding_attack(sq: u8, _occ: Bitboard, dirs: [i8; 4]) -> Bitboard {
    let mut attack = Bitboard::empty();
    for dir in dirs {
        let mut sqbb = Bitboard::square(sq);
        while !sqbb.is_empty() {
            sqbb = sqbb.shift1(dir);
            attack = attack | sqbb;
        }
    }
    attack
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bishop_attacks() {
        get_queen_attacks(20, Bitboard::empty()).print();
        //assert!(false);
    }
}
