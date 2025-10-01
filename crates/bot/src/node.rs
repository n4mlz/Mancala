use mancala::{Player, State};

use super::evaluator::Evaluator;

/// Single MCTS node (PUCT).
#[derive(Clone)]
pub struct Node {
    pub state: State,
    pub prior: f32,
    pub visits: u32,
    pub value_sum: f32,
    pub children: Vec<Node>,
    pub unexpanded: Vec<(usize, f32)>, // (action, prior)
    pub to_move: Player,
}

impl Node {
    pub fn new_root(state: State, priors: &[(usize, f32)]) -> Self {
        let to_move = state.current_player();
        let mut n = Self {
            state,
            prior: 1.0,
            visits: 0,
            value_sum: 0.0,
            children: Vec::new(),
            unexpanded: priors.to_vec(),
            to_move,
        };
        n.normalize_priors_if_needed();
        n
    }

    #[inline]
    pub fn is_terminal(&self) -> bool {
        self.state.is_terminal()
    }

    #[inline]
    pub fn value_mean(&self) -> f32 {
        if self.visits == 0 {
            0.0
        } else {
            self.value_sum / (self.visits as f32)
        }
    }

    fn normalize_priors_if_needed(&mut self) {
        let s: f32 = self.unexpanded.iter().map(|(_, p)| *p).sum();
        if s > 0.0 {
            for (_, p) in self.unexpanded.iter_mut() {
                *p /= s;
            }
        } else if !self.unexpanded.is_empty() {
            let u = 1.0 / (self.unexpanded.len() as f32);
            for (_, p) in self.unexpanded.iter_mut() {
                *p = u;
            }
        }
    }

    /// PUCT score: Q + c_puct * P * sqrt(N) / (1 + n)
    pub fn ucb(&self, child: &Node, c_puct: f32) -> f32 {
        let q_parent = if self.to_move == child.to_move {
            child.value_mean()
        } else {
            -child.value_mean()
        };

        let n = child.visits as f32;
        let n_parent = self.visits.max(1) as f32;
        q_parent + c_puct * child.prior * (n_parent.sqrt() / (1.0 + n))
    }

    pub fn best_child(&self, c_puct: f32) -> usize {
        let mut best = 0usize;
        let mut best_score = f32::NEG_INFINITY;
        for (i, ch) in self.children.iter().enumerate() {
            let s = self.ucb(ch, c_puct);
            if s > best_score {
                best_score = s;
                best = i;
            }
        }
        best
    }

    /// Expand one child using evaluator priors. Returns new child index.
    pub fn expand<E: Evaluator>(&mut self, eval: &E) -> Option<usize> {
        use rand::{distr::weighted::WeightedIndex, prelude::*};

        if self.is_terminal() || self.unexpanded.is_empty() {
            return None;
        }

        // Sample an action by prior (stochastic expansion).
        let weights: Vec<f32> = self.unexpanded.iter().map(|(_, p)| *p).collect();
        let dist = WeightedIndex::new(weights.iter().cloned().map(|w| w.max(1e-6))).ok()?;
        let mut rng = rand::rng();
        let idx = dist.sample(&mut rng);
        let (action, prior) = self.unexpanded.swap_remove(idx);

        let child_state = self.state.child_after_move(action).unwrap();
        let (child_priors, _v) = eval.policy_value(&child_state);

        let to_move = child_state.current_player();
        let mut child = Node {
            state: child_state,
            prior,
            visits: 0,
            value_sum: 0.0,
            children: Vec::new(),
            unexpanded: child_priors,
            to_move,
        };
        child.normalize_priors_if_needed();

        self.children.push(child);
        Some(self.children.len() - 1)
    }
}
