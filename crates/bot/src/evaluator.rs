use rand::seq::IndexedRandom;

use mancala::{Outcome, State};

/// policy: (action_index, prior in [0,1])  /  value in [-1,1] for current player.
pub trait Evaluator {
    fn policy_value(&self, state: &State) -> (Vec<(usize, f32)>, f32);
}

/// Baseline: uniform policy + light random rollout for value.
pub struct RandomEvaluator {
    playout_max_len: usize,
}

impl RandomEvaluator {
    pub fn new(playout_max_len: usize) -> Self {
        Self { playout_max_len }
    }
}

impl Default for RandomEvaluator {
    fn default() -> Self {
        Self::new(1024)
    }
}

impl Evaluator for RandomEvaluator {
    fn policy_value(&self, state: &State) -> (Vec<(usize, f32)>, f32) {
        let legal = state.legal_moves();
        let prior = if legal.is_empty() {
            Vec::new()
        } else {
            let p = 1.0f32 / (legal.len() as f32);
            legal.into_iter().map(|a| (a, p)).collect()
        };

        // quick rollout
        let mut rng = rand::rng();
        let mut s = state.clone();
        let root_player = s.current_player();

        for _ in 0..self.playout_max_len {
            if s.is_terminal() {
                break;
            }
            let moves = s.legal_moves();
            if moves.is_empty() {
                break;
            }
            let &m = moves.choose(&mut rng).unwrap();
            s = s.child_after_move(m).unwrap();
        }

        let v = if s.is_terminal() {
            match s.outcome() {
                Outcome::Win(p) if p == root_player => 1.0,
                Outcome::Win(_) => -1.0,
                Outcome::Draw => 0.0,
                Outcome::Ongoing => 0.0,
            }
        } else {
            0.0
        };

        (prior, v)
    }
}
