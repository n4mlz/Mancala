//! Watch two MCTS bots play Mancala. No asserts; prints boards & final result.

use bot::{RandomEvaluator, SearchConfig, mcts_search};
use mancala::{Outcome, Player, State};

fn mcts_pick(state: &State, sims: u32) -> Option<usize> {
    let eval = RandomEvaluator::default();
    let cfg = SearchConfig {
        simulations: sims,
        c_puct: 1.2,
    };
    mcts_search(state, cfg, &eval).chosen_action
}

fn main() {
    let mut s = State::new();
    let sims_per_move = 50000;

    println!("== Bot vs Bot ==");
    println!("{s}");

    while !s.is_terminal() {
        let to_move = s.current_player();
        let Some(action) = mcts_pick(&s, sims_per_move) else {
            println!("No legal moves. Stalemate?");
            break;
        };
        println!(">> {to_move} plays pit index {action}");
        s = s.child_after_move(action).expect("legal by construction");
        println!("{s}");
    }

    match s.outcome() {
        Outcome::Win(p) => println!(
            "Result: {p} wins.  score A={}, B={}",
            s.store(Player::A),
            s.store(Player::B)
        ),
        Outcome::Draw => println!(
            "Result: Draw.      score A={}, B={}",
            s.store(Player::A),
            s.store(Player::B)
        ),
        Outcome::Ongoing => println!("Result: Ongoing? (should not happen)"),
    }
}
