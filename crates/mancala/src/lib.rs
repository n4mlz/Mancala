//! Programmatic Kalah (Mancala variant) core for game-tree search.
//!
//! Public API surface is intentionally small:
//! - [`State`]: immutable game state
//! - [`State::legal_actions`]: enumerate successor states
//! - helpers: terminal check, winner, score, legal moves
//!
//! Rules are fixed by crate-level constants.

mod constants;
mod display;
mod outcome;
mod player;
mod state;

pub use constants::{PITS_PER_SIDE, STONES_PER_PIT};
pub use outcome::Outcome;
pub use player::Player;
pub use state::State;
