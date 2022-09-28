//! This module implements all of chess board representations and the like.
//! Maybe this will include representations of various chess variants too, we'll see!
//!
//! There is a board struct, which a position is defined with.
//! From a board you can generate a list of legal moves, and play said moves.

pub use crate::types::*;

pub mod bitboards;
mod types;

pub mod chess;
