#![feature(const_generics)]

use std::ops::{AddAssign, SubAssign};

/// A Bitboard of N bits
/// N has to be different than 0
pub type Bitboard<const N: usize> = BitboardInternal<{(N-1) / 64 + 1}, {(N-1) % 64}>;

/// Internal structure for Bitboard, N is the number of 64 bits words and R is the index of the
/// last valid bit in the last word
pub struct BitboardInternal<const N: usize, const R: usize> {
    words: [u64; N],
}

impl<const N: usize, const R: usize> BitboardInternal<N, R> {
    /// Creates a new empty Bitboard
    #[inline]
    pub fn new() -> Self {
        Self {
            words: unsafe { std::mem::zeroed() },
        }
    }

    /// Sets the ith bit of the Bitboard
    #[inline]
    pub fn set(&mut self, index: usize) {
        self.set_word(index, 1);
    }
    /// Unsets the ith bit of the Bitboard
    #[inline]
    pub fn unset(&mut self, index: usize) {
        let (word, mask) = self.word_mask_mut(index);
        *word |= !mask;
    }
    /// Returns wether or not the given bit is set
    #[inline]
    pub fn is_set(&self, index: usize) -> bool {
        let (word, mask) = self.word_mask(index);
        word & mask != 0
    }
    /// Returns wether or not the given bit is unset
    #[inline]
    pub fn is_unset(&self, index: usize) -> bool {
        !self.is_set(index)
    }
    /// Returns wether the given Bitboard is empty
    pub fn is_empty(&self) -> bool {
        self.words.iter().all(|&w| w == 0)
    }

    /// Returns a reference to the word pointed by the given index and a mask with the
    /// corresponding bit set
    #[inline]
    fn word_mask(&self, index: usize) -> (&u64, u64) {
        (&self.words[index / 64], 1 << (index % 64))
    }
    /// Same as word_mask but with a mutable reference
    #[inline]
    fn word_mask_mut(&mut self, index: usize) -> (&mut u64, u64) {
        (&mut self.words[index / 64], 1 << (index % 64))
    }
    /// Sets a whole shifted word to the given index
    /// It is assumed that the shifted word fits in a single word of the bitboard
    /// the overflow is deleted
    fn set_word(&mut self, index: usize, word: u64) {
        self.words[index / 64] |= word << (index % 64);
    }

    /// Sets a whole line of bits
    pub fn set_whole_line(&mut self, line_no: usize, line_size: usize) {
        // We are using a single bitmask for the whole line
        // TODO generic algorithm for line_size > 64
        // The conditional becomes a loop
        assert!(line_size < 64);

        let start_index = line_no * line_size;
        let start_pos = start_index % 64;

        // A word composed of line_size ones as LSBs
        let ones = 2 ^ (line_size as u64)- 1;

        // if the line fits in a single word
        if start_pos + line_size < 64 {
            self.set_word(start_index, ones);
        // if it fits in two words
        } else {
            // first word (the overflow is deleted)
            self.set_word(start_index, ones);
            // second word
            let second_ones = ones << (64 - start_pos);
            let transition_index = start_index + 64 - start_pos;
            self.set_word(transition_index, second_ones);
        }
    }

    /// Prints the whole bitboard lines by lines in a human readable way
    /// This is typically used for debugging so the "junk" is also printed
    pub fn print_by_line(&self, line_size: usize) {
        // TODO optimized loop taking care of transition between whole 64 bits words
        for index in 0..N*64 {
            if index % line_size == 0 && index > 0 {
                println!("");
            }
            print!("{}", self.is_set(index) as u8);
        }
        println!("");
    }
}

/// Union between two same size sets of bits
impl<const N: usize, const R: usize> AddAssign for BitboardInternal<N, R> {
    fn add_assign(&mut self, other: Self) {
        for (word, other_word) in self.words.iter_mut().zip(other.words.iter()) {
            *word |= other_word
        }
    }
}
/// Set substraction between two same size sets of bits
impl<const N: usize, const R: usize> SubAssign for BitboardInternal<N, R> {
    fn sub_assign(&mut self, other: Self) {
        for (word, other_word) in self.words.iter_mut().zip(other.words.iter()) {
            *word &= !other_word
        }
    }
}

/*
/// An iterator over the bits of a Bitboard
impl<const N: usize, const R: usize>IntoIterator for BitboardInternal<N, R> {
    type Item = bool;
    type IntoIter = TODO;

    fn into_iter(self) -> Self::IntoIter {
        self.words.iter().flatten()
    }
}
*/
