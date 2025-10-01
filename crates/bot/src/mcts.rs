use mancala::{Outcome, State};

use super::evaluator::Evaluator;
use super::node::Node;

#[derive(Copy, Clone)]
pub struct SearchConfig {
    pub simulations: u32,
    pub c_puct: f32,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            simulations: 10_000,
            c_puct: 1.4,
        }
    }
}

pub struct SearchReport {
    pub chosen_action: Option<usize>,
    pub root_visits: u32,
    pub child_visits: Vec<(usize, u32)>, // (action, visits)
}

/// Run MCTS and return argmax-visit action.
pub fn mcts_search<E: Evaluator>(root_state: &State, cfg: SearchConfig, eval: &E) -> SearchReport {
    let (root_priors, _root_v) = eval.policy_value(root_state);
    let mut root = Node::new_root(root_state.clone(), &root_priors);

    for _ in 0..cfg.simulations {
        simulate(&mut root, cfg.c_puct, eval);
    }

    // Choose action by visit count at root
    let mut best_action = None;
    let mut best_visits = 0u32;
    let mut stats = Vec::new();

    for ch in &root.children {
        // Derive which action produced this child
        let mut action: Option<usize> = None;
        for a in root_state.legal_moves() {
            if let Some(s) = root_state.child_after_move(a)
                && s == ch.state
            {
                action = Some(a);
                break;
            }
        }
        let a = action.unwrap_or(usize::MAX);
        stats.push((a, ch.visits));
        if ch.visits > best_visits {
            best_visits = ch.visits;
            best_action = Some(a);
        }
    }

    SearchReport {
        chosen_action: best_action,
        root_visits: root.visits,
        child_visits: stats,
    }
}

/// One simulation.
fn simulate<E: Evaluator>(root: &mut Node, c_puct: f32, eval: &E) {
    // Selection
    let mut path: Vec<*mut Node> = Vec::with_capacity(64);
    let mut node: *mut Node = root as *mut Node;

    unsafe {
        path.push(node);
        while !(*node).is_terminal() {
            if !(*node).unexpanded.is_empty() {
                break;
            }
            if (*node).children.is_empty() {
                break;
            }
            let i = (*node).best_child(c_puct);
            node = &mut (&mut (*node).children)[i] as *mut Node;
            path.push(node);
        }

        // Expansion → Evaluate
        let value = if !(*node).is_terminal() && !(*node).unexpanded.is_empty() {
            if let Some(i) = (*node).expand(eval) {
                node = &mut (&mut (*node).children)[i] as *mut Node;
                path.push(node);
            }
            evaluate_leaf(&*node, eval)
        } else {
            evaluate_leaf(&*node, eval)
        };

        // Backpropagation (flip sign only when the turn switches)
        let mut v = value;
        for i in (0..path.len()).rev() {
            let node_i = path[i];
            (*node_i).visits += 1;
            (*node_i).value_sum += v;

            if i > 0 {
                let parent = path[i - 1];
                if (*parent).to_move != (*node_i).to_move {
                    v = -v;
                }
            }
        }
    }
}

/// Evaluate a leaf: terminal → exact, else evaluator.value.
fn evaluate_leaf<E: Evaluator>(n: &Node, eval: &E) -> f32 {
    if n.is_terminal() {
        match n.state.outcome() {
            Outcome::Win(p) if p == n.to_move => -1.0,
            Outcome::Win(_) => 1.0,
            Outcome::Draw => 0.0,
            Outcome::Ongoing => 0.0,
        }
    } else {
        let (_pi, v) = eval.policy_value(&n.state);
        v
    }
}
