use crate::{Outcome, PITS_PER_SIDE, Player, STONES_PER_PIT};
use std::cmp::Ordering;

/// Immutable Mancala position.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct State {
    pits: [[u8; PITS_PER_SIDE]; 2],
    stores: [u8; 2],
    to_move: Player,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    /// Standard initial position.
    pub fn new() -> Self {
        Self {
            pits: [[STONES_PER_PIT; PITS_PER_SIDE]; 2],
            stores: [0, 0],
            to_move: Player::A,
        }
    }

    /// Whose turn it is.
    #[inline]
    pub fn current_player(&self) -> Player {
        self.to_move
    }

    /// Small pits for a side (read-only).
    #[inline]
    pub fn pits(&self, side: Player) -> &[u8; PITS_PER_SIDE] {
        &self.pits[side.idx()]
    }

    /// Stones in a side's store.
    #[inline]
    pub fn store(&self, side: Player) -> u8 {
        self.stores[side.idx()]
    }

    /// Legal moves as pit indices on the current side.
    pub fn legal_moves(&self) -> Vec<usize> {
        if self.is_terminal() {
            return Vec::new();
        }
        let side = self.to_move.idx();
        (0..PITS_PER_SIDE)
            .filter(|&i| self.pits[side][i] > 0)
            .collect()
    }

    /// Successor states after all legal moves, in ascending pit-index order.
    pub fn legal_actions(&self) -> Vec<State> {
        let moves = self.legal_moves();
        let mut out = Vec::with_capacity(moves.len());
        for m in moves {
            // safety: m is legal by construction
            out.push(self.child_after_move(m).unwrap());
        }
        out
    }

    /// Next state after applying `pit_index` if legal; otherwise `None`.
    pub fn child_after_move(&self, pit_index: usize) -> Option<State> {
        if self.is_terminal() || pit_index >= PITS_PER_SIDE {
            return None;
        }
        let side = self.to_move.idx();
        if self.pits[side][pit_index] == 0 {
            return None;
        }
        let mut s = State {
            pits: self.pits,
            stores: self.stores,
            to_move: self.to_move,
        };
        s.sow_from_pit(pit_index);
        Some(s)
    }

    /// Terminal if either side has no stones in small pits (after a move,
    /// remaining stones are swept to stores).
    pub fn is_terminal(&self) -> bool {
        self.pits[0].iter().all(|&x| x == 0) || self.pits[1].iter().all(|&x| x == 0)
    }

    /// Game outcome, assuming `is_terminal()` is true.
    pub fn outcome(&self) -> Outcome {
        if !self.is_terminal() {
            return Outcome::Ongoing;
        }
        match self.stores[0].cmp(&self.stores[1]) {
            Ordering::Greater => Outcome::Win(Player::A),
            Ordering::Less => Outcome::Win(Player::B),
            Ordering::Equal => Outcome::Draw,
        }
    }

    /// Store-score difference from `player`'s perspective.
    pub fn score_for(&self, player: Player) -> i32 {
        let a = self.stores[player.idx()] as i32;
        let b = self.stores[player.opponent().idx()] as i32;
        a - b
    }

    // ===== Internal engine =====

    fn sow_from_pit(&mut self, pit_index: usize) {
        let mover = self.to_move;
        let mover_i = mover.idx();

        let mut stones = self.pits[mover_i][pit_index];
        debug_assert!(stones > 0);
        self.pits[mover_i][pit_index] = 0;

        #[derive(Copy, Clone)]
        enum Loc {
            Pit { side: Player, idx: usize },
            Store { side: Player },
        }

        #[inline]
        fn next(loc: Loc) -> Loc {
            match loc {
                Loc::Pit { side, idx } if idx + 1 < PITS_PER_SIDE => {
                    Loc::Pit { side, idx: idx + 1 }
                }
                Loc::Pit { side, .. } => Loc::Store { side },
                Loc::Store { side } => Loc::Pit {
                    side: side.opponent(),
                    idx: 0,
                },
            }
        }

        let mut loc = Loc::Pit {
            side: mover,
            idx: pit_index,
        };
        let mut last = loc;

        while stones > 0 {
            loc = next(loc);

            // skip opponent's store
            if let Loc::Store { side } = loc
                && side == mover.opponent()
            {
                loc = next(loc);
            }

            match loc {
                Loc::Pit { side, idx } => self.pits[side.idx()][idx] += 1,
                Loc::Store { side } => self.stores[side.idx()] += 1,
            }

            stones -= 1;
            last = loc;
        }

        // capture: last stone landed on mover's empty pit; take opposite as well
        if let Loc::Pit { side, idx } = last
            && side == mover
            && self.pits[mover_i][idx] == 1
        {
            let opp = mover.opponent();
            let opp_i = opp.idx();
            let opp_idx = PITS_PER_SIDE - 1 - idx;
            let captured = self.pits[opp_i][opp_idx];
            if captured > 0 {
                self.pits[mover_i][idx] = 0;
                self.pits[opp_i][opp_idx] = 0;
                self.stores[mover_i] += captured + 1;
            }
        }

        // extra turn if last stone in mover's store; otherwise flip turn
        let extra = matches!(last, Loc::Store { side } if side == mover);
        if !extra {
            self.to_move = mover.opponent();
        }

        // end-of-game sweep if any side is empty
        let player_a_empty = self.pits[Player::A.idx()].iter().all(|&x| x == 0);
        let player_b_empty = self.pits[Player::B.idx()].iter().all(|&x| x == 0);
        if player_a_empty || player_b_empty {
            for i in 0..PITS_PER_SIDE {
                self.stores[0] += self.pits[0][i];
                self.pits[0][i] = 0;
                self.stores[1] += self.pits[1][i];
                self.pits[1][i] = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn total(st: &State) -> u16 {
        let a: u16 = st.pits(Player::A).iter().map(|&x| x as u16).sum();
        let b: u16 = st.pits(Player::B).iter().map(|&x| x as u16).sum();
        a + b + st.store(Player::A) as u16 + st.store(Player::B) as u16
    }

    #[test]
    fn initial_has_six_moves() {
        let s = State::new();
        assert_eq!(s.legal_moves().len(), PITS_PER_SIDE);
    }

    #[test]
    fn extra_turn_happens_on_initial_from_pit_2() {
        assert_eq!(PITS_PER_SIDE, 6);
        assert_eq!(STONES_PER_PIT, 4);
        let s = State::new();
        let me = s.current_player();
        let child = s.child_after_move(2).unwrap();
        assert_eq!(child.current_player(), me);
    }

    #[test]
    fn turn_switches_when_not_ending_in_own_store() {
        let s = State::new();
        let me = s.current_player();
        let child = s.child_after_move(0).unwrap();
        assert_eq!(child.current_player(), me.opponent());
    }

    #[test]
    fn capture_rule_works() {
        let mut s = State {
            pits: [[0; PITS_PER_SIDE]; 2],
            stores: [0, 0],
            to_move: Player::A,
        };
        s.pits[Player::A.idx()][0] = 1;
        s.pits[Player::A.idx()][1] = 0;
        s.pits[Player::B.idx()][PITS_PER_SIDE - 1 - 1] = 3;
        let child = s.child_after_move(0).unwrap();
        assert_eq!(child.store(Player::A), 4);
        assert_eq!(child.pits(Player::A)[1], 0);
        assert_eq!(child.pits(Player::B)[PITS_PER_SIDE - 1 - 1], 0);
    }

    #[test]
    fn no_capture_when_opposite_empty() {
        let mut s = State {
            pits: [[0; PITS_PER_SIDE]; 2],
            stores: [0, 0],
            to_move: Player::A,
        };
        s.pits[Player::A.idx()][0] = 1;
        s.pits[Player::A.idx()][1] = 0;
        s.pits[Player::B.idx()][PITS_PER_SIDE - 1 - 1] = 0;
        s.pits[Player::B.idx()][0] = 1; // make it non-terminal

        let child = s.child_after_move(0).unwrap();
        assert_eq!(child.store(Player::A), 0);
        assert_eq!(child.pits(Player::A)[1], 1);
    }

    #[test]
    fn child_is_none_on_terminal_position() {
        let s = State {
            pits: [[0; PITS_PER_SIDE]; 2],
            stores: [0, 0],
            to_move: Player::A,
        };
        assert!(s.is_terminal());
        assert!(s.child_after_move(0).is_none());
    }

    #[test]
    fn no_capture_when_landing_on_non_empty_own_pit() {
        let mut s = State {
            pits: [[0; PITS_PER_SIDE]; 2],
            stores: [0, 0],
            to_move: Player::A,
        };
        s.pits[Player::A.idx()][0] = 2;
        s.pits[Player::A.idx()][1] = 1;
        s.pits[Player::B.idx()][PITS_PER_SIDE - 1 - 1] = 5;
        let child = s.child_after_move(0).unwrap();
        assert_eq!(child.pits(Player::A)[1], 2);
        assert_eq!(child.store(Player::A), 0);
    }

    #[test]
    fn skip_opponents_store_on_sow() {
        let mut s = State {
            pits: [[1; PITS_PER_SIDE]; 2],
            stores: [0, 0],
            to_move: Player::A,
        };
        s.pits[Player::A.idx()][0] = 14;
        let before_b = s.store(Player::B);
        let t_before = total(&s);
        let child = s.child_after_move(0).unwrap();
        let after_b = child.store(Player::B);
        assert_eq!(after_b, before_b);
        assert_eq!(total(&child), t_before);
        assert!(!child.is_terminal());
    }

    #[test]
    fn wraparound_skips_opponents_store_and_preserves_total() {
        let mut s = State {
            pits: [[1; PITS_PER_SIDE]; 2],
            stores: [0, 0],
            to_move: Player::A,
        };
        s.pits[Player::A.idx()][5] = 20;
        let t_before = total(&s);
        let before_b = s.store(Player::B);
        let child = s.child_after_move(5).unwrap();
        assert_eq!(child.store(Player::B), before_b);
        assert_eq!(total(&child), t_before);
    }

    #[test]
    fn opponent_store_never_increases_during_my_move() {
        let s = State::new();
        for i in 0..PITS_PER_SIDE {
            let who = s.current_player();
            let before = s.store(who.opponent());
            let c = s.child_after_move(i);
            if let Some(c) = c {
                assert_eq!(c.store(who.opponent()), before);
            }
        }
    }

    #[test]
    fn terminal_sweep_when_side_becomes_empty() {
        let mut s = State {
            pits: [[0; PITS_PER_SIDE]; 2],
            stores: [0, 0],
            to_move: Player::A,
        };
        s.pits[Player::A.idx()][5] = 1;
        s.pits[Player::B.idx()][5] = 1;
        let child = s.child_after_move(5).unwrap();
        assert!(child.is_terminal());
        assert!(child.pits(Player::A).iter().all(|&x| x == 0));
        assert!(child.pits(Player::B).iter().all(|&x| x == 0));
        assert_eq!(child.store(Player::A) + child.store(Player::B), 2);
    }

    #[test]
    fn legal_moves_empty_when_terminal() {
        let s = State {
            pits: [[0; PITS_PER_SIDE]; 2],
            stores: [0, 0],
            to_move: Player::A,
        };
        assert!(s.is_terminal());
        assert!(s.legal_moves().is_empty());
        assert!(s.legal_actions().is_empty());
    }

    #[test]
    fn total_stones_invariant_for_many_moves() {
        let mut s = State::new();
        let t0 = total(&s);
        assert_eq!(t0, (2 * PITS_PER_SIDE as u16 * STONES_PER_PIT as u16));
        for i in 0..100 {
            if s.is_terminal() {
                break;
            }
            let moves = s.legal_moves();
            if moves.is_empty() {
                break;
            }
            let mv = moves[i % moves.len()];
            s = s.child_after_move(mv).unwrap();
            assert_eq!(total(&s), t0);
        }
    }
}
