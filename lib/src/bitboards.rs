use std::ops::*;

/// The Bitboard type is a redefined u64 which has added helper functions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bitboard(u64);

impl Bitboard {
    pub fn empty() -> Self {
        Bitboard(0)
    }

    /// Create a bitboard which has one bit set at the specified square
    pub fn square(sq: u8) -> Self {
        debug_assert!(sq <= 63);

        Bitboard(1 << sq)
    }

    /// Set a bit on the given bitboard
    #[inline]
    pub fn set_bit(&mut self, bit: u8) {
        debug_assert!(bit <= 63);

        self.0 |= 1 << bit;
    }

    /// Clear a bit on the given bitboard
    #[inline]
    pub fn clear_bit(&mut self, bit: u8) {
        debug_assert!(bit <= 63);

        self.0 &= !(1 << bit);
    }

    /// Test to see if a bit is set
    #[inline]
    pub fn is_set(&self, bit: u8) -> bool {
        bit <= 63 && (self.0 & 1 << bit != 0)
    }

    /// Run the bmi BLSI instruction, or equivalent if your cpu does not support it
    #[inline]
    pub fn blsi(&self) -> Bitboard {
        // x & -x
        Bitboard(self.0 & self.0.wrapping_neg())
    }

    /// Return the blsi, and also remove the blsi from the bitboard
    #[inline]
    pub fn popblsi(&mut self) -> Bitboard {
        let blsi = self.blsi();
        self.0 &= self.0 - 1;
        blsi
    }

    /// Return the index of the least significant bit
    #[inline]
    pub fn lsb(&self) -> u32 {
        self.0.trailing_zeros()
    }

    #[inline]
    pub fn poplsb(&mut self) -> u32 {
        let lsb = self.lsb();
        self.0 &= self.0 - 1;
        lsb
    }

    /// Return true if the bitboard is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Set a bit based on it's square coordinates
    #[inline]
    pub fn set_sq(&mut self, r: u8, f: u8) {
        self.set_bit(sq(r, f));
    }

    /// Shift a piece by some square value something something todo magnitude of 1
    #[inline]
    pub fn shift1(self, shift: i8) -> Bitboard {
        let mut result = self;
        if shift < 0 {
            const MASKS: [u64; 2] = [!0x0101010101010101, !0x8080808080808080];
            let mask = if shift % 8 == 0 {
                !0
            } else if shift % 8 == -1 {
                MASKS[0]
            } else if shift % 8 == -7 {
                MASKS[1]
            } else {
                panic!("invalid shift of magnitude 1 (not magnitude of 1)");
            };
            result.0 = (self.0 & mask).overflowing_shr(-shift as u32).0;
        } else {
            const MASKS: [u64; 2] = [!0x8080808080808080, !0x0101010101010101];
            let mask = if shift % 8 == 0 {
                !0
            } else if shift % 8 == 1 {
                MASKS[0]
            } else if shift % 8 == 7 {
                MASKS[1]
            } else {
                panic!("invalid shift of magnitude 1 (not magnitude of 1)");
            };
            result.0 = (self.0 & mask).overflowing_shl(shift as u32).0;
        }
        result
    }

    pub fn print(self) {
        for row in (0..8).rev() {
            for col in 0..8 {
                if self.0 & (1 << sq(row, col)) != 0 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

    pub fn lsb_iter(self) -> LsbIter {
        LsbIter { bb: self }
    }
}

impl From<u64> for Bitboard {
    fn from(b: u64) -> Bitboard {
        Bitboard(b)
    }
}

// this should not be here
#[inline]
pub fn sq(r: u8, f: u8) -> u8 {
    r * 8 + f
}

macro_rules! bitboard_operation {
    ($i:ty, $j:ty, $f:ident) => {
        impl $i for Bitboard {
            type Output = Self;

            fn $f(self, rhs: Self) -> Self::Output {
                Self(self.0.$f(rhs.0))
            }
        }

        impl $j for Bitboard {
            type Output = Self;

            fn $f(self, rhs: u64) -> Self::Output {
                Self(self.0.$f(rhs))
            }
        }
    };
}

// TODO add assigning operators

bitboard_operation!(Add, Add<u64>, add);
bitboard_operation!(BitAnd, BitAnd<u64>, bitand);
bitboard_operation!(BitOr, BitOr<u64>, bitor);
bitboard_operation!(BitXor, BitXor<u64>, bitxor);
bitboard_operation!(Shl, Shl<u64>, shl);
bitboard_operation!(Shr, Shr<u64>, shr);
bitboard_operation!(Sub, Sub<u64>, sub);

impl Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(self.0.not())
    }
}

// LSB iterator
// Iterate over the index of each bit starting from the least significant bit
// I'll do proper docs later
pub struct LsbIter {
    bb: Bitboard,
}

impl Iterator for LsbIter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if !self.bb.is_empty() {
            Some(self.bb.poplsb())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bitboards::*;
    #[test]
    fn test_bit_bounds() {
        assert_eq!(Bitboard(0).is_set(68), false);
    }

    #[test]
    fn bit_and() {
        assert_eq!(Bitboard(13) & Bitboard(7), Bitboard(5));
    }
}
